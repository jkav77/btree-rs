use btree_rs::{
    self, AlwaysSucceeds, BehaviorNode, Context, Status, SyncLeafNode, selector, sequence,
};

#[test]
fn basic_tree() {
    let mut ctx = Context::new();
    let two_tick_failure_node =
        SyncLeafNode::new(|ctx| match ctx.blackboard.get::<&str>("node_will_fail") {
            Some(val) => {
                println!(
                    "Failing node_will_fail becuase value \"{}\" is in blackboard key",
                    val
                );
                btree_rs::Status::Failure
            }
            None => {
                println!("Writing to node_will_fail blackboard key");
                ctx.blackboard.insert("node_will_fail", "value");
                btree_rs::Status::Running
            }
        });
    let two_tick_success_node =
        SyncLeafNode::new(
            |ctx| match ctx.blackboard.get::<&str>("node_will_succeed") {
                Some(val) => {
                    println!(
                        "Succeeding in node_will_succeed becuase value \"{}\" is in blackboard key",
                        val
                    );
                    btree_rs::Status::Success
                }
                None => {
                    println!("Writing to node_will_succeed blackboard key");
                    ctx.blackboard.insert("node_will_succeed", "value");
                    btree_rs::Status::Running
                }
            },
        );
    let sequence1 = sequence![AlwaysSucceeds {}, two_tick_failure_node,];
    let sequence2 = sequence![AlwaysSucceeds {}, two_tick_success_node,];
    let mut root = selector![sequence1, sequence2];
    println!("tick 1");
    assert_eq!(root.tick(&mut ctx), Status::Running);
    println!("tick 2");
    assert_eq!(root.tick(&mut ctx), Status::Running);
    println!("tick 3");
    assert_eq!(root.tick(&mut ctx), Status::Success);
}
