use chrono::{Datelike, NaiveTime, Weekday};

use crate::{api::models::RawEvent, logic::parse_title::parse_title};

#[derive(Debug, Clone)]
pub struct Lesson {
    pub weekday: Weekday,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub course_name: String,
    pub room: String,
    pub building: String,
    pub accessible: bool,
}

pub fn raw_event_to_lesson(raw: &RawEvent) -> Option<Lesson> {
    // usa parse_title(&raw.title) per ottenere ParsedLesson
    // usa raw.start (NaiveDateTime) per estrarre weekday e time
    // combina i due in un Lesson
    let weekday = raw.start.weekday();
    let start_time = raw.start.time();
    let end_time = raw.end.time();
    
    let parsed = parse_title(&raw.title)?;
    Some(Lesson {
        weekday,
        start_time,
        end_time,
        course_name: parsed.course_name,
        room: parsed.room,
        building: parsed.building,
        accessible: parsed.accessible,
    })
}