#![cfg_attr(not(feature = "std"), no_std)]
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
mod functions;
pub use functions::*;
pub use types::*;
pub use pallet_collective as Coll;
use Coll::Instance1;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	//use frame_support::pallet_prelude::*;
	//use frame_system::pallet_prelude::*;

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

	#[pallet::type_value]
	///Initializing function for the total number of employees
	pub fn InitTotalMembers<T: Config>() -> u32 {
		0
	}

	// Total number of Skill proposals
	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub type ProposalsNumber<T> = StorageValue<_, u32, ValueQuery, InitTotalMembers<T>>;


	// Total number of employees
	#[pallet::storage]
	#[pallet::getter(fn employees_number)]
	pub type EmployeesNumber<T> = StorageValue<_, u32, ValueQuery, InitTotalMembers<T>>;

	//Verified user's skills
	#[pallet::storage]
	#[pallet::getter(fn user_ver_skills)]
	pub type UserVerifiedSkills<T:Config> = 
		StorageMap<_, Twox64Concat, AccountIdOf<T>,BoundedVec<Skill<T>,T::MaxSkills>,ValueQuery>;

	//Unverified user's skills
	#[pallet::storage]
	#[pallet::getter(fn user_unv_skills)]
	pub type UserUnverifiedSkills<T:Config> = 
		StorageMap<_, Twox64Concat, AccountIdOf<T>,BoundedVec<Skill<T>,T::MaxSkills>,ValueQuery>;


	//Employees database
	#[pallet::storage]
	#[pallet::getter(fn employee)]
	pub type EmployeeLog<T:Config> = 
		StorageMap<_, Twox64Concat, AccountIdOf<T>,Employee<T>,OptionQuery>;

	
	#[pallet::type_value]
	/// Initializer for skills list
	pub fn InitSkillList<T: Config>() -> BoundedVec<Skill<T>,T::MaxSkills> {
		let v0 = Vec::new();
		BoundedVec::truncate_from(v0)
	}

	//Skills database
	#[pallet::storage]
	#[pallet::getter(fn skills)]
	pub type Skills<T:Config> = StorageValue<_, BoundedVec<Skill<T>,T::MaxSkills>,ValueQuery,InitSkillList<T>>;


	#[pallet::storage]
	#[pallet::getter(fn get_pending_skills)]
	pub type SkillsApprovalList<T: Config> =
	StorageMap<_, Twox64Concat, AccountIdOf<T>,Skill<T>,OptionQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
		/// A new skill was added to the skill database
		NewSkillCreated{when: BlockNumberFor<T>, what: BoundedVecOf<T> },
		/// A skill creation was rejected
		SkillCreationRejected { when: BlockNumberFor<T>, what: BoundedVecOf<T> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		///This is not a recognized skill
		NotARecognizedSkill,
		///No skill submitted by this user
		NoSkillSubmited,
		///This account is not connected to an employee account
		NotAnEmployee,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// New skill submission approval
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn approve_skill(origin: OriginFor<T>, account: T::AccountId) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin)?;
			ensure!(Self::employee(&account).is_some(), Error::<T>::NotAnEmployee);
			let pending_skill = Self::get_pending_skills(&account);			
			ensure!(pending_skill.is_some(), Error::<T>::NoSkillSubmited);
			let result = Self::approve_skill_helper(account);
			let skill = pending_skill.unwrap();
			match result{
				Ok(_) => {

					let now = <frame_system::Pallet<T>>::block_number();
					Self::deposit_event(Event::NewSkillCreated{
						when: now,
						what: skill.metadata,
					});

				},
				Err(e) => return Err(e),
			}

			Ok(().into())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn reject_skill(origin: OriginFor<T>,account:T::AccountId) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin)?;
			ensure!(Self::employee(&account).is_some(), Error::<T>::NotAnEmployee);
			let pending_skill = Self::get_pending_skills(&account);			
			ensure!(pending_skill.is_some(), Error::<T>::NoSkillSubmited);
			let result = Self::reject_skill_helper(account);
			let skill = pending_skill.unwrap();
			match result{
				Ok(_) => {

					let now = <frame_system::Pallet<T>>::block_number();
					Self::deposit_event(Event::SkillCreationRejected{
						when: now,
						what: skill.metadata,
					});

				},
				Err(e) => return Err(e),
			}

			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn submit_skill(origin: OriginFor<T>,metadata:BoundedVecOf<T>, skill_type: SkillFamily) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			ensure!(Self::employee(&who).is_some(), Error::<T>::NotAnEmployee);
			//create new skill
			let _skill:Skill<T> = Skill::new(metadata,skill_type,who);



			Ok(().into())
		}


	}
}
