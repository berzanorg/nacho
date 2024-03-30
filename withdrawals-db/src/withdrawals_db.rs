use std::{array, io::SeekFrom, path::Path};

use nacho_data_structures::{Field, FromBytes, ToBytes, U256};
use nacho_merkle_tree::Sibling;
use nacho_poseidon_hash::{create_poseidon_hasher, poseidon_hash, PoseidonHasher};
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
};

use crate::{
    constants::{
        WITHDRAWALS_TREE_MAX_LEAF_INDEX, WITHDRAWALS_TREE_ROOT_PADDING,
        WITHDRAWALS_TREE_SIZE_IN_BYTES,
    },
    single_withdrawal_witness::SingleWithdrawalWitness,
    WithdrawalsDbError, WITHDRAWALS_TREE_HEIGHT,
};

pub struct WithdrawalsDb {
    file: File,
    hasher: PoseidonHasher,
}

type Result<T> = std::result::Result<T, WithdrawalsDbError>;

impl WithdrawalsDb {
    pub async fn new(path: impl AsRef<Path>) -> Result<WithdrawalsDb> {
        let path = path.as_ref();

        let parent_dir_path = path
            .parent()
            .ok_or(WithdrawalsDbError::ParentDirectoryNotSpecified)?
            .to_string_lossy()
            .to_string();

        if parent_dir_path.is_empty() {
            return Err(WithdrawalsDbError::ParentDirectoryNotSpecified);
        }

        create_dir_all(parent_dir_path).await?;

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .await?;

        let mut hasher = create_poseidon_hasher();

        if file.metadata().await?.len() != WITHDRAWALS_TREE_SIZE_IN_BYTES {
            let mut current_zero: Field = 0.into();
            let mut padding = 0;

            for i in 0..WITHDRAWALS_TREE_HEIGHT {
                let leaves_count = 2_usize.pow(WITHDRAWALS_TREE_HEIGHT as u32 - i as u32 - 1);

                let bytes_repr = U256::from(current_zero).to_bytes();

                let buf: Vec<_> = vec![bytes_repr; leaves_count]
                    .into_iter()
                    .flatten()
                    .collect();

                file.seek(SeekFrom::Start(padding)).await?;
                file.write(&buf).await?;
                file.flush().await?;

                current_zero = poseidon_hash(&mut hasher, &[current_zero, current_zero]);
                padding += 32 * leaves_count as u64;
            }
        }

        Ok(Self { file, hasher })
    }

    pub async fn set(&mut self, leaf_index: u64, value: Field) -> Result<()> {
        if leaf_index > WITHDRAWALS_TREE_MAX_LEAF_INDEX {
            return Err(WithdrawalsDbError::LeafIndexesExceeded);
        }

        let mut cumulative_padding = 0;
        let mut current_leaf_index = leaf_index;
        let mut current_value = value;

        for i in 0..WITHDRAWALS_TREE_HEIGHT {
            let leaves_count = 2_usize.pow(WITHDRAWALS_TREE_HEIGHT as u32 - i as u32 - 1);
            let current_padding = 32 * current_leaf_index;

            let buf_repr = U256::from(current_value).to_bytes();

            self.file
                .seek(SeekFrom::Start(cumulative_padding + current_padding))
                .await?;
            self.file.write(&buf_repr).await?;
            self.file.flush().await?;

            if i == WITHDRAWALS_TREE_HEIGHT - 1 {
                break;
            }

            let sibling_is_left = current_leaf_index % 2 == 1;

            let sibling_leaf_index = if sibling_is_left {
                current_leaf_index - 1
            } else {
                current_leaf_index + 1
            };

            let sibling_padding = 32 * sibling_leaf_index;

            let mut sibling_buf = [0u8; 32];

            self.file
                .seek(SeekFrom::Start(cumulative_padding + sibling_padding))
                .await?;
            self.file.read_exact(&mut sibling_buf).await?;

            let sibling_value: Field = (&U256::from_bytes(sibling_buf)).into();

            let (left_value, right_value) = if sibling_is_left {
                (sibling_value, current_value)
            } else {
                (current_value, sibling_value)
            };

            cumulative_padding += 32 * leaves_count as u64;
            current_leaf_index = current_leaf_index / 2;
            current_value = poseidon_hash(&mut self.hasher, &[left_value, right_value]);
        }

        Ok(())
    }

