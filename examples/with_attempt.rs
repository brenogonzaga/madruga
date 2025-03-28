use madruga::{backoff::Backoff, madruga_retry, RetryResult, RetryStrategy};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let strategy = RetryStrategy::new(5, Backoff::Fixed(Duration::from_secs(1)));

    let result = madruga_retry!(strategy, |attempt| async move {
        if attempt < 3 {
            Err("Not yet!")
        } else {
            Ok("Finally!")
        }
    })
    .await;

    match result {
        RetryResult::Success(val) => println!("Success: {}", val),
        RetryResult::Failure(e) => println!("Final error: {}", e),
    }
}
