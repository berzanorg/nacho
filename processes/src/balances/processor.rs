use nacho_balances_db::{DoubleBalanceWitness, SingleBalanceWitness};
use nacho_data_structures::{Address, U256};
use tokio::sync::{mpsc, oneshot};

use super::Request;

#[derive(Clone, Copy, Debug)]
pub struct Processor {
    pub(crate) sender: &'static mpsc::Sender<Request>,
}

impl Processor {
    pub async fn get_token_amount(&self, address: Address, token_id: U256) -> Option<u64> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetTokenAmount {
                sender: oneshot_sender,
                owner: address,
                token_id,
            })
            .await
            .ok()?;

        let balance = oneshor_receiver.await.ok()?;

        balance
    }

    pub async fn get_balances(&self, address: Address) -> Option<Vec<(U256, u64)>> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetBalances {
                sender: oneshot_sender,
                owner: address,
            })
            .await
            .ok()?;

        let balances = oneshor_receiver.await.ok()?;

        balances
    }

    pub async fn get_single_witness(
        &self,
        address: Address,
        token_id: U256,
    ) -> Option<SingleBalanceWitness> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetSingleWitness {
                sender: oneshot_sender,
                owner: address,
                token_id,
            })
            .await
            .ok()?;

        let single_witness = oneshor_receiver.await.ok()?;

        single_witness
    }

    pub async fn get_double_witness(
        &self,
        address: Address,
        base_token_id: U256,
        quote_token_id: U256,
    ) -> Option<DoubleBalanceWitness> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetDoubleWitness {
                sender: oneshot_sender,
                owner: address,
                base_token_id,
                quote_token_id,
            })
            .await
            .ok()?;

        let double_witness = oneshor_receiver.await.ok()?;

        double_witness
    }

    pub async fn get_new_witness(&self) -> Option<SingleBalanceWitness> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetNewWitness {
                sender: oneshot_sender,
            })
            .await
            .ok()?;

        let single_witness = oneshor_receiver.await.ok()?;

        single_witness
    }

    pub async fn push_balance(
        &self,
        owner: Address,
        token_id: U256,
        token_amount: u64,
    ) -> Option<()> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::PushBalance {
                sender: oneshot_sender,
                owner,
                token_id,
                token_amount,
            })
            .await
            .ok()?;

        let result = oneshor_receiver.await.ok()?;

        result
    }

    pub async fn update_balance(
        &self,
        owner: Address,
        token_id: U256,
        token_amount: u64,
    ) -> Option<()> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::UpdateBalance {
                sender: oneshot_sender,
                owner,
                token_id,
                token_amount,
            })
            .await
            .ok()?;

        let result = oneshor_receiver.await.ok()?;

        result
    }

    pub async fn push_leaf(&self, owner: Address, token_id: U256, token_amount: u64) -> Option<()> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::PushLeaf {
                sender: oneshot_sender,
                owner,
                token_id,
                token_amount,
            })
            .await
            .ok()?;

        let result = oneshor_receiver.await.ok()?;

        result
    }

    pub async fn update_leaf(
        &self,
        owner: Address,
        token_id: U256,
        token_amount: u64,
    ) -> Option<()> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::UpdateLeaf {
                sender: oneshot_sender,
                owner,
                token_id,
                token_amount,
            })
            .await
            .ok()?;

        let result = oneshor_receiver.await.ok()?;

        result
    }
}