    pub async fn get(&mut self, leaf_index: u64) -> Result<Field> {
        if leaf_index > WITHDRAWALS_TREE_MAX_LEAF_INDEX {
            return Err(WithdrawalsDbError::LeafIndexesExceeded);
        }

        let padding = 32 * leaf_index;

        let mut buf = [0u8; 32];

        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.read_exact(&mut buf).await?;

        let value = (&U256(buf)).into();

        Ok(value)
    }

    pub async fn root(&mut self) -> Result<Field> {
        let padding = WITHDRAWALS_TREE_ROOT_PADDING;

        let mut buf = [0u8; 32];

        self.file.seek(SeekFrom::Start(padding)).await?;
        self.file.read_exact(&mut buf).await?;

        let value = (&U256(buf)).into();

        Ok(value)
    }

    pub async fn get_witness(&mut self, leaf_index: u64) -> Result<SingleWithdrawalWitness> {
        if leaf_index > WITHDRAWALS_TREE_MAX_LEAF_INDEX {
            return Err(WithdrawalsDbError::LeafIndexesExceeded);
        }

        let mut siblings = array::from_fn(|_| Sibling::default());

        let mut current_leaf_index = leaf_index;
        let mut cumulative_padding = 0;

        for i in 0..WITHDRAWALS_TREE_HEIGHT - 1 {
            let leaves_count = 2_usize.pow(WITHDRAWALS_TREE_HEIGHT as u32 - i as u32 - 1);

            let sibling_is_left = current_leaf_index % 2 == 1;

            let sibling_leaf_index = if sibling_is_left {
                current_leaf_index - 1
            } else {
                current_leaf_index + 1
            };

            let sibling_padding = 32 * sibling_leaf_index;

            let mut sibling_buf = [0u8; 32];

            self.file
                .seek(SeekFrom::Start(cumulative_padding + sibling_padding))
                .await?;
            self.file.read_exact(&mut sibling_buf).await?;

            let sibling_value = U256::from_bytes(sibling_buf);

            siblings[i] = Sibling {
                value: sibling_value,
                is_left: sibling_is_left,
            };

            current_leaf_index = current_leaf_index / 2;
            cumulative_padding += 32 * leaves_count as u64;
        }

        Ok(SingleWithdrawalWitness { siblings })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::remove_file;

    #[tokio::test]
    async fn creates_withdrawals_db() {
        let dir = "/tmp/nacho/tests/withdrawals_db/creates_withdrawals_db";

        let withdrawals_db = WithdrawalsDb::new(dir).await.unwrap();

        let _ = withdrawals_db;

        remove_file(dir).await.unwrap()
    }

    #[tokio::test]
    async fn sets_and_gets_leaves() {
        let dir = "/tmp/nacho/tests/withdrawals_db/sets_and_gets_leaves";

        let mut withdrawals_db = WithdrawalsDb::new(dir).await.unwrap();

        withdrawals_db.set(0, 12.into()).await.unwrap();
        withdrawals_db.set(45, 7.into()).await.unwrap();
        withdrawals_db.set(156, 267.into()).await.unwrap();

        assert_eq!(withdrawals_db.get(0).await.unwrap(), 12.into());
        assert_eq!(withdrawals_db.get(1).await.unwrap(), 0.into());
        assert_eq!(withdrawals_db.get(45).await.unwrap(), 7.into());
        assert_eq!(withdrawals_db.get(156).await.unwrap(), 267.into());
        assert_eq!(withdrawals_db.get(99999).await.unwrap(), 0.into());

        remove_file(dir).await.unwrap()
    }

    #[tokio::test]
    async fn updates_root() {
        let dir = "/tmp/nacho/tests/withdrawals_db/updates_root";

        let mut withdrawals_db = WithdrawalsDb::new(dir).await.unwrap();

        let root = withdrawals_db.root().await.unwrap();

        assert_eq!(
            root,
            "25436453236035485996795240493313170211557120058262356001829805101279552630634"
                .parse()
                .unwrap()
        );

        withdrawals_db.set(0, 435.into()).await.unwrap();

        let root = withdrawals_db.root().await.unwrap();

        assert_eq!(
            root,
            "9483955031203401160713579395725671566345301961932530213516288103888164633380"
                .parse()
                .unwrap()
        );

        withdrawals_db.set(85, 685.into()).await.unwrap();

        let root = withdrawals_db.root().await.unwrap();

        assert_eq!(
            root,
            "17336108694741431772855439559487367699612615888710642999387305390429680201132"
                .parse()
                .unwrap()
        );

        remove_file(dir).await.unwrap()
    }

    #[tokio::test]
    async fn gets_witnesses() {
        let dir = "/tmp/nacho/tests/withdrawals_db/gets_witnesses";

        let mut withdrawals_db = WithdrawalsDb::new(dir).await.unwrap();

        let witness = withdrawals_db.get_witness(0).await.unwrap();

        assert_eq!(
            witness
                .siblings
                .clone()
                .map(|sibling| Field::from(&sibling.value)),
            [
                "0".parse().unwrap(),
                "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                    .parse()
                    .unwrap(),
                "2447983280988565496525732146838829227220882878955914181821218085513143393976"
                    .parse()
                    .unwrap(),
                "544619463418997333856881110951498501703454628897449993518845662251180546746"
                    .parse()
                    .unwrap(),
                "20468198949394563802460512965219839480612000520504690501918527632215047268421"
                    .parse()
                    .unwrap(),
                "16556836945641263257329399459944072214107361158323688202689648863681494824075"
                    .parse()
                    .unwrap(),
                "15433636137932294330522564897643259724602670702144398296133714241278885195605"
                    .parse()
                    .unwrap(),
                "14472842460125086645444909368571209079194991627904749620726822601198914470820"
                    .parse()
                    .unwrap(),
                "21614416876217972474084851109688329000791937035724439964738173004620435920527"
                    .parse()
                    .unwrap(),
                "23396673455667782815008357063662227432928854130481827049177088579579506912772"
                    .parse()
                    .unwrap(),
                "16799216270319797546551726730220821530700130944535729528697378284641302758053"
                    .parse()
                    .unwrap(),
                "13496994890596928254174830027007320908142597452643688487140586680795895052589"
                    .parse()
                    .unwrap(),
                "3136367688481366987314253891173247447839122679172869317967104414474412425595"
                    .parse()
                    .unwrap(),
                "16414894720763442886261603851925762864778244212151669304308726942427436045416"
                    .parse()
                    .unwrap(),
                "22589430138891598861557640031858114956845676321373998952831060515802332123931"
                    .parse()
                    .unwrap(),
                "5791459643284782105200605043590392479676954402958680213128780189628932971164"
                    .parse()
                    .unwrap(),
                "16510281280427274356185141347114409143460588060107208117935011691496157196057"
                    .parse()
                    .unwrap(),
                "14486316384664676462434146886055824678546254125476991527791532204933765704550"
                    .parse()
                    .unwrap()
            ]
        );

        assert_eq!(
            witness.siblings.map(|sibling| sibling.is_left),
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false
            ]
        );

        withdrawals_db.set(0, 435.into()).await.unwrap();

        let witness = withdrawals_db.get_witness(0).await.unwrap();

        assert_eq!(
            witness
                .siblings
                .clone()
                .map(|sibling| Field::from(&sibling.value)),
            [
                "0".parse().unwrap(),
                "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                    .parse()
                    .unwrap(),
                "2447983280988565496525732146838829227220882878955914181821218085513143393976"
                    .parse()
                    .unwrap(),
                "544619463418997333856881110951498501703454628897449993518845662251180546746"
                    .parse()
                    .unwrap(),
                "20468198949394563802460512965219839480612000520504690501918527632215047268421"
                    .parse()
                    .unwrap(),
                "16556836945641263257329399459944072214107361158323688202689648863681494824075"
                    .parse()
                    .unwrap(),
                "15433636137932294330522564897643259724602670702144398296133714241278885195605"
                    .parse()
                    .unwrap(),
                "14472842460125086645444909368571209079194991627904749620726822601198914470820"
                    .parse()
                    .unwrap(),
                "21614416876217972474084851109688329000791937035724439964738173004620435920527"
                    .parse()
                    .unwrap(),
                "23396673455667782815008357063662227432928854130481827049177088579579506912772"
                    .parse()
                    .unwrap(),
                "16799216270319797546551726730220821530700130944535729528697378284641302758053"
                    .parse()
                    .unwrap(),
                "13496994890596928254174830027007320908142597452643688487140586680795895052589"
                    .parse()
                    .unwrap(),
                "3136367688481366987314253891173247447839122679172869317967104414474412425595"
                    .parse()
                    .unwrap(),
                "16414894720763442886261603851925762864778244212151669304308726942427436045416"
                    .parse()
                    .unwrap(),
                "22589430138891598861557640031858114956845676321373998952831060515802332123931"
                    .parse()
                    .unwrap(),
                "5791459643284782105200605043590392479676954402958680213128780189628932971164"
                    .parse()
                    .unwrap(),
                "16510281280427274356185141347114409143460588060107208117935011691496157196057"
                    .parse()
                    .unwrap(),
                "14486316384664676462434146886055824678546254125476991527791532204933765704550"
                    .parse()
                    .unwrap()
            ]
        );

        assert_eq!(
            witness.siblings.map(|sibling| sibling.is_left),
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false
            ]
        );

