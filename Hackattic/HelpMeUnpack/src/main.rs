use reqwest::get;
use tokio::runtime::Runtime;

const URL: &str = "https://hackattic.com/challenges/help_me_unpack/problem?access_token=be014966b3e84380";

async fn get_problem() -> String {
    let response = get(URL)
        .await
        .expect("Some error occured!");

    return response.text().await.expect("Some error occured!");
}

fn main() {
    let rt = Runtime::new().unwrap();

    let problem = rt.block_on(get_problem());

    println!("{problem}");
}
