use frame_system::Pallet as System;
use frame_support::pallet_prelude::{Parameter, Member};
use codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};

/// Context of a transaction that executed in
pub trait TransactionCtxT: Sized + Clone {
    type BlockNumber: Parameter + Member;
    type ExtrinsicId: Parameter + Member;
    
    fn current() -> Self;
    
    fn block_number(&self) -> Self::BlockNumber;
    fn extrinsic_id(&self) -> Self::ExtrinsicId;
    fn id(&self) -> TransactionCtxId<Self>;
}

/// Id of a context that transaction executed in
#[derive(Debug, Clone, Copy, Eq, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct TransactionCtxId<Ctx: TransactionCtxT + ?Sized> {
    pub block_number: Ctx::BlockNumber,
    pub extrinsic_id: Ctx::ExtrinsicId
}
impl<Ctx: TransactionCtxT> Default for TransactionCtxId<Ctx> {
    fn default() -> Self {
        Ctx::current().id()
    }
}

#[derive(Clone, Default, Eq, PartialEq)]
pub struct TransactionCtx<T: frame_system::Config>(sp_std::marker::PhantomData<T>);
impl<T: frame_system::Config> TransactionCtxT
    for TransactionCtx<T>
{
    type BlockNumber = T::BlockNumber;
    type ExtrinsicId = u32;

    fn current() -> Self { Self(Default::default()) }
    
    fn block_number(&self) -> Self::BlockNumber {
        System::<T>::block_number()
    }

    fn extrinsic_id(&self) -> Self::ExtrinsicId {
        System::<T>::extrinsic_index().unwrap_or_default()
    }

    fn id(&self) -> TransactionCtxId<Self> {
        TransactionCtxId {
            block_number: self.block_number(),
            extrinsic_id: self.extrinsic_id()
        }
    }
}

#[macro_export]
macro_rules! ctx_t {
    ($name:tt) => {
#[derive(Clone, Default, Eq, PartialEq)]
pub struct $name<T: TransactionCtxT>(T);

impl<T> TransactionCtxT for $name<T>
    where T: TransactionCtxT
{
    type BlockNumber = T::BlockNumber;
    type ExtrinsicId = T::ExtrinsicId;

    fn current() -> Self {
        Self(T::current())
    }

    fn block_number(&self) -> Self::BlockNumber {
        self.0.block_number()
    }

    fn extrinsic_id(&self) -> Self::ExtrinsicId {
        self.0.extrinsic_id()
    }

    fn id(&self) -> TransactionCtxId<Self> {
        let TransactionCtxId {
            block_number, extrinsic_id
        } = self.0.id();
        TransactionCtxId { block_number, extrinsic_id }
    }
}
    };
}
