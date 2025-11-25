use crate::core::{Context, Status};

pub trait BehaviorNode {
    fn tick(&self, ctx: &mut Context) -> Status;
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
    fn tick(&self, ctx: &mut Context) -> Status {
        (&self.action)(ctx)
    }
}

pub struct AlwaysFails {}
impl BehaviorNode for AlwaysFails {
    fn tick(&self, _: &mut Context) -> Status {
        Status::Failure
    }
}

pub struct AlwaysSucceeds {}
impl BehaviorNode for AlwaysSucceeds {
    fn tick(&self, _: &mut Context) -> Status {
        Status::Success
    }
}

pub struct AlwaysRunning {}
impl BehaviorNode for AlwaysRunning {
    fn tick(&self, _: &mut Context) -> Status {
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
    fn tick(&self, ctx: &mut Context) -> Status {
        for node in &self.children {
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
