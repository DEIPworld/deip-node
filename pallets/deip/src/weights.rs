//! Autogenerated weights for pallet_deip
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2022-01-19, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// appchain-deip
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_deip
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages
// 4096
// --output
// weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn create_project(d: u32) -> Weight;
    fn update_project() -> Weight;
    fn create_project_content(a: u32, r: u32) -> Weight;
    fn create_project_nda(p: u32) -> Weight;
    fn create_nda_content_access_request() -> Weight;
    fn fulfill_nda_content_access_request() -> Weight;
    fn reject_nda_content_access_request() -> Weight;
    fn create_review(d: u32) -> Weight;
    fn upvote_review() -> Weight;
    // fn add_domain() -> Weight;
    fn create_contract_agreement_project_license() -> Weight;
    fn create_contract_agreement_general_contract() -> Weight;
    fn accept_contract_agreement_project_license_unsigned() -> Weight;
    // fn accept_contract_agreement_project_license_signed_by_licenser() -> Weight;
    fn accept_contract_agreement_general_contract_partially_accepted() -> Weight;
    fn accept_contract_agreement_general_contract_finalized() -> Weight;
}

/// Weight functions for pallet_deip.
pub struct Weights<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for Weights<T> {
    // Storage: Deip Domains (r:1 w:0)
    // Storage: Deip ProjectMap (r:1 w:1)
    // Storage: Deip ProjectIdByTeamId (r:0 w:1)
    fn create_project(d: u32) -> Weight {
        (45_940_000 as Weight)
            // Standard Error: 111_000
            .saturating_add((9_544_000 as Weight).saturating_mul(d as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(d as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Deip ProjectMap (r:1 w:1)
    fn update_project() -> Weight {
        (36_294_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Deip ProjectContentMap (r:51 w:1)
    // Storage: Deip ProjectMap (r:1 w:0)
    // Storage: Deip ContentIdByProjectId (r:1 w:1)
    fn create_project_content(a: u32, r: u32) -> Weight {
        (34_234_000 as Weight)
            // Standard Error: 64_000
            .saturating_add((871_000 as Weight).saturating_mul(a as Weight))
            // Standard Error: 64_000
            .saturating_add((8_656_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Deip Ndas (r:1 w:1)
    // Storage: Deip NdaMap (r:0 w:1)
    // Storage: Deip ProjectMap (r:1 w:0)
    fn create_project_nda(p: u32) -> Weight {
        (71_768_000 as Weight)
            // Standard Error: 60_000
            .saturating_add((8_374_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Deip NdaMap (r:1 w:0)
    // Storage: Deip NdaAccessRequests (r:1 w:1)
    // Storage: Deip NdaAccessRequestMap (r:0 w:1)
    fn create_nda_content_access_request() -> Weight {
        (60_406_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Deip NdaAccessRequestMap (r:1 w:1)
    // Storage: Deip NdaMap (r:1 w:0)
    fn fulfill_nda_content_access_request() -> Weight {
        (51_652_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Deip NdaAccessRequestMap (r:1 w:1)
    // Storage: Deip NdaMap (r:1 w:0)
    fn reject_nda_content_access_request() -> Weight {
        (50_083_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Deip Domains (r:1 w:0)
    // Storage: Deip ReviewMap (r:1 w:1)
    // Storage: Deip ProjectContentMap (r:1 w:0)
    // Storage: Deip ReviewIdByAccountId (r:0 w:1)
    // Storage: Deip ReviewIdByProjectId (r:0 w:1)
    // Storage: Deip ReviewIdByContentId (r:0 w:1)
    fn create_review(d: u32) -> Weight {
        (54_148_000 as Weight)
            // Standard Error: 199_000
            .saturating_add((9_477_000 as Weight).saturating_mul(d as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(d as Weight)))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    // Storage: Deip Domains (r:1 w:0)
    // Storage: Deip ReviewMap (r:1 w:0)
    // Storage: Deip ReviewVoteMap (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Deip VoteIdByReviewId (r:0 w:1)
    // Storage: Deip VoteIdByAccountId (r:0 w:1)
    fn upvote_review() -> Weight {
        (87_332_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Deip DomainCount (r:1 w:1)
    // Storage: Deip Domains (r:1 w:1)
    // fn add_domain() -> Weight {
    //     (46_126_000 as Weight)
    //         .saturating_add(T::DbWeight::get().reads(2 as Weight))
    //         .saturating_add(T::DbWeight::get().writes(2 as Weight))
    // }
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Deip ContractAgreementMap (r:1 w:1)
    // Storage: Deip ProjectMap (r:1 w:0)
    // Storage: Deip ContractAgreementIdByType (r:0 w:1)
    fn create_contract_agreement_project_license() -> Weight {
        (62_469_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Deip ContractAgreementMap (r:1 w:1)
    // Storage: Deip ContractAgreementIdByType (r:0 w:1)
    fn create_contract_agreement_general_contract() -> Weight {
        (54_880_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Deip ContractAgreementMap (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    fn accept_contract_agreement_project_license_unsigned() -> Weight {
        (54_005_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Deip ContractAgreementMap (r:1 w:1)
    fn accept_contract_agreement_general_contract_partially_accepted() -> Weight {
        (47_963_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Deip ContractAgreementMap (r:1 w:1)
    fn accept_contract_agreement_general_contract_finalized() -> Weight {
        (61_951_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}
