use crate::utils::CustomErr;
use std::{env, error::Error, fmt::Display};

const NETWORK_ENV_NAME: &str = "network";

pub enum Network {
    Testnet,
    Mainnet,
    Devnet,
    Custom(String),
}

pub fn default() -> Network {
    from_env()
}

pub fn from_env() -> Network {
    if env::var_os(NETWORK_ENV_NAME).is_some() {
        if let Ok(value) = env::var(NETWORK_ENV_NAME) {
            return Network::from_name(value);
        }
    }
    Network::Mainnet
}

impl Network {
    pub fn from_name(name: String) -> Self {
        if name.eq("testnet") {
            return Network::Testnet;
        } else if name.eq("devnet") {
            return Network::Devnet;
        } else if name.eq("mainnet") {
            return Network::Mainnet;
        } else {
            return Network::Custom(name);
        }
    }

    pub fn get_gateway(&self) -> String {
        match self {
            Network::Testnet => String::from("https://fullnode.testnet.sui.io:443"),
            Network::Mainnet => String::from("https://fullnode.mainnet.sui.io:443"),
            Network::Devnet => String::from("https://fullnode.devnet.sui.io:443"),
            Network::Custom(url) => url.clone(),
        }
    }

    pub fn faucet_url(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Network::Devnet => Ok("https://faucet.devnet.sui.io/gas".to_string()),
            Network::Testnet => Ok("https://faucet.testnet.sui.io/gas".to_string()),
            Network::Mainnet => Err(Box::new(CustomErr::new("mainnet does not support faucet"))),
            Network::Custom(url) => Ok(format!("{}/gas", url)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Network::Testnet => String::from("testnet"),
            Network::Mainnet => String::from("mainnet"),
            Network::Devnet => String::from("devnet"),
            Network::Custom(url) => url.clone(),
        }
    }

    pub fn object_link(&self, object_id: &String) -> String {
        format!(
            "https://suiexplorer.com/object/{}?network={}",
            object_id,
            self.to_string()
        )
    }

    pub fn transaction_link(&self, digest: &String) -> String {
        format!(
            "https://suiexplorer.com/txblock/{}?network={}",
            digest,
            self.to_string()
        )
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.to_string(), self.get_gateway())
    }
}
