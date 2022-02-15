//! # Deip Module
//! A module for managing digital assets.
//!
//! - [`multisig::Config`](./trait.Config.html)
//! - [`Call`](./enum.Call.html)
//!
//! ## Overview
//!
//! This module contains functionality for managing different types of digital assets.
//!
//! It provides a hierarchy to simply operate digital assets in the real world.
//! The module contains entities Project and  Content of the Project with belongs to multi Account aka Team.
//!
//! Besides, the Module provides Proof of share functionality. Proof of Share is a term we
//! use for a special cryptographic proof that a sender actually sent, and the receiver
//! has actually received an encrypted payload and a key to decrypt it. Please refer to the attached image.
//! Includes entities like NDA and NDA Access Request.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `create_project` - Create Project belongs to Account (Team)
//! * [`create_investment_opportunity`](./enum.Call.html#variant.create_investment_opportunity)
//! * [`invest`](./enum.Call.html#variant.invest)
//! * `update_project` - Update Project info
//! * `create_project_content` - Create Project Content (Digital Asset)
//! * `create_project_nda` - Create NDA contract between sides
//! * `create_nda_content_access_request` - Some side request access to the data of contract
//! * `fulfill_nda_content_access_request` - Granter fulfill access request to the data
//! * `reject_nda_content_access_request` - Granter reject access request to the data
//! * [`create_review`](./enum.Call.html#variant.create_review)
//! * [`upvote_review`](./enum.Call.html#variant.upvote_review)
//! * [`create_contract_agreement`](./enum.Call.html#variant.create_contract_agreement)
//! * [`accept_contract_agreement`](./enum.Call.html#variant.accept_contract_agreement)
//! * [`reject_contract_agreement`](./enum.Call.html#variant.reject_contract_agreement)
//!
//! [`Call`]: ./enum.Call.html
//! [`Config`]: ./trait.Config.html

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    codec::{Decode, Encode},
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::{DispatchResult, Parameter},
    ensure,
    log::debug,
    pallet_prelude::*,
    storage::{IterableStorageDoubleMap, IterableStorageMap},
    traits::{Currency, ReservableCurrency},
    StorageMap,
};
use frame_system::{self as system, ensure_none, ensure_signed, offchain::SendTransactionTypes};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
pub use sp_core::{H160, H256};
use sp_runtime::{
    traits::{Member, ValidateUnsigned},
    RuntimeDebug,
};
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod api;

mod investment_opportunity;
use investment_opportunity::Status as SimpleCrowdfundingStatus;
pub use investment_opportunity::{
    FundingModel, FundingModelOf, Id as InvestmentId, Info as SimpleCrowdfunding,
};

mod contribution;
use contribution::Contribution as Investment;

mod review;
pub use review::{Id as ReviewId, Review, Vote as DeipReviewVote};

mod asset;
pub use asset::Asset as DeipAsset;

pub mod contract;
pub use contract::{
    AgreementOf as ContractAgreementOf, Id as ContractAgreementId,
    IndexTerms as ContractAgreementIndexTerms, TermsOf as ContractAgreementTermsOf,
};

use deip_transaction_ctx::{PortalCtxT, TransactionCtxId};

pub mod traits;

pub mod benchmarking;
pub mod weights;
use system::pallet_prelude::OriginFor;
pub use weights::{WeightInfo, Weights};

/// A maximum number of Domains. When domains reaches this number, no new domains can be added.
pub const MAX_DOMAINS: u32 = 100;

const NON_LOCAL: u8 = 100;

/// Possible statuses of Project inherited from Project Content type
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum ProjectContentType {
    Announcement,
    FinalResult,
    MilestoneArticle,
    MilestoneBook,
    MilestoneChapter,
    MilestoneCode,
    MilestoneConferencePaper,
    MilestoneCoverPage,
    MilestoneData,
    MilestoneExperimentFindings,
    MilestoneMethod,
    MilestoneNegativeResults,
    MilestonePatent,
    MilestonePoster,
    MilestonePreprint,
    MilestonePresentation,
    MilestoneRawData,
    MilestoneResearchProposal,
    MilestoneTechnicalReport,
    MilestoneThesis,
}

impl Default for ProjectContentType {
    fn default() -> ProjectContentType {
        ProjectContentType::Announcement
    }
}

/// Configuration trait. Pallet depends on frame_system and pallet_timestamp.
pub trait Config:
    frame_system::Config + pallet_timestamp::Config + SendTransactionTypes<Call<Self>>
{
    type TransactionCtx: PortalCtxT<Call<Self>>;

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    type DeipAccountId: Into<Self::AccountId> + From<Self::AccountId> + Parameter + Member + Default;

    type Currency: ReservableCurrency<Self::AccountId>;

    type AssetSystem: traits::DeipAssetSystem<Self::AccountId>;

    type DeipWeightInfo: WeightInfo;

    type MaxNdaParties: Get<u16>;
    type MaxInvestmentShares: Get<u16>;
}

/// Unique Project ID reference
pub type ProjectId = H160;
/// Unique DomainId reference
pub type DomainId = H160;
/// Unique Project Contnt reference
pub type ProjectContentId = H160;
/// Unique NDA reference
pub type NdaId = H160;
/// Unique NdaAccess Request reference
pub type NdaAccessRequestId = H160;

