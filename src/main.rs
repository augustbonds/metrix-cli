mod competitions;
mod udisc_scorecards;

use std::time::Duration;
use reqwest::{Client, Error, Response};
use futures::executor::block_on;
use tokio::time;
use crate::udisc_scorecards::FrontNine;

async fn login(client: &Client, username: &str, password: &str) -> Result<Response, Error> {
    let params = [("email", username), ("password", password), ("Login", "Login")];
    client.post("https://discgolfmetrix.com/?u=login")
        .form(&params)
        .send().await
}

fn read_password_from_file(path: &String) -> Vec<String> {
    let password_file_contents = std::fs::read_to_string(path).expect("Something went wrong");
    password_file_contents.split("\n").map(|s| s.to_string()).collect::<Vec<String>>()
}


#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        std::process::exit(-1);
    }

    let password_file = &args[1];
    let user_pass = read_password_from_file(password_file);

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build().unwrap();
    block_on(login(&client, &user_pass[0], &user_pass[1]));

    let vec = udisc_scorecards::parse_scorecards(&args[2]);
    for scorecard in vec {
        if scorecard.player_name == "August Bonds" {
            if scorecard.course_name == "Kastelholm DiscGolfPark" {
                println!("About to log training");
                let split: Vec<&str> = scorecard.date.split(' ').collect();
                competitions::log_training(&client, "Kastelholm DiscGolfPark", split[1], split[0], scorecard.front_nine());
                time::sleep(Duration::from_secs(3)).await;
                println!("Logged training: {:?}", scorecard);
            }
        }
    }
}
