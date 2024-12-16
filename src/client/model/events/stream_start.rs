use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct StreamStart {
    pub channel_id: String,
}



