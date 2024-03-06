use crate::mock::*;
use frame_support:: assert_ok;
pub use super::*;

fn next_block() {
	System::set_block_number(System::block_number() + 1);
	SkillsModule::begin_block(System::block_number());
    MarketModule::begin_block(System::block_number());
}


fn fast_forward_to(n: u64) {
	while System::block_number() < n {
		next_block();
	}
}

#[test]
fn market_test(){
    new_test_ext().execute_with(||{

        let metadata0: BoundedVec<u8, <Test as pallet_skills::Config>::StringLimit>=
			b"Richard Geere".to_vec().try_into().unwrap();

        let metadata1: BoundedVec<u8, <Test as pallet_skills::Config>::StringLimit>=
			b"Dave Poirier".to_vec().try_into().unwrap();

        let metadata2: BoundedVec<u8, <Test as pallet_skills::Config>::StringLimit>=
			b"Eve Picasso ".to_vec().try_into().unwrap();

        let metadata3: BoundedVec<u8,<Test as pallet_skills::Config>::StringLimit> =
			b"Communication Skills".to_vec().try_into().unwrap();

		let metadata4: BoundedVec<u8,<Test as pallet_skills::Config>::StringLimit> =
			b"Rust Programming".to_vec().try_into().unwrap();

        let metadata5: BoundedVec<u8,<Test as pallet_skills::Config>::StringLimit> =
			b"I need an updated substrate-node template. send you github repo by e-mail when you're done.".to_vec().try_into().unwrap();

        let council = Collective::members();
		assert_eq!(council.len(),3);

        //create 3 new employees
     assert_ok!(SkillsModule::new_employee(RuntimeOrigin::signed(council[0].clone()),RICHARD,metadata0));
     assert_ok!(SkillsModule::new_employee(RuntimeOrigin::signed(council[0].clone()),DAVE,metadata1));
     assert_ok!(SkillsModule::new_employee(RuntimeOrigin::signed(council[0].clone()),EVE,metadata2));

     //Richard propose a new skill
     assert_ok!(SkillsModule::submit_skill(RuntimeOrigin::signed(RICHARD), metadata4.clone(), Stype::Technical, SLevel::Level3));
     assert_ok!(SkillsModule::submit_skill(RuntimeOrigin::signed(DAVE), metadata3.clone(), Stype::Soft, SLevel::Level4));

// Council Vote positively
     let mut proposal = SkillsModule::get_proposal(RICHARD).unwrap();
     assert_eq!(proposal.approved,Sapproval::AWAITING);
     assert_ok!(SkillsModule::council_vote(RuntimeOrigin::signed(council[0].clone()), RICHARD, true));
     assert_ok!(SkillsModule::council_vote(RuntimeOrigin::signed(council[1].clone()), RICHARD, true));
     assert_ok!(SkillsModule::council_vote(RuntimeOrigin::signed(council[2].clone()), RICHARD, true));
     assert_ok!(SkillsModule::council_close(RuntimeOrigin::signed(council[2].clone()), RICHARD));

     let mut  proposal0 = SkillsModule::get_proposal(DAVE).unwrap();
     assert_eq!(proposal0.approved,Sapproval::AWAITING);
     assert_ok!(SkillsModule::council_vote(RuntimeOrigin::signed(council[0].clone()), DAVE, true));
     assert_ok!(SkillsModule::council_vote(RuntimeOrigin::signed(council[1].clone()), DAVE, true));
     assert_ok!(SkillsModule::council_vote(RuntimeOrigin::signed(council[2].clone()), DAVE, true));
     assert_ok!(SkillsModule::council_close(RuntimeOrigin::signed(council[2].clone()), DAVE));


     proposal = SkillsModule::get_proposal(RICHARD).unwrap();
     proposal0 = SkillsModule::get_proposal(DAVE).unwrap();
     assert_eq!(proposal0.approved,Sapproval::YES);
     assert_eq!(proposal.approved,Sapproval::YES);

// Dave adds both skills to his profile, Richard only adds one

assert_ok!(SkillsModule::add_my_skills(RuntimeOrigin::signed(DAVE), 0));
assert_ok!(SkillsModule::add_my_skills(RuntimeOrigin::signed(DAVE), 1));
assert_ok!(SkillsModule::add_my_skills(RuntimeOrigin::signed(RICHARD), 0));

let dave_skills = SkillsModule::user_unv_skills(DAVE);
//println!("Dave skills:{:?}",dave_skills);

//Eve submits a task proposal
assert_ok!(MarketModule::propose_task(RuntimeOrigin::signed(EVE), 0, 500, metadata5, RICHARD));


//Council votes on Eve proposal


assert_ok!(MarketModule::council_vote(RuntimeOrigin::signed(council[0].clone()), EVE, true));
assert_ok!(MarketModule::council_vote(RuntimeOrigin::signed(council[1].clone()), EVE, true));
assert_ok!(MarketModule::council_vote(RuntimeOrigin::signed(council[2].clone()), EVE, true));
let b = MarketModule::council_close(RuntimeOrigin::signed(council[2].clone()), EVE);

//println!("Events!!!:\n\n{:?}\n\n",System::events());
//assert_ok!(Bounties::approve_bounty(RuntimeOrigin::root(), 0));
let  event_ref = 
		record(RuntimeEvent::Bounties(pallet_bounties::Event::BountyApproved{index:0}));
		assert_eq!(true,System::events().contains(&event_ref));



let mut now = System::block_number();
     









    })     
}