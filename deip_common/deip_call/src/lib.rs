mod assets_call_args;

use codec::{Decode, Encode};
use frame_support::Parameter;
use node_runtime::{Call, Runtime};
use serde::{ser::Serializer, Deserialize, Serialize};
use sp_runtime::traits::{AtLeast32BitUnsigned, Member};
use sp_std::borrow::Borrow;

use pallet_deip_proposal::proposal::{BatchItem, InputProposalBatch};

use deip_serializable_u128::SerializableAtLeast32BitUnsigned;

use crate::assets_call_args::{
    AssetsApproveTransferCallArgs, AssetsBurnCallArgs, AssetsCancelApprovalCallArgs,
    AssetsClearMetadataCallArgs, AssetsCreateCallArgs, AssetsDestroyCallArgs,
    AssetsForceAssetStatusCallArgs, AssetsForceCancelApprovalCallArgs,
    AssetsForceClearMetadataCallArgs, AssetsForceCreateCallArgs, AssetsForceSetMetadataCallArgs,
    AssetsForceTransferCallArgs, AssetsFreezeAssetCallArgs, AssetsFreezeCallArgs,
    AssetsMintCallArgs, AssetsSetMetadataCallArgs, AssetsSetTeamCallArgs, AssetsThawCallArgs,
    AssetsTransferApprovedCallArgs, AssetsTransferCallArgs, AssetsTransferKeepAliveCallArgs,
    AssetsTransferOwnershipCallArgs,
};

#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize)]
pub struct WrappedCall<Call: Parameter + Member>(pub Call);

impl<Call: Parameter + Member> WrappedCall<Call> {
    pub fn wrap(call: &Call) -> Self {
        WrappedCall(call.clone())
    }
}

pub fn wrap_input_batch(
    batch: &InputProposalBatch<Runtime>,
) -> Vec<
    BatchItem<
        node_runtime::deip_account::DeipAccountId<node_runtime::AccountId>,
        WrappedCall<Call>,
    >,
> {
    batch
        .iter()
        .map(|x| BatchItem { account: x.account.clone(), call: WrappedCall::wrap(&x.call) })
        .collect()
}

impl Serialize for WrappedCall<Call> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            Call::Deip(deip_call) => Self::serialize_deip_call(deip_call, serializer),

            Call::DeipProposal(deip_proposal_call) =>
                Self::serialize_deip_proposal_call(deip_proposal_call, serializer),

            Call::DeipDao(deip_dao_call) =>
                Self::serialize_deip_dao_call(deip_dao_call, serializer),

            Call::Assets(deip_assets_call) =>
                Self::serialize_deip_assets_call(deip_assets_call, serializer),

            Call::System(_) |
            Call::DeipPortal(_) |
            Call::Timestamp(_) |
            Call::Grandpa(_) |
            Call::Balances(_) |
            Call::Sudo(_) |
            Call::TemplateModule(_) |
            Call::Babe(_) |
            Call::Authorship(_) |
            Call::OctopusAppchain(_) |
            Call::OctopusLpos(_) |
            Call::OctopusUpwardMessages(_) |
            Call::Session(_) |
            Call::ImOnline(_) |
            Call::Utility(_) |
            Call::Multisig(_) => CallObject {
                module: "unsupported_module",
                call: "unsupported_call",
                args: &UnsupportedCallArgs {},
            }
            .serialize(serializer),
        }
    }
}

