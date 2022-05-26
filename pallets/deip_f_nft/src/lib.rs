//! # Deip F-NFT
//!
//! A pallet for dealing with fractionalization of non-fungible assets.
//!
//! ## Overview
//!
//! The Deip F-NFT pallet provides functionality for management of fractionalized assets.
//!
//! To use it in runtime it is required to implement [`Config`].
//!
//! The supported dispatchable functions are documented in the [`Call`] enum.
//!
//! ### Goals
//!
//! * Provide functionality for fractionalization of a NFT asset.
//! * Ensure lock of an underlying NFT asset during the whole F-NFT life cycle.
//! * Issue/mint fungible token (FT) for fractions.
//! * Allow F-NFTs to be fused if all fractions are collected on owner's account.
//!
//! ## Interface
//!
//! * `create`: Creates new F-NFT /container/, taking the required deposit.
//! * `create_token_asset`: Creates new FT to be used for NFT fractions.
//! * `mint_token_asset`: Mints provided amount of fractions.
//! * `fractionalize`: Unlocks operations with fractions.
//! * `fuse`: Restores original NFT from fractions. But further actions required to unlock it.
//! * `burn_token_asset`: Reduces amount of locked FT fractions to zero.
//! * `release_token_asset`: Uncouples FT asset from F-NFT. To destroy it [`Config::Fungible`] pallet should be used.
//! * `destroy`: Destroys F-NFT /container/, releasing underlying NFT.
//! * `transfer`: Transfers F-NFT /container/ to dest account.
//!
//! Please refer to the [`Call`] enum and its associated variants for documentation on each
//! function.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod impl_fungibles;
pub mod types;
mod weights;

pub use pallet::*;
pub use weights::SubstrateWeight;

#[frame_support::pallet]
pub mod pallet {
    use crate::{
        types::{DepositBalanceOf, FNftDetails},
        weights::WeightInfo,
    };

