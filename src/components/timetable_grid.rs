use dioxus::prelude::*;
use crate::logic::course_group::CourseGroup;
use crate::logic::grid_position::{weekday_to_column, time_to_row, HOUR_START, HOUR_END, SLOT_MINUTES};

#[component]
pub fn TimetableGrid(selected: Signal<Vec<CourseGroup>>) -> Element {
    let total_rows = ((HOUR_END - HOUR_START) * 60 / SLOT_MINUTES) + 1; // +1 per l'header

    rsx! {
        div {
            style: "display: grid; grid-template-columns: 80px repeat(5, 1fr); grid-template-rows: repeat({total_rows}, 40px); gap: 1px;",
            class: "border rounded",

            // Header giorni
            div { style: "grid-column: 1; grid-row: 1;", "" }
            div { style: "grid-column: 2; grid-row: 1;", class: "font-bold text-center", "Lun" }
            div { style: "grid-column: 3; grid-row: 1;", class: "font-bold text-center", "Mar" }
            div { style: "grid-column: 4; grid-row: 1;", class: "font-bold text-center", "Mer" }
            div { style: "grid-column: 5; grid-row: 1;", class: "font-bold text-center", "Gio" }
            div { style: "grid-column: 6; grid-row: 1;", class: "font-bold text-center", "Ven" }

            for group in selected.read().iter() {
                for slot in &group.slots {
                    {
                        let col = weekday_to_column(slot.weekday);
                        let row_start = time_to_row(slot.start_time);
                        let row_end = time_to_row(slot.end_time);
                        rsx! {
                            div {
                                key: "{group.course_name}-{slot.weekday}-{slot.start_time}",
                                style: "grid-column: {col}; grid-row: {row_start} / {row_end};",
                                class: "bg-blue-200 border border-blue-500 rounded p-1 text-xs overflow-hidden",
                                "{slot.course_name}"
                            }
                        }
                    }
                }
            }
        }
    }
}