#[derive(Debug)]    
pub struct ParsedLesson {
    pub course_name: String,
    pub room: String,
    pub building: String,
    pub accessible: bool,
}

pub fn parse_title(title: &str) -> Option<ParsedLesson> {
    let (_, after_b) = title.split_once("<b>")?;
    let (course_name, _) = after_b.split_once("</b>")?;
    let course_name = course_name.trim().to_string();

    let (_, after_i) = title.split_once("</i>")?;
    let (location, _) = after_i.split_once("</p>")?;
    let location = location.trim();

    let (room, rest) = location.split_once('(')?;
    let room = room.trim().to_string();

    let (inside_parens, _after) = rest.split_once(')')?;
    let (building, accessible_str) = inside_parens.split_once(',')?;
    let building = building.trim().to_string();

    let accessible = accessible_str
        .trim()
        .trim_start_matches("Accessibile:")
        .trim()
        == "Si";

    Some(ParsedLesson {
        course_name,
        room,
        building,
        accessible,
    })
}

#[test]
fn test_parse() {
    let title = r#"<b>ORGANIC CHEMISTRY AND DYES</b><p class="cal-space"><i title="Aula" class="fa fa-location"></i> Aula B (CU022, Accessibile: No)</p>"#;
    let result = parse_title(title);
    println!("{:?}", result); // aggiungi #[derive(Debug)] sopra ParsedLesson per farlo funzionare
}

#[test]
fn test_parse_with_stray_a_tag() {
    let title = r#"<b>PLANT BIOLOGY FOR CULTURAL HERITAGE</b><p class="cal-space"><i title="Aula" class="fa fa-location"></i> Aula E (CU022, Accessibile: Si)</a></p>"#;
    let result = parse_title(title).unwrap();
    assert_eq!(result.accessible, true); // probabilmente fallisce ora
}