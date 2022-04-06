#![cfg(feature = "runtime-benchmarks")]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::*;
use core::convert::TryInto;
use frame_benchmarking::{account, benchmarks, whitelist_account, whitelisted_caller};
use frame_support::{traits::Get, weights::PostDispatchInfo};
use frame_system::{EventRecord, RawOrigin};
use sp_core::H160;
use sp_std::prelude::*;

use crate::{
    contract::{
        GenericContract, GenericContractOf, GenericContractStatus, License, LicenseOf,
        LicenseStatus, Terms, TermsOf,
    },
    Pallet,
};
use sp_runtime::traits::{Hash, Saturating, Scale, StaticLookup};

use deip_projects_info::DeipProjectsInfo;
use deip_serializable_u128::SerializableAtLeast32BitUnsigned;
use pallet_assets::Config as AssetsConfig;
use pallet_balances::Config as BalancesConfig;
use pallet_deip_assets::{
    Config as DeipAssetsConfig, DeipAssetIdOf, Pallet as DeipAssets, ProjectsInfoOf,
};

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::Event = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

fn init_member<T: Config>(index: u32) -> T::AccountId {
    let member = account::<T::AccountId>("member", index, SEED);
    whitelist_account!(member);
    member
}

fn init_team<T: Config>() -> T::DeipAccountId {
    whitelisted_caller::<T::AccountId>().into()
}

fn init_domain(idx: u8) -> Domain {
    Domain { external_id: DomainId::from([idx; 20]) }
}

fn create_domain<T: Config>(domain: Domain) -> Domain {
    let id = domain.external_id;
    let caller = whitelisted_caller::<T::AccountId>();
    Pallet::<T>::add_domain(RawOrigin::Signed(caller).into(), domain).unwrap();
    Domains::get(id)
}

fn init_project<T: Config>(idx: u8, domains: u8) -> ProjectOf<T> {
    let is_private: bool = false;
    let external_id: ProjectId = ProjectId::from([idx; 20]);
    let team_id: T::AccountId = init_team::<T>().into();
    let description: T::Hash = T::Hashing::hash("description".as_bytes());
    let domains: Vec<DomainId> = (0..domains)
        .map(|idx| create_domain::<T>(init_domain(idx + 1)).external_id)
        .collect();
    ProjectOf::<T> { is_private, external_id, team_id, description, domains }
}

fn _create_project<T: Config>(project: ProjectOf<T>) -> ProjectOf<T> {
    let ProjectOf::<T> { is_private, external_id, team_id, description, domains } = project;
    Pallet::<T>::create_project(
        RawOrigin::Signed(team_id.clone()).into(),
        is_private,
        external_id,
        team_id.into(),
        description,
        domains,
    )
    .unwrap();
    ProjectMapV1::<T>::get(external_id)
}

fn create_project_asset<T: Config + AssetsConfig + DeipAssetsConfig + BalancesConfig>(
    project: &ProjectOf<T>,
) -> DispatchResultWithPostInfo {
    pallet_balances::Pallet::<T>::set_balance(
        RawOrigin::Root.into(),
        T::Lookup::unlookup(project.team_id.clone()),
        <T as BalancesConfig>::Balance::max_value(),
        <T as BalancesConfig>::Balance::from(0u16),
    )
    .unwrap();
    DeipAssets::<T>::deip_create(
        RawOrigin::Signed(project.team_id.clone()).into(),
        T::AssetIdInit::asset_id(project.external_id.as_bytes()),
        project.team_id.clone().into(),
        <T as AssetsConfig>::Balance::from(200u16),
        Some(ProjectsInfoOf::<T>::project_id(project.external_id.as_bytes())),
    )
    .unwrap();
    Ok(None.into())
}

