use crate::api::models::TimetableResponse;
use chrono::NaiveDate;
use gloo_net::http::Request;

pub async fn fetch_timetable(
    course_id: u32,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<TimetableResponse, String> {
    let url = format!(
        "http://127.0.0.1:8787?course={}&start={}T00:00:00&end={}T00:00:00",
        course_id,
        start.format("%Y-%m-%d"),
        end.format("%Y-%m-%d"),
    );

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Errore di rete: {e}"))?;

    if !response.ok() {
        return Err(format!("Risposta HTTP {}", response.status()));
    }

    response
        .json::<TimetableResponse>()
        .await
        .map_err(|e| format!("Errore di parsing JSON: {e}"))
}