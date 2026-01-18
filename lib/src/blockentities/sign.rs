use super::*;

#[derive(Debug, Clone)]
pub struct Sign {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub front_text: NbtTag,
	pub back_text: NbtTag,
	pub is_waxed: bool,
}

impl CommonBlockEntity for Sign {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			front_text: NbtTag::TagCompound("front_text".to_string(), Vec::new()),
			back_text: NbtTag::TagCompound("back_text".to_string(), Vec::new()),
			is_waxed: false,
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Item] {
		return &mut [];
	}

	fn get_contained_items_owned(&self) -> Vec<Item> {
		return Vec::new();
	}
}

impl From<Sign> for Vec<NbtTag> {
	fn from(value: Sign) -> Self {
		return vec![NbtTag::Byte("is_waxed".to_string(), if value.is_waxed { 1 } else { 0 }), value.front_text, value.back_text];
	}
}

impl TryFrom<NbtListTag> for Sign {
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

		return Ok(Sign {
			position,
			components: Vec::new(),
			front_text: value.get_child("front_text").unwrap_or(&NbtTag::TagCompound("front_text".to_string(), Vec::new())).clone(),
			back_text: value.get_child("back_text").unwrap_or(&NbtTag::TagCompound("back_text".to_string(), Vec::new())).clone(),
			is_waxed: value.get_child("is_waxed").unwrap_or(&NbtTag::Byte("is_waxed".to_string(), 0)).as_byte() == 1,
		});
	}
}
