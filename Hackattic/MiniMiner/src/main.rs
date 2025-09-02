use reqwest;
use tokio;
use std::env;
use dotenv::dotenv;
use serde::Deserialize;

const URL: &str = "https://hackattic.com/challenges/mini_miner";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Data {
    hex: String,
    int: i32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Block {
    nonce: Option<i32>,
    data: Vec<Data>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Problem {
    difficulty: i32,
    block: Block
}

async fn get_problem(client: &reqwest::Client, access_token: &str) -> Problem {
    let response = client
        .get(format!("{URL}/problem?access_token={access_token}"))
        .send()
        .await
        .expect("Some error occured while fetching client response!");

    let problem: Problem = response.json().await.expect("Some error occured while parsing the response!");

    return problem;
}

fn process(problem: &Problem) {
    let difficulty = problem.difficulty;
    let block: &Block = &problem.block;

    println!("{block:?}");
}

#[tokio::main]
async fn main() {
    let _ = dotenv();

    let access_token: String = env::var("ACCESS_TOKEN").expect("Couldn't find the access token in environment file!");

    let client = reqwest::ClientBuilder::new()
                 .build()
                 .expect("Not able to initialize client!");

    let problem = get_problem(&client, &access_token).await;
    process(&problem);
}