impl WrappedCall<Call> {
    fn serialize_deip_call<S>(
        deip_call: &pallet_deip::Call<Runtime>,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        use pallet_deip::Call::*;

        match deip_call {
            create_project(is_private, external_id, team_id, description, domains) => CallObject {
                module: "deip",
                call: "create_project",
                args: &DeipCreateProjectCallArgs {
                    is_private,
                    external_id,
                    team_id,
                    description,
                    domains,
                },
            }
            .serialize(serializer),

            create_investment_opportunity(external_id, creator, shares, funding_model) =>
                CallObject {
                    module: "deip",
                    call: "create_investment_opportunity",
                    args: &DeipCreateInvestmentOpportunityCallArgs {
                        external_id,
                        creator,
                        shares,
                        funding_model,
                    },
                }
                .serialize(serializer),

            activate_crowdfunding(sale_id) => CallObject {
                module: "deip",
                call: "activate_crowdfunding",
                args: &DeipActivateCrowdfundingCallArgs { sale_id },
            }
            .serialize(serializer),

            expire_crowdfunding(sale_id) => CallObject {
                module: "deip",
                call: "expire_crowdfunding",
                args: &DeipExpireCrowdfundingCallArgs { sale_id },
            }
            .serialize(serializer),

            finish_crowdfunding(sale_id) => CallObject {
                module: "deip",
                call: "finish_crowdfunding",
                args: &DeipFinishCrowdfundingCallArgs { sale_id },
            }
            .serialize(serializer),

            invest(id, amount) => CallObject {
                module: "deip",
                call: "invest",
                args: &DeipInvestCallArgs { id, amount },
            }
            .serialize(serializer),

            update_project(project_id, description, is_private) => CallObject {
                module: "deip",
                call: "update_project",
                args: &DeipUpdateProjectCallArgs { project_id, description, is_private },
            }
            .serialize(serializer),

            create_project_content(
                external_id,
                project_external_id,
                team_id,
                content_type,
                description,
                content,
                authors,
                references,
            ) => CallObject {
                module: "deip",
                call: "create_project_content",
                args: &DeipCreateProjectContentCallArgs {
                    external_id,
                    project_external_id,
                    team_id,
                    content_type,
                    description,
                    content,
                    authors,
                    references,
                },
            }
            .serialize(serializer),

            create_project_nda(
                external_id,
                end_date,
                contract_hash,
                maybe_start_date,
                parties,
                projects,
            ) => CallObject {
                module: "deip",
                call: "create_project_nda",
                args: &DeipCreateProjectNdaCallArgs {
                    external_id,
                    end_date,
                    contract_hash,
                    maybe_start_date,
                    parties,
                    projects,
                },
            }
            .serialize(serializer),

            create_nda_content_access_request(
                external_id,
                nda_external_id,
                encrypted_payload_hash,
                encrypted_payload_iv,
            ) => CallObject {
                module: "deip",
                call: "create_nda_content_access_request",
                args: &DeipCreateProjectNdaAccessRequestCallArgs {
                    external_id,
                    nda_external_id,
                    encrypted_payload_hash,
                    encrypted_payload_iv,
                },
            }
            .serialize(serializer),

            fulfill_nda_content_access_request(
                external_id,
                encrypted_payload_encryption_key,
                proof_of_encrypted_payload_encryption_key,
            ) => CallObject {
                module: "deip",
                call: "fulfill_nda_content_access_request",
                args: &DeipFulfillNdaAccessRequestCallArgs {
                    external_id,
                    encrypted_payload_encryption_key,
                    proof_of_encrypted_payload_encryption_key,
                },
            }
            .serialize(serializer),

            reject_nda_content_access_request(external_id) => CallObject {
                module: "deip",
                call: "reject_nda_content_access_request",
                args: &DeipRejectNdaAccessRequestCallArgs { external_id },
            }
            .serialize(serializer),

            create_review(
                external_id,
                author,
                content,
                domains,
                assessment_model,
                weight,
                project_content_external_id,
            ) => CallObject {
                module: "deip",
                call: "create_review",
                args: &DeipCreateReviewCallArgs {
                    external_id,
                    author,
                    content,
                    domains,
                    assessment_model,
                    weight,
                    project_content_external_id,
                },
            }
            .serialize(serializer),

            upvote_review(review_id, domain_id) => CallObject {
                module: "deip",
                call: "upvote_review",
                args: &DeipUpvoteReviewCallArgs { review_id, domain_id },
            }
            .serialize(serializer),

            add_domain(domain) => CallObject {
                module: "deip",
                call: "add_domain",
                args: &DeipAddDomainCallArgs { domain },
            }
            .serialize(serializer),

            create_contract_agreement(
                id,
                creator,
                parties,
                hash,
                activation_time,
                expiration_time,
                terms,
            ) => CallObject {
                module: "deip",
                call: "create_contract_agreement",
                args: &DeipCreateContractAgreementCallArgs {
                    id,
                    creator,
                    parties,
                    hash,
                    activation_time,
                    expiration_time,
                    terms,
                },
            }
            .serialize(serializer),

            accept_contract_agreement(id, party) => CallObject {
                module: "deip",
                call: "accept_contract_agreement",
                args: &DeipAcceptContractAgreementCallArgs { id, party },
            }
            .serialize(serializer),

            reject_contract_agreement(id, party) => CallObject {
                module: "deip",
                call: "reject_contract_agreement",
                args: &DeipRejectContractAgreementCallArgs { id, party },
            }
            .serialize(serializer),

            __PhantomItem(..) => unreachable!(),
        }
    }

