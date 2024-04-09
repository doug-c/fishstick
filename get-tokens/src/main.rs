use ethers::abi::{parse_abi, ParamType};
use ethers::{
    providers::{Middleware, StreamExt, Provider, Ws},
    types::{H160, H256},
};
use eyre::Result;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = Provider::<Ws>::connect("wss://ethereum-sepolia.core.chainstack.com/ws/b470d6c37e069cff25641864159c3015").await?;
    let mut stream = provider.subscribe_blocks().await?.take(1);
    while let Some(block) = stream.next().await {
        dbg!("{:?}", block.hash);
    }

    Ok(())
}

