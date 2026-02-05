use tokio;

#[tokio::main]
async fn main() {
    backend::run().await;
}
