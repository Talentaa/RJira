use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Status {
    OPEN,
    INPROGRESS,
    RESOLVED,
    CLOSED,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u64>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::OPEN,
            stories: Vec::new(),
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::OPEN,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct DBState {
    pub last_item_id: u64,
    pub epics: HashMap<u64, Epic>,
    pub stories: HashMap<u64, Story>,
}
