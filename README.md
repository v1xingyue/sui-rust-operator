# sui-rust-operator

用rust 封装的一个和sui进行交互的组件库。

## 项目起因

sui 官方提供的sdk [https://docs.sui.io/testnet/build/rust-sdk](https://docs.sui.io/testnet/build/rust-sdk) 过于臃肿，使用不太方便。

## 基本思路

1. 使用 std ，reqwest 完成基本的rpc 的http 请求部分
2. 使用 serde serde_json 完成相关结构的封装
3. 按照 官方提供的 [RPC 文档](https://docs.sui.io/sui-jsonrpc)，和sui 进行交互。
4. 对于需要签名的请求，根据 unsafe 请求，构建未签名的 tx-bytes
5. 使用 ed25519-dalek 封装账号结构，对未签名的tx-bytes 完成签名，重新发送，完成执行

## 已实现的功能

1. 模块发布

需要使用sui 准备好编译完成的bytes 文件。

```shell
sui move build --dump-bytecode-as-base64 --path . > delopy.json
```




## 示例合约介绍

playground 提供一个测试模块，已完成move_call 的相关功能

1. 合约 资源 ${address}::hello_world::HelloWorldObject
2. entry : mint, mint_to , update_text, destroy

## 程序演示

## TODO 

