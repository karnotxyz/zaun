use appchain_utils::invoke_contract;
use appchain_utils::LocalWalletSignerMiddleware;
use color_eyre::Result;
use starknet_core::types::{Felt, InvokeTransactionResult};

pub struct CoreContract {
    signer: LocalWalletSignerMiddleware,
    address: Felt,
}

impl CoreContract {
    pub fn new(address: Felt, signer: LocalWalletSignerMiddleware) -> Self {
        Self { signer, address }
    }

    pub async fn update_state(
        &self,
        snos_output: Vec<Felt>,
        layout_bridge_output: Vec<Felt>,
    ) -> Result<InvokeTransactionResult> {
        let mut calldata = Vec::with_capacity(snos_output.len() + layout_bridge_output.len() + 2);
        calldata.push(Felt::from(snos_output.len()));
        calldata.extend(snos_output);
        calldata.push(Felt::from(layout_bridge_output.len()));
        calldata.extend(layout_bridge_output);

        invoke_contract(&self.signer, self.address, "update_state", calldata).await
    }
}