type AccountIdOf<T> = <T as system::Config>::AccountId;
pub type DeipAccountIdOf<T> = <T as crate::Config>::DeipAccountId;
type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
pub type HashOf<T> = <T as system::Config>::Hash;
pub type ProjectOf<T> = Project<HashOf<T>, AccountIdOf<T>>;
pub type ReviewOf<T> = Review<HashOf<T>, AccountIdOf<T>>;
pub type NdaOf<T> = Nda<HashOf<T>, AccountIdOf<T>, MomentOf<T>>;
pub type NdaAccessRequestOf<T> = NdaAccessRequest<HashOf<T>, AccountIdOf<T>>;
pub type ProjectContentOf<T> = ProjectContent<HashOf<T>, AccountIdOf<T>>;
pub type SimpleCrowdfundingOf<T> = SimpleCrowdfunding<
    MomentOf<T>,
    DeipAssetIdOf<T>,
    DeipAssetBalanceOf<T>,
    TransactionCtxId<TransactionCtxOf<T>>,
>;
pub type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
pub type InvestmentOf<T> = Investment<AccountIdOf<T>, DeipAssetBalanceOf<T>, MomentOf<T>>;
pub type DeipAssetIdOf<T> =
    <<T as Config>::AssetSystem as traits::DeipAssetSystem<AccountIdOf<T>>>::AssetId;
pub type DeipAssetBalanceOf<T> =
    <<T as Config>::AssetSystem as traits::DeipAssetSystem<AccountIdOf<T>>>::Balance;
pub type DeipAssetOf<T> = DeipAsset<DeipAssetIdOf<T>, DeipAssetBalanceOf<T>>;
type DeipReviewVoteOf<T> = DeipReviewVote<AccountIdOf<T>, MomentOf<T>>;
type TransactionCtxOf<T> = <T as Config>::TransactionCtx;

/// PPossible project domains
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Domain {
    /// Reference for external world and uniques control
    pub external_id: DomainId,
}

/// Core entity of pallet. Everything connected to Project.
/// Only Account (Team) stand before Project in hierarchy.
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Project<Hash, AccountId> {
    /// Determine visible project or not
    is_private: bool,
    /// Reference for external world and uniques control
    external_id: ProjectId,
    /// Reference to the Team
    team_id: AccountId,
    /// Hash of Project description
    description: Hash,
    /// List of Domains aka tags Project matches
    domains: Vec<DomainId>,
}

/// Digital asset. Contains information of content and authors of Digital asset.
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct ProjectContent<Hash, AccountId> {
    /// Reference for external world and uniques control
    external_id: ProjectContentId,
    /// Reference to the Project
    project_external_id: ProjectId,
    /// Reference to the Team
    team_id: AccountId,
    /// Type of content. Determine status of Project
    content_type: ProjectContentType,
    /// Hash of the content ddescription
    description: Hash,
    /// Hast of digital asset
    content: Hash,
    /// Authors of Digital asset
    authors: Vec<AccountId>,
    /// List of References to other digital assets whith will be used in current digital asset.
    references: Option<Vec<ProjectContentId>>,
}

/// NDA contract between parties. Usually about dislocating or not dislocating some confidential info
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Nda<Hash, AccountId, Moment> {
    /// Reference to Multisig Account with involved parties
    contract_creator: AccountId,
    /// Reference for external world and uniques control
    external_id: NdaId,
    /// Unix Timestamp. Exparation date of contract
    end_date: Moment,
    /// Unix Timestamp. Entry into force of the contract
    start_date: Option<Moment>,
    /// Hash of the contract
    contract_hash: Hash,
    /// Involved Parties
    parties: Vec<AccountId>,
    /// Involved Projects
    projects: Vec<ProjectId>,
}

/// Statuses of NDA access requests
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
enum NdaAccessRequestStatus {
    Pending,
    Fulfilled,
    Rejected,
}

impl Default for NdaAccessRequestStatus {
    fn default() -> NdaAccessRequestStatus {
        NdaAccessRequestStatus::Pending
    }
}

/// NDA access request. One of the partice may decide to request to receive
/// some info included into contract. Holder should fulfill or reject this request.
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct NdaAccessRequest<Hash, AccountId> {
    /// Reference for external world and uniques control
    external_id: NdaAccessRequestId,
    /// Reference to NDA
    nda_external_id: NdaId,
    /// Reference to Requester (creator of this request)
    requester: AccountId,
    /// Payload witch need to be decrypted
    encrypted_payload_hash: Hash,
    /// IV of encrypted payload
    encrypted_payload_iv: Vec<u8>,
    /// Execution status
    status: NdaAccessRequestStatus,
    /// Reference to access granter if approved
    grantor: Option<AccountId>,
    /// Ecrypted key witch can decrypt payload
    encrypted_payload_encryption_key: Option<Vec<u8>>,
    /// Proof that requester has access to the encrypted data with his key
    proof_of_encrypted_payload_encryption_key: Option<Vec<u8>>,
}

