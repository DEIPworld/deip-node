use crate::*;
use crate::{mock::*};
use sp_core::{H256, offchain::{TransactionPoolExt, testing::*}};
use frame_support::{assert_ok, assert_noop,
    traits::{UnfilteredDispatchable, OnFinalize, OnInitialize, OffchainWorker}};
use sp_runtime::offchain::OffchainWorkerExt;
use std::time::{SystemTime, UNIX_EPOCH};
use sp_io::TestExternalities;
use sp_std::sync::Arc;
use parking_lot::RwLock;
use sp_runtime::traits::{Zero, One};

const DAY_IN_MILLIS: u64 = 86400000;

type BlockNumber = <Test as system::Config>::BlockNumber;

fn create_ok_project(maybe_account_id: Option<<Test as system::Config>::AccountId>) 
    -> (ProjectId, ProjectOf<Test>, DomainId, <Test as system::Config>::AccountId, ) {
    let domain_id = DomainId::random();
    let account_id: <Test as system::Config>::AccountId = maybe_account_id.unwrap_or(DEFAULT_ACCOUNT_ID);
    let project_id = ProjectId::random();

    assert_ok!(Deip::add_domain(Origin::signed(account_id), Domain { external_id: domain_id.clone() }));

    let project = ProjectOf::<Test> {
        is_private: false,
        external_id: project_id,
        team_id: account_id,
        description: H256::random(),
        domains: vec![domain_id],
    };

    assert_ok!(Deip::create_project(Origin::signed(account_id),
        project.is_private,
        project.external_id.clone(),
        project.team_id.clone(),
        project.description.clone(),
        project.domains.clone(),
    ));

    (project_id, project, domain_id, account_id)
}

fn create_ok_nda() -> (NdaId, NdaOf<Test>) {
    let (project_id, ..) = create_ok_project(None);
    let project_nda_id =  NdaId::random();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards :)").as_millis() as u64;
    
    let end_date = now + DAY_IN_MILLIS;
    let contract_hash = H256::random();
    let maybe_start_date = None;
    let parties = vec![DEFAULT_ACCOUNT_ID];
    let projects = vec![project_id];
    

    assert_ok!(
        Deip::create_project_nda(
            Origin::signed(DEFAULT_ACCOUNT_ID), 
            project_nda_id, 
            end_date, 
            contract_hash, 
            maybe_start_date,
            parties.clone(),
            projects.clone()
        )
    );

    let expected_nda  = Nda {
        contract_creator: DEFAULT_ACCOUNT_ID,
        external_id: project_nda_id,
        end_date,
        start_date: maybe_start_date,
        contract_hash,
        parties,
        projects
    };

    (project_nda_id, expected_nda)
        
}

fn create_ok_nda_content_access_request(project_nda_id: NdaId) -> (NdaAccessRequestId, NdaAccessRequestOf<Test>) {
    let access_request_id = NdaAccessRequestId::random();
    let encrypted_payload_hash = H256::random();
    let encrypted_payload_iv = vec![1, 2, 3];

    assert_ok!(
        Deip::create_nda_content_access_request(
            Origin::signed(DEFAULT_ACCOUNT_ID), 
            access_request_id, 
            project_nda_id,
            encrypted_payload_hash, 
            encrypted_payload_iv.clone()
        )
    );

    let expected_nda_request = NdaAccessRequest {
        external_id: access_request_id,
        nda_external_id: project_nda_id, 
        requester: DEFAULT_ACCOUNT_ID,
        encrypted_payload_hash,
        encrypted_payload_iv,
        status: NdaAccessRequestStatus::Pending,
        grantor: None,
        encrypted_payload_encryption_key: None,
        proof_of_encrypted_payload_encryption_key: None,
    };

    (access_request_id, expected_nda_request)
}

fn create_issue_asset(
    account_id: AccountIdOf<Test>,
    id: DeipAssetIdOf<Test>,
    amount: DeipAssetBalanceOf<Test>,
    project_id: Option<ProjectId>,
) {
    let call = pallet_deip_assets::Call::<Test>::create_asset(
        id,
        account_id,
        1u32.into(),
        project_id,
    );
    let result = call.dispatch_bypass_filter(Origin::signed(account_id));
    assert_ok!(result);

    let call = pallet_deip_assets::Call::<Test>::issue_asset(id, account_id, amount);
    let result = call.dispatch_bypass_filter(Origin::signed(account_id));
    assert_ok!(result);
}

/// convert an externalities to one that can handle offchain worker tests.
/// Check substrate-v3.0.0/frame/staking/src/tests.rs +3452
fn offchainify(ext: &mut TestExternalities, iterations: u32) -> Arc<RwLock<PoolState>> {
    let (offchain, offchain_state) = TestOffchainExt::new();
    let (pool, pool_state) = TestTransactionPoolExt::new();

    let mut seed = [0_u8; 32];
    seed[0..4].copy_from_slice(&iterations.to_le_bytes());
    offchain_state.write().seed = seed;

    ext.register_extension(OffchainWorkerExt::new(offchain));
    ext.register_extension(TransactionPoolExt::new(pool));

    pool_state
}

fn decode_validate_deip_call(encoded: &[u8]) -> crate::Call<Test> {
    let mut encoded = encoded.clone();
    let extrinsic: Extrinsic = Decode::decode(&mut encoded).unwrap();

    let call = extrinsic.call;
    let inner = match call {
        mock::Call::Deip(inner) => inner,
        _ => unreachable!(),
    };

    assert_eq!(
        <Deip as sp_runtime::traits::ValidateUnsigned>::validate_unsigned(
            TransactionSource::Local,
            &inner,
        ).is_ok(),
        true
    );

    inner
}

#[test]
fn add_domain() {
    new_test_ext().execute_with(|| {
        let domain_id = DomainId::random();
        // Dispatch a signed add domian extrinsic.
        assert_ok!(Deip::add_domain(Origin::signed(DEFAULT_ACCOUNT_ID), Domain { external_id: domain_id.clone() }));
        
        // Read pallet storage and assert an expected result.
        assert_eq!(Deip::domain_count(), 1);
        assert!(
            <Domains>::contains_key(domain_id),
            "Domains did not contain domain, value was `{}`",
            domain_id
        );
    });
}

