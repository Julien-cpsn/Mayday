use serde::Deserialize;
use uuid::Uuid;
use crate::drivers::MessagingDriverConfigs;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub discussion_name: String,
    pub uuid: Uuid,
    pub driver: MessagingDriverConfigs
}