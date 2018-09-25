use node::*;
use tree::Tree;
use NodeId;

pub struct Ancestors<'a, T: 'a> {
    node_id: Option<NodeId>,
    tree: &'a Tree<T>,
}

impl<'a, T> Ancestors<'a, T> {
    pub(crate) fn new(node_id: Option<NodeId>, tree: &'a Tree<T>) -> Ancestors<T> {
        Ancestors { node_id, tree }
    }
}

impl<'a, T> Iterator for Ancestors<'a, T> {
    type Item = NodeRef<'a, T>;

    fn next(&mut self) -> Option<NodeRef<'a, T>> {
        self.node_id
            .take()
            .and_then(|node_id| self.tree.get_node_relatives(node_id).parent)
            .map(|id| {
                self.node_id = Some(id);
                self.tree.new_node_ref(id)
            })
    }
}

pub struct NextSiblings<'a, T: 'a> {
    node_id: Option<NodeId>,
    tree: &'a Tree<T>,
}

impl<'a, T> NextSiblings<'a, T> {
    pub(crate) fn new(node_id: Option<NodeId>, tree: &'a Tree<T>) -> NextSiblings<T> {
        NextSiblings { node_id, tree }
    }
}

impl<'a, T> Iterator for NextSiblings<'a, T> {
    type Item = NodeRef<'a, T>;

    fn next(&mut self) -> Option<NodeRef<'a, T>> {
        self.node_id.take().map(|node_id| {
            self.node_id = self.tree.get_node_relatives(node_id).next_sibling;
            self.tree.new_node_ref(node_id)
        })
    }
}
