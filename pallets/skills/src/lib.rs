pub use pallet::*;
/* 
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;*/

mod types;
pub use types::*;
pub use pallet_collective as Coll;
use Coll::Instance1;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: 
	frame_system::Config
	+ Coll::Config<Instance1> {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: ReservableCurrency<Self::AccountId>;
		type CouncilOrigin: EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>;
		/// The maximum length of data stored on-chain.
		#[pallet::constant]
		type StringLimit: Get<u32>;
		#[pallet::constant]
		type BasicWage: Get<BalanceOf<Self>>;
		#[pallet::constant]
		type CheckPeriod: Get<BlockNumberFor<Self>>;
		#[pallet::constant]
		type MaxSkills: Get<u32>+Clone;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn employees_number)]
	pub type EmployeesNumber<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn user_skills)]
	pub type UserSkills<T:Config> = 
		StorageMap<_, Twox64Concat, AccountIdOf<T>,BoundedVec<Skill<T>,T::MaxSkills>,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn employee)]
	pub type EmployeeLog<T:Config> = 
		StorageMap<_, Twox64Concat, AccountIdOf<T>,Employee<BoundedVecOf<T>,BalanceOf<T>,BlockNumberOf<T>>,OptionQuery>;

	#[pallet::type_value]
	/// Initializer for skills list
	pub fn InitSkillList<T: Config>() -> BoundedVec<Skill<T>,T::MaxSkills> {
		let v0 = Vec::new();
		BoundedVec::truncate_from(v0)
	}

	#[pallet::storage]
	#[pallet::getter(fn skills)]
	pub type Skills<T:Config> = StorageValue<_, BoundedVec<Skill<T>,T::MaxSkills>,ValueQuery,InitSkillList<T>>;


	#[pallet::storage]
	#[pallet::getter(fn get_pending_skills)]
	pub type SkillsApprovalList<T: Config> =
		StorageValue<_, BoundedVec<Skill<T>,T::MaxSkills>, ValueQuery,InitSkillList<T>>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
