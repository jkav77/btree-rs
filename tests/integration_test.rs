use btree_rs::{self, AlwaysSucceeds, BehaviorNode, Context, Status, SyncLeafNode, sequence};

#[test]
fn simple_sequence() {
    let mut ctx = Context::new();
    let root = sequence![
        AlwaysSucceeds {},
        SyncLeafNode::new(|ctx| match ctx.blackboard.get::<&str>("test") {
            Some(_) => btree_rs::Status::Success,
            None => {
                ctx.blackboard.insert("test", "value");
                btree_rs::Status::Running
            }
        })
    ];
    assert_eq!(root.tick(&mut ctx), Status::Running);
    assert_eq!(root.tick(&mut ctx), Status::Success);
}
