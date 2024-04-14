use std::{io::SeekFrom, path::Path};

use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};

use crate::error::ProofsDbError;

pub struct ProofsDb {
    metadata_file: File,
}

type Result<T> = std::result::Result<T, ProofsDbError>;

impl ProofsDb {
    /// Creates a new instance of Proofs DB in the given path.
    pub async fn new(path: impl AsRef<Path>) -> Result<ProofsDb> {
        let path = path.as_ref();

        create_dir_all(path).await?;

        let mut metadata_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.join("metadata"))
            .await?;

        if metadata_file.metadata().await?.len() != 16 {
            let proof_count: u64 = 0;

            let merged_proof_count: u64 = 0;

            let buf: [u8; 16] = [proof_count.to_le_bytes(), merged_proof_count.to_le_bytes()]
                .concat()
                .try_into()
                .unwrap();

            metadata_file.seek(SeekFrom::Start(0)).await?;
            metadata_file.write_all(&buf).await?;
            metadata_file.flush().await?;
        }

        Ok(Self { metadata_file })
    }

    /// Increments the proof count stored on disk.
    pub async fn increment_proof_count(&mut self) -> Result<()> {
        let mut buf = [0u8; 8];
        self.metadata_file.seek(SeekFrom::Start(0)).await?;
        self.metadata_file.read_exact(&mut buf).await?;

        let mut proof_count = u64::from_le_bytes(buf);

        proof_count += 1;

        self.metadata_file.seek(SeekFrom::Start(0)).await?;
        self.metadata_file
            .write_all(&proof_count.to_le_bytes())
            .await?;
        self.metadata_file.flush().await?;

        Ok(())
    }

    /// Increments the merged proof count stored on disk.
    pub async fn increment_merged_proof_count(&mut self) -> Result<()> {
        let mut buf = [0u8; 8];
        self.metadata_file.seek(SeekFrom::Start(8)).await?;
        self.metadata_file.read_exact(&mut buf).await?;

        let mut merged_proof_count = u64::from_le_bytes(buf);

        merged_proof_count += 1;

        self.metadata_file.seek(SeekFrom::Start(8)).await?;
        self.metadata_file
            .write_all(&merged_proof_count.to_le_bytes())
            .await?;
        self.metadata_file.flush().await?;

        Ok(())
    }

    /// Returns the proof count stored on disk.
    pub async fn read_proof_count(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.metadata_file.seek(SeekFrom::Start(0)).await?;
        self.metadata_file.read_exact(&mut buf).await?;

        let proof_count = u64::from_le_bytes(buf);

        Ok(proof_count)
    }

    /// Returns the merged proof count stored on disk.
    pub async fn read_merged_proof_count(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.metadata_file.seek(SeekFrom::Start(8)).await?;
        self.metadata_file.read_exact(&mut buf).await?;

        let merged_proof_count = u64::from_le_bytes(buf);

        Ok(merged_proof_count)
    }
}

#[cfg(test)]
mod tests {
    use tokio::fs::remove_dir_all;

    use super::*;

    #[tokio::test]
    async fn creates_proofs_db() {
        let dir = "/tmp/nacho/tests/proof_db/creates_proofs_db";

        let proofs_db = ProofsDb::new(dir).await.unwrap();
        let _ = proofs_db;

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn initializes_metadata() {
        let dir = "/tmp/nacho/tests/proof_db/initializes_metadata";

        let mut proofs_db = ProofsDb::new(dir).await.unwrap();

        let proof_count = proofs_db.read_proof_count().await.unwrap();
        assert_eq!(proof_count, 0);

        let merged_proof_count = proofs_db.read_merged_proof_count().await.unwrap();
        assert_eq!(merged_proof_count, 0);

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn increments_and_reads_proof_count() {
        let dir = "/tmp/nacho/tests/proof_db/increments_and_reads_proof_count";

        let mut proofs_db = ProofsDb::new(dir).await.unwrap();

        let proof_count = proofs_db.read_proof_count().await.unwrap();
        assert_eq!(proof_count, 0);

        proofs_db.increment_proof_count().await.unwrap();

        let proof_count = proofs_db.read_proof_count().await.unwrap();
        assert_eq!(proof_count, 1);

        proofs_db.increment_proof_count().await.unwrap();

        let proof_count = proofs_db.read_proof_count().await.unwrap();
        assert_eq!(proof_count, 2);

        proofs_db.increment_proof_count().await.unwrap();

        let proof_count = proofs_db.read_proof_count().await.unwrap();
        assert_eq!(proof_count, 3);

        remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn increments_and_reads_merged_proof_count() {
        let dir = "/tmp/nacho/tests/proof_db/increments_and_reads_merged_proof_count";

        let mut proofs_db = ProofsDb::new(dir).await.unwrap();

        let proof_count = proofs_db.read_merged_proof_count().await.unwrap();
        assert_eq!(proof_count, 0);

        proofs_db.increment_merged_proof_count().await.unwrap();

        let proof_count = proofs_db.read_merged_proof_count().await.unwrap();
        assert_eq!(proof_count, 1);

        proofs_db.increment_merged_proof_count().await.unwrap();

        let proof_count = proofs_db.read_merged_proof_count().await.unwrap();
        assert_eq!(proof_count, 2);

        proofs_db.increment_merged_proof_count().await.unwrap();

        let proof_count = proofs_db.read_merged_proof_count().await.unwrap();
        assert_eq!(proof_count, 3);

        remove_dir_all(dir).await.unwrap();
    }
}
