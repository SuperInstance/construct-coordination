/// Core types for Construct coordination layer.

/// A coordination node in the Construct network.
#[derive(Debug, Clone)]
pub struct CoordNode {
    pub id: String,
    pub layer: u8,
    pub peers: Vec<String>,
}

impl CoordNode {
    pub fn new(id: impl Into<String>, layer: u8) -> Self {
        Self {
            id: id.into(),
            layer,
            peers: Vec::new(),
        }
    }
}

/// Coordination message between nodes.
#[derive(Debug, Clone)]
pub struct CoordMessage {
    pub from: String,
    pub to: String,
    pub payload: Vec<u8>,
    pub seq: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_node() {
        let node = CoordNode::new("node-1", 0);
        assert_eq!(node.layer, 0);
        assert!(node.peers.is_empty());
    }
}
