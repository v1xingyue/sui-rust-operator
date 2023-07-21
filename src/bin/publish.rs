use sui_rust_operator::{
    client,
    keystore::Keystore,
    network, print_beauty,
    utils::{self, ADVISE_GAS_BUDGET},
};

#[tokio::main]
async fn main() {
    let network = network::default();
    let store = Keystore::default();
    let account = store.load_account(0).unwrap();

    print_beauty!("network : {}", network);
    print_beauty!("account : {}", account.to_address());
    print_beauty!("gateway : {}", network.get_gateway());

    print_beauty!("init done.",);

    let compiled = utils::CompiledModule::from_file("./playground/delopy.json".to_string());
    println!("modules : {:?}", compiled.modules);
    println!("dependencies : {:?}", compiled.dependencies);

    let myclient = client::debug_client(network);

    let gas_object = match myclient
        .get_avaliable_gas(account.to_address(), ADVISE_GAS_BUDGET)
        .await
    {
        Err(err) => {
            print_beauty!("no gas found : {}", err);
            None
        }
        Ok(msg) => Some(msg),
    };

    assert!(gas_object.is_some());

    let pub_info = myclient
        .unsafe_publish(
            account.to_address(),
            compiled.modules,
            compiled.dependencies,
            gas_object.unwrap().coin_object_id,
            ADVISE_GAS_BUDGET,
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
