use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommunicationConfig {
    /// Address this robot listens on for incoming commands (TCP).
    /// e.g. "0.0.0.0:9000"
    pub command_listen_addr: String,
    /// Address of the station to send telemetry to (UDP).
    /// e.g. "192.168.1.100:9001"
    pub telemetry_target_addr: String,
    /// Local UDP bind address for sending telemetry.
    /// e.g. "0.0.0.0:0"  (let OS pick a port)
    pub telemetry_bind_addr: String,
}