use crate::utils::current_timestamp;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_value, Value};
use std::fmt::Display;
use std::vec::Vec;

pub const VERSION: &str = "0.0.0";
pub const PAYLOAD_JSONRPC_VERSION: &str = &"2.0";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransactionBlockResponseOptions {
    show_input: bool,
    show_raw_input: bool,
    show_effects: bool,
    show_events: bool,
    show_object_changes: bool,
    show_balance_changes: bool,
}

impl TransactionBlockResponseOptions {
    pub fn new(
        show_input: bool,
        show_raw_input: bool,
        show_effects: bool,
        show_events: bool,
        show_object_changes: bool,
        show_balance_changes: bool,
    ) -> Self {
        Self {
            show_input,
            show_raw_input,
            show_effects,
            show_events,
            show_object_changes,
            show_balance_changes,
        }
    }

    pub fn default_options() -> Self {
        Self::new(true, true, true, true, true, true)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterOption {
    show_type: bool,
    show_owner: bool,
    show_previous_transaction: bool,
    show_display: bool,
    show_content: bool,
    show_bcs: bool,
    show_storage_rebate: bool,
}

impl Default for FilterOption {
    fn default() -> Self {
        Self::new(true, true, true, true, true, true, false)
    }
}

impl FilterOption {
    pub fn new(
        show_type: bool,
        show_owner: bool,
        show_previous_transaction: bool,
        show_display: bool,
        show_content: bool,
        show_bcs: bool,
        show_storage_rebate: bool,
    ) -> Self {
        Self {
            show_type,
            show_owner,
            show_previous_transaction,
            show_display,
            show_content,
            show_bcs,
            show_storage_rebate,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Vec<Value>,
}

impl Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "payload is : {}", serde_json::to_string(self).unwrap())
    }
}

pub struct TypeArguments(Vec<String>);

impl Into<Vec<Value>> for TypeArguments {
    fn into(self) -> Vec<Value> {
        self.0.into_iter().map(|s| Value::String(s)).collect()
    }
}

impl Payload {
    pub fn build(method: String, params: Vec<Value>) -> Self {
        Self {
            jsonrpc: String::from(PAYLOAD_JSONRPC_VERSION),
            id: current_timestamp(),
            method,
            params,
        }
    }

    pub fn method_paylod(method: String) -> Self {
        Self::build(method, vec![])
    }

    pub fn sui_get_object(object_id: &String, option: &FilterOption) -> Self {
        Self::build(
            String::from("sui_getObject"),
            vec![
                Value::String(object_id.to_owned()),
                to_value(option).unwrap(),
            ],
        )
    }

    pub fn unsafe_transfer_object(
        owner_address: &str,
        object_id: &str,
        gas_object: &str,
        gas_budget: u64,
        to_address: &str,
    ) -> Self {
        Self::build(
            String::from("unsafe_transferObject"),
            vec![
                Value::String(owner_address.to_string()),
                Value::String(object_id.to_string()),
                Value::String(gas_object.to_string()),
                Value::String(format!("{}", gas_budget)),
                Value::String(to_address.to_string()),
            ],
        )
    }

    pub fn safe_transaction_block_payload(tx_bytes: &str, signatures: &str) -> Self {
        let option = TransactionBlockResponseOptions::default_options();
        Self::build(
            String::from("sui_executeTransactionBlock"),
            vec![
                Value::String(tx_bytes.to_string()),
                Value::Array(vec![Value::String(signatures.to_string())]),
                to_value(&option).unwrap(),
                Value::String("WaitForLocalExecution".to_string()),
            ],
        )
    }

    pub fn publish(
        owner_address: String,
        modules: Vec<String>,
        dependencies: Vec<String>,
        gas_object: String,
        gas_budget: u64,
    ) -> Self {
        Self::build(
            "unsafe_publish".to_string(),
            vec![
                Value::String(owner_address),
                Value::from(modules),
                Value::from(dependencies),
                Value::from(gas_object),
                Value::String(format!("{}", gas_budget)),
            ],
        )
    }

    pub fn move_call(
        owner_address: String,
        package_object_id: String,
        module: String,
        function: String,
        type_arguments: Vec<String>,
        arguments: Vec<Value>,
        gas_object: String,
        gas_budget: u64,
    ) -> Self {
        Self::build(
            String::from("unsafe_moveCall"),
            vec![
                Value::String(owner_address.to_string()),
                Value::String(package_object_id.to_string()),
                Value::String(module.to_string()),
                Value::String(function.to_string()),
                json!(type_arguments),
                Value::Array(arguments),
                Value::String(gas_object.to_string()),
                Value::String(format!("{}", gas_budget)),
                Value::Null,
            ],
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct FaucetInfo {
    #[serde(rename = "FixedAmountRequest")]
    fixed_amount_request: FaucetRequest,
}

#[derive(Serialize, Deserialize)]
struct FaucetRequest {
    recipient: String,
}

pub fn new_faucet(recipient: String) -> FaucetInfo {
    FaucetInfo {
        fixed_amount_request: FaucetRequest { recipient },
    }
}

#[derive(Serialize, Deserialize)]
pub struct QueryOption {
    options: FilterOption,
    filter: QueryFilter,
}

impl Default for QueryOption {
    fn default() -> Self {
        Self {
            options: FilterOption::default(),
            filter: QueryFilter::default(),
        }
    }
}

impl QueryOption {
    pub fn with_options(options: FilterOption) -> Self {
        Self {
            options,
            filter: QueryFilter::default(),
        }
    }

    pub fn with_package(package: String) -> Self {
        Self {
            options: FilterOption::default(),
            filter: QueryFilter::MatchAll(vec![QueryFilter::Package(package)]),
        }
    }

    pub fn with_module(package: String, module: String) -> Self {
        Self {
            options: FilterOption::default(),
            filter: QueryFilter::MatchAll(vec![QueryFilter::MoveModule { package, module }]),
        }
    }

    pub fn with_strutc_type(struct_type: String) -> Self {
        Self {
            options: FilterOption::default(),
            filter: QueryFilter::MatchAll(vec![QueryFilter::StructType(struct_type)]),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum QueryFilter {
    MatchAll(Vec<QueryFilter>),
    MatchAny(Vec<QueryFilter>),
    MatchNone(Vec<QueryFilter>),
    Package(String),
    MoveModule {
        /// the Move package ID
        package: String,
        module: String,
    },
    StructType(String),
}

impl Default for QueryFilter {
    fn default() -> Self {
        QueryFilter::MatchNone(vec![])
    }
}
