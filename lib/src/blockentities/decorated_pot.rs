use super::*;

#[derive(Debug, Clone)]
pub struct DecoratedPot {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub item: Item,
}

impl CommonBlockEntity for DecoratedPot {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			item: Item::default(),
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Item] {
		return &mut [];
	}

	fn get_contained_items_owned(&self) -> Vec<Item> {
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

		let mut item = Item::default();
		if let Some(raw_item) = value.get_child("Item") {
			item = Item {
				id: raw_item.get_child("id").unwrap().as_string().to_string(),
				count: raw_item.get_child("count").unwrap().as_int() as u8,
				components: Vec::new(),
			};
		}

		return Ok(DecoratedPot {
			position,
			components: Vec::new(),
			item,
		});
	}
}
