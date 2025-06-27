pub mod error;
pub mod model;
pub mod utils;
pub mod foundation;
pub mod protocol_model { include!(concat!(env!("OUT_DIR"), "/ws.protocol.rs")); }