decl_event! {
    /// Events type.
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        Project = ProjectOf<T>,
        Review = ReviewOf<T>,
    {
        // ==== Projects ====

        /// Event emitted when a project has been created. [BelongsTo, Project]
        ProjectCreated(AccountId, Project),
        /// Event emitted when a project is removed by the owner. [BelongsTo, Project]
        ProjectRemoved(AccountId, Project),
        /// Event emitted when a project is removed by the owner. [BelongsTo, ProjectId]
        ProjectUpdated(AccountId, ProjectId),

        // ==== Project Content ====

        /// Event emitted when a project contnet has been created. [BelongsTo, ProjectContentId]
        ProjectContnetCreated(AccountId, ProjectContentId),

        // ==== NDA ====

        /// Event emitted when a NDA has been created. [BelongsTo, NdaId]
        NdaCreated(AccountId, NdaId),
        /// Event emitted when a NDA Access request has been created. [BelongsTo, NdaAccessRequestId]
        NdaAccessRequestCreated(AccountId, NdaAccessRequestId),
        //  /// Event emitted when a NDA Access request has been fulfilled. [BelongsTo, NdaAccessRequestId]
        NdaAccessRequestFulfilled(AccountId, NdaAccessRequestId),
        //  /// Event emitted when a NDA Access request has been rejected. [BelongsTo, NdaAccessRequestId]
        NdaAccessRequestRejected(AccountId, NdaAccessRequestId),

        /// Added a domain. [Creator, DomainId]
        DomainAdded(AccountId, DomainId),

        /// Event emitted when a review has been created. [BelongsTo, Review]
        ReviewCreated(AccountId, Review),
        /// Emitted when a DAO votes for a review
        ReviewUpvoted(ReviewId, AccountId, DomainId),

        /// Event emitted when a simple crowd funding has been created.
        SimpleCrowdfundingCreated(InvestmentId),
        /// Event emitted when a simple crowd funding has been activated.
        SimpleCrowdfundingActivated(InvestmentId),
        /// Event emitted when a simple crowd funding has finished.
        SimpleCrowdfundingFinished(InvestmentId),
        /// Event emitted when a simple crowd funding has expired.
        SimpleCrowdfundingExpired(InvestmentId),
        /// Event emitted when DAO invested to an opportunity
        Invested(InvestmentId, AccountId),

        ContractAgreementCreated(ContractAgreementId),
        ContractAgreementAccepted(ContractAgreementId, AccountId),
        ContractAgreementFinalized(ContractAgreementId),
        ContractAgreementRejected(ContractAgreementId, AccountId),
    }
}

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Config> {
        // ==== Projects ====

        /// The project does not exist.
        NoSuchProject,
        /// The project is created by another account, so caller can't remove it.
        NotProjectOwner,
        /// Cannot add domain into the porject because this domain not exists
        DomainNotExists,
        /// Cannot add a project because a project with this ID is already a exists
        ProjectAlreadyExists,

        // ==== Project Content ====

        /// Cannot add a project content because a project content with this ID is already a exists.
        ProjectContentAlreadyExists,
        /// Project does not belong to the team.
        ProjectNotBelongToTeam,
        /// The project content does not exist.
        NoSuchProjectContent,
        /// The Reference does not exist.
        NoSuchReference,
        /// Cannot add a project content because a project with this ID is already a finished
        ProjectAlreadyFinished,


        // ==== Domains ====

        /// Cannot add another domain because the limit is already reached
        DomainLimitReached,
        /// Cannot add domain because this domain is already a exists
        DomainAlreadyExists,

        // ==== NDA ====

        /// Cannot add a NDA because a NDA with this ID is already a exists.
        NdaAlreadyExists,
        /// Nda Access Request with this ID is  already a exists.
        NdaAccessRequestAlreadyExists,
        /// The NDA with this ID does not exist.
        NoSuchNda,
        /// The NDA Access Request with this ID does not exist.
        NoSuchNdaAccessRequest,
        /// The start of the contract has not yet arrived, so contract can't be fulfilled or rejected
        NdaContractIsNotActiveYet,
        /// NDA start date must be later or equal current moment
        NdaStartDateMustBeLaterOrEqualCurrentMoment,
        /// NDA end date must be later current moment
        NdaEndDateMustBeLaterCurrentMoment,
        /// NDA start date must be less than end date
        NdaStartDateMustBeLessThanEndDate,
        /// Team of all projects must specified as party
        TeamOfAllProjectsMustSpecifiedAsParty,
        /// Nda access request already finalized
        NdaAccessRequestAlreadyFinalized,
        TooMuchNdaParties,

        /// Cannot add a review because a review with this ID already exists
        ReviewAlreadyExists,
        ReviewNoDomainSpecified,
        ReviewVoteAlreadyExists,
        ReviewVoteNoSuchDomain,
        ReviewVoteNoSuchReview,
        ReviewVoteUnrelatedDomain,
        ReviewAlreadyVotedWithDomain,

        // ==== General =====

        /// Access Forbiten
        NoPermission,

        // Investment opportunity errors
        InvestmentOpportunityStartTimeMustBeLaterOrEqualCurrentMoment,
        InvestmentOpportunityEndTimeMustBeLaterStartTime,
        InvestmentOpportunitySoftCapMustBeGreaterOrEqualMinimum,
        InvestmentOpportunityHardCapShouldBeGreaterOrEqualSoftCap,
        InvestmentOpportunityAlreadyExists,
        InvestmentOpportunityBalanceIsNotEnough,
        InvestmentOpportunityFailedToReserveAsset,
        InvestmentOpportunityAssetAmountMustBePositive,
        InvestmentOpportunitySecurityTokenNotSpecified,
        InvestmentOpportunityNotFound,
        InvestmentOpportunityShouldBeInactive,
        InvestmentOpportunityShouldBeStarted,
        InvestmentOpportunityShouldBeActive,
        InvestmentOpportunityExpirationWrongState,
        InvestmentOpportunityWrongAssetId,
        InvestmentOpportunityCapDifferentAssets,
        InvestmentOpportunityTooMuchShares,

        // Possible errors when DAO tries to invest to an opportunity
        InvestingNotFound,
        InvestingNotActive,
        InvestingNotEnoughFunds,
        InvestingWrongAsset,

        ContractAgreementNoParties,
        ContractAgreementStartTimeMustBeLaterOrEqualCurrentMoment,
        ContractAgreementEndTimeMustBeLaterStartTime,
        ContractAgreementAlreadyExists,
        ContractAgreementFeeMustBePositive,
        ContractAgreementLicenseTwoPartiesRequired,
        ContractAgreementLicenseProjectTeamIsNotListedInParties,
        ContractAgreementNotFound,
        ContractAgreementWrongAgreement,
        ContractAgreementAlreadyAccepted,
        ContractAgreementLicensePartyIsNotLicenser,
        ContractAgreementLicensePartyIsNotLicensee,
        ContractAgreementLicenseExpired,
        ContractAgreementLicenseNotEnoughBalance,
        ContractAgreementLicenseFailedToChargeFee,
        ContractAgreementLicenseIsNotActive,
        ContractAgreementPartyIsNotListed,
        ContractAgreementAlreadyAcceptedByParty,
        ContractAgreementRejected,
    }
}

