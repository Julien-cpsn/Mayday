use serde::Deserialize;
use crate::drivers::loopback::{LoopbackConfig, LoopbackMessaging};
use crate::drivers::irc::{IrcConfig, IrcMessaging};
use crate::models::driver::MessagingDriver;
use crate::config_to_driver;

mod loopback;
mod irc;

config_to_driver! {
    #[derive(Debug, Deserialize)]
    #[serde(tag = "type")]
    pub enum MessagingDriverConfigs {
        #[serde(alias = "loopback", alias = "LOOPBACK")]
        Loopback(LoopbackConfig) -> LoopbackMessaging,
        #[serde(alias = "irc", alias = "IRC")]
        Irc(IrcConfig) -> IrcMessaging,
    }
}