use std::vec;
use sui_rust_operator::{client, keystore::Keystore, network, payload::Payload};

#[tokio::main]
async fn main() {
    let network: network::Network = network::default();
    println!("gateway is : {}", network.get_gateway());
    println!("network is : {}", network);

    let myclient = client::default_client(network);

    let store = Keystore::default();
    let account = store.load_account(0).unwrap();

    let payload = Payload::move_call(
        account.to_address(),
        "0x988fb71f38bb0323eeb5014c7a00e5988b047c09f39d58f157fc67d43ddfc091".to_string(),
        "hello_world".to_string(),
        "mint".to_string(),
        vec![],
        vec![],
        "0x6abb224a86b8e571f221ea6bf6a5028923b29b13201a3c29f6fdaaaa3b4cbb97".to_string(),
        3000_000,
    );

    match myclient.send_payload(&payload).await {
        Err(err) => {
            println!("{}", err)
        }
        Ok(result) => {
            println!("unsigned result : {}", result.text().await.unwrap())
        }
    };

    match myclient
        .unsafe_move_call(
            account.to_address(),
            "0x988fb71f38bb0323eeb5014c7a00e5988b047c09f39d58f157fc67d43ddfc091".to_string(),
            "hello_world".to_string(),
            "mint".to_string(),
            vec![],
            vec![],
            "0x6abb224a86b8e571f221ea6bf6a5028923b29b13201a3c29f6fdaaaa3b4cbb97".to_string(),
            3000_000,
        )
        .await
    {
        Err(err) => {
            println!("{}", err)
        }
        Ok(data) => {
            println!("json result : {}", serde_json::to_string(&data).unwrap())
        }
    }
}
