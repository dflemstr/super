use config;
use id;
use supervisor;

pub struct SupervisorRange {
    name: String,
    program: config::Program,
    index: u32,
    count: u32,
}

impl SupervisorRange {
    pub fn new(name: String, program: config::Program, count: u32)
           -> SupervisorRange {
        SupervisorRange {
            name: name,
            program: program,
            index: 0,
            count: count,
        }
    }
}

impl Iterator for SupervisorRange {
    type Item = supervisor::Supervisor;
    fn next(&mut self) -> Option<supervisor::Supervisor> {
        if self.index == self.count {
            None
        } else {
            self.index += 1;

            let i = if self.count == 1 {
                None
            } else {
                Some(self.index)
            };

            let id = id::Id::new(self.name.clone(), i);

            Some(supervisor::Supervisor::new(id, self.program.clone()))
        }
    }
}
