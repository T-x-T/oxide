use super::*;

#[derive(Debug, Clone)]
pub struct TrappedChest {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub inventory: Vec<Slot>,
}

impl CommonBlockEntity for TrappedChest {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			inventory: vec![Slot::default(); 27],
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Slot] {
		return &mut self.inventory;
	}

	fn get_contained_items_owned(&self) -> Vec<Slot> {
		return self.inventory.clone();
	}
}

impl From<TrappedChest> for Vec<NbtTag> {
	fn from(value: TrappedChest) -> Self {
		vec![value.inventory.into()]
	}
}

impl TryFrom<NbtListTag> for TrappedChest {
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

		let mut inventory = vec![Slot::default(); 27];
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

		return Ok(TrappedChest {
			position,
			components: Vec::new(),
			inventory,
		});
	}
}
