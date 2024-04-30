#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/* 
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;*/

mod types;
mod functions;
pub use types::*;
pub use pallet_collective as Coll;
pub use pallet_balances as BALANCES;
use Coll::Instance1;
use frame_support::traits::OriginTrait;

#[frame_support::pallet]
pub mod pallet {
	//use Coll::GenesisConfig;


use super::*;
	//use frame_support::pallet_prelude::*;
	//use frame_system::pallet_prelude::*;
	use sp_std::fmt::Debug;
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: 
	frame_system::Config
	+ Coll::Config<Instance1>+BALANCES::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: ReservableCurrency<Self::AccountId>;
		type CouncilOrigin: EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>;
		type RuntimeCall: Parameter
			+ UnfilteredDispatchable<RuntimeOrigin = <Self as frame_system::Config>::RuntimeOrigin>
			+ From<Call<Self>>
			+ GetDispatchInfo;

		/// The maximum length of data stored on-chain.
		#[pallet::constant]
		type StringLimit: Get<u32>;

		///Basic employee wage
		#[pallet::constant]
		type BasicWage: Get<BalanceOf<Self>>;
		#[pallet::constant]
		type CheckPeriod: Get<BlockNumberFor<Self>>;
		#[pallet::constant]
		type MaxSkills: Get<u32>+Clone;
		#[pallet::constant]
		type SkillLifetime: Get<BlockNumberFor<Self>>;
		#[pallet::constant]
		type Sp: Get<u32>;
		#[pallet::constant]
		type Xp: Get<u32>;
		#[pallet::constant]
		type BudgetAccount: Get<PalletId>;
		#[pallet::constant]
		type InitialBudget: Get<<Self as BALANCES::Config>::Balance>;
		type CheckCycle: Get<BlockNumberFor<Self>>;

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

	
	//Verified skills lifetime counter
	#[pallet::storage]
	#[pallet::getter(fn user_skill_counter)]
	pub type SkillTimeCounter<T> = 
		StorageDoubleMap<_,Twox64Concat,AccountIdOf<T>,Twox64Concat,Skill<T>,VskillCounter<T>,OptionQuery>;
	

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

	#[pallet::storage]
	#[pallet::getter(fn get_proposal)]
	pub type SkillsProposalList<T: Config> =
	StorageMap<_, Twox64Concat, AccountIdOf<T>,SkillProposal<T>,OptionQuery>;

	#[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config>{

