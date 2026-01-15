// src/main.rs
//! Arbitrum-Sepolia 简单合约读取示例
//! 读取 WETH9 合约的 name / symbol / decimals

use alloy::{
    network::EthereumWallet,
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use eyre::Result;
use std::str::FromStr;

// 1. WETH9 ABI（与链上字节码 100 % 匹配）
sol! {
    #[sol(rpc)]
    contract WETH9 {
        function name()     public view returns (string memory);
        function symbol()   public view returns (string memory);
        function decimals() public view returns (uint8);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 2. 连到 Arbitrum-Sepolia（去掉空格）
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc".parse()?;
    // 只读调用无需真实私钥，但 alloy 要求 wallet
    let dummy_pk = "0x0000000000000000000000000000000000000000000000000000000000000001";
    let signer = PrivateKeySigner::from_str(dummy_pk)?;
    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new().wallet(wallet).connect_http(rpc_url);

    // 3. WETH9 合约地址（Arbitrum-Sepolia 官方）
    let weth = WETH9::new(
        "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73".parse()?,
        provider,
    );

    // 4. 读取只读方法
    println!("name     : {}", weth.name().call().await?);
    println!("symbol   : {}", weth.symbol().call().await?);
    println!("decimals : {}", weth.decimals().call().await?);

    Ok(())
}