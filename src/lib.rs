mod core;
mod nodes;

pub use crate::core::{Blackboard, Context, Status};
pub use crate::nodes::{
    AlwaysFails, AlwaysRunning, AlwaysSucceeds, BehaviorNode, SequenceNode, SyncLeafNode,
};

#[cfg(test)]
mod tests {
    use crate::{
        AlwaysFails, AlwaysRunning, AlwaysSucceeds,
        core::{Context, Status},
        nodes::{BehaviorNode, SequenceNode, SyncLeafNode},
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
        let nodes: Vec<Box<dyn BehaviorNode>> =
            vec![Box::new(AlwaysFails {}), Box::new(AlwaysSucceeds {})];
        let sequence_node = SequenceNode::new(nodes);
        assert_eq!(sequence_node.tick(&mut ctx), Status::Failure);

        let nodes: Vec<Box<dyn BehaviorNode>> =
            vec![Box::new(AlwaysSucceeds {}), Box::new(AlwaysFails {})];
        let sequence_node = SequenceNode::new(nodes);
        assert_eq!(sequence_node.tick(&mut ctx), Status::Failure);
    }

    #[test]
    fn sequence_succeeds() {
        let nodes: Vec<Box<dyn BehaviorNode>> =
            vec![Box::new(AlwaysSucceeds {}), Box::new(AlwaysSucceeds {})];
        let sequence_node = SequenceNode::new(nodes);
        let mut ctx = Context::new();
        assert_eq!(sequence_node.tick(&mut ctx), Status::Success)
    }

    #[test]
    fn sequence_running() {
        let nodes: Vec<Box<dyn BehaviorNode>> =
            vec![Box::new(AlwaysRunning {}), Box::new(AlwaysSucceeds {})];
        let sequence_node = SequenceNode::new(nodes);
        let mut ctx = Context::new();
        assert_eq!(sequence_node.tick(&mut ctx), Status::Running)
    }
}
