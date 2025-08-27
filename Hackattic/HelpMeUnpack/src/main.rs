use reqwest::get;
use tokio::runtime::Runtime;
use std::env;
use dotenv::dotenv;
use serde::{Serialize,Deserialize};
use serde_json::from_str;
use base64::{engine::general_purpose, Engine as _ };

const URL: &str = "https://hackattic.com/challenges/help_me_unpack/problem?access_token=";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Problem {
    bytes: String
}

#[derive(Serialize,Debug)]
#[allow(dead_code)]
struct Result {
    int: i32,
    uint: u32,
    short: i16,
    float: f32,
    double: f64,
    double_big_endian: f64
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

    let int: i32 = get_int(&problem[..4].try_into().unwrap());
    let uint: u32 = get_uint(&problem[4..8].try_into().unwrap());
    let short: i16 = get_short(&problem[8..10].try_into().unwrap());
    let float: f32 = get_float(&problem[10..14].try_into().unwrap());
    let double: f64 = get_double(&problem[14..22].try_into().unwrap());
    let double_big_endian: f64 = get_double_big_endian(&problem[22..30].try_into().unwrap());

    let solution = Result {
        int: int,
        uint: uint,
        short: short,
        float: float,
        double: double,
        double_big_endian: double_big_endian
    };

    let solution_json = convert_to_json(&solution);

    println!("{solution_json}");
}

fn convert_to_json(solution: &Result) -> String {
    serde_json::to_string(solution).expect("Could not serialize the solution!")
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

fn get_double_big_endian(double_big_endian_bytes: &[u8; 8]) -> f64 {
    let double_big_endian: f64 = f64::from_le_bytes(*double_big_endian_bytes);
    return double_big_endian;
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
