use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::job::Job;
use crate::scheduler::SchedulerError::JobAlreadyExists;

pub struct Scheduler {
    jobs: HashMap<String, Job>,
}

#[derive(Debug, Clone)]
enum SchedulerError {
    JobAlreadyExists(Job)
}

impl Display for SchedulerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!("Display for Scheduler Error")
    }
}

impl Error for SchedulerError {

}

impl Scheduler {

    pub fn new() -> Self {
        Scheduler {
            jobs: HashMap::new()
        }
    }

    pub fn add_job(&mut self, job: Job) -> Result<(), SchedulerError> {
        if self.jobs.contains_key(job.name()) {
            Err(JobAlreadyExists(job))
        } else {
            self.jobs.insert(job.name().to_string(), job);
            Ok(())
        }
    }

    pub fn remove_job() {
        unimplemented!("remove_job")
    }

    pub fn get_job() {
        unimplemented!("get_job")
    }

    pub fn run() {
        unimplemented!("run")
    }
}