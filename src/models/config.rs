use std::any::Any;
use uuid::Uuid;

pub trait DriverConfig : Any + Send + Sync {
    fn discussion_name(&self) -> &str;
    fn uuid(&self) -> &Uuid;
}