decl_storage! {
    trait Store for Module<T: Config> as Deip {
        ProjectMap: map hasher(identity) ProjectId => ProjectOf<T>;

        ProjectIdByTeamId: double_map hasher(blake2_128_concat) AccountIdOf<T>, hasher(identity) ProjectId => ();

        SimpleCrowdfundingMap: map hasher(identity) InvestmentId => SimpleCrowdfundingOf<T>;

        /// Contains various contributions from DAOs
        InvestmentMap: map hasher(identity) InvestmentId => Vec<(T::AccountId, InvestmentOf<T>)>;

        ProjectContentMap: map hasher(identity) ProjectContentId => ProjectContentOf<T>;
        ContentIdByProjectId: double_map hasher(identity) ProjectId, hasher(identity) ProjectContentId => ();

        /// NDA list, guarantees uniquest and provides NDA listing
        Ndas get(fn nda_list): Vec<(ProjectId, T::AccountId)>;
        /// Map to NDA Info
        NdaMap get(fn nda): map hasher(identity) NdaId => NdaOf<T>;

        /// NDA Access Requests list, guarantees uniquest and provides NDA Access Requests listing
        NdaAccessRequests get(fn nda_requests): Vec<(NdaAccessRequestId, NdaId, T::AccountId)>;
        /// Map to NDA Access Requests Info
        NdaAccessRequestMap get(fn nda_request): map hasher(identity) NdaAccessRequestId => NdaAccessRequestOf<T>;

        ReviewMap: map hasher(identity) ReviewId => ReviewOf<T>;

        ReviewIdByProjectId: double_map hasher(identity) ProjectId, hasher(identity) ReviewId => ();
        ReviewIdByContentId: double_map hasher(identity) ProjectContentId, hasher(identity) ReviewId => ();
        ReviewIdByAccountId: double_map hasher(blake2_128_concat) AccountIdOf<T>, hasher(identity) ReviewId => ();

        ReviewVoteMap: map hasher(blake2_128_concat) (ReviewId, AccountIdOf<T>, DomainId) => DeipReviewVoteOf<T>;

        VoteIdByReviewId: double_map hasher(identity) ReviewId, hasher(blake2_128_concat) (ReviewId, AccountIdOf<T>, DomainId) => ();
        VoteIdByAccountId: double_map hasher(blake2_128_concat) AccountIdOf<T>, hasher(blake2_128_concat) (ReviewId, AccountIdOf<T>, DomainId) => ();

        // The set of all Domains.
        Domains get(fn domains) config(): map hasher(blake2_128_concat) DomainId => Domain;
        // The total number of domains stored in the map.
        // Because the map does not store its size, we must store it separately
        DomainCount get(fn domain_count) config(): u32;

        ContractAgreementMap: map hasher(blake2_128_concat) ContractAgreementId => ContractAgreementOf<T>;
        ContractAgreementIdByType: double_map hasher(twox_64_concat) ContractAgreementIndexTerms, hasher(blake2_128_concat) ContractAgreementId => ();
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        /// Allow a user to create project.
        ///
        /// The origin for this call must be _Signed_.
        ///
        /// - `project`: [Project](./struct.Project.html) to be created.
        #[weight = {
            let d = domains.len() as u32;
            T::DeipWeightInfo::create_project(d)
        }]
        fn create_project(origin,
            is_private: bool,
            external_id: ProjectId,
            team_id: T::DeipAccountId,
            description: T::Hash,
            domains: Vec<DomainId>
        ) {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let account = ensure_signed(origin)?;

            let project = ProjectOf::<T> {
                is_private,
                external_id,
                team_id: team_id.into(),
                description,
                domains
            };

            ensure!(account == project.team_id, Error::<T>::NoPermission);

            for domain in &project.domains {
                ensure!(Domains::contains_key(&domain), Error::<T>::DomainNotExists);
            }

            ensure!(!ProjectMap::<T>::contains_key(project.external_id), Error::<T>::ProjectAlreadyExists);

            ProjectMap::<T>::insert(project.external_id, project.clone());
            ProjectIdByTeamId::<T>::insert(project.team_id.clone(), project.external_id, ());

            Self::deposit_event(RawEvent::ProjectCreated(account, project));
        }

        /// Allows DAO to create an investment opportunity.
        ///
        /// The origin for this call must be _Signed_.
        ///
        /// - `external_id`: id of the sale. Must be unique.
        /// - `project_id`: id of the project which tokens are intended to sale.
        /// - `investment_type`: specifies type of created investment opportunity. For possible
        /// variants and details see [`FundingModel`].
        #[weight = {
            let s = shares.len() as u32;
            T::DeipWeightInfo::create_investment_opportunity(s)
        }]
        fn create_investment_opportunity(origin,
            external_id: InvestmentId,
            creator: T::DeipAccountId,
            shares: Vec<DeipAssetOf<T>>,
            funding_model: FundingModelOf<T>,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            Self::create_investment_opportunity_impl(account, external_id, creator.into(), shares, funding_model)
        }

        #[weight = {
            T::DeipWeightInfo::activate_crowdfunding()
        }]
        fn activate_crowdfunding(origin, sale_id: InvestmentId) -> DispatchResult {
            ensure_none(origin)?;
            Self::activate_crowdfunding_impl(sale_id)
        }

        #[weight = {
            T::DeipWeightInfo::expire_crowdfunding_already_expired()
                .max(T::DeipWeightInfo::expire_crowdfunding())
        }]
        fn expire_crowdfunding(origin, sale_id: InvestmentId) -> DispatchResultWithPostInfo {
            ensure_none(origin)?;
            Self::expire_crowdfunding_impl(sale_id)
        }

        #[weight = {
            T::DeipWeightInfo::finish_crowdfunding()
        }]
        fn finish_crowdfunding(origin, sale_id: InvestmentId) -> DispatchResult {
            ensure_none(origin)?;
            Self::finish_crowdfunding_impl(sale_id)
        }

        /// Allows DAO to invest to an opportunity.
        ///
        /// The origin for this call must be _Signed_.
        ///
        /// - `id`: identifier of the investment opportunity
        /// - `amount`: amount of units to invest. The account should have enough funds on
        ///     the balance. This amount is reserved until the investment finished or expired
        #[weight = {
            T::DeipWeightInfo::invest()
                .max(T::DeipWeightInfo::invest_hard_cap_reached())
        }]
        fn invest(origin,
            id: InvestmentId,
            asset: DeipAssetOf<T>
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin)?;
            Self::invest_to_crowdfunding_impl(account, id, asset)
        }

        /// Allow a user to update project.
        ///
        /// The origin for this call must be _Signed_.
        ///
        /// - `project_id`: [Project]((./struct.Project.html)) identifier (external_id) to be updated
        /// - `description`: Optional. Hash of description
        /// - `is_private`: Optional.  Determine visible project or not
        #[weight = {
            T::DeipWeightInfo::update_project()
        }]
        fn update_project(origin, project_id: ProjectId, description: Option<T::Hash>, is_private: Option<bool>) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let account = ensure_signed(origin)?;

            ProjectMap::<T>::mutate_exists(project_id, |maybe_project| -> DispatchResult {
                let project = maybe_project.as_mut().ok_or(Error::<T>::NoSuchProject)?;

                ensure!(project.team_id == account, Error::<T>::NoPermission);

                // TODO make sure that we don't lose first 2 bytes of the hash
                if let Some(value) = description  {
                    project.description = value;
                }

                if let Some(value) = is_private  {
                    project.is_private = value;
                }

                Ok(())
            })?;

            // Emit an event that the project was updated.
            Self::deposit_event(RawEvent::ProjectUpdated(account, project_id));

            Ok(())
        }

        /// Allow a user to create project content.
        ///
        /// The origin for this call must be _Signed_.
        ///
        /// - `content`: [Content](./struct.ProjectContent.html) to be created
        #[weight = {
            let _a = authors.len() as u32;
            let r = references.as_ref().map(|x| x.len()).unwrap_or(0) as u32;
            T::DeipWeightInfo::create_project_content(_a, r)
        }]
        fn create_project_content(origin,
            external_id: ProjectContentId,
            project_external_id: ProjectId,
            team_id: T::DeipAccountId,
            content_type: ProjectContentType,
            description: T::Hash,
            content: T::Hash,
            authors: Vec<T::DeipAccountId>,
            references: Option<Vec<ProjectContentId>>
        ) {
            let account = ensure_signed(origin)?;

            let content = ProjectContentOf::<T> {
                external_id,
                project_external_id,
                team_id: team_id.into(),
                content_type,
                description,
                content,
                authors: authors.into_iter().map(Into::into).collect(),
                references
            };

            ensure!(!ProjectContentMap::<T>::contains_key(&content.external_id), Error::<T>::ProjectContentAlreadyExists);

            let project = ProjectMap::<T>::get(content.project_external_id);

            ensure!(!project.external_id.is_zero(), Error::<T>::NoSuchProject);
            ensure!(project.team_id == content.team_id, Error::<T>::ProjectNotBelongToTeam);
            ensure!(!Self::is_project_finished(&project.external_id), Error::<T>::ProjectAlreadyFinished);

            if let Some(references) = &content.references {
                let is_all_references_exists = references
                    .iter()
                    .all(|&reference| ProjectContentMap::<T>::contains_key(reference));

                ensure!(is_all_references_exists, Error::<T>::NoSuchReference);
            }

            ProjectContentMap::<T>::insert(content.external_id, content.clone());
            ContentIdByProjectId::insert(content.project_external_id, content.external_id, ());

            Self::deposit_event(RawEvent::ProjectContnetCreated(account, content.external_id));
        }

        // /// Allow a user to create [NDA](./struct.Nda.html).
        // ///
        // /// The origin for this call must be _Signed_.
        // ///
        // /// - `end_date`: Unix Timestamp. Exparation date of contract
        // /// - `contract_hash`: Hash of the contract
        // /// - `maybe_start_date`: Optional. Unix Timestamp. Entry into force of the contract
        // /// - `parties`: List of involved Parties
        // /// - `projects`: List of involved Projects
        // #[weight = {
        //     let p = parties.len() as u32;
        //     T::DeipWeightInfo::create_project_nda(p)
        // }]
        // fn create_project_nda(origin,
        //     external_id: NdaId,
        //     end_date: T::Moment,
        //     contract_hash: T::Hash,
        //     maybe_start_date: Option<T::Moment>,
        //     parties: Vec<T::DeipAccountId>,
        //     projects: Vec<ProjectId>
        // ) {
        //     let mut projects = projects;
        //     projects.dedup();
        //     let contract_creator = ensure_signed(origin)?;
        //     let timestamp = pallet_timestamp::Pallet::<T>::get();
        //
        //     ensure!(end_date > timestamp, Error::<T>::NdaEndDateMustBeLaterCurrentMoment);
        //
        //     ensure!(projects.len() <= T::MaxNdaParties::get() as usize, Error::<T>::TooMuchNdaParties);
        //     ensure!(parties.len() <= T::MaxNdaParties::get() as usize, Error::<T>::TooMuchNdaParties);
        //
        //     if let Some(start_date) = maybe_start_date {
        //         ensure!(start_date >= timestamp, Error::<T>::NdaStartDateMustBeLaterOrEqualCurrentMoment);
        //         ensure!(end_date > start_date, Error::<T>::NdaStartDateMustBeLessThanEndDate);
        //     }
        //
        //     let parties: Vec<T::AccountId> = parties.into_iter().map(Into::into).collect();
        //
        //     projects.iter()
        //         .try_for_each(|id| -> DispatchResult {
        //             let project = ProjectMap::<T>::get(id);
        //
        //             ensure!(!project.external_id.is_zero(), Error::<T>::NoSuchProject);
        //             ensure!(parties.contains(&project.team_id), Error::<T>::TeamOfAllProjectsMustSpecifiedAsParty);
        //
        //             Ok(())
        //         })?;
        //
        //     let mut nda_list = Ndas::<T>::get();
        //
        //     let index_to_insert_nda = nda_list.binary_search_by_key(&external_id, |&(external_id, ..)| external_id)
        //         .err().ok_or(Error::<T>::NdaAlreadyExists)?;
        //
        //
        //     let nda = Nda {
        //         contract_creator: contract_creator.clone(),
        //         external_id,
        //         end_date,
        //         start_date: maybe_start_date,
        //         contract_hash,
        //         parties,
        //         projects
        //     };
        //
        //     nda_list.insert(index_to_insert_nda, (nda.external_id, contract_creator.clone()));
        //     Ndas::<T>::put(nda_list);
        //
        //     NdaMap::<T>::insert(nda.external_id, nda);
        //
        //     // Emit an event that the NDA was created.
        //     Self::deposit_event(RawEvent::NdaCreated(contract_creator, external_id));
        //
        // }


        // /// Create [request](./struct.NdaAccessRequest.html) to access NDA content
        // ///
        // /// The origin for this call must be _Signed_.
        // ///
        // /// - `external_id`: Reference for external world and uniques control
        // /// - `nda_external_id`: Reference to NDA
        // /// - `encrypted_payload_hash`: Payload witch need to be decrypted
        // /// - `encrypted_payload_iv`: IV of encrypted payload
        // #[weight = {
        //     T::DeipWeightInfo::create_nda_content_access_request()
        // }]
        // fn create_nda_content_access_request(
        //     origin,
        //     external_id: NdaAccessRequestId,
        //     nda_external_id: NdaId,
        //     encrypted_payload_hash: T::Hash,
        //     encrypted_payload_iv: Vec<u8>,
        // ) {
        //     let account = ensure_signed(origin)?;
        //     let timestamp = pallet_timestamp::Pallet::<T>::get();
        //
        //     let nda = NdaMap::<T>::get(nda_external_id);
        //
        //     ensure!(!nda.external_id.is_zero(), Error::<T>::NoSuchNda);
        //     ensure!(nda.start_date <= Some(timestamp), Error::<T>::NdaContractIsNotActiveYet);
        //
        //     let mut nda_requests = NdaAccessRequests::<T>::get();
        //
        //     let index_to_insert_nda_request = nda_requests.binary_search_by_key(&external_id, |&(external_id, ..)| external_id)
        //         .err().ok_or(Error::<T>::NdaAccessRequestAlreadyExists)?;
        //
        //     let nda_request = NdaAccessRequest {
        //         external_id,
        //         nda_external_id,
        //
        //         requester: account.clone(),
        //         encrypted_payload_hash,
        //         encrypted_payload_iv,
        //         status: NdaAccessRequestStatus::Pending,
        //         grantor: None,
        //         encrypted_payload_encryption_key: None,
        //         proof_of_encrypted_payload_encryption_key: None,
        //     };
        //     nda_requests.insert(index_to_insert_nda_request, (external_id, nda_external_id, account.clone()));
        //     NdaAccessRequests::<T>::put(nda_requests);
        //
        //     NdaAccessRequestMap::<T>::insert(nda_request.external_id, nda_request);
        //
        //     // Emit an event that the NDA was created.
        //     Self::deposit_event(RawEvent::NdaAccessRequestCreated(account, external_id));
        //
        //
        // }

        // /// Fulfill NDA access request
        // ///
        // /// The origin for this call must be _Signed_.
        // ///
        // /// - `external_id`: Reference for external world and uniques control
        // /// - `encrypted_payload_encryption_key`: Ecrypted key witch can decrypt payload
        // /// - `proof_of_encrypted_payload_encryption_key`: Proof that requester has access to the encrypted data with his key
        // #[weight = {
        //     T::DeipWeightInfo::fulfill_nda_content_access_request()
        // }]
        // fn fulfill_nda_content_access_request(
        //     origin,
        //     external_id: NdaAccessRequestId,
        //     encrypted_payload_encryption_key: Vec<u8>,
        //     proof_of_encrypted_payload_encryption_key: Vec<u8>,
        // ) {
        //     let account = ensure_signed(origin)?;
        //
        //     NdaAccessRequestMap::<T>::mutate_exists(external_id, |maybe_nda_access_request| -> DispatchResult {
        //         let mut nda_access_request = maybe_nda_access_request.as_mut().ok_or(Error::<T>::NoSuchNdaAccessRequest)?;
        //
        //         ensure!(nda_access_request.status == NdaAccessRequestStatus::Pending, Error::<T>::NdaAccessRequestAlreadyFinalized);
        //         ensure!(NdaMap::<T>::contains_key(nda_access_request.nda_external_id), Error::<T>::NoSuchNda);
        //
        //         nda_access_request.status = NdaAccessRequestStatus::Fulfilled;
        //         nda_access_request.grantor = Some(account.clone());
        //         nda_access_request.encrypted_payload_encryption_key = Some(encrypted_payload_encryption_key);
        //         nda_access_request.proof_of_encrypted_payload_encryption_key = Some(proof_of_encrypted_payload_encryption_key);
        //
        //         Ok(())
        //     })?;
        //
        //     // Emit an event that the NDA was fulfilled.
        //     Self::deposit_event(RawEvent::NdaAccessRequestFulfilled(account, external_id));
        //
        // }

        // /// Reject NDA access request
        // ///
        // /// The origin for this call must be _Signed_.
        // ///
        // /// - `external_id`: Reference for external world and uniques control
        // #[weight = {
        //     T::DeipWeightInfo::reject_nda_content_access_request()
        // }]
        // fn reject_nda_content_access_request(
        //      origin,
        //      external_id: NdaAccessRequestId,
        //  ) {
        //      let account = ensure_signed(origin)?;
        //
        //      NdaAccessRequestMap::<T>::mutate_exists(external_id, |maybe_nda_access_request| -> DispatchResult {
        //         let mut nda_access_request = maybe_nda_access_request.as_mut().ok_or(Error::<T>::NoSuchNdaAccessRequest)?;
        //
        //
        //         ensure!(nda_access_request.status == NdaAccessRequestStatus::Pending, Error::<T>::NdaAccessRequestAlreadyFinalized);
        //         ensure!(NdaMap::<T>::contains_key(nda_access_request.nda_external_id), Error::<T>::NoSuchNda);
        //
        //         nda_access_request.status = NdaAccessRequestStatus::Rejected;
        //
        //         Ok(())
        //      })?;
        //
        //      // Emit an event that the NDA was rejected.
        //      Self::deposit_event(RawEvent::NdaAccessRequestRejected(account, external_id));
        //
        // }

        /// Allow a user to create review.
        ///
        /// The origin for this call must be _Signed_.
        ///
        /// - `review`: [Review](./struct.Review.html) to be created
        #[weight = {
            let d = domains.len() as u32;
            T::DeipWeightInfo::create_review(d)
        }]
        fn create_review(origin,
            external_id: ReviewId,
            author: T::DeipAccountId,
            content: T::Hash,
            domains: Vec<DomainId>,
            assessment_model: u32,
            weight: Vec<u8>,
            project_content_external_id: ProjectContentId,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            Self::create_review_impl(account, external_id, author, content, domains, assessment_model, weight, project_content_external_id)
        }

        /// Allows DAO to vote for a review.
        ///
        /// The origin for this call must be _Signed_.
        #[weight = {
            T::DeipWeightInfo::upvote_review()
        }]
        fn upvote_review(origin,
            review_id: ReviewId,
            domain_id: DomainId,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            Self::upvote_review_impl(account, review_id, domain_id)
        }

        /// Allows DAO to create a contract agreement between parties.
        ///
        /// The origin for this call must be _Signed_.
        /// - `creator` - creator of the contract agreement. A contract can be created by
        ///     a thirdparty
        /// - `parties` - signatures from all parties must be collected in order
        ///     to consider the contract as approved
        /// - `hash` - hash of contract agreement offchain metadata
        /// - `activation_time`/`expiration_time`
        /// - `terms` - specifies type of the contract agreement. For details see [`ContractAgreementTerms`].
        #[weight = {
            T::DeipWeightInfo::create_contract_agreement_project_license()
                .max(T::DeipWeightInfo::create_contract_agreement_general_contract())
        }]
        fn create_contract_agreement(origin,
            id: ContractAgreementId,
            creator: T::DeipAccountId,
            parties: Vec<T::DeipAccountId>,
            hash: HashOf<T>,
            activation_time: Option<MomentOf<T>>,
            expiration_time: Option<MomentOf<T>>,
            terms: ContractAgreementTermsOf<T>,
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin)?;
            let parties = parties.into_iter().map(Into::into).collect();
            Self::create_contract_agreement_impl(account, id, creator.into(), parties, hash, activation_time, expiration_time, terms)
        }

        /// Allows a party to sign the contract agreement created earlier.
        ///
        /// The origin for this call must be _Signed_.
        /// - `id` - identifies the contract to accept. Check [`ContractAgreementTerms`] for
        ///     supported types
        #[weight = {
            T::DeipWeightInfo::accept_contract_agreement_project_license_unsigned()
                .max(T::DeipWeightInfo::accept_contract_agreement_project_license_signed_by_licenser())
                .max(T::DeipWeightInfo::accept_contract_agreement_general_contract_partially_accepted())
                .max(T::DeipWeightInfo::accept_contract_agreement_general_contract_finalized())
        }]
        fn accept_contract_agreement(origin,
            id: ContractAgreementId,
            party: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin)?;
            Self::accept_contract_agreement_impl(account, id, party.into())
        }

        /// Allows a party to reject the contract agreement created earlier.
        /// Contract makes a transition to the `Rejected` state and cannot be
        /// accepted by remaining parties anymore.
        ///
        /// The origin for this call must be _Signed_.
        /// - `id` - identifies the contract to accept. Check [`ContractAgreementTerms`] for
        ///     supported types
        #[weight = 10_000]
        fn reject_contract_agreement(origin,
            id: ContractAgreementId,
            party: T::DeipAccountId,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            Self::reject_contract_agreement_impl(account, id, party.into())
        }

        fn offchain_worker(_n: T::BlockNumber) {
            if !sp_io::offchain::is_validator() {
                return;
            }

            Self::process_investment_opportunities_offchain();
        }
    }
}

