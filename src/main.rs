use std::time::Duration;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let timeout_duration = Duration::from_secs(5);
    let url = "https://www.google.com";
    let response = timeout(timeout_duration, reqwest::get(url)).await;
    match response {
        Ok(result) => {
            match result {
                Ok(res) => {
                    let status = res.status();
                    println!("Status from Google: {}", status);
                }
                Err(err) => {
                    println!("Request error: {}", err);
                }
            }
        }
        Err(_) => {
            println!("Request timed out!");
        }
    }
    Ok(())
}

