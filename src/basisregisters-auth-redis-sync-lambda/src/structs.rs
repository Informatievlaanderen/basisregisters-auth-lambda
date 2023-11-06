extern crate serde;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug,Clone, Deserialize, Serialize)]
pub struct ApiKeyItem {
    #[serde(rename = "ApiKey")]
    pub api_key: String,

    #[serde(rename = "Revoked")]
    pub revoked: Option<bool>,

    #[serde(rename = "ApiKeyId")]
    pub api_key_id: Option<String>,

    #[serde(rename = "ClientName")]
    pub client_name: Option<String>,

    #[serde(rename = "Description")]
    pub description: Option<String>,

    #[serde(rename = "Plan")]
    pub plan: String,

    #[serde(rename = "UsagePlanID")]
    pub usage_plan_id: String,

    #[serde(rename = "SyncAccess")]
    pub sync_access: Option<bool>,

    #[serde(rename = "WrAccess")]
    pub wr_access: Option<bool>,

    #[serde(rename = "Tickets")]
    pub tickets: Option<bool>,
}