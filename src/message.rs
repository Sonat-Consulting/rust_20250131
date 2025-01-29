use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use iroh::NodeId;

#[derive(Deserialize, Serialize, Debug)]
pub enum Message {
    AboutMe { from: NodeId, name: String },
    Message { from: NodeId, text: String },
    AboutUs { names: HashMap<NodeId, String>}
}

impl Message {
    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("infallible")
    }
}