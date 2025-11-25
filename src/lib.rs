mod core;
mod nodes;

pub use crate::core::{Blackboard, Context, Status};
pub use crate::nodes::{
    AlwaysFails, AlwaysRunning, AlwaysSucceeds, BehaviorNode, SelectorNode, SequenceNode,
    SyncLeafNode,
};

#[cfg(test)]
mod tests {
    use crate::{
        AlwaysSucceeds,
        core::{Context, Status},
        nodes::{AlwaysFails, AlwaysRunning, BehaviorNode, SelectorNode, SyncLeafNode},
        selector, sequence,
    };

    #[test]
    fn always_fails() {
        let mut node = AlwaysFails {};
        let mut ctx = Context::new();
        assert_eq!(node.tick(&mut ctx), Status::Failure);
    }

    #[test]
    fn always_succeeds() {
        let mut node = AlwaysSucceeds {};
        let mut ctx = Context::new();
        assert_eq!(node.tick(&mut ctx), Status::Success);
    }

    #[test]
    fn sync_action_node() {
        let action = |_ctx: &mut Context| Status::Running;
        let mut node: SyncLeafNode<'static> = SyncLeafNode::new(action);
        let mut ctx = Context::new();
        assert_eq!(node.tick(&mut ctx), Status::Running);
    }

    #[test]
    fn sequence_fails() {
        let mut ctx = Context::new();
        let mut sequence_node = sequence![AlwaysFails {}, AlwaysSucceeds {}];
        assert_eq!(sequence_node.tick(&mut ctx), Status::Failure);

        let mut sequence_node = sequence![AlwaysSucceeds {}, AlwaysFails {}];
        assert_eq!(sequence_node.tick(&mut ctx), Status::Failure);
    }

    #[test]
    fn sequence_succeeds() {
        let mut sequence_node = sequence![AlwaysSucceeds {}, AlwaysSucceeds {}];
        let mut ctx = Context::new();
        assert_eq!(sequence_node.tick(&mut ctx), Status::Success)
    }

    #[test]
    fn sequence_running() {
        let mut sequence_node = sequence![AlwaysRunning {}, AlwaysSucceeds {}];
        let mut ctx = Context::new();
        assert_eq!(sequence_node.tick(&mut ctx), Status::Running)
    }

    #[test]
    fn selector_success() {
        let mut selector_node = SelectorNode::new(vec![Box::new(AlwaysSucceeds {})]);
        let mut ctx = Context::new();
        assert_eq!(selector_node.tick(&mut ctx), Status::Success);

        let mut selector_node =
            SelectorNode::new(vec![Box::new(AlwaysFails {}), Box::new(AlwaysSucceeds {})]);
        assert_eq!(selector_node.tick(&mut ctx), Status::Success);
    }

    #[test]
    fn selector_failure() {
        let mut selector_node = SelectorNode::new(vec![Box::new(AlwaysFails {})]);
        let mut ctx = Context::new();
        assert_eq!(selector_node.tick(&mut ctx), Status::Failure);
    }

    #[test]
    fn selector_running() {
        let mut selector_node = SelectorNode::new(vec![Box::new(AlwaysRunning {})]);
        let mut ctx = Context::new();
        assert_eq!(selector_node.tick(&mut ctx), Status::Running);

        let mut selector_node = SelectorNode::new(vec![
            Box::new(AlwaysRunning {}),
            Box::new(AlwaysSucceeds {}),
        ]);
        assert_eq!(selector_node.tick(&mut ctx), Status::Running);
    }

    #[test]
    fn selector_macro() {
        let mut selector_node = selector![AlwaysSucceeds {}];
        let mut ctx = Context::new();
        assert_eq!(selector_node.tick(&mut ctx), Status::Success);
    }
}
