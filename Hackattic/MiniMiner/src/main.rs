use reqwest;
use tokio;
use std::env;
use dotenv::dotenv;

const URL: &str = "https://hackattic.com/challenges/mini_miner";

async fn get_problem(client: &reqwest::Client, access_token: &str) -> String {
    let response = client
        .get(format!("{URL}/problem?access_token={access_token}"))
        .send()
        .await
        .expect("Some error occured while fetching client response!");

    return response.text().await.expect("Some error occured while parsing the response!");
}

#[tokio::main]
async fn main() {
    let _ = dotenv();

    let access_token: String = env::var("ACCESS_TOKEN").expect("Couldn't find the access token in environment file!");

    let client = reqwest::ClientBuilder::new()
                 .build()
                 .expect("Not able to initialize client!");

    let problem = get_problem(&client, &access_token).await;
    println!("{problem:?}");
}
