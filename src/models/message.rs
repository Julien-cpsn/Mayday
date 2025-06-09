use chrono::Local;
use native_db::{native_db, Key, ToKey};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Message {
    #[primary_key]
    pub timestamp: DateTime,
    #[secondary_key]
    pub sender: Option<String>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
pub struct DateTime(pub chrono::DateTime<Local>);

impl ToKey for DateTime {
    fn to_key(&self) -> Key {
        Key::new(self.0.timestamp_millis().to_be_bytes().to_vec())
    }

    fn key_names() -> Vec<String> {
        vec!["DateTime".to_string()]
    }
}