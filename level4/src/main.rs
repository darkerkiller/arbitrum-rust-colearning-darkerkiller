use alloy::{
    network::EthereumWallet,
    primitives::{utils::parse_units, Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::{ensure, Result};
use std::{env, str::FromStr};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 读环境变量
    let rpc_url = env::var("SEPOLIA_RPC")?;
    let pk      = env::var("PRIVATE_KEY")?;
    ensure!(!pk.starts_with("0x0000"), "私钥不能是模板值，请替换！");

    // 2. 解析命令行参数
    let args: Vec<String> = env::args().skip(1).collect();
    ensure!(args.len() == 2, "用法: <to_address> <eth_amount>");
    let to: Address = args[0].parse()?;
    let value_wei: U256 = parse_units(&args[1], "ether")?.into();

    // 3. 构建 signer + provider
    let signer = PrivateKeySigner::from_str(&pk)?;
    let from = signer.address();
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect_http(rpc_url.parse()?);

    // 4. 实时 gas 价格 & 固定限额
    let gas_price = provider.get_gas_price().await?;
    let gas_price = gas_price * 110 / 100;
    let gas_limit: u64 = 21_000;

    // 5. 组装交易
    let tx = TransactionRequest::default()
        .from(from)
        .to(to)
        .value(value_wei)
        .gas_limit(gas_limit)
        .gas_price(gas_price);

    // 6. 签名并广播
    let pending = provider.send_transaction(tx).await?;
    println!("✅ 交易已提交，哈希：{}", pending.tx_hash());
    Ok(())
}