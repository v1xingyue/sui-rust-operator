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

1. 账号管理

* 创建账号，暂时只支持创建ed25519类型的账号。
* env 加载 指定env 的名字，从env 中获取账号的 secret_key 的 hex 编码
* 从keystore load。解析 ~/.sui/sui_config/sui.keystore 的内容，获得账号列表，通过序号获得地址
* dump_hex_seed 导出私钥

```rust
let store = Keystore::default();
let account = store.load_account(0).unwrap();
let new_account: SuiAccount = SuiAccount::new_account();

println!("Account: {} , seed : {} ", new_account, new_account.dump_hex_seed());
```

2.network 网络设置

* 通过env 制定network 类型，默认为 mainnet
* 获得network 对象以后可以获取 rpc gateway, faucet url

```rust
let network = network::default();
println!("network : {}", network);
println!("gateway : {}", network.get_gateway());
println!("faucet_url : {}", network.faucet_url());
```

3.获取交互client

* rust 和 sui 之间的交互通过client 完成
* payload 模块构建发送的json 信息
* response 模块中处理各种数据返回
* debug_client 执行的时候会打印payload 的详细内容

```rust
let myclient = client::debug_client(network);
```

4.读取操作

* get_object_id 根据object_id 获取struct

```rust
match myclient.get_object_id(&object_id).await {
    Ok(object) => {
        println!(
            "object detail : {}",
            serde_json::to_string_pretty(&object).unwrap()
        )
    }
    Err(err) => {
        println!("get object error : {}", err)
    }
}
```

* get_owned_objects 获得当前address 拥有的资源

```rust
match myclient.get_owned_objects(account.to_address(), query, None, None)
    .await
{
    Err(err) => {
        println!("err :{}", err)
    }
    Ok(result) => {
        println!("data found : {}", result.result.data.len());
    }
}
```

* get_all_balances 获取所有的余额

```rust
match myclient.get_all_balances(account.to_address()).await {
    Err(err) => {
        println!("err :{}", err)
    }
    Ok(result) => {
        for balance in result.result {
            println!(" {} => {}", balance.coin_type, balance.total_balance)
        }
    }
}
```

* get_all_coins 获得所有的coin
* get_gas_list 获得 SUI 的coin list

5.sui 链写操作

通过unsafe rpc 发送数据构建tx_bytes. account 使用对应的私钥，对数据完成签名，再次发送。

* unsafe_transfer_object 构建转移交易
* unsafe_move_call 构建合约调用
* unsafe_publish 构建发布交易

构建完成交易后，通过 account.sign_unsafe_transaciton 完成签名。最后使用 send_payload_effect 发送交易。

```rust
match myclient
    .unsafe_move_call(
        &account.to_address(),
        &"0x988fb71f38bb0323eeb5014c7a00e5988b047c09f39d58f157fc67d43ddfc091",
        &"hello_world",
        "mint",
        vec![],
        vec![],
        &"0x6abb224a86b8e571f221ea6bf6a5028923b29b13201a3c29f6fdaaaa3b4cbb97",
        3000_000,
    )
    .await
{
    Err(err) => {
        println!("{}", err)
    }
    Ok(data) => {
        let signed_payload = account.sign_unsafe_transaciton(data.result);

        let effet = myclient.send_payload_effect(&signed_payload).await.unwrap();

        println!("reuslt : {}", serde_json::to_string_pretty(&effet).unwrap());
        println!(
            "transaction link : {}",
            myclient
                .network
                .transaction_link(&effet.result.digest.to_string())
        )
    }
}
```

* 模块发布相关

需要使用sui 准备好编译完成的bytes 文件。

```shell
sui move build --dump-bytecode-as-base64 --path . > delopy.json
```

## 示例合约介绍

[playground](./playground/) 提供一个测试模块，已完成move_call 的相关功能

1. 合约 资源 ${address}::hello_world::HelloWorldObject
2. entry : mint, mint_to , update_text, destroy

## links

* [https://docs.rs/sui-rust-operator/latest/sui_rust_operator/](https://docs.rs/sui-rust-operator/latest/sui_rust_operator/)
* [https://docs.sui.io/testnet/build/rust-sdk](https://docs.sui.io/testnet/build/rust-sdk)
* [https://docs.sui.io/sui-jsonrpc#sui_devInspectTransactionBlock](https://docs.sui.io/sui-jsonrpc#sui_devInspectTransactionBlock)
* [https://github.com/coming-chat/go-sui-sdk](https://github.com/coming-chat/go-sui-sdk)

## TODO

1. 支持 其他的秘钥类型
2. 支持 批量交易 batch transaction
3. 多签交易
4. BCS support
5. sui hook caller 和web2 无缝融合 