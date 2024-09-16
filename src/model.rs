use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Quest {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub deadline: Option<NaiveDate>,
    pub completed: Option<bool>,
    pub createdAt: Option<DateTime<Local>>,
    pub updatedAt: Option<DateTime<Local>>,
}

pub struct AppState {
    pub quest_db: Arc<Mutex<Vec<Quest>>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            quest_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryOption {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct UpdateQuestSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub deadline: Option<NaiveDate>,
    pub completed: Option<bool>,
}