    fn serialize_deip_proposal_call<S>(
        deip_proposal_call: &pallet_deip_proposal::Call<Runtime>,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        use pallet_deip_proposal::Call::*;

        match deip_proposal_call {
            propose(batch, external_id) => CallObject {
                module: "deip_proposal",
                call: "propose",
                args: &DeipProposalProposeCallArgs { batch: &wrap_input_batch(batch), external_id },
            }
            .serialize(serializer),

            decide(proposal_id, decision, _) => CallObject {
                module: "deip_proposal",
                call: "decide",
                args: &DeipProposalDecideCallArgs { proposal_id, decision },
            }
            .serialize(serializer),

            expire(proposal_id) => CallObject {
                module: "deip_proposal",
                call: "expire",
                args: &DeipProposalExpireCallArgs { proposal_id },
            }
            .serialize(serializer),

            __Ignore(..) => unreachable!(),
        }
    }

    fn serialize_deip_dao_call<S>(
        deip_dao_call: &pallet_deip_dao::Call<Runtime>,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        use pallet_deip_dao::Call::*;

        match deip_dao_call {
            create(name, authority, metadata) => CallObject {
                module: "deip_dao",
                call: "create",
                args: &DeipDaoCreateCallArgs { name, authority, metadata },
            }
            .serialize(serializer),

            alter_authority(alter_authority_) => CallObject {
                module: "deip_dao",
                call: "alter_authority",
                args: &DeipDaoAlterAuthorityCallArgs { alter_authority: alter_authority_ },
            }
            .serialize(serializer),

            update_dao(metadata) => CallObject {
                module: "deip_dao",
                call: "update_dao",
                args: &DeipDaoUpdateCallArgs { metadata },
            }
            .serialize(serializer),

            on_behalf(name, call) => CallObject {
                module: "deip_dao",
                call: "on_behalf",
                args: &DeipDaoOnBehalfCallArgs { name, call: &WrappedCall::wrap(call.borrow()) },
            }
            .serialize(serializer),

            __Ignore(..) => unreachable!(),
        }
    }

