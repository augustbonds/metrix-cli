#[derive(Debug)]
pub struct Error {

}

//TODO eventually all aland courses and all layouts

pub fn get_course_id(course_name: &str) -> Result<&'static str, Error> {
    match course_name {
        "LC Mariehamn DiscGolfPark" => Ok("15904"),
        "Vesterkalmare" => Ok("23647"),
        "Stallhagen DiscGolfPark" => Ok("19351"),
        "Kastelholm DiscGolfPark" => Ok("19757"),
        _ => Err(Error {})
    }
}