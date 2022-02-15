#![cfg_attr(not(feature = "std"), no_std)]

pub trait TenantLookupT<AccountId> {
    type TenantId;
    fn lookup(key: &AccountId) -> Option<Self::TenantId>;
}
