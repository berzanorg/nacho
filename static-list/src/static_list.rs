use crate::StaticListError;
use std::{io::SeekFrom, path::Path};
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};

type Result<T> = std::result::Result<T, StaticListError>;

#[derive(Debug)]
pub struct StaticList<const C: usize, const L: usize> {
    file: File,
}

impl<const C: usize, const L: usize> StaticList<C, L> {
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let parent_dir_path = path
            .parent()
            .ok_or(StaticListError::ParentDirectoryNotSpecified)?
            .to_string_lossy()
            .to_string();

        if parent_dir_path.is_empty() {
            return Err(StaticListError::ParentDirectoryNotSpecified);
        }

        create_dir_all(parent_dir_path).await?;

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .await?;

        if file.metadata().await?.len() == 0 {
            let buf = vec![0u8; C * L];

            file.seek(SeekFrom::Start(0)).await?;
            file.write_all(&buf).await?;
            file.flush().await?;
        }

        Ok(Self { file })
    }

    pub async fn set(&mut self, index: u64, buf: &[u8; C]) -> Result<()> {
        let padding = index * C as u64;

        let len = self.file.metadata().await?.len();

        if len < padding + C as u64 {
            return Err(StaticListError::IndexOutOfBounds);
        }

        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.write_all(buf).await?;
        self.file.flush().await?;

        Ok(())
    }

    pub async fn get(&mut self, index: u64) -> Result<[u8; C]> {
        let padding = index * C as u64;

        let len = self.file.metadata().await?.len();

        if len < padding + C as u64 {
            return Err(StaticListError::IndexOutOfBounds);
        }

        let mut buf = [0u8; C];

        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.read_exact(&mut buf).await?;

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::remove_file;

    #[tokio::test]
    async fn creates_list() {
        let dir = "/tmp/nacho/tests/static_list/creates_list";

        let list = StaticList::<2, 4>::new(dir).await.unwrap();

        let _ = list;

        remove_file(dir).await.unwrap();
    }

    #[tokio::test]
    async fn sets_buffers() {
        let dir = "/tmp/nacho/tests/static_list/sets_buffers";

        let mut list = StaticList::<2, 2>::new(dir).await.unwrap();

        list.set(0, &[100, 90]).await.unwrap();
        list.set(1, &[80, 70]).await.unwrap();

        remove_file(dir).await.unwrap();
    }

    #[tokio::test]
    async fn gets_buffers() {
        let dir = "/tmp/nacho/tests/static_list/gets_buffers";

        let mut list = StaticList::<2, 2>::new(dir).await.unwrap();

        list.set(0, &[100, 90]).await.unwrap();
        list.set(1, &[80, 70]).await.unwrap();

        let buf = list.get(0).await.unwrap();
        assert_eq!(buf, [100, 90]);

        let buf = list.get(1).await.unwrap();
        assert_eq!(buf, [80, 70]);

        remove_file(dir).await.unwrap();
    }

    #[tokio::test]
    async fn doesnt_get_buffers_at_unset_indexes() {
        let dir = "/tmp/nacho/tests/static_list/doesnt_get_buffers_at_unset_indexes";

        let mut list = StaticList::<2, 5>::new(dir).await.unwrap();

        let err = list.get(6).await.unwrap_err();
        assert!(matches!(err, StaticListError::IndexOutOfBounds));

        let err = list.get(7).await.unwrap_err();
        assert!(matches!(err, StaticListError::IndexOutOfBounds));

        remove_file(dir).await.unwrap();
    }

    #[tokio::test]
    async fn doesnt_set_buffers_at_indexes_out_of_bounds() {
        let dir = "/tmp/nacho/tests/static_list/doesnt_set_buffers_at_indexes_out_of_bounds";

        let mut list = StaticList::<2, 2>::new(dir).await.unwrap();

        let err = list.set(3, &[100, 90]).await.unwrap_err();
        assert!(matches!(err, StaticListError::IndexOutOfBounds));

        let err = list.set(4, &[80, 70]).await.unwrap_err();
        assert!(matches!(err, StaticListError::IndexOutOfBounds));

        remove_file(dir).await.unwrap();
    }

    #[tokio::test]
    async fn doesnt_create_list_without_parent_dir() {
        let dir = "doesnt_create_list_without_parent_dir";

        let err = StaticList::<2, 3>::new(dir).await.unwrap_err();

        assert!(matches!(err, StaticListError::ParentDirectoryNotSpecified));
    }
}
