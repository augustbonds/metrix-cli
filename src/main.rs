mod competitions;
mod udisc_scorecards;
mod courses;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use reqwest::{Client, Error, Response};
use futures::executor::block_on;
use tokio::time;
use clap::Parser;
use scraper::{Html, Selector};
use crate::courses::get_course_id;
use crate::udisc_scorecards::{FrontNine, UDiscScorecard};

async fn login(client: &Client, username: &str, password: &str) -> Result<Response, Error> {
    let params = [("email", username), ("password", password), ("Login", "Login")];
    client.post("https://discgolfmetrix.com/?u=login")
        .form(&params)
        .send().await
}

struct MetrixCredentials {
    username: String,
    password: String,
}

fn read_password_from_file(path: &String) -> MetrixCredentials {
    let fopen = File::open(path);
    match fopen {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut username: String = String::new();
            let mut password: String = String::new();
            for (i, x)  in reader.lines().enumerate() {
                match i {
                    0 => username = x.unwrap().trim().to_owned(),
                    1 => password = x.unwrap().trim().to_owned(),
                    _ => ()
                }
            }
            return MetrixCredentials {username, password}

        }
        Err(_) => {
            eprintln!("Could not open password file {}", path);
            std::process::exit(-1);
        }
    }
}

///Import your UDisc scores to discgolfmetrix
#[derive(Parser, Debug)]
struct Args {

    /// The name of the player as written in the 'Player'-column of the CSV.
    #[clap(long)]
    player_name: String,

    /// Path to the password file
    #[clap(long)]
    password_file: String,

    /// Path to the UDisc CSV-file.
    #[clap(short,long)]
    udisc_csv: String,

    /// Skip first n rounds in the CSV.
    #[clap(short, long, default_value_t = 0)]
    skip: usize,

    /// Limit number of inserted rounds to n
    #[clap(short, long, default_value_t = 0)]
    limit: usize,

    #[clap(long)]
    dry_run: bool,

    #[clap(long)]
    debug: bool,

}

#[tokio::main]
async fn main() {

    let args: Args = Args::parse();

    if args.password_file.is_empty() || args.udisc_csv.is_empty() {
        std::process::exit(-1);
    }

    // Import metrix credentials
    let user_pass = read_password_from_file(&args.password_file);

    let client = Client::builder()
        .cookie_store(true)
        .build().unwrap();
    // Login to metrix
    if !args.dry_run {
      let login_result = block_on(login(&client, &user_pass.username, &user_pass.password));
        match login_result {
            Ok(response) => {
                let html = Html::parse_document(&response.text().await.unwrap());
                let selector = Selector::parse("div .message-error").unwrap();
                match html.select(&selector).next() {
                    Some(_) => {
                        eprintln!("(1)Failed to log in to discgolfmetrix. Do you have a working internet connection and did you provide the correct username/password?");
                        std::process::exit(-1);
                    }
                    None => ()
                }
            },
            Err(_) => {
                eprintln!("(2)Failed to log in to discgolfmetrix. Do you have a working internet connection and did you provide the correct username/password?");
                std::process::exit(-1);
            }
        }
    }

    // Read scorecards from csv
    let scorecards = udisc_scorecards::parse_scorecards(&args.udisc_csv);
    let filtered_by_player: Vec<UDiscScorecard> = scorecards.into_iter().filter(|scorecard| scorecard.player_name == args.player_name).collect();
    let filtered_by_aland_courses: Vec<UDiscScorecard> = filtered_by_player.into_iter().filter(|scorecard| get_course_id(&scorecard.course_name, &scorecard.layout_name) != None).collect();

    if args.skip > filtered_by_aland_courses.len() {
        eprintln!("Trying to skip {} scorecards when there are only {} scorecards matching player {}", args.skip, filtered_by_aland_courses.len(), args.player_name)
    }

    println!("{} scorecards to insert for player {}", filtered_by_aland_courses.len(), args.player_name);

    let mut slice = &filtered_by_aland_courses[args.skip..];
    println!("Skipping {} first scorecards", args.skip);

    if args.limit != 0 {
        slice = &slice[..args.limit];
    }
    for scorecard in slice {
        let get_metrix_id = get_course_id(&scorecard.course_name, &scorecard.layout_name);
        match get_metrix_id {
            Some((num_tees, id)) => {
                if args.dry_run {
                    println!("DRY: imported scorecard {:?}", &scorecard);
                } else {
                    log_scorecard(&client, &scorecard, num_tees, id).await
                }
            }
            None => {println!("Skipped {:?}", &scorecard);}
        }
    }
}

async fn log_scorecard(client: &Client, scorecard: &UDiscScorecard, num_tees: usize, metrix_id: &str) {
    let split: Vec<&str> = scorecard.date.split(' ').collect();
    competitions::log_training(&client, &scorecard, metrix_id, num_tees, split[1], split[0]);
    time::sleep(Duration::from_secs(4)).await;
    println!("Logged training: {:?}", scorecard);
}
