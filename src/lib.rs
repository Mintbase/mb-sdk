#[cfg(any(feature = "de", feature = "ser"))]
pub mod events;

pub mod types;

// re-exports
pub use near_events;
pub use near_sdk;

// TODO: defining a prelude
