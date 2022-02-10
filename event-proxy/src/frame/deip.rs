use codec::Decode;
use frame_support::Parameter;
use frame_system::Config;
use sp_runtime::traits::Member;
use sp_std::prelude::*;

use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

pub trait Deip: Config {
    type DomainId: Parameter + Member + Serialize;
    type ProjectId: Parameter + Member + Serialize;
    type Project: Parameter + Member + Serialize;
    type ReviewId: Parameter + Member + Serialize;
    type Review: Parameter + Member + Serialize;
    type NdaId: Parameter + Member + Serialize;
    type NdaAccessRequestId: Parameter + Member + Serialize;
    type ProjectContentId: Parameter + Member + Serialize;
    type InvestmentId: Parameter + Member + Serialize;
    type FundingModel: Parameter + Member + Serialize;
    type ContractAgreementId: Parameter + Member + Serialize;
    type ContractAgreementTerms: Parameter + Member + Serialize;
}

const ACCOUNT_ID_KEY: &str = "account_id";

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ProjectCreatedEvent<T: Deip>(T::AccountId, T::Project);
impl<T: Deip> Serialize for ProjectCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ProjectCreatedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("project", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ProjectRemovedEvent<T: Deip>(T::AccountId, T::Project);
impl<T: Deip> Serialize for ProjectRemovedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ProjectRemovedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("project", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ProjectUpdatedEvent<T: Deip>(T::AccountId, T::ProjectId);
impl<T: Deip> Serialize for ProjectUpdatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ProjectUpdatedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("project_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ProjectContentCreatedEvent<T: Deip>(T::AccountId, T::ProjectContentId);
impl<T: Deip> Serialize for ProjectContentCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ProjectContentCreatedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("content_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct NdaCreatedEvent<T: Deip>(T::AccountId, T::NdaId);
impl<T: Deip> Serialize for NdaCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("NdaCreatedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("nda_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct NdaAccessRequestCreatedEvent<T: Deip>(T::AccountId, T::NdaAccessRequestId);
impl<T: Deip> Serialize for NdaAccessRequestCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("NdaAccessRequestCreatedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("access_request_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct NdaAccessRequestFulfilledEvent<T: Deip>(T::AccountId, T::NdaAccessRequestId);
impl<T: Deip> Serialize for NdaAccessRequestFulfilledEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("NdaAccessRequestFulfilledEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("access_request_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct NdaAccessRequestRejectedEvent<T: Deip>(T::AccountId, T::NdaAccessRequestId);
impl<T: Deip> Serialize for NdaAccessRequestRejectedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("NdaAccessRequestRejectedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("access_request_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct DomainAddedEvent<T: Deip>(T::AccountId, T::DomainId);
impl<T: Deip> Serialize for DomainAddedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DomainAddedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("domain_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ReviewCreatedEvent<T: Deip>(T::AccountId, T::Review);
impl<T: Deip> Serialize for ReviewCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ReviewCreatedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("review", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ReviewUpvotedEvent<T: Deip>(T::ReviewId, T::AccountId, T::DomainId);
impl<T: Deip> Serialize for ReviewUpvotedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ReviewUpvotedEvent", 2)?;
        s.serialize_field("review_id", &self.0)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.1)?;
        s.serialize_field("domain_id", &self.2)?;
        s.end()
    }
}

const INVESTMENT_ID_KEY: &str = "investment_id";

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct SimpleCrowdfundingCreatedEvent<T: Deip>(T::InvestmentId);
impl<T: Deip> Serialize for SimpleCrowdfundingCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SimpleCrowdfundingCreatedEvent", 1)?;
        s.serialize_field(INVESTMENT_ID_KEY, &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct SimpleCrowdfundingActivatedEvent<T: Deip>(T::InvestmentId);
impl<T: Deip> Serialize for SimpleCrowdfundingActivatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SimpleCrowdfundingActivatedEvent", 1)?;
        s.serialize_field(INVESTMENT_ID_KEY, &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct SimpleCrowdfundingFinishedEvent<T: Deip>(T::InvestmentId);
impl<T: Deip> Serialize for SimpleCrowdfundingFinishedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SimpleCrowdfundingFinishedEvent", 1)?;
        s.serialize_field(INVESTMENT_ID_KEY, &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct SimpleCrowdfundingExpiredEvent<T: Deip>(T::InvestmentId);
impl<T: Deip> Serialize for SimpleCrowdfundingExpiredEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SimpleCrowdfundingExpiredEvent", 1)?;
        s.serialize_field(INVESTMENT_ID_KEY, &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct InvestedEvent<T: Deip>(T::InvestmentId, T::AccountId);
impl<T: Deip> Serialize for InvestedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("InvestedEvent", 2)?;
        s.serialize_field(INVESTMENT_ID_KEY, &self.0)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.1)?;
        s.end()
    }
}

const CONTRACTAGREEMENT_ID_KEY: &str = "contractagreement_id";

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ContractAgreementCreatedEvent<T: Deip>(T::ContractAgreementId);
impl<T: Deip> Serialize for ContractAgreementCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ContractAgreementCreatedEvent", 1)?;
        s.serialize_field(CONTRACTAGREEMENT_ID_KEY, &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ContractAgreementAcceptedEvent<T: Deip>(T::ContractAgreementId, T::AccountId);
impl<T: Deip> Serialize for ContractAgreementAcceptedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ContractAgreementAcceptedEvent", 2)?;
        s.serialize_field(CONTRACTAGREEMENT_ID_KEY, &self.0)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ContractAgreementFinalizedEvent<T: Deip>(T::ContractAgreementId);
impl<T: Deip> Serialize for ContractAgreementFinalizedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ContractAgreementFinalizedEvent", 1)?;
        s.serialize_field(CONTRACTAGREEMENT_ID_KEY, &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ContractAgreementRejectedEvent<T: Deip>(T::ContractAgreementId, T::AccountId);
impl<T: Deip> Serialize for ContractAgreementRejectedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ContractAgreementRejectedEvent", 2)?;
        s.serialize_field(CONTRACTAGREEMENT_ID_KEY, &self.0)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.1)?;
        s.end()
    }
}
