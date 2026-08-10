#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod component_pool;
mod entity;

pub use component_pool::ComponentPool;
pub use entity::EntityId;
