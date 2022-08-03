use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::CanonicalAddr;

use crate::state::{ManufactureId, PharmacistId, SymptomToken, BatchId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub pharmacists: Vec<ManufactureId>,
    pub manufacturers: Vec<PharmacistId>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Increment {},
    Reset { count: i32 },
    CreateBatch { batch_id: String, locations: String, threshold: u64},
    AddPatient { symptom_token: SymptomToken, batch_id: BatchId},
    AddSymptom { symptom_token: SymptomToken, batch_id: BatchId}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
    CheckBatch {batch_id: BatchId}
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CheckBatchResponse {
    pub threshold_reached: bool,
    pub locations: Vec<String>
}
