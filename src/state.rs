use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use std::collections::HashMap;
use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

pub static CONFIG_KEY_M: &[u8] = b"manufacturers";
pub static CONFIG_KEY_P: &[u8] = b"pharmacists";
pub static CONFIG_KEY_B: &[u8] = b"batches";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: CanonicalAddr,
}

pub type ManufactureId = CanonicalAddr;
pub type PharmacistId = CanonicalAddr;
pub type SymptomToken = [u8; 32];
pub type BatchId = [u8; 32];

type ManufactureList = Vec<ManufactureId>;

type PharmacistList = Vec<PharmacistId>;

type BatchList = HashMap<BatchId, BatchState>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BatchState {
    pub locations: String,
    pub symptoms: HashMap<SymptomToken, bool>,
    pub threshold: u64,
    pub count: u64,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
