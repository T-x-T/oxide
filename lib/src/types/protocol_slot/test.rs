use super::*;

mod from_protocol_slot_for_slot {
	use super::*;

	#[test]
	fn correctly_converts_count_and_id() {
		let input = ProtocolSlot { count: 12, id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() };
		let res = Slot::from(input);
		assert_eq!(res.get_count(), 12);
		assert_eq!(res.get_item_id(), 1);
	}
}

mod from_slot_for_protocol_slot {
	use super::*;

	#[test]
	fn correctly_converts_count_and_id() {
		let input = Slot::new(1, 12);
		let res = ProtocolSlot::from(input);
		assert_eq!(res.count, 12);
		assert_eq!(res.id, 1);
	}
}

mod from_option_protocol_slot_for_slot {
	use super::*;

	#[test]
	fn correctly_converts_count_and_id_with_count_12() {
		let input = Some(ProtocolSlot { count: 12, id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
		let res = Slot::from(input);
		assert_eq!(res.get_count(), 12);
		assert_eq!(res.get_item_id(), 1);
	}

	#[test]
	fn correctly_converts_count_and_id_with_count_0() {
		let input = Some(ProtocolSlot { count: 0, id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
		let res = Slot::from(input);
		assert_eq!(res.get_count(), 0);
		assert_eq!(res.get_item_id(), 1);
	}

	#[test]
	fn correctly_converts_count_and_id_with_none() {
		let input = None;
		let res = Slot::from(input);
		assert_eq!(res.get_count(), 0);
		assert_eq!(res.get_item_id(), 0);
	}
}

mod from_slot_for_option_protocol_slot {
	use super::*;

	#[test]
	fn correctly_converts_count_and_id() {
		let input = Slot::new(1, 12);
		let res = Option::<ProtocolSlot>::from(input).unwrap();
		assert_eq!(res.count, 12);
		assert_eq!(res.id, 1);
	}

	#[test]
	fn returns_none_with_count_0() {
		let input = Slot::new(1, 0);
		let res = Option::<ProtocolSlot>::from(input);
		assert!(res.is_none());
	}
}
