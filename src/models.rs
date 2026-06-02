use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Entry {
    pub id: u32,
    pub date: NaiveDate,
    pub message: String,
    pub tag: Option<String>,
}
