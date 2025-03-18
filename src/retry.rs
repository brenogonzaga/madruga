use crate::{backoff::Backoff, logger::Log};

pub enum RetryResult<T, E> {
    Success(T),
    Failure(E),
}

pub struct RetryStrategy {
    pub retries: usize,
    pub backoff: Backoff,
    pub logger: Option<Box<dyn Log>>,
}

impl RetryStrategy {
    pub fn new(retries: usize, backoff: Backoff) -> Self {
        RetryStrategy {
            retries,
            backoff,
            logger: None,
        }
    }

    pub fn with_logger(mut self, logger: impl Log + 'static) -> Self {
        self.logger = Some(Box::new(logger));
        self
    }
}

pub async fn retry_async<F, Fut, T, E>(
    strategy: RetryStrategy,
    mut operation: F,
) -> RetryResult<T, E>
where
    F: FnMut(usize) -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut attempt = 0;
    loop {
        if let Some(logger) = &strategy.logger {
            let _ = logger.log_event(&format!("Attempt {} started", attempt));
        } else {
            println!("Attempt {} started", attempt);
        }
        let result = operation(attempt).await;
        match result {
            Ok(val) => {
                if let Some(logger) = &strategy.logger {
                    let _ = logger.log_event("Operation succeeded");
                } else {
                    println!("Operation succeeded");
                }
                return RetryResult::Success(val);
            }
            Err(err) => {
                if attempt >= strategy.retries {
                    if let Some(logger) = &strategy.logger {
                        let _ = logger.log_error("Operation failed after all retries");
                    } else {
                        eprintln!("Operation failed after all retries");
                    }
                    return RetryResult::Failure(err);
                } else {
                    if let Some(logger) = &strategy.logger {
                        let _ = logger.log_event("Operation failed, retrying...");
                    } else {
                        println!("Operation failed, retrying...");
                    }
                    tokio::time::sleep(strategy.backoff.delay_for(attempt)).await;
                }
            }
        }
        attempt += 1;
    }
}
