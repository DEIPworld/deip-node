use crate::{
    frame::{self, deip_proposal::DeipProposal},
    RuntimeT,
};

use pallet_deip_assets::pallet_assets;

use deip_call::WrappedCall;

type RealRuntime = node_template_runtime::Runtime;

impl DeipProposal for RuntimeT {
    type ProposalBatch = pallet_deip_proposal::proposal::ProposalBatch<RealRuntime>;
    type InputProposalBatch = pallet_deip_proposal::proposal::InputProposalBatch<RealRuntime>;
    type ProposalId = pallet_deip_proposal::proposal::ProposalId;
    type Call = node_template_runtime::Call;
    type BatchItem = pallet_deip_proposal::proposal::ProposalBatchItemOf<RealRuntime>;
    type ProposalState = pallet_deip_proposal::proposal::ProposalState;
    type WrappedBatch = Vec<
        pallet_deip_proposal::proposal::BatchItem<
            node_template_runtime::AccountId,
            Self::WrappedCall,
        >,
    >;
    type WrappedInputBatch = Vec<
        pallet_deip_proposal::proposal::BatchItem<
            node_template_runtime::deip_account::DeipAccountId<node_template_runtime::AccountId>,
            Self::WrappedCall,
        >,
    >;
    type WrappedCall = WrappedCall<<Self as DeipProposal>::Call>;

    fn wrap_batch<T: From<Self::WrappedBatch>>(batch: &Self::ProposalBatch) -> T {
        batch
            .iter()
            .map(|x| pallet_deip_proposal::proposal::BatchItem {
                account: x.account.clone(),
                call: WrappedCall::wrap(&x.call),
            })
            .collect::<Self::WrappedBatch>()
            .into()
    }

    fn wrap_input_batch(batch: &Self::InputProposalBatch) -> Self::WrappedInputBatch {
        deip_call::wrap_input_batch(batch)
    }
}

impl frame::deip::Deip for RuntimeT {
    type DomainId = pallet_deip::DomainId;
    type ProjectId = pallet_deip::ProjectId;
    type Project = pallet_deip::Project<Self::Hash, Self::AccountId>;
    type ReviewId = pallet_deip::ReviewId;
    type Review = pallet_deip::Review<Self::Hash, Self::AccountId>;
    type NdaId = pallet_deip::NdaId;
    type NdaAccessRequestId = pallet_deip::NdaAccessRequestId;
    type ProjectContentId = pallet_deip::ProjectContentId;
    type InvestmentId = pallet_deip::InvestmentId;
    type FundingModel = pallet_deip::FundingModelOf<RealRuntime>;
    type ContractAgreementId = pallet_deip::ContractAgreementId;
    type ContractAgreementTerms = pallet_deip::ContractAgreementTermsOf<RealRuntime>;
}

impl frame::deip_dao::DeipDao for RuntimeT {
    type Dao = pallet_deip_dao::dao::DaoOf<RealRuntime>;
}

type AssetId = <RealRuntime as pallet_assets::Config>::AssetId;
type Balance = <RealRuntime as pallet_assets::Config>::Balance;

impl frame::assets::Assets for RuntimeT {
    type AssetId = AssetId;
    type Balance = Balance;
}

// impl frame::octopus_appchain::OctopusAppchain for RuntimeT {
//     type Balance = <<RealRuntime as pallet_octopus_appchain::Config>::Currency as frame_support::traits::Currency<<RealRuntime as frame_system::Config>::AccountId>>::Balance;
//     type AssetBalance = <<RealRuntime as pallet_octopus_appchain::Config>::Assets as frame_support::traits::fungibles::Inspect<<RealRuntime as frame_system::Config>::AccountId>>::Balance;
//     type AssetId = <<RealRuntime as pallet_octopus_appchain::Config>::Assets as frame_support::traits::fungibles::Inspect<<RealRuntime as frame_system::Config>::AccountId>>::AssetId;
// }

// impl frame::octopus_lpos::OctopusLpos for RuntimeT {
//     type EraIndex = pallet_octopus_lpos::EraIndex;
// }
