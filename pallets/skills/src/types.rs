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
pub use scale_info::{prelude::vec, TypeInfo};
pub use serde::{Deserialize, Serialize};

pub type BalanceOf<T> =
	<<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type BlockNumberOf<T> = BlockNumberFor<T>;
pub type BoundedVecOf<T> = BoundedVec<u8, <T as pallet::Config>::StringLimit>;


#[derive(Clone, Encode, Decode, Default, PartialEq, Eq, TypeInfo, Copy, Serialize, Deserialize, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum SkillFamily{
	Soft,
    #[default]
	Technical,	
}

#[derive(Clone, Encode, Decode, Default, PartialEq, Eq, TypeInfo, Copy, Serialize, Deserialize, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Approvals{
	YES,
	NO,
	#[default]
	AWAITING,
}

#[derive(Clone, Encode, Decode, Default, PartialEq, Eq, TypeInfo, Copy, Serialize, Deserialize, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum SkillLevel{
	#[default]
    Level1,    
	Level2,
    Level3,
    Level4,	
}

#[derive(Clone, Encode, Decode, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Skill<T: Config>{
	pub metadata: BoundedVecOf<T>,
	pub skill_type: SkillFamily,
	pub creation_block: BlockNumberOf<T>,
	pub skill_level:SkillLevel,
    pub confirmed:bool,
    pub skill_number: u8,
}

impl<T:Config>Skill<T>{
	pub fn new(metadata:BoundedVecOf<T>, skill_type: SkillFamily) -> Self{
		let creation_block = <frame_system::Pallet<T>>::block_number();
		let skill_level = SkillLevel::default();
		let skill_list:BoundedVec<Skill<T>,T::MaxSkills> = Skills::get();
		let skill_number = skill_list.into_inner().len() as u8;
		let new_skill = Skill{metadata,skill_type,creation_block,skill_level,confirmed:false,skill_number};

		Skills::<T>::mutate(|list|{
			list.try_push(new_skill.clone()).map_err(|_| "Max number of skills reached").ok();
		});

		new_skill
	}
}

#[derive(Clone, Encode, Decode, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Employee<T:Config>{
	pub name: BoundedVecOf<T>,
	pub uid: u32,
	pub sp:u32,
	pub xp:u32,
	pub wage: BalanceOf<T>,
	pub creation_block: BlockNumberOf<T>,
}
impl<T:Config>Employee<T>{
	pub fn new(account:T::AccountId, name:BoundedVecOf<T>) -> Self{
		let uid = EmployeesNumber::<T>::get();
		let sp = 0;
		let xp = 0;
		let wage = T::BasicWage::get();
		let creation_block  = <frame_system::Pallet<T>>::block_number();

		let new_employee = Employee{name,uid,sp,xp,wage,creation_block};
		EmployeeLog::<T>::insert(account,&new_employee);
		EmployeesNumber::<T>::put(uid.saturating_add(1));

		new_employee

	}
}

#[derive(Clone, Encode, Decode, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct SkillProposal<T: Config>{	
	pub skill: Option<Skill<T>>,
	pub creation_block: BlockNumberOf<T>,
	pub proposal_hash: T::Hash,
	pub proposal_index: u32,
	pub session_closed: bool, 
	pub approved: Approvals,
}

