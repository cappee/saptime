use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TimetableResponse {
    pub events: Vec<RawEvent>,
    pub filters: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RawEvent {
    pub id: i64,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub title: String,
    pub description: String,
    #[serde(rename = "textColor")]
    pub text_color: String,
    pub curricula: Vec<serde_json::Value>,
    pub course_years: Vec<serde_json::Value>,
    pub partitions: Vec<serde_json::Value>,
    pub ssds: Vec<serde_json::Value>,
}