use std::process;

use config;

pub struct Supervisor {
    id: String,
    program: config::Program,
    process: Option<process::Child>,
}

impl Supervisor {
    pub fn start(id: String, program: config::Program) -> Supervisor {
        Supervisor {
            id: id,
            program: program,
            process: None,
        }
    }
}
