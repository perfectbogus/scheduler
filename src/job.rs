use std::fmt::{Display, Formatter};
use std::time::Duration;
use chrono::{Utc, DateTime};

#[derive(Debug, Clone)]
pub struct Job {
    name: String,
    expire: DateTime<Utc>,
    interval: Duration,
    message: String,
}

#[derive(Debug, Clone)]
enum JobError {
    ExpirationInPast(DateTime<Utc>),
}

impl Display for JobError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"Datetime Error:")
    }
}

impl Job {
    pub fn new(name: &str, expire: DateTime<Utc>, interval: Duration, message: &str) -> Result<Job, JobError> {
        if !Self::is_greater_than_now(expire) {
            Err(JobError::ExpirationInPast(expire))
        } else {
            Ok(Self {
                name: name.to_string(),
                expire,
                interval,
                message: message.to_string(),
            })
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn expire(&self) -> DateTime<Utc> {
        self.expire
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }

    pub fn message(&self) -> &str  {
        &self.message
    }

    pub fn execute(&self) {
        println!("{} {}",Utc::now(), self.message);
    }

    pub fn has_expired(&self) -> bool {
        self.expire < Utc::now()
    }

    fn is_greater_than_now(datetime: DateTime<Utc>) -> bool {
        datetime > Utc::now()
    }

}