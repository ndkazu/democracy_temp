use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
pub use super::*;

fn next_block() {
	System::set_block_number(System::block_number() + 1);
	SkillsModule::begin_block(System::block_number());
}

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

fn fast_forward_to(n: u64) {
	while System::block_number() < n {
		next_block();
	}
}

#[test]
fn employee_test(){
    new_test_ext().execute_with(||{

        let council = Collective::members();
		assert_eq!(council.len(),3);

        //create a new employee
        assert_ok!(SkillsModule::new_employee(RuntimeOrigin::signed(council[0].clone()),RICHARD,bvec![0,0,3]));

        //Employee
    })
}