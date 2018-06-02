pub mod core;
pub mod error;

use self::core::CoreTree;
use self::core::NodeId;
use self::error::NodeIdError;
use node::*;

///
/// A tree structure containing `Node`s.
///
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

        Tree {
            root_id,
            core_tree,
        }
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
    /// Returns a reference to the `NodeId` of the root node of the `Tree`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree = Tree::new(1);
    ///
    /// let root_id = tree.root_id();
    ///
    /// assert_eq!(tree.get(root_id).ok().unwrap().data(), &1);
    /// ```
    ///
    pub fn root_id(&self) -> &NodeId {
        &self.root_id
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
        self.new_node_ref(self.root_id.clone())
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
        let node_id = self.root_id.clone();
        self.new_node_mut(node_id)
    }

    ///
    /// Returns the `NodeRef` pointing to the `Node` that the given `NodeId` identifies.  If the
    /// `NodeId` in question points to nothing (or belongs to a different `Tree`) an `Err`-value
    /// will be returned; otherwise, an `Ok`-value will be returned.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree = Tree::new(1);
    /// let root_id = tree.root_id();
    ///
    /// let root = tree.get(root_id);
    /// assert!(root.is_ok());
    ///
    /// let root = root.ok().unwrap();
    /// assert_eq!(root.data(), &1);
    /// ```
    ///
    pub fn get(&self, node_id: &NodeId) -> Result<NodeRef<T>, NodeIdError> {
        let _ = self.core_tree.get(node_id)?;
        Ok(self.new_node_ref(node_id.clone()))
    }

    ///
    /// Returns the `NodeMut` pointing to the `Node` that the given `NodeId` identifies.  If the
    /// `NodeId` in question points to nothing (or belongs to a different `Tree`) an `Err`-value
    /// will be returned; otherwise, an `Ok`-value will be returned.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let mut tree = Tree::new(1);
    /// let root_id = tree.root_id().clone();
    ///
    /// let root = tree.get_mut(&root_id);
    /// assert!(root.is_ok());
    ///
    /// let mut root = root.ok().unwrap();
    ///
    /// *root.data() = 2;
    /// assert_eq!(root.data(), &mut 2);
    /// ```
    ///
    pub fn get_mut(&mut self, node_id: &NodeId) -> Result<NodeMut<T>, NodeIdError> {
        let _ = self.core_tree.get_mut(node_id)?;
        Ok(self.new_node_mut(node_id.clone()))
    }

    pub(crate) unsafe fn get_node_unchecked(&self, node_id: &NodeId) -> &Node<T> {
        self.core_tree.get_unchecked(node_id)
    }

    pub(crate) unsafe fn get_node_unchecked_mut(&mut self, node_id: &NodeId) -> &mut Node<T> {
        self.core_tree.get_unchecked_mut(node_id)
    }

    pub(crate) fn new_node_ref(&self, node_id: NodeId) -> NodeRef<T> {
        NodeRef {
            node_id,
            tree: self,
        }
    }

    pub(crate) fn new_node_mut(&mut self, node_id: NodeId) -> NodeMut<T> {
        NodeMut {
            node_id,
            tree: self,
        }
    }

    pub(crate) fn set_prev_siblings_next_sibling(
        &mut self,
        current_id: &NodeId,
        next_sibling: Option<NodeId>,
    ) {
        if let Some(prev_sibling_id) = self.get_node_prev_sibling_id(current_id) {
            self.set_next_sibling(&prev_sibling_id, next_sibling);
        }
    }

    pub(crate) fn set_next_siblings_prev_sibling(
        &mut self,
        current_id: &NodeId,
        prev_sibling: Option<NodeId>,
    ) {
        if let Some(next_sibling_id) = self.get_node_next_sibling_id(current_id) {
            self.set_prev_sibling(&next_sibling_id, prev_sibling);
        }
    }

    pub(crate) fn set_parent(&mut self, node_id: &NodeId, parent_id: Option<NodeId>) {
        let node = unsafe { self.get_node_unchecked_mut(node_id) };
        node.relatives.parent = parent_id;
    }

    pub(crate) fn set_prev_sibling(&mut self, node_id: &NodeId, prev_sibling: Option<NodeId>) {
        let node = unsafe { self.get_node_unchecked_mut(node_id) };
        node.relatives.prev_sibling = prev_sibling;
    }

    pub(crate) fn set_next_sibling(&mut self, node_id: &NodeId, next_sibling: Option<NodeId>) {
        let node = unsafe { self.get_node_unchecked_mut(node_id) };
        node.relatives.next_sibling = next_sibling;
    }

    pub(crate) fn set_first_child(&mut self, node_id: &NodeId, first_child: Option<NodeId>) {
        let node = unsafe { self.get_node_unchecked_mut(node_id) };
        node.relatives.first_child = first_child;
    }

    pub(crate) fn set_last_child(&mut self, node_id: &NodeId, last_child: Option<NodeId>) {
        let node = unsafe { self.get_node_unchecked_mut(node_id) };
        node.relatives.last_child = last_child;
    }

    pub(crate) fn get_node_prev_sibling_id(&mut self, node_id: &NodeId) -> Option<NodeId> {
        let node = unsafe { self.get_node_unchecked_mut(node_id) };
        node.relatives.prev_sibling.clone()
    }
    pub(crate) fn get_node_next_sibling_id(&mut self, node_id: &NodeId) -> Option<NodeId> {
        let node = unsafe { self.get_node_unchecked_mut(node_id) };
        node.relatives.next_sibling.clone()
    }

    pub(crate) fn get_node_relatives(&self, node_id: &NodeId) -> Relatives {
        let node = unsafe { self.get_node_unchecked(node_id) };
        node.relatives.clone()
    }
}

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
        let root = tree.get(root_id).ok().unwrap();
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
        assert!(root.is_ok());

        let root = root.ok().unwrap();
        assert_eq!(root.data(), &1);
    }

    #[test]
    fn get_mut() {
        let mut tree = Tree::new(1);

        let root_id = tree.root_id().clone();
        let root = tree.get_mut(&root_id);
        assert!(root.is_ok());

        let mut root = root.ok().unwrap();
        assert_eq!(root.data(), &mut 1);

        *root.data() = 2;
        assert_eq!(root.data(), &mut 2);
    }

    #[test]
    fn get_node_unchecked() {
        let tree = Tree::new(1);

        let root_id = tree.root_id();
        let root = unsafe { tree.get_node_unchecked(root_id) };

        assert_eq!(root.data, 1);
    }

    #[test]
    fn get_node_unchecked_mut() {
        let mut tree = Tree::new(1);

        let root_id = tree.root_id().clone();
        let root = unsafe { tree.get_node_unchecked_mut(&root_id) };

        assert_eq!(root.data, 1);

        root.data = 2;
        assert_eq!(root.data, 2);
    }
}