#[test]
fn cant_add_duplicate_domain() {
    new_test_ext().execute_with(|| {
        let domain_id = DomainId::random();
        
        assert_ok!(Deip::add_domain(Origin::signed(DEFAULT_ACCOUNT_ID), Domain { external_id: domain_id.clone() }));

        assert_noop!(
            Deip::add_domain(Origin::signed(DEFAULT_ACCOUNT_ID), Domain { external_id: domain_id.clone() }),
            Error::<Test>::DomainAlreadyExists
        );
    })
}

#[test]
fn add_project() {
    new_test_ext().execute_with(|| {
        let (project_id, project, _, team) = create_ok_project(None);

        let project_stored = ProjectMap::<Test>::get(project_id);

        assert!(
            <ProjectMap<Test>>::contains_key(project_id),
            "Project Map did not contain the project, value was `{}`",
            project_id
        );

        assert_eq!(project, project_stored);

        assert!(
            ProjectIdByTeamId::<Test>::contains_key(team, project_id),
            "ProjectIdByTeamId did not contain project, value was `{}`",
            project_id
        );
    })
}

#[test]
fn cant_add_project_with_non_exixsted_domain() {
    new_test_ext().execute_with(|| {
        let domain = DomainId::random();
        let account_id = DEFAULT_ACCOUNT_ID;

        assert_noop!(
            Deip::create_project(Origin::signed(DEFAULT_ACCOUNT_ID),
                false,
                ProjectId::random(),
                account_id,
                H256::random(),
                vec![domain]),
            Error::<Test>::DomainNotExists
        );
    })
}

#[test]
fn cant_add_duplicated_project() {
    new_test_ext().execute_with(|| {
        let (_, project, ..) = create_ok_project(None);

        assert_noop!(
            Deip::create_project(Origin::signed(DEFAULT_ACCOUNT_ID),
                project.is_private,
                project.external_id,
                project.team_id,
                project.description,
                project.domains),
            Error::<Test>::ProjectAlreadyExists
        );
    })
}

#[test]
fn update_project() {
    new_test_ext().execute_with(|| {
        let (project_id, ..) = create_ok_project(None);

        let new_description = H256::random();

        assert_ok!(Deip::update_project(Origin::signed(DEFAULT_ACCOUNT_ID), project_id, Some(new_description), Some(true)));

        let project_stored = ProjectMap::<Test>::get(project_id);

        assert_eq!(project_stored.description, new_description);
        assert_eq!(project_stored.is_private, true);
    })
}

#[test]
fn cant_update_project_not_belonged_to_your_signature() {
    new_test_ext().execute_with(|| {
        let account_id: u64 = 2;
        let wrong_account_id = 1;

        let (project_id, ..) = create_ok_project(Some(account_id));

        let new_description = H256::random();

        assert_noop!(
            Deip::update_project(Origin::signed(wrong_account_id), project_id, Some(new_description), Some(true)),
            Error::<Test>::NoPermission
        );
    })
}

#[test]
fn cant_update_not_existed_project() {
    new_test_ext().execute_with(|| {
        let project_id = ProjectId::random();

        assert_noop!(
            Deip::update_project(Origin::signed(DEFAULT_ACCOUNT_ID), project_id, None, None),
            Error::<Test>::NoSuchProject
        );
    })
}

#[test]
fn create_project_content() {
    new_test_ext().execute_with(|| {
        let (project_id, ..) = create_ok_project(None);
        let project_content_id =  ProjectContentId::random();

        assert_ok!(Deip::create_project_content(Origin::signed(DEFAULT_ACCOUNT_ID),
            project_content_id,
            project_id,
            DEFAULT_ACCOUNT_ID,
            ProjectContentType::Announcement,
            H256::random(),
            H256::random(),
            vec![DEFAULT_ACCOUNT_ID],
            None));

        assert!(
            <ProjectContentMap<Test>>::contains_key(project_content_id),
            "Project Content Map did not contain key, value was `{}`",
            project_content_id
        );

        assert!(
            ContentIdByProjectId::contains_key(project_id, project_content_id),
            "ContentIdByProjectId does not contain the key: `{}`, `{}`",
            project_id,
            project_content_id
        );
    })
}

#[test]
fn create_project_content_with_references() {
    new_test_ext().execute_with(|| {
        let (project_id, ..) = create_ok_project(None);

        let project_content_id = ProjectContentId::random();
        let description = H256::random();
        let content = H256::random();

        assert_ok!(Deip::create_project_content(Origin::signed(DEFAULT_ACCOUNT_ID),
            project_content_id,
            project_id,
            DEFAULT_ACCOUNT_ID,
            ProjectContentType::Announcement,
            description,
            content,
            vec![DEFAULT_ACCOUNT_ID],
            None));

        let project_content_with_reference_id = ProjectContentId::random();

        assert_ok!(Deip::create_project_content(Origin::signed(DEFAULT_ACCOUNT_ID),
            project_content_with_reference_id,
            project_id,
            DEFAULT_ACCOUNT_ID,
            ProjectContentType::Announcement,
            description,
            content,
            vec![DEFAULT_ACCOUNT_ID],
            Some(vec![project_content_id])));

        assert!(
            <ProjectContentMap<Test>>::contains_key(project_content_with_reference_id),
            "Project Content Map did not contain key, value was `{}`",
            project_content_with_reference_id
        );

        assert!(
            ContentIdByProjectId::contains_key(project_id, project_content_with_reference_id),
            "ContentIdByProjectId does not contain the key: `{}`, `{}`",
            project_id,
            project_content_with_reference_id
        );
    })
}

#[test]
fn cant_add_duplicated_project_content() {
    new_test_ext().execute_with(|| {
        let (project_id, ..) = create_ok_project(None);

        let content_id = ProjectContentId::random();
        let description = H256::random();
        let content = H256::random();

        assert_ok!(Deip::create_project_content(Origin::signed(DEFAULT_ACCOUNT_ID),
            content_id,
            project_id,
            DEFAULT_ACCOUNT_ID,
            ProjectContentType::Announcement,
            description,
            content,
            vec![DEFAULT_ACCOUNT_ID],
            None));

        assert_noop!(
            Deip::create_project_content(Origin::signed(DEFAULT_ACCOUNT_ID),
                content_id,
                project_id,
                DEFAULT_ACCOUNT_ID,
                ProjectContentType::Announcement,
                description,
                content,
                vec![DEFAULT_ACCOUNT_ID],
                None),
            Error::<Test>::ProjectContentAlreadyExists
        );
    })
}

