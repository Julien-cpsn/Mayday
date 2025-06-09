use native_db::Models;
use once_cell::sync::Lazy;
use crate::models::message::Message;

pub static MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models.define::<Message>().unwrap();
    models
});