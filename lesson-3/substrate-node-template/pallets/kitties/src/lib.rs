#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Decode, Encode};
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement, Randomness, ReservableCurrency},
		Parameter,
	};
	use frame_system::pallet_prelude::*;
	use sp_io::hashing::blake2_128;
	use sp_runtime::traits::{
		AtLeast32BitUnsigned, Bounded, MaybeDisplay, MaybeMallocSizeOf, MaybeSerializeDeserialize,
		Member, Zero,
	};
	use sp_std::fmt::Debug;

	#[derive(Encode, Decode)]
	pub struct Kitty(pub [u8; 16]);

	//type KittyInde = u32;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: pallet_balances::Config + frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		/// The block number type used by the runtime.
		type KittyIndex: Parameter
			+ Member
			+ MaybeSerializeDeserialize
			+ Debug
			+ MaybeDisplay
			+ AtLeast32BitUnsigned
			+ Default
			+ Bounded
			+ Copy
			+ sp_std::hash::Hash
			+ sp_std::str::FromStr
			+ MaybeMallocSizeOf;
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		#[pallet::constant]
		type Deposit: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type KittiesCount<T: Config> = StorageValue<_, T::KittyIndex>;
	//pub type KittiesCount<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> =
		StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<Kitty>, ValueQuery>;
	//pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyInde, Option<Kitty>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> =
		StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<T::AccountId>, ValueQuery>;
	//StorageMap<_, Blake2_128Concat, KittyInde, Option<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_price)]
	pub type KittyPrice<T: Config> =
		StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<BalanceOf<T>>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyCreate(T::AccountId, T::KittyIndex),
		KittyTransfer(T::AccountId, T::AccountId, T::KittyIndex),
		KittySetPrice(T::AccountId, BalanceOf<T>, T::KittyIndex),
		KittyBuy(T::AccountId, BalanceOf<T>, T::KittyIndex),
		//KittyCreate(T::AccountId, KittyInde),
		//KittyTransfer(T::AccountId, T::AccountId, KittyInde),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		KittiesCountOverflow,
		/// Errors should have helpful documentation associated with them.
		NotOwner,
		SameParentIndex,
		InvalidKittyIndex,
		KittyPriceOverflow,
		KittyNotSale,
		NotEnoughToken,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(0)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			let kitty_id = match Self::kitties_count() {
				Some(id) => {
					ensure!(id != T::KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				}
				None => Zero::zero(),
				//Some(id) => {
				//	ensure!(id != KittyInde::max_value(), Error::<T>::KittiesCountOverflow);
				//	id
				//}
				//None => 0,
			};

			let dna = Self::random_value(&who);

			let deposit = T::Deposit::get();
			T::Currency::reserve(&who, deposit).map_err(|_| Error::<T>::NotEnoughToken)?;

			Kitties::<T>::insert(kitty_id, Some(Kitty(dna)));

			Owner::<T>::insert(kitty_id, Some(who.clone()));

			// Update storage.
			KittiesCount::<T>::put(kitty_id + 1u32.into());
			//KittiesCount::<T>::put(kitty_id + 1);

			// Emit an event.
			Self::deposit_event(Event::KittyCreate(who, kitty_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(
			origin: OriginFor<T>,
			new_owner: T::AccountId,
			kitty_id: T::KittyIndex,
			//kitty_id: KittyInde,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id), Error::<T>::NotOwner);

			let deposit = T::Deposit::get();
			T::Currency::reserve(&new_owner, deposit)?;
			T::Currency::unreserve(&who, deposit);

			Owner::<T>::insert(kitty_id, Some(new_owner.clone()));

			Self::deposit_event(Event::KittyTransfer(who, new_owner, kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn bread(
			origin: OriginFor<T>,
			kitty_id_1: T::KittyIndex,
			kitty_id_2: T::KittyIndex,
			//kitty_id_1: KittyInde,
			//kitty_id_2: KittyInde,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameParentIndex);

			let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyIndex)?;

			let kitty_id = match Self::kitties_count() {
				Some(id) => {
					ensure!(id != T::KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				}
				None => Zero::zero(),
				//Some(id) => {
				//	ensure!(id != KittyInde::max_value(), Error::<T>::KittiesCountOverflow);
				//	id
				//}
				//None => 0,
			};

			let dna_1 = kitty1.0;
			let dna_2 = kitty2.0;

			let selector = Self::random_value(&who);

			let mut new_dna = [0u8; 16];

			for i in 0..dna_1.len() {
				new_dna[i] = (selector[i] & dna_1[i]) | (!selector[i] & dna_2[i]);
			}

			let deposit = T::Deposit::get();
			T::Currency::reserve(&who, deposit).map_err(|_| Error::<T>::NotEnoughToken)?;

			Kitties::<T>::insert(kitty_id, Some(Kitty(new_dna)));

			Owner::<T>::insert(kitty_id, Some(who.clone()));

			KittiesCount::<T>::put(kitty_id + 1u32.into());
			//KittiesCount::<T>::put(kitty_id + 1);

			Self::deposit_event(Event::KittyCreate(who, kitty_id));

			Ok(())
		}
		#[pallet::weight(0)]
		pub fn set_price(
			origin: OriginFor<T>,
			price: BalanceOf<T>,
			kitty_id: T::KittyIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id), Error::<T>::NotOwner);

			ensure!(price != <BalanceOf<T>>::max_value(), Error::<T>::KittyPriceOverflow);

			KittyPrice::<T>::insert(kitty_id, Some(price.clone()));

			Self::deposit_event(Event::KittySetPrice(who, price, kitty_id));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn buy_kitty(
			origin: OriginFor<T>,
			mount: BalanceOf<T>,
			kitty_id: T::KittyIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(mount != <BalanceOf<T>>::max_value(), Error::<T>::KittyPriceOverflow);
			ensure!(
				Some(<BalanceOf<T>>::zero()) != KittyPrice::<T>::get(kitty_id),
				Error::<T>::KittyNotSale
			);
			ensure!(Some(mount) >= KittyPrice::<T>::get(kitty_id), Error::<T>::NotEnoughToken);
			let old_owner = Owner::<T>::get(kitty_id).ok_or(Error::<T>::NotOwner)?;
			T::Currency::transfer(&who, &old_owner, mount, ExistenceRequirement::KeepAlive)?;

			let deposit = T::Deposit::get();
			T::Currency::reserve(&who, deposit).map_err(|_| Error::<T>::NotEnoughToken)?;
			T::Currency::unreserve(&old_owner, deposit);

			Owner::<T>::insert(kitty_id, Some(who.clone()));
			KittyPrice::<T>::insert(kitty_id, Some(<BalanceOf<T>>::zero()));

			Self::deposit_event(Event::KittyBuy(who, mount, kitty_id));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}
	}
}