#[test]
fn cant_add_project_content_with_wrong_project_reference() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Deip::create_project_content(Origin::signed(DEFAULT_ACCOUNT_ID),
                ProjectContentId::random(),
                ProjectId::random(),
                DEFAULT_ACCOUNT_ID,
                ProjectContentType::Announcement,
                H256::random(),
                H256::random(),
                vec![DEFAULT_ACCOUNT_ID],
                None),
            Error::<Test>::NoSuchProject
        );
    })
}

#[test]
fn cant_add_project_content_to_incorrect_team() {
    new_test_ext().execute_with(|| {
        let (project_id, ..) = create_ok_project(None);
        let wrong_account_id = 234;

        assert_noop!(
            Deip::create_project_content(Origin::signed(DEFAULT_ACCOUNT_ID),
                ProjectContentId::random(),
                project_id,
                wrong_account_id,
                ProjectContentType::Announcement,
                H256::random(),
                H256::random(),
                vec![DEFAULT_ACCOUNT_ID],
                None),
            Error::<Test>::ProjectNotBelongToTeam
        );
    })
}

#[test]
fn cant_add_project_content_to_finished_project() {
    new_test_ext().execute_with(|| {
        let (project_id, ..) = create_ok_project(None);
        
        let description = H256::random();
        let content = H256::random();

        assert_ok!(Deip::create_project_content(Origin::signed(DEFAULT_ACCOUNT_ID),
            ProjectContentId::random(),
            project_id,
            DEFAULT_ACCOUNT_ID,
            ProjectContentType::FinalResult,
            description,
            content,
            vec![DEFAULT_ACCOUNT_ID],
            None,));

        assert_noop!(
            Deip::create_project_content(Origin::signed(DEFAULT_ACCOUNT_ID),
                ProjectContentId::random(),
                project_id,
                DEFAULT_ACCOUNT_ID,
                ProjectContentType::MilestoneCode,
                description,
                content,
                vec![DEFAULT_ACCOUNT_ID],
                None,),
            Error::<Test>::ProjectAlreadyFinished
        );
    })
}

#[test]
fn cant_add_project_content_with_wrong_references() {
    new_test_ext().execute_with(|| {
        let (project_id, ..) = create_ok_project(None);
        
        assert_noop!(
            Deip::create_project_content(Origin::signed(DEFAULT_ACCOUNT_ID),
                ProjectContentId::random(),
                project_id,
                DEFAULT_ACCOUNT_ID,
                ProjectContentType::Announcement,
                H256::random(),
                H256::random(),
                vec![DEFAULT_ACCOUNT_ID],
                Some(vec![ProjectContentId::random()]),),
            Error::<Test>::NoSuchReference
        );
    })
}

#[test]
fn create_project_nda() {
    new_test_ext().execute_with(|| {
        let (project_nda_id, expected_nda) = create_ok_nda();

        let nda_list = Ndas::<Test>::get();
        let nda_stored = NdaMap::<Test>::get(project_nda_id);

        assert!(
            <NdaMap<Test>>::contains_key(project_nda_id),
            "NDA Map did not contain key, value was `{}`",
            project_nda_id
        );

        assert_eq!(expected_nda, nda_stored);

        assert!(
            nda_list.binary_search_by_key(&project_nda_id, |&(external_id, ..)| external_id).is_ok(),
            "NDA List did not contain the NDA, value was `{}`",
            project_nda_id
        );
    })
}

#[test]
fn cant_create_project_nda_ends_in_past() {
    new_test_ext().execute_with(|| {
        let (project_id, ..) = create_ok_project(None);
        let project_nda_id =  NdaId::random();        
        let end_date = 0;

        let contract_hash = H256::random();
        let maybe_start_date = None;
        let parties = vec![DEFAULT_ACCOUNT_ID];
        let projects = vec![project_id];

        assert_noop!(
            Deip::create_project_nda(
                Origin::signed(DEFAULT_ACCOUNT_ID), 
                project_nda_id, 
                end_date, 
                contract_hash, 
                maybe_start_date,
                parties.clone(),
                projects.clone()
            ),
            Error::<Test>::NdaEndDateMustBeLaterCurrentMoment
        );
    })
}

#[test]
fn cant_create_project_nda_with_start_date_greater_end_date() {
    new_test_ext().execute_with(|| {
        let (project_id, ..) = create_ok_project(None);
        let project_nda_id =  NdaId::random();        

        let end_date = 1;
        let maybe_start_date = Some(3);

        let contract_hash = H256::random();

        let parties = vec![DEFAULT_ACCOUNT_ID];
        let projects = vec![project_id];

        assert_noop!(
            Deip::create_project_nda(
                Origin::signed(DEFAULT_ACCOUNT_ID), 
                project_nda_id, 
                end_date, 
                contract_hash, 
                maybe_start_date,
                parties.clone(),
                projects.clone()
            ),
            Error::<Test>::NdaStartDateMustBeLessThanEndDate
        );
    })
}

#[test]
fn cant_create_project_nda_with_non_existed_project() {
    new_test_ext().execute_with(|| {
        let project_id = ProjectId::random();
        let project_nda_id =  NdaId::random();        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards :)").as_millis() as u64;
    
        let end_date = now + DAY_IN_MILLIS;
        
        let contract_hash = H256::random();
        let maybe_start_date = None;
        let parties = vec![DEFAULT_ACCOUNT_ID];
        let projects = vec![project_id];
        

        assert_noop!(
            Deip::create_project_nda(
                Origin::signed(DEFAULT_ACCOUNT_ID), 
                project_nda_id, 
                end_date, 
                contract_hash, 
                maybe_start_date,
                parties.clone(),
                projects.clone()
            ),
            Error::<Test>::NoSuchProject
        );

    })
}

