use crate::core_tree::CoreTree;
use crate::node::*;
use crate::NodeId;

///
/// A tree structure containing `Node`s.
///
#[derive(Debug, PartialEq)]
pub struct Tree<T> {
    pub(crate) root_id: NodeId,
    pub(crate) core_tree: CoreTree<T>,
}

impl<T> Tree<T> {
    ///
    /// Creates a new `Tree` with the given root value and a capacity of 1.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree = Tree::new(1);
    ///
    /// # assert_eq!(tree.root().data(), &1);
    /// # assert_eq!(tree.capacity(), 1);
    /// ```
    ///
    pub fn new(root: T) -> Tree<T> {
        Tree::new_with_capacity(root, 1)
    }

    ///
    /// Creates a new `Tree` with the given root value and capacity.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree = Tree::new_with_capacity(1, 2);
    ///
    /// # assert_eq!(tree.root().data(), &1);
    /// # assert_eq!(tree.capacity(), 2);
    /// ```
    ///
    pub fn new_with_capacity(root: T, capacity: usize) -> Tree<T> {
        let mut core_tree: CoreTree<T> = CoreTree::new(capacity);
        let root_id = core_tree.insert(root);

        Tree { root_id, core_tree }
    }

    ///
    /// Returns the `Tree`'s current capacity.  Capacity is defined as the number of times new
    /// `Node`s can be added to the `Tree` before it must allocate more memory.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree = Tree::new_with_capacity(1, 2);
    ///
    /// # assert_eq!(tree.root().data(), &1);
    /// assert_eq!(tree.capacity(), 2);
    /// ```
    ///
    pub fn capacity(&self) -> usize {
        self.core_tree.capacity()
    }

    ///
    /// Returns the `NodeId` of the root node of the `Tree`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree = Tree::new(1);
    ///
    /// let root_id = tree.root_id();
    ///
    /// assert_eq!(tree.get(root_id).unwrap().data(), &1);
    /// ```
    ///
    pub fn root_id(&self) -> NodeId {
        self.root_id
    }

    ///
    /// Returns a `NodeRef` pointing to the root `Node` of the `Tree`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree = Tree::new(1);
    ///
    /// let root = tree.root();
    ///
    /// assert_eq!(root.data(), &1);
    /// ```
    ///
    pub fn root(&self) -> NodeRef<T> {
        self.new_node_ref(self.root_id)
    }

    ///
    /// Returns a `NodeMut` pointing to the root `Node` of the `Tree`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let mut tree = Tree::new(1);
    ///
    /// let mut root = tree.root_mut();
    /// assert_eq!(root.data(), &mut 1);
    ///
    /// *root.data() = 2;
    /// assert_eq!(root.data(), &mut 2);
    /// ```
    ///
    pub fn root_mut(&mut self) -> NodeMut<T> {
        self.new_node_mut(self.root_id)
    }

    ///
    /// Returns the `NodeRef` pointing to the `Node` that the given `NodeId` identifies.  If the
    /// `NodeId` in question points to nothing (or belongs to a different `Tree`) a `None`-value
    /// will be returned; otherwise, a `Some`-value will be returned.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree = Tree::new(1);
    /// let root_id = tree.root_id();
    ///
    /// let root = tree.get(root_id);
    /// assert!(root.is_some());
    ///
    /// let root = root.unwrap();
    /// assert_eq!(root.data(), &1);
    /// ```
    ///
    pub fn get(&self, node_id: NodeId) -> Option<NodeRef<T>> {
        let _ = self.core_tree.get(node_id)?;
        Some(self.new_node_ref(node_id))
    }

    ///
    /// Returns the `NodeMut` pointing to the `Node` that the given `NodeId` identifies.  If the
    /// `NodeId` in question points to nothing (or belongs to a different `Tree`) a `None`-value
    /// will be returned; otherwise, a `Some`-value will be returned.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let mut tree = Tree::new(1);
    /// let root_id = tree.root_id();
    ///
    /// let root = tree.get_mut(root_id);
    /// assert!(root.is_some());
    ///
    /// let mut root = root.unwrap();
    ///
    /// *root.data() = 2;
    /// assert_eq!(root.data(), &mut 2);
    /// ```
    ///
    pub fn get_mut(&mut self, node_id: NodeId) -> Option<NodeMut<T>> {
        let _ = self.core_tree.get_mut(node_id)?;
        Some(self.new_node_mut(node_id))
    }

    pub(crate) fn get_node(&self, node_id: NodeId) -> Option<&Node<T>> {
        self.core_tree.get(node_id)
    }

    pub(crate) fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut Node<T>> {
        self.core_tree.get_mut(node_id)
    }

    fn new_node_ref(&self, node_id: NodeId) -> NodeRef<T> {
        NodeRef::new(node_id, self)
    }

    fn new_node_mut(&mut self, node_id: NodeId) -> NodeMut<T> {
        NodeMut::new(node_id, self)
    }

