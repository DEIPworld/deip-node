use codec::{Decode, Encode};
use frame_support::{
    pallet_prelude::{Member, Parameter, TypeInfo},
    Hashable,
};
use frame_system::Pallet as System;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_io::hashing::twox_128;
use sp_std::prelude::*;

/// Context of a transaction that executed in
pub trait TransactionCtxT: Sized + Clone {
    type BlockNumber: Parameter + Member;
    type ExtrinsicId: Parameter + Member;

    fn current() -> Self;

    fn block_number(&self) -> Self::BlockNumber;
    fn extrinsic_id(&self) -> Self::ExtrinsicId;
    fn id(&self) -> TransactionCtxId<Self>;
    fn extrinsic_data(&self) -> Vec<u8>;
}

/// Id of a context that transaction executed in
#[derive(Debug, Clone, Copy, Eq, PartialEq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct TransactionCtxId<Ctx: TransactionCtxT> {
    pub block_number: Ctx::BlockNumber,
    pub extrinsic_id: Ctx::ExtrinsicId,
}
impl<Ctx: TransactionCtxT> Default for TransactionCtxId<Ctx> {
    fn default() -> Self {
        Ctx::current().id()
    }
}

#[derive(Clone, Default, Eq, PartialEq, TypeInfo)]
pub struct TransactionCtx<T: frame_system::Config + TypeInfo>(sp_std::marker::PhantomData<T>);

impl<T: frame_system::Config + TypeInfo> TransactionCtxT for TransactionCtx<T> {
    type BlockNumber = T::BlockNumber;
    type ExtrinsicId = u32;

    fn current() -> Self {
        Self(Default::default())
    }

    fn block_number(&self) -> Self::BlockNumber {
        System::<T>::block_number()
    }

    fn extrinsic_id(&self) -> Self::ExtrinsicId {
        System::<T>::extrinsic_index().unwrap_or_default()
    }

    fn id(&self) -> TransactionCtxId<Self> {
        TransactionCtxId { block_number: self.block_number(), extrinsic_id: self.extrinsic_id() }
    }

    /// Data of the current extrinsic.
    fn extrinsic_data(&self) -> Vec<u8> {
        let mut key = Vec::with_capacity(40);
        key.extend(twox_128(b"System"));
        key.extend(twox_128(b"ExtrinsicData"));
        key.extend(self.extrinsic_id().twox_64_concat());
        let encoded = sp_io::storage::get(&key[..]).unwrap();
        Vec::<u8>::decode(&mut &encoded[..]).unwrap()
    }
}

#[macro_export]
macro_rules! ctx_t {
    ($name:tt) => {
        #[derive(Clone, Default, Eq, PartialEq, scale_info::TypeInfo)]
        pub struct $name<T: TransactionCtxT>(T);

        impl<T> TransactionCtxT for $name<T>
        where
            T: TransactionCtxT,
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
                let TransactionCtxId { block_number, extrinsic_id } = self.0.id();
                TransactionCtxId { block_number, extrinsic_id }
            }

            fn extrinsic_data(&self) -> Vec<u8> {
                self.0.extrinsic_data()
            }
        }
    };
}
