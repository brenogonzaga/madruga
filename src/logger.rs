use anyhow::Result;
use black::BlackBox;

pub trait Log: Send + Sync {
    fn log_event(&self, event: &str) -> Result<()>;
    fn log_error(&self, error: &str) -> Result<()>;
}

pub struct Logger {
    inner: BlackBox,
}

impl Logger {
    pub fn new(filename: &str) -> Result<Self> {
        let inner = BlackBox::new(filename)?;
        Ok(Logger { inner })
    }
}

impl Log for Logger {
    fn log_event(&self, event: &str) -> Result<()> {
        self.inner.log_event(event)
    }
    fn log_error(&self, error: &str) -> Result<()> {
        self.inner.log_error(error)
    }
}
