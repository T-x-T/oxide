use super::*;

#[derive(Debug, Clone)]
pub struct BrewingStand {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub items: Vec<Item>,               //0: left, 1: middle, 2: right, 3: ingredient, 4: fuel
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
			items: vec![Item::default(); 5],
			brew_time: 0,
			fuel: 0,
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Item] {
		return &mut self.items;
	}

	fn get_contained_items_owned(&self) -> Vec<Item> {
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

		let mut inventory = vec![Item::default(); 5];
		if let Some(items) = value.get_child("Items") {
			for entry in items.as_list() {
				inventory[entry.get_child("Slot").unwrap().as_byte() as usize] = Item {
					id: entry.get_child("id").unwrap().as_string().to_string(),
					count: entry.get_child("count").unwrap().as_int() as u8,
					components: Vec::new(),
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
