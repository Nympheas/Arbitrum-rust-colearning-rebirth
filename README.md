# Arbitrum-rust-colearning-rebirth
## 开发环境配置

### 1. 钱包配置
- **网络名称**: Arbitrum Sepolia
- **Chain ID**: 421614
- **状态**: 已成功连接并在 MetaMask 中切换至该网络。

![MetaMask配置截图](./images/截屏2026-01-10 下午12.19.13.png) 

### 2. 测试币领取
- **领取渠道**: Alchemy / Arbitrum Bridge
- **余额确认**: 账户已存入 0.1+ Sepolia ETH，用于部署合约及交互。
3.rust检查测试余额
/level2-balance-query/balance.rs
![测试结果截图](./images/余额.png) 

4.gas费预估
/level2-balance-query/gas-fee.rs
![测试结果截图](./images/gas费预估.png) 

5.转账
/level2-balance-query/transfer.rs
![测试结果截图](./images/转账.png) 
![测试结果截图](./images/浏览器.png) 

6.合约
/level2-balance-query/contract_read.rs
![测试结果截图](./images/合约.png)