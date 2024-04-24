pub struct HookData {
    account: String,
    balance: u64,
    network: crate::network::Network,
}

impl HookData {
    pub fn to_string(&self) -> String {
        String::from(format!(
            "account: {}, balance: {},network:{}",
            self.account,
            self.balance,
            self.network.to_string()
        ))
    }

    pub fn new(account: &str) -> Self {
        Self {
            account: String::from(account),
            balance: 0,
            network: crate::network::from_env(),
        }
    }
}
