use std::{any::Any, collections::HashMap};

#[derive(Debug, PartialEq)]
pub enum Status {
    Success,
    Failure,
    Running,
}

pub struct Context {
    blackboard: HashMap<String, Box<dyn Any>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            blackboard: HashMap::new(),
        }
    }
}
