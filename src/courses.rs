//TODO eventually all aland courses and all layouts
pub fn get_course_id(course_name: &str) -> Option<&'static str> {
    match course_name {
        "LC Mariehamn DiscGolfPark" => Some("15904"),
        "Vesterkalmare" => Some("23647"),
        "Stallhagen DiscGolfPark" => Some("19351"),
        "Kastelholm DiscGolfPark" => Some("19757"),
        _ => None
    }
}