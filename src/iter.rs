use node::node_ref::NodeRef;
use tree::core::NodeId;
use tree::Tree;

pub struct Ancestors<'a, T: 'a> {
    node_id: Option<NodeId>,
    tree: &'a Tree<T>,
}

impl<'a, T> Ancestors<'a, T> {
    pub fn new(node_id: NodeId, tree: &'a Tree<T>) -> Ancestors<T> {
        Ancestors {
            node_id: Some(node_id),
            tree,
        }
    }
}

impl<'a, T> Iterator for Ancestors<'a, T> {
    type Item = NodeRef<'a, T>;

    fn next(&mut self) -> Option<NodeRef<'a, T>> {
        self.node_id
            .take()
            .and_then(|node_id| self.tree.get_node_relatives(&node_id).parent)
            .map(|parent_id| {
                self.node_id = Some(parent_id.clone());
                self.tree.new_node_ref(parent_id)
            })
    }
}
