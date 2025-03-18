use madruga::{backoff::Backoff, madruga_retry, RetryResult, RetryStrategy};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let strategy = RetryStrategy::new(3, Backoff::Fixed(Duration::from_secs(1)));

    async fn retry_operation() -> Result<(), &'static str> {
        Err("Simple failure")
    }

    let result: RetryResult<_, _> = madruga_retry!(strategy, || retry_operation).await;

    match result {
        RetryResult::Success(val) => println!("Success: {:?}", val),
        RetryResult::Failure(e) => println!("Error: {}", e),
    }
}
