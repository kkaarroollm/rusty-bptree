pub type NodeId = usize;

#[derive(Debug)]
enum Node<K, V> {
    Internal { keys: Vec<K>, children: Vec<NodeId> },
    Leaf     { entries: Vec<(K, V)>, next: Option<NodeId> },
}

pub struct BPlusTree<K: Ord + Clone, V: Clone, const ORDER: usize = 4> {
    
    root: NodeId,
    nodes: Vec<Node<K, V>>,
}

impl<K: Ord + Clone, V: Clone, const ORDER: usize> BPlusTree<K, V, ORDER> {
    pub fn new() -> Self {
        assert!(ORDER >= 3, "useless setting less than 3 in bp-tree ");
        let mut nodes = Vec::new();
        nodes.push(Node::Leaf { entries: Vec::new(), next: None });
        Self { root: 0, nodes }
    }
    pub fn debug_dump(&self) -> String {
        format!("BPlusTree<ORDER={}> (nodes={})", ORDER, self.nodes.len())
    }
}
