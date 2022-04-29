// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod types;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::types::{DepositBalanceOf, FNftDetails};
    use codec::HasCompact;
    use deip_asset_lock::LockableAsset;
    use frame_support::{
        ensure,
        log::error,
        pallet_prelude::{
            DispatchError, DispatchResult, Get, IsType, MaxEncodedLen, MaybeSerializeDeserialize,
            Member, StorageMap,
        },
        sp_runtime::traits::{AtLeast32BitUnsigned, StaticLookup, Zero},
        traits::{
            tokens::{
                fungibles::{Create, Destroy, Inspect as FtInspect, Mutate},
                nonfungibles::Inspect as NftInspect,
            },
            ReservableCurrency,
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
        type FNftId: Member + Parameter + Default + Copy + HasCompact;

        /// The currency mechanism, used for paying for reserves.
        type Currency: ReservableCurrency<Self::AccountId>;

        /// The basic amount of funds that must be reserved for an FNft asset.
        #[pallet::constant]
        type FNftDeposit: Get<DepositBalanceOf<Self, I>>;

        /// Identifier for the FT asset.
        type AssetId: Member + Parameter + Default + Copy + HasCompact;

        /// Identifier for the NFT asset class.
        type ClassId: Member + Parameter + Default + Copy + HasCompact;

        /// Identifier for the NFT asset instance.
        type InstanceId: Member + Parameter + Default + Copy + HasCompact;

        /// Witness data for the destroy transactions.
        type DestroyWitness: Parameter;

        /// The fungible assets mechanism.
        type Fungible: LockableAsset<Self::AccountId, AssetId = Self::AssetId>
            + Create<Self::AccountId>
            + FtInspect<Self::AccountId, AssetId = Self::AssetId, Balance = Self::FungibleBalance>
            + Mutate<Self::AccountId, AssetId = Self::AssetId, Balance = Self::FungibleBalance>
            + Destroy<
                Self::AccountId,
                AssetId = Self::AssetId,
                Balance = Self::FungibleBalance,
                DestroyWitness = Self::DestroyWitness,
            >;

        /// The non-fungible assets mechanism.
        type NonFungible: LockableAsset<Self::AccountId, AssetId = (Self::ClassId, Self::InstanceId)>
            + NftInspect<Self::AccountId, ClassId = Self::ClassId, InstanceId = Self::InstanceId>;

        /// The units in which `Self::Fungible` records balances.
        type FungibleBalance: Member
            + Parameter
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TypeInfo;
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

    #[pallet::pallet]
    pub struct Pallet<T, I = ()>(_);

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        #[pallet::weight(1)]
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

            FNft::<T, I>::insert(
                id,
                FNftDetails {
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
                },
            );

            let event = Event::Created { id, class, instance };
            Self::deposit_event(event);
            Ok(())
        }

        #[pallet::weight(1)]
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
                T::Fungible::create(token, account, is_sufficient, min_balance)?;

                error!("❗️❗️❗️ protect from minting token asset @TODO ❗️❗️❗️");

                details.token = Some(token);
                Ok(())
            })?;

            let event = Event::TokenAssetCreated { id, token };
            Self::deposit_event(event);

            Ok(())
        }

        #[pallet::weight(1)]
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

                T::Fungible::mint_into(token_id, &account, amount)?;

                T::Fungible::lock(&account, token_id).unwrap();

                details.amount = amount;

                Ok(token_id)
            })?;

            let event = Event::TokenAssetMinted { id, token, amount };
            Self::deposit_event(event);

            Ok(())
        }

        #[pallet::weight(1)]
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

                T::Fungible::unlock(&account, details.token.ok_or(Error::<T, I>::UnknownToken)?)
                    .unwrap();

                error!("❗️❗️❗️ leave protection against burn/destroy @TODO ❗️❗️❗️");

                Ok(())
            })?;

            let event = Event::Fractionalized { id };
            Self::deposit_event(event);
            Ok(())
        }

        #[pallet::weight(1)]
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

        #[pallet::weight(1)]
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
                    T::Fungible::burn_from(token_id, &account, amount)?;

                    details.amount = Zero::zero();

                    Ok((token_id, amount))
                })?;

            let event = Event::TokenAssetBurned { id, token, amount };
            Self::deposit_event(event);

            Ok(())
        }

        #[pallet::weight(1)]
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
                T::Fungible::destroy(token, witness, Some(account))?;

                details.token = None;

                let event = Event::TokenAssetDestroyed { id, token };
                Self::deposit_event(event);

                Ok(())
            })
        }

        #[pallet::weight(1)]
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

        #[pallet::weight(1)]
        pub fn transfer(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::FNftId,
            dest: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            let dest = T::Lookup::lookup(dest)?;

            FNft::<T, I>::mutate_exists(id, |details| -> DispatchResult {
                let details = details.as_mut().ok_or(Error::<T, I>::FNftIdNotFound)?;
                ensure!(account == details.owner, Error::<T, I>::WrongOwner);

                details.owner = dest.clone();
                details.admin = dest.clone();
                details.freezer = dest;

                error!("❗️❗️❗️ add missing checks and transfer logic @TODO ❗️❗️❗️");

                Ok(())
            })?;

            Ok(())
        }
    }
}