    use codec::HasCompact;
    use deip_asset_lock::LockableAsset;
    use frame_support::{
        ensure,
        pallet_prelude::{
            DispatchError, DispatchResult, Get, IsType, MaxEncodedLen, MaybeSerializeDeserialize,
            Member, StorageDoubleMap, StorageMap,
        },
        sp_runtime::traits::{AtLeast32BitUnsigned, StaticLookup, Zero},
        traits::{
            tokens::{
                fungibles::{Create, Destroy, Inspect as FtInspect, Mutate as FtMutate},
                nonfungibles::{Inspect as NftInspect, Transfer as NftTransfer},
            },
            BalanceStatus, ReservableCurrency,
        },
        Blake2_128Concat, Parameter,
    };
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};
    use scale_info::TypeInfo;

    #[cfg(feature = "runtime-benchmarks")]
    use frame_support::traits::tokens::nonfungibles::{Create as NftCreate, Mutate as NftMutate};

    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

        /// Identifier for the F-NFT asset.
        type FNftId: Member + Parameter + Default + Copy + HasCompact + MaxEncodedLen;

        /// The currency mechanism, used for paying for reserves.
        type Currency: ReservableCurrency<Self::AccountId>;

        /// The basic amount of funds that must be reserved for an FNft asset.
        #[pallet::constant]
        type FNftDeposit: Get<DepositBalanceOf<Self, I>>;

        /// Identifier for the FT asset.
        type AssetId: Member + Parameter + Default + Copy + HasCompact + MaxEncodedLen;

        /// Identifier for the NFT asset class.
        type ClassId: Member + Parameter + Default + Copy + HasCompact + MaxEncodedLen;

        /// Identifier for the NFT asset instance.
        type InstanceId: Member + Parameter + Default + Copy + HasCompact + MaxEncodedLen;

        /// Witness data for the destroy transactions.
        type DestroyWitness: Parameter + MaxEncodedLen;

        /// The fungible assets mechanism.
        type Fungible: LockableAsset<Self::AccountId, AssetId = Self::AssetId>
            + FtInspect<Self::AccountId, AssetId = Self::AssetId, Balance = Self::FungibleBalance>
            + Create<Self::AccountId>
            + FtMutate<Self::AccountId, AssetId = Self::AssetId, Balance = Self::FungibleBalance>
            + Destroy<
                Self::AccountId,
                AssetId = Self::AssetId,
                Balance = Self::FungibleBalance,
                DestroyWitness = Self::DestroyWitness,
            >;

        /// The non-fungible assets mechanism.
        #[cfg(not(feature = "runtime-benchmarks"))]
        type NonFungible: LockableAsset<Self::AccountId, AssetId = (Self::ClassId, Self::InstanceId)>
            + NftInspect<Self::AccountId, ClassId = Self::ClassId, InstanceId = Self::InstanceId>
            + NftTransfer<Self::AccountId, ClassId = Self::ClassId, InstanceId = Self::InstanceId>;

        /// For benchmarking purposes `Create` trait is required.
        #[cfg(feature = "runtime-benchmarks")]
        type NonFungible: LockableAsset<Self::AccountId, AssetId = (Self::ClassId, Self::InstanceId)>
            + NftInspect<Self::AccountId, ClassId = Self::ClassId, InstanceId = Self::InstanceId>
            + NftCreate<Self::AccountId>
            + NftMutate<Self::AccountId>
            + NftTransfer<Self::AccountId, ClassId = Self::ClassId, InstanceId = Self::InstanceId>;

        /// The units in which `Self::Fungible` records balances.
        type FungibleBalance: Member
            + Parameter
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TypeInfo;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config<I>, I: 'static = ()> {
        /// Some `FNftId` /container/ was created.
        Created { id: T::FNftId, class: T::ClassId, instance: T::InstanceId },
        /// FT for fractions was created.
        TokenAssetCreated { id: T::FNftId, token: T::AssetId },
        /// FT for fractions was minted.
        TokenAssetMinted { id: T::FNftId, token: T::AssetId, amount: T::FungibleBalance },
        /// NFT was fractionalized, fractions were unlocked.
        Fractionalized { id: T::FNftId },
        /// Fractions were locked, NFT was fused.
        Fused { id: T::FNftId },
        /// FT for fractions was burned.
        TokenAssetBurned { id: T::FNftId, token: T::AssetId, amount: T::FungibleBalance },
        /// FT for fractions was decupled from F-NFT.
        TokenAssetReleased { id: T::FNftId, token: T::AssetId },
        /// Some `FNftId` /container/ was destroyed.
        Destroyed { id: T::FNftId },
        /// Some `FNftId` /container/ was transferred.
        Transferred { id: T::FNftId, from: T::AccountId, to: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T, I = ()> {
        /// `FNftId` not found.
        FNftIdNotFound,
        /// Operation can be performed only by owner.
        WrongOwner,
        /// `FNftId` is already taken.
        InUse,
        /// F-Nft corresponding token is not created.
        UnknownToken,
        /// F-Nft corresponding token is already minted and cannot be used for F-Nft.
        TokenIsAlreadyMinted,
        /// Nft is already fractionalized. Some operations are unavailable.
        NftIsFractionalized,
        /// Token asset must be locked before fractionalization.
        /// Token asset should have been locked in `token_asset_mint`.
        TokenAssetIsNotLocked,
        /// Nft is not fractionalized so it cannot be fused.
        NftIsNotFractionalized,
        /// Owner must collect all tokens on it's account for F-NFT to be fused.
        OwnerDoesNotHaveAllTokens,
        /// F-Nft corresponding token should be burned, for eg before `destroy_token_asset`.
        TokenAssetNotBurned,
        /// F-Nft corresponding token should be destroyed before F-NFT destruction.
        TokenAssetNotReleased,
        /// NFT asset doesn’t exist (or somehow has no owner).
        UnknownClassInstance,
    }

    #[pallet::storage]
    /// Details for a F-NFT asset.
    pub(super) type FNft<T: Config<I>, I: 'static = ()> = StorageMap<
        _,
        Blake2_128Concat,
        T::FNftId,
        FNftDetails<
            T::AccountId,
            DepositBalanceOf<T, I>,
            T::ClassId,
            T::InstanceId,
            T::AssetId,
            T::FungibleBalance,
        >,
    >;

    #[pallet::storage]
    /// Id mapping of NFT and fractions.
    pub(super) type NftClassInstanceToFtAssetId<T: Config<I>, I: 'static = ()> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::ClassId,
        Blake2_128Concat,
        T::InstanceId,
        T::AssetId,
    >;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::generate_storage_info]
    pub struct Pallet<T, I = ()>(_);

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        /// Create a new F-NFT /container/ from a public origin with provided NFT.
        ///
        /// The new F-NFT has no fractions initially and its owner is the origin.
        ///
        /// The origin must be Signed and the sender must have sufficient funds free.
        /// The origin must be an owner of the provided NFT.
        ///
        /// NFT must be unlocked according to [`LockableAsset`] trait.
        /// Because further fractionalization requires NFT to be locked by origin in this operation.
        ///
        /// Funds of sender are reserved by `FNftDeposit`.
        ///
        /// Parameters:
        /// - `id`: The identifier of the new F-NFT /container/.
        /// This must not be currently in use to identify an existing F-NFT.
        /// - `class`: The identifier of the class of the NFT to be fractionalized.
        /// - `instance`: The identifier of the instance of the NFT to be fractionalized.
        ///
        /// Emits `Created` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::weight(T::WeightInfo::create())]
        pub fn create(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::FNftId,
            class: T::ClassId,
            instance: T::InstanceId,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;

            ensure!(!FNft::<T, I>::contains_key(id), Error::<T, I>::InUse);

            let owner = T::NonFungible::owner(&class, &instance)
                .ok_or(Error::<T, I>::UnknownClassInstance)?;
            ensure!(account == owner, Error::<T, I>::WrongOwner);

            T::NonFungible::lock(&account, (class, instance))?;

            let deposit = T::FNftDeposit::get();
            T::Currency::reserve(&account, deposit)?;

            let details = FNftDetails {
                owner: account,
                deposit,
                class,
                instance,
                token: None,
                amount: Zero::zero(),
                is_fractionalized: false,
            };
            FNft::<T, I>::insert(id, details);

            let event = Event::Created { id, class, instance };
            Self::deposit_event(event);
            Ok(())
        }

        /// Create a new FT to be used for fractions.
        ///
        /// The origin must be Signed and must be an owner of the provided F-NFT.
        ///
        /// F-NFT must not be fractionalized.
        /// FT `AssetId` must not be currently in use.
        ///
        /// Parameters:
        /// - `id`: The identifier of the existing F-NFT /container/.
        /// - `token`: The identifier of the FT to be created.
        /// - `is_sufficient`: Whether a non-zero balance of the FT is deposit of sufficient
		/// value to account for the state bloat associated with its balance storage.
        /// - `min_balance`: The minimum balance of the FT that any single account must
		/// have. If an account's balance is reduced below this, then it collapses to zero.
        ///
        /// Emits `TokenAssetCreated` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::weight(T::WeightInfo::create_token_asset())]
        pub fn create_token_asset(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::FNftId,
            token: T::AssetId,
            is_sufficient: bool,
            min_balance: T::FungibleBalance,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;

            FNft::<T, I>::mutate(id, |details| -> DispatchResult {
                let details = details.as_mut().ok_or(Error::<T, I>::FNftIdNotFound)?;
                ensure!(account == details.owner, Error::<T, I>::WrongOwner);
                ensure!(!details.is_fractionalized, Error::<T, I>::NftIsFractionalized);
                T::Fungible::create(token, account.clone(), is_sufficient, min_balance)?;

                T::Fungible::lock(&account, token)?;

                details.token = Some(token);
                NftClassInstanceToFtAssetId::<T, I>::insert(details.class, details.instance, token);
                Ok(())
            })?;

            let event = Event::TokenAssetCreated { id, token };
            Self::deposit_event(event);

            Ok(())
        }


        /// Mint the FT to be used for fractions.
        ///
        /// The origin must be Signed and must be an owner of the provided F-NFT.
        ///
        /// F-NFT must not be fractionalized.
        /// FT must not be minted before.
        /// FT `AssetId` must be created with `create_token_asset` operation before minting.
        ///
        /// Parameters:
        /// - `id`: The identifier of the existing F-NFT /container/.
        /// - `amount`: The amount of the FT to be minted.
        ///
        /// Emits `TokenAssetMinted` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::weight(T::WeightInfo::mint_token_asset())]
        pub fn mint_token_asset(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::FNftId,
            amount: T::FungibleBalance,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            let token = FNft::<T, I>::mutate(id, |details| -> Result<_, DispatchError> {
                let details = details.as_mut().ok_or(Error::<T, I>::FNftIdNotFound)?;
                ensure!(account == details.owner, Error::<T, I>::WrongOwner);
                ensure!(!details.is_fractionalized, Error::<T, I>::NftIsFractionalized);
                ensure!(details.amount == Zero::zero(), Error::<T, I>::TokenIsAlreadyMinted);

                let token_id = details.token.ok_or(Error::<T, I>::UnknownToken)?;

                T::Fungible::unlock_mint(&account, token_id)?;

                T::Fungible::mint_into(token_id, &account, amount)?;

                T::Fungible::lock(&account, token_id)?;

                details.amount = amount;

                Ok(Default::default())
            })?;

            let event = Event::TokenAssetMinted { id, token, amount };
            Self::deposit_event(event);

            Ok(())
        }

        /// Creates the full-fledged fractions of the NFT.
        ///
        /// The origin must be Signed and must be an owner of the provided F-NFT.
        ///
        /// F-NFT must not be fractionalized.
        /// FT must be locked. If everything is correct, it should be locked on `mint_token_asset` stage.
        ///
        /// Parameters:
        /// - `id`: The identifier of the existing F-NFT /container/.
        ///
        /// Emits `Fractionalized` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::weight(T::WeightInfo::fractionalize())]
        pub fn fractionalize(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::FNftId,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            FNft::<T, I>::mutate(id, |details| -> DispatchResult {
                let details = details.as_mut().ok_or(Error::<T, I>::FNftIdNotFound)?;
                ensure!(account == details.owner, Error::<T, I>::WrongOwner);
                ensure!(!details.is_fractionalized, Error::<T, I>::NftIsFractionalized);
                ensure!(
                    T::Fungible::is_locked(details.token.ok_or(Error::<T, I>::UnknownToken)?),
                    Error::<T, I>::TokenAssetIsNotLocked
                );

                details.is_fractionalized = true;

                let token = details.token.ok_or(Error::<T, I>::UnknownToken)?;
                T::Fungible::unlock_transfer(&account, token)?;

                Ok(())
            })?;

            let event = Event::Fractionalized { id };
            Self::deposit_event(event);
            Ok(())
        }

        /// Fuses fractions of the NFT into original asset.
        ///
        /// The origin must be Signed and must be an owner of the provided F-NFT.
        ///
        /// F-NFT must be fractionalized.
        /// Balance of the owner account must be equal to fractions total minted amount.
        /// I.e. owner must have all fractions to fuse original NFT.
        ///
        /// Parameters:
        /// - `id`: The identifier of the existing F-NFT /container/.
        ///
        /// Emits `Fused` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::weight(T::WeightInfo::fuse())]
        pub fn fuse(origin: OriginFor<T>, #[pallet::compact] id: T::FNftId) -> DispatchResult {
            let account = ensure_signed(origin)?;

            FNft::<T, I>::mutate(id, |details| -> DispatchResult {
                let details = details.as_mut().ok_or(Error::<T, I>::FNftIdNotFound)?;
                ensure!(account == details.owner, Error::<T, I>::WrongOwner);
                ensure!(details.is_fractionalized, Error::<T, I>::NftIsNotFractionalized);

                let token = details.token.ok_or(Error::<T, I>::UnknownToken)?;
                let balance = T::Fungible::balance(token, &account);
                ensure!(balance == details.amount, Error::<T, I>::OwnerDoesNotHaveAllTokens);

                T::Fungible::lock(&account, token)?;

                details.is_fractionalized = false;
                Ok(())
            })?;

            let event = Event::Fused { id };
            Self::deposit_event(event);
            Ok(())
        }

        /// Burns all fractions of the F-NFT.
        ///
        /// The origin must be Signed and must be an owner of the provided F-NFT.
        ///
        /// F-NFT must not be fractionalized.
        ///
        /// Parameters:
        /// - `id`: The identifier of the existing F-NFT /container/.
        ///
        /// Emits `TokenAssetBurned` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::weight(T::WeightInfo::burn_token_asset())]
        pub fn burn_token_asset(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::FNftId,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;

            let (token, amount) =
                FNft::<T, I>::mutate(id, |details| -> Result<_, DispatchError> {
                    let details = details.as_mut().ok_or(Error::<T, I>::FNftIdNotFound)?;
                    ensure!(account == details.owner, Error::<T, I>::WrongOwner);
                    ensure!(!details.is_fractionalized, Error::<T, I>::NftIsFractionalized);

                    let token_id = details.token.ok_or(Error::<T, I>::UnknownToken)?;
                    let amount = details.amount;

                    T::Fungible::unlock(&account, token_id)?;

                    T::Fungible::burn_from(token_id, &account, amount)?;

                    T::Fungible::lock(&account, token_id)?;

                    details.amount = Zero::zero();

                    Ok((token_id, amount))
                })?;

            let event = Event::TokenAssetBurned { id, token, amount };
            Self::deposit_event(event);

            Ok(())
        }

        /// Releases FT `AssetId` used for fractions.
        /// Destruction of the FT asset class, can be performed via pallet used in [`Config::Fungible`].
        ///
        /// The origin must be Signed and must be an owner of the provided F-NFT.
        /// F-NFT must not be fractionalized.
        /// All FT must be burned to this point.
        ///
        /// Parameters:
        /// - `id`: The identifier of the existing F-NFT /container/.
        ///
        /// Emits `TokenAssetReleased` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::weight(T::WeightInfo::release_token_asset())]
        pub fn release_token_asset(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::FNftId,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            FNft::<T, I>::mutate(id, |details| -> DispatchResult {
                let details = details.as_mut().ok_or(Error::<T, I>::FNftIdNotFound)?;
                ensure!(account == details.owner, Error::<T, I>::WrongOwner);
                ensure!(!details.is_fractionalized, Error::<T, I>::NftIsFractionalized);
                ensure!(details.amount.is_zero(), Error::<T, I>::TokenAssetNotBurned);

                let token = details.token.ok_or(Error::<T, I>::UnknownToken)?;

                T::Fungible::unlock(&account, token)?;

                details.token = None;
                NftClassInstanceToFtAssetId::<T, I>::remove(details.class, details.instance);

                let event = Event::TokenAssetReleased { id, token };
                Self::deposit_event(event);

                Ok(())
            })
        }

        /// Destroys `FNftId` /container/. Completely releases underlying NFT.
        ///
        /// The origin must be Signed and must be an owner of the provided F-NFT.
        /// F-NFT must not be fractionalized.
        /// All FT must be burned to this point.
        /// FT `AssetId` must be released.
        /// 
        /// Funds of owner will be unreserved by `FNftDeposit`.
        ///
        /// Parameters:
        /// - `id`: The identifier of the existing F-NFT /container/.
        ///
        /// Emits `Destroyed` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::weight(T::WeightInfo::destroy())]
        pub fn destroy(origin: OriginFor<T>, #[pallet::compact] id: T::FNftId) -> DispatchResult {
            let account = ensure_signed(origin)?;
            FNft::<T, I>::mutate_exists(id, |details| {
                if let Some(details) = details {
                    ensure!(account == details.owner, Error::<T, I>::WrongOwner);
                    ensure!(!details.is_fractionalized, Error::<T, I>::NftIsFractionalized);
                    ensure!(details.amount.is_zero(), Error::<T, I>::TokenAssetNotBurned);
                    ensure!(details.token.is_none(), Error::<T, I>::TokenAssetNotReleased);
                    T::NonFungible::unlock(&account, (details.class, details.instance))?;
                    T::Currency::unreserve(&account, details.deposit);
                } else {
                    return Err(Error::<T, I>::FNftIdNotFound)
                }
                *details = None;

                Ok(())
            })?;

            let event = Event::Destroyed { id };
            Self::deposit_event(event);

            Ok(())
        }

        /// Moves `FNftId` /container/ with underlying NFT from the sender account to another.
        ///
        /// The origin must be Signed and must be an owner of the provided F-NFT.
        /// F-NFT must be fractionalized.
        /// 
        /// Funds of owner will be unreserved by `FNftDeposit`.
        ///
        /// Parameters:
        /// - `id`: The identifier of the existing F-NFT /container/.
        /// - `dest`: The account F-NFT will be transferred to.
        ///
        /// Emits `Transferred` event when successful.
        ///
        /// Weight: `O(1)`
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::FNftId,
            dest: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            let dest = T::Lookup::lookup(dest)?;

            FNft::<T, I>::mutate(id, |details| -> DispatchResult {
                let details = details.as_mut().ok_or(Error::<T, I>::FNftIdNotFound)?;
                ensure!(account == details.owner, Error::<T, I>::WrongOwner);
                ensure!(details.is_fractionalized, Error::<T, I>::NftIsNotFractionalized);

                T::NonFungible::unlock_transfer(&account, (details.class, details.instance))?;

                let status = BalanceStatus::Reserved;
                T::Currency::repatriate_reserved(&details.owner, &dest, details.deposit, status)?;

                details.owner = dest.clone();
                T::NonFungible::transfer(&details.class, &details.instance, &dest)?;

                T::NonFungible::lock_transfer(&dest, (details.class, details.instance))
            })?;

            let event = Event::Transferred { id, from: account, to: dest };
            Self::deposit_event(event);

            Ok(())
        }
    }

    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        pub(crate) fn class_instance_to_ft_id(
            class: T::ClassId,
            instance: T::InstanceId,
        ) -> Option<T::AssetId> {
            NftClassInstanceToFtAssetId::<T, I>::get(class, instance)
        }
    }
}
