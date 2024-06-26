
pub use super::*;
use sp_runtime::{traits::Lookup, Percent};
use frame_support::traits::OriginTrait;
impl<T: Config> Pallet<T> {

    //Helper function for skill approval
    pub fn approve_skill_helper(from_who:T::AccountId) -> DispatchResultWithPostInfo{
        let skill0 = SkillsApprovalList::<T>::iter();
        let mut exist = false;

        for skill in skill0{
            //Check that account is an employee 
            if from_who==skill.0{
                let mut sk = skill.1.clone();
                sk.skill_number=Self::skills().into_inner().len() as u8;
				sk.confirmed=true;

                //Add skill to skill database
                Skills::<T>::mutate(|list|{
                    list.try_push(sk.clone()).map_err(|_| "Max number of skills reached").ok();
                });

				SkillsProposalList::<T>::mutate(&from_who,|val|{
					let mut proposal = val.clone().unwrap();
					proposal.approved = Approvals::YES;
					*val = Some(proposal);
					});

                //Remove skill from waiting list
                SkillsApprovalList::<T>::remove(&from_who);
                
                exist = true;
                break;
            }
        }
        ensure!(exist,Error::<T>::NoSkillSubmited);
        Ok(().into())
    }   



    //Helper function for skill rejection
    pub fn reject_skill_helper(from_who:T::AccountId) -> DispatchResultWithPostInfo{
        let skills = SkillsApprovalList::<T>::iter();
        
        let mut exist = false;

        for skill in skills{
            //Check that account is an employee 
            if from_who==skill.0{

				SkillsProposalList::<T>::mutate(&from_who,|val|{
					let mut proposal = val.clone().unwrap();
					proposal.approved = Approvals::NO;
					*val = Some(proposal);
					});
                //Remove skill from waiting list
                SkillsApprovalList::<T>::remove(&from_who);
                
                exist = true;
                break;
            }
        }
        ensure!(exist,Error::<T>::NoSkillSubmited);
        Ok(().into())
    }   
    pub fn get_origin(account_id: AccountIdOf<T>) -> <T as frame_system::Config>::RuntimeOrigin {
		frame_system::RawOrigin::Signed(account_id).into()
	}

