use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct BuildConfig {
    payload: PathBuf,
    config: MainboardConfig,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MainboardConfig {
    Ast,
    SiFive,
}
