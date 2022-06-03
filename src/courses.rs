pub fn get_course_id(course_name: &str, layout_name: &str) -> Option<(usize, &'static str)> {
    match (course_name, layout_name) {
        ("LC Mariehamn DiscGolfPark", "LC Mariehamn") => Some((9,"15904")),
        ("Vesterkalmare", "AX Discgolf Layout ") => Some((9,"23647")), //This has been set to private..
        ("Stallhagen DiscGolfPark", "Main (yellow only)") => Some((9,"19351")),
        ("Kastelholm DiscGolfPark", "Kastelholm") => Some((9,"19757")),
        ("Svinö DiscGolfPark", "Svinö") => Some((9, "19387")),
        ("Skag DiscGolfPark", "Skag") => Some((18, "23808")),
        ("Föglö DiscGolfPark", "Main") => Some((9, "19524")),
        ("Käringsund DiscGolfPark", "Käringsund") => Some((9, "19760")),
        ("Soltuna DiscGolfPark", "AMA + PRO") => Some((18, "19311")),
        ("Soltuna DiscGolfPark", "AMA") => Some((9, "19310")),
        _ => None
    }
}