impl<T: Config> ValidateUnsigned for Module<T> {
    type Call = Call<T>;

    /// Validate unsigned call to this module.
    ///
    /// By default unsigned transactions are disallowed, but implementing the validator
    /// here we make sure that some particular calls (the ones produced by offchain worker)
    /// are being whitelisted and marked as valid.
    fn validate_unsigned(source: TransactionSource, call: &Self::Call) -> TransactionValidity {
        // Firstly let's check that we get the local transaction.
        if !matches!(source, TransactionSource::Local | TransactionSource::InBlock) {
            return InvalidTransaction::Custom(NON_LOCAL).into()
        }

        match call {
            Call::activate_crowdfunding { sale_id: id } => {
                let sale = SimpleCrowdfundingMap::<T>::try_get(id)
                    .map_err(|_| InvalidTransaction::Stale)?;
                if !matches!(sale.status, SimpleCrowdfundingStatus::Inactive) {
                    return InvalidTransaction::Stale.into()
                }

                ValidTransaction::with_tag_prefix("DeipOffchainWorker")
                    .propagate(false)
                    .longevity(5)
                    .and_provides((b"activate_crowdfunding", *id))
                    .build()
            },
            Call::expire_crowdfunding { sale_id: id } => {
                let sale = SimpleCrowdfundingMap::<T>::try_get(id)
                    .map_err(|_| InvalidTransaction::Stale)?;
                if !matches!(sale.status, SimpleCrowdfundingStatus::Active) {
                    return InvalidTransaction::Stale.into()
                }

                ValidTransaction::with_tag_prefix("DeipOffchainWorker")
                    .propagate(false)
                    .longevity(5)
                    .and_provides((b"expire_crowdfunding", *id))
                    .build()
            },
            Call::finish_crowdfunding { sale_id: id } => {
                let sale = SimpleCrowdfundingMap::<T>::try_get(id)
                    .map_err(|_| InvalidTransaction::Stale)?;
                if !matches!(sale.status, SimpleCrowdfundingStatus::Active) {
                    return InvalidTransaction::Stale.into()
                }

                ValidTransaction::with_tag_prefix("DeipOffchainWorker")
                    .propagate(false)
                    .longevity(5)
                    .and_provides((b"finish_crowdfunding", *id))
                    .build()
            },
            _ => InvalidTransaction::Call.into(),
        }
    }
}

