use std::vec;

use sui_rust_operator::{
    client,
    hook::{HookCaller, Target},
    keystore::Keystore,
    network,
};
#[tokio::main]
async fn main() {
    let store: Keystore = Keystore::default();
    let account = store.load_account(0).unwrap();
    let network = network::from_env();
    let client = client::default_client(&network);
    let mut hook: HookCaller<'_> = HookCaller::new(
        Target::new(
            String::from("0x2b79486eaddff4fe262519e409214faefde25bcef88bac4f61a799a3d2e490bc"),
            String::from("hello_world"),
            String::from("mint"),
        ),
        account,
        &client,
    );

    for _ in 1..=300 {
        hook.call(vec![], vec![]).await;
        hook.call(vec![], vec![]).await;
    }
}
