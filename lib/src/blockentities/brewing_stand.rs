use super::*;

#[derive(Debug, Clone)]
pub struct BrewingStand {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub items: Vec<Slot>,               //0: left, 1: middle, 2: right, 3: ingredient, 4: fuel
	pub brew_time: i16,
	pub fuel: u8,
}

impl CommonBlockEntity for BrewingStand {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			items: vec![Slot::default(); 5],
			brew_time: 0,
			fuel: 0,
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Slot] {
		return &mut self.items;
	}

	fn get_contained_items_owned(&self) -> Vec<Slot> {
		return self.items.clone();
	}
}

impl From<BrewingStand> for Vec<NbtTag> {
	fn from(value: BrewingStand) -> Self {
		return vec![value.items.into(), NbtTag::Short("BrewTime".to_string(), value.brew_time), NbtTag::Byte("Fuel".to_string(), value.fuel)];
	}
}

impl TryFrom<NbtListTag> for BrewingStand {
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

		let mut inventory = vec![Slot::default(); 5];
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

		let brew_time = value.get_child("BrewTime").unwrap_or(&NbtTag::Short(String::new(), 0)).as_short();
		let fuel = value.get_child("Fuel").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte();

		return Ok(BrewingStand {
			position,
			components: Vec::new(),
			items: inventory,
			brew_time,
			fuel,
		});
	}
}