		pub employees: Vec<(Option<T::AccountId>,BoundedVecOf<T>)>,
	}

	#[pallet::genesis_build]
	impl<T:Config> BuildGenesisConfig for GenesisConfig<T>{
		fn build(&self) {
			let council_member = Coll::Pallet::<T, Instance1>::members()[0].clone();
			let origin0 = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(council_member));
			for i in self.employees.clone(){
				if i.0.is_some(){
					let employee0 = i.0.clone().unwrap();
					crate::Pallet::<T>::new_employee(origin0.clone(),employee0,i.1.clone()).ok();
				}
			}
			
			let origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Root);
			crate::Pallet::<T>::set_budget(origin).ok();

		}
	}

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
		/// A member of the Background Council has voted
		CouncilVoted{who: T::AccountId, proposal_index: u32, when: BlockNumberOf<T>},
		/// A proposal has been closed by a Council member
		CouncilSessionClosed{who: T::AccountId, proposal_index: u32, when: BlockNumberOf<T>},
		/// A new employee was created
		EmployeeCreated{who: BoundedVecOf<T>, when: BlockNumberOf<T>},
		/// An Unverified skill was added to employee profile
		UnverifiedSkillAdded{who: BoundedVecOf<T>, what: BoundedVecOf<T>, when: BlockNumberOf<T>},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// This is not a recognized skill
		NotARecognizedSkill,
		/// No skill submitted by this user
		NoSkillSubmited,
		/// This account is not connected to an employee account
		NotAnEmployee,
		/// This account does not belong to a council member
		NotACouncilMember,
		/// The Proposal does not exist
		ProposalDoesNotExist,
	}


	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Weight: see `begin_block`
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			Self::begin_block(n)
		}
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
			let _who = T::CouncilOrigin::ensure_origin(origin)?;
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
		pub fn submit_skill(origin: OriginFor<T>,metadata:BoundedVecOf<T>, skill_type: SkillFamily, skill_level: SkillLevel) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			ensure!(Self::employee(&who).is_some(), Error::<T>::NotAnEmployee);
			//create new skill
			let skill:Skill<T> = Skill::new(metadata,skill_type,who.clone(),skill_level);

			Self::start_council_session(who,skill).ok();


			Ok(().into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn council_vote(origin:OriginFor<T>,candidate:T::AccountId,approve:bool) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			ensure!(
				Coll::Pallet::<T, Instance1>::members().contains(&caller),
				Error::<T>::NotACouncilMember
			);
			let proposal_all = Self::get_proposal(&candidate).unwrap();
			let index = proposal_all.proposal_index;
			let result = Self::vote_action(caller.clone(),candidate,approve);
			

			match result{
				Ok(_) => {
					let now = <frame_system::Pallet<T>>::block_number();
					// deposit event
					Self::deposit_event(Event::CouncilVoted{
						who: caller,
						proposal_index: index,
						when: now,
						});
					},
				Err(e) => return Err(e),
				}
			

			Ok(().into())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn council_close(origin:OriginFor<T>,candidate:T::AccountId) -> DispatchResultWithPostInfo{
			let caller = ensure_signed(origin)?;
			let proposal_all = Self::get_proposal(&candidate).unwrap();
			let index = proposal_all.proposal_index;
			let result = Self::closing_vote(caller.clone(),candidate.clone());
			

			match result{
				Ok(_) => {
					let now = <frame_system::Pallet<T>>::block_number();

			Self::deposit_event(Event::CouncilSessionClosed{
				who: caller,
				proposal_index: index,
				when: now,
			});
				},
				Err(e) => return Err(e),
			}
			
			Ok(().into())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn new_employee(origin:OriginFor<T>, candidate: T::AccountId,name: BoundedVecOf<T>) -> DispatchResultWithPostInfo{
			let caller = ensure_signed(origin)?;
			ensure!(
				Coll::Pallet::<T, Instance1>::members().contains(&caller),
				Error::<T>::NotACouncilMember
			);
			let now = <frame_system::Pallet<T>>::block_number();
			let _employee: Employee<T> = Employee::new(candidate,name.clone());

			Self::deposit_event(Event::EmployeeCreated{
				who: name,
				when: now,
			});


			Ok(().into())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn add_my_skills(origin:OriginFor<T>, skill_number: u32) -> DispatchResultWithPostInfo{
			let caller = ensure_signed(origin)?;
			// Caller is an employee
			ensure!(EmployeeLog::<T>::contains_key(&caller), Error::<T>::NotAnEmployee);
			let employee = Self::employee(&caller).unwrap();
			let now = <frame_system::Pallet<T>>::block_number();
			let skill = &Self::skills().into_inner()[skill_number as usize];
			Employee::<T>::add_my_skill(caller,&skill).ok();

			Self::deposit_event(Event::UnverifiedSkillAdded{
				who: employee.name,
				what: skill.metadata.clone(),
				when: now,
			});

			Ok(().into())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn set_budget(origin:OriginFor<T>)-> DispatchResultWithPostInfo{
			let caller = ensure_root(origin.clone())?;

			let user: T::AccountId = T::BudgetAccount::get().into_account_truncating();
			let user_lookup = T::Lookup::unlookup(user.clone());
			let amount0 = T::InitialBudget::get();		
			BALANCES::Pallet::<T>::force_set_balance(origin,user_lookup,amount0).unwrap();
			
			Ok(().into())
		}
	}
}
