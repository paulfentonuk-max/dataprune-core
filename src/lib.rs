//! Dataprune Core Library
//! 
//! Cryptographic primitives, SecureBridge protocol, and shared utilities
//! for the Dataprune AI optimization platform.

pub mod crypto;
pub mod protocol;
pub mod common;

// Re-export commonly used items
pub use crypto::{Cipher, KeyExchange};
pub use protocol::{SecureBridge, Message, MessageType};
pub use common::{Result, Error};
