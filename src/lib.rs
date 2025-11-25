mod core;
mod nodes;

pub use crate::core::{Blackboard, Context, Status};
pub use crate::nodes::{
    AlwaysFails, AlwaysRunning, AlwaysSucceeds, BehaviorNode, SequenceNode, SyncLeafNode,
};

#[macro_export]
macro_rules! sequence {
    ($($node:expr),+ $(,)?) => {{
        let nodes: Vec<Box<dyn $crate::BehaviorNode>> =
            vec![$(Box::new($node) as Box<dyn $crate::BehaviorNode>),+];
        $crate::SequenceNode::new(nodes)
    }};
}

#[cfg(test)]
mod tests {
    use crate::{
        AlwaysFails, AlwaysRunning, AlwaysSucceeds,
        core::{Context, Status},
        nodes::{BehaviorNode, SyncLeafNode},
    };

    #[test]
    fn always_fails() {
        let node = AlwaysFails {};
        let mut ctx = Context::new();
        assert_eq!(node.tick(&mut ctx), Status::Failure);
    }

    #[test]
    fn always_succeeds() {
        let node = AlwaysSucceeds {};
        let mut ctx = Context::new();
        assert_eq!(node.tick(&mut ctx), Status::Success);
    }

    #[test]
    fn sync_action_node() {
        let action = |_ctx: &mut Context| Status::Running;
        let node: SyncLeafNode<'static> = SyncLeafNode::new(action);
        let mut ctx = Context::new();
        assert_eq!(node.tick(&mut ctx), Status::Running);
    }

    #[test]
    fn sequence_fails() {
        let mut ctx = Context::new();
        let sequence_node = sequence![AlwaysFails {}, AlwaysSucceeds {}];
        assert_eq!(sequence_node.tick(&mut ctx), Status::Failure);

        let sequence_node = sequence![AlwaysSucceeds {}, AlwaysFails {}];
        assert_eq!(sequence_node.tick(&mut ctx), Status::Failure);
    }

    #[test]
    fn sequence_succeeds() {
        let sequence_node = sequence![AlwaysSucceeds {}, AlwaysSucceeds {}];
        let mut ctx = Context::new();
        assert_eq!(sequence_node.tick(&mut ctx), Status::Success)
    }

    #[test]
    fn sequence_running() {
        let sequence_node = sequence![AlwaysRunning {}, AlwaysSucceeds {}];
        let mut ctx = Context::new();
        assert_eq!(sequence_node.tick(&mut ctx), Status::Running)
    }
}
