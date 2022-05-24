// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod impl_fungibles;
#[cfg(test)]
mod mock;
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
                nonfungibles::{
                    Create as NftCreate, Inspect as NftInspect, Mutate as NftMutate,
                    Transfer as NftTransfer,
                },
            },
            BalanceStatus, ReservableCurrency,
        },
        Blake2_128Concat, Parameter,
    };
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};
    use scale_info::TypeInfo;

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
        Created { id: T::FNftId, class: T::ClassId, instance: T::InstanceId },
        TokenAssetCreated { id: T::FNftId, token: T::AssetId },
        TokenAssetMinted { id: T::FNftId, token: T::AssetId, amount: T::FungibleBalance },
        Fractionalized { id: T::FNftId },
        Fused { id: T::FNftId },
        TokenAssetBurned { id: T::FNftId, token: T::AssetId, amount: T::FungibleBalance },
        TokenAssetDestroyed { id: T::FNftId, token: T::AssetId },
        Destroyed { id: T::FNftId },
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
        TokenAssetNotDestroyed,
        /// NFT asset doesn’t exist (or somehow has no owner).
        UnknownClassInstance,
    }

    #[pallet::storage]
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

            T::NonFungible::lock(&account, (class, instance)).unwrap();

            let deposit = T::FNftDeposit::get();
            T::Currency::reserve(&account, deposit)?;

            let details = FNftDetails {
                owner: account.clone(),
                issuer: account.clone(),
                admin: account.clone(),
                freezer: account,
                deposit,
                is_frozen: false,
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

                T::Fungible::unlock_mint(&account, token_id).unwrap();

                T::Fungible::mint_into(token_id, &account, amount)?;

                T::Fungible::lock(&account, token_id).unwrap();

                details.amount = amount;

                Ok(Default::default())
            })?;

            let event = Event::TokenAssetMinted { id, token, amount };
            Self::deposit_event(event);

            Ok(())
        }

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
                T::Fungible::unlock_transfer(&account, token).unwrap();

                Ok(())
            })?;

            let event = Event::Fractionalized { id };
            Self::deposit_event(event);
            Ok(())
        }

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

                T::Fungible::lock(&account, token).unwrap();

                details.is_fractionalized = false;
                Ok(())
            })?;

            let event = Event::Fused { id };
            Self::deposit_event(event);
            Ok(())
        }

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

                    T::Fungible::unlock(&account, token_id).unwrap();

                    T::Fungible::burn_from(token_id, &account, amount)?;

                    T::Fungible::lock(&account, token_id).unwrap();

                    details.amount = Zero::zero();

                    Ok((token_id, amount))
                })?;

            let event = Event::TokenAssetBurned { id, token, amount };
            Self::deposit_event(event);

            Ok(())
        }

        // @TODO somehow to figure out how to include Fungible::destroy weight.
        #[pallet::weight(T::WeightInfo::destroy_token_asset())]
        pub fn destroy_token_asset(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::FNftId,
            witness: <T::Fungible as Destroy<T::AccountId>>::DestroyWitness,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            FNft::<T, I>::mutate(id, |details| -> DispatchResult {
                let details = details.as_mut().ok_or(Error::<T, I>::FNftIdNotFound)?;
                ensure!(account == details.owner, Error::<T, I>::WrongOwner);
                ensure!(!details.is_fractionalized, Error::<T, I>::NftIsFractionalized);
                ensure!(details.amount.is_zero(), Error::<T, I>::TokenAssetNotBurned);

                let token = details.token.ok_or(Error::<T, I>::UnknownToken)?;

                T::Fungible::unlock(&account, token).unwrap();

                let witness = T::Fungible::destroy(token, witness, Some(account))?;

                details.token = None;
                NftClassInstanceToFtAssetId::<T, I>::remove(details.class, details.instance);

                let event = Event::TokenAssetDestroyed { id, token };
                Self::deposit_event(event);

                Ok(())
            })
        }

        #[pallet::weight(T::WeightInfo::destroy())]
        pub fn destroy(origin: OriginFor<T>, #[pallet::compact] id: T::FNftId) -> DispatchResult {
            let account = ensure_signed(origin)?;
            FNft::<T, I>::mutate_exists(id, |details| {
                if let Some(details) = details {
                    ensure!(account == details.owner, Error::<T, I>::WrongOwner);
                    ensure!(!details.is_fractionalized, Error::<T, I>::NftIsFractionalized);
                    ensure!(details.amount.is_zero(), Error::<T, I>::TokenAssetNotBurned);
                    ensure!(details.token.is_none(), Error::<T, I>::TokenAssetNotDestroyed);
                    T::NonFungible::unlock(&account, (details.class, details.instance)).unwrap();
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
