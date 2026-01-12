// Cargo.toml
// [dependencies]
// alloy = { version = "0.9", features = ["provider-http","reqwest"] }
// tokio  = { version = "1", features = ["full"] }
// eyre   = "0.6"

use alloy::{
    primitives::utils::format_units,
    providers::{Provider, ProviderBuilder},
};
use eyre::Result;

/// 返回 (gas_price_gwei, gas_price_eth, estimated_fee_eth)
async fn estimate_transfer_fee(rpc_url: &str) -> Result<(f64, f64, f64)> {
    const SIMPLE_TRANSFER_GAS: u128 = 21_000; // 行业通用值 [^12^]

    let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);

    // 1. 实时获取 gas 价格（单位 wei）
    let gas_price_wei = provider.get_gas_price().await?;

    // 2. 转成 gwei 和 ETH，方便阅读
    let gas_price_gwei: f64 = format_units(gas_price_wei, "gwei")?.parse()?;
    let gas_price_eth: f64 = format_units(gas_price_wei, "ether")?.parse()?;

    // 3. 计算预估手续费
    let fee_eth = gas_price_eth * SIMPLE_TRANSFER_GAS as f64;

    Ok((gas_price_gwei, gas_price_eth, fee_eth))
}

#[tokio::main]
async fn main() -> Result<()> {
    let rpc = "https://arbitrum-sepolia-testnet.api.pocket.network"; // Arbitrum Sepolia
    let (gwei, _eth, fee) = estimate_transfer_fee(rpc).await?;
    println!(
        "Arbitrum-Sepolia 实时 gas 价格：{:.4} gwei，\
         预估普通转账（21 K gas）手续费：{:.8} ETH",
        gwei, fee
    );
    Ok(())
}