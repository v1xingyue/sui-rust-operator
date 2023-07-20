use sui_rust_operator::{client, keystore::Keystore, network, utils};

#[tokio::main]
async fn main() {
    let network = network::default();
    let store = Keystore::default();
    let account = store.load_account(0).unwrap();

    println!("network : {}", network);
    println!("account : {}", account.to_address());
    println!("gateway : {}", network.get_gateway());

    let compiled = utils::CompiledModule::from_file("./playground/delopy.json".to_string());
    println!("modules : {:?}", compiled.modules);
    println!("dependencies : {:?}", compiled.dependencies);

    let myclient = client::debug_client(network);

    let pub_info = myclient
        .unsafe_publish(
            account.to_address(),
            compiled.modules,
            compiled.dependencies,
            "0x5a382376d9584cdb1d08595bec4d061cf25d95ffb6cf7fbbed475dd436bcfc0e".to_string(),
            3000_000,
        )
        .await
        .unwrap();
    println!("{}", pub_info.result.tx_bytes);

    let signed_payload = account.sign_unsafe_transaciton(pub_info.result);
    let effet = myclient.send_payload_effect(&signed_payload).await.unwrap();

    println!("reuslt : {}", serde_json::to_string_pretty(&effet).unwrap());
    println!(
        "transaction link : {}",
        myclient
            .network
            .transaction_link(&effet.result.digest.to_string())
    )
}
