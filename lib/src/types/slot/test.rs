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

	#[test]
	fn returns_error_on_invalid_item_name() {
		let res = Slot::new_from_name("minecraft:item_that_will_never_ever_exist", 12);
		assert!(res.is_err_and(|x| x.to_string().contains("minecraft:item_that_will_never_ever_exist")));
	}
}

mod get_item_id {
	use super::*;

	#[test]
	fn returns_correct_id() {
		let slot = Slot::new(43, 12);
		let res = slot.get_item_id();
		assert_eq!(res, 43);
	}
}

mod get_item_name {
	use super::*;

	#[test]
	fn returns_correct_name() {
		let slot = Slot::new(1, 12);
		let res = slot.get_item_name();
		assert_eq!(res, "minecraft:stone");
	}
}

mod get_count {
	use super::*;

	#[test]
	fn returns_correct_count() {
		let slot = Slot::new(43, 12);
		let res = slot.get_count();
		assert_eq!(res, 12);
	}
}

mod get_max_stack_size {
	use super::*;

	#[test]
	fn returns_correct_value() {
		let slot = Slot::new(1, 12);
		let res = slot.get_max_stack_size();
		assert_eq!(res, 64);
	}
}

mod set_count_mut {
	use super::*;

	#[test]
	fn correctly_sets_to_0() {
		let mut slot = Slot::new(1, 12);
		slot.set_count_mut(0).unwrap();
		assert_eq!(slot, Slot { item_id: 1, count: 0 });
	}

	#[test]
	fn correctly_sets_to_10() {
		let mut slot = Slot::new(1, 12);
		slot.set_count_mut(10).unwrap();
		assert_eq!(slot, Slot { item_id: 1, count: 10 });
	}

	#[test]
	fn correctly_sets_to_max_stack_size() {
		let mut slot = Slot::new(1, 12);
		slot.set_count_mut(10).unwrap();
		assert_eq!(slot, Slot { item_id: 1, count: 10 });
	}

	#[test]
	fn fails_if_count_exceeds_max_stack_size() {
		let mut slot = Slot::new(1, 12);
		let res = slot.set_count_mut(65);
		assert!(res.is_err_and(|x| x.to_string() == SlotError::MaxStackSizeExceeded.to_string()));
		assert_eq!(slot, Slot { item_id: 1, count: 12 });
	}
}

mod add_count_mut {
	use super::*;

	#[test]
	fn adding_0_does_nothing() {
		let mut slot = Slot::new(1, 12);
		slot.add_count_mut(0).unwrap();
		assert_eq!(slot, Slot { item_id: 1, count: 12 });
	}

	#[test]
	fn correctly_adds_10() {
		let mut slot = Slot::new(1, 12);
		slot.add_count_mut(10).unwrap();
		assert_eq!(slot, Slot { item_id: 1, count: 22 });
	}

	#[test]
	fn correctly_adds_to_max_stack_size() {
		let mut slot = Slot::new(1, 12);
		slot.add_count_mut(52).unwrap();
		assert_eq!(slot, Slot { item_id: 1, count: 64 });
	}

	#[test]
	fn fails_if_count_exceeds_max_stack_size() {
		let mut slot = Slot::new(1, 12);
		let res = slot.add_count_mut(65);
		assert!(res.is_err_and(|x| x.to_string() == SlotError::MaxStackSizeExceeded.to_string()));
		assert_eq!(slot, Slot { item_id: 1, count: 12 });
	}
}

mod add_count_mut_with_overflow {
	use super::*;

	#[test]
	fn adding_0_does_nothing() {
		let mut slot = Slot::new(1, 12);
		let res = slot.add_count_mut_with_overflow(0).unwrap();
		assert_eq!(slot, Slot { item_id: 1, count: 12 });
		assert_eq!(res, Slot { item_id: 1, count: 0 });
	}

	#[test]
	fn correctly_adds_10() {
		let mut slot = Slot::new(1, 12);
		let res = slot.add_count_mut_with_overflow(10).unwrap();
		assert_eq!(slot, Slot { item_id: 1, count: 22 });
		assert_eq!(res, Slot { item_id: 1, count: 0 });
	}

	#[test]
	fn correctly_adds_to_max_stack_size() {
		let mut slot = Slot::new(1, 12);
		let res = slot.add_count_mut_with_overflow(52).unwrap();
		assert_eq!(slot, Slot { item_id: 1, count: 64 });
		assert_eq!(res, Slot { item_id: 1, count: 0 });
	}

	#[test]
	fn count_exceeds_max_stack_size() {
		let mut slot = Slot::new(1, 12);
		let res = slot.add_count_mut_with_overflow(65).unwrap();
		assert_eq!(slot, Slot { item_id: 1, count: 64 });
		assert_eq!(res, Slot { item_id: 1, count: 13 });
	}

	#[test]
	fn overflow_over_max_stack_size_returns_error() {
		let mut slot = Slot::new(1, 12);
		let res = slot.add_count_mut_with_overflow(200);
		assert_eq!(slot, Slot { item_id: 1, count: 12 });
		assert!(res.is_err_and(|x| x.to_string() == SlotError::MaxStackSizeExceeded.to_string()));
	}
}
