use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{default::Default, fmt::Display, vec};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Either<A, B> {
    A(A),
    B(B),
}

#[derive(Serialize, Deserialize)]
pub struct JsonResult<T: Default> {
    pub jsonrpc: String,
    #[serde(default)]
    pub result: T,
    #[serde(default)]
    pub error: RpcError,
}

#[derive(Serialize, Deserialize)]
pub struct RpcError {
    code: i32,
    message: String,
}

impl Default for RpcError {
    fn default() -> Self {
        Self {
            code: 0,
            message: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SimpleObject {
    data: ObjectData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectData {
    object_id: String,
    version: String,
    digest: String,
    #[serde(rename = "type")]
    object_type: String,
    owner: Owner,
    previous_transaction: String,
    #[serde(default)]
    storage_rebate: String,
    content: ObjectContent,
}

#[derive(Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "ObjectOwner")]
    #[serde(default)]
    object_owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddressOwner {
    #[serde(rename = "AddressOwner")]
    address_owner: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectContent {
    data_type: String,
    #[serde(rename = "type")]
    object_type: String,
    has_public_transfer: bool,
    fields: Value,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsafeTransactionResult {
    pub tx_bytes: String,
    gas: Vec<MiniObject>,
    input_objects: Vec<InputObject>,
}

#[derive(Serialize, Deserialize)]
struct InputObject {
    #[serde(flatten)]
    data: Either<MovePackage, ImmOrOwnedMoveObject>,
}

#[derive(Serialize, Deserialize)]
struct MovePackage {
    #[serde(rename = "MovePackage")]
    move_package: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniObject {
    object_id: String,
    version: u64,
    digest: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImmOrOwnedMoveObject {
    #[serde(rename = "ImmOrOwnedMoveObject")]
    imm_or_owned_move_object: MiniObject,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEffectResult {
    pub digest: String,
    pub events: Vec<Value>,
    raw_transaction: String,
    transaction: Value,
    effects: Option<TransactionEffects>,
}

impl Default for TransactionEffectResult {
    fn default() -> Self {
        Self {
            digest: String::from(""),
            events: vec![],
            raw_transaction: String::from(""),
            transaction: Value::Null,
            effects: None,
        }
    }
}

impl Default for UnsafeTransactionResult {
    fn default() -> Self {
        Self {
            tx_bytes: String::from(""),
            gas: vec![],
            input_objects: vec![],
        }
    }
}

impl Default for SimpleObject {
    fn default() -> Self {
        Self {
            data: ObjectData {
                object_id: String::from(""),
                version: String::from(""),
                digest: String::from(""),
                object_type: String::from(""),
                owner: Owner::default(),
                previous_transaction: String::from(""),
                storage_rebate: String::from(""),
                content: ObjectContent::default(),
            },
        }
    }
}

impl Default for Owner {
    fn default() -> Self {
        Self {
            object_owner: String::from(""),
        }
    }
}

impl Default for ObjectContent {
    fn default() -> Self {
        Self {
            data_type: String::from(""),
            object_type: String::from(""),
            has_public_transfer: false,
            fields: Value::Null,
        }
    }
}

impl Default for MiniObject {
    fn default() -> Self {
        Self {
            object_id: String::from(""),
            version: 0,
            digest: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectList {
    pub data: Vec<SimpleObject>,
    next_cursor: Option<String>,
    has_next_page: bool,
}

impl Default for ObjectList {
    fn default() -> Self {
        Self {
            data: vec![],
            next_cursor: Some("".to_string()),
            has_next_page: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub coin_type: String,
    pub coin_object_count: u64,
    pub total_balance: String,
    pub locked_balance: Value,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinList {
    pub data: Vec<CoinInfo>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    pub coin_type: String,
    pub coin_object_id: String,
    pub version: String,
    pub digest: String,
    pub balance: String,
    pub previous_transaction: String,
}

impl Display for CoinInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<< coin type: {} , balance: {} , id: {} >>",
            &self.coin_type, self.balance, self.coin_object_id
        )
    }
}

impl Default for CoinInfo {
    fn default() -> Self {
        Self {
            coin_type: String::from(""),
            coin_object_id: String::from(""),
            version: String::from(""),
            digest: String::from(""),
            balance: String::from(""),
            previous_transaction: String::from(""),
        }
    }
}

impl Default for CoinList {
    fn default() -> Self {
        Self { data: vec![] }
    }
}

impl CoinInfo {
    pub fn balance_u64(&self) -> u64 {
        self.balance.parse::<u64>().unwrap()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerWithReference {
    owner: Either<String, AddressOwner>,
    reference: MiniObject,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEffects {
    status: StatusMessage,
    executed_epoch: String,
    message_version: String,
    dependencies: Vec<String>,
    transaction_digest: String,
    gas_object: OwnerWithReference,
    modified_at_versions: Vec<ObjectVersion>,
    gas_used: GasUsed,
    created: Vec<OwnerWithReference>,
    mutated: Vec<OwnerWithReference>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectVersion {
    object_id: String,
    sequence_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct StatusMessage {
    status: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasUsed {
    computation_cost: String,
    storage_cost: String,
    storage_rebate: String,
    non_refundable_storage_fee: String,
}