    fn serialize_deip_assets_call<S>(
        deip_assets_call: &pallet_deip_assets::Call<Runtime>,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        use pallet_deip_assets::Call::*;

        let module = "deip_assets";
        match deip_assets_call {
            // pallet_assets::Call::create
            create(id, admin, min_balance) => {
                let call = "create";
                let args = AssetsCreateCallArgs::new(*id, admin, *min_balance);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::force_create
            force_create(id, owner, is_suff, min_balance) => {
                let call = "force_create";
                let args = AssetsForceCreateCallArgs::new(*id, owner, *is_suff, *min_balance);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::destroy
            destroy(id, _witness) => {
                // todo!("find a way to serialize witness")
                let call = "destroy";
                let args = AssetsDestroyCallArgs::new(*id);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::mint
            mint(id, beneficiary, amount) => {
                let call = "mint";
                let args = AssetsMintCallArgs::new(*id, beneficiary, *amount);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::burn
            burn(id, who, amount) => {
                let call = "burn";
                let args = AssetsBurnCallArgs::new(*id, who, *amount);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::transfer
            transfer(id, target, amount) => {
                let call = "transfer";
                let args = AssetsTransferCallArgs::new(*id, target, *amount);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::transfer_keep_alive
            transfer_keep_alive(id, target, amount) => {
                let call = "transfer_keep_alive";
                let args = AssetsTransferKeepAliveCallArgs::new(*id, target, *amount);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::force_transfer
            force_transfer(id, source, dest, amount) => {
                let call = "force_transfer";
                let args = AssetsForceTransferCallArgs::new(*id, source, dest, *amount);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::freeze
            freeze(id, who) => {
                let call = "freeze";
                let args = AssetsFreezeCallArgs::new(*id, who);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::thaw
            thaw(id, who) => {
                let call = "thaw";
                let args = AssetsThawCallArgs::new(*id, who);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::freeze_asset
            freeze_asset(id) => {
                let call = "freeze_asset";
                let args = AssetsFreezeAssetCallArgs::new(*id);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::thaw_asset
            thaw_asset(id) => {
                let call = "thaw_asset";
                let args = AssetsFreezeAssetCallArgs::new(*id);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::transfer_ownership
            transfer_ownership(id, owner) => {
                let call = "transfer_ownership";
                let args = AssetsTransferOwnershipCallArgs::new(*id, owner);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::set_team
            set_team(id, issuer, admin, freezer) => {
                let call = "set_team";
                let args = AssetsSetTeamCallArgs::new(*id, issuer, admin, freezer);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::set_metadata
            set_metadata(id, name, symbol, decimals) => {
                let call = "set_metadata";
                let args = AssetsSetMetadataCallArgs::new(*id, name, symbol, *decimals);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::clear_metadata
            clear_metadata(id) => {
                let call = "clear_metadata";
                let args = AssetsClearMetadataCallArgs::new(*id);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::force_set_metadata
            force_set_metadata(id, name, symbol, decimals, is_frozen) => {
                let call = "force_set_metadata";
                let args =
                    AssetsForceSetMetadataCallArgs::new(*id, name, symbol, *decimals, *is_frozen);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::force_clear_metadata
            force_clear_metadata(id) => {
                let call = "force_clear_metadata";
                let args = AssetsForceClearMetadataCallArgs::new(*id);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::force_asset_status
            force_asset_status(
                id,
                owner,
                issuer,
                admin,
                freezer,
                min_balance,
                is_sufficient,
                is_frozen,
            ) => {
                let call = "force_asset_status";
                let args = AssetsForceAssetStatusCallArgs::new(
                    *id,
                    owner,
                    issuer,
                    admin,
                    freezer,
                    *min_balance,
                    *is_sufficient,
                    *is_frozen,
                );
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::approve_transfer
            approve_transfer(id, delegate, amount) => {
                let call = "approve_transfer";
                let args = AssetsApproveTransferCallArgs::new(*id, delegate, *amount);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::cancel_approval
            cancel_approval(id, delegate) => {
                let call = "cancel_approval";
                let args = AssetsCancelApprovalCallArgs::new(*id, delegate);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::force_cancel_approval
            force_cancel_approval(id, owner, delegate) => {
                let call = "force_cancel_approval";
                let args = AssetsForceCancelApprovalCallArgs::new(*id, owner, delegate);
                CallObject::new(module, call, args).serialize(serializer)
            },

            // pallet_assets::Call::transfer_approved
            transfer_approved(id, owner, destination, amount) => {
                let call = "transfer_approved";
                let args = AssetsTransferApprovedCallArgs::new(*id, owner, destination, *amount);
                CallObject::new(module, call, args).serialize(serializer)
            },

            deip_create_asset(id, admin, min_balance, project_id) => CallObject {
                module,
                call: "deip_create_asset",
                args: &DeipAssetsCreateAssetCallArgs::new(id, admin, min_balance, project_id),
            }
            .serialize(serializer),

            deip_destroy(id, _witness) => CallObject {
                module,
                call: "deip_destroy",
                args: &DeipAssetsDestroyCallArgs { id, witness: () },
            }
            .serialize(serializer),

            deip_issue_asset(id, beneficiary, amount) => CallObject {
                module,
                call: "deip_issue_asset",
                args: &DeipAssetsIssueAssetCallArgs::new(id, beneficiary, amount),
            }
            .serialize(serializer),

            deip_burn(id, who, amount) => CallObject {
                module,
                call: "deip_burn",
                args: &DeipAssetsBurnCallArgs::new(id, who, amount),
            }
            .serialize(serializer),

            deip_transfer(id, target, amount) => CallObject {
                module,
                call: "deip_transfer",
                args: &DeipAssetsTransferCallArgs::new(id, target, amount),
            }
            .serialize(serializer),

            deip_freeze(id, who) => CallObject {
                module,
                call: "deip_freeze",
                args: &DeipAssetsFreezeCallArgs { id, who },
            }
            .serialize(serializer),

            deip_thaw(id, who) =>
                CallObject { module, call: "deip_thaw", args: &DeipAssetsThawCallArgs { id, who } }
                    .serialize(serializer),

            deip_freeze_asset(id) => CallObject {
                module,
                call: "deip_freeze_asset",
                args: &DeipAssetsFreezeAssetCallArgs { id },
            }
            .serialize(serializer),

            deip_thaw_asset(id) => CallObject {
                module,
                call: "deip_thaw_asset",
                args: &DeipAssetsThawAssetCallArgs { id },
            }
            .serialize(serializer),

            deip_transfer_ownership(id, owner) => CallObject {
                module,
                call: "deip_transfer_ownership",
                args: &DeipAssetsTransferOwnershipCallArgs { id, owner },
            }
            .serialize(serializer),

            deip_set_team(id, issuer, admin, freezer) => CallObject {
                module,
                call: "deip_set_team",
                args: &DeipAssetsSetTeamCallArgs { id, issuer, admin, freezer },
            }
            .serialize(serializer),

            deip_set_metadata(id, name, symbol, decimals) => CallObject {
                module,
                call: "deip_set_metadata",
                args: &DeipAssetsSetMetadataCallArgs { id, name, symbol, decimals },
            }
            .serialize(serializer),

            deip_wipe_zero_balance(asset, account) => CallObject {
                module,
                call: "deip_wipe_zero_balance",
                args: &DeipAssetsWipeZeroBalanceCallArgs { asset, account },
            }
            .serialize(serializer),

            __Ignore(..) => unreachable!(),
        }
    }
}

#[derive(Serialize)]
struct UnsupportedCallArgs {}

#[derive(Serialize)]
struct DeipAssetsSetMetadataCallArgs<A, B, C, D> {
    id: A,
    name: B,
    symbol: C,
    decimals: D,
}

#[derive(Serialize)]
struct DeipAssetsWipeZeroBalanceCallArgs<A, B> {
    asset: A,
    account: B,
}

#[derive(Serialize)]
struct DeipAssetsSetTeamCallArgs<A, B, C, D> {
    id: A,
    issuer: B,
    admin: C,
    freezer: D,
}

#[derive(Serialize)]
struct DeipAssetsTransferOwnershipCallArgs<A, B> {
    id: A,
    owner: B,
}

#[derive(Serialize)]
struct DeipAssetsThawAssetCallArgs<A> {
    id: A,
}

#[derive(Serialize)]
struct DeipAssetsFreezeAssetCallArgs<A> {
    id: A,
}

#[derive(Serialize)]
struct DeipAssetsThawCallArgs<A, B> {
    id: A,
    who: B,
}

#[derive(Serialize)]
struct DeipAssetsFreezeCallArgs<A, B> {
    id: A,
    who: B,
}

#[derive(Serialize)]
struct DeipAssetsTransferCallArgs<A, B, C: Clone + AtLeast32BitUnsigned> {
    id: A,
    target: B,
    amount: SerializableAtLeast32BitUnsigned<C>,
}

impl<A, B, C: Clone + AtLeast32BitUnsigned> DeipAssetsTransferCallArgs<A, B, C> {
    fn new(id: A, target: B, amount: &C) -> Self {
        Self { id, target, amount: SerializableAtLeast32BitUnsigned(amount.clone()) }
    }
}

#[derive(Serialize)]
struct DeipAssetsBurnCallArgs<A, B, C: Clone + AtLeast32BitUnsigned> {
    id: A,
    who: B,
    amount: SerializableAtLeast32BitUnsigned<C>,
}

impl<A, B, C: Clone + AtLeast32BitUnsigned> DeipAssetsBurnCallArgs<A, B, C> {
    fn new(id: A, who: B, amount: &C) -> Self {
        Self { id, who, amount: SerializableAtLeast32BitUnsigned(amount.clone()) }
    }
}

#[derive(Serialize)]
struct DeipAssetsIssueAssetCallArgs<A, B, C: Clone + AtLeast32BitUnsigned> {
    id: A,
    beneficiary: B,
    amount: SerializableAtLeast32BitUnsigned<C>,
}

impl<A, B, C: Clone + AtLeast32BitUnsigned> DeipAssetsIssueAssetCallArgs<A, B, C> {
    fn new(id: A, beneficiary: B, amount: &C) -> Self {
        Self { id, beneficiary, amount: SerializableAtLeast32BitUnsigned(amount.clone()) }
    }
}

#[derive(Serialize)]
struct DeipAssetsDestroyCallArgs<A, B> {
    id: A,
    witness: B,
}

#[derive(Serialize)]
struct DeipAssetsCreateAssetCallArgs<A, B, D: Clone + AtLeast32BitUnsigned, E> {
    id: A,
    admin: B,
    min_balance: SerializableAtLeast32BitUnsigned<D>,
    project_id: E,
}

impl<A, B, D: Clone + AtLeast32BitUnsigned, E> DeipAssetsCreateAssetCallArgs<A, B, D, E> {
    fn new(id: A, admin: B, min_balance: &D, project_id: E) -> Self {
        Self {
            id,
            admin,
            min_balance: SerializableAtLeast32BitUnsigned(min_balance.clone()),
            project_id,
        }
    }
}

#[derive(Serialize)]
struct DeipDaoOnBehalfCallArgs<A, B> {
    name: A,
    call: B,
}

#[derive(Serialize)]
struct DeipDaoAlterAuthorityCallArgs<A> {
    alter_authority: A,
}

#[derive(Serialize)]
struct DeipDaoUpdateCallArgs<A> {
    metadata: A,
}

#[derive(Serialize)]
struct DeipDaoCreateCallArgs<A, B, C> {
    name: A,
    authority: B,
    metadata: C,
}

#[derive(Serialize)]
struct DeipProposalDecideCallArgs<A, B> {
    proposal_id: A,
    decision: B,
}

#[derive(Serialize)]
struct DeipProposalExpireCallArgs<A> {
    proposal_id: A,
}

#[derive(Serialize)]
struct DeipProposalProposeCallArgs<A, B> {
    batch: A,
    external_id: B,
}

#[derive(Serialize)]
struct DeipAddDomainCallArgs<A> {
    domain: A,
}

#[derive(Serialize)]
struct DeipCreateContractAgreementCallArgs<A, B, C, D, E, F, G> {
    id: A,
    creator: B,
    parties: C,
    hash: D,
    activation_time: E,
    expiration_time: F,
    terms: G,
}

#[derive(Serialize)]
struct DeipAcceptContractAgreementCallArgs<A, B> {
    id: A,
    party: B,
}

#[derive(Serialize)]
struct DeipRejectContractAgreementCallArgs<A, B> {
    id: A,
    party: B,
}

#[derive(Serialize)]
struct DeipCreateReviewCallArgs<A, B, C, D, E, F, G> {
    external_id: A,
    author: B,
    content: C,
    domains: D,
    assessment_model: E,
    weight: F,
    project_content_external_id: G,
}

#[derive(Serialize)]
struct DeipUpvoteReviewCallArgs<A, B> {
    review_id: A,
    domain_id: B,
}

#[derive(Serialize)]
struct DeipRejectNdaAccessRequestCallArgs<A> {
    external_id: A,
}

#[derive(Serialize)]
struct DeipFulfillNdaAccessRequestCallArgs<A, B, C> {
    external_id: A,
    encrypted_payload_encryption_key: B,
    proof_of_encrypted_payload_encryption_key: C,
}

#[derive(Serialize)]
struct DeipCreateProjectNdaAccessRequestCallArgs<A, B, C, D> {
    external_id: A,
    nda_external_id: B,
    encrypted_payload_hash: C,
    encrypted_payload_iv: D,
}

#[derive(Serialize)]
struct DeipCreateProjectNdaCallArgs<A, B, C, D, E, F> {
    external_id: A,
    end_date: B,
    contract_hash: C,
    maybe_start_date: D,
    parties: E,
    projects: F,
}

#[derive(Serialize)]
struct DeipCreateProjectContentCallArgs<A, B, C, D, E, F, G, H> {
    external_id: A,
    project_external_id: B,
    team_id: C,
    content_type: D,
    description: E,
    content: F,
    authors: G,
    references: H,
}

#[derive(Serialize)]
struct DeipInvestCallArgs<A, B> {
    id: A,
    amount: B,
}

#[derive(Serialize)]
struct DeipUpdateProjectCallArgs<A, B, C> {
    project_id: A,
    description: B,
    is_private: C,
}

#[derive(Serialize)]
struct DeipFinishCrowdfundingCallArgs<A> {
    sale_id: A,
}

#[derive(Serialize)]
struct DeipExpireCrowdfundingCallArgs<A> {
    sale_id: A,
}

#[derive(Serialize)]
struct DeipActivateCrowdfundingCallArgs<A> {
    sale_id: A,
}

#[derive(Serialize)]
struct DeipCreateInvestmentOpportunityCallArgs<A, B, C, D> {
    external_id: A,
    creator: B,
    shares: C,
    funding_model: D,
}

#[derive(Serialize)]
struct DeipCreateProjectCallArgs<A, B, C, D, E> {
    is_private: A,
    external_id: B,
    team_id: C,
    description: D,
    domains: E,
}

#[derive(Serialize)]
struct CallObject<A, B, C> {
    module: A,
    call: B,
    args: C,
}

impl<A, B, C> CallObject<A, B, C> {
    fn new(module: A, call: B, args: C) -> Self {
        Self { module, call, args }
    }
}