    pub fn start_council_session(account: T::AccountId, skill:Skill<T>) -> DispatchResultWithPostInfo{

        //Create proposal
		let proposal0 = 
        Call::<T>::approve_skill{
            account: account.clone()
        };
        let proposal0 = Self::get_formatted_call(proposal0.into());
		let proposal = proposal0.unwrap();
        let _hash = T::Hashing::hash_of(&proposal);
        let proposal_len:u32 = proposal.using_encoded(|p| p.len() as u32);
		
		let council_member = Coll::Pallet::<T,Instance1>::members()[0].clone();
		let council_origin= Self::get_origin(council_member);

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
				let mut proposal_all = SkillProposal::<T>::new(account.clone(), Some(skill.clone()),proposal_hash.clone());
				proposal_all.proposal_index = index;
				proposal_all.proposal_hash = proposal_hash;
				SkillsProposalList::<T>::insert(&account, proposal_all);
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

    pub fn vote_action(caller: T::AccountId,candidate_account: T::AccountId,approve:bool) -> DispatchResultWithPostInfo{
		
		// Check that the caller is a council member
		ensure!(
			Coll::Pallet::<T, Instance1>::members().contains(&caller),
			Error::<T>::NotACouncilMember
		);
		// Check that the proposal exists
		ensure!(
			SkillsProposalList::<T>::contains_key(&candidate_account),
			Error::<T>::ProposalDoesNotExist
		);
		let proposal_all = Self::get_proposal(candidate_account.clone()).unwrap();
		let proposal_hash = proposal_all.proposal_hash;
		let proposal_index = proposal_all.proposal_index;
		let origin = Self::get_origin(caller.clone());
		// Execute the council vote
		Coll::Pallet::<T, Instance1>::vote(
			origin,
			proposal_hash,
			proposal_index,
			approve,
		).ok();

		Ok(().into())
	}

    pub fn closing_vote(caller: T::AccountId,candidate_account: T::AccountId) -> DispatchResultWithPostInfo{

		// Check that the caller is a council member
		ensure!(
			Coll::Pallet::<T, Instance1>::members().contains(&caller),
			Error::<T>::NotACouncilMember
		);
		// Check that the proposal exists
		ensure!(
			SkillsProposalList::<T>::contains_key(&candidate_account),
			Error::<T>::ProposalDoesNotExist
		);
		let proposal_all = Self::get_proposal(candidate_account.clone()).unwrap();
		let proposal_hash = proposal_all.proposal_hash;
		let proposal = Coll::Pallet::<T,Instance1>::proposal_of(proposal_hash.clone()).unwrap();
		let proposal_len = proposal.clone().encoded_size();
		let index = proposal_all.proposal_index;
		let proposal_weight = proposal.get_dispatch_info().weight;
		let origin = Self::get_origin(caller.clone());
		Coll::Pallet::<T,Instance1>::close(
			origin,
			proposal_hash,
			index,
			proposal_weight,
			proposal_len as u32,
		).ok();

		SkillsProposalList::<T>::mutate(&candidate_account,|val|{
			let mut proposal = val.clone().unwrap();
			proposal.session_closed = true;
			*val = Some(proposal);
			});

		Ok(().into())

	}

	
	pub fn increase_wage(account:AccountIdOf<T>, mut employee:Employee<T>, increase:bool)-> DispatchResultWithPostInfo{
		let sp_diff = T::Sp::get();
		let old_xp = employee.xp;
		let new_xp = employee.sp%sp_diff;
		
	   if increase==true && new_xp>old_xp{   
		employee.xp = new_xp;
		employee.wage =employee.wage.saturating_add(Percent::from_percent(new_xp as u8).mul_floor(employee.wage)) ;
	  
	   }else{
		employee.wage =employee.wage.saturating_sub(Percent::from_percent(old_xp as u8).mul_floor(employee.wage)) ;
	   }
		
	
	   EmployeeLog::<T>::mutate(account.clone(),|val|{
		*val = Some(employee);
	   });
		Ok(().into())
	}

    pub fn begin_block(now: BlockNumberOf<T>) -> Weight{
		let max_block_weight = Weight::from_parts(1000_u64,0);
		let pay_cycle= T::CheckCycle::get();
		if (now%pay_cycle).is_zero(){
			//get list of employees
			let employees:Vec<_> = EmployeeLog::<T>::iter().collect();

			//Convert the duration of a payment cycle into u128
			let pcycle0:u128=pay_cycle.try_into().ok().unwrap();

			let now0:u128 = now.try_into().ok().unwrap();
			
			//calculate number of payment cycles so far(pcycle0)
			let cycle0 = now0.saturating_div(pcycle0);

			for employee in employees{
				let mut emp= employee.1.clone();
				let salary0 = emp.wage;
				let s0:u128= salary0.try_into().ok().unwrap();
				let mut salary: T::Balance =s0.try_into().ok().unwrap();

				//calculate the cycle based salary
				salary = salary.saturating_mul(pcycle0.try_into().ok().unwrap());

				

				
				let payment_fund: T::AccountId = T::BudgetAccount::get().into_account_truncating();
				//pay employee if there is enough money in the fund
				let fund_bal = BALANCES::Pallet::<T>::free_balance(&payment_fund);
				//debug_assert!(fund_bal>salary,"The salary is greater than the fund!!!");
				if fund_bal>salary{
					//debug_assert!(emp.payment_cycle<cycle0, "payment cycles are up-to-date!");
					while emp.payment_cycle<cycle0 && fund_bal>salary {
					let dest= <T as frame_system::Config>::Lookup::unlookup(employee.clone().0);
					BALANCES::Pallet::<T>::transfer_keep_alive(<T as frame_system::Config>::RuntimeOrigin::signed(payment_fund.clone()), dest, salary).ok();
					emp.payment_cycle=emp.payment_cycle+1;
				}				
				}
				//Update payed cycles number in employee's profile
				EmployeeLog::<T>::mutate(employee.clone().0,|val|{
					*val=Some(emp);
				});
			}
		}
		if (now % T::CheckPeriod::get()).is_zero(){
			
			let proposal_iter = SkillsProposalList::<T>::iter();
			for proposal_all in proposal_iter{
				let test = (proposal_all.1.session_closed,proposal_all.1.approved); 
				let prop = match test{
					(true,Approvals::AWAITING) => 0,
					(true,Approvals::YES) => 1,
					_ => 2,
				};
				if prop == 0 {
					let proposal = Call::<T>::reject_skill
					{
						account: proposal_all.0.clone()
					};

					let council_member = Coll::Pallet::<T,Instance1>::members()[0].clone();
					proposal.dispatch_bypass_filter(frame_system::RawOrigin::Signed(council_member).into()).ok();
					SkillsProposalList::<T>::remove(&proposal_all.0.clone());
				} else if prop == 1 {
					SkillsProposalList::<T>::remove(&proposal_all.0);
				}
			}
			
		}
		max_block_weight
	}





}