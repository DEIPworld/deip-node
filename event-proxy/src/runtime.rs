use crate::{frame::deip_proposal::DeipProposal, RuntimeT};

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
