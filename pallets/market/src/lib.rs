//! # Market Pallet

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;
pub use pallet_skills as SK;
pub use pallet_collective as Coll;
use Coll::Instance1;
pub use pallet_bounties as Bount;
pub use pallet_treasury as Treasury;
mod types;
mod functions;
pub use functions::*;
pub use types::*;
/*
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;
*/
// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;


	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The pallet's configuration trait.
	///
	/// All our types and constants a pallet depends on must be declared here.
	/// These types are defined generically and made concrete when the pallet is declared in the
	/// `runtime/src/lib.rs` file of your chain.
	#[pallet::config]
	pub trait Config: 
    frame_system::Config
    + Coll::Config<Instance1>
    + SK::Config
    + Bount::Config
    +Treasury::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type RuntimeCall: Parameter
			+ UnfilteredDispatchable<RuntimeOrigin = <Self as frame_system::Config>::RuntimeOrigin>
			+ From<Call<Self>>
			+ GetDispatchInfo;
		
	}

	/// A storage item for this pallet.
	///
	/// In this template, we are declaring a storage item called `Something` that stores a single
	/// `u32` value. Learn more about runtime storage here: <https://docs.substrate.io/build/runtime-storage/>
	/// The [`getter`] macro generates a function to conveniently retrieve the value from storage.
	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

    #[pallet::storage]
	#[pallet::getter(fn get_proposal)]
	pub type TasksProposalList<T: Config> =
	StorageDoubleMap<
		_, 
		Blake2_128Concat, 
		AccountIdOf<T>,
		Blake2_128Concat,
		Bount::BountyIndex,
		TaskProposal<T>,
		OptionQuery
		>;

	#[pallet::storage]
	#[pallet::getter(fn needed_skills)]
	pub type TaskSkills<T: Config> =
	StorageMap<_, Twox64Concat, Bount::BountyIndex,BoundedVec<SK::Skill<T>,T::MaxSkills>,ValueQuery>;

    #[pallet::type_value]
	///Initializing function for the total number of employees
	pub fn InitTotal<T: Config>() -> u32 {
		0
	}

	// Total number of Skill proposals
	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub type ProposalsNumber<T> = StorageValue<_, u32, ValueQuery, InitTotal<T>>;



	/// Events that functions in this pallet can emit.
	///
	/// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
	/// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
	/// documentation for each event field and its parameters is added to a node's metadata so it
	/// can be used by external interfaces or tools.
	///
	///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
	/// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
	/// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A user has successfully set a new value.
		SomethingStored {
			/// The new value set.
			something: u32,
			/// The account who set the new value.
			who: T::AccountId,
		},
		NeededSkillAdded{
			what: BoundedVecOf<T>,
			task: BoundedVecOf<T>,
			by_who: BoundedVecOf<T>,
			when: BlockNumberOf<T>,
		}
	}

	/// Errors that can be returned by this pallet.
	///
	/// Errors tell users that something went wrong so it's important that their naming is
	/// informative. Similar to events, error documentation is added to a node's metadata so it's
	/// equally important that they have helpful documentation associated with them.
	///
	/// This type of runtime error can be up to 4 bytes in size should you want to return additional
	/// information.
	#[pallet::error]
	pub enum Error<T> {
		/// The value retrieved was `None` as no value was previously set.
		NoneValue,
		/// There was an attempt to increment the value in storage over `u32::MAX`.
		StorageOverflow,
		NotAnExistingTask,
		NotAPendingTask,
		NotATaskProposal,
	}

	/// The pallet's dispatchable functions ([`Call`]s).
	///
	/// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	/// These functions materialize as "extrinsics", which are often compared to transactions.
	/// They must always return a `DispatchResult` and be annotated with a weight and call index.
	///
	/// The [`call_index`] macro is used to explicitly
	/// define an index for calls in the [`Call`] enum. This is useful for pallets that may
	/// introduce new dispatchables over time. If the order of a dispatchable changes, its index
	/// will also change which will break backwards compatibility.
	///
	/// The [`weight`] macro is used to assign a weight to each call.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a single u32 value as a parameter, writes the value
		/// to storage and emits an event.
		///
		/// It checks that the _origin_ for this call is _Signed_ and returns a dispatch
		/// error if it isn't. Learn more about origins here: <https://docs.substrate.io/build/origins/>
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn additional_task_skills(origin: OriginFor<T>,task_id:Bount::BountyIndex, skill:SK::Skill<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			//Origin is an employee
			ensure!(SK::Pallet::<T>::employee(&who).is_some(), SK::Error::<T>::NotAnEmployee);
			
			//Proposal exists
			ensure!(TasksProposalList::<T>::contains_key(&who,task_id),Error::<T>::NotATaskProposal);
			
			//Task exists
			let bounty = Bount::Pallet::<T>::bounties(task_id);
			ensure!(bounty.is_some(), Error::<T>::NotAnExistingTask);
			let status = bounty.unwrap().get_status();
			ensure!(status==Bount::BountyStatus::Proposed, Error::<T>::NotAPendingTask);


			//add skill to task list, and update proposal task_listy
			TaskSkills::<T>::mutate(task_id,|list|{
				list.try_push(skill.clone()).map_err(|_| "Max number of skills reached").ok();
			});
			let needed_skills = Self::needed_skills(task_id);
			let mut proposal = Self::get_proposal(&who,task_id).unwrap();
			proposal.needed_skills =needed_skills;

		TasksProposalList::<T>::mutate(who.clone(),task_id,|val|{
			*val = Some(proposal.clone());
		});


			// Emit an event.
			let when =  <frame_system::Pallet<T>>::block_number();
			let what = skill.metadata;
			let task = proposal.description;
			let employee = SK::Pallet::<T>::employee(&who).unwrap();
			let by_who = employee.name;

			Self::deposit_event(Event::NeededSkillAdded{
				what,
				task,
				by_who,
				when
			});


			//Self::deposit_event(Event::SomethingStored { something, who });

			// Return a successful `DispatchResult`
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn approve_task(origin: OriginFor<T>,account: T::AccountId) -> DispatchResult {
			let _who = T::CouncilOrigin::ensure_origin(origin.clone())?;
			let task_iter = TasksProposalList::<T>::iter();
			let mut b_id = 0;
			let mut cur=account.clone();
			for task in task_iter{
				let acc = task.0;
				if acc == account{
					b_id = task.1;
					cur = task.2.curator;
					
					
				}
			}
			

			//Assess that the id is linked to a created bounty, not yet approved
			let bounty = Bount::Pallet::<T>::bounties(b_id);
			ensure!(bounty.is_some(), Error::<T>::NotAnExistingTask);

			//Assess that task status is 'awaiting for approval'
			let status = bounty.unwrap().get_status();
			ensure!(status==Bount::BountyStatus::Proposed, Error::<T>::NotAPendingTask);

			

			Bount::Pallet::<T>::approve_bounty(origin.clone(),b_id).ok();
			Bount::Pallet::<T>::propose_curator(origin,b_id,T::Lookup::unlookup(cur),Zero::zero()).ok();
			
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn propose_task(origin: OriginFor<T>, skill:SK::Skill<T>, value:BalanceOf<T>,description:BoundedVecOf<T>, curator:T::AccountId) -> DispatchResult{

			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin.clone())?;
			//Origin is an employee
			ensure!(SK::Pallet::<T>::employee(&who).is_some(), SK::Error::<T>::NotAnEmployee);
			
			//propose the bounty
			Bount::Pallet::<T>::propose_bounty(origin.clone(),value,description.clone().into_inner()).ok();
			
			//start the council session
			Self::start_task_session(who,curator,description,value,skill).ok();

			Ok(())
		}

	}
}