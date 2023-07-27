use serde_json::{json, Value};
use std::vec;
use sui_rust_operator::{client, keystore::Keystore, network, payload::Payload};

#[tokio::main]
async fn main() {
    let network: network::Network = network::default();
    println!("gateway is : {}", network.get_gateway());
    println!("network is : {}", network);
    let myclient = client::default_client(&network);

    let store = Keystore::default();
    let account = store.load_account(0).unwrap();

    let payload = Payload::move_call(
        account.to_address(),
        "0x66b54ca0bee63651789b2f53b99ce100adb4a6b5c35667eae4ac2279c9a9acb2".to_string(),
        "my_nft".to_string(),
        "mint".to_string(),
        vec![],
        vec![
            json!(String::from("hello").into_bytes()),
            json!(String::from("hello").into_bytes()),
        ],
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
            String::from("0x988fb71f38bb0323eeb5014c7a00e5988b047c09f39d58f157fc67d43ddfc091"),
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
            let signed_payload = account.sign_unsafe_transaciton(&data.result);

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
}
