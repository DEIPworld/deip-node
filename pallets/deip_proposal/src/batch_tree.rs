/// Module contains operations on nested proposals

use sp_std::collections::vec_deque::VecDeque;
use sp_std::prelude::*;
use sp_std::iter::{Peekable, Iterator};

use crate::proposal::ProposalBatchX;
use crate::batch_item_kind::{BatchItemKind, BatchItemKindT};

use super::Config;


/// Visited tree node abstraction
pub struct BatchTreeNode<Data> {
    /// Nested level
    pub depth: usize,
    pub data: Data,
}

/// Marker-type used for traverse operation flow control 
pub struct StopTraverse;

/// Batch tree traverse operation.
/// Invokes `visit_node` callback on each node and accepts flow-control commands from it
pub fn traverse_batch_tree<'a, T: Config, V, Data: BatchItemKindT<T>>(
    root: &'a ProposalBatchX<Data>,
    mut visit_node: V
)
    where V: FnMut(BatchTreeNode<&'a Data>) -> Option<StopTraverse>,
{
    let mut stack = VecDeque::<Peekable<Box<dyn Iterator<Item=&Data>>>>::new();
    let boxed: Box<dyn Iterator<Item=&Data>> = Box::new(root.iter());
    stack.push_front(boxed.peekable());
    while !stack.is_empty() {
        let depth = stack.len();
        while let Some(data) = stack.front_mut().unwrap().next() {
            if visit_node(BatchTreeNode { depth, data }).is_some() {
                return
            }
            match BatchItemKindT::<T>::kind(data) {
                BatchItemKind::Propose(batch) => {
                    let boxed: Box<dyn Iterator<Item=&Data>> = Box::new(batch.iter());
                    stack.push_front(boxed.peekable());
                    break
                },
                _ => ()
            }
        }
        if stack.front_mut().unwrap().peek().is_none() {
            stack.pop_front();
        }
    }
}
