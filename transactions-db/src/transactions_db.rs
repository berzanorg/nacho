use std::{io::SeekFrom, path::Path};

use nacho_data_structures::TxStatus;
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};

use crate::TransactionsDbError;

type Result<T> = std::result::Result<T, TransactionsDbError>;

pub struct TransactionsDb {
    file: File,
}

impl TransactionsDb {
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let parent_dir_path = path
            .parent()
            .ok_or(TransactionsDbError::ParentDirectoryNotSpecified)?
            .to_string_lossy()
            .to_string();

        if parent_dir_path.is_empty() {
            return Err(TransactionsDbError::ParentDirectoryNotSpecified);
        }

        create_dir_all(parent_dir_path).await?;

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .await?;

        let mut db = Self { file };

        if db.file.metadata().await?.len() == 0 {
            db.set_tx_count(0).await?;
            db.set_executed_until(0).await?;
            db.set_proved_until(0).await?;
            db.set_settled_until(0).await?;
            db.set_merged_until(0).await?;
        }

        Ok(db)
    }

    pub async fn set_tx_count(&mut self, tx_count: u64) -> Result<()> {
        let padding = 0;

        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.write_u64(tx_count).await?;
        self.file.flush().await?;

        Ok(())
    }

    pub async fn set_executed_until(&mut self, until_tx_id: u64) -> Result<()> {
        let padding = 8;

        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.write_u64(until_tx_id).await?;
        self.file.flush().await?;

        Ok(())
    }

    pub async fn set_proved_until(&mut self, until_tx_id: u64) -> Result<()> {
        let padding = 16;

        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.write_u64(until_tx_id).await?;
        self.file.flush().await?;

        Ok(())
    }

    pub async fn set_settled_until(&mut self, until_tx_id: u64) -> Result<()> {
        let padding = 24;

        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.write_u64(until_tx_id).await?;
        self.file.flush().await?;

        Ok(())
    }

    pub async fn set_merged_until(&mut self, until_tx_id: u64) -> Result<()> {
        let padding = 32;

        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.write_u64(until_tx_id).await?;
        self.file.flush().await?;

        Ok(())
    }

    pub async fn get_tx_count(&mut self) -> Result<u64> {
        let padding = 0;

        self.file.seek(SeekFrom::Start(padding)).await?;

        let tx_count = self.file.read_u64().await?;

        Ok(tx_count)
    }

    pub async fn get_executed_until(&mut self) -> Result<u64> {
        let padding = 8;

        self.file.seek(SeekFrom::Start(padding)).await?;

        let tx_count = self.file.read_u64().await?;

        Ok(tx_count)
    }

    pub async fn get_proved_until(&mut self) -> Result<u64> {
        let padding = 16;

        self.file.seek(SeekFrom::Start(padding)).await?;

        let tx_count = self.file.read_u64().await?;

        Ok(tx_count)
    }

    pub async fn get_settled_until(&mut self) -> Result<u64> {
        let padding = 24;

        self.file.seek(SeekFrom::Start(padding)).await?;

        let tx_count = self.file.read_u64().await?;

        Ok(tx_count)
    }

    pub async fn get_merged_until(&mut self) -> Result<u64> {
        let padding = 32;

        self.file.seek(SeekFrom::Start(padding)).await?;

        let tx_count = self.file.read_u64().await?;

        Ok(tx_count)
    }

    pub async fn add_new_tx(&mut self) -> Result<u64> {
        let len = self.file.metadata().await?.len();
        let tx_count = self.get_tx_count().await?;

        let padding = 40 + (tx_count / 8);

        if len < padding + 1 {
            self.write_byte(padding, 0b0000_0000).await?;
        }

        self.set_tx_count(tx_count + 1).await?;

        let tx_id = tx_count;

        Ok(tx_id)
    }

    pub async fn set_rejected(&mut self, tx_id: u64) -> Result<()> {
        let tx_count = self.get_tx_count().await?;

        if tx_id >= tx_count {
            return Err(TransactionsDbError::TxDoesntExist);
        }

        let padding = 40 + (tx_id / 8);

        let byte = self.read_byte(padding).await?;

        let bit_position = tx_id % 8;

        let bitmask = 1 << bit_position;

        let new_byte = byte | bitmask;

        self.write_byte(padding, new_byte).await?;

        Ok(())
    }

    pub async fn get_status(&mut self, tx_id: u64) -> Result<TxStatus> {
        let tx_count = self.get_tx_count().await?;

        if tx_id >= tx_count {
            return Err(TransactionsDbError::TxDoesntExist);
        }

        let padding = 40 + (tx_id / 8);

        let byte = self.read_byte(padding).await?;

        let bit_position = tx_id % 8;

        let bitmask = 1 << bit_position;

        let is_rejected = (byte & bitmask) != 0;

        if is_rejected {
            return Ok(TxStatus::Rejected);
        }

        let settled_until = self.get_settled_until().await?;

        if tx_id < settled_until {
            return Ok(TxStatus::Settled);
        }

        let proved_until = self.get_proved_until().await?;

        if tx_id < proved_until {
            return Ok(TxStatus::Proved);
        }

        let executed_until = self.get_executed_until().await?;

        if tx_id < executed_until {
            return Ok(TxStatus::Executed);
        }

        Ok(TxStatus::Pending)
    }

    async fn write_byte(&mut self, padding: u64, byte: u8) -> Result<()> {
        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.write_u8(byte).await?;
        self.file.flush().await?;

        Ok(())
    }

    async fn read_byte(&mut self, padding: u64) -> Result<u8> {
        self.file.seek(SeekFrom::Start(padding)).await?;
        let byte = self.file.read_u8().await?;

        Ok(byte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::remove_file;

    #[tokio::test]
    async fn creates_tx_statuses_db_correctly() {
        let dir = "/tmp/nacho/tests/tx_statuses_db/creates_tx_statuses_db_correctly";

        let mut tx_db = TransactionsDb::new(dir).await.unwrap();

        let tx_count = tx_db.get_tx_count().await.unwrap();
        let executed_until = tx_db.get_executed_until().await.unwrap();
        let proved_until = tx_db.get_proved_until().await.unwrap();
        let settled_until = tx_db.get_settled_until().await.unwrap();
        let merged_until = tx_db.get_merged_until().await.unwrap();

        assert_eq!(tx_count, 0);
        assert_eq!(executed_until, 0);
        assert_eq!(proved_until, 0);
        assert_eq!(settled_until, 0);
        assert_eq!(merged_until, 0);

        remove_file(dir).await.unwrap();
    }

    #[tokio::test]
    async fn adds_new_txs() {
        let dir = "/tmp/nacho/tests/tx_statuses_db/adds_new_txs";

        let mut tx_db = TransactionsDb::new(dir).await.unwrap();

        let tx_count = tx_db.get_tx_count().await.unwrap();
        assert_eq!(tx_count, 0);

        tx_db.add_new_tx().await.unwrap();

        let tx_count = tx_db.get_tx_count().await.unwrap();
        assert_eq!(tx_count, 1);

        tx_db.add_new_tx().await.unwrap();

        let tx_count = tx_db.get_tx_count().await.unwrap();
        assert_eq!(tx_count, 2);

        tx_db.add_new_tx().await.unwrap();

        let tx_count = tx_db.get_tx_count().await.unwrap();
        assert_eq!(tx_count, 3);

        tx_db.add_new_tx().await.unwrap();

        let tx_count = tx_db.get_tx_count().await.unwrap();
        assert_eq!(tx_count, 4);

        tx_db.add_new_tx().await.unwrap();

        let tx_count = tx_db.get_tx_count().await.unwrap();
        assert_eq!(tx_count, 5);

        tx_db.add_new_tx().await.unwrap();

        let tx_count = tx_db.get_tx_count().await.unwrap();
        assert_eq!(tx_count, 6);

        tx_db.add_new_tx().await.unwrap();

        let tx_count = tx_db.get_tx_count().await.unwrap();
        assert_eq!(tx_count, 7);

        tx_db.add_new_tx().await.unwrap();

        let tx_count = tx_db.get_tx_count().await.unwrap();
        assert_eq!(tx_count, 8);

        tx_db.add_new_tx().await.unwrap();

        let tx_count = tx_db.get_tx_count().await.unwrap();
        assert_eq!(tx_count, 9);

        remove_file(dir).await.unwrap();
    }

    #[tokio::test]
    async fn gets_and_updates_tx_statuses_correctly() {
        let dir = "/tmp/nacho/tests/tx_statuses_db/gets_and_updates_tx_statuses_correctly";

        let mut tx_db = TransactionsDb::new(dir).await.unwrap();

        tx_db.add_new_tx().await.unwrap();
        tx_db.add_new_tx().await.unwrap();
        tx_db.add_new_tx().await.unwrap();
        tx_db.add_new_tx().await.unwrap();
        tx_db.add_new_tx().await.unwrap();
        tx_db.add_new_tx().await.unwrap();
        tx_db.add_new_tx().await.unwrap();
        tx_db.add_new_tx().await.unwrap();
        tx_db.add_new_tx().await.unwrap();
        tx_db.add_new_tx().await.unwrap();

        let tx_status = tx_db.get_status(5).await.unwrap();
        assert_eq!(tx_status, TxStatus::Pending);

        tx_db.set_executed_until(7).await.unwrap();
        tx_db.set_proved_until(4).await.unwrap();
        tx_db.set_settled_until(2).await.unwrap();
        tx_db.set_rejected(3).await.unwrap();

        let tx_status = tx_db.get_status(0).await.unwrap();
        assert_eq!(tx_status, TxStatus::Settled);

        let tx_status = tx_db.get_status(1).await.unwrap();
        assert_eq!(tx_status, TxStatus::Settled);

        let tx_status = tx_db.get_status(2).await.unwrap();
        assert_eq!(tx_status, TxStatus::Proved);

        let tx_status = tx_db.get_status(3).await.unwrap();
        assert_eq!(tx_status, TxStatus::Rejected);

        let tx_status = tx_db.get_status(4).await.unwrap();
        assert_eq!(tx_status, TxStatus::Executed);

        let tx_status = tx_db.get_status(5).await.unwrap();
        assert_eq!(tx_status, TxStatus::Executed);

        let tx_status = tx_db.get_status(6).await.unwrap();
        assert_eq!(tx_status, TxStatus::Executed);

        let tx_status = tx_db.get_status(7).await.unwrap();
        assert_eq!(tx_status, TxStatus::Pending);

        let tx_status = tx_db.get_status(8).await.unwrap();
        assert_eq!(tx_status, TxStatus::Pending);

        let tx_status = tx_db.get_status(9).await.unwrap();
        assert_eq!(tx_status, TxStatus::Pending);

        remove_file(dir).await.unwrap();
    }
}
