use crate::components::lesson_list::LessonList;
use crate::components::theme_toggle::ThemeToggle;
use crate::components::timetable_grid::TimetableGrid;
use crate::logic::course_group::{CourseGroup, group_by_course};
use crate::{components::course_form::CourseForm};
use crate::theme::use_theme;
use chrono::{Local, Duration};
use dioxus::prelude::*;

use crate::logic::{dedup::dedup_lessons, lesson::Lesson, validation::is_valid_course_id, lesson::raw_event_to_lesson};
use crate::api::client::fetch_timetable;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    let dark_mode = use_theme();
    
    let course_id = use_signal(|| String::new());
    let start_date = use_signal(|| Local::now().date_naive());
    let selected: Signal<Vec<CourseGroup>> = use_signal(|| Vec::new());

    let timetable = use_resource(move || {
        let id_str = course_id();
        let start = start_date();
        async move {
            if !is_valid_course_id(&id_str) {
                return None;
            }
            let id: u32 = id_str.parse().ok()?;
            let end = start + Duration::days(35);

            match fetch_timetable(id, start, end).await {
                Ok(response) => {
                    let lessons: Vec<Lesson> = response.events.iter()
                        .filter_map(raw_event_to_lesson)
                        .collect();
                    let slots = dedup_lessons(&lessons);
                    Some(Ok(slots))
                }
                Err(e) => Some(Err(e)),
            }
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        ThemeToggle { dark_mode }
        CourseForm {
            course_id: course_id,
            start_date: start_date
        }
        match &*timetable.read() {
            Some(Some(Ok(slots))) => {
                let course_groups = group_by_course(slots);
                rsx! {
                    LessonList { course_groups, selected }
                }
            }
            Some(Some(Err(e))) => rsx! { div { "Errore: {e}" } },
            Some(None) => rsx! { div { "Inserisci un ID corso valido (5 cifre)" } },
            None => rsx! { div { "Caricamento..." } },
        }
        TimetableGrid { selected }
    }
}