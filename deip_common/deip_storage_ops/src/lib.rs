//! Module contains abstractions over storage operations
//!
//! 

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::collections::vec_deque::VecDeque;

/// Storage operation
pub trait StorageOp {
    fn exec(self);
}

/// Fifo-queue for storage operations
pub struct StorageOpsQueue<T>(VecDeque<T>);
impl<T> StorageOpsQueue<T> {
    /// Add storage operation
    pub fn push_op(&mut self, op: T) -> &mut Self {
        self.0.push_back(op);
        self
    }
    fn pop_op(&mut self) -> Option<T> { self.0.pop_front() }
}

/// Multi-ops storage transaction 
pub struct StorageOpsTransaction<Op>(StorageOpsQueue<Op>);
impl<Op: StorageOp> StorageOpsTransaction<Op> {
    /// New storage transaction
    pub fn new() -> Self { Self(StorageOpsQueue(VecDeque::new())) }
    
    /// Execute callable then perform storage operations provided via ops-queue
    pub fn commit<R>(mut self, transactional: impl FnOnce(&mut StorageOpsQueue<Op>) -> R) -> R {
        let result = transactional(&mut self.0);
        while let Some(op) = self.0.pop_op() {
            op.exec();
        }
        result
    }
}
