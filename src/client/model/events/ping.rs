
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Ping {
    pub timestamp: i64,
}