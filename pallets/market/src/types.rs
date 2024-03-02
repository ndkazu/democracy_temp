pub use super::*;
//use frame_support::traits::fungibles::metadata;
pub use frame_support::{
	assert_ok,
	dispatch::{DispatchResult},
	pallet_prelude::*,
	sp_runtime::traits::{AccountIdConversion, Hash, Saturating, StaticLookup, Zero},
	storage::{child,bounded_vec::BoundedVec},
	traits::{
		UnfilteredDispatchable,Currency, ExistenceRequirement, Get, LockableCurrency, ReservableCurrency, WithdrawReasons,
	},
	dispatch::GetDispatchInfo,
	PalletId,
};
pub use sp_std::vec::Vec;
pub use frame_system::{ensure_signed, ensure_root, pallet_prelude::*, RawOrigin};
pub use scale_info::{prelude::{vec,boxed::Box}, TypeInfo};
pub use serde::{Deserialize, Serialize};

type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;
pub type BalanceOf<T> = Treasury::BalanceOf<T>;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type BlockNumberOf<T> = BlockNumberFor<T>;
pub type BoundedVecOf<T> = BoundedVec<u8, <T as SK::Config>::StringLimit>;

#[derive(Clone, Encode, Decode, Default, PartialEq, Eq, TypeInfo, Copy, Serialize, Deserialize, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum TaskStatus{
	Open,
	InWork,
	#[default]
	CouncilReview,
	Completed,

}


#[derive(Clone, Encode, Decode, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TaskProposal<T: Config>{	
	pub account:T::AccountId,
	pub curator:T::AccountId,
	pub description: BoundedVecOf<T>,
	pub needed_skills: BoundedVec<SK::Skill<T>,T::MaxSkills>,
    pub value: BalanceOf<T>,
	pub creation_block: BlockNumberOf<T>,
	pub proposal_hash: T::Hash,
	pub proposal_index: u32,
	pub session_closed: bool, 
	pub approved: SK::Approvals,
}

impl<T:Config>TaskProposal<T>{
	pub fn new(account:T::AccountId, curator:T::AccountId, description:BoundedVecOf<T>, value: BalanceOf<T>, proposal: T::Hash,skill:SK::Skill<T>) -> Self{
		let now = <frame_system::Pallet<T>>::block_number();
		let proposal_hash =  T::Hashing::hash_of(&proposal);
		let proposal_index =Bount::Pallet::<T>::bounty_count();
		let needed_skills:BoundedVec<SK::Skill<T>,T::MaxSkills> = BoundedVec::truncate_from(vec![skill]);
		ProposalsNumber::<T>::put(proposal_index);
		let proposal = TaskProposal {account, curator, description,needed_skills,value,creation_block:now,proposal_hash,proposal_index,session_closed:false,approved: SK::Approvals::default()};

		proposal

	}
}