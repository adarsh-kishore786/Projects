use reqwest::get;
use tokio::runtime::Runtime;
use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use serde_json::from_str;
use base64::{engine::general_purpose, Engine as _ };

const URL: &str = "https://hackattic.com/challenges/help_me_unpack/problem?access_token=";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Problem {
    bytes: String
}

#[allow(dead_code)]
struct Result {
    int: i32,
    uint: u32,
    short: i16,
    float: f32,
    double: f64,
    double_network: f64
}

async fn get_problem(access_token: &str) -> String {
    let response = get(format!("{URL}{access_token}"))
        .await
        .expect("Some error occured while reading response!");

    return response.text().await.expect("Some error occured while parsing the response!");
}

fn unpack(problem_bytes: &str) {
    let problem: Vec<u8> = general_purpose::STANDARD.decode(problem_bytes)
        .expect("There was a problem in decoding the payload");

    let int: i32 = get_int(&problem);
    println!("{:02x?}", problem);
}

fn get_int(problem: &Vec<u8>) -> i32 {
    32
}

fn main() {
    let _ = dotenv();

    let access_token: String = env::var("ACCESS_TOKEN").expect("Couldn't find the access token in environment file!");
    let rt = Runtime::new().unwrap();

    let response: &str = &rt.block_on(get_problem(&access_token));
    let problem_struct: Problem = from_str(response).expect("The response is not in the format expected!");
    let problem_bytes: String = problem_struct.bytes;

    unpack(&problem_bytes);
}
