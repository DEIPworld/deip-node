#![allow(unused_imports)]
use codec::{Encode, Decode, Codec};
use crate::Config;
use frame_system::{Pallet as System};
use frame_support::weights::DispatchInfo;
use sp_runtime::{
    DispatchResult,
    traits::{SignedExtension, DispatchInfoOf, Dispatchable, PostDispatchInfoOf},
    transaction_validity::TransactionValidityError,
};
use sp_std::marker::PhantomData;

/// Application tag (tenant) transaction metadata.
///
/// No any validity checks.
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct CheckPortalExt<T: Config>(T::PortalId, PhantomData<T>);

impl<T: Config> CheckPortalExt<T> {
    // utility constructor. Used only in client/factory code.
    pub fn from(id: T::PortalId) -> Self {
        Self(id, Default::default())
    }
}

impl<T: Config> sp_std::fmt::Debug for CheckPortalExt<T> {
    #[cfg(feature = "std")]
    fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        write!(f, "CheckPortalExt({:?})", self.0)
    }

    #[cfg(not(feature = "std"))]
    fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        Ok(())
    }
}

impl<T: Config + Send + Sync> SignedExtension for CheckPortalExt<T> where
    <T as frame_system::Config>::Call: Dispatchable<Info=DispatchInfo> + Send + Sync
{
    type AccountId = T::AccountId;
    type Call = <T as frame_system::Config>::Call;
    type AdditionalSigned = ();
    type Pre = T::PortalId;
    const IDENTIFIER: &'static str = "CheckPortalExt";

    fn additional_signed(&self) -> sp_std::result::Result<Self::AdditionalSigned, TransactionValidityError>
    {
        Ok(())
    }

    fn pre_dispatch(
        self,
        who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        self.validate(who, call, info, len)
            .map_err::<TransactionValidityError, _>(Into::into)?;
        Ok(self.0)
    }
    
    fn post_dispatch(
		pre: Self::Pre,
		_info: &DispatchInfoOf<Self::Call>,
		_post_info: &PostDispatchInfoOf<Self::Call>,
		_len: usize,
		_result: &DispatchResult,
	) -> Result<(), TransactionValidityError> {
        let portal_id = pre;
        let idx = System::<T>::extrinsic_index().unwrap_or_default();
        let n = System::<T>::block_number();
        crate::PortalTagOfTransaction::<T>::append(n, portal_id, idx);
		Ok(())
	}
}
