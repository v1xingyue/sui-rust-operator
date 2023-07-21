use crate::network::Network;
use crate::payload::{self, FilterOption, Payload};
use crate::response::{
    Balance, CoinInfo, CoinList, JsonResult, ObjectList, SimpleObject, TransactionEffect,
    UnsafeTransactionResult,
};
use crate::utils::CustomErr;
use reqwest::{self, Response};
use serde_json::{to_value, Value};
use std::error::Error;

pub struct Client {
    pub network: Network,
    debug: bool,
}

pub fn debug_client(network: Network) -> Client {
    Client {
        network,
        debug: true,
    }
}

pub fn default_client(network: Network) -> Client {
    Client {
        network,
        debug: false,
    }
}

impl Client {
    pub fn set_debug(&mut self) {
        self.debug = true;
    }

    pub async fn get_faucet(&self, recipient: String) {
        let info = payload::new_faucet(recipient);
        if self.debug {
            println!(
                "send payload : {}",
                serde_json::to_string_pretty(&info).unwrap()
            );
        }
        match self.network.faucet_url() {
            Err(err) => {
                println!("{}", err);
            }
            Ok(url) => {
                if self.debug {
                    println!("faucet url : {}", url);
                }
                let client = reqwest::Client::new();
                let resp = client
                    .post(url)
                    .header("Content-Type", "application/json")
                    .json(&info)
                    .send()
                    .await
                    .unwrap();
                println!("get faucet result : {}", resp.text().await.unwrap());
            }
        }
    }

    pub async fn send_payload(&self, payload: &Payload) -> Result<Response, Box<dyn Error>> {
        let client = reqwest::Client::new();
        if self.debug {
            println!(
                "send palyload : {}",
                serde_json::to_string(&payload).unwrap()
            );
        }
        match client
            .post(self.network.get_gateway())
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
        {
            Err(err) => Err(Box::new(err)),
            Ok(resp) => {
                if self.debug {
                    println!(
                        "status : {} , content-length: {} ",
                        resp.status(),
                        resp.content_length().unwrap(),
                    )
                }
                Ok(resp)
            }
        }
    }

