pub use super::*;
pub use frame_support::{
	ensure,
    pallet_prelude::*,
	error::BadOrigin,
    dispatch::RawOrigin,
	traits::{
		defensive_prelude::*,
		schedule::{v3::Named as ScheduleNamed, DispatchTime},
		Bounded, Currency, EnsureOrigin, Get, Hash as PreimageHash, LockIdentifier,
		LockableCurrency, OnUnbalanced, QueryPreimage, ReservableCurrency, StorePreimage,
		WithdrawReasons,
	},
	weights::Weight,
};
pub use frame_system::pallet_prelude::{BlockNumberFor, OriginFor};
pub use frame_system::pallet_prelude::*;
pub use sp_runtime::{
	traits::{Bounded as ArithBounded, One, Saturating, StaticLookup, Zero},
	ArithmeticError, DispatchError, DispatchResult,
};
pub use sp_std::prelude::*;

type BalanceOf<T> =
	<<T as DEM::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as DEM::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;
pub type CallOf<T> = <T as frame_system::Config>::RuntimeCall;
pub type BoundedCallOf<T> = Bounded<CallOf<T>>;
type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;


impl<T: Config> Pallet<T> {

pub fn make_proposal(call: CallOf<T>) -> BoundedCallOf<T> {
	<T as DEM::Config>::Preimages::bound(call).unwrap()
}

pub fn add_proposal(who:T::AccountId,call: CallOf<T>) -> DispatchResult {
	
	let value = <T as DEM::Config>::MinimumDeposit::get();
	let proposal = Self::make_proposal(call);
	DEM::Pallet::<T>::propose(RawOrigin::Signed(who).into(), proposal.clone(), value)?;
	Ok(())
}

pub fn start_dem_referendum(proposal:BoundedCallOf<T> ,delay:BlockNumberFor<T>) -> DEM::ReferendumIndex{
    let threshold = DEM::VoteThreshold::SimpleMajority;    
    let referendum_index =
            DEM::Pallet::<T>::internal_start_referendum(proposal, threshold, delay);
    referendum_index
}


pub fn account_vote(b: BalanceOf<T>) -> DEM::AccountVote<BalanceOf<T>> {
	let v = DEM::Vote { aye: true, conviction: DEM::Conviction::Locked1x };

	DEM::AccountVote::Standard { vote: v, balance: b }
}

pub fn get_formatted_call(call: <T as Config>::RuntimeCallv) -> <T as Config>::RuntimeCallv {
    call
}

}