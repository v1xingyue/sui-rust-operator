use serde_json::Value;
use sui_rust_operator::{
    client,
    keystore::Keystore,
    network,
    payload::QueryOption,
    print_beauty,
    response::CoinInfo,
    utils::{self, ADVISE_GAS_BUDGET},
};

#[tokio::main]
async fn main() {
    let network = network::default();
    let store = Keystore::default();
    let account = store.load_account(0).unwrap();

    print_beauty!("network is : {}", network);
    print_beauty!("account is : {}", account.to_address());
    print_beauty!("network gateway is : {}", network.get_gateway());

    let client = client::default_client(network);
    for balance in client
        .get_all_balances(account.to_address())
        .await
        .unwrap()
        .result
    {
        print_beauty!(
            "balance {} , count :  {} , total: {}",
            balance.coin_type,
            balance.coin_object_count,
            balance.total_balance
        );
    }

    print_beauty!("loading gas list ...");

    let gas_result = client
        .get_avaliable_gas(account.to_address(), ADVISE_GAS_BUDGET)
        .await;
    let gas_object: CoinInfo = if gas_result.is_err() {
        print_beauty!("sorry,you don't have enough gas. Bye!!!");
        CoinInfo::default()
    } else {
        gas_result.unwrap()
    };

    print_beauty!("publish example module : ");

    let compiled = utils::CompiledModule::from_file("./playground/delopy.json".to_string());
    print_beauty!("modules : {:?}", compiled.modules);
    print_beauty!("dependencies : {:?}", compiled.dependencies);

    print_beauty!("now publish ...");
    let pub_info = client
        .unsafe_publish(
            account.to_address(),
            compiled.modules,
            compiled.dependencies,
            gas_object.coin_object_id.to_string(),
            ADVISE_GAS_BUDGET,
        )
        .await
        .unwrap();
    print_beauty!("transcation bytes : {}", pub_info.result.tx_bytes);
    print_beauty!("sign with account.");

    let effect = pub_info
        .result
        .with_signed_execute(&client, &account)
        .await
        .unwrap();

    print_beauty!(
        "publish reuslt : {}",
        serde_json::to_string_pretty(&effect).unwrap()
    );
    print_beauty!(
        "transaction link : {}",
        client
            .network
            .transaction_link(&effect.result.digest.to_string())
    );

    if let Some(effects) = &effect.result.effects {
        let imutables = effects.find_imutable_object();
        if imutables.len() > 0 {
            let package_id = imutables[0].to_string();
            print_beauty!("just publish one module : {}", package_id);
            match client
                .unsafe_move_call(
                    account.to_address(),
                    package_id.clone(),
                    String::from("hello_world"),
                    "mint".to_string(),
                    vec![],
                    vec![],
                    gas_object.coin_object_id.to_string(),
                    ADVISE_GAS_BUDGET,
                )
                .await
            {
                Ok(result) => {
                    let signed_payload = account.sign_unsafe_transaciton(&result.result);
                    let result = client.send_payload_effect(&signed_payload).await.unwrap();
                    print_beauty!("mint transaction done : {}", result.result.digest);

                    let struct_type =
                        format!("{}::hello_world::HelloWorldObject", package_id.clone());
                    let query = QueryOption::with_strutc_type(struct_type);

                    print_beauty!("query options : {}", serde_json::to_string(&query).unwrap());

                    let objects = client
                        .get_owned_objects(account.to_address(), query, None, None)
                        .await
                        .unwrap();
                    for object in objects.result.data {
                        print_beauty!("HelloWorldObject with id  : {}", object.data.object_id);

                        print_beauty!("now remove this object .");

                        match client
                            .unsafe_move_call(
                                account.to_address(),
                                package_id.clone(),
                                String::from("hello_world"),
                                "destroy".to_string(),
                                vec![],
                                vec![Value::String(object.data.object_id.to_string())],
                                gas_object.coin_object_id.to_string(),
                                ADVISE_GAS_BUDGET,
                            )
                            .await
                        {
                            Ok(data) => {
                                let effect = data
                                    .result
                                    .with_signed_execute(&client, &account)
                                    .await
                                    .unwrap();

                                print_beauty!(
                                    "destroy transaction link : {}",
                                    client
                                        .network
                                        .transaction_link(&effect.result.digest.to_string())
                                );
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
