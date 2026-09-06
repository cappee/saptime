use crate::logic::{dedup::DedupedSlot, overlap::conflicts_with};
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseGroup {
    pub course_name: String,
    pub slots: Vec<DedupedSlot>,
}

pub fn group_by_course(slots: &[DedupedSlot]) -> Vec<CourseGroup> {
    // Raggruppiamo gli slot per nome del corso
    let mut course_map: HashMap<String, Vec<DedupedSlot>> = HashMap::new();
    for slot in slots {
        course_map.entry(slot.course_name.clone()).or_default().push(slot.clone());
    }
    course_map.into_iter().map(|(course_name, slots)| CourseGroup { course_name, slots }).collect()
}

pub fn course_conflicts_with<'a>(
    candidate: &CourseGroup,
    selected: &'a [DedupedSlot],
) -> Vec<&'a DedupedSlot> {
    candidate
        .slots
        .iter()
        .flat_map(|slot| conflicts_with(slot, selected))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}