use crate::iter::Ancestors;
use crate::iter::NextSiblings;
use crate::node::Node;
use crate::tree::Tree;
use crate::NodeId;

///
/// An immutable reference to a given `Node`'s data and its relatives.
///
pub struct NodeRef<'a, T: 'a> {
    pub(crate) node_id: NodeId,
    pub(crate) tree: &'a Tree<T>,
}

impl<'a, T> NodeRef<'a, T> {
    ///
    /// Returns a reference to the data contained by the given `Node`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree = Tree::new(1);
    /// let root = tree.root();
    ///
    /// assert_eq!(root.data(), &1);
    /// ```
    ///
    pub fn data(&self) -> &T {
        if let Ok(node) = self.tree.get_node(self.node_id) {
            &node.data
        } else {
            unreachable!()
        }
    }

    ///
    /// Returns a `NodeRef` pointing to this `Node`'s parent.  Returns a `Some`-value containing
    /// the `NodeRef` if this `Node` has a parent; otherwise returns a `None`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree = Tree::new(1);
    /// let root = tree.root();
    ///
    /// assert!(root.parent().is_none());
    /// ```
    ///
    pub fn parent(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .relatives
            .parent
            .map(|id| self.tree.new_node_ref(id))
    }

    ///
    /// Returns a `NodeRef` pointing to this `Node`'s previous sibling.  Returns a `Some`-value
    /// containing the `NodeRef` if this `Node` has a previous sibling; otherwise returns a `None`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree = Tree::new(1);
    /// let root = tree.root();
    ///
    /// assert!(root.prev_sibling().is_none());
    /// ```
    ///
    pub fn prev_sibling(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .relatives
            .prev_sibling
            .map(|id| self.tree.new_node_ref(id))
    }

    ///
    /// Returns a `NodeRef` pointing to this `Node`'s next sibling.  Returns a `Some`-value
    /// containing the `NodeRef` if this `Node` has a next sibling; otherwise returns a `None`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree = Tree::new(1);
    /// let root = tree.root();
    ///
    /// assert!(root.next_sibling().is_none());
    /// ```
    ///
    pub fn next_sibling(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .relatives
            .next_sibling
            .map(|id| self.tree.new_node_ref(id))
    }

    ///
    /// Returns a `NodeRef` pointing to this `Node`'s first child.  Returns a `Some`-value
    /// containing the `NodeRef` if this `Node` has a first child; otherwise returns a `None`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree = Tree::new(1);
    /// let root = tree.root();
    ///
    /// assert!(root.first_child().is_none());
    /// ```
    ///
    pub fn first_child(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .relatives
            .first_child
            .map(|id| self.tree.new_node_ref(id))
    }

    ///
    /// Returns a `NodeRef` pointing to this `Node`'s last child.  Returns a `Some`-value
    /// containing the `NodeRef` if this `Node` has a last child; otherwise returns a `None`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree = Tree::new(1);
    /// let root = tree.root();
    ///
    /// assert!(root.last_child().is_none());
    /// ```
    ///
    pub fn last_child(&self) -> Option<NodeRef<T>> {
        self.get_self_as_node()
            .relatives
            .last_child
            .map(|id| self.tree.new_node_ref(id))
    }

    ///
    /// Returns a `Iterator` over the given `Node`'s ancestors.  Each call to `Iterator::next()`
    /// returns a `NodeRef` pointing to the current `Node`'s parent.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let mut tree = Tree::new(1);
    ///
    /// let leaf_id = tree.root_mut()
    ///     .append(2)
    ///     .append(3)
    ///     .append(4)
    ///     .node_id();
    ///
    /// let leaf = tree.get(leaf_id).ok().unwrap();
    ///
    /// let values = [3, 2, 1];
    /// for (i, ancestor) in leaf.ancestors().enumerate() {
    ///     assert_eq!(ancestor.data(), &values[i]);
    /// }
    /// ```
    ///
    pub fn ancestors(&self) -> impl Iterator<Item = NodeRef<T>> {
        Ancestors::new(Some(self.node_id), self.tree)
    }

    ///
    /// Returns a `Iterator` over the given `Node`'s children.  Each call to `Iterator::next()`
    /// returns a `NodeRef` pointing to the next child of the given `Node`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let mut tree = Tree::new(1);
    /// {
    ///     let mut root = tree.root_mut();
    ///     root.append(2);
    ///     root.append(3);
    ///     root.append(4);
    /// }
    /// let root = tree.root();
    ///
    /// let values = [2, 3, 4];
    /// for (i, child) in root.children().enumerate() {
    ///     assert_eq!(child.data(), &values[i]);
    /// }
    /// ```
    ///
    pub fn children(&self) -> impl Iterator<Item = NodeRef<T>> {
        let first_child_id = self.tree.get_node_relatives(self.node_id).first_child;
        NextSiblings::new(first_child_id, self.tree)
    }

    fn get_self_as_node(&self) -> &Node<T> {
        if let Ok(node) = self.tree.get_node(self.node_id) {
            &node
        } else {
            unreachable!()
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod node_ref_tests {
    use crate::tree::Tree;

    #[test]
    fn data() {
        let tree = Tree::new(1);
        let root_id = tree.root_id();
        let root_ref = tree.get(root_id).ok().unwrap();
        assert_eq!(root_ref.data(), &1);
    }

    #[test]
    fn parent() {
        let tree = Tree::new(1);
        let root_id = tree.root_id();
        let root_ref = tree.get(root_id).ok().unwrap();
        assert!(root_ref.parent().is_none());
    }

    #[test]
    fn prev_sibling() {
        let tree = Tree::new(1);
        let root_id = tree.root_id();
        let root_ref = tree.get(root_id).ok().unwrap();
        assert!(root_ref.prev_sibling().is_none());
    }

    #[test]
    fn next_sibling() {
        let tree = Tree::new(1);
        let root_id = tree.root_id();
        let root_ref = tree.get(root_id).ok().unwrap();
        assert!(root_ref.next_sibling().is_none());
    }

    #[test]
    fn first_child() {
        let tree = Tree::new(1);
        let root_id = tree.root_id();
        let root_ref = tree.get(root_id).ok().unwrap();
        assert!(root_ref.first_child().is_none());
    }

    #[test]
    fn last_child() {
        let tree = Tree::new(1);
        let root_id = tree.root_id();
        let root_ref = tree.get(root_id).ok().unwrap();
        assert!(root_ref.last_child().is_none());
    }

    #[test]
    fn ancestors() {
        let mut tree = Tree::new(1);
        let node_id;
        {
            let mut root_mut = tree.root_mut();
            node_id = root_mut.append(2).append(3).append(4).append(5).node_id();
        }
        let tree = tree;

        let values = [4, 3, 2, 1];

        let bottom_node = tree.get(node_id).ok().unwrap();
        for (i, node_ref) in bottom_node.ancestors().enumerate() {
            assert_eq!(node_ref.data(), &values[i]);
        }
    }

    #[test]
    fn children() {
        let mut tree = Tree::new(1);
        {
            let mut root_mut = tree.root_mut();
            root_mut.append(2);
            root_mut.append(3);
            root_mut.append(4);
            root_mut.append(5);
        }
        let tree = tree;

        let values = [2, 3, 4, 5];
        let root = tree.root();

        for (i, node_ref) in root.children().enumerate() {
            assert_eq!(node_ref.data(), &values[i]);
        }
    }
}
