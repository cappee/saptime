pub fn is_valid_course_id(s: &str) -> bool {
    // Check if the string consists only of digits and is exactly 5 characters long
    s.len() == 5 && s.chars().all(|c| c.is_ascii_digit())
}