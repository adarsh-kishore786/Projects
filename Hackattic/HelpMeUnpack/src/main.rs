use reqwest;
use tokio::runtime::Runtime;
use std::env;
use dotenv::dotenv;
use serde::{Serialize,Deserialize};
use serde_json::from_str;
use base64::{engine::general_purpose, Engine as _ };

const URL: &str = "https://hackattic.com/challenges/help_me_unpack";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Problem {
    bytes: String
}

#[derive(Serialize,Debug)]
struct Result {
    int: i32,
    uint: u32,
    short: i16,
    float: f32,
    double: f64,
    big_endian_double: f64
}

async fn get_problem(client: &reqwest::Client, access_token: &str) -> String {
    let response = client
        .get(format!("{URL}/problem?access_token={access_token}"))
        .send()
        .await
        .expect("Some error occured while reading response!");

    return response.text().await.expect("Some error occured while parsing the response!");
}

async fn post_solution(client: &reqwest::Client, access_token: &str, solution: &Result) -> String {
    let response = client
        .post(format!("{URL}/solve?access_token={access_token}&playground=1"))
        .json(solution)
        .send()
        .await
        .expect("Some error occured while sending the solution!");

    return response.text().await.expect("There was an error in reading POST response!");
}

fn unpack(problem_bytes: &str) -> Result {
    let problem: Vec<u8> = general_purpose::STANDARD.decode(problem_bytes)
        .expect("There was a problem in decoding the payload");

    let int: i32 = get_int(&problem[..4].try_into().unwrap());
    let uint: u32 = get_uint(&problem[4..8].try_into().unwrap());
    let short: i16 = get_short(&problem[8..10].try_into().unwrap());
    let float: f32 = get_float(&problem[12..16].try_into().unwrap());
    let double: f64 = get_double(&problem[16..24].try_into().unwrap());
    let big_endian_double: f64 = get_big_endian_double(&problem[24..32].try_into().unwrap());

    return Result {
        int,
        uint,
        short,
        float,
        double,
        big_endian_double
    };
}

fn get_int(int_bytes: &[u8; 4]) -> i32 {
    let int: i32 = i32::from_le_bytes(*int_bytes);
    return int;
}

fn get_uint(uint_bytes: &[u8; 4]) -> u32 {
    let uint: u32 = u32::from_le_bytes(*uint_bytes);
    return uint;
}

fn get_short(short_bytes: &[u8; 2]) -> i16 {
    let short: i16 = i16::from_le_bytes(*short_bytes);
    return short;
}

fn get_float(float_bytes: &[u8; 4]) -> f32 {
    let float: f32 = f32::from_le_bytes(*float_bytes);
    return float;
}

fn get_double(double_bytes: &[u8; 8]) -> f64 {
    let double: f64 = f64::from_le_bytes(*double_bytes);
    return double;
}

fn get_big_endian_double(big_endian_double_bytes: &[u8; 8]) -> f64 {
    let big_endian_double: f64 = f64::from_be_bytes(*big_endian_double_bytes);
    return big_endian_double;
}

fn main() {
    let _ = dotenv();

    let access_token: String = env::var("ACCESS_TOKEN").expect("Couldn't find the access token in environment file!");
    let rt = Runtime::new().unwrap();
    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("Not able to initalize client!");

    let response: &str = &rt.block_on(get_problem(&client, &access_token));
    let problem_struct: Problem = from_str(response).expect("The response is not in the format expected!");
    let problem_bytes: String = problem_struct.bytes;

    let solution: Result = unpack(&problem_bytes);
    println!("{solution:?}");
    let response: &str = &rt.block_on(post_solution(&client, &access_token, &solution));

    println!("{response}");
}
