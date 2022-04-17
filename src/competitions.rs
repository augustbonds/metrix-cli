use futures::executor::block_on;
use reqwest::{Client, multipart, Response};
use scraper::{Html, Selector};

//time: 15:16
//date: 2022-01-22
//course_id: "23647" //vk
pub fn log_training(client: &Client, course_name: &str, time: &str, date: &str, scores: [u32;9]) {
    let competition_id = block_on(start_training(&client, course_name));
    block_on(edit_start_time(&client, &competition_id, time, date));
    block_on(enter_scorecard(&client, &competition_id, scores));
    block_on(reqwest::get("https://discgolfmetrix.com/".to_string() + &competition_id));
}

async fn start_training(client: &Client, course_name: &str) -> String {
    let url = "https://discgolfmetrix.com/?u=competition_add";
    let course_id = get_course_id(course_name);
    if course_id == "" {
        return String::new()
    }
    let response = client.get(url)
        .query(&[("courseid", course_id), ("create_training", "1"), ("competition_type", "1"), ("record_type", "2")])
        .send().await;
    let res_url = response.unwrap().url().clone();
    println!("{}", res_url);

    for pair in res_url.query_pairs(){
        if pair.0 == "ID" {
            return pair.1.to_string()
        }
    }
    return String::new()
}

fn get_course_id(course_name: &str) -> &'static str {
    match course_name {
        "LC Mariehamn DiscGolfPark" => "15904",
        "Vesterkalmare" => "23647",
        "Stallhagen DiscGolfPark" => "19351",
        "Kastelholm DiscGolfPark" => "19757",
        _ => ""
    }
}

async fn edit_start_time(client: &Client, competition_id: &str, time: &str, date: &str) {
    let form = multipart::Form::new()
        .text("record_type", "2")
        .text("name", "Training")
        .text("multiday_value", "0")
        .text("date", date.to_string())
        .text("time", time.to_string())
        .text("date_start", date.to_string())
        .text("date_end", date.to_string())
        .text("comment", "")
        .text("accesslevel", "0")
        .text("game_mode", "r")
        .text("usegroups", "0")
        .text("metrix", "1")
        .text("is_competition_closed", "0")
        .text("Action", "Save")
        .text("country_code", "FI")
        .text("rating", "1")
        .text("active_accordion", "accordion_details");

    let url = "https://discgolfmetrix.com/?u=competition_edit";
    let response = client.post(url)
        .query(&[("ID", competition_id)])
        .multipart(form).send().await;
    println!("{}", response.unwrap().url())
}

async fn get_scorecard_id(client: &Client, competition_id: &str) -> String {
    //get scorecard ID
    //https://discgolfmetrix.com/?u=competition_score_desktop&ID=2073409
    // parse out this: <input type="hidden" name="scorecard_id[]" value="7395567">

    let scorecard_id_url = "https://discgolfmetrix.com/?u=competition_score_desktop";
    let result = client.get(scorecard_id_url).query(&[("ID", competition_id)]).send().await;
    let response = result.unwrap();
    let html = Html::parse_document(&response.text().await.unwrap());
    let selector = Selector::parse("input[name=scorecard_id\\[\\]]").unwrap();
    let scorecard_id = html.select(&selector).next().unwrap().value().attr("value").unwrap().to_string();
    println!("{}", scorecard_id);
    scorecard_id
}

async fn enter_scorecard(client: &Client, competition_id: &str, scores: [u32;9]) {

    let scorecard_id = get_scorecard_id(client, competition_id).await;

    let mut score_data:Vec<(String, String)> = Vec::new();
    score_data.push(("tee_count".to_string(), "9".to_string()));
    score_data.push(("competitor_count".to_string(), "1".to_string()));
    score_data.push(("metrix_mode".to_string(), "2".to_string()));
    score_data.push(("scorecard_id[]".to_string(), scorecard_id));
    score_data.push(("tee_no[]".to_string(), "1".to_string()));
    score_data.push(("tee_no[]".to_string(), "2".to_string()));
    score_data.push(("tee_no[]".to_string(), "3".to_string()));
    score_data.push(("tee_no[]".to_string(), "4".to_string()));
    score_data.push(("tee_no[]".to_string(), "5".to_string()));
    score_data.push(("tee_no[]".to_string(), "6".to_string()));
    score_data.push(("tee_no[]".to_string(), "7".to_string()));
    score_data.push(("tee_no[]".to_string(), "8".to_string()));
    score_data.push(("tee_no[]".to_string(), "9".to_string()));
    for score in scores {
        score_data.push(("score[]".to_string(), score.to_string()));
    }
    for i in 1..9 {
        score_data.push(("icp[]".to_string(), "".to_string()));
    }
    for i in 1..9 {
        score_data.push(("penalties[]".to_string(), "".to_string()));
    }
    score_data.push(("ActionSave".to_string(), "Save".to_string()));

    let score_url = "https://discgolfmetrix.com/?u=competition_score_desktop";
    let result1 = client.post(score_url)
        .query(&[("selected_group", ""), ("metrix_mode", "2"), ("player", "0"), ("ID", competition_id)])
        .form(&score_data).send().await.unwrap();

    println!("{}", result1.url());
}

struct Competition {
    pub id: String
}

pub async fn competitions(client: &Client) -> Response {
    let url = "https://discgolfmetrix.com/competitions_list_server.php?view=2&my_all=1&view=2&from=1&to=20&page=my";
    client.get(url).send().await.unwrap()
}

fn parse_competitions(xml : &str) -> Vec<Competition> {
    let fragment = Html::parse_fragment(xml);

    let trselector = Selector::parse("tbody > tr").unwrap();
    let tdselector = Selector::parse("td").unwrap();

    let mut vec = Vec::new();
    for tr in fragment.select(&trselector) {
        let onclick = tr.value().attr("onclick").unwrap();
        for td in tr.select(&tdselector) {

        }
        vec.push(Competition { id: onclick.to_string()})
    }

    vec
}

#[cfg(test)]
mod tests {
    use crate::competitions;

    #[test]
    fn it_works() {
        let infile_name = "src/competitions.html";
        let xml = std::fs::read_to_string(infile_name).expect("Failed to read {}");

        let vec = competitions::parse_competitions(&xml);
        assert_eq!(vec.len(), 78);
    }
}
