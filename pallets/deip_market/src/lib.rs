#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_support::dispatch::DispatchResult;
use frame_support::{ensure, transactional};
use frame_support::traits::{Currency, ExistenceRequirement, ReservableCurrency};
use frame_system::{ensure_signed, Config as SystemConfig};
use deip_asset_system::*;
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

//mod tests;
pub mod weights;
pub mod benchmarking;

use weights::WeightInfo;

pub use pallet::*;

#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Listing<Account, Value, Time> {
	owner: Account,
	price: Value,
	until: Option<Time>,
}

#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Offer<Account, Value, Time> {
	maker: Account,
	price: Value,
	until: Option<Time>,
}

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as SystemConfig>::AccountId>>::Balance;

pub type ListingOf<T> = Listing<<T as SystemConfig>::AccountId, BalanceOf<T>, <T as SystemConfig>::BlockNumber>;

pub type OfferOf<T> = Offer<<T as SystemConfig>::AccountId, BalanceOf<T>, <T as SystemConfig>::BlockNumber>;

pub type CollectionIdOf<T> = <<T as Config>::Tokens as NFTImplT>::CollectionId;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: ReservableCurrency<Self::AccountId>;

		type Token: Member + Parameter + Default + Copy;

		type Tokens: NFTImplT<
				Fingerprint=Self::Token,
				Account=Self::AccountId,
			>;

		#[pallet::constant]
		type MinOfferPrice: Get<BalanceOf<Self>>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn offers)]
	pub type Offers<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, T::Token, Blake2_128Concat, T::AccountId, OfferOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn listed)]
	pub type Listed<T: Config> =
		StorageMap<_, Blake2_128Concat, T::Token, ListingOf<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The price for a token was changed
		PriceChanged {
			owner: T::AccountId,
			token: T::Token,
			price: Option<BalanceOf<T>>,
		},
		/// Token was sold to a new owner
		Sold {
			owner: T::AccountId,
			buyer: T::AccountId,
			token: T::Token,
			price: BalanceOf<T>,
		},
		/// Token listed on Marketplace
		Listed {
			owner: T::AccountId,
			token: T::Token,
			price: BalanceOf<T>,
		},
		/// Token unlisted on Marketplace
		Unlisted {
			owner: T::AccountId,
			token: T::Token,
		},
		/// Offer was placed on a token
		OfferAdded {
			offerer: T::AccountId,
			token: T::Token,
			price: BalanceOf<T>,
		},
		/// Offer was withdrawn
		OfferWithdrawn {
			offerer: T::AccountId,
			token: T::Token,
		},
		/// Offer was accepted
		OfferAccepted {
			owner: T::AccountId,
			offerer: T::AccountId,
			token: T::Token,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// No permissions for account to interact with NFT
		PermissionDenied,
		/// Token cannot be bought
		TokenNotForSale,
		/// Offer already accepted and cannot withdraw
		CannotWithdrawOffer,
		/// Cannot find token listing information
		NotListed,
		/// Cannot make offer on NFT on own NFT
		CannotOfferOnOwnToken,
		/// Cannot buy NFT that is already owned
		CannotBuyOwnToken,
		/// Offer is unknown
		UnknownOffer,
		/// Cannot list NFT owned by a NFT
		CannotListNftOwnedByNft,
		/// Cannot list a non-existing NFT
		TokenNotFound,
		/// Offer is below the OfferMinimumAmount threshold
		OfferTooLow,
		/// Account cannot offer on a NFT again with an active offer
		AlreadyOffered,
		/// Listing or offer has expired and cannot be bought
		HasExpired,
		/// Price differs from when `buy` was executed
		UnexpectedPrice,
		/// Not possible to list non-transferable NFT
		NonTransferable,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/*#[pallet::weight(10000)]
		#[transactional]
		pub fn list_draft(
			origin: OriginFor<T>,
			
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::is_portal_account(&who), Error::<T>::NoPermission);
			todo!()
		}

		#[pallet::weight(10000)]
		#[transactional]
		pub fn redeem(
			origin: OriginFor<T>,
			voucher: Voucher<T>,
			
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::is_portal_account(&who), Error::<T>::NoPermission);
			todo!()
		}*/

		/// List a token on the Marketplace for purchase. A listing can be cancelled, and is
		/// automatically considered cancelled when a `buy` is executed on top of a given listing.
		/// An NFT that has another NFT as its owner CANNOT be listed. An NFT owned by a NFT must
		/// first be sent to an account before being listed.
		///
		/// Parameters:
		/// - `origin` - Account of owner of the RMRK NFT to be listed
		/// - `token` - Token identifier
		/// - `price` - Price of the token
		/// - `until` - Optional BlockNumber for when the listing expires
		#[pallet::weight(T::WeightInfo::list())]
		#[transactional]
		pub fn list(
			origin: OriginFor<T>,
			token: T::Token,
			price: BalanceOf<T>,
			until: Option<T::BlockNumber>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::check_expired(until)?;
			Self::lock(&token, &who)?;
			Listed::<T>::insert(token, Listing { owner: who.clone(), price, until });
			Self::deposit_event(Event::Listed { owner: who, token, price });
			Ok(())
		}

		/// Unlist NFT on the Marketplace and remove from storage in `Listed`.
		///
		/// Parameters:
		/// - `origin` - Account owner of the listed RMRK NFT
		/// - `token` - Token identifier
		#[pallet::weight(T::WeightInfo::unlist())]
		#[transactional]
		pub fn unlist(
			origin: OriginFor<T>,
			token: T::Token,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let listing = Listed::<T>::take(&token).ok_or_else(|| Error::<T>::NotListed)?;
			if listing.owner != who {
				let owner = Self::owner(&token)
					.ok_or(Error::<T>::TokenNotFound)?;
				ensure!(who == owner, Error::<T>::PermissionDenied);
			} else {
				Self::unlock(&token, &who)?;
			}
			Self::deposit_event(Event::Unlisted { owner: listing.owner, token });
			Ok(())
		}

		/// Buy a listed NFT. Ensure that the NFT is available for purchase and has not recently
		/// been purchased, sent, or burned.
		///
		/// Parameters:
		/// - `origin` - Account of the potential buyer
		/// - `token` - Token identifier
		/// - `value` - Price at which buyer purchased at
		#[pallet::weight(T::WeightInfo::buy())]
		#[transactional]
		pub fn buy(
			origin: OriginFor<T>,
			token: T::Token,
			value: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let owner = Self::owner(&token).ok_or_else(|| Error::<T>::TokenNotFound)?;
			ensure!(who != owner, Error::<T>::CannotBuyOwnToken);
			let listing = Listed::<T>::take(&token).ok_or(Error::<T>::NotListed)?;
			ensure!(listing.owner == owner, Error::<T>::TokenNotForSale);
			Self::check_expired(listing.until)?;
			ensure!(value >= listing.price, Error::<T>::UnexpectedPrice);
			Self::make_transfer(who, owner, token, value)
		}

		/// Make an offer on a NFT for purchase. An offer can be set with an expiration where
		/// the offer can no longer be accepted by the NFT owner
		///
		/// Parameters:
		/// - `origin` - Account of the potential buyer
		/// - `token` - Token identifier
		/// - `price` - Price of the token
		/// - `until` - Optional BlockNumber for when the offer expires
		#[pallet::weight(T::WeightInfo::make_offer())]
		#[transactional]
		pub fn make_offer(
			origin: OriginFor<T>,
			token: T::Token,
			price: BalanceOf<T>,
			until: Option<T::BlockNumber>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::check_expired(until)?;
			ensure!(price >= T::MinOfferPrice::get(), Error::<T>::OfferTooLow);
			ensure!(!Self::has_active_offer(&token, &who), Error::<T>::AlreadyOffered);
			let owner = Self::owner(&token).ok_or(Error::<T>::TokenNotFound)?;
			ensure!(who != owner, Error::<T>::CannotOfferOnOwnToken);
			T::Currency::reserve(&who, price)?;
			let offer = Offer { maker: who.clone(), price, until };
			Offers::<T>::insert(token, who.clone(), offer);
			Self::deposit_event(Event::OfferAdded { offerer: who, token,  price });
			Ok(())
		}

		/// Withdraw an offer on a NFT, such that it is no longer available to be accepted by
		/// the NFT owner
		///
		/// Parameters:
		/// - `origin` - Account that wants to withdraw their offer
		/// - `token` - Token identifier
		#[pallet::weight(T::WeightInfo::withdraw_offer())]
		#[transactional]
		pub fn withdraw_offer(
			origin: OriginFor<T>,
			token: T::Token,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let offer = Offers::<T>::take(&token, &who).ok_or(Error::<T>::UnknownOffer)?;
			ensure!(who == offer.maker, Error::<T>::PermissionDenied);
			T::Currency::unreserve(&offer.maker, offer.price);
			Self::deposit_event(Event::OfferWithdrawn { offerer: offer.maker, token });
			Ok(())
		}

		/// Accept an offer on a RMRK NFT from a potential buyer.
		///
		/// Parameters:
		/// - `origin` - Account of the current owner that is accepting the offerer's offer
		/// - `token` - Token identifier
		/// - `offerer` - Account that made the offer
		#[pallet::weight(T::WeightInfo::accept_offer())]
		#[transactional]
		pub fn accept_offer(
			origin: OriginFor<T>,
			token: T::Token,
			offerer: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(who != offerer, Error::<T>::CannotBuyOwnToken);
			let offer = Offers::<T>::take(&token, &offerer).ok_or(Error::<T>::UnknownOffer)?;
			ensure!(offerer == offer.maker, Error::<T>::UnknownOffer); // should be unreachable
			Self::check_expired(offer.until)?;
			T::Currency::unreserve(&offer.maker, offer.price);
			Self::make_transfer(offer.maker, who.clone(), token, offer.price)?;
			Self::deposit_event(Event::OfferAccepted { owner: who, offerer, token });
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn is_portal_account(account: &T::AccountId) -> bool {
		todo!()
	}

	fn check_expired(time: Option<T::BlockNumber>) -> Result<(), Error<T>> {
		if let Some(t) = time {
			let now = frame_system::Pallet::<T>::block_number();
			if t <= now {
				return Err(Error::<T>::HasExpired);
			}
		}
		Ok(())
	}

	fn make_transfer(buyer: T::AccountId, owner: T::AccountId, token: T::Token, price: BalanceOf<T>) -> DispatchResult {
		Self::unlock(&token, &owner)?;
		T::Currency::transfer(&buyer, &owner, price, ExistenceRequirement::KeepAlive)?;
		Self::change_owner(&token, &owner, &buyer)?;
		Self::deposit_event(Event::Sold { owner, buyer, token, price });
		Ok(())
	}

	fn change_owner(token: &T::Token, from: &T::AccountId, to: &T::AccountId) -> Result<(), Error<T>> {
		transfer_item::<T::Tokens>(token.clone(), from, to)
			.map_err(|_| Error::<T>::NonTransferable)
	}

	fn owner(token: &T::Token) -> Option<T::AccountId> {
		pick_item::<T::Tokens>(token.clone())
			.ok()
			.map(|x| x.account().clone())
	}

	fn lock(token: &T::Token, account: &T::AccountId) -> DispatchResult {
		// TODO
		let owner = Self::owner(&token).ok_or(Error::<T>::TokenNotFound)?;
		ensure!(owner.eq(account), Error::<T>::PermissionDenied);
		Ok(())
	}

	fn unlock(token: &T::Token, account: &T::AccountId) -> DispatchResult {
		// TODO
		let owner = Self::owner(&token).ok_or(Error::<T>::TokenNotFound)?;
		ensure!(owner.eq(account), Error::<T>::PermissionDenied);
		Ok(())
	}

	/// Helper function to check if an account has already submitted an offer
	///
	/// Parameters:
	/// - token: Token identifier
	/// - offerer: The account that may or may not have already sent an offer
	#[inline]
	fn has_active_offer(token: &T::Token, offerer: &T::AccountId) -> bool {
		Offers::<T>::contains_key(token, offerer)
	}
}
