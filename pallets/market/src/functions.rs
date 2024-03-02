pub use super::*;
impl<T: Config> Pallet<T> {

pub fn create_task() -> DispatchResultWithPostInfo{
    Ok(().into())
}

pub fn start_task_session(account:T::AccountId,curator:T::AccountId, description:BoundedVecOf<T>, value: BalanceOf<T>, skill: SK::Skill<T>) -> DispatchResultWithPostInfo{

    //Create proposal
    let proposal0 = 
    Call::<T>::approve_task{
        account: account.clone()
    };
    let proposal0 = Self::get_formatted_call(proposal0.into());
    let proposal = proposal0.unwrap();
    let _hash = T::Hashing::hash_of(&proposal);
    let proposal_len:u32 = proposal.using_encoded(|p| p.len() as u32);
    
    let council_member = Coll::Pallet::<T,Instance1>::members()[0].clone();
    let council_origin= SK::Pallet::<T>::get_origin(council_member);

    //Start Collective refererendum
    Coll::Pallet::<T,Instance1>::propose(
        council_origin,
        2,
        Box::new(proposal.clone()),
        proposal_len,
    )?;
    let mut index:u32 = Coll::Pallet::<T,Instance1>::proposal_count();
    index = index.saturating_sub(1);

    //Update proposal index and hash
    let proposal_hashes =  Coll::Pallet::<T,Instance1>::proposals().into_iter();
    for proposal_hash in proposal_hashes{
        let prop0 = Coll::Pallet::<T,Instance1>::proposal_of(proposal_hash.clone()).unwrap();
        if proposal == prop0{
            let mut proposal_all = TaskProposal::<T>::new(account.clone(), curator.clone(), description.clone(),value,proposal_hash.clone(),skill.clone());
            proposal_all.proposal_index = index;
            proposal_all.proposal_hash = proposal_hash;
            TasksProposalList::<T>::insert(&account,Bount::Pallet::<T>::bounty_count(), proposal_all);
            let task_id = Bount::Pallet::<T>::bounty_count();
            TaskSkills::<T>::mutate(task_id,|list|{
				list.try_push(skill.clone()).map_err(|_| "Max number of skills reached").ok();
			});
        }
        
    }


    Ok(().into())
}

pub fn get_formatted_call(call: <T as Config>::RuntimeCall) -> Option<<T as Coll::Config<Instance1>>::Proposal> {
    let call_encoded: Vec<u8> = call.encode();
    let ref_call_encoded = &call_encoded;

    if let Ok(call_formatted) = <T as pallet_collective::Config<Instance1>>::Proposal::decode(
        &mut &ref_call_encoded[..],
    ) {
        Some(call_formatted)
    } else {
        None
    }
}


}