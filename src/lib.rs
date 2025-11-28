//! `btree-rs` provides simple building blocks for constructing synchronous behavior trees.
//!
//! The crate intentionally focuses on an ergonomic API for prototyping AIs or simulation agents.
//! Import the types you need from the prelude-like re-exports and use [`sequence!`] or
//! [`selector!`] helpers to wire composites together.

mod core;
mod nodes;

pub use crate::core::{Blackboard, Context, Status};
pub use crate::nodes::{
    AlwaysFails, AlwaysRunning, AlwaysSucceeds, BehaviorNode, SelectorNode, SequenceNode,
    SyncLeafNode,
};
