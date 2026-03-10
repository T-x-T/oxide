use super::*;

#[derive(Debug, Clone)]
pub struct Dispenser {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub inventory: Vec<Slot>,           //len 9 for the 3x3 grid
}

impl CommonBlockEntity for Dispenser {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			inventory: vec![Slot::default(); 9],
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Slot] {
		return &mut self.inventory;
	}

	fn get_contained_items_owned(&self) -> Vec<Slot> {
		return self.inventory.clone();
	}
}

impl From<Dispenser> for Vec<NbtTag> {
	fn from(value: Dispenser) -> Self {
		vec![value.inventory.into()]
	}
}

impl TryFrom<NbtListTag> for Dispenser {
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

		let mut inventory = vec![Slot::default(); 9];
		if let Some(items) = value.get_child("Items") {
			for entry in items.as_list() {
				inventory[entry.get_child("Slot").unwrap().as_byte() as usize] = Slot {
					id: data::items::get_item_id_by_name(entry.get_child("id").unwrap().as_string()),
					count: entry.get_child("count").unwrap().as_int(),
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				};
			}
		}

		return Ok(Dispenser {
			position,
			components: Vec::new(),
			inventory,
		});
	}
}
