use serde::Deserialize;
use crate::drivers::loopback::LoopbackConfig;

pub mod loopback;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum MessagingDriverConfigs {
    #[serde(alias = "loopback")]
    Loopback(LoopbackConfig),
}