use ethers::prelude::*;
use std::sync::Arc;

abigen!(
    WETH_Contract,
    r#"[
        function name() view returns (string)
        function symbol() view returns (string)
    ]"#,
);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // 1. 使用不需要 API Key 的公共节点 (LlamaNodes)
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let client = Arc::new(provider);

    // 2. 核心诊断：打印 Chain ID (Arbitrum Sepolia 应该是 421614)
    println!("Connecting to RPC...");
    let chain_id = client.get_chainid().await?;
    println!("Connected! Chain ID: {}", chain_id); 

    let contract_address = "0x75faf114eafb1BDbe2F0316DF893fd58CE46AA4d".parse::<Address>()?;
    let contract = WETH_Contract::new(contract_address, client.clone());

    // 3. 检查合约代码是否存在 (防止连错链)
    let code = client.get_code(contract_address, None).await?;
    println!("Contract code length: {}", code.len());

    if code.is_empty() {
        println!("❌ Error: No contract found at this address. Check if you're on Sepolia!");
    } else {
        // 4. 调用合约方法
        let name = contract.name().call().await?;
        let symbol = contract.symbol().call().await?;

        println!("----------------------------------------------");
        println!("✅ Contract Name:   {}", name);
        println!("✅ Contract Symbol: {}", symbol);
        println!("----------------------------------------------");
    }

    Ok(())
}