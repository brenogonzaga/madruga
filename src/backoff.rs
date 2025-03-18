use rand::Rng;
use std::time::Duration;

#[derive(Clone, Debug)]
pub enum Backoff {
    Fixed(Duration),
    Exponential {
        base: Duration,
        factor: f64,
        max: Duration,
    },
    Jitter {
        base: Duration,
        max_jitter: Duration,
    },
}

impl Backoff {
    pub fn delay_for(&self, attempt: usize) -> Duration {
        match self {
            Backoff::Fixed(d) => *d,
            Backoff::Exponential { base, factor, max } => {
                let delay = base.as_millis() as f64 * factor.powi(attempt as i32);
                Duration::from_millis(delay.min(max.as_millis() as f64) as u64)
            }
            Backoff::Jitter { base, max_jitter } => {
                let mut rng = rand::rng();
                let jitter = rng.random_range(0..=max_jitter.as_millis() as u64);
                *base + Duration::from_millis(jitter)
            }
        }
    }
}
