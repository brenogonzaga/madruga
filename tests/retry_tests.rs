use madruga::logger::Log;
use madruga::{RetryResult, RetryStrategy, backoff::Backoff, retry_async};
use std::time::Duration;

#[tokio::test]
async fn test_success_on_first_try() {
    let strategy = RetryStrategy::new(3, Backoff::Fixed(Duration::from_millis(10)));
    let result: RetryResult<&str, &str> = retry_async(strategy, |_| async { Ok("ok") }).await;
    assert!(matches!(result, RetryResult::Success(_)));
}

#[tokio::test]
async fn test_retry_until_success() {
    let strategy = RetryStrategy::new(3, Backoff::Fixed(Duration::from_millis(10)));
    use std::sync::{Arc, Mutex};

    let state = Arc::new(Mutex::new(0));
    let state_clone = Arc::clone(&state);

    let result = retry_async(strategy, move |_| {
        let state_clone = Arc::clone(&state_clone);
        async move {
            let mut state = state_clone.lock().unwrap();
            *state += 1;
            if *state < 2 {
                Err("fail")
            } else {
                Ok("success")
            }
        }
    })
    .await;

    assert!(matches!(result, RetryResult::Success("success")));
}

#[tokio::test]
async fn test_failure_after_all_attempts() {
    let strategy = RetryStrategy::new(2, Backoff::Fixed(Duration::from_millis(10)));

    let result: RetryResult<&str, &str> =
        retry_async(strategy, |_| async { Err("fail always") }).await;

    assert!(matches!(result, RetryResult::Failure("fail always")));
}

#[tokio::test]
async fn test_exponential_backoff_success() {
    let strategy = RetryStrategy::new(
        3,
        Backoff::Exponential {
            base: Duration::from_millis(10),
            factor: 2.0,
            max: Duration::from_millis(100),
        },
    );
    use std::sync::{Arc, Mutex};

    let counter = Arc::new(Mutex::new(0));
    let counter_clone = Arc::clone(&counter);

    let result = retry_async(strategy, move |_| {
        let counter_inner = Arc::clone(&counter_clone);
        async move {
            let mut count = counter_inner.lock().unwrap();
            *count += 1;
            if *count < 2 {
                Err("error")
            } else {
                Ok("exp success")
            }
        }
    })
    .await;

    assert!(matches!(result, RetryResult::Success("exp success")));
    let final_count = *counter.lock().unwrap();
    assert_eq!(final_count, 2);
}

#[tokio::test]
async fn test_jitter_backoff_failure() {
    let strategy = RetryStrategy::new(
        1,
        Backoff::Jitter {
            base: Duration::from_millis(10),
            max_jitter: Duration::from_millis(5),
        },
    );

    let result: RetryResult<&str, &str> =
        retry_async(strategy, |_| async { Err("jitter error") }).await;

    assert!(matches!(result, RetryResult::Failure("jitter error")));
}

use std::result::Result;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct DummyLogger {
    events: Arc<Mutex<Vec<String>>>,
    errors: Arc<Mutex<Vec<String>>>,
}

impl DummyLogger {
    fn new() -> Self {
        DummyLogger {
            events: Arc::new(Mutex::new(vec![])),
            errors: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl Log for DummyLogger {
    fn log_event(&self, msg: &str) -> Result<(), anyhow::Error> {
        self.events.lock().unwrap().push(msg.to_string());
        Ok(())
    }
    fn log_error(&self, msg: &str) -> Result<(), anyhow::Error> {
        self.errors.lock().unwrap().push(msg.to_string());
        Ok(())
    }
}

#[tokio::test]
async fn test_logger_integration() {
    let dummy_logger = DummyLogger::new();
    let strategy =
        RetryStrategy::new(2, Backoff::Fixed(Duration::from_millis(10))).with_logger(dummy_logger);
    let result: RetryResult<&str, &str> = retry_async(strategy, |_| async { Err("fail") }).await;
    assert!(matches!(result, RetryResult::Failure("fail")));
}
