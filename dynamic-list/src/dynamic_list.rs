use std::{collections::HashMap, io::SeekFrom, path::Path};
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};

use crate::DynamicListError;

type Result<T> = std::result::Result<T, DynamicListError>;

#[derive(Debug)]
pub struct DynamicList<const C: usize> {
    file: File,
}

impl<const C: usize> DynamicList<C> {
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let parent_dir_path = path
            .parent()
            .ok_or(DynamicListError::ParentDirectoryNotSpecified)?
            .to_string_lossy()
            .to_string();

        if parent_dir_path.is_empty() {
            return Err(DynamicListError::ParentDirectoryNotSpecified);
        }

        create_dir_all(parent_dir_path).await?;

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .await?;

        Ok(Self { file })
    }

    pub async fn push(&mut self, buf: [u8; C]) -> Result<u64> {
        let padding = self.file.metadata().await?.len();

        let index = padding / C as u64;

        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.write_all(&buf).await?;
        self.file.flush().await?;

        Ok(index)
    }

    pub async fn set(&mut self, index: u64, buf: [u8; C]) -> Result<()> {
        let padding = index * C as u64;

        let len = self.file.metadata().await?.len();

        if len < padding + C as u64 {
            return Err(DynamicListError::IndexOutOfBounds);
        }

        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.write_all(&buf).await?;
        self.file.flush().await?;

        Ok(())
    }

    pub async fn get(&mut self, index: u64) -> Result<[u8; C]> {
        let padding = index * C as u64;

        let len = self.file.metadata().await?.len();

        if len < padding + C as u64 {
            return Err(DynamicListError::IndexOutOfBounds);
        }

        let mut buf = [0u8; C];

        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.read_exact(&mut buf).await?;

        Ok(buf)
    }

    pub async fn for_each<F, K, V, E>(
        &mut self,
        map: &mut HashMap<K, V>,
        f: F,
    ) -> std::result::Result<(), E>
    where
        F: Fn([u8; C], u64, &mut HashMap<K, V>) -> std::result::Result<(), E>,
        E: From<DynamicListError>,
    {
        let len = self
            .file
            .metadata()
            .await
            .map_err(|err| DynamicListError::Io(err))?
            .len();

        let items_count = len / C as u64;

        for i in 0..items_count {
            let mut buf = [0u8; C];
            let padding = i * C as u64;

            self.file
                .seek(SeekFrom::Start(padding))
                .await
                .map_err(|err| DynamicListError::Io(err))?;
            self.file
                .read_exact(&mut buf)
                .await
                .map_err(|err| DynamicListError::Io(err))?;

            f(buf, i, map)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::remove_file;

    #[tokio::test]
    async fn creates_list() {
        let dir = "/tmp/nacho/tests/dynamic_list/creates_list";

        let list = DynamicList::<2>::new(dir).await.unwrap();

        let _ = list;

        remove_file(dir).await.unwrap();
    }

    #[tokio::test]
    async fn pushes_buffers() {
        let dir = "/tmp/nacho/tests/dynamic_list/pushes_buffers";

        let mut list = DynamicList::<2>::new(dir).await.unwrap();

        list.push([100, 90]).await.unwrap();
        list.push([80, 70]).await.unwrap();
        list.push([60, 50]).await.unwrap();
        list.push([40, 30]).await.unwrap();
        list.push([20, 10]).await.unwrap();

        remove_file(dir).await.unwrap();
    }

    #[tokio::test]
    async fn sets_buffers() {
        let dir = "/tmp/nacho/tests/dynamic_list/sets_buffers";

        let mut list = DynamicList::<2>::new(dir).await.unwrap();

        list.push([100, 90]).await.unwrap();
        list.push([80, 70]).await.unwrap();

        list.set(0, [100, 90]).await.unwrap();
        list.set(1, [80, 70]).await.unwrap();

        remove_file(dir).await.unwrap();
    }

    #[tokio::test]
    async fn gets_buffers() {
        let dir = "/tmp/nacho/tests/dynamic_list/gets_buffers";

        let mut list = DynamicList::<2>::new(dir).await.unwrap();

        list.push([100, 90]).await.unwrap();
        list.push([80, 70]).await.unwrap();

        let buf = list.get(0).await.unwrap();
        assert_eq!(buf, [100, 90]);

        let buf = list.get(1).await.unwrap();
        assert_eq!(buf, [80, 70]);

        remove_file(dir).await.unwrap();
    }

    #[tokio::test]
    async fn runs_given_function_for_each_buffer_correctly() {
        let dir = "/tmp/nacho/tests/dynamic_list/runs_given_function_for_each_buffer_correctly";

        let mut list = DynamicList::<2>::new(dir).await.unwrap();

        list.push([10, 20]).await.unwrap();
        list.push([20, 30]).await.unwrap();
        list.push([30, 40]).await.unwrap();
        list.push([40, 50]).await.unwrap();
        list.push([50, 60]).await.unwrap();

        let mut map = HashMap::<u8, u8>::new();
        list.for_each(&mut map, |buf, i, _| {
            assert_eq!(buf[0], (i as u8 + 1) * 10);
            assert_eq!(buf[1], (i as u8 + 2) * 10);
            Ok::<(), DynamicListError>(())
        })
        .await
        .unwrap();

        remove_file(dir).await.unwrap();
    }

    #[tokio::test]
    async fn doesnt_get_buffers_at_unset_indexes() {
        let dir = "/tmp/nacho/tests/dynamic_list/doesnt_get_buffers_at_unset_indexes";

        let mut list = DynamicList::<2>::new(dir).await.unwrap();

        let err = list.get(0).await.unwrap_err();
        assert!(matches!(err, DynamicListError::IndexOutOfBounds));

        let err = list.get(1).await.unwrap_err();
        assert!(matches!(err, DynamicListError::IndexOutOfBounds));

        remove_file(dir).await.unwrap();
    }

    #[tokio::test]
    async fn doesnt_set_buffers_at_indexes_out_of_bounds() {
        let dir = "/tmp/nacho/tests/dynamic_list/doesnt_set_buffers_at_indexes_out_of_bounds";

        let mut list = DynamicList::<2>::new(dir).await.unwrap();

        let err = list.set(0, [100, 90]).await.unwrap_err();
        assert!(matches!(err, DynamicListError::IndexOutOfBounds));

        let err = list.set(1, [80, 70]).await.unwrap_err();
        assert!(matches!(err, DynamicListError::IndexOutOfBounds));

        remove_file(dir).await.unwrap();
    }

    #[tokio::test]
    async fn doesnt_create_list_without_parent_dir() {
        let dir = "doesnt_create_list_without_parent_dir";

        let err = DynamicList::<2>::new(dir).await.unwrap_err();

        assert!(matches!(err, DynamicListError::ParentDirectoryNotSpecified));
    }
}
