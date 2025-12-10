use super::*;

#[derive(Debug, Clone)]
pub struct Dropper {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub inventory: Vec<Item>,           //len 9 for the 3x3 grid
}

impl CommonBlockEntity for Dropper {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			inventory: vec![Item::default(); 9],
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Item] {
		return &mut self.inventory;
	}

	fn get_contained_items_owned(&self) -> Vec<Item> {
		return self.inventory.clone();
	}
}

impl From<Dropper> for Vec<NbtTag> {
	fn from(value: Dropper) -> Self {
		vec![value.inventory.into()]
	}
}

impl TryFrom<NbtListTag> for Dropper {
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

		let mut inventory = vec![Item::default(); 9];
		if let Some(items) = value.get_child("Items") {
			for entry in items.as_list() {
				inventory[entry.get_child("Slot").unwrap().as_byte() as usize] = Item {
					id: entry.get_child("id").unwrap().as_string().to_string(),
					count: entry.get_child("count").unwrap().as_int() as u8,
					components: Vec::new(),
				};
			}
		}

		return Ok(Dropper {
			position,
			components: Vec::new(),
			inventory,
		});
	}
}
