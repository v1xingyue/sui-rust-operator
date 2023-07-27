use crate::{
    account::SuiAccount,
    client::Client,
    network::Network,
    print_beauty,
    utils::{self, current_timestamp},
};
use serde_json::Value;

struct UpdateGas {
    gas_object: String,
    expire_at: u64,
}

impl UpdateGas {
    pub fn expired(&self) -> bool {
        self.expire_at <= current_timestamp()
    }
}

impl Default for UpdateGas {
    fn default() -> Self {
        Self {
            gas_object: String::from(""),
            expire_at: 0,
        }
    }
}

pub struct HookCaller<'a> {
    target: Target,
    account: SuiAccount,
    client: &'a Client<'a>,
    gas: UpdateGas,
}

pub struct Target {
    package: String,
    module: String,
    fun_name: String,
    type_args: Vec<String>,
}

impl Target {
    pub fn new(package: String, module: String, fun_name: String, type_args: Vec<String>) -> Self {
        Self {
            package,
            module,
            fun_name,
            type_args,
        }
    }
}

impl Default for Target {
    fn default() -> Self {
        Self::new(String::from(""), String::from(""), String::from(""), vec![])
    }
}

impl<'a> HookCaller<'a> {
    pub fn get_network(&self) -> &Network {
        self.client.network
    }
    pub async fn call(&mut self, arguments: Vec<Value>) {
        self.update_gas().await;
        print_beauty!("you will call sui network : ");

        let result = self
            .client
            .unsafe_move_call(
                self.account.to_address(),
                self.target.package.to_string(),
                self.target.module.to_string(),
                self.target.fun_name.to_string(),
                self.target.type_args.to_owned(),
                arguments,
                self.gas.gas_object.to_string(),
                utils::ADVISE_GAS_BUDGET,
            )
            .await
            .unwrap();

        let effet = result
            .result
            .with_signed_execute(&self.client, &self.account)
            .await
            .unwrap();
        print_beauty!(
            "transaction goes : {}",
            self.client.network.transaction_link(&effet.result.digest)
        );
    }

    async fn update_gas(&mut self) {
        if self.gas.expired() {
            print_beauty!("now update gas!!!!");
            match self
                .client
                .get_avaliable_gas(self.account.to_address(), utils::ADVISE_GAS_BUDGET)
                .await
            {
                Err(err) => {
                    print_beauty!(
                        "gas error {} , with address: {}",
                        err,
                        self.account.to_address()
                    );
                    panic!();
                }
                Ok(gas_result) => {
                    self.gas.gas_object = gas_result.coin_object_id;
                    self.gas.expire_at = current_timestamp() + 120;
                }
            }
        }
    }

    pub fn new(target: Target, account: SuiAccount, client: &'a Client) -> Self {
        Self {
            target,
            account,
            client,
            gas: UpdateGas::default(),
        }
    }
}
