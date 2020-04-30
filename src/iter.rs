use crate::node::*;
use crate::tree::Tree;
use crate::NodeId;

// todo: document this

pub struct Ancestors<'a, T> {
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
                NodeRef::new(id, self.tree)
            })
    }
}

// possibly re-name this, not sure how I feel about it
pub struct NextSiblings<'a, T> {
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
            NodeRef::new(node_id, self.tree)
        })
    }
}

/// Depth-first pre-order iterator
pub struct PreOrder<'a, T> {
    start: Option<NodeRef<'a, T>>,
    children: Vec<NextSiblings<'a, T>>,
    tree: &'a Tree<T>,
}

impl<'a, T> PreOrder<'a, T> {
    pub(crate) fn new(node: &NodeRef<'a, T>, tree: &'a Tree<T>) -> PreOrder<'a, T> {
        let children = vec![];
        let start = tree.get(node.node_id());
        PreOrder {
            start,
            children,
            tree,
        }
    }
}

impl<'a, T> Iterator for PreOrder<'a, T> {
    type Item = NodeRef<'a, T>;

    fn next(&mut self) -> Option<NodeRef<'a, T>> {
        if let Some(node) = self.start.take() {
            let first_child_id = node.first_child().map(|child_ref| child_ref.node_id());
            self.children
                .push(NextSiblings::new(first_child_id, self.tree));
            Some(node)
        } else {
            while !self.children.is_empty() {
                if let Some(node_ref) = self.children.last_mut().and_then(Iterator::next) {
                    if let Some(first_child) = node_ref.first_child() {
                        self.children
                            .push(NextSiblings::new(Some(first_child.node_id()), self.tree));
                    }
                    return Some(node_ref);
                }
                self.children.pop();
            }
            None
        }
    }
}

/// Depth-first post-order iterator
pub struct PostOrder<'a, T> {
    nodes: Vec<(NodeRef<'a, T>, NextSiblings<'a, T>)>,
    tree: &'a Tree<T>,
}

impl<'a, T> PostOrder<'a, T> {
    pub(crate) fn new(node: &NodeRef<'a, T>, tree: &'a Tree<T>) -> PostOrder<'a, T> {
        let node = tree
            .get(node.node_id())
            .expect("getting node of node ref id");
        let first_child_id = node.first_child().map(|first_child| first_child.node_id());
        let nodes = vec![(node, NextSiblings::new(first_child_id, tree))];
        PostOrder { nodes, tree }
    }
}

impl<'a, T> Iterator for PostOrder<'a, T> {
    type Item = NodeRef<'a, T>;

    fn next(&mut self) -> Option<NodeRef<'a, T>> {
        if let Some((node, mut children)) = self.nodes.pop() {
            if let Some(next) = children.next() {
                self.nodes.push((node, children));
                let mut node_id = next.node_id();
                loop {
                    let node = self.tree.get(node_id).expect("getting node of node ref id");
                    if let Some(first_child) = node.first_child() {
                        node_id = first_child.node_id();
                        let mut children = NextSiblings::new(Some(node_id), self.tree);
                        assert!(children.next().is_some(), "skipping first child");
                        self.nodes.push((node, children));
                    } else {
                        break Some(node);
                    }
                }
            } else {
                Some(node)
            }
        } else {
            None
        }
    }
}

/// Depth-first level-order iterator
pub struct LevelOrder<'a, T> {
    start: NodeRef<'a, T>,
    levels: Vec<(NodeId, NextSiblings<'a, T>)>,
    tree: &'a Tree<T>,
}

impl<'a, T> LevelOrder<'a, T> {
    pub(crate) fn new(node: &NodeRef<'a, T>, tree: &'a Tree<T>) -> LevelOrder<'a, T> {
        let start = tree
            .get(node.node_id())
            .expect("getting node of node ref id");
        let levels = Vec::new();
        LevelOrder {
            start,
            levels,
            tree,
        }
    }
}

impl<'a, T> Iterator for LevelOrder<'a, T> {
    type Item = NodeRef<'a, T>;

    fn next(&mut self) -> Option<NodeRef<'a, T>> {
        if self.levels.is_empty() {
            let first_child_id = self.start.first_child().map(|child| child.node_id());
            self.levels.push((
                self.start.node_id(),
                NextSiblings::new(first_child_id, self.tree),
            ));
            let node = self
                .tree
                .get(self.start.node_id())
                .expect("getting node of existing node ref id");
            Some(node)
        } else {
            let mut on_level = self.levels.len();
            let next_level = on_level + 1;
            let mut level = on_level;
            while level > 0 {
                if let Some(node) = self.levels.last_mut().expect("non-empty levels").1.next() {
                    if level >= on_level {
                        return Some(node);
                    } else {
                        let first_child_id = node.first_child().map(|child| child.node_id());
                        self.levels
                            .push((node.node_id(), NextSiblings::new(first_child_id, self.tree)));
                        level += 1;
                    }
                } else {
                    let (node_id, _) = self.levels.pop().expect("on level > 0");
                    if let Some(next) = self.levels.last_mut().and_then(|level| level.1.next()) {
                        let first_child_id = next.first_child().map(|child| child.node_id());
                        self.levels
                            .push((next.node_id(), NextSiblings::new(first_child_id, self.tree)));
                    } else if level == 1 {
                        if on_level < next_level {
                            on_level += 1;
                            let node = self
                                .tree
                                .get(node_id)
                                .expect("getting node of existing node ref id");
                            let first_child_id =
                                node.first_child().map(|child| child.node_id());
                            self.levels.push((
                                node.node_id(),
                                NextSiblings::new(first_child_id, self.tree),
                            ));
                        } else {
                            break;
                        }
                    } else {
                        level -= 1;
                    }
                }
            }
            None
        }
    }
}
