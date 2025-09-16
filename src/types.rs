use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub title: String,
    pub time: String,
    #[serde(default)]
    pub detail: Option<String>,
}
