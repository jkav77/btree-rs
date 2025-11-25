use std::{any::Any, collections::HashMap};

#[derive(Debug, PartialEq)]
pub enum Status {
    Success,
    Failure,
    Running,
}

pub struct Context {
    pub blackboard: Blackboard,
}

impl Context {
    pub fn new() -> Self {
        Context {
            blackboard: Blackboard::new(),
        }
    }
}

/// Blackboard provides typed access to a heterogeneous map keyed by strings.
pub struct Blackboard {
    entries: HashMap<String, Box<dyn Any>>,
}

impl Blackboard {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Store a value under the provided key, replacing the existing value if present.
    pub fn insert<T: 'static>(&mut self, key: impl Into<String>, value: T) {
        self.entries.insert(key.into(), Box::new(value));
    }

    /// Retrieve an immutable reference to a typed value if the key exists and the type matches.
    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.entries.get(key)?.downcast_ref::<T>()
    }

    /// Retrieve a mutable reference to a typed value if it exists and the type matches.
    pub fn get_mut<T: 'static>(&mut self, key: &str) -> Option<&mut T> {
        self.entries.get_mut(key)?.downcast_mut::<T>()
    }

    /// Returns true when the key exists independent of the stored value type.
    pub fn contains(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    /// Remove a typed value if it exists and matches the requested type.
    pub fn remove<T: 'static>(&mut self, key: &str) -> Option<T> {
        let value = self.entries.remove(key)?;
        value.downcast::<T>().ok().map(|boxed| *boxed)
    }
}
