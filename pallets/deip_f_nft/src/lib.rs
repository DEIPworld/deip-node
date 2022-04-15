// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod types;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::types::{DepositBalanceOf, FNftDetails};
    use codec::HasCompact;
    use frame_support::{
        ensure,
        log::error,
        pallet_prelude::{
            DispatchError, DispatchResult, DispatchResultWithPostInfo, Get, IsType, Member,
            StorageMap,
        },
        sp_runtime::traits::{StaticLookup, Zero},
        traits::ReservableCurrency,
        Blake2_128Concat, Parameter,
    };
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};
    use pallet_deip_assets::pallet_assets::DestroyWitness as AssetsDestroyWitness;

    type Assets<T> = pallet_deip_assets::pallet_assets::Pallet<T>;
    type AssetIdOf<T> = <T as pallet_deip_assets::pallet_assets::Config>::AssetId;
    type AssetsBalanceOf<T> = <T as pallet_deip_assets::pallet_assets::Config>::Balance;
    type CurrencyOf<T, I> = <T as Config<I>>::Currency;

    #[pallet::config]
    pub trait Config<I: 'static = ()>:
        frame_system::Config + pallet_deip_assets::Config + pallet_deip_uniques::Config
    {
        /// The overarching event type.
        type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

        /// Identifier for the F-NFT asset.
        type FNftId: Member + Parameter + Default + Copy + HasCompact;

        /// The currency mechanism, used for paying for reserves.
        type Currency: ReservableCurrency<Self::AccountId>;

        /// The basic amount of funds that must be reserved for an FNft asset.
        #[pallet::constant]
        type FNftDeposit: Get<DepositBalanceOf<Self, I>>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config<I>, I: 'static = ()> {
        Created { id: T::FNftId, class: T::ClassId, instance: T::InstanceId },
        TokenAssetCreated { id: T::FNftId, token: AssetIdOf<T> },
        TokenAssetMinted { id: T::FNftId, token: AssetIdOf<T>, amount: AssetsBalanceOf<T> },
        Fractionalized { id: T::FNftId },
        Fused { id: T::FNftId },
        TokenAssetBurned { id: T::FNftId, token: AssetIdOf<T>, amount: AssetsBalanceOf<T> },
        TokenAssetDestroyed { id: T::FNftId, token: AssetIdOf<T> },
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
        /// NFT must be locked before fractionalization.
        NotLocked,
        /// F-Nft corresponding class is created.
        UnknownClass,
        /// F-Nft corresponding class is not minted and cannot be used for F-Nft.
        ClassIsNotMinted,
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
    }

    #[pallet::storage]
    pub(super) type FNft<T: Config<I>, I: 'static = ()> = StorageMap<
        _,
        Blake2_128Concat,
        T::FNftId,
        FNftDetails<
            T::AccountId,
            DepositBalanceOf<T, I>,
            T::NftClassId,
            T::InstanceId,
            T::AssetsAssetId,
            AssetsBalanceOf<T>,
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
            error!("❗️❗️❗️ ensure class and instance exists @TODO ❗️❗️❗️");
            ensure!(true, Error::<T, I>::UnknownClass);
            error!("❗️❗️❗️ ensure class and instance is minted @TODO ❗️❗️❗️");
            ensure!(true, Error::<T, I>::ClassIsNotMinted);

            error!("❗️❗️❗️ lock NFT @TODO ❗️❗️❗️");

            let deposit = T::FNftDeposit::get();
            CurrencyOf::<T, I>::reserve(&account, deposit)?;

            FNft::<T, I>::insert(
                id,
                FNftDetails {
                    owner: account.clone(),
                    issuer: account.clone(),
                    admin: account.clone(),
                    freezer: account,
                    total_deposit: deposit,
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
            token: AssetIdOf<T>,
            min_balance: T::Balance,
        ) -> DispatchResult {
            let account = ensure_signed(origin.clone())?;

            FNft::<T, I>::mutate(id, |details| -> DispatchResult {
                let details = details.as_mut().ok_or(Error::<T, I>::FNftIdNotFound)?;
                ensure!(account == details.owner, Error::<T, I>::WrongOwner);
                ensure!(!details.is_fractionalized, Error::<T, I>::NftIsFractionalized);
                let admin = <T::Lookup as StaticLookup>::unlookup(account);
                Assets::<T>::create(origin, token, admin, min_balance)?;
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
            amount: T::Balance,
        ) -> DispatchResult {
            let account = ensure_signed(origin.clone())?;
            let token = FNft::<T, I>::mutate(id, |details| -> Result<_, DispatchError> {
                let details = details.as_mut().ok_or(Error::<T, I>::FNftIdNotFound)?;
                ensure!(account == details.owner, Error::<T, I>::WrongOwner);
                ensure!(!details.is_fractionalized, Error::<T, I>::NftIsFractionalized);

                let beneficiary = <T::Lookup as StaticLookup>::unlookup(account);
                let token_id = details.token.ok_or(Error::<T, I>::UnknownToken)?;
                Assets::<T>::mint(origin, token_id, beneficiary, amount)?;

                error!("❗️❗️❗️ lock token asset @TODO ❗️❗️❗️");

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
                ensure!(true, Error::<T, I>::TokenAssetIsNotLocked);

                details.is_fractionalized = true;

                error!("❗️❗️❗️ unlock token asset, but leave protection against burn/destroy @TODO ❗️❗️❗️");

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

                error!("❗️❗️❗️ lock tokens @TODO ❗️❗️❗️");

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
            let account = ensure_signed(origin.clone())?;

            let (token, amount) =
                FNft::<T, I>::mutate(id, |details| -> Result<_, DispatchError> {
                    let details = details.as_mut().ok_or(Error::<T, I>::FNftIdNotFound)?;
                    ensure!(account == details.owner, Error::<T, I>::WrongOwner);
                    ensure!(!details.is_fractionalized, Error::<T, I>::NftIsFractionalized);

                    let who = <T::Lookup as StaticLookup>::unlookup(account);
                    let token_id = details.token.ok_or(Error::<T, I>::UnknownToken)?;
                    let amount = details.amount;
                    Assets::<T>::burn(origin, token_id, who, amount)?;
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
            witness: AssetsDestroyWitness,
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin.clone())?;
            FNft::<T, I>::mutate(id, |details| -> DispatchResultWithPostInfo {
                let details = details.as_mut().ok_or(Error::<T, I>::FNftIdNotFound)?;
                ensure!(account == details.owner, Error::<T, I>::WrongOwner);
                ensure!(!details.is_fractionalized, Error::<T, I>::NftIsFractionalized);
                ensure!(details.amount.is_zero(), Error::<T, I>::TokenAssetNotBurned);

                let token = details.token.ok_or(Error::<T, I>::UnknownToken)?;
                let info = Assets::<T>::destroy(origin, token, witness)?;

                details.token = None;

                let event = Event::TokenAssetDestroyed { id, token };
                Self::deposit_event(event);

                Ok(info)
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
    }
}