#[test]
fn cant_create_project_nda_with_not_correct_parties() {
    new_test_ext().execute_with(|| {
        let (project_id, ..) = create_ok_project(None);
        let project_nda_id =  NdaId::random();        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards :)").as_millis() as u64;
    
        let end_date = now + DAY_IN_MILLIS;

        let wrong_account_id = 4;
        
        let contract_hash = H256::random();
        let maybe_start_date = None;
        let parties = vec![wrong_account_id];
        let projects = vec![project_id];
        

        assert_noop!(
            Deip::create_project_nda(
                Origin::signed(wrong_account_id), 
                project_nda_id, 
                end_date, 
                contract_hash, 
                maybe_start_date,
                parties.clone(),
                projects.clone()
            ),
            Error::<Test>::TeamOfAllProjectsMustSpecifiedAsParty
        );
    })
}

#[test]
fn cant_create_duplicated_project_nda() {
    new_test_ext().execute_with(|| {
        let (project_nda_id, ..) = create_ok_nda();
        
        let (project_id, ..) = create_ok_project(None);    
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards :)").as_millis() as u64;
    
        let end_date = now + DAY_IN_MILLIS;
        
        let contract_hash = H256::random();
        let maybe_start_date = None;
        let parties = vec![DEFAULT_ACCOUNT_ID];
        let projects = vec![project_id];

        assert_noop!(
            Deip::create_project_nda(
                Origin::signed(DEFAULT_ACCOUNT_ID), 
                project_nda_id, 
                end_date, 
                contract_hash, 
                maybe_start_date,
                parties.clone(),
                projects.clone()
            ),
            Error::<Test>::NdaAlreadyExists
        );
    })
}

#[test]
fn create_nda_content_access_request() {
    new_test_ext().execute_with(|| {
        let (project_nda_id, ..) = create_ok_nda();

        let (access_request_id, expected_nda_request) = create_ok_nda_content_access_request(project_nda_id);

        let nda_list = NdaAccessRequests::<Test>::get();
        let nda_stored = NdaAccessRequestMap::<Test>::get(access_request_id);

        assert!(
            <NdaAccessRequestMap<Test>>::contains_key(access_request_id),
            "NDA request Map did not contain key, value was `{}`",
            access_request_id

        );

        assert_eq!(expected_nda_request, nda_stored);

        assert!(
            nda_list.binary_search_by_key(&access_request_id, |&(external_id, ..)| external_id).is_ok(),
            "NDA request List did not contain the NDA request, value was `{}`",
            access_request_id
        );

    })
}


#[test]
fn cant_create_nda_content_access_with_non_existed_nda() {
    new_test_ext().execute_with(|| {
        let project_nda_id = NdaId::random();

        let access_request_id = NdaAccessRequestId::random();
        let encrypted_payload_hash = H256::random();
        let encrypted_payload_iv = vec![1, 2, 3];

        assert_noop!(
            Deip::create_nda_content_access_request(
                Origin::signed(DEFAULT_ACCOUNT_ID), 
                access_request_id, 
                project_nda_id,
                encrypted_payload_hash, 
                encrypted_payload_iv.clone()
            ),
            Error::<Test>::NoSuchNda
        );

    })
}

#[test]
fn cant_create_duplicated_nda_content_access() {
    new_test_ext().execute_with(|| {
        let (project_nda_id, ..) = create_ok_nda();
        let (access_request_id, expected_nda_request) = create_ok_nda_content_access_request(project_nda_id);

        assert_noop!(
            Deip::create_nda_content_access_request(
                Origin::signed(DEFAULT_ACCOUNT_ID), 
                access_request_id, 
                project_nda_id,
                expected_nda_request.encrypted_payload_hash, 
                expected_nda_request.encrypted_payload_iv
            ),
            Error::<Test>::NdaAccessRequestAlreadyExists
        );

    })
}


#[test]
fn fulfill_nda_content_access_request() {
    new_test_ext().execute_with(|| {
        let (project_nda_id, ..) = create_ok_nda();

        let (access_request_id, nda_request) = create_ok_nda_content_access_request(project_nda_id);

        let encrypted_payload_encryption_key = vec![1,3,4,2];
        let proof_of_encrypted_payload_encryption_key = vec![3,4,5,6];

        assert_ok!(
            Deip::fulfill_nda_content_access_request(
                Origin::signed(DEFAULT_ACCOUNT_ID), 
                access_request_id.clone(), 
                encrypted_payload_encryption_key.clone(), 
                proof_of_encrypted_payload_encryption_key.clone()
            )
        );

        let nda_stored = NdaAccessRequestMap::<Test>::get(access_request_id);

        let expected_nda_request = NdaAccessRequest {
            status: NdaAccessRequestStatus::Fulfilled,
            grantor: Some(DEFAULT_ACCOUNT_ID),
            encrypted_payload_encryption_key: Some(encrypted_payload_encryption_key),
            proof_of_encrypted_payload_encryption_key: Some(proof_of_encrypted_payload_encryption_key),
            ..nda_request
        };

        assert_eq!(expected_nda_request, nda_stored);

    })
}


#[test]
fn cant_fulfill_not_existed_nda_content_access_request() {
    new_test_ext().execute_with(|| {
        let access_request_id = NdaAccessRequestId::random();

        let encrypted_payload_encryption_key = vec![1,3,4,2];
        let proof_of_encrypted_payload_encryption_key = vec![3,4,5,6];

        assert_noop!(
            Deip::fulfill_nda_content_access_request(
                Origin::signed(DEFAULT_ACCOUNT_ID), 
                access_request_id.clone(), 
                encrypted_payload_encryption_key.clone(), 
                proof_of_encrypted_payload_encryption_key.clone()
            ),
            Error::<Test>::NoSuchNdaAccessRequest
        );

    })
}