impl<T: Config> Module<T> {
    fn is_project_finished(project_id: &ProjectId) -> bool {
        ContentIdByProjectId::iter_prefix(project_id)
            .map(|(k, _)| ProjectContentMap::<T>::get(k))
            .any(|c| c.content_type == ProjectContentType::FinalResult)
    }

    pub fn get_project(project_id: &ProjectId) -> Option<ProjectOf<T>> {
        ProjectMap::<T>::try_get(project_id).ok()
    }

    pub fn try_get_project_team(id: &ProjectId) -> Option<AccountIdOf<T>> {
        match ProjectMap::<T>::try_get(*id) {
            Err(_) => None,
            Ok(project) => Some(project.team_id),
        }
    }

    pub fn get_domain(domain_id: &DomainId) -> Option<Domain> {
        Domains::try_get(domain_id).ok()
    }

    pub fn get_project_content(id: &ProjectContentId) -> Option<ProjectContentOf<T>> {
        ProjectContentMap::<T>::try_get(id).ok()
    }

    pub fn get_nda(nda_id: &NdaId) -> Option<NdaOf<T>> {
        NdaMap::<T>::try_get(nda_id).ok()
    }

    pub fn get_review(id: &ReviewId) -> Option<ReviewOf<T>> {
        ReviewMap::<T>::try_get(id).ok()
    }

