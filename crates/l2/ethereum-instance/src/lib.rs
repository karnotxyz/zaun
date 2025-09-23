use ethers::abi::Tokenize;
use ethers::contract::{ContractError, ContractFactory, ContractInstance};
use ethers::prelude::SignerMiddleware;
use ethers::providers::{Http, Provider, ProviderError};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Bytes;
use ethers::utils::{Anvil, AnvilInstance};
use hex::FromHex;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use ethers::middleware::Middleware;

use ethers::types::U256;

/// Ethers library allows multiple signer backends and transports.
/// For simplicity we use local wallet (basically private key) and
/// HTTP transport in this crate.
pub use utils::LocalWalletSignerMiddleware;

/// Sandbox is typically used for E2E scenarios so we need to speed things up
const POLLING_INTERVAL_MS: u64 = 10;
const ANVIL_DEFAULT_ENDPOINT: &str = "http://127.0.0.1:8545";
const ANVIL_DEFAULT_CHAIN_ID: u64 = 31337;
const ANVIL_DEFAULT_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("['bytecode']['object'] is not a string")]
    BytecodeObject,
    #[error(transparent)]
    Hex(#[from] hex::FromHexError),
    #[error("Failed to parse URL")]
    UrlParser,
    #[error(transparent)]
    EthersContract(#[from] ContractError<LocalWalletSignerMiddleware>),
    #[error(transparent)]
    EthersProvider(#[from] ProviderError),
    #[error("Invalid contract build artifacts: missing field `{0}`")]
    ContractBuildArtifacts(&'static str),
}

/// A convenient wrapper over an already running or spawned Anvil local devnet or ethereum
#[allow(dead_code)]
pub struct EthereumClient {
    /// If initialized keeps an Anvil instance to properly shutdown it at the end
    client: Option<AnvilInstance>,
    /// Pre-configured local signer
    signer: Arc<LocalWalletSignerMiddleware>,
}

impl EthereumClient {
    /// Creates a new sandbox instance.
    /// Will try to attach to already running Anvil instance or custom rpc and private key provided to the function.
    /// if not provided any argument it will attack to a default anvil instance with default anvil params.
    pub fn attach(
        rpc_endpoint: Option<String>,
        priv_key: Option<String>,
        chain_id: Option<u64>,
    ) -> Result<Self, Error> {
        let rpc_endpoint = rpc_endpoint.unwrap_or_else(|| {
            std::env::var("ETH_RPC_ENDPOINT")
                .ok()
                .unwrap_or_else(|| ANVIL_DEFAULT_ENDPOINT.into())
        });

        let provider = Provider::<Http>::try_from(rpc_endpoint)
            .map_err(|_| Error::UrlParser)?
            .interval(Duration::from_millis(POLLING_INTERVAL_MS));

        let priv_key = priv_key.unwrap_or_else(|| ANVIL_DEFAULT_PRIVATE_KEY.to_owned());

        let wallet: LocalWallet = priv_key.parse().expect("Failed to parse private key");

        let chain_id = chain_id.unwrap_or(ANVIL_DEFAULT_CHAIN_ID);

        let client = SignerMiddleware::new(provider.clone(), wallet.with_chain_id(chain_id));

        Ok(Self {
            client: None,
            signer: Arc::new(client),
        })
    }

    /// Creates a new sandbox instance.
    /// A new Anvil instance will be spawned using binary located at:
    ///     - `anvil_path` parameter (if specified)
    ///     - ${ANVIL_PATH} environment variable (if set)
    ///     - ~/.foundry/bin/anvil (default)
    pub fn spawn(anvil_path: Option<PathBuf>) -> Self {
        let anvil_path: PathBuf = anvil_path.unwrap_or_else(|| {
            std::env::var("ANVIL_PATH")
                .map(Into::into)
                .ok()
                .unwrap_or_else(|| dirs::home_dir().unwrap().join(".foundry/bin/anvil"))
        });

        // Will panic if invalid path
        let anvil = Anvil::at(anvil_path).spawn();

        let provider = Provider::<Http>::try_from(anvil.endpoint())
            .expect("Failed to connect to Anvil")
            .interval(Duration::from_millis(POLLING_INTERVAL_MS));

        let wallet: LocalWallet = anvil.keys()[0].clone().into();
        let client =
            SignerMiddleware::new(provider.clone(), wallet.with_chain_id(anvil.chain_id()));

        Self {
            client: Some(anvil),
            signer: Arc::new(client),
        }
    }

    /// Returns local client configured for the running Anvil instance
    pub fn signer(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.signer.clone()
    }
}

/// Deploys new smart contract using:
///     - Forge build artifacts (JSON file contents)
///     - Constructor args (use () if no args expected)
pub async fn deploy_contract<T: Tokenize>(
    client: Arc<LocalWalletSignerMiddleware>,
    contract_build_artifacts: &str,
    contructor_args: T,
) -> Result<ContractInstance<Arc<LocalWalletSignerMiddleware>, LocalWalletSignerMiddleware>, Error>
{
    let (abi, bytecode) = {
        let mut artifacts: serde_json::Value = serde_json::from_str(contract_build_artifacts)?;

        let abi_value = artifacts
            .get_mut("abi")
            .ok_or_else(|| Error::ContractBuildArtifacts("abi"))?
            .take();
        let bytecode_value = artifacts
            .get_mut("bytecode")
            .ok_or_else(|| Error::ContractBuildArtifacts("bytecode"))?
            .get_mut("object")
            .ok_or_else(|| Error::ContractBuildArtifacts("bytecode.object"))?
            .take();

        let abi = serde_json::from_value(abi_value)?;
        let bytecode = Bytes::from_hex(bytecode_value.as_str().ok_or(Error::BytecodeObject)?)?;
        (abi, bytecode)
    };

    let factory = ContractFactory::new(abi, bytecode, client.clone());

    let mut deployer = factory.deploy(contructor_args)?;

    // Get current gas price from the network
    let current_gas_price = client.get_gas_price().await
      .unwrap();

    // Estimate gas using the client directly
    let estimated_gas = client.estimate_gas(&deployer.tx, None).await
      .unwrap();

    // Add 20% buffer to the estimated gas
    let gas_with_buffer = estimated_gas * 120 / 100;

    let total_cost_wei: U256 = gas_with_buffer * current_gas_price;

    // Convert to ETH for display (1 ETH = 10^18 wei)
    let total_cost_eth = total_cost_wei.as_u128() as f64 / 1e18;

    // Convert gas price to GWEI for display (1 GWEI = 10^9 wei)
    let gas_price_gwei = current_gas_price.as_u64() as f64 / 1e9;

    // Get network information
    let chain_id = client.get_chainid().await
      .unwrap();

    println!("Chain ID: {}", chain_id);

    // Add this before your deployment
    let balance = client.get_balance(client.address(), None).await
      .unwrap();


    let balance_eth = balance.as_u128() as f64 / 1e18;
    println!("Wallet address: {:?}", client.address());
    println!("Current balance: {:.6} ETH", balance_eth);
    println!("Required for transaction: {:.6} ETH", total_cost_eth);

    if balance < total_cost_wei {
        return Err(Error::EthersProvider(ProviderError::CustomError(
            format!("Insufficient balance. Have: {:.6} ETH, Need: {:.6} ETH", balance_eth, total_cost_eth)
        )));
    }

    println!("Estimated gas units: {}", estimated_gas);
    println!("Gas with buffer: {}", gas_with_buffer);
    println!("Current gas price: {:.2} GWEI", gas_price_gwei);
    println!("Total transaction cost: {:.6} ETH", total_cost_eth);

    // Set both gas limit and gas price
    deployer.tx.set_gas(gas_with_buffer);
    deployer.tx.set_gas_price(current_gas_price);

    Ok(deployer.send().await?)
}
