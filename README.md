# madruga

**Resilient retry with laziness.**

A crate for those who need retries with backoff — but with style. `madruga` attempts to solve problems, but only as far as it's worth it.

## Features

- Backoff strategies (fixed, exponential, jitter)
- Ease of use with `retry_async`
- Optional humorous messages
- Compatible with `tokio`

## Basic Example

```rust
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

```