#[test]
fn cant_fulfill_finalized_nda_content_access_request() {
    new_test_ext().execute_with(|| {
        let (project_nda_id, ..) = create_ok_nda();

        let (access_request_id, ..) = create_ok_nda_content_access_request(project_nda_id);

        let encrypted_payload_encryption_key = vec![1,3,4,2];
        let proof_of_encrypted_payload_encryption_key = vec![3,4,5,6];

        assert_ok!(
            Deip::fulfill_nda_content_access_request(
                Origin::signed(DEFAULT_ACCOUNT_ID), 
                access_request_id.clone(), 
                encrypted_payload_encryption_key.clone(), 
                proof_of_encrypted_payload_encryption_key.clone()
            )
        );

        assert_noop!(
            Deip::fulfill_nda_content_access_request(
                Origin::signed(DEFAULT_ACCOUNT_ID), 
                access_request_id.clone(), 
                encrypted_payload_encryption_key.clone(), 
                proof_of_encrypted_payload_encryption_key.clone()
            ),
            Error::<Test>::NdaAccessRequestAlreadyFinalized
        );

    })
}

#[test]
fn reject_nda_content_access_request() {
    new_test_ext().execute_with(|| {
        let (project_nda_id, ..) = create_ok_nda();

        let (access_request_id, nda_request) = create_ok_nda_content_access_request(project_nda_id);

        assert_ok!(
            Deip::reject_nda_content_access_request(
                Origin::signed(DEFAULT_ACCOUNT_ID), 
                access_request_id.clone(), 
            )
        );

        let nda_stored = NdaAccessRequestMap::<Test>::get(access_request_id);

        let expected_nda_request = NdaAccessRequest {
            status: NdaAccessRequestStatus::Rejected,
            ..nda_request
        };

        assert_eq!(expected_nda_request, nda_stored);

    })
}


#[test]
fn cant_reject_not_existed_nda_content_access_request() {
    new_test_ext().execute_with(|| {
        let access_request_id = NdaAccessRequestId::random();

        assert_noop!(
            Deip::reject_nda_content_access_request(
                Origin::signed(DEFAULT_ACCOUNT_ID), 
                access_request_id.clone(), 
            ),
            Error::<Test>::NoSuchNdaAccessRequest
        );

    })
}

#[test]
fn cant_reject_finalized_nda_content_access_request() {
    new_test_ext().execute_with(|| {
        let (project_nda_id, ..) = create_ok_nda();

        let (access_request_id, ..) = create_ok_nda_content_access_request(project_nda_id);

        assert_ok!(
            Deip::reject_nda_content_access_request(
                Origin::signed(DEFAULT_ACCOUNT_ID), 
                access_request_id.clone(), 
            )
        );

        assert_noop!(
            Deip::reject_nda_content_access_request(
                Origin::signed(DEFAULT_ACCOUNT_ID), 
                access_request_id.clone(), 
            ),
            Error::<Test>::NdaAccessRequestAlreadyFinalized
        );
    })
}

#[test]
fn simple_crowdfunding_create_should_fail() {
    new_test_ext2().execute_with(|| {
        let start_time = pallet_timestamp::Pallet::<Test>::get();
        assert_noop!(Deip::create_simple_crowdfunding(DEFAULT_ACCOUNT_ID,
            H160::random(),
            start_time,
            start_time + 1,
            DeipAsset::new(DeipAssetId(0u32), 100u32.into()),
            DeipAsset::new(DeipAssetId(0u32), 120u32.into()),
            vec![DeipAsset::new(DeipAssetId(0u32), 100u32.into()), DeipAsset::new(DeipAssetId(14u32), 200u32.into())]
        ),
        Error::<Test>::InvestmentOpportunityWrongAssetId);

        assert_noop!(Deip::create_simple_crowdfunding(DEFAULT_ACCOUNT_ID,
            H160::random(),
            start_time,
            start_time + 1,
            DeipAsset::new(DeipAssetId(0u32), 100u32.into()),
            DeipAsset::new(DeipAssetId(0u32), 120u32.into()),
            vec![]
        ),
        Error::<Test>::InvestmentOpportunitySecurityTokenNotSpecified);
    })
}

#[test]
fn simple_crowdfunding_hard_cap_reached() {
    let mut ext = new_test_ext2();
    let state = offchainify(&mut ext, 2);
    ext.execute_with(|| {
        let (ref project_id, .., ref account_id) = create_ok_project(None);

        let base_asset_id = DeipAssetId(3u32);
        let base_asset_total = 120_000u64;
        create_issue_asset(ALICE_ACCOUNT_ID, base_asset_id, base_asset_total, None);

        let call = pallet_deip_assets::Call::<Test>::transfer(base_asset_id, BOB_ACCOUNT_ID, base_asset_total / 2);
        let result = call.dispatch_bypass_filter(Origin::signed(ALICE_ACCOUNT_ID));
        assert_ok!(result);

        let usd_id = DeipAssetId(0u32);
        let usd_total = 100_000u64;
        create_issue_asset(*account_id, usd_id, usd_total, Some(*project_id));

        let eur_id = DeipAssetId(1u32);
        let eur_total = 80_000u64;
        create_issue_asset(*account_id, eur_id, eur_total, Some(*project_id));

        let balance_before = DeipAssets::account_balance(account_id, &base_asset_id);

        let start_time = pallet_timestamp::Pallet::<Test>::get();
        let sale_id = H160::random();
        let soft_cap = 100_000u64;
        let hard_cap = base_asset_total;
        let usd_to_sale = 80_000u64;
        let eur_to_sale = 75_000u64;
        assert_ok!(Deip::create_simple_crowdfunding(
            DEFAULT_ACCOUNT_ID,
            sale_id,
            start_time,
            start_time + 100,
            DeipAsset::new(base_asset_id, soft_cap),
            DeipAsset::new(base_asset_id, hard_cap),
            vec![DeipAsset::new(usd_id, usd_to_sale), DeipAsset::new(eur_id, eur_to_sale)]
        ));

        Deip::offchain_worker(System::block_number());
        assert_eq!(state.read().transactions.len(), 1);

        let inner = decode_validate_deip_call(&state.read().transactions[0]);
        match inner {
            crate::Call::activate_crowdfunding(id) => Deip::activate_crowdfunding_impl(id).unwrap(),
            _ => unreachable!(),
        };

        assert_ok!(Deip::invest_to_crowdfunding_impl(
            BOB_ACCOUNT_ID,
            sale_id,
            DeipAsset::new(base_asset_id, hard_cap / 2),
        ));

        // investors should get their tokens in any case
        let call = pallet_deip_assets::Call::<Test>::freeze(usd_id, BOB_ACCOUNT_ID);
        let _result = call.dispatch_bypass_filter(Origin::signed(*account_id));

        let call = pallet_deip_assets::Call::<Test>::freeze_asset(eur_id);
        let _result = call.dispatch_bypass_filter(Origin::signed(*account_id));

        assert_ok!(Deip::invest_to_crowdfunding_impl(
            ALICE_ACCOUNT_ID,
            sale_id,
            DeipAsset::new(base_asset_id, hard_cap / 2),
        ));

        assert_eq!(DeipAssets::account_balance(&BOB_ACCOUNT_ID, &usd_id), usd_to_sale / 2);
        assert_eq!(DeipAssets::account_balance(&ALICE_ACCOUNT_ID, &usd_id), usd_to_sale / 2);
        assert_eq!(DeipAssets::account_balance(account_id, &usd_id), DeipAssets::total_supply(&usd_id) - usd_to_sale);

        assert_eq!(DeipAssets::account_balance(&BOB_ACCOUNT_ID, &eur_id), eur_to_sale / 2);
        assert_eq!(DeipAssets::account_balance(&ALICE_ACCOUNT_ID, &eur_id), eur_to_sale / 2);
        assert_eq!(DeipAssets::account_balance(account_id, &eur_id), DeipAssets::total_supply(&eur_id) - eur_to_sale);

        assert_eq!(DeipAssets::account_balance(account_id, &base_asset_id), hard_cap + balance_before);
    })
}

