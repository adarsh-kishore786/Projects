use reqwest::get;
use tokio::runtime::Runtime;
use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use serde_json::from_str;

const URL: &str = "https://hackattic.com/challenges/help_me_unpack/problem?access_token=";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Problem {
    bytes: String
}

async fn get_problem(access_token: String) -> String {
    let response = get(format!("{URL}{access_token}"))
        .await
        .expect("Some error occured while reading response!");

    return response.text().await.expect("Some error occured while parsing the response!");
}

fn main() {
    let _ = dotenv();

    let access_token: String = env::var("ACCESS_TOKEN").expect("Couldn't find the access token in environment file!");
    let rt = Runtime::new().unwrap();

    let response: &str = &rt.block_on(get_problem(access_token));
    let problem_struct: Problem = from_str(response).expect("The response is not in the format expected!");
    let problem_bytes: String = problem_struct.bytes;

    println!("{problem_bytes}");
}
