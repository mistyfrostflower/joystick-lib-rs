use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct TipMeta {
    pub who: String,
    pub what: String,
    pub how_much: usize,
    pub tip_menu_item: Option<String>,
}

#[derive(Debug)]
pub struct Tipped {
    pub tipper: String,
    pub amount: usize,
    pub redeem: Option<String>,
    pub channel_id: String,
}