fn init_project_content<T: Config>(
    project: &ProjectOf<T>,
    authors: u8,
    references: Option<&[ProjectContentOf<T>]>,
) -> ProjectContentOf<T> {
    let external_id: ProjectContentId = project.external_id;
    let project_external_id: ProjectId = project.external_id;
    let team_id: T::AccountId = project.team_id.clone();
    let content_type: ProjectContentType = ProjectContentType::Announcement;
    let description: T::Hash = T::Hashing::hash("project content description".as_bytes());
    let content: T::Hash = T::Hashing::hash("project content".as_bytes());
    let authors: Vec<T::AccountId> = (0..authors).map(|idx| init_member::<T>(idx as u32)).collect();
    let references: Option<Vec<ProjectContentId>> =
        references.map(|x| x.iter().map(|y| y.external_id).collect());
    ProjectContentOf::<T> {
        external_id,
        project_external_id,
        team_id,
        content_type,
        description,
        content,
        authors,
        references,
    }
}

fn _create_project_content<T: Config>(project_content: ProjectContentOf<T>) -> ProjectContentOf<T> {
    let ProjectContentOf::<T> {
        external_id,
        project_external_id,
        team_id,
        content_type,
        description,
        content,
        authors,
        references,
    } = project_content;
    let authors = authors.into_iter().map(Into::into).collect();
    Pallet::<T>::create_project_content(
        RawOrigin::Signed(team_id.clone()).into(),
        external_id,
        project_external_id,
        team_id.into(),
        content_type,
        description,
        content,
        authors,
        references,
    )
    .unwrap();
    ProjectContentMapV1::<T>::get(external_id)
}

fn create_reference_project<T: Config>(project_idx: u8) -> ProjectContentOf<T> {
    let project = init_project::<T>(project_idx, 0);
    let project = _create_project::<T>(project);
    let project_content = init_project_content::<T>(&project, 0, None);
    _create_project_content::<T>(project_content)
}

fn project_ttl<T: Config>() -> T::Moment {
    T::MinimumPeriod::get().mul(T::BlockNumber::from(10u16))
}
fn now<T: Config>() -> T::Moment {
    pallet_timestamp::Pallet::<T>::get()
}

// fn init_project_nda<T: Config>(idx: u8, parties: &[ProjectOf<T>]) -> NdaOf<T>
// {
//     let contract_creator: T::AccountId = whitelisted_caller();
//     let external_id: NdaId = NdaId::from([idx; 20]);
//     let end_date: T::Moment = now::<T>() + project_ttl::<T>();
//     let start_date: Option<T::Moment> = Some(now::<T>());
//     let contract_hash: T::Hash = T::Hashing::hash("contract".as_bytes());
//     let projects: Vec<ProjectId> = parties.iter().map(|x| x.external_id).collect();
//     let parties: Vec<T::AccountId> = parties.iter().map(|x| x.team_id.clone()).collect();
//     NdaOf::<T> {
//         contract_creator,
//         external_id,
//         end_date,
//         start_date,
//         contract_hash,
//         parties,
//         projects,
//     }
// }

// fn _create_project_nda<T: Config>(nda: NdaOf<T>) -> NdaOf<T> {
//     let NdaOf::<T> {
//         contract_creator,
//         external_id,
//         end_date,
//         start_date,
//         contract_hash,
//         parties,
//         projects,
//     } = nda;
//     Pallet::<T>::create_project_nda(
//         RawOrigin::Signed(contract_creator).into(),
//         external_id,
//         end_date,
//         contract_hash,
//         start_date,
//         parties.into_iter().map(Into::into).collect(),
//         projects
//     ).unwrap();
//     NdaMapV1::<T>::get(external_id)
// }

