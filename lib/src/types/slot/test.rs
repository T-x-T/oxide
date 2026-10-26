use super::*;

mod new {
	use super::*;

	#[test]
	fn sets_contents_correctly() {
		let res = Slot::new(1, 12);
		assert_eq!(res, Slot { item_id: 1, count: 12 });
	}
}

mod new_from_name {
	use super::*;

	#[test]
	fn sets_contents_correctly() {
		let res = Slot::new_from_name("minecraft:stone", 12);
		assert!(res.is_ok_and(|x| x == Slot { item_id: 1, count: 12 }));
	}
}
