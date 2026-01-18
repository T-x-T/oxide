use super::*;

#[derive(Debug, Clone)]
pub struct Beacon {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub primary_potion_effect: Option<String>,
	pub secondary_potion_effect: Option<String>,
}

impl CommonBlockEntity for Beacon {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			primary_potion_effect: None,
			secondary_potion_effect: None,
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Item] {
		return &mut [];
	}

	fn get_contained_items_owned(&self) -> Vec<Item> {
		return Vec::new();
	}
}

impl From<Beacon> for Vec<NbtTag> {
	fn from(value: Beacon) -> Self {
		let mut output: Vec<NbtTag> = Vec::new();

		if let Some(primary_effect) = value.primary_potion_effect {
			output.push(NbtTag::String("primary_effect".to_string(), primary_effect));
		}
		if let Some(secondary_effect) = value.secondary_potion_effect {
			output.push(NbtTag::String("secondary_effect".to_string(), secondary_effect));
		}

		return output;
	}
}

impl TryFrom<NbtListTag> for Beacon {
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

		return Ok(Beacon {
			position,
			components: Vec::new(),
			primary_potion_effect: value.get_child("primary_effect").map(|x| x.as_string().to_string()),
			secondary_potion_effect: value.get_child("secondary_effect").map(|x| x.as_string().to_string()),
		});
	}
}
