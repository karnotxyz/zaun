use async_trait::async_trait;
use ethers::prelude::H160;
use ethers::{
    prelude::abigen,
    providers::Middleware,
    types::{TransactionReceipt, U256},
};

use utils::errors::Error;

type Address = H160;

abigen!(
    StarknetEthBridge,
    "../../../build_artifacts/starkgate_82e651f/StarknetLegacyBridge.json",
);

#[async_trait]
pub trait StarknetEthBridgeTrait<M: Middleware> {
    async fn set_max_total_balance(
        &self,
        max_total_balance: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn set_max_deposit(
        &self,
        max_deposit: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn set_l2_token_bridge(
        &self,
        l2_token_bridge: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn deposit(
        &self,
        amount: U256,
        l2_recipient: U256,
        fee: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn withdraw(
        &self,
        amount: U256,
        l1_recipient: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>>;
    async fn identify(&self) -> Result<String, Error<M>>;
}

#[async_trait]
impl<T, M: Middleware> StarknetEthBridgeTrait<M> for T
where
    T: AsRef<StarknetEthBridge<M>> + Send + Sync,
{
    async fn set_max_total_balance(
        &self,
        max_total_balance: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .set_max_total_balance(max_total_balance)
            .send()
            .await?
            .confirmations(2)
            .await
            .map_err(Into::into)
    }

    async fn set_max_deposit(
        &self,
        max_deposit: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .set_max_deposit(max_deposit)
            .send()
            .await?
            .confirmations(2)
            .await
            .map_err(Into::into)
    }

    async fn set_l2_token_bridge(
        &self,
        l2_token_bridge: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .set_l2_token_bridge(l2_token_bridge)
            .send()
            .await?
            .confirmations(2)
            .await
            .map_err(Into::into)
    }

    async fn deposit(
        &self,
        amount: U256,
        l2_recipient: U256,
        fee: U256,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .deposit_with_amount(amount, l2_recipient)
            .value(fee)
            .send()
            .await?
            .confirmations(2)
            .await
            .map_err(Into::into)
    }

    async fn withdraw(
        &self,
        amount: U256,
        l1_recipient: Address,
    ) -> Result<Option<TransactionReceipt>, Error<M>> {
        self.as_ref()
            .withdraw_with_recipient(amount, l1_recipient)
            .send()
            .await?
            .confirmations(2)
            .await
            .map_err(Into::into)
    }

    async fn identify(&self) -> Result<String, Error<M>> {
        self.as_ref().identify().call().await.map_err(Into::into)
    }
}
