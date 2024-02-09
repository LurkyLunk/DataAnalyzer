use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataModel {
    pub id: u32,
    pub content: String,
}
