use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::Zero;

#[test]
fn create_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		//assert_eq!(Kitties::<Test>::get(0u32.into()), Some((1, frame_system::Pallet::<Test>::block_number())));
		let kitty_id: KittyIndex = Zero::zero();
		assert_eq!(Owner::<Test>::get(&kitty_id), Some(1u64));
		assert_eq!(KittiesCount::<Test>::get(), Some(1u32));
	});
}

#[test]
fn create_failed_by_not_enough_token() {
	new_test_ext().execute_with(|| {
		assert_noop!(KittiesModule::create(Origin::signed(7)), Error::<Test>::NotEnoughToken);
	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let kitty_id: KittyIndex = Zero::zero();
		assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, kitty_id.clone()));
		assert_eq!(Owner::<Test>::get(&kitty_id), Some(2u64));
		assert_eq!(KittiesCount::<Test>::get(), Some(1u32));
	});
}

#[test]
fn transfer_failed_by_not_owner() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_noop!(
			KittiesModule::transfer(Origin::signed(2), 3, 0u32.into()),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn bread_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(2)));
		let kitty_id_1: KittyIndex = Zero::zero();
		let kitty_id_2: KittyIndex = 1u32.into();
		assert_ok!(KittiesModule::bread(Origin::signed(1), kitty_id_1.clone(), kitty_id_2.clone()));
		let new_kitty_id: KittyIndex = 2u32.into();
		assert_eq!(Owner::<Test>::get(&new_kitty_id), Some(1u64));
		assert_eq!(KittiesCount::<Test>::get(), Some(3u32));
	});
}

#[test]
fn bread_failed_by_same_parent() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(2)));
		assert_noop!(
			KittiesModule::bread(Origin::signed(1), 0u32.into(), 0u32.into()),
			Error::<Test>::SameParentIndex
		);
	});
}

#[test]
fn set_price_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let price: u64 = 20;
		let kitty_id: KittyIndex = Zero::zero();
		assert_ok!(KittiesModule::set_price(Origin::signed(1), price, kitty_id));
		assert_eq!(KittyPrice::<Test>::get(&kitty_id), Some(20u64));
	});
}

#[test]
fn set_price_failed_by_not_owner() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_noop!(
			KittiesModule::set_price(Origin::signed(2), 20u64, 0u32),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn buy_kitty_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let price: u64 = 20;
		let kitty_id: KittyIndex = Zero::zero();
		assert_ok!(KittiesModule::set_price(Origin::signed(1), price, kitty_id));
		assert_ok!(KittiesModule::buy_kitty(Origin::signed(2), price, kitty_id));
		assert_eq!(Owner::<Test>::get(&kitty_id), Some(2u64));
	});
}
#[test]
fn buy_kitty_failed_by_not_sale() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::set_price(Origin::signed(1), 20u64, 0u32));
		assert_ok!(KittiesModule::buy_kitty(Origin::signed(2), 20u64, 0u32));
		assert_noop!(
			KittiesModule::buy_kitty(Origin::signed(3), 20u64, 0u32),
			Error::<Test>::KittyNotSale
		);
	});
}
