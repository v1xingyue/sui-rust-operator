use crate::payload::Payload;
use crate::response::UnsafeTransactionResult;
use crate::utils::{base64_decode, base64_encode, CustomErr};
use blake2b_simd::{Hash, Params};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};
use hex;
use rand::rngs::OsRng;
use std::error::Error;
use std::{env, fmt::Display, str};

const INTENT_VERSION_V0: u8 = 0;
const APPID_SUI: u8 = 0;
pub enum IntentScope {
    TransactionData = 0,
    TransactionEffects = 1,
    CheckpointSummary = 2,
    PersonalMessage = 3,
}

pub enum SignatureScheme {
    ED25519 = 0x00,
    Secp256k1 = 0x01,
    Secp256r1 = 0x02,
    MultiSig = 0x03,
}

pub struct SuiAccount {
    pair: Keypair,
}

pub fn msg_hash(msg: &Vec<u8>) -> Hash {
    let mut state = Params::new().hash_length(32).to_state();
    state.update(&msg).finalize()
}

impl SuiAccount {
    pub fn from_keystore(store_str: &str) -> Result<Self, Box<dyn Error>> {
        match base64_decode(store_str) {
            // 第一个字节是秘钥对类型
            Ok(data) => SuiAccount::from_seed(&data[1..]),
            Err(err) => Err(Box::new(err)),
        }
    }

    pub fn from_seed(seed_bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        match SecretKey::from_bytes(&seed_bytes) {
            Err(err) => Err(Box::new(err)),
            Ok(secret) => {
                let public: PublicKey = (&secret).into();
                Ok(Self {
                    pair: Keypair { secret, public },
                })
            }
        }
    }

    // 通过env 获取 account, env 中value 保存 secret_key 的 bytes 的 hex 编码
    pub fn from_env(name: &str) -> Result<Self, Box<dyn Error>> {
        match env::var(name) {
            Ok(val) => {
                if val == "" {
                    Err(Box::new(CustomErr::new("env value empty!")))
                } else {
                    let key_bytes = hex::decode(val).unwrap();
                    match SecretKey::from_bytes(&key_bytes) {
                        Err(err) => Err(Box::new(err)),
                        Ok(secret) => {
                            let public: PublicKey = (&secret).into();
                            Ok(SuiAccount {
                                pair: Keypair { secret, public },
                            })
                        }
                    }
                }
            }
            Err(err) => Err(Box::new(err)),
        }
    }

    pub fn new_account() -> Self {
        let mut csprng = OsRng {};
        let pair = Keypair::generate(&mut csprng);
        Self { pair }
    }
    pub fn to_address(&self) -> String {
        let mut payload: Vec<u8> = vec![0];
        payload.extend_from_slice(self.pair.public.as_bytes());
        let h = msg_hash(&payload);
        format!("0x{}", h.to_hex())
    }

    pub fn sign_data(&self, msg_b64: &str, scope: IntentScope) -> Vec<u8> {
        let scheme = SignatureScheme::ED25519;
        let msg_bytes = base64_decode(msg_b64).unwrap();
        let pub_bytes: &[u8; 32] = self.pair.public.as_bytes();
        let mut intent_message: Vec<u8> = vec![scope as u8, INTENT_VERSION_V0, APPID_SUI];
        intent_message.append(&mut msg_bytes.to_vec());
        println!("intent : {}", base64_encode(&intent_message));
        let h = msg_hash(&intent_message);
        println!("blake2b: {}", hex::encode(h.as_bytes()));
        let signature: Signature = self.pair.sign(h.as_bytes());
        let mut wrapper_signature: Vec<u8> = vec![scheme as u8];
        wrapper_signature.append(&mut signature.to_bytes().to_vec());
        wrapper_signature.append(&mut pub_bytes.to_vec());
        wrapper_signature
    }
    pub fn sign_unsafe_transaciton(&self, unsafe_transaction: UnsafeTransactionResult) -> Payload {
        let result = self.sign_data(&unsafe_transaction.tx_bytes, IntentScope::TransactionData);
        Payload::safe_transaction_block_payload(
            &unsafe_transaction.tx_bytes,
            &base64_encode(&result),
        )
    }
}

impl Display for SuiAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sui account: {} ", &self.to_address())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sign_data() {
        let data_b64 = "AAACACAKJ/b307eQf7zEJl7o5j9URzEqj1P7Jwo2+JLm8mQAjwEAxeH0cqZyHhw5O9ex6npRVN/VqjeaklGk0sd3652k4IBGAAAAAAAAACAhCZTCQGadfZFHMUOmF/7vzYjaOL3iOFOttgQ8Vq8WRwEBAQEBAAEAAAon9vfTt5B/vMQmXujmP1RHMSqPU/snCjb4kubyZACPASyl4AYgixH+XTi5XBSI10IUmYMOMKQxedmoPg/qzXfZRQAAAAAAAAAgbcf/hvgkTrSAFqX06JcGyUca6ZeqRPOSOhgE/MNEw88KJ/b307eQf7zEJl7o5j9URzEqj1P7Jwo2+JLm8mQAj+gDAAAAAAAAwMYtAAAAAAAA" ;
        let pair_from_keystore = "AAI9gSWWADI9gC6E53o1pfhaPSdhxNbQGjT6zTIjeijF";
        let account = SuiAccount::from_keystore(pair_from_keystore).unwrap();
        assert_eq!(
            account.to_address(),
            "0x0a27f6f7d3b7907fbcc4265ee8e63f5447312a8f53fb270a36f892e6f264008f"
        );
        let signature = account.sign_data(data_b64, IntentScope::TransactionData);
        println!("{:?}", base64_encode(&signature));
        assert_eq!(
            "ABCbWyMJdo/y+RDUSqJ0TghGwzfQbmVTYHdb/FQ9SX3YybVkRrB+6nh4qutm7E1ZRqUzzC0YiG2FY9rl5IQkNAewlwaDbsn0alvR1qMy7xdd9548ZGz4MI7Mp0lic5Scsg==",
            base64_encode(&signature)
        )
    }

    #[test]
    fn test_decode_from_key_store() {
        match base64_decode("AAI9gSWWADI9gC6E53o1pfhaPSdhxNbQGjT6zTIjeijF") {
            Ok(data) => match SuiAccount::from_seed(&data[1..]) {
                Err(err) => {
                    panic!("get account error : {}", err)
                }
                Ok(account) => {
                    assert_eq!(
                        account.to_address(),
                        "0x0a27f6f7d3b7907fbcc4265ee8e63f5447312a8f53fb270a36f892e6f264008f"
                    )
                }
            },
            Err(err) => {
                panic!("err {}", err)
            }
        }
    }
}