    pub async fn send_payload_effect(
        &self,
        payload: &Payload,
    ) -> Result<JsonResult<TransactionEffect>, Box<dyn Error>> {
        match self.send_payload(payload).await {
            Ok(resp) => match resp.json::<JsonResult<TransactionEffect>>().await {
                Err(err) => Err(Box::new(err)),
                Ok(json_object) => Ok(json_object),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn unsafe_transfer_object(
        &self,
        owner_address: &str,
        object_id: &str,
        gas_object: &str,
        gas_budget: u64,
        to_address: &str,
    ) -> Result<JsonResult<UnsafeTransactionResult>, Box<dyn Error>> {
        let payload: Payload = Payload::unsafe_transfer_object(
            owner_address,
            object_id,
            gas_object,
            gas_budget,
            to_address,
        );
        let res = self.send_payload(&payload).await;
        match res {
            Err(err) => Err(err),
            Ok(resp) => match resp.json::<JsonResult<UnsafeTransactionResult>>().await {
                Err(err) => Err(Box::new(err)),
                Ok(json_object) => Ok(json_object),
            },
        }
    }

    pub async fn unsafe_move_call(
        &self,
        owner_address: &str,
        package_object_id: &str,
        module: &str,
        function: &str,
        type_arguments: Vec<Value>,
        arguments: Vec<Value>,
        gas_object: &str,
        gas_budget: u64,
    ) -> Result<JsonResult<UnsafeTransactionResult>, Box<dyn Error>> {
        let payload: Payload = Payload::move_call(
            owner_address,
            package_object_id,
            module,
            function,
            type_arguments,
            arguments,
            gas_object,
            gas_budget,
        );

        let res = self.send_payload(&payload).await;
        match res {
            Err(err) => Err(err),
            Ok(resp) => match resp.json::<JsonResult<UnsafeTransactionResult>>().await {
                Err(err) => Err(Box::new(err)),
                Ok(json_object) => Ok(json_object),
            },
        }
    }

    pub async fn unsafe_publish(
        &self,
        owner_address: String,
        modules: Vec<String>,
        dependencies: Vec<String>,
        gas_object: String,
        gas_budget: u64,
    ) -> Result<JsonResult<UnsafeTransactionResult>, Box<dyn Error>> {
        let payload: Payload =
            Payload::publish(owner_address, modules, dependencies, gas_object, gas_budget);

        let res = self.send_payload(&payload).await;
        match res {
            Err(err) => Err(err),
            Ok(resp) => match resp.json::<JsonResult<UnsafeTransactionResult>>().await {
                Err(err) => Err(Box::new(err)),
                Ok(json_object) => Ok(json_object),
            },
        }
    }

    pub async fn get_object_id(
        &self,
        object_id: &String,
    ) -> Result<JsonResult<SimpleObject>, Box<dyn Error>> {
        let payload: Payload = Payload::sui_get_object(object_id, &FilterOption::default());
        match self.send_payload(&payload).await {
            Err(err) => Err(err),
            Ok(resp) => match resp.json::<JsonResult<SimpleObject>>().await {
                Err(err) => Err(Box::new(err)),
                Ok(json_object) => Ok(json_object),
            },
        }
    }

    pub async fn get_owned_objects(
        &self,
        owner_address: String,
        query: payload::QueryOption,
        cursor: Option<String>,
        limit: Option<u64>,
    ) -> Result<JsonResult<ObjectList>, Box<dyn Error>> {
        let payload = Payload::build(
            String::from("suix_getOwnedObjects"),
            vec![
                Value::String(owner_address),
                to_value(query).unwrap(),
                match cursor {
                    None => Value::Null,
                    Some(v) => Value::String(v),
                },
                match limit {
                    None => Value::Null,
                    Some(v) => Value::from(v),
                },
            ],
        );
        match self.send_payload(&payload).await {
            Err(err) => Err(err),
            Ok(resp) => match resp.json::<JsonResult<ObjectList>>().await {
                Err(err) => Err(Box::new(err)),
                Ok(json_object) => Ok(json_object),
            },
        }
    }

    pub async fn get_all_balances(
        &self,
        owner_address: String,
    ) -> Result<JsonResult<Vec<Balance>>, Box<dyn Error>> {
        let payload = Payload::build(
            "suix_getAllBalances".to_string(),
            vec![Value::String(owner_address)],
        );

        match self.send_payload(&payload).await {
            Err(err) => Err(err),
            Ok(resp) => match resp.json::<JsonResult<Vec<Balance>>>().await {
                Err(err) => Err(Box::new(err)),
                Ok(json_object) => Ok(json_object),
            },
        }
    }

    pub async fn get_all_coins(
        &self,
        owner_address: String,
        coin_type: String,
        cursor: Option<String>,
        limit: Option<u64>,
    ) -> Result<JsonResult<CoinList>, Box<dyn Error>> {
        let payload = Payload::build(
            "suix_getCoins".to_string(),
            vec![
                Value::String(owner_address),
                Value::String(coin_type),
                match cursor {
                    None => Value::Null,
                    Some(v) => Value::String(v),
                },
                match limit {
                    None => Value::Null,
                    Some(v) => Value::from(v),
                },
            ],
        );

        match self.send_payload(&payload).await {
            Err(err) => Err(err),
            Ok(resp) => match resp.json::<JsonResult<CoinList>>().await {
                Err(err) => Err(Box::new(err)),
                Ok(json_object) => Ok(json_object),
            },
        }
    }

    pub async fn get_gas_list(
        &self,
        owner_address: String,
    ) -> Result<JsonResult<CoinList>, Box<dyn Error>> {
        self.get_all_coins(owner_address, "0x2::sui::SUI".to_string(), None, None)
            .await
    }

    pub async fn get_avaliable_gas(
        &self,
        owner_address: String,
        amount: u64,
    ) -> Result<CoinInfo, Box<dyn Error>> {
        match self.get_gas_list(owner_address).await {
            Err(err) => Err(err),
            Ok(data) => {
                for coin in data.result.data {
                    if coin.balance_u64() > amount {
                        return Ok(coin);
                    }
                }
                Err(CustomErr::new_box("coin not found"))
            }
        }
    }
}
