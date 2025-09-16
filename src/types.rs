use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Task {
    title: String,
    time: String,
    #[serde(default)]
    detail: Option<String>,
}
