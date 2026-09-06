use dioxus::prelude::*;
use chrono::NaiveDate;

#[component]
pub fn CourseForm(mut course_id: Signal<String>, mut start_date: Signal<NaiveDate>) -> Element {
    rsx! {
        div { class: "flex gap-4 p-4",
            input {
                r#type: "text",
                placeholder: "ID corso (es. 33601)",
                value: "{course_id}",
                oninput: move |evt| course_id.set(evt.value()),
            }
            input {
                r#type: "date",
                value: "{start_date}",
                oninput: move |evt| {
                    if let Ok(parsed) = NaiveDate::parse_from_str(&evt.value(), "%Y-%m-%d") {
                        start_date.set(parsed);
                    }
                },
            }
        }
    }
}