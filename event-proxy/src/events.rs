use std::fmt::Debug;

// use substrate_subxt::{RawEvent, Event, system::System};
use codec::Decode;
use frame_support::pallet_prelude::Member;
use serde::{ser::Serializer, Deserialize, Serialize};
use subxt::{Config, Event};

use sp_runtime::{
    generic::Block,
    traits::{Block as _Block, Header as _Header},
};
use subxt::RawEvent;

use crate::{
    appchain_deip::deip_proposal::events::{Approved, Proposed},
    frame::{
        assets::{self, Assets},
        deip::{self, Deip},
        deip_dao::{self, DeipDao},
        deip_proposal::{self, DeipProposal},
    },
};

mod mapping;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct BlockMetadata<T: Config> {
    pub number: T::BlockNumber,
    pub hash: T::Hash,
    pub parent_hash: T::Hash,
}

impl<T> BlockMetadata<T>
where
    T: Config,
    T::Extrinsic: Member,
{
    pub fn new(block: &Block<T::Header, T::Extrinsic>) -> Self {
        Self {
            number: block.header().number().to_owned(),
            hash: block.header().hash(),
            parent_hash: block.header().parent_hash().to_owned(),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum TypedEvent<D, I>
where
    Self: From<D> + From<I>,
{
    Domain(D),
    Infrastructure(I),
}

pub type SpecializedEvent<T> = TypedEvent<DomainEvent<T>, InfrastructureEvent<T>>;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum InfrastructureEventData<BlockCreated> {
    BlockCreated(BlockCreated),
}
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum InfrastructureEventMeta {
    BlockCreated { domain_events: u32 },
}

pub type InfrastructureEvent<T> =
    BaseEvent<InfrastructureEventData<BlockMetadata<T>>, InfrastructureEventMeta>;

impl<T> InfrastructureEvent<T>
where
    T: Config,
    T::Extrinsic: Send + Sync,
{
    pub fn block_created(block: &Block<T::Header, T::Extrinsic>, domain_events: u32) -> Self {
        Self {
            name: "block_created".to_string(),
            data: InfrastructureEventData::BlockCreated(BlockMetadata::new(block)),
            meta: InfrastructureEventMeta::BlockCreated { domain_events },
        }
    }
}

#[derive(Serialize, Debug)]
pub struct BaseEvent<Data, Meta> {
    name: String,
    data: Data,
    meta: Meta,
}

pub type PortalId = sp_core::H160;
pub type ExtrinsicIndex = u32;

#[derive(Serialize, Debug)]
pub struct DomainEventMeta<Block> {
    index: ExtrinsicIndex,
    block: Block,
    portal_id: PortalId,
}

pub type DomainEvent<T> = BaseEvent<DomainEventData<T>, DomainEventMeta<BlockMetadata<T>>>;

impl<T: Config> From<DomainEvent<T>> for SpecializedEvent<T>
where
    T: Deip + DeipProposal + DeipDao + Assets,
{
    fn from(source: DomainEvent<T>) -> Self {
        Self::Domain(source)
    }
}

impl<T: Config> From<InfrastructureEvent<T>> for SpecializedEvent<T>
where
    T: Deip + DeipProposal + DeipDao + Assets,
{
    fn from(source: InfrastructureEvent<T>) -> Self {
        Self::Infrastructure(source)
    }
}

impl<T: DeipProposal + Deip + DeipDao + Assets> Serialize for DomainEventData<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        pub use DomainEventData::*;
        match self {
            // =============== DeipProposal:
            ProposalProposed(e) => e.serialize(serializer),
            ProposalApproved(e) => e.serialize(serializer),
            ProposalRevokedApproval(e) => e.serialize(serializer),
            ProposalResolved(e) => e.serialize(serializer),
            ProposalExpired(e) => e.serialize(serializer),
            // =============== Deip:
            ProjectCreated(e) => e.serialize(serializer),
            ProjectRemoved(e) => e.serialize(serializer),
            ProjectUpdated(e) => e.serialize(serializer),
            ProjectContentCreated(e) => e.serialize(serializer),
            NdaCreated(e) => e.serialize(serializer),
            NdaAccessRequestCreated(e) => e.serialize(serializer),
            NdaAccessRequestFulfilled(e) => e.serialize(serializer),
            NdaAccessRequestRejected(e) => e.serialize(serializer),
            DomainAdded(e) => e.serialize(serializer),
            ReviewCreated(e) => e.serialize(serializer),
            ReviewUpvoted(e) => e.serialize(serializer),
            SimpleCrowdfundingCreated(e) => e.serialize(serializer),
            SimpleCrowdfundingActivated(e) => e.serialize(serializer),
            SimpleCrowdfundingFinished(e) => e.serialize(serializer),
            SimpleCrowdfundingExpired(e) => e.serialize(serializer),
            Invested(e) => e.serialize(serializer),
            ContractAgreementCreated(e) => e.serialize(serializer),
            ContractAgreementAccepted(e) => e.serialize(serializer),
            ContractAgreementFinalized(e) => e.serialize(serializer),
            ContractAgreementRejected(e) => e.serialize(serializer),
            // =============== DeipDao:
            DaoCreate(e) => e.serialize(serializer),
            DaoAlterAuthority(e) => e.serialize(serializer),
            DaoMetadataUpdated(e) => e.serialize(serializer),
            // =============== Assets:
            AssetClassCreated(e) => e.serialize(serializer),
            AssetIssued(e) => e.serialize(serializer),
            AssetTransferred(e) => e.serialize(serializer),
            AssetBurned(e) => e.serialize(serializer),
            AssetTeamChanged(e) => e.serialize(serializer),
            AssetOwnerChanged(e) => e.serialize(serializer),
            #[cfg(not(feature = "octopus"))]
            AssetForceTransferred(e) => e.serialize(serializer),
            AssetAccountFrozen(e) => e.serialize(serializer),
            AssetAccountThawed(e) => e.serialize(serializer),
            AssetFrozen(e) => e.serialize(serializer),
            AssetThawed(e) => e.serialize(serializer),
            AssetClassDestroyed(e) => e.serialize(serializer),
            AssetClassForceCreated(e) => e.serialize(serializer),
            #[cfg(not(feature = "octopus"))]
            AssetMaxZombiesChanged(e) => e.serialize(serializer),
            AssetMetadataSet(e) => e.serialize(serializer),
            #[cfg(feature = "octopus")]
            AssetMetadataCleared(e) => e.serialize(serializer),
            #[cfg(feature = "octopus")]
            AssetApprovedTransfer(e) => e.serialize(serializer),
            #[cfg(feature = "octopus")]
            AssetApprovalCancelled(e) => e.serialize(serializer),
            #[cfg(feature = "octopus")]
            AssetTransferredApproved(e) => e.serialize(serializer),
            #[cfg(feature = "octopus")]
            AssetStatusChanged(e) => e.serialize(serializer),
        }
    }
}

#[derive(Debug)]
pub enum DomainEventData<T: DeipProposal + Deip + DeipDao + Assets> {
    // DeipProposal:
    ProposalProposed(Proposed),
    ProposalApproved(Approved),
    ProposalRevokedApproval(deip_proposal::RevokedApprovalEvent<T>),
    ProposalResolved(deip_proposal::ResolvedEvent<T>),
    ProposalExpired(deip_proposal::ExpiredEvent<T>),
    // Deip:
    ProjectCreated(deip::ProjectCreatedEvent<T>),
    ProjectRemoved(deip::ProjectRemovedEvent<T>),
    ProjectUpdated(deip::ProjectUpdatedEvent<T>),
    ProjectContentCreated(deip::ProjectContentCreatedEvent<T>),
    NdaCreated(deip::NdaCreatedEvent<T>),
    NdaAccessRequestCreated(deip::NdaAccessRequestCreatedEvent<T>),
    NdaAccessRequestFulfilled(deip::NdaAccessRequestFulfilledEvent<T>),
    NdaAccessRequestRejected(deip::NdaAccessRequestRejectedEvent<T>),
    DomainAdded(deip::DomainAddedEvent<T>),
    ReviewCreated(deip::ReviewCreatedEvent<T>),
    ReviewUpvoted(deip::ReviewUpvotedEvent<T>),
    SimpleCrowdfundingCreated(deip::SimpleCrowdfundingCreatedEvent<T>),
    SimpleCrowdfundingActivated(deip::SimpleCrowdfundingActivatedEvent<T>),
    SimpleCrowdfundingFinished(deip::SimpleCrowdfundingFinishedEvent<T>),
    SimpleCrowdfundingExpired(deip::SimpleCrowdfundingExpiredEvent<T>),
    Invested(deip::InvestedEvent<T>),
    ContractAgreementCreated(deip::ContractAgreementCreatedEvent<T>),
    ContractAgreementAccepted(deip::ContractAgreementAcceptedEvent<T>),
    ContractAgreementFinalized(deip::ContractAgreementFinalizedEvent<T>),
    ContractAgreementRejected(deip::ContractAgreementRejectedEvent<T>),
    // DeipDao:
    DaoCreate(deip_dao::DaoCreateEvent<T>),
    DaoAlterAuthority(deip_dao::DaoAlterAuthorityEvent<T>),
    DaoMetadataUpdated(deip_dao::DaoMetadataUpdatedEvent<T>),
    // Assets:
    AssetClassCreated(assets::CreatedEvent<T>),
    AssetIssued(assets::IssuedEvent<T>),
    AssetTransferred(assets::TransferredEvent<T>),
    AssetBurned(assets::BurnedEvent<T>),
    AssetTeamChanged(assets::TeamChangedEvent<T>),
    AssetOwnerChanged(assets::OwnerChangedEvent<T>),
    #[cfg(not(feature = "octopus"))]
    AssetForceTransferred(assets::ForceTransferredEvent<T>),
    AssetAccountFrozen(assets::FrozenEvent<T>),
    AssetAccountThawed(assets::ThawedEvent<T>),
    AssetFrozen(assets::AssetFrozenEvent<T>),
    AssetThawed(assets::AssetThawedEvent<T>),
    AssetClassDestroyed(assets::DestroyedEvent<T>),
    AssetClassForceCreated(assets::ForceCreatedEvent<T>),
    #[cfg(not(feature = "octopus"))]
    AssetMaxZombiesChanged(assets::MaxZombiesChangedEvent<T>),
    AssetMetadataSet(assets::MetadataSetEvent<T>),
    #[cfg(feature = "octopus")]
    AssetMetadataCleared(assets::MetadataClearedEvent<T>),
    #[cfg(feature = "octopus")]
    AssetApprovedTransfer(assets::ApprovedTransferEvent<T>),
    #[cfg(feature = "octopus")]
    AssetApprovalCancelled(assets::ApprovalCancelledEvent<T>),
    #[cfg(feature = "octopus")]
    AssetTransferredApproved(assets::TransferredApprovedEvent<T>),
    #[cfg(feature = "octopus")]
    AssetStatusChanged(assets::AssetStatusChangedEvent<T>),
}

pub fn known_domain_events<T>(
    raw: &(ExtrinsicIndex, RawEvent),
    block: &Block<<T as Config>::Header, T::Extrinsic>,
    portal_id: &PortalId,
) -> Result<Option<SpecializedEvent<T>>, codec::Error>
where
    T: DeipProposal + Deip + DeipDao + Assets + Debug + Config,
    T::Extrinsic: Member + Send + Sync,
{
    let (index, raw) = raw;
    let meta =
        DomainEventMeta { index: *index, block: BlockMetadata::new(block), portal_id: *portal_id };

    use crate::{appchain_deip::deip_proposal::events::*, events::DomainEventData::*};

    info!("Event from: {} - {}", raw.pallet.as_str(), raw.variant.as_str());
    let event = match (raw.pallet.as_str(), raw.variant.as_str()) {
        // =========== DeipProposal:
        (Proposed::PALLET, Proposed::EVENT) => DomainEvent {
            name: "proposal_proposed".to_string(),
            data: decode_event_data(raw).map(ProposalProposed)?,
            meta,
        },
        (Approved::PALLET, Approved::EVENT) => DomainEvent {
            name: "proposal_approved".to_string(),
            data: decode_event_data(raw).map(ProposalApproved)?,
            meta,
        },
        (RevokedApproval::PALLET, RevokedApproval::EVENT) => DomainEvent {
            name: "proposal_revokedApproval".to_string(),
            data: decode_event_data(raw).map(ProposalRevokedApproval)?,
            meta,
        },
        (Resolved::PALLET, Resolved::EVENT) => DomainEvent {
            name: "proposal_resolved".to_string(),
            data: decode_event_data(raw).map(ProposalResolved)?,
            meta,
        },
        (Expired::PALLET, Expired::EVENT) => DomainEvent {
            name: "proposal_expired".to_string(),
            data: decode_event_data(raw).map(ProposalExpired)?,
            meta,
        },
        // =========== Deip:
        // (deip::ProjectCreatedEvent::<T>::PALLET, deip::ProjectCreatedEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "project_created".to_string(),
        //         data: decode_event_data(raw).map(ProjectCreated)?,
        //         meta,
        //     },
        // (deip::ProjectRemovedEvent::<T>::PALLET, deip::ProjectRemovedEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "project_removed".to_string(),
        //         data: decode_event_data(raw).map(ProjectRemoved)?,
        //         meta,
        //     },
        // (deip::ProjectUpdatedEvent::<T>::PALLET, deip::ProjectUpdatedEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "project_updated".to_string(),
        //         data: decode_event_data(raw).map(ProjectUpdated)?,
        //         meta,
        //     },
        // (
        //     deip::ProjectContentCreatedEvent::<T>::PALLET,
        //     deip::ProjectContentCreatedEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "project_contentCreated".to_string(),
        //     data: decode_event_data(raw).map(ProjectContentCreated)?,
        //     meta,
        // },
        // (deip::NdaCreatedEvent::<T>::PALLET, deip::NdaCreatedEvent::<T>::EVENT) => DomainEvent {
        //     name: "project_ndaCreated".to_string(),
        //     data: decode_event_data(raw).map(NdaCreated)?,
        //     meta,
        // },
        // (
        //     deip::NdaAccessRequestCreatedEvent::<T>::PALLET,
        //     deip::NdaAccessRequestCreatedEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "project_ndaAccessRequestCreated".to_string(),
        //     data: decode_event_data(raw).map(NdaAccessRequestCreated)?,
        //     meta,
        // },
        // (
        //     deip::NdaAccessRequestFulfilledEvent::<T>::PALLET,
        //     deip::NdaAccessRequestFulfilledEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "project_ndaAccessRequestFulfilled".to_string(),
        //     data: decode_event_data(raw).map(NdaAccessRequestFulfilled)?,
        //     meta,
        // },
        // (
        //     deip::NdaAccessRequestRejectedEvent::<T>::PALLET,
        //     deip::NdaAccessRequestRejectedEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "project_ndaAccessRequestRejected".to_string(),
        //     data: decode_event_data(raw).map(NdaAccessRequestRejected)?,
        //     meta,
        // },
        // (deip::DomainAddedEvent::<T>::PALLET, deip::DomainAddedEvent::<T>::EVENT) => DomainEvent {
        //     name: "project_domainAdded".to_string(),
        //     data: decode_event_data(raw).map(DomainAdded)?,
        //     meta,
        // },
        // (deip::ReviewCreatedEvent::<T>::PALLET, deip::ReviewCreatedEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "project_reviewCreated".to_string(),
        //         data: decode_event_data(raw).map(ReviewCreated)?,
        //         meta,
        //     },
        // (deip::ReviewUpvotedEvent::<T>::PALLET, deip::ReviewUpvotedEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "project_reviewUpvoted".to_string(),
        //         data: decode_event_data(raw).map(ReviewUpvoted)?,
        //         meta,
        //     },
        // (
        //     deip::SimpleCrowdfundingCreatedEvent::<T>::PALLET,
        //     deip::SimpleCrowdfundingCreatedEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "project_tokenSaleCreated".to_string(),
        //     data: decode_event_data(raw).map(SimpleCrowdfundingCreated)?,
        //     meta,
        // },
        // (
        //     deip::SimpleCrowdfundingActivatedEvent::<T>::PALLET,
        //     deip::SimpleCrowdfundingActivatedEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "project_tokenSaleActivated".to_string(),
        //     data: decode_event_data(raw).map(SimpleCrowdfundingActivated)?,
        //     meta,
        // },
        // (
        //     deip::SimpleCrowdfundingFinishedEvent::<T>::PALLET,
        //     deip::SimpleCrowdfundingFinishedEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "project_tokenSaleFinished".to_string(),
        //     data: decode_event_data(raw).map(SimpleCrowdfundingFinished)?,
        //     meta,
        // },
        // (
        //     deip::SimpleCrowdfundingExpiredEvent::<T>::PALLET,
        //     deip::SimpleCrowdfundingExpiredEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "project_tokenSaleExpired".to_string(),
        //     data: decode_event_data(raw).map(SimpleCrowdfundingExpired)?,
        //     meta,
        // },
        // (deip::InvestedEvent::<T>::PALLET, deip::InvestedEvent::<T>::EVENT) => DomainEvent {
        //     name: "project_tokenSaleContributed".to_string(),
        //     data: decode_event_data(raw).map(Invested)?,
        //     meta,
        // },
        // (
        //     deip::ContractAgreementCreatedEvent::<T>::PALLET,
        //     deip::ContractAgreementCreatedEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "deip_contractAgreementCreated".to_string(),
        //     data: decode_event_data(raw).map(ContractAgreementCreated)?,
        //     meta,
        // },
        // (
        //     deip::ContractAgreementAcceptedEvent::<T>::PALLET,
        //     deip::ContractAgreementAcceptedEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "deip_contractAgreementAccepted".to_string(),
        //     data: decode_event_data(raw).map(ContractAgreementAccepted)?,
        //     meta,
        // },
        // (
        //     deip::ContractAgreementFinalizedEvent::<T>::PALLET,
        //     deip::ContractAgreementFinalizedEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "deip_contractAgreementFinalized".to_string(),
        //     data: decode_event_data(raw).map(ContractAgreementFinalized)?,
        //     meta,
        // },
        // (
        //     deip::ContractAgreementRejectedEvent::<T>::PALLET,
        //     deip::ContractAgreementRejectedEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "deip_contractAgreementRejected".to_string(),
        //     data: decode_event_data(raw).map(ContractAgreementRejected)?,
        //     meta,
        // },
        // // =========== DeipDao:
        // (deip_dao::DaoCreateEvent::<T>::PALLET, deip_dao::DaoCreateEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "dao_create".to_string(),
        //         data: decode_event_data(raw).map(DaoCreate)?,
        //         meta,
        //     },
        // (
        //     deip_dao::DaoAlterAuthorityEvent::<T>::PALLET,
        //     deip_dao::DaoAlterAuthorityEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "dao_alterAuthority".to_string(),
        //     data: decode_event_data(raw).map(DaoAlterAuthority)?,
        //     meta,
        // },
        // (
        //     deip_dao::DaoMetadataUpdatedEvent::<T>::PALLET,
        //     deip_dao::DaoMetadataUpdatedEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "dao_metadataUpdated".to_string(),
        //     data: decode_event_data(raw).map(DaoMetadataUpdated)?,
        //     meta,
        // },
        // // =========== Assets:
        // (assets::CreatedEvent::<T>::PALLET, assets::CreatedEvent::<T>::EVENT) => DomainEvent {
        //     name: "asset_class_created".to_string(),
        //     data: decode_event_data(raw).map(AssetClassCreated)?,
        //     meta,
        // },
        // (assets::IssuedEvent::<T>::PALLET, assets::IssuedEvent::<T>::EVENT) => DomainEvent {
        //     name: "asset_issued".to_string(),
        //     data: decode_event_data(raw).map(AssetIssued)?,
        //     meta,
        // },
        // (assets::TransferredEvent::<T>::PALLET, assets::TransferredEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "asset_transferred".to_string(),
        //         data: decode_event_data(raw).map(AssetTransferred)?,
        //         meta,
        //     },
        // (assets::BurnedEvent::<T>::PALLET, assets::BurnedEvent::<T>::EVENT) => DomainEvent {
        //     name: "asset_burned".to_string(),
        //     data: decode_event_data(raw).map(AssetBurned)?,
        //     meta,
        // },
        // (assets::TeamChangedEvent::<T>::PALLET, assets::TeamChangedEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "asset_team_changed".to_string(),
        //         data: decode_event_data(raw).map(AssetTeamChanged)?,
        //         meta,
        //     },
        // (assets::OwnerChangedEvent::<T>::PALLET, assets::OwnerChangedEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "asset_owner_changed".to_string(),
        //         data: decode_event_data(raw).map(AssetOwnerChanged)?,
        //         meta,
        //     },
        // #[cfg(not(feature = "octopus"))]
        // (assets::ForceTransferredEvent::<T>::PALLET, assets::ForceTransferredEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "asset_force_transferred".to_string(),
        //         data: decode_event_data(raw).map(AssetForceTransferred)?,
        //         meta,
        //     },
        // (assets::FrozenEvent::<T>::PALLET, assets::FrozenEvent::<T>::EVENT) => DomainEvent {
        //     name: "asset_account_frozen".to_string(),
        //     data: decode_event_data(raw).map(AssetAccountFrozen)?,
        //     meta,
        // },
        // (assets::ThawedEvent::<T>::PALLET, assets::ThawedEvent::<T>::EVENT) => DomainEvent {
        //     name: "asset_account_thawed".to_string(),
        //     data: decode_event_data(raw).map(AssetAccountThawed)?,
        //     meta,
        // },
        // (assets::AssetFrozenEvent::<T>::PALLET, assets::AssetFrozenEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "asset_frozen".to_string(),
        //         data: decode_event_data(raw).map(AssetFrozen)?,
        //         meta,
        //     },
        // (assets::AssetThawedEvent::<T>::PALLET, assets::AssetThawedEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "asset_thawed".to_string(),
        //         data: decode_event_data(raw).map(AssetThawed)?,
        //         meta,
        //     },
        // (assets::DestroyedEvent::<T>::PALLET, assets::DestroyedEvent::<T>::EVENT) => DomainEvent {
        //     name: "asset_class_destroyed".to_string(),
        //     data: decode_event_data(raw).map(AssetClassDestroyed)?,
        //     meta,
        // },
        // (assets::ForceCreatedEvent::<T>::PALLET, assets::ForceCreatedEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "asset_class_force_created".to_string(),
        //         data: decode_event_data(raw).map(AssetClassForceCreated)?,
        //         meta,
        //     },
        // #[cfg(not(feature = "octopus"))]
        // (
        //     assets::MaxZombiesChangedEvent::<T>::PALLET,
        //     assets::MaxZombiesChangedEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "asset_max_zombies_changed".to_string(),
        //     data: decode_event_data(raw).map(AssetMaxZombiesChanged)?,
        //     meta,
        // },
        // (assets::MetadataSetEvent::<T>::PALLET, assets::MetadataSetEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "asset_metadata_set".to_string(),
        //         data: decode_event_data(raw).map(AssetMetadataSet)?,
        //         meta,
        //     },
        // #[cfg(feature = "octopus")]
        // (assets::MetadataClearedEvent::<T>::PALLET, assets::MetadataClearedEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "asset_metadata_cleared".to_string(),
        //         data: decode_event_data(raw).map(AssetMetadataCleared)?,
        //         meta,
        //     },
        // #[cfg(feature = "octopus")]
        // (assets::ApprovedTransferEvent::<T>::PALLET, assets::ApprovedTransferEvent::<T>::EVENT) =>
        //     DomainEvent {
        //         name: "asset_approved_transfer".to_string(),
        //         data: decode_event_data(raw).map(AssetApprovedTransfer)?,
        //         meta,
        //     },
        // #[cfg(feature = "octopus")]
        // (
        //     assets::ApprovalCancelledEvent::<T>::PALLET,
        //     assets::ApprovalCancelledEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "asset_approval_cancelled".to_string(),
        //     data: decode_event_data(raw).map(AssetApprovalCancelled)?,
        //     meta,
        // },
        // #[cfg(feature = "octopus")]
        // (
        //     assets::TransferredApprovedEvent::<T>::PALLET,
        //     assets::TransferredApprovedEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "asset_transferred_approved".to_string(),
        //     data: decode_event_data(raw).map(AssetTransferredApproved)?,
        //     meta,
        // },
        // #[cfg(feature = "octopus")]
        // (
        //     assets::AssetStatusChangedEvent::<T>::PALLET,
        //     assets::AssetStatusChangedEvent::<T>::EVENT,
        // ) => DomainEvent {
        //     name: "asset_status_changed".to_string(),
        //     data: decode_event_data(raw).map(AssetStatusChanged)?,
        //     meta,
        // },
        _ => return Ok(None),
    };
    info!("Event decoded: {}", event.name);
    Ok(Some(event.into()))
}

fn decode_event_data<T: Decode>(e: &RawEvent) -> Result<T, codec::Error> {
    T::decode(&mut &e.data[..])
}
