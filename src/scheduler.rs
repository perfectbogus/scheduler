use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use chrono::Utc;
use crate::job::Job;


pub struct Scheduler {
    jobs: HashMap<String, Job>,
}

#[derive(Debug, Clone)]
enum SchedulerError {
    JobAlreadyExists(String),
    JobDoesntExists(String),
}

impl Display for SchedulerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SchedulerError::JobAlreadyExists(job) =>
                writeln!(f, "Job {} already exists", job),
            SchedulerError::JobDoesntExists(job_name) =>
                writeln!(f, "Job {} does not exists", job_name)
        }
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
            Err(SchedulerError::JobAlreadyExists(job.name().to_string()))
        } else {
            self.jobs.insert(job.name().to_string(), job);
            Ok(())
        }
    }

    pub fn remove_job(&mut self, job_name: &str) -> Result<Job, SchedulerError>{
        if !self.jobs.contains_key(job_name) {
            Err(SchedulerError::JobDoesntExists(job_name.to_string()))
        } else {
            Ok(self.jobs.remove(job_name).unwrap())
        }
    }

    pub fn get_job(&self, job_name: &str) -> Option<&Job> {
        self.jobs.get(job_name)
    }

    pub fn run(&mut self) {
        let mut jobs_to_remove = Vec::new();

        for (name, job) in self.jobs.iter_mut() {
            if job.is_due() {
                job.execute();
            }

            if job.should_remove() {
                jobs_to_remove.push(name.clone());
            }
        }

        // Remove expired jobs
        for name in jobs_to_remove {
            self.jobs.remove(&name);
        }
    }
}