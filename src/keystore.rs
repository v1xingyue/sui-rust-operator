use crate::account::SuiAccount;
use crate::utils::CustomErr;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct Keystore {
    values: Vec<String>,
}

pub fn default_path() -> String {
    format!("{}/.sui/sui_config/sui.keystore", env::var("HOME").unwrap())
}

impl Keystore {
    pub fn default() -> Self {
        Self::load_from("")
    }
    pub fn load_from(path: &str) -> Self {
        let actual_path = if path.is_empty() {
            default_path()
        } else {
            path.to_owned()
        };
        let mut file = File::open(actual_path).expect("can't open keystore file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("read file failed...");

        let values = serde_json::from_str(&contents).expect("无法反序列化数据");
        Self { values }
    }

    pub fn load_account(&self, idx: usize) -> Result<SuiAccount, Box<dyn Error>> {
        if idx < self.values.len() {
            let pair = self.values[idx].to_owned();
            SuiAccount::from_keystore(&pair)
        } else {
            Err(Box::new(CustomErr::new("account index is too big ")))
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl IntoIterator for Keystore {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        (0..self.values.len()).collect::<Vec<usize>>().into_iter()
    }
}
