use std::collections::HashMap;
use crate::job::Job;

pub struct Scheduler {
    jobs: HashMap<String, Job>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            jobs: HashMap::new()
        }
    }

    pub fn add_job() {
        unimplemented!("add_job")
    }

    pub fn remove_job() {
        unimplemented!("remove_job")
    }

    
}