#[test]
fn simple_crowdfunding_expired() {
    let mut ext = new_test_ext2();
    let state = offchainify(&mut ext, 2);
    ext.execute_with(|| {
        let (ref project_id, .., ref account_id) = create_ok_project(None);

        let base_asset_id = DeipAssetId(3u32);
        let base_asset_total = 120_000u64;
        create_issue_asset(*account_id, base_asset_id, base_asset_total, Some(*project_id));

        let call = pallet_deip_assets::Call::<Test>::transfer(base_asset_id, ALICE_ACCOUNT_ID, base_asset_total / 2);
        let result = call.dispatch_bypass_filter(Origin::signed(*account_id));
        assert_ok!(result);

        let call = pallet_deip_assets::Call::<Test>::transfer(base_asset_id, BOB_ACCOUNT_ID, base_asset_total / 2);
        let result = call.dispatch_bypass_filter(Origin::signed(*account_id));
        assert_ok!(result);

        let usd_id = DeipAssetId(0u32);
        let usd_total = 100_000u64;
        create_issue_asset(*account_id, usd_id, usd_total, Some(*project_id));

        let eur_id = DeipAssetId(1u32);
        let eur_total = 80_000u64;
        create_issue_asset(*account_id, eur_id, eur_total, Some(*project_id));

        let balance_before = DeipAssets::account_balance(account_id, &base_asset_id);
        let bob_balance_before = DeipAssets::account_balance(&BOB_ACCOUNT_ID, &base_asset_id);

        let start_time_in_blocks = 5;
        let start_time = pallet_timestamp::Pallet::<Test>::get() + start_time_in_blocks * BLOCK_TIME;
        let sale_id = H160::random();
        let soft_cap = 100_000u64;
        let alice_investing = soft_cap / 2;
        let hard_cap = base_asset_total;
        let usd_to_sale = 80_000u64;
        let eur_to_sale = 75_000u64;
        let duration_in_blocks = 5;
        assert_ok!(Deip::create_simple_crowdfunding(
            DEFAULT_ACCOUNT_ID,
            sale_id,
            start_time,
            start_time + duration_in_blocks * BLOCK_TIME,
            DeipAsset::new(base_asset_id, soft_cap),
            DeipAsset::new(base_asset_id, hard_cap),
            vec![DeipAsset::new(usd_id, usd_to_sale), DeipAsset::new(eur_id, eur_to_sale)]
        ));

        let start_block = System::block_number() + start_time_in_blocks + 1;
        while System::block_number() < start_block {
            let block_number = System::block_number();
            <System as OnFinalize<BlockNumber>>::on_finalize(block_number);
            Deip::offchain_worker(System::block_number());
            System::set_block_number(block_number + 1);
            <System as OnInitialize<BlockNumber>>::on_initialize(System::block_number());
            Timestamp::set_timestamp(System::block_number() * BLOCK_TIME + INIT_TIMESTAMP);
        }

        assert_eq!(state.read().transactions.len(), 1);

        let inner = decode_validate_deip_call(&state.read().transactions[0]);
        match inner {
            crate::Call::activate_crowdfunding(id) => Deip::activate_crowdfunding_impl(id).unwrap(),
            _ => unreachable!(),
        };

        state.write().transactions.clear();

        assert_ok!(Deip::invest_to_crowdfunding_impl(
            BOB_ACCOUNT_ID,
            sale_id,
            DeipAsset::new(base_asset_id, soft_cap / 4),
        ));

        assert_ok!(Deip::invest_to_crowdfunding_impl(
            ALICE_ACCOUNT_ID,
            sale_id,
            DeipAsset::new(base_asset_id, alice_investing),
        ));

        // make alice zombie
        let alice_remainder = DeipAssets::account_balance(&ALICE_ACCOUNT_ID, &base_asset_id);
        let call = pallet_deip_assets::Call::<Test>::transfer(base_asset_id, BOB_ACCOUNT_ID, alice_remainder);
        let result = call.dispatch_bypass_filter(Origin::signed(ALICE_ACCOUNT_ID));
        assert_ok!(result);

        let _a = Balances::slash(&ALICE_ACCOUNT_ID, Balances::total_balance(&ALICE_ACCOUNT_ID));

        // since the sale expired the tokens should be transfered back to
        // the seller doesn't matter if assets/accounts frozen or not
        let call = pallet_deip_assets::Call::<Test>::freeze(eur_id, BOB_ACCOUNT_ID);
        let _result = call.dispatch_bypass_filter(Origin::signed(*account_id));

        let call = pallet_deip_assets::Call::<Test>::freeze_asset(usd_id);
        let _result = call.dispatch_bypass_filter(Origin::signed(*account_id));

        let end_block = start_block + duration_in_blocks + 1;
        while System::block_number() < end_block {
            let block_number = System::block_number();
            <System as OnFinalize<BlockNumber>>::on_finalize(block_number);
            Deip::offchain_worker(System::block_number());
            System::set_block_number(block_number + 1);
            <System as OnInitialize<BlockNumber>>::on_initialize(System::block_number());
            Timestamp::set_timestamp(System::block_number() * BLOCK_TIME + INIT_TIMESTAMP);
        }

        let inner = decode_validate_deip_call(&state.read().transactions[0]);
        match inner {
            crate::Call::expire_crowdfunding(id) => Deip::expire_crowdfunding_impl(id).unwrap(),
            _ => unreachable!(),
        };

        assert_eq!(DeipAssets::account_balance(&BOB_ACCOUNT_ID, &base_asset_id), bob_balance_before + alice_remainder);
        assert_eq!(DeipAssets::account_balance(&ALICE_ACCOUNT_ID, &base_asset_id), alice_investing);
        assert_eq!(DeipAssets::account_balance(account_id, &base_asset_id), balance_before);

        assert_eq!(DeipAssets::account_balance(&BOB_ACCOUNT_ID, &usd_id), 0);
        assert_eq!(DeipAssets::account_balance(&ALICE_ACCOUNT_ID, &usd_id), 0);
        assert_eq!(DeipAssets::account_balance(account_id, &usd_id), usd_total);

        assert_eq!(DeipAssets::account_balance(&BOB_ACCOUNT_ID, &eur_id), 0);
        assert_eq!(DeipAssets::account_balance(&ALICE_ACCOUNT_ID, &eur_id), 0);
        assert_eq!(DeipAssets::account_balance(account_id, &eur_id), eur_total);
    })
}

