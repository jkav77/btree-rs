use crate::core::{Context, Status};

/// Trait implemented by every node that can participate in a behavior tree.
pub trait BehaviorNode {
    /// Tick the node, mutating the [`Context`] and returning the resulting [`Status`].
    fn tick(&mut self, ctx: &mut Context) -> Status;
}

/// A leaf node that runs a provided action
///
/// # Examples
/// ```
/// use btree_rs::{BehaviorNode, Context, Status};
///
/// let mut ctx = Context::new();
/// let action = |_ctx: &mut Context| Status::Running;
/// let mut node = btree_rs::SyncLeafNode::new(action);
/// assert_eq!(node.tick(&mut ctx), Status::Running);
/// ```
pub struct SyncLeafNode<'a> {
    action: Box<dyn Fn(&mut Context) -> Status + 'a>,
}

impl<'a> SyncLeafNode<'a> {
    /// Wrap a closure in a [`BehaviorNode`] implementation.
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

/// Utility node that always reports [`Status::Failure`].
///
/// # Examples
/// ```
/// use btree_rs::{BehaviorNode, Context, Status};
///
/// let mut node = btree_rs::AlwaysFails {};
/// let mut ctx = Context::new();
/// assert_eq!(node.tick(&mut ctx), Status::Failure);
/// ```
pub struct AlwaysFails {}
impl BehaviorNode for AlwaysFails {
    fn tick(&mut self, _: &mut Context) -> Status {
        Status::Failure
    }
}

/// Utility node that always reports [`Status::Success`].
///
/// # Examples
/// ```
/// use btree_rs::{BehaviorNode, Context, Status};
///
/// let mut ctx = Context::new();
/// let mut node = btree_rs::AlwaysSucceeds {};
/// assert_eq!(node.tick(&mut ctx), Status::Success);
/// ```
pub struct AlwaysSucceeds {}
impl BehaviorNode for AlwaysSucceeds {
    fn tick(&mut self, _: &mut Context) -> Status {
        Status::Success
    }
}

/// Utility node that always reports [`Status::Running`].
pub struct AlwaysRunning {}
impl BehaviorNode for AlwaysRunning {
    fn tick(&mut self, _: &mut Context) -> Status {
        Status::Running
    }
}

/// A sequence node that may tick multiple children in one tick.
///
/// If any child returns running, execution will continue with that child on the next tick.
///
/// # Returns
/// - [`Status::Success`] if all children succeed
/// - [`Status::Running`] if a child returns running
/// - [`Status::Failure`] when any child returns failure
///
/// # Examples
/// ```
/// use btree_rs::{AlwaysSucceeds, AlwaysFails, AlwaysRunning, BehaviorNode, Status, Context};
///
/// let mut ctx = Context::new();
/// let mut sequence_node = btree_rs::sequence![AlwaysFails {}, AlwaysSucceeds {}];
/// assert_eq!(sequence_node.tick(&mut ctx), Status::Failure);
///
/// let mut sequence_node = btree_rs::sequence![AlwaysSucceeds {}, AlwaysFails {}];
/// assert_eq!(sequence_node.tick(&mut ctx), Status::Failure);
///
/// let mut sequence_node = btree_rs::sequence![AlwaysSucceeds {}, AlwaysSucceeds {}];
/// assert_eq!(sequence_node.tick(&mut ctx), Status::Success);
///
/// let mut sequence_node = btree_rs::sequence![AlwaysRunning {}, AlwaysSucceeds {}];
/// assert_eq!(sequence_node.tick(&mut ctx), Status::Running);
/// ```
pub struct SequenceNode {
    children: Vec<Box<dyn BehaviorNode>>,
}

impl SequenceNode {
    /// Create a [`SequenceNode`] from owned children.
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

/// Convenience macro for building a [`SequenceNode`] without manual boxing.
#[macro_export]
macro_rules! sequence {
    ($($node:expr),+ $(,)?) => {{
        let nodes: Vec<Box<dyn $crate::BehaviorNode>> =
            vec![$(Box::new($node) as Box<dyn $crate::BehaviorNode>),+];
        $crate::SequenceNode::new(nodes)
    }};
}

/// A selector node that may tick multiple children in one tick until one returns [`Status::Success`]
///
/// # Returns
/// - [`Status::Success`] if any child returns [`Status::Success`]
/// - [`Status::Running`] if a child returns [`Status::Running`]
/// - [`Status::Failure`] if all nodes return [`Status::Failure`]
///
/// # Examples
/// ```
/// use btree_rs::{AlwaysSucceeds, AlwaysFails, AlwaysRunning, BehaviorNode, Status, Context};
///
/// let mut ctx = Context::new();
/// let mut selector_node = btree_rs::SelectorNode::new(vec![Box::new(AlwaysSucceeds {})]);
/// assert_eq!(selector_node.tick(&mut ctx), Status::Success);
///
/// let mut selector_node = btree_rs::SelectorNode::new(vec![Box::new(AlwaysFails {}), Box::new(AlwaysSucceeds {})]);
/// assert_eq!(selector_node.tick(&mut ctx), Status::Success);
///
/// let mut selector_node = btree_rs::SelectorNode::new(vec![Box::new(AlwaysFails {})]);
/// assert_eq!(selector_node.tick(&mut ctx), Status::Failure);
///
/// let mut selector_node = btree_rs::SelectorNode::new(vec![Box::new(AlwaysRunning {})]);
/// assert_eq!(selector_node.tick(&mut ctx), Status::Running);
///
/// let mut selector_node = btree_rs::SelectorNode::new(vec![Box::new(AlwaysRunning {}), Box::new(AlwaysSucceeds {})]);
/// assert_eq!(selector_node.tick(&mut ctx), Status::Running);
/// ```
pub struct SelectorNode {
    children: Vec<Box<dyn BehaviorNode>>,
    current_index: usize,
}

impl SelectorNode {
    /// Create a [`SelectorNode`] with owned children.
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

/// Convenience macro for building a [`SelectorNode`] without manual boxing.
#[macro_export]
macro_rules! selector {
    ($($node:expr),+ $(,)?) => {{
        let nodes: Vec<Box<dyn $crate::BehaviorNode>> =
            vec![$(Box::new($node) as Box<dyn $crate::BehaviorNode>),+];
        $crate::SelectorNode::new(nodes)
    }};
}