// fn init_nda_content_access_request<T: Config>(idx: u8, nda: &NdaOf<T>) -> NdaAccessRequestOf<T>
// {
//     let external_id: NdaAccessRequestId = NdaAccessRequestId::from([idx; 20]);
//     let nda_external_id: NdaId = nda.external_id;
//     let requester: T::AccountId = whitelisted_caller();
//     let encrypted_payload_hash: T::Hash = T::Hashing::hash("encrypted payload".as_bytes());
//     let encrypted_payload_iv: Vec<u8> = "encrypted payload iv".as_bytes().to_vec();
//     NdaAccessRequestOf::<T> {
//         external_id,
//         nda_external_id,
//         requester,
//         encrypted_payload_hash,
//         encrypted_payload_iv,
//         status: NdaAccessRequestStatus::Pending,
//         grantor: None,
//         encrypted_payload_encryption_key: None,
//         proof_of_encrypted_payload_encryption_key: None,
//     }
// }

// fn _create_nda_content_access_request<T: Config>(request: NdaAccessRequestOf<T>)
//     -> NdaAccessRequestOf<T>
// {
//     let NdaAccessRequestOf::<T> {
//         external_id,
//         nda_external_id,
//         requester,
//         encrypted_payload_hash,
//         encrypted_payload_iv,
//         ..
//     } = request;
//     Pallet::<T>::create_nda_content_access_request(
//         RawOrigin::Signed(requester).into(),
//         external_id,
//         nda_external_id,
//         encrypted_payload_hash,
//         encrypted_payload_iv
//     ).unwrap();
//     NdaAccessRequestMapV1::<T>::get(external_id)
// }

fn init_review<T: Config>(
    idx: u8,
    domains: &[DomainId],
    project_content: &ProjectContentOf<T>,
) -> ReviewOf<T> {
    let external_id: ReviewId = ReviewId::from([idx; 20]);
    let author: T::AccountId = whitelisted_caller();
    let content: T::Hash = T::Hashing::hash("review content".as_bytes());
    let domains: Vec<DomainId> = domains.iter().copied().collect();
    let assessment_model: u32 = 10;
    let weight: Vec<u8> = vec![];
    let project_content_external_id: ProjectContentId = project_content.external_id;
    ReviewOf::<T> {
        external_id,
        author,
        content,
        domains,
        assessment_model,
        weight,
        project_content_external_id,
    }
}

fn _create_review<T: Config>(review: ReviewOf<T>) -> ReviewOf<T> {
    let ReviewOf::<T> {
        external_id,
        author,
        content,
        domains,
        assessment_model,
        weight,
        project_content_external_id,
    } = review;
    Pallet::<T>::create_review(
        RawOrigin::Signed(author.clone()).into(),
        external_id,
        author.into(),
        content,
        domains,
        assessment_model,
        weight,
        project_content_external_id,
    )
    .unwrap();
    ReviewMapV1::<T>::get(external_id)
}

