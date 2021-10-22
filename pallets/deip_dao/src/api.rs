use sp_std::prelude::*;

use codec::Codec;

use super::dao::{DaoId, Dao};

pub type GetResult<AccountId> = Option<Dao<AccountId, DaoId>>;
pub type GetMultiResult<AccountId> = Vec<Option<Dao<AccountId, DaoId>>>;

sp_api::decl_runtime_apis! {
    pub trait DeipDaoRuntimeApi<AccountId>
        where AccountId: Codec
    {
        fn get(id: DaoId) -> GetResult<AccountId>;
        fn get_multi(ids: Vec<DaoId>) -> GetMultiResult<AccountId>;
    }
}

use super::{Pallet, Config, DaoRepository};

impl<T: Config> Pallet<T> {
    pub fn rpc_get(id: DaoId) -> GetResult<T::AccountId> {
        DaoRepository::<T>::try_get(id).ok()
    }
    pub fn rpc_get_multi(ids: Vec<DaoId>) -> GetMultiResult<T::AccountId> {
        ids.into_iter().map(|x| DaoRepository::<T>::try_get(x).ok()).collect()
    }
}
