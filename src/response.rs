use serde::Serialize;

use crate::model::Quest;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct QuestData {
    pub quest: Quest,
    pub days_left: Option<i64>,
    pub min_hours_per_day: Option<f64>,
}

#[derive(Serialize, Debug)]
pub struct SingleQuestResponse {
    pub status: String,
    pub data: QuestData,
}

#[derive(Serialize, Debug)]
pub struct QuestListResponse {
    pub status: String,
    pub results: usize,
    pub quests: Vec<Quest>,
}