benchmarks! {
    where_clause { where T: pallet_deip_assets::Config + pallet_balances::Config }

    create_project {
        let d in 1 .. 50;
        let project = init_project::<T>(0, d as u8);
    }: _(RawOrigin::Signed(project.team_id.clone()),
            project.is_private,
            project.external_id,
            project.team_id.clone().into(),
            project.description,
            project.domains.clone())
    verify {
        assert_last_event::<T>(Event::<T>::ProjectCreated(
            project.team_id.clone(),
            project
        ).into());
    }

    update_project {
        let project = init_project::<T>(0, 0);
        let project = _create_project::<T>(project);
        let description = T::Hashing::hash("updated description".as_bytes());
        let is_private = true;
    }: _(RawOrigin::Signed(project.team_id.clone()),
            project.external_id,
            Some(description),
            Some(is_private))
    verify {
        assert_last_event::<T>(Event::<T>::ProjectUpdated(
            project.team_id,
            project.external_id
        ).into());
    }

    create_project_content {
        let a in 0 .. 50;
        let r in 0 .. 50;

        let project = _create_project::<T>(init_project::<T>(1, 0));

        let references = (0..r)
            .map(|idx| create_reference_project::<T>(idx as u8 + 2))
            .collect::<Vec<_>>();

        let project_content = init_project_content::<T>(
            &project,
            a as u8,
            Some(references.as_slice())
        );
        let ProjectContentOf::<T> {
            external_id,
            project_external_id,
            team_id,
            content_type,
            description,
            content,
            authors,
            references,
        } = project_content;

        let authors: Vec<DeipAccountIdOf<T>> = authors.into_iter()
            .map(Into::into)
            .collect();

    }: _(RawOrigin::Signed(team_id.clone()),
            external_id,
            project_external_id,
            team_id.clone().into(),
            content_type,
            description,
            content,
            authors,
            references)
    verify {
        assert_last_event::<T>(Event::<T>::ProjectContnetCreated(
            team_id,
            external_id
        ).into());
    }

    // create_project_nda {
    //     let p in 0 .. T::MaxNdaParties::get().try_into().unwrap();
    //     let mut parties = vec![];
    //     for i in 0..p {
    //         let project = _create_project::<T>(init_project::<T>(i as u8 + 1, 0));
    //         parties.push(project);
    //     }
    //     let NdaOf::<T> {
    //         contract_creator,
    //         external_id,
    //         end_date,
    //         start_date,
    //         contract_hash,
    //         parties,
    //         projects,
    //     } = init_project_nda::<T>(1, parties.as_slice());
    //
    // }: _(RawOrigin::Signed(contract_creator.clone()),
    //         external_id,
    //         end_date,
    //         contract_hash,
    //         start_date,
    //         parties.into_iter().map(Into::into).collect(),
    //         projects)
    // verify {
    //     assert_last_event::<T>(Event::<T>::NdaCreated(
    //         contract_creator,
    //         external_id
    //     ).into());
    // }
    //
    // create_nda_content_access_request {
    //     let project = init_project::<T>(1, 0);
    //     let project = _create_project::<T>(project);
    //     let nda = init_project_nda::<T>(1, &[project]);
    //     let nda = _create_project_nda::<T>(nda);
    //     let NdaAccessRequestOf::<T> {
    //         external_id,
    //         nda_external_id,
    //         requester,
    //         encrypted_payload_hash,
    //         encrypted_payload_iv,
    //         ..
    //     } = init_nda_content_access_request::<T>(1, &nda);
    //
    // }: _(RawOrigin::Signed(requester.clone()),
    //         external_id,
    //         nda_external_id,
    //         encrypted_payload_hash,
    //         encrypted_payload_iv)
    // verify {
    //     assert_last_event::<T>(Event::<T>::NdaAccessRequestCreated(
    //         requester,
    //         external_id
    //     ).into());
    // }
    //
    // fulfill_nda_content_access_request {
    //     let project = init_project::<T>(1, 0);
    //     let project = _create_project::<T>(project);
    //     let nda = init_project_nda::<T>(1, &[project]);
    //     let nda = _create_project_nda::<T>(nda);
    //     let request = init_nda_content_access_request::<T>(1, &nda);
    //     let request = _create_nda_content_access_request::<T>(request);
    //
    //     let grantor: T::AccountId = whitelisted_caller();
    //     let encrypted_payload_encryption_key: Vec<u8> = vec![1];
    //     let proof_of_encrypted_payload_encryption_key: Vec<u8> = vec![2];
    //
    // }: _(RawOrigin::Signed(grantor.clone()),
    //         request.external_id,
    //         encrypted_payload_encryption_key,
    //         proof_of_encrypted_payload_encryption_key)
    // verify {
    //     assert_last_event::<T>(Event::<T>::NdaAccessRequestFulfilled(
    //         grantor,
    //         request.external_id
    //     ).into());
    // }
    //
    // reject_nda_content_access_request {
    //     let project = init_project::<T>(1, 0);
    //     let project = _create_project::<T>(project);
    //     let nda = init_project_nda::<T>(1, &[project]);
    //     let nda = _create_project_nda::<T>(nda);
    //     let request = init_nda_content_access_request::<T>(1, &nda);
    //     let request = _create_nda_content_access_request::<T>(request);
    //
    //     let grantor: T::AccountId = whitelisted_caller();
    //
    // }: _(RawOrigin::Signed(grantor.clone()), request.external_id)
    // verify {
    //     assert_last_event::<T>(Event::<T>::NdaAccessRequestRejected(
    //         grantor,
    //         request.external_id
    //     ).into());
    // }

    create_review {
        let d in 1 .. 50;

        let project = init_project::<T>(1, d as u8);
        let project = _create_project::<T>(project);

        let project_content = init_project_content::<T>(&project, 0, None);
        let project_content = _create_project_content::<T>(project_content);

        let review = init_review::<T>(1, &project.domains[..], &project_content);

    }: _(RawOrigin::Signed(review.author.clone()),
            review.external_id,
            review.author.clone().into(),
            review.content,
            review.domains.clone(),
            review.assessment_model,
            review.weight.clone(),
            review.project_content_external_id)
    verify {
        assert_last_event::<T>(Event::<T>::ReviewCreated(
            review.author.clone(),
            review
        ).into());
    }

    upvote_review {
        let project = init_project::<T>(1, 10);
        let project = _create_project::<T>(project);

        let project_content = init_project_content::<T>(&project, 0, None);
        let project_content = _create_project_content::<T>(project_content);

        let review = init_review::<T>(1, &project.domains[..], &project_content);
        let review = _create_review::<T>(review);

        let domain_id = review.domains[0];

    }: _(RawOrigin::Signed(review.author.clone()),
            review.external_id,
            domain_id)
    verify {
        assert_last_event::<T>(Event::<T>::ReviewUpvoted(
            review.external_id,
            review.author,
            domain_id
        ).into());
    }

    // add_domain {
    //     let domain = init_domain(1);
    //     let account: T::AccountId = whitelisted_caller();
    //     let external_id = domain.external_id;
    //
    // }: _(RawOrigin::Signed(account.clone()), domain)
    // verify {
    //     assert_last_event::<T>(Event::<T>::DomainAdded(
    //         account,
    //         external_id
    //     ).into());
    // }

    create_contract_agreement_project_license {
        let project = init_project::<T>(1, 0);
        let project = _create_project::<T>(project);

        let terms = init_license_agreement::<T>(
            &project,
            T::asset_id([38; 20].as_slice())
        );
        let parties = Parties::<T>::license_agreement(&project, init_member::<T>(99));
        let agreement = init_contract_agreement::<T>(1, terms, parties);

        let license = as_unsigned_license_agreement::<T>(agreement);

        let id = license.id;

    }: create_contract_agreement(RawOrigin::Signed(license.creator.clone()),
            license.id,
            license.creator.clone().into(),
            LicenseAgreementParties::<T> {
                licenser: license.licenser,
                licensee: license.licensee
            }.to_vec(),
            license.hash,
            license.activation_time,
            license.expiration_time,
            TermsOf::<T>::LicenseAgreement {
                source: license.project_id,
                price: license.price
            })
    verify {
        assert_last_event::<T>(Event::<T>::ContractAgreementCreated(
            id
        ).into());
    }

    create_contract_agreement_generic_contract {
        let project = init_project::<T>(1, 0);
        let project = _create_project::<T>(project);

        let terms = init_generic_contract_agreement::<T>();
        let parties = Parties::<T>::ContractAgreement((100..105).map(init_member::<T>).collect());
        let agreement = init_contract_agreement::<T>(1, terms, parties);

        let license = as_partially_accepted_contract::<T>(agreement);

        let id = license.id;

    }: create_contract_agreement(RawOrigin::Signed(license.creator.clone()),
            license.id,
            license.creator.clone().into(),
            Parties::<T>::ContractAgreement(license.parties).into_contract_agreement(),
            license.hash,
            license.activation_time,
            license.expiration_time,
            TermsOf::<T>::GenericContractAgreement)
    verify {
        assert_last_event::<T>(Event::<T>::ContractAgreementCreated(
            id
        ).into());
    }

    accept_contract_agreement_project_license_unsigned {
        let project = init_project::<T>(1, 0);

        let project = _create_project::<T>(project);

        let terms = init_license_agreement::<T>(
            &project,
            T::asset_id([38; 20].as_slice())
        );
        let parties = Parties::<T>::license_agreement(&project, init_member::<T>(99));
        let agreement = init_contract_agreement::<T>(1, terms, parties);
        let agreement = _create_contract_agreement::<T>(agreement);

        let license = as_unsigned_license_agreement::<T>(agreement);
        let id = license.id;
        let party = license.licenser;

    }: accept_contract_agreement(RawOrigin::Signed(party.clone()),
            id,
            party.clone().into())
    verify {
        assert_last_event::<T>(Event::<T>::ContractAgreementAccepted(
            id,
            party
        ).into());
    }

    accept_contract_agreement_project_license_signed_by_licenser {
        let project = init_project::<T>(1, 0);
        let project = _create_project::<T>(project);
        create_project_asset::<T>(&project)?;

        let terms = init_license_agreement::<T>(
            &project,
            T::asset_id([38; 20].as_slice())
        );
        let parties = Parties::<T>::license_agreement(&project, init_member::<T>(99));
        let agreement = init_contract_agreement::<T>(1, terms, parties);
        let agreement = _create_contract_agreement::<T>(agreement);

        let license = as_unsigned_license_agreement::<T>(agreement);

        let id = license.id;

        // Sign by Licenser:
        Pallet::<T>::accept_contract_agreement(
            RawOrigin::Signed(license.licenser.clone()).into(),
            id,
            license.licenser.clone().into()
        ).unwrap();

        let party = license.licensee;

        _add_balance::<T>(
            party.clone(),
            T::AssetIdInit::asset_id([38; 20].as_slice())
        )?;

    }: accept_contract_agreement(RawOrigin::Signed(party.clone()),
            id,
            party.clone().into())
    verify {
        assert_last_event::<T>(Event::<T>::ContractAgreementFinalized(
            id
        ).into());
    }

    accept_contract_agreement_generic_contract_partially_accepted {
        let project = init_project::<T>(1, 0);
        let project = _create_project::<T>(project);

        let terms = init_generic_contract_agreement::<T>();
        let parties = Parties::<T>::ContractAgreement((100..105).map(init_member::<T>).collect());
        let agreement = init_contract_agreement::<T>(1, terms, parties);
        let agreement = _create_contract_agreement::<T>(agreement);

        let contract = as_partially_accepted_contract::<T>(agreement);

        let id = contract.id;

        let party = contract.parties.get(0).unwrap().clone();

    }: accept_contract_agreement(RawOrigin::Signed(party.clone()),
            id,
            party.clone().into())
    verify {
        assert_last_event::<T>(Event::<T>::ContractAgreementAccepted(
            id,
            party
        ).into());
    }

    accept_contract_agreement_generic_contract_finalized {
        let project = init_project::<T>(1, 0);
        let project = _create_project::<T>(project);

        let terms = init_generic_contract_agreement::<T>();
        let parties = Parties::<T>::ContractAgreement((100..105).map(init_member::<T>).collect());
        let agreement = init_contract_agreement::<T>(1, terms, parties);
        let agreement = _create_contract_agreement::<T>(agreement);

        let contract = as_partially_accepted_contract::<T>(agreement);

        let id = contract.id;

        for p in &contract.parties[1..] {
            Pallet::<T>::accept_contract_agreement(
               RawOrigin::Signed(p.clone()).into(),
                id,
                p.clone().into()
            ).unwrap();
        }

        let party = contract.parties.get(0).unwrap().clone();

    }: accept_contract_agreement(RawOrigin::Signed(party.clone()),
            id,
            party.clone().into())
    verify {
        assert_last_event::<T>(Event::<T>::ContractAgreementFinalized(
            id,
        ).into());
    }
}

