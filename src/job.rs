use std::time::Duration;
use chrono::Utc;
pub struct Job {
    name: String,
    expire: Utc,
    interval: Duration,
    task: Box<dyn Fn() + Send + 'static>,
}