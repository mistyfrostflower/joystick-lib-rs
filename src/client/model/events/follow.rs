use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct FollowMeta {
    pub who: String,
    pub what: String,
}

#[derive(Debug)]
pub struct Follow {
    pub user: String,
    pub text: String,
    pub channel_id: String,
}