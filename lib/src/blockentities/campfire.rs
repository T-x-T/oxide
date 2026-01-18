use super::*;

#[derive(Debug, Clone)]
pub struct Campfire {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub items: Vec<Item>,               //up to 4
	pub cooking_times: Vec<i32>,
	pub cooking_total_times: Vec<i32>,
}

impl CommonBlockEntity for Campfire {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			items: Vec::new(),
			cooking_times: Vec::new(),
			cooking_total_times: Vec::new(),
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Item] {
		return &mut self.items;
	}

	fn get_contained_items_owned(&self) -> Vec<Item> {
		return self.items.clone();
	}
}

impl From<Campfire> for Vec<NbtTag> {
	fn from(value: Campfire) -> Self {
		return vec![
			value.items.into(),
			NbtTag::IntArray("CookingTimes".to_string(), value.cooking_times),
			NbtTag::IntArray("CookingTotalTimes".to_string(), value.cooking_total_times),
		];
	}
}

impl TryFrom<NbtListTag> for Campfire {
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

		let items: Vec<Item> = value
			.get_child("Items")
			.unwrap_or(&NbtTag::List(String::new(), Vec::new()))
			.as_list()
			.iter()
			.map(|entry| Item {
				id: entry.get_child("id").unwrap().as_string().to_string(),
				count: entry.get_child("count").unwrap().as_int() as u8,
				components: Vec::new(),
			})
			.collect();

		let cooking_times = value.get_child("CookingTimes").unwrap_or(&NbtTag::IntArray(String::new(), Vec::new())).as_int_array();
		let cooking_total_times = value.get_child("CookingTotalTimes").unwrap_or(&NbtTag::IntArray(String::new(), Vec::new())).as_int_array();

		return Ok(Campfire {
			position,
			components: Vec::new(),
			items,
			cooking_times,
			cooking_total_times,
		});
	}
}
