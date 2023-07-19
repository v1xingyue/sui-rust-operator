use sui_rust_operator::{client, network};

#[tokio::main]
async fn main() {
    let network = network::default();
    let myclient: client::Client = client::default_client(network);
    let object_id =
        "0x945c5f2175e9814f46ee6bf4c39576434a645c98c77a54d62fd2a8b6ab9870e4".to_string();
    println!("gateway is : {}", myclient.network.get_gateway());
    println!("network is : {}", myclient.network);
    println!(
        "object link is : {} ",
        myclient.network.object_link(&object_id)
    );

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
}
