use std::fmt;

#[derive(Debug)]
pub struct Id {
    name: String,
    instance: Option<u32>,
}

impl Id {
    pub fn new(name: String, instance: Option<u32>) -> Id {
        Id {
            name: name,
            instance: instance,
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        if let Some(instance) = self.instance {
            write!(f, "{}[{}]", self.name, instance)
        } else {
            write!(f, "{}", self.name)
        }
    }
}
