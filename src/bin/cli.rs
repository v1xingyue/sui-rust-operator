use sui_rust_operator::{client, keystore::Keystore, network, payload};

#[tokio::main]
async fn main() {
    let network = network::default();
    let store = Keystore::default();
    let myclient = client::debug_client(network);
    println!("account count : {}", store.len());
    for idx in (0..=store.len() - 1).collect::<Vec<usize>>() {
        match store.load_account(idx) {
            Err(err) => println!("account load error : {}", err),
            Ok(account) => {
                println!("account address : {}", account.to_address());
                myclient.get_faucet(account.to_address()).await;
                let query = payload::QueryOption::with_package(
                    "0xe818a1389a61d628b375d0c13f8f472b18e17bb9da5b2ea52c0f01653cd5ada6"
                        .to_string(),
                );

                match myclient
                    .get_owned_objects(account.to_address(), query, None, None)
                    .await
                {
                    Err(err) => {
                        println!("err :{}", err)
                    }
                    Ok(result) => {
                        println!("data found : {}", result.result.data.len());
                    }
                }

                // let query = payload::QueryOption::with_module(
                //     "0xe818a1389a61d628b375d0c13f8f472b18e17bb9da5b2ea52c0f01653cd5ada6"
                //         .to_string(),
                //     "hello_world".to_string(),
                // );

                // myclient
                //     .get_owned_objects(account.to_address(), query, None, None)
                //     .await
            }
        }
    }
}
