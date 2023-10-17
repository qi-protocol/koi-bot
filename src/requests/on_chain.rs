use anyhow::anyhow;
use dotenv::dotenv;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{U256, U64},
};
use std::convert::TryFrom;

/// Type to query on chain info
#[derive(Debug, Clone)]
pub(crate) struct OnChainInfoQuery {
    provider: Provider<Http>,
}

impl OnChainInfoQuery {
    pub(crate) fn new(chain_id: u64) -> anyhow::Result<Self> {
        dotenv().ok();
        let provider = match chain_id {
            137 => {
                let rpc_url = std::env::var("POLYGON_RPC_URL")?;
                let provider = Provider::<Http>::try_from(rpc_url)?;
                log::info!("Connected to Polygon mainnet");
                provider
            }
            1 => {
                let rpc_url = std::env::var("ETH_RPC_URL")?;
                let provider = Provider::<Http>::try_from(rpc_url)?;
                log::info!("Connected to Ethereum mainnet");
                provider
            }
            _ => return Err(anyhow!("Unsupported chain id: {}", chain_id)),
        };
        Ok(Self { provider })
    }

    /// Gets the block number and gas fee
    pub(crate) async fn query_info(&self) -> anyhow::Result<(U64, U256)> {
        let block_number = self.provider.get_block_number().await?;

        let gas_price = self.provider.get_gas_price().await?;

        Ok((block_number, gas_price))
    }
}

/// Helper function to query the block number and gas fee from supported networks
pub(crate) async fn get_on_chain_info() -> anyhow::Result<String> {
    let eth_provider = OnChainInfoQuery::new(1).unwrap();
    let poly_provider = OnChainInfoQuery::new(137).unwrap();
    let (eth_block_number, eth_gas_price) = eth_provider.query_info().await.unwrap();
    let (poly_block_number, poly_gas_price) = poly_provider.query_info().await.unwrap();
    let eth_gas_price = eth_gas_price / 1_000_000_000u64;
    let poly_gas_price = poly_gas_price / 1_000_000_000u64;
    let message = format!(
        "*Ethereum*\n*Gas:* {} Gwei  ═  *Block:* {}\n\n*Polygon*\n*Gas:* {} Gwei  ═  *Block:* {}",
        eth_gas_price, eth_block_number, poly_gas_price, poly_block_number
    );

    Ok(message)
}
