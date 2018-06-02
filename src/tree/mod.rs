pub mod core;
pub mod error;

use self::core::CoreTree;
use self::core::NodeId;
use self::error::NodeIdError;
use node::node_mut::NodeMut;
use node::node_ref::NodeRef;
use node::Node;
use node::Relatives;

///
/// A `Tree` "builder" which provides more control over how `Tree`s are created.
///
pub struct TreeBuilder<T> {
    pub(crate) root: Option<T>,
    pub(crate) capacity: Option<usize>,
}

impl<T> Default for TreeBuilder<T> {
    fn default() -> TreeBuilder<T> {
        TreeBuilder::new()
    }
}

impl<T> TreeBuilder<T> {
    ///
    /// Returns a new `TreeBuilder` with default settings.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let tree_builder: TreeBuilder<i32> = TreeBuilder::new();
    /// ```
    ///
    pub fn new() -> TreeBuilder<T> {
        TreeBuilder {
            root: None,
            capacity: None,
        }
    }

    ///
    /// Consumes the `TreeBuilder` and returns a new one with the given root value.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let tree_builder = TreeBuilder::new().with_root(5);
    /// ```
    ///
    pub fn with_root(self, root: T) -> TreeBuilder<T> {
        TreeBuilder {
            root: Some(root),
            capacity: self.capacity,
        }
    }

    ///
    /// Consumes the `TreeBuilder` and returns a new one with the given capacity.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let tree_builder: TreeBuilder<i32> = TreeBuilder::new().with_capacity(20);
    /// ```
    ///
    pub fn with_capacity(self, capacity: usize) -> TreeBuilder<T> {
        TreeBuilder {
            root: self.root,
            capacity: Some(capacity),
        }
    }

    ///
    /// Consumes the `TreeBuilder` and builds a `Tree` with the settings the `TreeBuilder` was
    /// given.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let tree = TreeBuilder::new()
    ///     .with_root(5)
    ///     .with_capacity(20)
    ///     .build();
    ///
    /// # assert_eq!(tree.capacity(), 20);
    /// # assert_eq!(tree.root().unwrap().data(), &5);
    /// ```
    ///
    pub fn build(self) -> Tree<T> {
        let mut core_tree = CoreTree::new(self.capacity.unwrap_or(0));
        let root_id = self.root.map(|data| core_tree.insert(data));

        Tree { root_id, core_tree }
    }
}

///
/// A tree structure containing `Node`s.
///
pub struct Tree<T> {
    pub(crate) root_id: Option<NodeId>,
    pub(crate) core_tree: CoreTree<T>,
}

impl<T> Default for Tree<T> {
    fn default() -> Tree<T> {
        Tree::new()
    }
}

impl<T> Tree<T> {
    ///
    /// Creates a new `Tree` with default settings (no root and no capacity).
    ///
    /// If you would like to specify a root and/or capacity upon creation, consider using
    /// `TreeBuilder` instead of `Tree::new()`.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    ///
    /// let tree: Tree<i32> = Tree::new();
    ///
    /// # assert!(tree.root().is_none());
    /// # assert_eq!(tree.capacity(), 0);
    /// ```
    ///
    pub fn new() -> Tree<T> {
        Tree {
            root_id: None,
            core_tree: CoreTree::new(0),
        }
    }

    ///
    /// Returns the `Tree`'s current capacity.  Capacity is defined as the number of times new
    /// `Node`s can be added to the `Tree` before it must allocate more memory.
    ///
    /// ```
    /// use slab_tree::tree::Tree;
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let tree: Tree<i32> = TreeBuilder::new().with_capacity(5).build();
    ///
    /// assert_eq!(tree.capacity(), 5);
    /// # assert!(tree.root().is_none());
    /// ```
    ///
    pub fn capacity(&self) -> usize {
        self.core_tree.capacity()
    }

    ///
    /// Returns a reference to the `NodeId` of the root node of the `Tree`.  If there is a root
    /// `Node` in the tree a `Some`-value is returned, otherwise a `None` is returned.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let tree = TreeBuilder::new().with_root(1).build();
    ///
    /// assert!(tree.root_id().is_some());
    /// ```
    ///
    pub fn root_id(&self) -> Option<&NodeId> {
        self.root_id.as_ref()
    }

    ///
    /// Returns a `NodeRef` pointing to the root `Node` of the `Tree`.  If there is a root
    /// `Node` in the tree a `Some`-value is returned, otherwise a `None` is returned.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let tree = TreeBuilder::new().with_root(1).build();
    ///
    /// assert!(tree.root().is_some());
    /// assert_eq!(tree.root().unwrap().data(), &1);
    /// ```
    ///
    pub fn root(&self) -> Option<NodeRef<T>> {
        self.root_id.clone().map(|id| self.new_node_ref(id))
    }

    ///
    /// Returns a `NodeMut` pointing to the root `Node` of the `Tree`.  If there is a root
    /// `Node` in the tree a `Some`-value is returned, otherwise a `None` is returned.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let root = tree.root_mut();
    ///
    /// assert!(root.is_some());
    /// let mut root = root.unwrap();
    ///
    /// *root.data() = 2;
    /// assert_eq!(root.data(), &mut 2);
    /// ```
    ///
    pub fn root_mut(&mut self) -> Option<NodeMut<T>> {
        self.root_id.clone().map(move |id| self.new_node_mut(id))
    }

