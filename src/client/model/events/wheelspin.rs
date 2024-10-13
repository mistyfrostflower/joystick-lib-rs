use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct WheelspinMeta {
    pub who: String,
    pub what: String,
    pub how_much: usize,
    pub prize: String,
}

#[derive(Debug)]
pub struct Wheelspin {
    pub tipper: String,
    pub amount: usize,
    pub redeem: String,
    pub channel_id: String,
}