fn init_license_agreement<T: Config + DeipAssetsConfig>(
    source: &ProjectOf<T>,
    asset_id: crate::investment_opportunity::DeipAssetId<T>,
) -> TermsOf<T> {
    TermsOf::<T>::LicenseAgreement {
        source: source.external_id,
        price: DeipAsset::<T>::new(asset_id, <_>::from(200u16)),
    }
}

fn init_generic_contract_agreement<T: Config>() -> TermsOf<T> {
    TermsOf::<T>::GenericContractAgreement
}

struct LicenseAgreementParties<T: Config> {
    licenser: T::AccountId,
    licensee: T::AccountId,
}
enum Parties<T: Config> {
    LicenseAgreement(LicenseAgreementParties<T>),
    ContractAgreement(Vec<T::AccountId>),
}
impl<T: Config> LicenseAgreementParties<T> {
    fn to_vec<U: From<T::AccountId>>(self) -> Vec<U> {
        vec![self.licenser.into(), self.licensee.into()]
    }
}
impl<T: Config> Parties<T> {
    fn license_agreement(licenser: &ProjectOf<T>, licensee: T::AccountId) -> Self {
        Self::LicenseAgreement(LicenseAgreementParties {
            licenser: licenser.team_id.clone(),
            licensee,
        })
    }
    fn into_license_agreement(self) -> LicenseAgreementParties<T> {
        match self {
            Self::LicenseAgreement(parties) => parties,
            _ => unreachable!(),
        }
    }
    fn into_contract_agreement<U: From<T::AccountId>>(self) -> Vec<U> {
        match self {
            Self::ContractAgreement(parties) => parties.into_iter().map(U::from).collect(),
            _ => unreachable!(),
        }
    }
}