    pub fn get_investment_opportunity(id: &InvestmentId) -> Option<SimpleCrowdfundingOf<T>> {
        SimpleCrowdfundingMap::<T>::try_get(id).ok()
    }

    pub fn get_contract_agreement(id: &ContractAgreementId) -> Option<ContractAgreementOf<T>> {
        ContractAgreementMap::<T>::try_get(id).ok()
    }

    // /// Allow a user to create domains.
    // ///
    // /// The origin for this call must be _Signed_.
    // ///
    // /// - `project`: [Domain](./struct.Domain.html) to be created.
    // #[weight = {
    //     T::DeipWeightInfo::add_domain()
    // }]
    #[allow(dead_code)]
    fn add_domain(origin: OriginFor<T>, domain: Domain) -> DispatchResult {
        let account = ensure_signed(origin)?;

        let domain_count = DomainCount::get();
        ensure!(domain_count < MAX_DOMAINS, Error::<T>::DomainLimitReached);

        let external_id = domain.external_id;

        // We don't want to add duplicate domains, so we check whether the potential new
        // domain is already present in the list. Because the domains is stored as a hash
        // map this check is constant time O(1)
        ensure!(!Domains::contains_key(&external_id), Error::<T>::DomainAlreadyExists);

        // Insert the new domin and emit the event
        Domains::insert(&external_id, domain);
        DomainCount::put(domain_count + 1); // overflow check not necessary because of maximum

        Self::deposit_event(RawEvent::DomainAdded(account, external_id));
        Ok(())
    }
}
