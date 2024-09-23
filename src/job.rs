use std::fmt::{Display, Formatter};
use std::error::Error;
use std::ops::Add;
use chrono::{Utc, DateTime, Duration};

#[derive(Debug, Clone)]
pub struct Job {
    name: String,
    expire: DateTime<Utc>,
    interval: Duration,
    message: String,
    last_time_executed: DateTime<Utc>
}

#[derive(Debug, Clone)]
enum JobError {
    ExpirationInPast(DateTime<Utc>),
    ZeroInterval,
}

impl Error for JobError {

}

impl Display for JobError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JobError::ExpirationInPast(date) =>
                writeln!(f, "Job Expiration date is in the past {:?}", date),
            JobError::ZeroInterval =>
                writeln!(f, "Job Interval Zero not allowed"),
        }
    }
}

impl Job {
    pub fn new(name: &str, expire: DateTime<Utc>, interval: Duration, message: &str) -> Result<Job, JobError> {
        if Self::datetime_is_in_the_past(expire) {
            Err(JobError::ExpirationInPast(expire))
        } else if Self::interval_is_zero(interval) {
            Err(JobError::ZeroInterval)
        } else {
            Ok(Self {
                name: name.to_string(),
                expire,
                interval,
                message: message.to_string(),
                last_time_executed: Utc::now(),
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

    pub fn execute(&mut self) {
        println!("{} {}",Utc::now(), self.message);
        self.last_time_executed = Utc::now();
    }

    fn datetime_is_in_the_past(datetime: DateTime<Utc>) -> bool {
        datetime < Utc::now()
    }

    fn interval_is_zero(interval: Duration) -> bool {
        interval.is_zero()
    }

    pub fn update_expiration(&mut self, expiration: DateTime<Utc>) -> Result<(), JobError>{
        if Self::datetime_is_in_the_past(expiration) {
            Err(JobError::ExpirationInPast(expiration))
        } else {
            self.expire = expiration;
            Ok(())
        }
    }

    pub fn is_due(&self) -> bool {
        Utc::now() >= self.last_time_executed.add(self.interval)
    }

    pub fn next_run_time(&self) -> DateTime<Utc> {
        self.last_time_executed.add(self.interval)
    }

    pub fn should_remove(&self) -> bool {
        Utc::now() >= self.expire
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration as CDuration, TimeDelta};

    #[test]
    fn when_valid_parameters_create_job() {
        let name = "test-job-valid-parameters";
        let expire = Utc::now() + CDuration::hours(1);
        let interval = CDuration::seconds(15);
        let message = "test-job-message";

        let result = Job::new(name, expire, interval, message);
        assert!(result.is_ok());

        let job = result.unwrap();
        assert_eq!(job.name(), name);
        assert_eq!(job.expire(), expire);
        assert_eq!(job.interval(), interval);
        assert_eq!(job.message(), message);
    }

    #[test]
    fn when_invalid_expire_get_job_error() {
        let name = "test-job-invalid-expire";
        let expire_in_past = Utc::now() - CDuration::hours(1);
        let interval = CDuration::seconds(15);
        let message = "test job invalid expire";

        let result = Job::new(name, expire_in_past, interval, message);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(error, JobError::ExpirationInPast(..)))
    }

    #[test]
    fn when_invalid_zero_interval_get_job_error() {
        let name = "test-job-invalid-interval";
        let expire = Utc::now() + CDuration::hours(1);
        let zero_interval = CDuration::seconds(0);
        let message = "test job invalid interval";

        let result = Job::new(name, expire, zero_interval, message);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(error, JobError::ZeroInterval))
    }

    #[test]
    fn when_job_executed_validated_is_due() {

    }

    #[test]
    fn when_job_executed_validate_execution() {

    }

    #[test]
    fn when_update_expiration_validate() {

    }

    #[test]
    fn when_job_executed_check_next_run_time() {

    }

    #[test]
    fn when_job_expire_remove_it() {
        
    }
}