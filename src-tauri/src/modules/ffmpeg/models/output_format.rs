use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Default, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../src/lib/models/")]
pub struct OutputFormat {
    pub name: &'static str,
    pub extension: &'static str,
    pub preset: &'static str,
    pub limitations: Vec<Limitation>,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../src/lib/models/")]
pub enum Limitation {
    NoBitrate,
    NoAudio,
}
