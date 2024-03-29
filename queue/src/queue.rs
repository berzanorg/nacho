use crate::error::QueueError;
use std::{io::SeekFrom, marker::PhantomData, path::Path};
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};

type Result<T> = std::result::Result<T, QueueError>;

/// An on-disk FIFO (first in, first out) queue optimized for constant memory usage, high performance, and low disk usage.
///
/// Items are stored in a file and it has a garbage collector to remove popped items from disk.
///
/// The constant generic parameter `C` represents the size of each item in bytes.
///
/// The generic parameter `T` represents the type of the data structure that is going to be stored.
///
/// It requires the type `T` to implement `Into<[u8; C]>` and the type `&T` to implement `From<[u8; C]>`traits.
///
pub struct Queue<const C: usize, T>
where
    for<'a> &'a T: Into<[u8; C]>,
    T: From<[u8; C]>,
{
    file: File,
    phantom: PhantomData<T>,
}

impl<const C: usize, T> Queue<C, T>
where
    for<'a> &'a T: Into<[u8; C]>,
    T: From<[u8; C]>,
{
    /// Creates a new `Queue` at the given path.
    ///
    /// # Examples
    ///
    /// Create and implement a type that meets the requirements:
    ///
    /// ```rs
    /// struct User {
    ///     points: u16,
    /// }
    ///
    /// impl From<[u8; 2]> for User {
    ///     fn from(value: [u8; 2]) -> Self {
    ///         Self { points: u16::from_le_bytes(value) }
    ///     }
    /// }
    ///
    /// impl From<&User> for [u8; 2] {
    ///     fn from(value: &User) -> [u8; 2] {
    ///         value.points.to_le_bytes()
    ///     }
    /// }
    /// ```
    ///
    /// Create a queue:
    /// ```rs
    /// let queue = Queue::<4, T>::new("tmp/nacho/tests/queue").await?;
    /// ```
    ///
    /// Push an item:
    ///
    /// ```rs
    /// let item = User { points: 42 };
    /// queue.push(&item).await?;
    /// ```
    ///
    /// Pop an item:
    ///
    /// ```rs
    /// let item: Option<T> = queue.pop().await?;
    /// ```
    ///
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let parent_dir_path = path
            .parent()
            .ok_or(QueueError::NoParentDirectorySpecified {
                path: path.to_string_lossy().to_string(),
            })?
            .to_string_lossy()
            .to_string();

        if parent_dir_path.is_empty() {
            return Err(QueueError::NoParentDirectorySpecified {
                path: path.to_string_lossy().to_string(),
            });
        }

        create_dir_all(parent_dir_path).await?;

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .await?;

        Ok(Self {
            file,
            phantom: PhantomData,
        })
    }

    /// Pushes the given item to the end of the queue.
    ///
    /// # Examples
    ///
    /// Push an item:
    ///
    /// ```rs
    /// queue.push(&item).await?;
    /// ```
    ///
    pub async fn push(&mut self, item: &T) -> Result<()> {
        let item: [u8; C] = item.into();

        let file_len = self.get_file_len().await?;

        if file_len == 0 {
            self.init_pointer().await?;
        }

        self.file.seek(SeekFrom::End(0)).await?;
        self.file.write(&item).await?;
        self.file.flush().await?;

        Ok(())
    }

    /// Returns the first item of the queue.
    ///
    /// It returns `None` if the queue is empty.
    ///
    /// Each pop operation increments the pointer by the size of an item which enables tracking the top of the queue.
    ///
    /// It also runs garbage collection when the pointer is after 128 items which might take some time.
    ///
    /// # Examples
    ///
    /// Pop an item:
    ///
    /// ```rs
    /// let item: Option<T> = queue.pop().await?;
    /// ```
    ///
    pub async fn pop(&mut self) -> Result<Option<T>> {
        let file_len = self.file.metadata().await?.len();
        if file_len == 0 {
            self.init_pointer().await?;
        }

        let pointer = self.get_pointer().await?;

        let item = self.read_oldest_item(file_len, pointer).await?;

        let new_pointer = match item {
            Some(_) => {
                let new_pointer = pointer + C as u64;
                self.set_pointer(new_pointer).await?;
                new_pointer
            }
            None => pointer,
        };
        println!("np: {}", new_pointer);

        if new_pointer == 8 + 128 * C as u64 {
            self.run_garbage_collector(file_len, new_pointer).await?;
        }

        Ok(item)
    }

    /// Initializes the value of the pointer as 8 which is the size of the pointer in bytes.
    ///
    /// This function is for internal use, do not use it outside of this crate.
    ///
    /// # Examples
    ///
    /// Initialize the pointer:
    ///
    /// ```rs
    /// self.init_pointer().await?;
    /// ```
    ///
    #[inline]
    async fn init_pointer(&mut self) -> Result<()> {
        self.file.seek(SeekFrom::Start(0)).await?;
        self.file.write(&8_u64.to_le_bytes()).await?;
        self.file.flush().await?;

        Ok(())
    }

    /// Returns the value of the pointer that allows the queue to keep track of popped items.
    ///
    /// This function is for internal use, do not use it outside of this crate.
    ///
    /// # Examples
    ///
    /// Get the pointer:
    ///
    /// ```rs
    /// let pointer = self.get_pointer().await?;
    /// ```
    ///
    #[inline]
    async fn get_pointer(&mut self) -> Result<u64> {
        let mut buf = [0_u8; 8];

        self.file.seek(SeekFrom::Start(0)).await?;
        self.file.read_exact(&mut buf).await?;

        let pointer: u64 = u64::from_le_bytes(buf);

        Ok(pointer)
    }

    /// Updates the value of the pointer that allows the queue to keep track of popped items.
    ///
    /// This function is for internal use, do not use it outside of this crate.
    ///
    /// # Examples
    ///
    /// Set the pointer:
    ///
    /// ```rs
    /// self.set_pointer(new_pointer).await?;
    /// ```
    ///
    #[inline]
    async fn set_pointer(&mut self, new_value: u64) -> Result<()> {
        self.file.seek(SeekFrom::Start(0)).await?;
        self.file.write(&new_value.to_le_bytes()).await?;
        self.file.flush().await?;

        Ok(())
    }

    /// Returns the length of the file that stores the items in the queue.
    ///
    /// This function is for internal use, do not use it outside of this crate.
    ///
    /// # Examples
    ///
    /// Get the file length:
    ///
    /// ```rs
    /// let file_len = self.get_file_len().await?;
    /// ```
    ///
    #[inline]
    async fn get_file_len(&mut self) -> Result<u64> {
        Ok(self.file.metadata().await?.len())
    }

    /// Runs the garbage collector to remove popped items from disk.
    ///
    /// It rewrites the remaining items in chunks.
    /// It is optimized for constant memory usage.
    ///
    /// This function should not be called regularly as it is computation heavy.
    ///
    /// This function is for internal use, do not use it outside of this crate.
    ///
    /// # Examples
    ///
    /// Run the garbage collector:
    ///
    /// ```rs
    /// self.run_garbage_collector(file_len, new_pointer).await?;
    /// ```
    ///
    #[inline]
    async fn run_garbage_collector(&mut self, file_len: u64, pointer: u64) -> Result<()> {
        let content_size = file_len - pointer;
        let new_file_len = content_size + 8;
        let items_count = content_size / C as u64;

        let (chunk_count, chunks_len, remaining_count) = match items_count {
            ..=127 => (0, 0, items_count),
            128..=1023 => (128, items_count / 128, items_count % 128),
            1024.. => (1024, items_count / 1024, items_count % 1024),
        };

        for i in 0..chunks_len {
            let mut buf = vec![0_u8; C * chunk_count];

            let padding = C as u64 * i * chunk_count as u64;

            self.file.seek(SeekFrom::Start(pointer + padding)).await?;
            self.file.read_exact(&mut buf).await?;

            self.file.seek(SeekFrom::Start(8 + padding)).await?;
            self.file.write(&buf).await?;
        }

        if remaining_count != 0 {
            let mut buf = vec![0_u8; C * remaining_count as usize];

            let padding = C as u64 * chunks_len * chunk_count as u64;

            self.file.seek(SeekFrom::Start(pointer + padding)).await?;
            self.file.read_exact(&mut buf).await?;

            self.file.seek(SeekFrom::Start(8 + padding)).await?;
            self.file.write(&buf).await?;
        }

        self.file.flush().await?;

        self.init_pointer().await?;

        self.file.set_len(new_file_len).await?;

        Ok(())
    }

    /// Returns the oldest item in the queue.
    ///
    /// Returns `None` if the queue is empty.
    ///
    /// This function is for internal use, do not use it outside of this crate.
    ///
    /// # Examples
    ///
    /// Read the oldest item:
    /// ```rs
    /// let item: Option<T> = self.read_oldest_item(file_len, pointer).await?;
    /// ```
    ///
    #[inline]
    async fn read_oldest_item(&mut self, file_len: u64, pointer: u64) -> Result<Option<T>> {
        let mut buf = [0_u8; C];

        if file_len < pointer + C as u64 {
            Ok(None)
        } else {
            self.file.seek(SeekFrom::Start(pointer)).await?;
            self.file.read_exact(&mut buf).await?;

            let item: T = buf.into();

            Ok(Some(item))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::remove_file;

    #[derive(Debug, PartialEq)]
    struct T {
        num: u32,
    }

    impl From<[u8; 4]> for T {
        fn from(value: [u8; 4]) -> Self {
            Self {
                num: u32::from_le_bytes(value),
            }
        }
    }

    impl From<&T> for [u8; 4] {
        fn from(value: &T) -> Self {
            value.num.to_le_bytes()
        }
    }

    #[tokio::test]
    pub async fn creates_queue() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/queue/creates_queue";

        let _ = Queue::<4, T>::new(dir).await?;

        Ok(remove_file(dir).await?)
    }

    #[tokio::test]
    pub async fn doesnt_create_queue_when_parent_dir_not_given(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "doesnt_create_queue_when_parent_dir_not_given";

        match Queue::<4, T>::new(dir).await {
            Err(QueueError::NoParentDirectorySpecified { path }) => {
                assert_eq!(path, dir)
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    #[tokio::test]
    pub async fn pushes_and_pops_items() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/queue/pushes_and_pops_items";
        // remove_file(dir).await?;
        let mut queue = Queue::<4, T>::new(dir).await?;

        assert_eq!(queue.pop().await?, None);

        queue.push(&T { num: 5 }).await?;

        assert_eq!(queue.pop().await?, Some(T { num: 5 }));

        assert_eq!(queue.pop().await?, None);

        queue.push(&T { num: 7 }).await?;
        queue.push(&T { num: 8 }).await?;
        queue.push(&T { num: 9 }).await?;

        assert_eq!(queue.pop().await?, Some(T { num: 7 }));
        assert_eq!(queue.pop().await?, Some(T { num: 8 }));
        assert_eq!(queue.pop().await?, Some(T { num: 9 }));

        assert_eq!(queue.pop().await?, None);
        assert_eq!(queue.pop().await?, None);

        queue.push(&T { num: 4 }).await?;

        assert_eq!(queue.pop().await?, Some(T { num: 4 }));

        assert_eq!(queue.pop().await?, None);
        assert_eq!(queue.pop().await?, None);

        for i in 0..128 {
            queue.push(&T { num: i }).await?;
        }

        for i in 0..128 {
            assert_eq!(queue.pop().await?, Some(T { num: i }));
        }

        for i in 0..400 {
            queue.push(&T { num: i }).await?;
        }

        for i in 0..400 {
            assert_eq!(queue.pop().await?, Some(T { num: i }));
        }

        for i in 0..1024 {
            queue.push(&T { num: i }).await?;
        }

        for i in 0..1024 {
            assert_eq!(queue.pop().await?, Some(T { num: i }));
        }

        for i in 0..2070 {
            queue.push(&T { num: i }).await?;
        }

        for i in 0..2070 {
            assert_eq!(queue.pop().await?, Some(T { num: i }));
        }

        Ok(remove_file(dir).await?)
    }

    #[tokio::test]
    pub async fn collects_garbage() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let dir = "/tmp/nacho/tests/queue/collects_garbage";

        let mut queue = Queue::<4, T>::new(dir).await?;

        for i in 0..128 {
            queue.push(&T { num: i }).await?;
        }

        let file_len = queue.file.metadata().await?.len();
        assert_eq!(file_len, 128 * 4 + 8);

        for _ in 0..128 {
            queue.pop().await?;
        }

        let file_len = queue.file.metadata().await?.len();
        assert_eq!(file_len, 0 * 4 + 8);

        for i in 0..140 {
            queue.push(&T { num: i }).await?;
        }

        let file_len = queue.file.metadata().await?.len();
        assert_eq!(file_len, 140 * 4 + 8);

        for _ in 0..140 {
            queue.pop().await?;
        }

        let file_len = queue.file.metadata().await?.len();
        assert_eq!(file_len, 12 * 4 + 8);

        for i in 0..128 {
            queue.push(&T { num: i }).await?;
        }

        let file_len = queue.file.metadata().await?.len();
        assert_eq!(file_len, 140 * 4 + 8);

        for _ in 0..128 {
            queue.pop().await?;
        }

        let file_len = queue.file.metadata().await?.len();
        assert_eq!(file_len, 12 * 4 + 8);

        for i in 0..515 {
            queue.push(&T { num: i }).await?;
        }

        let file_len = queue.file.metadata().await?.len();
        assert_eq!(file_len, (12 + 515) * 4 + 8);

        for _ in 0..515 {
            queue.pop().await?;
        }

        let file_len = queue.file.metadata().await?.len();
        assert_eq!(file_len, (12 + 3) * 4 + 8);

        for i in 0..2050 {
            queue.push(&T { num: i }).await?;
        }

        let file_len = queue.file.metadata().await?.len();
        assert_eq!(file_len, (12 + 3 + 2050) * 4 + 8);

        for _ in 0..2050 {
            queue.pop().await?;
        }

        let file_len = queue.file.metadata().await?.len();
        assert_eq!(file_len, (12 + 3 + 2) * 4 + 8);

        Ok(remove_file(dir).await?)
    }
}
