use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

// 测试正常存证
#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), Some((1, frame_system::Pallet::<Test>::block_number())));
	});
}

// 测试重复存证
#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofAlreadyExist);
	})
}

// 测试长度溢出
#[test]
fn create_claim_failed_when_claim_over_len() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1,2,3,4];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::OverLen);
	})
}

// 测试正常吊销存证
#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), None);
	})
}

// 测试吊销不存在存证
#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];

		assert_noop!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()),Error::<Test>::ClaimNotExist);
	})
}

// 测试吊销他人的存证
#[test]
fn revoke_claim_failed_when_claim_is_not_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(PoeModule::revoke_claim(Origin::signed(2), claim.clone()), Error::<Test>::NotClaimOwner);
	})
}

// 测试吊销长度超过范围的存证
#[test]
fn revoke_claim_failed_when_claim_is_over_len() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1,2,3];

		assert_noop!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()),Error::<Test>::OverLen);
	})
}

// 测试正常转移存证拥有人
#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
		assert_eq!(Proofs::<Test>::get(&claim), Some((2, frame_system::Pallet::<Test>::block_number())));
	})
}

// 测试转移不存在的存证
#[test]
fn transfer_claim_failed_when_proof_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		
		assert_noop!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2), Error::<Test>::ProofNotExist);
	})
}

// 测试转移别人的存证
#[test]
fn transfer_claim_failed_when_claim_is_not_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3), Error::<Test>::NotClaimOwner);
	})
}

// 测试转移长度超限的存证
#[test]
fn transfer_claim_failed_when_claim_is_over_len() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1,2,3];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3), Error::<Test>::OverLen);
	})
}
