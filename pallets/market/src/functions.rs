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

pub fn get_task_infos(account: T::AccountId) -> Option<(Bount::BountyIndex,TaskProposal<T>)>{
    let task_iter = TasksProposalList::<T>::iter();
			
            let mut task0:Option<(u32,TaskProposal<T>)>=None;
            let mut id =0;
			for task in task_iter{                
            if task.0==account{
                let taskk=task.2;
                id = task.1;
                task0 = Some((id,taskk));
                break
            }
            }
            task0
}


pub fn vote_action(caller: T::AccountId,task_account: T::AccountId,approve:bool) -> DispatchResultWithPostInfo{
		
    // Check that the caller is a council member
    ensure!(
        Coll::Pallet::<T, Instance1>::members().contains(&caller),
        Error::<T>::NotACouncilMember
    );
    let infos = Self::get_task_infos(task_account.clone()).unwrap();
    // Check that the proposal exists
    ensure!(
        TasksProposalList::<T>::contains_key(&task_account,infos.0),
        Error::<T>::NotATaskProposal,

    );
    let proposal_all = Self::get_proposal(task_account.clone(),infos.0).unwrap();
    let proposal_hash = proposal_all.proposal_hash;
    let proposal_index = proposal_all.proposal_index;
    let origin = SK::Pallet::<T>::get_origin(caller.clone());
    // Execute the council vote
    Coll::Pallet::<T, Instance1>::vote(
        origin,
        proposal_hash,
        proposal_index,
        approve,
    ).ok();

    Ok(().into())
}

pub fn closing_vote(caller: T::AccountId,task_account: T::AccountId) -> DispatchResultWithPostInfo{

    // Check that the caller is a council member
    ensure!(
        Coll::Pallet::<T, Instance1>::members().contains(&caller),
        Error::<T>::NotACouncilMember
    );
    // Check that the proposal exists
    let infos = Self::get_task_infos(task_account.clone()).unwrap();
    // Check that the proposal exists
    ensure!(
        TasksProposalList::<T>::contains_key(&task_account,infos.0),
        Error::<T>::NotATaskProposal,

    );
    let proposal_all = Self::get_proposal(task_account.clone(),infos.0).unwrap();
    let proposal_hash = proposal_all.proposal_hash;
    let proposal = Coll::Pallet::<T,Instance1>::proposal_of(proposal_hash.clone()).unwrap();
    let proposal_len = proposal.clone().encoded_size();
    let index = proposal_all.proposal_index;
    let proposal_weight = proposal.get_dispatch_info().weight;
    let origin = SK::Pallet::<T>::get_origin(caller.clone());
    Coll::Pallet::<T,Instance1>::close(
        origin,
        proposal_hash,
        index,
        proposal_weight,
        proposal_len as u32,
    ).ok();

    TasksProposalList::<T>::mutate(&task_account,infos.0,|val|{
        let mut proposal = val.clone().unwrap();
        proposal.session_closed = true;
        *val = Some(proposal);
        });

    Ok(().into())

}

pub fn calculate_sp(skill_level: Level) -> u32{
    let sp = match skill_level {
        Level::Level1 => 1,
        Level::Level2 => 2,
        Level::Level3 => 3,
        Level::Level4 => 4,
    };
    sp
}

pub fn upgrade_employee(account: T::AccountId,task_owner: T::AccountId) -> DispatchResultWithPostInfo{
    let task = Self::get_task_infos(task_owner).unwrap();
    let needed_skills = Self::needed_skills(task.0).into_inner();
    let mut employee = SK::Pallet::<T>::employee(account.clone()).unwrap();
    let employee_skills_unv = SK::Pallet::<T>::user_unv_skills(&account).into_inner();
    let employee_ver_skills = SK::Pallet::<T>::user_ver_skills(&account).into_inner();
    let old_sp = employee.sp;
    let old_xp = employee.xp;
    for sk in needed_skills{
        //Upgrade Employee SP
        let add_sp = Self::calculate_sp(sk.skill_level);
        employee.sp = old_sp.saturating_add(add_sp);

        if !employee_ver_skills.contains(&sk){
            //move skill from unverified to verified
            let index = employee_skills_unv.iter().position(|r| *r==sk).unwrap();
            SK::UserUnverifiedSkills::mutate(account.clone(),|list: &mut pallet_skills::BoundedVec<SK::Skill<T>, _>|{
                list.remove(index);
            });
            let test = employee_skills_unv.contains(&sk);
            match test{
                true => {
                    SK::UserVerifiedSkills::mutate(account.clone(),|list: &mut pallet_skills::BoundedVec<SK::Skill<T>, _>|{
                        list.try_push(sk.clone()).map_err(|_| "Max number of skills reached").ok();
                    });
                },
                false => ()
            }
           
        }

    }

    //Upgrade employee xp
    let sp = T::Sp::get();
    let xp = T::Xp::get();
   if employee.sp>old_sp && employee.sp % sp==0{
    employee.xp = old_xp.saturating_add(xp);
   }

   SK::EmployeeLog::<T>::mutate(account.clone(),|val|{
    *val = Some(employee);
   });

    Ok(().into())
}


pub fn begin_block(now: BlockNumberOf<T>) -> Weight{
    let max_block_weight = Weight::from_parts(1000_u64,0);
    if (now % T::CheckPeriod::get()).is_zero(){
        let proposal_iter = TasksProposalList::<T>::iter();
        for proposal_all in proposal_iter{
            let test = (proposal_all.2.session_closed,proposal_all.2.approved); 
            let prop = match test{
                (true,SK::Approvals::AWAITING) => 0,
                (true,SK::Approvals::YES) => 1,
                _ => 2,
            };
            if prop == 0 {
                let proposal = Call::<T>::reject_task
                {
                    account: proposal_all.0.clone()
                };

                let council_member = Coll::Pallet::<T,Instance1>::members()[0].clone();
                proposal.dispatch_bypass_filter(frame_system::RawOrigin::Signed(council_member).into()).ok();
                TasksProposalList::<T>::remove(&proposal_all.0.clone(),proposal_all.1);
            } else if prop == 1 {
                
                TasksProposalList::<T>::remove(&proposal_all.0,proposal_all.1);
            }
        }
        
    }
    max_block_weight
}


}