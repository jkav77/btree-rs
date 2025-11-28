### btree-rs

Small, synchronous behavior-tree primitives for Rust game and simulation projects. The library
exposes a minimal set of building blocks (context, blackboard, leaf nodes, selectors, and sequences)
that can be easily composed to prototype AI decision making or as a teaching aid.

#### Status

The crate is pre-release but already usable. APIs follow idiomatic Rust documentation standards so
you can comfortably publish it to crates.io or showcase it on GitHub.

### Features

- Blackboard exposes typed access so nodes can share state without tight coupling.
- `Context` is intentionally lightweight and easy to extend for domain-specific data.
- `SequenceNode`, `SelectorNode`, and the `sequence!`/`selector!` macros make composition painless.
- `SyncLeafNode` lets you wrap any closure into a behavior node for quick experimentation.

### Installation

Until the crate is published to crates.io, use a git dependency in your project:

```toml
[dependencies]
btree-rs = { git = "https://github.com/<your-user>/btree-rs", tag = "main" }
```

After publishing, switch to `btree-rs = "0.1"` in your `Cargo.toml`.

### Usage

Build a tree by mixing custom nodes and the provided helpers:

```rust
use btree_rs::{
    selector, sequence, AlwaysFails, AlwaysRunning, Blackboard, Context, SelectorNode,
    SequenceNode, Status, SyncLeafNode,
};

fn main() {
    let mut ctx = Context::new();
    ctx.blackboard.insert("attempts", 0usize);

    let mut patrol = SyncLeafNode::new(|ctx: &mut Context| {
        let attempts = ctx.blackboard.get_mut::<usize>("attempts").unwrap();
        *attempts += 1;
        if *attempts < 3 {
            Status::Running
        } else {
            Status::Success
        }
    });

    // Try patrol, then fallback to other actions.
    let mut behavior = selector![
        patrol,
        sequence![AlwaysRunning {}, AlwaysFails {}],
        SelectorNode::new(vec![Box::new(AlwaysRunning {})])
    ];

    assert_eq!(behavior.tick(&mut ctx), Status::Running);
}
```

### Development

Run the unit tests to validate changes:

```bash
cargo test
```

### Roadmap Ideas

- Add asynchronous leaf-node support (e.g., futures) for game loops that tick per frame.
- Provide a builder-style API that hides boxing entirely.
- Offer serde helpers so blackboards can be inspected at runtime.

Issues and pull requests are welcome.
