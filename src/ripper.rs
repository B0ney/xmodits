//! The soul of XMODITS

pub mod extraction;
pub mod handle;
pub mod signal;
pub mod stop_flag;
pub mod subscription;

pub use extraction::strict_loading;
pub use handle::Handle;
pub use signal::Signal;
pub use subscription::{xmodits_subscription, Message};
