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
use madruga::{retry_async, RetryStrategy, RetryResult};
use madruga::backoff::Backoff;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let strategy = RetryStrategy::new(5, Backoff::Fixed(Duration::from_secs(1)))
        .with_language(Language::En);

    let result = madruga_retry!(strategy, |attempt| async move {
        if attempt < 3 {
            Err("Not yet...")
        } else {
            Ok("Now!")
        }
    })
    .await;

    match result {
        RetryResult::Success(val) => println!("Success: {}", val),
        RetryResult::Failure(e) => println!("Final error: {}", e),
    }
}
```