    pub(crate) fn set_prev_siblings_next_sibling(
        &mut self,
        current_id: NodeId,
        next_sibling: Option<NodeId>,
    ) {
        if let Some(prev_sibling_id) = self.get_node_prev_sibling_id(current_id) {
            self.set_next_sibling(prev_sibling_id, next_sibling);
        }
    }

    pub(crate) fn set_next_siblings_prev_sibling(
        &mut self,
        current_id: NodeId,
        prev_sibling: Option<NodeId>,
    ) {
        if let Some(next_sibling_id) = self.get_node_next_sibling_id(current_id) {
            self.set_prev_sibling(next_sibling_id, prev_sibling);
        }
    }

    pub(crate) fn set_parent(&mut self, node_id: NodeId, parent_id: Option<NodeId>) {
        if let Some(node) = self.get_node_mut(node_id) {
            node.relatives.parent = parent_id;
        } else {
            unreachable!()
        }
    }

    pub(crate) fn set_prev_sibling(&mut self, node_id: NodeId, prev_sibling: Option<NodeId>) {
        if let Some(node) = self.get_node_mut(node_id) {
            node.relatives.prev_sibling = prev_sibling;
        } else {
            unreachable!()
        }
    }

    pub(crate) fn set_next_sibling(&mut self, node_id: NodeId, next_sibling: Option<NodeId>) {
        if let Some(node) = self.get_node_mut(node_id) {
            node.relatives.next_sibling = next_sibling;
        } else {
            unreachable!()
        }
    }

    pub(crate) fn set_first_child(&mut self, node_id: NodeId, first_child: Option<NodeId>) {
        if let Some(node) = self.get_node_mut(node_id) {
            node.relatives.first_child = first_child;
        } else {
            unreachable!()
        }
    }

    pub(crate) fn set_last_child(&mut self, node_id: NodeId, last_child: Option<NodeId>) {
        if let Some(node) = self.get_node_mut(node_id) {
            node.relatives.last_child = last_child;
        } else {
            unreachable!()
        }
    }

    pub(crate) fn get_node_prev_sibling_id(&self, node_id: NodeId) -> Option<NodeId> {
        if let Some(node) = self.get_node(node_id) {
            node.relatives.prev_sibling
        } else {
            unreachable!()
        }
    }

    pub(crate) fn get_node_next_sibling_id(&self, node_id: NodeId) -> Option<NodeId> {
        if let Some(node) = self.get_node(node_id) {
            node.relatives.next_sibling
        } else {
            unreachable!()
        }
    }

    pub(crate) fn get_node_relatives(&self, node_id: NodeId) -> Relatives {
        if let Some(node) = self.get_node(node_id) {
            node.relatives
        } else {
            unreachable!()
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tree_tests {
    use super::*;

    #[test]
    fn capacity() {
        let tree = Tree::new(1);
        assert_eq!(tree.capacity(), 1);

        let tree = Tree::new_with_capacity(1, 5);
        assert_eq!(tree.capacity(), 5);
    }

    #[test]
    fn root_id() {
        let tree = Tree::new(1);
        let root_id = tree.root_id();
        let root = tree.get(root_id).unwrap();
        assert_eq!(root.data(), &1);
    }

    #[test]
    fn root() {
        let tree = Tree::new(1);
        let root = tree.root();
        assert_eq!(root.data(), &1);
    }

    #[test]
    fn root_mut() {
        let mut tree = Tree::new(1);
        assert_eq!(tree.root_mut().data(), &mut 1);

        *tree.root_mut().data() = 2;
        assert_eq!(tree.root_mut().data(), &mut 2);
    }

    #[test]
    fn get() {
        let tree = Tree::new(1);

        let root_id = tree.root_id();
        let root = tree.get(root_id);
        assert!(root.is_some());

        let root = root.unwrap();
        assert_eq!(root.data(), &1);
    }

    #[test]
    fn get_mut() {
        let mut tree = Tree::new(1);

        let root_id = tree.root_id();
        let root = tree.get_mut(root_id);
        assert!(root.is_some());

        let mut root = root.unwrap();
        assert_eq!(root.data(), &mut 1);

        *root.data() = 2;
        assert_eq!(root.data(), &mut 2);
    }

    #[test]
    fn get_node() {
        let tree = Tree::new(1);

        let root_id = tree.root_id();
        let root = tree.get_node(root_id);
        assert!(root.is_some());

        let root = root.unwrap();
        assert_eq!(root.data, 1);
    }

    #[test]
    fn get_node_mut() {
        let mut tree = Tree::new(1);

        let root_id = tree.root_id();
        let root = tree.get_node_mut(root_id);
        assert!(root.is_some());

        let root = root.unwrap();
        assert_eq!(root.data, 1);

        root.data = 2;
        assert_eq!(root.data, 2);
    }
}
