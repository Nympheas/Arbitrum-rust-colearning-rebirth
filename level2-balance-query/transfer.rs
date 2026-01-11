use ethers::prelude::*;
use ethers::utils::format_units;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // 0. 加载环境变量
    dotenv().ok();
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    let priv_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in .env");

    // 1. 设置连接与钱包
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let chain_id = provider.get_chainid().await?; // 动态获取链 ID
    
    // 将私钥解析为本地钱包，并关联链 ID 以防止重放攻击
    let wallet = priv_key.parse::<LocalWallet>()?.with_chain_id(chain_id.as_u64());
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // 2. 地址校验与配置
    let from_addr = client.address();
    let to_addr = "0x9aECfDA1C00E07F9e85e12371b154A025b101813".parse::<Address>()?; // 填入你的接收地址
    let amount = U256::from(1000000000000000u64); // 转账金额：0.001 ETH (单位 Wei)

    println!("Sender: {:?}", from_addr);
    println!("Receiver: {:?}", to_addr);

    // 3. Gas 费设置
let mut gas_price = client.get_gas_price().await?;
// 核心修复：提高出价以应对网络波动
gas_price = gas_price * 120 / 100; 

let gas_limit = U256::from(21000);
let estimate_fee = gas_price * gas_limit;
println!("Current Gas Price (with buffer): {} Gwei", format_units(gas_price, "gwei")?);
println!("Estimated Gas Fee: {} ETH", format_units(estimate_fee, "ether")?);

// 4. 构建交易请求
let tx = TransactionRequest::new()
    .to(to_addr)
    .value(amount)
    .gas(gas_limit)
    .gas_price(gas_price); // 确保这里使用了加价后的 gas_price

    // 5. 发送交易并等待确认
    println!("Sending transaction...");
    let pending_tx = client.send_transaction(tx, None).await?;
    
    // 获取交易哈希
    let tx_hash = pending_tx.tx_hash();
    println!("Transaction Sent! Hash: {:?}", tx_hash);

    // 等待收据确认 (通常 Arbitrum 很快)
    let receipt = pending_tx.confirmations(1).await?
        .ok_or_else(|| eyre::eyre!("Transaction receipt not found"))?;

    println!("Transaction confirmed in block: {:?}", receipt.block_number);
    println!("Explore: https://sepolia.arbiscan.io/tx/{:?}", tx_hash);

    Ok(())
}