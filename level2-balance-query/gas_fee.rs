use ethers::prelude::*;
use ethers::utils::format_units;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // 1. 配置 Arbitrum Sepolia RPC 节点
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let client = Arc::new(provider);

    println!("Fetching real-time gas data from Arbitrum Sepolia...");

    // 2. 动态获取实时 Gas 价格 (Gas Price)
    // 使用 provider.get_gas_price() 从节点获取当前的 Wei 价格
    let gas_price = client.get_gas_price().await?;
    
    // 3. 设置基础转账 Gas 限额 (Gas Limit)
    // 对于标准的 ETH 转账，行业通用值通常为 21,000 单位
    let gas_limit = U256::from(21000);

    // 4. 计算预估 Gas 费
    // 公式：Gas 费 = Gas 价格 × Gas 限额
    let estimated_fee_wei = gas_price * gas_limit;

    // 5. 格式化输出
    // 将 Wei 转换为 Gwei (方便查看价格) 和 ETH (方便查看最终费用)
    let gas_price_gwei = format_units(gas_price, "gwei")?;
    let estimated_fee_eth = format_units(estimated_fee_wei, "ether")?;

    println!("----------------------------------------------");
    println!("Real-time Gas Price: {} Gwei", gas_price_gwei);
    println!("Standard Gas Limit:  {}", gas_limit);
    println!("----------------------------------------------");
    println!(">> Estimated Total Gas Fee: {} ETH", estimated_fee_eth);
    println!("----------------------------------------------");

    Ok(())
}