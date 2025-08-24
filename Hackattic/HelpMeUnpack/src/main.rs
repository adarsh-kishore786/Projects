use reqwest::get;
use tokio::runtime::Runtime;
use std::env;
use dotenv::dotenv;

const URL: &str = "https://hackattic.com/challenges/help_me_unpack/problem?access_token=";

async fn get_problem(access_token: &str) -> String {
    let response = get(URL.to_owned() + access_token)
        .await
        .expect("Some error occured while reading response!");

    return response.text().await.expect("Some error occured while parsing the response!");
}

fn main() {
    let _ = dotenv();

    let access_token = env::var("ACCESS_TOKEN").expect("Couldn't find the access token in environment file!");
    let rt = Runtime::new().unwrap();

    let problem = rt.block_on(get_problem(&access_token));

    println!("{problem}");
}
