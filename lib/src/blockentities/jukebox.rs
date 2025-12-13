use super::*;

#[derive(Debug, Clone)]
pub struct Jukebox {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
}

impl CommonBlockEntity for Jukebox {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Item] {
		return &mut [];
	}

	fn get_contained_items_owned(&self) -> Vec<Item> {
		return Vec::new();
	}
}

impl From<Jukebox> for Vec<NbtTag> {
	fn from(_value: Jukebox) -> Self {
		return Vec::new();
	}
}

impl TryFrom<NbtListTag> for Jukebox {
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

		return Ok(Jukebox {
			position,
			components: Vec::new(),
		});
	}
}
