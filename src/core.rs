use std::{any::Any, collections::HashMap};

#[derive(Debug, PartialEq)]
pub enum Status {
    Success,
    Failure,
    Running,
}

pub struct Blackboard {
    blackboard: HashMap<String, Box<dyn Any>>,
}

impl Blackboard {
    pub fn new() -> Self {
        Blackboard {
            blackboard: HashMap::new(),
        }
    }
}

pub struct Context {
    blackboard: Blackboard,
}

impl Context {
    pub fn new() -> Self {
        Context {
            blackboard: Blackboard::new(),
        }
    }
}
