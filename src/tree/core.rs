use std::marker::PhantomData;
use snowflake::ProcessUniqueId;
use slab::Slab;

use node::Node;
use tree::error::*;

// todo: document this

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NodeId {
    tree_id: ProcessUniqueId,
    index: usize,
}

pub struct CoreTree<N, T>
where
    N: Node<T>,
{
    id: ProcessUniqueId,
    root: Option<NodeId>,
    slab: Slab<N>,
    phantom: PhantomData<T>,
}

impl <N, T> CoreTree<N, T>
where
    N: Node<T>,
{
    pub fn new(root: Option<N>, capacity: usize) -> CoreTree<N, T> {
        let mut tree = CoreTree {
            id: ProcessUniqueId::new(),
            root: None,
            slab: Slab::with_capacity(capacity),
            phantom: PhantomData,
        };

        if let Some(root_node) = root {
            let root_id = NodeId {
                tree_id: tree.id,
                index: tree.slab.insert(root_node),
            };
            tree.root = Some(root_id);
        }

        tree
    }

    pub fn capacity(&self) -> usize {
        self.slab.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.slab.reserve(additional);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.slab.reserve_exact(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.slab.shrink_to_fit();
    }

    pub fn clear(&mut self) {
        self.root = None;
        self.slab.clear();
    }

    pub fn len(&self) -> usize {
        self.slab.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slab.is_empty()
    }

    pub fn insert(&mut self, node: N) -> NodeId {
        let key = self.slab.insert(node);
        self.new_node_id(key)
    }

    pub fn remove(&mut self, node_id: NodeId) -> N {
        self.slab.remove(node_id.index)
    }

    pub fn set_root(&mut self, new_root: N) -> NodeId {
        let new_root_id = self.insert(new_root);
        self.root = Some(new_root_id.clone());
        new_root_id
    }

    pub fn root(&self) -> Option<&NodeId> {
        self.root.as_ref()
    }

    pub fn get(&self, node_id: &NodeId) -> Result<&N, NodeIdError> {
        self.validate_node_id(node_id)?;
        match self.slab.get(node_id.index) {
            Some(node) => Ok(node),
            None => Err(NodeIdError::BadNodeId),
        }
    }

    pub fn get_mut(&mut self, node_id: &NodeId) -> Result<&mut N, NodeIdError> {
        self.validate_node_id(node_id)?;
        match self.slab.get_mut(node_id.index) {
            Some(node) => Ok(node),
            None => Err(NodeIdError::BadNodeId),
        }
    }

    pub unsafe fn get_unchecked(&self, node_id: &NodeId) -> &N {
        self.slab.get_unchecked(node_id.index)
    }

    pub unsafe fn get_unchecked_mut(&mut self, node_id: &NodeId) -> &mut N {
        self.slab.get_unchecked_mut(node_id.index)
    }

    fn new_node_id(&self, index: usize) -> NodeId {
        NodeId {
            tree_id: self.id,
            index,
        }
    }

    fn validate_node_id(&self, node_id: &NodeId) -> Result<(), NodeIdError> {
        if node_id.tree_id != self.id {
            return Err(NodeIdError::WrongTree);
        }
        Ok(())
    }
}
