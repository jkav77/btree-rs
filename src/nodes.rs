use crate::core::{Context, Status};

pub trait BehaviorNode {
    fn tick(&mut self, ctx: &mut Context) -> Status;
}

pub struct SyncLeafNode<'a> {
    action: Box<dyn Fn(&mut Context) -> Status + 'a>,
}

impl<'a> SyncLeafNode<'a> {
    pub fn new<F>(action: F) -> Self
    where
        F: Fn(&mut Context) -> Status + 'a,
    {
        Self {
            action: Box::new(action),
        }
    }
}

impl<'a> BehaviorNode for SyncLeafNode<'a> {
    fn tick(&mut self, ctx: &mut Context) -> Status {
        (&self.action)(ctx)
    }
}

pub struct AlwaysFails {}
impl BehaviorNode for AlwaysFails {
    fn tick(&mut self, _: &mut Context) -> Status {
        Status::Failure
    }
}

pub struct AlwaysSucceeds {}
impl BehaviorNode for AlwaysSucceeds {
    fn tick(&mut self, _: &mut Context) -> Status {
        Status::Success
    }
}

pub struct AlwaysRunning {}
impl BehaviorNode for AlwaysRunning {
    fn tick(&mut self, _: &mut Context) -> Status {
        Status::Running
    }
}

pub struct SequenceNode {
    children: Vec<Box<dyn BehaviorNode>>,
}

impl SequenceNode {
    pub fn new(children: Vec<Box<dyn BehaviorNode>>) -> Self {
        SequenceNode { children }
    }
}

impl BehaviorNode for SequenceNode {
    fn tick(&mut self, ctx: &mut Context) -> Status {
        for node in self.children.iter_mut() {
            match node.tick(ctx) {
                Status::Running => return Status::Running,
                Status::Failure => return Status::Failure,
                Status::Success => continue,
            }
        }
        Status::Success
    }
}

#[macro_export]
macro_rules! sequence {
    ($($node:expr),+ $(,)?) => {{
        let nodes: Vec<Box<dyn $crate::BehaviorNode>> =
            vec![$(Box::new($node) as Box<dyn $crate::BehaviorNode>),+];
        $crate::SequenceNode::new(nodes)
    }};
}

pub struct SelectorNode {
    children: Vec<Box<dyn BehaviorNode>>,
    current_index: usize,
}

impl SelectorNode {
    pub fn new(children: Vec<Box<dyn BehaviorNode>>) -> Self {
        SelectorNode {
            current_index: 0,
            children,
        }
    }
}

impl BehaviorNode for SelectorNode {
    fn tick(&mut self, ctx: &mut Context) -> Status {
        while self.current_index < self.children.len() {
            match self.children[self.current_index].tick(ctx) {
                Status::Success => {
                    self.current_index = 0;
                    return Status::Success;
                }
                Status::Running => {
                    return Status::Running;
                }
                Status::Failure => {
                    self.current_index += 1;
                }
            }
        }
        self.current_index = 0;
        Status::Failure
    }
}

#[macro_export]
macro_rules! selector {
    ($($node:expr),+ $(,)?) => {{
        let nodes: Vec<Box<dyn $crate::BehaviorNode>> =
            vec![$(Box::new($node) as Box<dyn $crate::BehaviorNode>),+];
        $crate::SelectorNode::new(nodes)
    }};
}
