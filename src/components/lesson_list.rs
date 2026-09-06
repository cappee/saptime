use dioxus::prelude::*;

use crate::logic::{course_group::{CourseGroup, course_conflicts_with}, dedup::DedupedSlot};

#[component]
pub fn LessonList(
    course_groups: Vec<CourseGroup>,
    selected: Signal<Vec<CourseGroup>>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2",
            for group in course_groups {
                {
                    let is_selected = selected.read().iter().any(|g| g.course_name == group.course_name);

                    let other_selected: Vec<CourseGroup> = selected.read()
                        .iter()
                        .filter(|g| g.course_name != group.course_name)
                        .cloned()
                        .collect();
                    let flattened = flatten_selected_slots(&other_selected);
                    let conflicts = course_conflicts_with(&group, &flattened);
                    let has_conflict = !conflicts.is_empty();

                    let bg_class = if is_selected {
                        "bg-blue-200 border-blue-500"
                    } else if has_conflict {
                        "bg-red-100"
                    } else {
                        "bg-green-100"
                    };

                    let cursor_class = if has_conflict && !is_selected { "cursor-not-allowed opacity-60" } else { "cursor-pointer" };

                    rsx! {
                        div {
                            key: "{group.course_name}",
                            class: "flex items-center {cursor_class} gap-2 p-2 border rounded {bg_class}",
                            onclick: move |_| {
                                let mut selected_groups = selected.read().clone();
                                let already_selected = selected_groups.iter().any(|g| g.course_name == group.course_name);

                                if already_selected {
                                    // Deselezionare è sempre permesso
                                    selected_groups.retain(|g| g.course_name != group.course_name);
                                    selected.set(selected_groups);
                                } else if !has_conflict {
                                    // Selezionare è permesso solo se non c'è conflitto
                                    selected_groups.push(group.clone());
                                    selected.set(selected_groups);
                                }
                                // else: has_conflict && !already_selected → non fare nulla, click ignorato
                            },
                            span { "{group.course_name}" }
                            if is_selected {
                                span { class: "text-blue-700", "✓ Selezionato" }
                            } else if has_conflict {
                                span { class: "text-red-600", "⚠️ Conflitto" }
                            }
                        }
                    }
                }
            }     
        }
    }
}

fn flatten_selected_slots(selected: &[CourseGroup]) -> Vec<DedupedSlot> {
    // hint: flat_map di nuovo, come course_conflicts_with
    selected.iter().flat_map(|g| g.slots.clone()).collect()
}