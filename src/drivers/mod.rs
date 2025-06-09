use serde::Deserialize;
use crate::drivers::loopback::{LoopbackConfig, LoopbackMessaging};
use crate::models::driver::MessagingDriver;
use crate::config_to_driver;

mod loopback;

config_to_driver! {
    #[derive(Debug, Deserialize)]
    #[serde(tag = "type")]
    pub enum MessagingDriverConfigs {
        #[serde(alias = "loopback", alias = "LOOPBACK")]
        Loopback(LoopbackConfig) -> LoopbackMessaging,
    }
}