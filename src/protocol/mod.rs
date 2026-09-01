//! SecureBridge Protocol Implementation

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub version: u8,
    pub msg_type: MessageType,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Handshake,
    Data,
    Control,
    Heartbeat,
}

pub struct SecureBridge {
    state: BridgeState,
}

#[derive(Debug, Clone)]
enum BridgeState {
    Disconnected,
    Connecting,
    Connected,
}

impl SecureBridge {
    pub fn new() -> Self {
        Self {
            state: BridgeState::Disconnected,
        }
    }
    
    pub fn connect(&mut self) {
        self.state = BridgeState::Connecting;
    }
    
    pub fn is_connected(&self) -> bool {
        matches!(self.state, BridgeState::Connected)
    }
}
