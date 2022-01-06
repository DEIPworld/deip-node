/// Module contains some assertions on proposal's batch

use crate::proposal::{ProposalBatchX, ProposalId};
use crate::batch_item_kind::{BatchItemKindT, BatchItemKind};
use crate::batch_tree::{traverse_batch_tree, BatchTreeNode, StopTraverse};
use sp_std::collections::btree_map::BTreeMap;

use super::Config;

/// Proposal assertions enumeration
pub enum ProposalAssertions {
    /// Reached depth limit of nested proposals
    DepthLimit,
    /// Proposal has self-references
    SelfReference,
    /// Size limit violated
    SizeLimit
}

/// Perform some assertions on proposal object
pub fn assert_proposal<T: Config, BatchItem: BatchItemKindT<T>>(
    batch: &ProposalBatchX<BatchItem>,
    proposal_id: &ProposalId,
    depth_limit: usize,
    size_limit: usize
)
    -> Option<ProposalAssertions>
{
    let mut res = None;
    let mut size: BTreeMap<usize, usize> = BTreeMap::new();
    traverse_batch_tree::<T, _, _>(&batch, |node: BatchTreeNode<&BatchItem>| {
        *size.entry(node.depth).or_insert(0) += 1;
        if size[&node.depth] > size_limit {
            res = Some(ProposalAssertions::SizeLimit);
            return Some(StopTraverse)
        }
        if node.depth > depth_limit {
            res = Some(ProposalAssertions::DepthLimit);
            return Some(StopTraverse)
        }
        if let BatchItemKind::Decide(id) = BatchItemKindT::<T>::kind(node.data) {
            if id == proposal_id {
                res = Some(ProposalAssertions::SelfReference);
                return Some(StopTraverse)
            }
        }
        None
    });
    res
}
