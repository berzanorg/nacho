use crate::error::EventsDbError;
use std::{io::SeekFrom, path::Path};
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};

pub struct EventsDb {
    file: File,
}

type Result<T> = std::result::Result<T, EventsDbError>;

impl EventsDb {
    pub async fn new(path: impl AsRef<Path>) -> Result<EventsDb> {
        let path = path.as_ref();

        create_dir_all(path).await?;

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.join("file"))
            .await?;

        if file.metadata().await?.len() != 8 {
            let buf = [0u8; 8]; // 2 x u32 as u8

            file.seek(SeekFrom::Start(0)).await?;
            file.write_all(&buf).await?;
            file.flush().await?;
        }

        Ok(Self { file })
    }

    pub async fn get_last_fetched_blocks(&mut self) -> Result<(u32, u32)> {
        let mut buf = [0u8; 8];
        self.file.seek(SeekFrom::Start(0)).await?;
        self.file.read_exact(&mut buf).await?;

        let from_block_deposited = u32::from_le_bytes(buf[0..4].try_into().unwrap());
        let from_block_withdrawn = u32::from_le_bytes(buf[4..8].try_into().unwrap());

        Ok((from_block_deposited, from_block_withdrawn))
    }

    pub async fn set_last_fetched_blocks(
        &mut self,
        from_block_deposited: u32,
        from_block_withdrawn: u32,
    ) -> Result<()> {
        let mut buf = [0u8; 8];

        buf[0..4].copy_from_slice(&from_block_deposited.to_le_bytes());
        buf[4..8].copy_from_slice(&from_block_withdrawn.to_le_bytes());

        self.file.seek(SeekFrom::Start(0)).await?;
        self.file.write_all(&buf).await?;
        self.file.flush().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tokio::fs::remove_dir_all;

    use super::*;

    #[tokio::test]
    async fn creates_events_db() {
        let dir = "/tmp/nacho/tests/events_db/creates_events_db";

        let events_db = EventsDb::new(dir).await.unwrap();
        let _ = events_db;

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn initializes_correctly() {
        let dir = "/tmp/nacho/tests/events_db/initializes_correctly";

        let mut events_db = EventsDb::new(dir).await.unwrap();

        let (from_block_deposited, from_block_withdrawn) =
            events_db.get_last_fetched_blocks().await.unwrap();

        assert_eq!(from_block_deposited, 0);
        assert_eq!(from_block_withdrawn, 0);

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn sets_and_gets_last_fetched_blocks() {
        let dir = "/tmp/nacho/tests/events_db/sets_and_gets_last_fetched_blocks";

        let mut events_db = EventsDb::new(dir).await.unwrap();

        let (from_block_deposited, from_block_withdrawn) =
            events_db.get_last_fetched_blocks().await.unwrap();

        assert_eq!(from_block_deposited, 0);
        assert_eq!(from_block_withdrawn, 0);

        events_db.set_last_fetched_blocks(45, 43).await.unwrap();

        let (from_block_deposited, from_block_withdrawn) =
            events_db.get_last_fetched_blocks().await.unwrap();

        assert_eq!(from_block_deposited, 45);
        assert_eq!(from_block_withdrawn, 43);

        remove_dir_all(dir).await.unwrap();
    }
}