    ///
    /// Returns the `NodeRef` pointing to the `Node` that the given `NodeId` identifies.  If the
    /// `NodeId` in question points to nothing (or belongs to a different `Tree`) an `Err`-value
    /// will be returned; otherwise, an `Ok`-value will be returned.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let tree = TreeBuilder::new().with_root(1).build();
    /// let root_id = tree.root_id().unwrap();
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
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    ///
    /// // not normally necessary, but for this example to be short, we need this clone
    /// let root_id = tree.root_id().unwrap().clone();
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
        node.parent = parent_id;
    }

    pub(crate) fn set_prev_sibling(&mut self, node_id: &NodeId, prev_sibling: Option<NodeId>) {
        let node = unsafe { self.get_node_unchecked_mut(node_id) };
        node.prev_sibling = prev_sibling;
    }

    pub(crate) fn set_next_sibling(&mut self, node_id: &NodeId, next_sibling: Option<NodeId>) {
        let node = unsafe { self.get_node_unchecked_mut(node_id) };
        node.next_sibling = next_sibling;
    }

    pub(crate) fn set_first_child(&mut self, node_id: &NodeId, first_child: Option<NodeId>) {
        let node = unsafe { self.get_node_unchecked_mut(node_id) };
        node.first_child = first_child;
    }

    pub(crate) fn set_last_child(&mut self, node_id: &NodeId, last_child: Option<NodeId>) {
        let node = unsafe { self.get_node_unchecked_mut(node_id) };
        node.last_child = last_child;
    }

    pub(crate) fn get_node_prev_sibling_id(&mut self, node_id: &NodeId) -> Option<NodeId> {
        let node = unsafe { self.get_node_unchecked_mut(node_id) };
        node.prev_sibling.clone()
    }
    pub(crate) fn get_node_next_sibling_id(&mut self, node_id: &NodeId) -> Option<NodeId> {
        let node = unsafe { self.get_node_unchecked_mut(node_id) };
        node.next_sibling.clone()
    }

    pub(crate) fn get_node_relatives(&self, node_id: &NodeId) -> Relatives {
        let node = unsafe { self.get_node_unchecked(node_id) };
        Relatives {
            parent: node.parent.clone(),
            prev_sibling: node.prev_sibling.clone(),
            next_sibling: node.next_sibling.clone(),
            first_child: node.first_child.clone(),
            last_child: node.last_child.clone(),
        }
    }
}

#[cfg(test)]
mod tree_builder_tests {
    use super::*;

    #[test]
    fn with_root_and_capacity() {
        let tb = TreeBuilder::new().with_root(1).with_capacity(2);
        assert!(tb.root.is_some());
        assert_eq!(tb.root.unwrap(), 1);
        assert_eq!(tb.capacity.unwrap(), 2);
    }

    #[test]
    fn build() {
        let tree = TreeBuilder::new().with_root(1).with_capacity(2).build();
        assert!(tree.root_id.is_some());
        assert_eq!(tree.core_tree.capacity(), 2);
    }
}

#[cfg(test)]
mod tree_tests {
    use super::*;

    #[test]
    fn capacity() {
        let tree: Tree<i32> = Tree::new();
        assert_eq!(tree.capacity(), 0);

        let tree: Tree<i32> = TreeBuilder::new().with_capacity(5).build();
        assert_eq!(tree.capacity(), 5);
    }

    #[test]
    fn root_id() {
        let tree: Tree<i32> = Tree::new();
        assert!(tree.root_id().is_none());

        let tree = TreeBuilder::new().with_root(1).build();
        assert!(tree.root_id().is_some());
    }

    #[test]
    fn root() {
        let tree: Tree<i32> = Tree::new();
        assert!(tree.root().is_none());

        let tree = TreeBuilder::new().with_root(1).build();
        assert!(tree.root().is_some());
        assert_eq!(tree.root().unwrap().data(), &1);
    }

    #[test]
    fn root_mut() {
        let mut tree: Tree<i32> = Tree::new();
        assert!(tree.root_mut().is_none());

        let mut tree = TreeBuilder::new().with_root(1).build();
        assert!(tree.root().is_some());
        assert_eq!(tree.root_mut().unwrap().data(), &mut 1);

        *tree.root_mut().unwrap().data() = 2;
        assert_eq!(tree.root_mut().unwrap().data(), &mut 2);
    }

    #[test]
    fn get() {
        let tree = TreeBuilder::new().with_root(1).build();

        let root_id = tree.root_id();
        assert!(root_id.is_some());

        let root = tree.get(root_id.unwrap());
        assert!(root.is_ok());

        let root = root.ok().unwrap();
        assert_eq!(root.data(), &1);
    }

    #[test]
    fn get_mut() {
        let mut tree = TreeBuilder::new().with_root(1).build();

        let root_id = tree.root_id().cloned();
        assert!(root_id.is_some());

        let root = tree.get_mut(&root_id.unwrap());
        assert!(root.is_ok());

        let mut root = root.ok().unwrap();
        assert_eq!(root.data(), &mut 1);

        *root.data() = 2;
        assert_eq!(root.data(), &mut 2);
    }

    #[test]
    fn get_node_unchecked() {
        let tree = TreeBuilder::new().with_root(1).build();

        let root_id = tree.root_id();
        assert!(root_id.is_some());

        let root = unsafe { tree.get_node_unchecked(root_id.unwrap()) };

        assert_eq!(root.data, 1);
    }

    #[test]
    fn get_node_unchecked_mut() {
        let mut tree = TreeBuilder::new().with_root(1).build();

        let root_id = tree.root_id().cloned();
        assert!(root_id.is_some());

        let root = unsafe { tree.get_node_unchecked_mut(&root_id.unwrap()) };

        assert_eq!(root.data, 1);

        root.data = 2;
        assert_eq!(root.data, 2);
    }
}
