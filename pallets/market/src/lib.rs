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
pub use types::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
/*
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;
*/
// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	use frame_system::EnsureRootWithSuccess;

// Import various useful types required by all FRAME pallets.
	use super::*;
	


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
    + Treasury::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type RuntimeCall: Parameter
			+ UnfilteredDispatchable<RuntimeOrigin = <Self as frame_system::Config>::RuntimeOrigin>
			+ From<Call<Self>>
			+ GetDispatchInfo;
		#[pallet::constant]
		type Sp: Get<u32>;

		#[pallet::constant]
		type Xp: Get<u32>;

		
	}

	/// A storage item for this pallet.
	///
	/// In this template, we are declaring a storage item called `Something` that stores a single
	/// `u32` value. Learn more about runtime storage here: <https://docs.substrate.io/build/runtime-storage/>
	/// The [`getter`] macro generates a function to conveniently retrieve the value from storage.
	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	/// (task_owner,task_id,task_proposal)
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

	/// (task_id,BoudedVec<skills>)
	#[pallet::storage]
	#[pallet::getter(fn needed_skills)]
	pub type TaskSkills<T: Config> =
	StorageMap<_, Twox64Concat, Bount::BountyIndex,BoundedVec<SK::Skill<T>,T::MaxSkills>,ValueQuery>;

	/// (task_owner, task_status)
	#[pallet::storage]
	#[pallet::getter(fn status)]
	pub type TaskStat<T: Config> =
	StorageMap<_, Twox64Concat, T::AccountId,Status<T>,OptionQuery>;


	/// (task_worker,BoundedVec<task_id>)
	#[pallet::storage]
	#[pallet::getter(fn worker)]
	pub type TaskWorker<T: Config> =
	StorageMap<_, Twox64Concat, T::AccountId,BoundedVec<u32,T::MaxSkills>,ValueQuery>;

	/// (bounty_id,curator_account)
	#[pallet::storage]
	#[pallet::getter(fn curator)]
	pub type ProposedCurator<T: Config> =
	StorageMap<_, Twox64Concat, Bount::BountyIndex,(u32,T::AccountId),OptionQuery>;

	/// (bounty_id,curator_account)
	#[pallet::storage]
	#[pallet::getter(fn active_curator)]
	pub type ActiveCurator<T: Config> =
	StorageMap<_, Twox64Concat, Bount::BountyIndex,(u32,T::AccountId),OptionQuery>;


    #[pallet::type_value]
	///Initializing function for the total number of employees
	pub fn InitTotal<T: Config>() -> u32 {
		0
	}

	/// Total number of Skill proposals
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
		},
		/// A new task creation was requested
		/// A member of the Council has voted
		CouncilVoted{who: T::AccountId, proposal_index: u32, when: BlockNumberOf<T>},
		/// A Task proposal session has been closed by a Council member
		CouncilSessionClosed{who: T::AccountId, proposal_index: u32, when: BlockNumberOf<T>},
		/// A curator was proposed by the council
		CuratorProposed{who: T::AccountId, when:BlockNumberOf<T>},
		/// Task creation approved by the council
		/// Task creation rejected by the council
		/// The Curator proposed by the task creator has been contacted by the council  
		CouncilContactedACurator{who: T::AccountId, when: BlockNumberOf<T>},

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

		/// Task not created
		NotAnExistingTask,

		/// This does not correspond to a created task
		NotAPendingTask,

		/// This task proposal does not exists
		NotATaskProposal,

		/// The account does not belong to a Council member
		NotACouncilMember,

		/// This operation is not permitted for your account
		NotPermitted,

		/// This task has been picked up by another employee
		AlreadyPickedUpByX,

		/// Employee already working on task
		AlreadyPickedUpByYou
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Weight: see `begin_block`
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			Self::begin_block(n)
		}
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
		
		/// Add needed skills to a task
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn additional_task_skills(origin: OriginFor<T>,task_id:Bount::BountyIndex, skill_id:u32) -> DispatchResult {
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
			ensure!(status==Bount::BountyStatus::Approved||status==Bount::BountyStatus::Funded||status==Bount::BountyStatus::Proposed, Error::<T>::NotAPendingTask);

			let skill_list = SK::Pallet::<T>::skills().into_inner();
			let skill = skill_list[skill_id as usize].clone();
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

		/// Submitted task approval function
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn approve_task(origin: OriginFor<T>,task_owner: T::AccountId) -> DispatchResult {
			let _who = T::CouncilOrigin::ensure_origin(origin.clone())?;
			
						
			let task0 = Self::get_task_infos(task_owner.clone()).unwrap();
			let b_id =task0.0;
			debug_assert!(b_id==0);
			//Assess that the id is linked to a created bounty, not yet approved
			let bounty = Bount::Pallet::<T>::bounties(b_id);
			ensure!(bounty.is_some(), Error::<T>::NotAnExistingTask);

			//Assess that task status is 'awaiting for approval'
			let status = bounty.unwrap().get_status();
			ensure!(status==Bount::BountyStatus::Proposed, Error::<T>::NotAPendingTask);

			Bount::Pallet::<T>::approve_bounty(RawOrigin::Root.into(),b_id)?;
			let now = <frame_system::Pallet<T>>::block_number();
			TaskStat::<T>::mutate(task_owner.clone(),|val|{
				let mut val0 = val.clone().unwrap();
				val0.changed_when = now;
				val0.status = TaskStatus::Open;
				*val = Some(val0);

			});

			
			Ok(())
		}

		/// Submitted task rejection
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn reject_task(origin: OriginFor<T>, task_owner: T::AccountId) -> DispatchResult{
			let _caller = T::RejectOrigin::ensure_origin(origin.clone());
			
			
			let task0 = Self::get_task_infos(task_owner.clone()).unwrap();
			let b_id =task0.0;		

			//Assess that the id is linked to a created bounty, not yet approved
			let bounty = Bount::Pallet::<T>::bounties(b_id);
			ensure!(bounty.is_some(), Error::<T>::NotAnExistingTask);
			Bount::Pallet::<T>::close_bounty(origin,b_id).ok();

			let now = <frame_system::Pallet<T>>::block_number();
			TaskStat::<T>::mutate(task_owner.clone(),|val|{
				let mut val0 = val.clone().unwrap();
				val0.changed_when = now;
				val0.status = TaskStatus::Rejected;
				*val = Some(val0);

			});

			Ok(())
		}

		/// Employee submits a new task to the council
		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn propose_task(origin: OriginFor<T>,skill_id:u32, reward: BalanceOf<T>, description:BoundedVecOf<T>,curator0:T::AccountId) -> DispatchResult{

			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin.clone())?;
			let mut curator = Some(curator0);
			//Origin is an employee
			ensure!(SK::Pallet::<T>::employee(&who).is_some(), SK::Error::<T>::NotAnEmployee);
			if !curator.is_some(){
				curator = Some(who.clone());
			}
			let skills = SK::Pallet::<T>::skills().into_inner();
			let skill = skills[skill_id as usize].clone();			
			

			//propose the bounty
			Bount::Pallet::<T>::propose_bounty(origin.clone(),reward,description.clone().into_inner())?;
			let now = <frame_system::Pallet<T>>::block_number();
			let status:Status<T> = Status{worker:None,status: TaskStatus::CouncilReview,changed_when:now,};
			TaskStat::<T>::insert(who.clone(),status);
			//start the council session
			let call = 
				Call::<T>::approve_task{
				task_owner: who.clone()
				};
			Self::start_task_session(who,call,curator.unwrap(),description,reward,skill).ok();

			Ok(())
		}

		/// Curator suggested by task_owner is contacted by the Council
		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn propose_curator(origin:OriginFor<T>, task_owner:T::AccountId) -> DispatchResultWithPostInfo{
			let _max = ensure_signed(origin.clone())?;
			let task0 = Self::get_task_infos(task_owner.clone()).unwrap();
			let root:OriginFor<T> = RawOrigin::Root.into();
			let _max=T::SpendOrigin::ensure_origin(root.clone());

			let b_id =task0.0;
			let cur = task0.1.curator;
			let index:u32 = Coll::Pallet::<T,Instance1>::proposal_count();
			
    	
			let call = 
				Call::<T>::curator_proposed{
				curator: cur.clone()
				};

			let _bed = Bount::Pallet::<T>::propose_curator(root,b_id,T::Lookup::unlookup(cur.clone()),Zero::zero())?;
			
			Self::start_council(call);

			ProposedCurator::<T>::insert(b_id,(index,cur.clone()));
			Ok(().into())
		}

		/// Curator accepts role 
		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn accept_curator(origin:OriginFor<T>, task_owner:T::AccountId) -> DispatchResult{
			let who = ensure_signed(origin.clone())?;
			let task0 = Self::get_task_infos(task_owner.clone()).unwrap();
			let b_id =task0.0;
			let cur = task0.1.curator;
			//Proposal status is Approved
			let bounty = Bount::Pallet::<T>::bounties(b_id).unwrap();
			let bounty_status = bounty.get_status();
			ensure!(bounty_status == Bount::BountyStatus::CuratorProposed{curator: cur.clone()}, Error::<T>::NotPermitted);

			let curator_infos = Self::curator(b_id).unwrap();
			ActiveCurator::<T>::insert(b_id,curator_infos);

			ensure!(who==cur,Error::<T>::NotPermitted);
			Bount::Pallet::<T>::accept_curator(origin,b_id).ok();
			ProposedCurator::<T>::remove(b_id);
			Ok(())
		}

		/// Council member vote
		#[pallet::call_index(6)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn council_vote(origin:OriginFor<T>,task_owner:T::AccountId,approve:bool,for_curator:bool) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			ensure!(
				Coll::Pallet::<T, Instance1>::members().contains(&caller),
				Error::<T>::NotACouncilMember
			);
			let task0 = Self::get_task_infos(task_owner.clone().clone()).unwrap();
			let b_id = task0.0;
			let proposal_all = Self::get_proposal(&task_owner,b_id).unwrap();
			let index = proposal_all.proposal_index;
			let result = Self::vote_action(caller.clone(),task_owner,approve,for_curator);
			

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

		
		/// Council member close session
		#[pallet::call_index(7)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn council_close(origin:OriginFor<T>,task_owner:T::AccountId,for_curator: bool) -> DispatchResultWithPostInfo{
			let caller = ensure_signed(origin)?;

			let task0 = Self::get_task_infos(task_owner.clone().clone()).unwrap();
			let b_id = task0.0;
			let proposal_all = Self::get_proposal(&task_owner,b_id).unwrap();
			let index = proposal_all.proposal_index;
			let result = Self::closing_vote(caller.clone(),task_owner.clone(),for_curator);
			

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


		/// Worker picks a task
		#[pallet::call_index(8)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn pick_task(origin:OriginFor<T>, task_owner:T::AccountId) -> DispatchResultWithPostInfo{
			let caller = ensure_signed(origin)?;
			let worker_list = Some(Self::worker(caller.clone()));
			let task_infos = Self::get_task_infos(task_owner.clone()).unwrap();
			let b_id = task_infos.0;
			
			//Employee is already working on the task
			if worker_list.is_some(){
				let list = worker_list.unwrap().into_inner();
				
				ensure!(!list.contains(&b_id), Error::<T>::AlreadyPickedUpByYou);

				TaskWorker::<T>::mutate(caller.clone(),|list|{
					list.try_push(b_id).map_err(|_| "Max number of skills reached").ok();
				});
				
			} else{
				let v0 = vec![b_id];
				let v1:BoundedVec<u32,T::MaxSkills> = BoundedVec::truncate_from(v0.clone());

				TaskWorker::<T>::insert(caller,v1);
				let now = <frame_system::Pallet<T>>::block_number();
			TaskStat::<T>::mutate(task_owner.clone(),|val|{
				let mut val0 = val.clone().unwrap();
				val0.changed_when = now;
				val0.status = TaskStatus::InWork;
				*val = Some(val0);

			});

			}
					

			Ok(().into())
		}

		/// Worker claims financial reward after being acknowledged by curator
		#[pallet::call_index(9)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn worker_claims_reward(origin: OriginFor<T>, task_owner:T::AccountId) -> DispatchResultWithPostInfo{
			let who = ensure_signed(origin.clone())?;
			let task_infos = Self::get_task_infos(task_owner.clone()).unwrap();
			let worker = Some(Self::worker(who));
			ensure!(worker.is_some(), Error::<T>::NotPermitted);
			let id_list = worker.unwrap().into_inner();
			let task_id = task_infos.0;
			ensure!(id_list.contains(&task_id), Error::<T>::NotPermitted);
			Bount::Pallet::<T>::claim_bounty(origin,task_id).ok();

			Ok(().into())
		}

		/// Curator rewards worker
		#[pallet::call_index(10)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn curator_rewards_worker(origin: OriginFor<T>, task_owner:T::AccountId, task_worker:T::AccountId) -> DispatchResultWithPostInfo{
			let who = ensure_signed(origin.clone())?;
			let task_infos = Self::get_task_infos(task_owner.clone()).unwrap();
			let curator = task_infos.1.curator;
			ensure!(curator == who, Error::<T>::NotPermitted);

			let now = <frame_system::Pallet<T>>::block_number();
			TaskStat::<T>::mutate(task_owner.clone(),|val|{
				let mut val0 = val.clone().unwrap();
				val0.worker = Some(task_worker.clone());
				val0.changed_when = now;
				val0.status = TaskStatus::Completed;
				*val = Some(val0);
			});

			Bount::Pallet::<T>::award_bounty(origin,task_infos.0,T::Lookup::unlookup(task_worker.clone())).ok();
			Self::upgrade_employee(task_worker.clone(),task_owner).ok();

			Ok(().into())
		}

		/// Curator rewards worker
		#[pallet::call_index(11)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn curator_proposed(origin: OriginFor<T>, curator:T::AccountId) -> DispatchResultWithPostInfo{
			let _who = T::CouncilOrigin::ensure_origin(origin.clone())?;
			let when = <frame_system::Pallet<T>>::block_number();
			Self::deposit_event(Event::CuratorProposed{
				who: curator,
				when
			});
			Ok(().into())
		}
	}
}