        withdrawals_db.set(85, 685.into()).await.unwrap();

        let witness = withdrawals_db.get_witness(42).await.unwrap();

        assert_eq!(
            witness
                .siblings
                .clone()
                .map(|sibling| Field::from(&sibling.value)),
            [
                "0".parse().unwrap(),
                "21565680844461314807147611702860246336805372493508489110556896454939225549736"
                    .parse()
                    .unwrap(),
                "2447983280988565496525732146838829227220882878955914181821218085513143393976"
                    .parse()
                    .unwrap(),
                "544619463418997333856881110951498501703454628897449993518845662251180546746"
                    .parse()
                    .unwrap(),
                "20468198949394563802460512965219839480612000520504690501918527632215047268421"
                    .parse()
                    .unwrap(),
                "4634928718464554397739609044028692299605741010323962508454757219290440362024"
                    .parse()
                    .unwrap(),
                "1250935003133161145562566742231784049374770790421194102685733568202465496596"
                    .parse()
                    .unwrap(),
                "14472842460125086645444909368571209079194991627904749620726822601198914470820"
                    .parse()
                    .unwrap(),
                "21614416876217972474084851109688329000791937035724439964738173004620435920527"
                    .parse()
                    .unwrap(),
                "23396673455667782815008357063662227432928854130481827049177088579579506912772"
                    .parse()
                    .unwrap(),
                "16799216270319797546551726730220821530700130944535729528697378284641302758053"
                    .parse()
                    .unwrap(),
                "13496994890596928254174830027007320908142597452643688487140586680795895052589"
                    .parse()
                    .unwrap(),
                "3136367688481366987314253891173247447839122679172869317967104414474412425595"
                    .parse()
                    .unwrap(),
                "16414894720763442886261603851925762864778244212151669304308726942427436045416"
                    .parse()
                    .unwrap(),
                "22589430138891598861557640031858114956845676321373998952831060515802332123931"
                    .parse()
                    .unwrap(),
                "5791459643284782105200605043590392479676954402958680213128780189628932971164"
                    .parse()
                    .unwrap(),
                "16510281280427274356185141347114409143460588060107208117935011691496157196057"
                    .parse()
                    .unwrap(),
                "14486316384664676462434146886055824678546254125476991527791532204933765704550"
                    .parse()
                    .unwrap()
            ]
        );

        assert_eq!(
            witness.siblings.map(|sibling| sibling.is_left),
            [
                false, true, false, true, false, true, false, false, false, false, false, false,
                false, false, false, false, false, false
            ]
        );

        remove_file(dir).await.unwrap()
    }
}
