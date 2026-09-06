use crate::logic::lesson::Lesson;
use chrono::{NaiveTime, Weekday};
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoomVariant {
    pub room: String,
    pub building: String,
    pub accessible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DedupedSlot {
    pub course_name: String,
    pub weekday: Weekday,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub room_variants: Vec<RoomVariant>,
}

pub fn dedup_lessons(lessons: &[Lesson]) -> Vec<DedupedSlot> {
    let mut map: HashMap<(String, Weekday, NaiveTime, NaiveTime), Vec<&Lesson>> = HashMap::new();

    for lesson in lessons {
        let key = (lesson.course_name.clone(), lesson.weekday, lesson.start_time, lesson.end_time);
        map.entry(key).or_default().push(lesson);
    }

    map.into_iter().map(|(key, lessons)| {
        let (course_name, weekday, start_time, end_time) = key;
        // Raccogli le varianti di stanza e edificio in un HashSet per evitare duplicati
        let room_variants = lessons.iter().map(|l| RoomVariant {
            room: l.room.clone(),
            building: l.building.clone(),
            accessible: l.accessible,
        }).collect::<HashSet<_>>().into_iter().collect();
        DedupedSlot {
            course_name,
            weekday,
            start_time,
            end_time,
            room_variants,
        }
    }).collect()
}