fn init_contract_agreement<T: Config>(
    idx: u8,
    terms: TermsOf<T>,
    parties: Parties<T>,
) -> ContractAgreementOf<T> {
    let id = ContractAgreementId::from([idx; 20]);
    let creator: T::AccountId = whitelisted_caller();

    let hash: HashOf<T> = T::Hashing::hash(b"contract agreement");
    let activation_time: Option<MomentOf<T>> = None;
    let expiration_time: Option<MomentOf<T>> = None;

    match terms {
        Terms::LicenseAgreement { source, price } => {
            let parties = parties.into_license_agreement();
            let license = License {
                id,
                creator,
                licenser: parties.licenser,
                licensee: parties.licensee,
                hash,
                activation_time,
                expiration_time,
                project_id: source,
                price,
            };
            ContractAgreementOf::<T>::License(LicenseStatus::Unsigned(license))
        },
        Terms::GenericContractAgreement => {
            use crate::contract::GenericContractStatus::PartiallyAccepted;
            let parties: Vec<T::AccountId> = parties.into_contract_agreement();
            let contract =
                GenericContract { id, creator, parties, hash, activation_time, expiration_time };
            ContractAgreementOf::<T>::GenericContract(PartiallyAccepted {
                contract,
                accepted_by: vec![],
            })
        },
    }
}

