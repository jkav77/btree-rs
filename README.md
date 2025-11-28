> [!IMPORTANT]
> btree-rs isn't ready for production and there is no roadmap to improve it further.

### btree-rs

Small, synchronous behavior-tree primitives for Rust autonomy projects. The library
exposes a minimal set of nodes along with a typed blackboard that can be composed to prototype autonomous behavior.

### Features

- Blackboard exposes typed access so nodes can share state without tight coupling.
- `SequenceNode`, `SelectorNode`, and the `sequence!`/`selector!` macros make composition painless.
- `SyncLeafNode` lets you wrap any closure into a behavior node for quick experimentation.

### Installation

The crate is not published to crates.io, so you can install from github.

```toml
[dependencies]
btree-rs = { git = "https://github.com/<your-user>/btree-rs", tag = "main" }
```

### Usage

Build a tree by mixing custom nodes and the provided helpers, as shown in the [integration tests](./tests/integration_test.rs).

### Development

Enter the development environment with `nix develop`.

Run the unit tests to validate changes:

```bash
cargo test
```

### Roadmap Ideas

- Add the tick engine
- Add asynchronous leaf-node support
- Provide a builder-style API that hides boxing entirely.
