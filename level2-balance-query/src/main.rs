use alloy::{
    primitives::{address, utils::format_units},
    providers::{Provider, ProviderBuilder},
};
use std::str::FromStr;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let rpc_url = "https://arbitrum-sepolia-testnet.api.pocket.network".parse()?; // 可替换
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // 2. 待查询的地址
    let target = address!("0x2ed455eE8A8b2adad9Af66218de0d348561a45C7"); // 示例地址
    // 如果你想把地址当参数传进来：
    // let target = Address::from_str("0x3f1f78ed98cd180794f1346f5bd379d5ec47de90")?;

    // 3. 查余额（单位：wei）
    let balance_wei = provider.get_balance(target).await?;

    // 4. 转成 ETH 字符串（18 位小数）
    let balance_eth = format_units(balance_wei, "ether")?;

    println!("Address {} balance: {} ETH", target, balance_eth);
    Ok(())
}