fn _create_contract_agreement<T: Config>(
    agreement: ContractAgreementOf<T>,
) -> ContractAgreementOf<T> {
    let id = match agreement {
        ContractAgreementOf::<T>::License(LicenseStatus::Unsigned(license)) => {
            let License {
                id,
                creator,
                licenser,
                licensee,
                hash,
                activation_time,
                expiration_time,
                project_id,
                price,
            } = license;
            Pallet::<T>::create_contract_agreement(
                RawOrigin::Signed(creator.clone()).into(),
                id,
                creator.into(),
                LicenseAgreementParties::<T> { licenser, licensee }.to_vec(),
                hash,
                activation_time,
                expiration_time,
                TermsOf::<T>::LicenseAgreement { source: project_id, price },
            )
            .unwrap();
            id
        },
        ContractAgreementOf::<T>::GenericContract(GenericContractStatus::PartiallyAccepted {
            contract,
            ..
        }) => {
            let GenericContract { id, creator, parties, hash, activation_time, expiration_time } =
                contract;
            Pallet::<T>::create_contract_agreement(
                RawOrigin::Signed(creator.clone()).into(),
                id,
                creator.into(),
                Parties::<T>::ContractAgreement(parties).into_contract_agreement(),
                hash,
                activation_time,
                expiration_time,
                TermsOf::<T>::GenericContractAgreement,
            )
            .unwrap();
            id
        },
        _ => unreachable!(),
    };
    ContractAgreementMap::<T>::get(id)
}

