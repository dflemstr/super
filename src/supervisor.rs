use std::process;

use config;
use id;

pub struct Supervisor {
    id: id::Id,
    program: config::Program,
    process: Option<process::Child>,
}

impl Supervisor {
    pub fn new(id: id::Id, program: config::Program) -> Supervisor {
        Supervisor {
            id: id,
            program: program,
            process: None,
        }
    }

    pub fn start(&self) {
        println!("[{}] Starting supervisor", self.id);
    }

    pub fn stop(&self) {
        println!("[{}] Stopping supervisor", self.id);
    }
}
