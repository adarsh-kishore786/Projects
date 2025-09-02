use reqwest;
use tokio;
use std::env;
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

const URL: &str = "https://hackattic.com/challenges/mini_miner";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Data {
    _hex: String,
    _int: i32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Block {
    _nonce: Option<i32>,
    data: Vec<Data>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Problem {
    difficulty: usize,
    block: Block
}

#[derive(Serialize, Debug)]
struct Result {
    nonce: i32
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

fn process(problem: &Problem) -> Result {
    let difficulty = problem.difficulty;
    let block: &Block = &problem.block;
    let data: &Vec<Data> = &block.data;

    let data_str = construct_str(data);
    let result_str = format!("{{\"data\":{data_str},\"nonce\":__NONCE__}}");

    let nonce = find_nonce(difficulty, &result_str);
    return Result {
        nonce
    };
}

fn find_nonce(difficulty: usize, base_str: &str) -> i32 {
    let mut nonce = 0;

    let ideal_hash = std::iter::repeat('0').take(difficulty).collect::<String>();

    loop {
        let res = base_str.replace("__NONCE__", &nonce.to_string());
        let mut hasher = Sha256::new();
        hasher.update(res);
        let result = hasher.finalize();

        let mut bit_string = String::from("");

        for byte in result {
            bit_string = format!("{bit_string}{byte:08b}")
        }
        let bit_string = &bit_string[..difficulty];

        if bit_string == ideal_hash {
            break; 
        }

        nonce += 1;
    }

    return nonce;
}

async fn post_solution(client: &reqwest::Client, access_token: &str, solution: &Result) -> String {
    let response = client
        .post(format!("{URL}/solve?access_token={access_token}"))
        .json(solution)
        .send()
        .await
        .expect("Some error occured while sending the solution!");

    return response.text()
        .await
        .expect("There was an error in reading the POST response!");
}

fn construct_str(data: &Vec<Data>) -> String {
    let mut data_str = format!("{data:?}");
    data_str = data_str.replace(" ", "")
        .replace("Data", "")
        .replace("{", "[")
        .replace("}", "]")
        .replace("_hex:", "")
        .replace("_int:", "");

    return data_str;
}

#[tokio::main]
async fn main() {
    let _ = dotenv();

    let access_token: String = env::var("ACCESS_TOKEN").expect("Couldn't find the access token in environment file!");

    let client = reqwest::ClientBuilder::new()
                 .build()
                 .expect("Not able to initialize client!");

    let problem = get_problem(&client, &access_token).await;
    let solution = process(&problem);
    let response = post_solution(&client, &access_token, &solution).await;

    println!("{response}");
}
