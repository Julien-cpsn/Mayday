use chrono::{DateTime, Local};

pub struct Message {
    pub sender: Option<String>,
    pub text: String,
    pub timestamp: DateTime<Local>,
}