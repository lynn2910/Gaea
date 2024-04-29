use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum BorderType {
    None          = 0,
    Regional      = 1,
    BetweenGroups = 2
}