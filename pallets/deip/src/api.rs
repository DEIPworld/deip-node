#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use super::*;
use codec::Codec;
use sp_std::vec::Vec;

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
    pub trait DeipApi<AccountId, Moment, AssetId, AssetBalance, Hash>
        where
            AccountId: Codec,
            Moment: Codec,
            AssetId: Codec,
            AssetBalance: Codec,
            Hash: Codec,
    {
        fn get_project(project_id: &ProjectId) -> Option<Project<Hash, AccountId>>;
        fn get_domain(domain_id: &DomainId) -> Option<Domain>;
        fn get_project_content(id: &ProjectContentId) -> Option<ProjectContent<Hash, AccountId>>;
        fn get_nda(nda_id: &NdaId) -> Option<Nda<Hash, AccountId, Moment>>;
        fn get_review(review_id: &ReviewId) -> Option<Review<Hash, AccountId>>;
        fn get_investment_opportunity(id: &InvestmentId) -> Option<SimpleCrowdfunding<Moment, AssetId, AssetBalance>>;
        fn get_contract_agreement(id: &ContractAgreementId) -> Option<contract::Agreement<AccountId, Hash, Moment, DeipAsset<AssetId, AssetBalance>>>;
    }
}
