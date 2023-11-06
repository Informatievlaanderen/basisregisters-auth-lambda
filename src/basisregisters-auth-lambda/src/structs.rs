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

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct XApiToken {
    #[serde(rename = "apikey")]
    pub api_key: String,

    #[serde(rename = "clientname")]
    pub client_name: String,

    #[serde(rename = "metadata")]
    pub metadata: XApiTokenMetaData,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct XApiTokenMetaData {
    #[serde(rename = "syncaccess")]
    pub sync_access: bool,

    #[serde(rename = "wraccess")]
    pub wr_access: bool,

    #[serde(rename = "tickets")]
    pub tickets: bool,
}
