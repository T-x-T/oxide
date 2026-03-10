use super::*;

#[derive(Debug, Clone)]
pub struct DecoratedPot {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub item: Slot,
}

impl CommonBlockEntity for DecoratedPot {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			item: Slot::default(),
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Slot] {
		return &mut [];
	}

	fn get_contained_items_owned(&self) -> Vec<Slot> {
		return vec![self.item.clone()];
	}
}

impl From<DecoratedPot> for Vec<NbtTag> {
	fn from(value: DecoratedPot) -> Self {
		return vec![value.item.into()];
	}
}

impl TryFrom<NbtListTag> for DecoratedPot {
	type Error = Box<dyn Error>;

	fn try_from(value: NbtListTag) -> Result<Self, Self::Error> {
		let x = value.get_child("x").unwrap().as_int();
		let y = value.get_child("y").unwrap().as_int() as i16;
		let z = value.get_child("z").unwrap().as_int();
		let position = BlockPosition {
			x,
			y,
			z,
		};

		let mut item = Slot::default();
		if let Some(raw_item) = value.get_child("Item") {
			item = Slot {
				id: data::items::get_item_id_by_name(raw_item.get_child("id").unwrap().as_string()),
				count: raw_item.get_child("count").unwrap().as_int(),
				components_to_add: Vec::new(),
				components_to_remove: Vec::new(),
			};
		}

		return Ok(DecoratedPot {
			position,
			components: Vec::new(),
			item,
		});
	}
}