#[test]
fn two_simultaneous_crowdfundings_expired() {
    let mut ext = new_test_ext2();
    let state = offchainify(&mut ext, 2);
    ext.execute_with(|| {
        let base_asset_id = DeipAssetId(3u32);
        let base_asset_total = 120_000u64;
        create_issue_asset(DEFAULT_ACCOUNT_ID, base_asset_id, base_asset_total, None);

        let call = pallet_deip_assets::Call::<Test>::transfer(base_asset_id, ALICE_ACCOUNT_ID, base_asset_total / 2);
        let result = call.dispatch_bypass_filter(Origin::signed(DEFAULT_ACCOUNT_ID));
        assert_ok!(result);

        let call = pallet_deip_assets::Call::<Test>::transfer(base_asset_id, BOB_ACCOUNT_ID, base_asset_total / 2);
        let result = call.dispatch_bypass_filter(Origin::signed(DEFAULT_ACCOUNT_ID));
        assert_ok!(result);

        let usd_id = DeipAssetId(0u32);
        let usd_total = 100_000u64;
        create_issue_asset(ALICE_ACCOUNT_ID, usd_id, usd_total, None);

        let eur_id = DeipAssetId(1u32);
        let eur_total = 80_000u64;
        create_issue_asset(BOB_ACCOUNT_ID, eur_id, eur_total, None);

        let bob_usd_balance_before = DeipAssets::account_balance(&BOB_ACCOUNT_ID, &usd_id);
        let bob_eur_balance_before = DeipAssets::account_balance(&BOB_ACCOUNT_ID, &eur_id);

        let alice_usd_balance_before = DeipAssets::account_balance(&ALICE_ACCOUNT_ID, &usd_id);
        let alice_base_balance_before = DeipAssets::account_balance(&ALICE_ACCOUNT_ID, &base_asset_id);

        let start_time_in_blocks = 5;
        let start_time = pallet_timestamp::Pallet::<Test>::get() + start_time_in_blocks * BLOCK_TIME;
        let eur_sale_id = H160::random();
        let usd_sale_id = H160::random();
        let soft_cap = 50_000u64;
        let hard_cap = 60_000_u64;
        let usd_to_sale = 70_000u64;
        let eur_to_sale = 75_000u64;
        let duration_in_blocks = 5;
        assert_ok!(Deip::create_simple_crowdfunding(
            BOB_ACCOUNT_ID,
            eur_sale_id,
            start_time,
            start_time + duration_in_blocks * BLOCK_TIME,
            DeipAsset::new(usd_id, soft_cap),
            DeipAsset::new(usd_id, hard_cap),
            vec![DeipAsset::new(eur_id, eur_to_sale)]
        ));

        assert_ok!(Deip::create_simple_crowdfunding(
            ALICE_ACCOUNT_ID,
            usd_sale_id,
            start_time,
            start_time + duration_in_blocks * BLOCK_TIME,
            DeipAsset::new(base_asset_id, soft_cap),
            DeipAsset::new(base_asset_id, hard_cap),
            vec![DeipAsset::new(usd_id, usd_to_sale)]
        ));

        let start_block = System::block_number() + start_time_in_blocks + 1;
        while System::block_number() < start_block {
            let block_number = System::block_number();
            <System as OnFinalize<BlockNumber>>::on_finalize(block_number);
            Deip::offchain_worker(System::block_number());
            System::set_block_number(block_number + 1);
            <System as OnInitialize<BlockNumber>>::on_initialize(System::block_number());
            Timestamp::set_timestamp(System::block_number() * BLOCK_TIME + INIT_TIMESTAMP);
        }

        assert_eq!(state.read().transactions.len(), 2);

        let inner = decode_validate_deip_call(&state.read().transactions[0]);
        match inner {
            crate::Call::activate_crowdfunding(id) => Deip::activate_crowdfunding_impl(id).unwrap(),
            _ => unreachable!(),
        };

        let inner = decode_validate_deip_call(&state.read().transactions[1]);
        match inner {
            crate::Call::activate_crowdfunding(id) => Deip::activate_crowdfunding_impl(id).unwrap(),
            _ => unreachable!(),
        };

        state.write().transactions.clear();

        assert_ok!(Deip::invest_to_crowdfunding_impl(
            BOB_ACCOUNT_ID,
            usd_sale_id,
            DeipAsset::new(base_asset_id, soft_cap / 4),
        ));

        assert_ok!(Deip::invest_to_crowdfunding_impl(
            ALICE_ACCOUNT_ID,
            eur_sale_id,
            DeipAsset::new(usd_id, soft_cap / 2),
        ));

        // since the sale expired the tokens should be transfered back to
        // the seller doesn't matter if assets/accounts frozen or not
        let call = pallet_deip_assets::Call::<Test>::freeze(eur_id, BOB_ACCOUNT_ID);
        let _result = call.dispatch_bypass_filter(Origin::signed(BOB_ACCOUNT_ID));

        let call = pallet_deip_assets::Call::<Test>::freeze_asset(usd_id);
        let _result = call.dispatch_bypass_filter(Origin::signed(ALICE_ACCOUNT_ID));

        let end_block = start_block + duration_in_blocks + 1;
        while System::block_number() < end_block {
            let block_number = System::block_number();
            <System as OnFinalize<BlockNumber>>::on_finalize(block_number);
            Deip::offchain_worker(System::block_number());
            System::set_block_number(block_number + 1);
            <System as OnInitialize<BlockNumber>>::on_initialize(System::block_number());
            Timestamp::set_timestamp(System::block_number() * BLOCK_TIME + INIT_TIMESTAMP);
        }

        let inner = decode_validate_deip_call(&state.read().transactions[0]);
        match inner {
            crate::Call::expire_crowdfunding(id) => Deip::expire_crowdfunding_impl(id).unwrap(),
            _ => unreachable!(),
        };

        let inner = decode_validate_deip_call(&state.read().transactions[1]);
        match inner {
            crate::Call::expire_crowdfunding(id) => Deip::expire_crowdfunding_impl(id).unwrap(),
            _ => unreachable!(),
        };

        assert_eq!(DeipAssets::account_balance(&ALICE_ACCOUNT_ID, &base_asset_id), alice_base_balance_before);

        assert_eq!(DeipAssets::account_balance(&BOB_ACCOUNT_ID, &usd_id), bob_usd_balance_before);
        assert_eq!(DeipAssets::account_balance(&ALICE_ACCOUNT_ID, &usd_id), alice_usd_balance_before);

        assert_eq!(DeipAssets::account_balance(&BOB_ACCOUNT_ID, &eur_id), bob_eur_balance_before);
    })
}

