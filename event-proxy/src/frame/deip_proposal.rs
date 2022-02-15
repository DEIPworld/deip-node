use codec::Decode;
use frame_support::Parameter;
use sp_runtime::traits::Member;

use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};
use subxt::Config;

use crate::runtime_api::api::deip_proposal::events::{
    Approved, Expired, Resolved, RevokedApproval,
};

pub trait DeipProposal: Config {
    type ProposalBatch: Parameter + Member;
    type InputProposalBatch: Parameter + Member;
    type ProposalId: Parameter + Member + Serialize;
    type Call: Parameter + Member;
    type BatchItem: Parameter + Member;
    type ProposalState: Parameter + Member + Serialize;
    /// Wrapper type to perform data transformations before serialization
    type WrappedBatch: Parameter + Member + Serialize;
    /// Wrapper type to perform data transformations before serialization
    type WrappedInputBatch: Parameter + Member + Serialize;
    /// Wrapper type to perform data transformations before serialization
    type WrappedCall: Parameter + Member + Serialize;

    fn wrap_batch<T: From<Self::WrappedBatch>>(batch: &Self::ProposalBatch) -> T;

    fn wrap_input_batch(batch: &Self::InputProposalBatch) -> Self::WrappedInputBatch;
}

impl Serialize for Approved {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ApprovedEvent", 2)?;
        s.serialize_field("member", &self.member)?;
        s.serialize_field("proposal_id", &self.proposal_id)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ProposedEvent<T: DeipProposal> {
    pub author: T::AccountId,
    pub batch: T::ProposalBatch,
    pub proposal_id: T::ProposalId,
}

impl<T> Serialize for ProposedEvent<T>
where
    T: DeipProposal,
    T::AccountId: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ProposedEvent", 3)?;
        s.serialize_field("author", &self.author)?;
        s.serialize_field("batch", &T::wrap_batch::<T::WrappedBatch>(&self.batch))?;
        s.serialize_field("proposal_id", &self.proposal_id)?;
        s.end()
    }
}

impl Serialize for RevokedApproval {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("RevokedApprovalEvent", 2)?;
        s.serialize_field("member", &self.member)?;
        s.serialize_field("proposal_id", &self.proposal_id)?;
        s.end()
    }
}

impl Serialize for Resolved {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ResolvedEvent", 3)?;
        s.serialize_field("member", &self.member)?;
        s.serialize_field("proposal_id", &self.proposal_id)?;
        s.serialize_field("state", &self.state)?;
        s.end()
    }
}

impl Serialize for Expired {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ExpiredEvent", 1)?;
        s.serialize_field("proposal_id", &self.proposal_id)?;
        s.end()
    }
}
