use chrono::{DateTime, Local};
use ormlite::Model;
use serde::{Deserialize, Serialize};

#[derive(Model, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[ormlite(table = "messages")]
pub struct Message {
    #[ormlite(primary_key)]
    pub timestamp: DateTime<Local>,
    pub sender: Option<String>,
    pub text: String,
}