#[test]
fn create_license_agreement_well_known_cases() {
    new_test_ext2().execute_with(|| {
        assert_noop!(Deip::create_contract_agreement_impl(
            BOB_ACCOUNT_ID,
            ContractAgreementId::random(),
            BOB_ACCOUNT_ID,
            vec![BOB_ACCOUNT_ID.into(), DEFAULT_ACCOUNT_ID.into()],
            HashOf::<Test>::random(),
            None,
            None,
            ContractAgreementTermsOf::<Test>::LicenseAgreement{
                source: ProjectId::random(),
                price: DeipAsset::new(Default::default(), One::one()),
            }),
            Error::<Test>::NoSuchProject);

        let (ref project_id, .., ref account_id) = create_ok_project(Some(ALICE_ACCOUNT_ID));
        assert_noop!(Deip::create_contract_agreement_impl(
            BOB_ACCOUNT_ID,
            ContractAgreementId::random(),
            BOB_ACCOUNT_ID,
            vec![BOB_ACCOUNT_ID.into(), DEFAULT_ACCOUNT_ID.into()],
            HashOf::<Test>::random(),
            None,
            None,
            ContractAgreementTermsOf::<Test>::LicenseAgreement{
                source: *project_id,
                price: DeipAsset::new(Default::default(), One::one()),
            }),
            Error::<Test>::ContractAgreementLicenseProjectTeamIsNotListedInParties);

        assert_noop!(Deip::create_contract_agreement_impl(
            *account_id,
            ContractAgreementId::random(),
            *account_id,
            vec![BOB_ACCOUNT_ID.into()],
            HashOf::<Test>::random(),
            None,
            None,
            ContractAgreementTermsOf::<Test>::LicenseAgreement{
                source: *project_id,
                price: DeipAsset::new(Default::default(), Zero::zero()),
            }),
            Error::<Test>::ContractAgreementFeeMustBePositive);

        assert_noop!(Deip::create_contract_agreement_impl(
            *account_id,
            ContractAgreementId::random(),
            *account_id,
            vec![BOB_ACCOUNT_ID.into()],
            HashOf::<Test>::random(),
            Some(pallet_timestamp::Pallet::<Test>::get()),
            Some(pallet_timestamp::Pallet::<Test>::get()),
            ContractAgreementTermsOf::<Test>::LicenseAgreement{
                source: *project_id,
                price: DeipAsset::new(Default::default(), Zero::zero()),
            }),
            Error::<Test>::ContractAgreementEndTimeMustBeLaterStartTime);

        assert_noop!(Deip::create_contract_agreement_impl(
            *account_id,
            ContractAgreementId::random(),
            *account_id,
            vec![BOB_ACCOUNT_ID.into()],
            HashOf::<Test>::random(),
            None,
            Some(pallet_timestamp::Pallet::<Test>::get()),
            ContractAgreementTermsOf::<Test>::LicenseAgreement{
                source: *project_id,
                price: DeipAsset::new(Default::default(), Zero::zero()),
            }),
            Error::<Test>::ContractAgreementEndTimeMustBeLaterStartTime);

        let license_id = ContractAgreementId::random();
        assert!(Deip::create_contract_agreement_impl(
            *account_id,
            license_id,
            *account_id,
            vec![BOB_ACCOUNT_ID.into(), account_id.clone().into()],
            HashOf::<Test>::random(),
            None,
            None,
            ContractAgreementTermsOf::<Test>::LicenseAgreement{
                source: *project_id,
                price: DeipAsset::new(Default::default(), One::one()),
            }).is_ok());

        assert_noop!(Deip::create_contract_agreement_impl(
            *account_id,
            license_id,
            *account_id,
            vec![BOB_ACCOUNT_ID.into(), account_id.clone().into()],
            HashOf::<Test>::random(),
            None,
            None,
            ContractAgreementTermsOf::<Test>::LicenseAgreement{
                source: *project_id,
                price: DeipAsset::new(Default::default(), One::one()),
            }),
            Error::<Test>::ContractAgreementAlreadyExists);
    })
}
