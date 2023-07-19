use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::default::Default;

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
    object_owner: String,
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
pub struct TransactionEffect {
    pub digest: String,
    pub events: Vec<Value>,
    raw_transaction: String,
    transaction: Value,
    effects: Value,
}

impl Default for TransactionEffect {
    fn default() -> Self {
        Self {
            digest: String::from(""),
            events: vec![],
            raw_transaction: String::from(""),
            transaction: Value::Null,
            effects: Value::Null,
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
