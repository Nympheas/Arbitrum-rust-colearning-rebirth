use ethers::prelude::*;
use ethers::utils::format_units;
use std::convert::TryFrom;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // 1. Arbitrum Sepolia 测试网 RPC 节点地址
    // 提示：可以从 Alchemy 或 Infura 获取，这里使用公共节点
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    
    // 2. 实例化 Provider
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let client = Arc::new(provider);

    // 3. 指定要查询的 Arbitrum 测试网地址 
    let target_address = "0x9aECfDA1C00E07F9e85e12371b154A025b101813".parse::<Address>()?;

    // 4. 查询余额 (返回的是 U256 格式的 Wei)
    let balance_wei = client.get_balance(target_address, None).await?;

    // 5. 将 Wei 转换为可读的 ETH 格式 (18位小数)
    // format_units 会返回字符串
    let balance_eth = format_units(balance_wei, "ether")?;

    println!("----------------------------------------------");
    println!("Arbitrum Sepolia Address: {:?}", target_address);
    println!("Balance: {} ETH", balance_eth);
    println!("----------------------------------------------");

    Ok(())
}
