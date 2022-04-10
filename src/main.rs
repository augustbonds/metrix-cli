mod competitions;

use reqwest::{Client, Error, Response};
use futures::executor::block_on;

async fn login(client: &Client, username: &str, password: &str) -> Result<Response, Error> {
    let params = [("email", username), ("password", password), ("Login", "Login")];
    client.post("https://discgolfmetrix.com/?u=login")
        .form(&params)
        .send().await
}


#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        
        std::process::exit(-1);
    }

    let password_file = &args[1];
    
    let password_file_contents = std::fs::read_to_string(password_file).expect("Something went wrong");
    let user_pass = password_file_contents.split("\n").collect::<Vec<&str>>();

    let client = reqwest::Client::builder().cookie_store(true).build().unwrap();
    let login_result = block_on(login(&client, user_pass[0], user_pass[1]));

    match login_result {
        Ok(_) => println!("Got response!"),
        Err(_) => println!("Failed")
    }

    let result = block_on(competitions::competitions(&client));
    println!("{}", result.text().await.unwrap());

}


//

//back
//back_url
//email=aganom@gmail.com
//password=aO2I72l6XX8yoG6mEQzp
//Login=Login