fn as_unsigned_license_agreement<T: Config>(agreement: ContractAgreementOf<T>) -> LicenseOf<T> {
    match agreement {
        ContractAgreementOf::<T>::License(LicenseStatus::Unsigned(license)) => license,
        _ => unreachable!(),
    }
}

fn as_partially_accepted_contract<T: Config>(
    agreement: ContractAgreementOf<T>,
) -> GenericContractOf<T> {
    match agreement {
        ContractAgreementOf::<T>::GenericContract(GenericContractStatus::PartiallyAccepted {
            contract,
            ..
        }) => contract,
        _ => unreachable!(),
    }
}

use deip_asset_system::AssetIdInitT;
use sp_runtime::traits::Bounded;

fn _add_balance<T: Config + AssetsConfig + DeipAssetsConfig + BalancesConfig>(
    party: T::AccountId,
    asset_id: pallet_deip_assets::DeipAssetIdOf<T>,
) -> DispatchResultWithPostInfo {
    pallet_balances::Pallet::<T>::set_balance(
        RawOrigin::Root.into(),
        T::Lookup::unlookup(party.clone()),
        <T as BalancesConfig>::Balance::max_value(),
        <T as BalancesConfig>::Balance::from(0u16),
    )
    .unwrap();

    let asset_admin: <T as DeipAssetsConfig>::DeipAccountId = party.clone().into();
    let min_balance = <T as AssetsConfig>::Balance::from(200u16);

    DeipAssets::<T>::deip_create(
        RawOrigin::Signed(party.clone()).into(),
        asset_id.clone(),
        asset_admin.clone(),
        min_balance.clone(),
        None,
    )
    .unwrap();

    DeipAssets::<T>::deip_mint(
        RawOrigin::Signed(party.clone()).into(),
        asset_id,
        asset_admin,
        min_balance,
    )?;

    Ok(None.into())
}

fn _create_asset<T: Config + AssetsConfig + DeipAssetsConfig + BalancesConfig>(
    asset_id: pallet_deip_assets::DeipAssetIdOf<T>,
    admin: T::AccountId,
    min_balance: <T as AssetsConfig>::Balance,
) -> DispatchResultWithPostInfo {
    pallet_balances::Pallet::<T>::set_balance(
        RawOrigin::Root.into(),
        T::Lookup::unlookup(admin.clone()),
        <T as BalancesConfig>::Balance::max_value(),
        <T as BalancesConfig>::Balance::min_value(),
    )?;

    let asset_admin: <T as DeipAssetsConfig>::DeipAccountId = admin.clone().into();

    DeipAssets::<T>::deip_create(
        RawOrigin::Signed(admin).into(),
        asset_id,
        asset_admin,
        min_balance,
        None,
    )
}

fn _mint<T: Config + AssetsConfig + DeipAssetsConfig + BalancesConfig>(
    asset_id: pallet_deip_assets::DeipAssetIdOf<T>,
    admin: T::AccountId,
    beneficiary: T::AccountId,
    amount: <T as AssetsConfig>::Balance,
) -> DispatchResultWithPostInfo {
    DeipAssets::<T>::deip_mint(
        RawOrigin::Signed(admin).into(),
        asset_id,
        beneficiary.into(),
        amount,
    )
}
