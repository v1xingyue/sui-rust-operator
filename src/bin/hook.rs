use std::vec;
use sui_rust_operator::{
    client,
    hook::{HookCaller, Target},
    hookserver,
    keystore::Keystore,
    network, print_beauty, utils,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    const BIND_HOST: &str = "127.0.0.1";
    const BIND_PORT: u16 = 8080;

    let visit_url = format!("http://{}:{}", BIND_HOST, BIND_PORT);

    let store: Keystore = Keystore::default();
    let account = store.load_account(0).unwrap();
    let network = network::from_env();
    let client = client::default_client(network);
    let mut hook: HookCaller = HookCaller::new(
        Target::new(
            String::from("0x2b79486eaddff4fe262519e409214faefde25bcef88bac4f61a799a3d2e490bc"),
            String::from("hello_world"),
            String::from("mint"),
        ),
        account,
        client,
    );

    for _ in 1..=2 {
        hook.call(vec![], vec![]).await;
    }

    print_beauty!("now start an api hook server ... {}", visit_url);

    hookserver::start(BIND_HOST, BIND_PORT, hook).await
}
