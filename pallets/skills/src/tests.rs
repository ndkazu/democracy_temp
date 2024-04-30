
use crate::mock::*;
use frame_support:: assert_ok;
pub use super::*;

fn next_block() {
	System::set_block_number(System::block_number() + 1);
	SkillsModule::begin_block(System::block_number());
}


fn fast_forward_to(n: u64) {
	while System::block_number() < n {
		next_block();
	}
}

#[test]
fn employee_test(){
    new_test_ext().execute_with(||{

		let metadata0: BoundedVec<u8, <Test as Config>::StringLimit>=
			b"Richard Geere".to_vec().try_into().unwrap();

			let metadata1: BoundedVec<u8,<Test as Config>::StringLimit> =
			b"Rust Programming".to_vec().try_into().unwrap();

        let council = Collective::members();
		assert_eq!(council.len(),3);

        //create a new employee
        assert_ok!(SkillsModule::new_employee(RuntimeOrigin::signed(council[0].clone()),RICHARD,metadata0));

		//Get a budget for Salaries payments
		assert_ok!(SkillsModule::set_budget(RuntimeOrigin::root()));
		let budget_account = <Test as Config>::BudgetAccount::get().into_account_truncating();
		let bal = Balances::free_balance(&budget_account);
		println!("The budget account is: ${budget_account}");
		assert_eq!(bal,10_000_000_000*BSX);

		//richard initial balance
		let rich_bal = Balances::free_balance(&RICHARD);
		let mut now = System::block_number();
		println!("the blocknumber is: {:?}\n",now);
		now = System::block_number().saturating_mul(<Test as Config>::CheckCycle::get());
		println!("the new blocknumber is: {:?}\n",now);
		fast_forward_to(now);
		assert_ne!(rich_bal,Balances::free_balance(&RICHARD));
		println!("Richard balance:{:?}",Balances::free_balance(&RICHARD));
		
		

        //Employee propose a new skill
		assert_ok!(SkillsModule::submit_skill(RuntimeOrigin::signed(RICHARD), metadata1.clone(), Stype::Technical, SLevel::Level3));
		let proposal = SkillsModule::get_proposal(RICHARD).unwrap();
		assert_eq!(proposal.approved,Sapproval::AWAITING);
		assert_ok!(SkillsModule::council_vote(RuntimeOrigin::signed(council[0].clone()), RICHARD, true));
		assert_ok!(SkillsModule::council_vote(RuntimeOrigin::signed(council[1].clone()), RICHARD, true));
		assert_ok!(SkillsModule::council_vote(RuntimeOrigin::signed(council[2].clone()), RICHARD, true));
		assert_ok!(SkillsModule::council_close(RuntimeOrigin::signed(council[2].clone()), RICHARD));

		
		expect_events(vec![
			RuntimeEvent::SkillsModule(Event::CouncilSessionClosed{ 
				who: council[2].clone(), 
				proposal_index: 0, 
				when: now.clone()
			})
		]);

		//council vote


		
		let  event_ref = 
		record(RuntimeEvent::SkillsModule(Event::NewSkillCreated{when: System::block_number(), what: metadata1}));
		assert_eq!(true,System::events().contains(&event_ref));

		
		now = System::block_number().saturating_mul(<Test as Config>::CheckPeriod::get());
		fast_forward_to(now);
		
		//Employee add an unverified skill to his profile
		let skill = SkillsModule::skills().into_inner();
		let skill0 = skill.clone()[0].clone();


		assert_ok!(SkillsModule::add_my_skills(RuntimeOrigin::signed(RICHARD), 0));
		assert_eq!(SkillsModule::user_unv_skills(RICHARD).into_inner()[0].clone(), skill0)
		
    })
}