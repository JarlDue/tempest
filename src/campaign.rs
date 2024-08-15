use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Campaign {
    pub name: String,
    pub version: String,
    pub description: String,
    pub base_url: String,
    pub scenarios: Vec<Scenario>,
    pub success_criteria: SuccessCriteria,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Scenario {
    pub name: String,
    pub endpoint: String,
    pub method: HttpMethod,
    pub rate: u32,
    pub duration: u32,
    #[serde(default)]
    pub query_params: HashMap<String, String>,
    #[serde(default)]
    pub json_content: Option<serde_json::Value>,
    #[serde(default)]
    pub raw_content: Option<String>,
    #[serde(default)]
    pub response: Option<ResponseExtraction>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseExtraction {
    pub extract: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuccessCriteria {
    pub max_response_time: u32,
    pub error_rate_threshold: f32,
}

impl Campaign {
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}
