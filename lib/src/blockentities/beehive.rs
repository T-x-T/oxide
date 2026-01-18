use super::*;

#[derive(Debug, Clone)]
pub struct Beehive {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub bees: Vec<Bee>,
	pub flower_position: Option<BlockPosition>,
}

#[derive(Debug, Clone)]
pub struct Bee {
	entity_data: Vec<NbtTag>,
	min_ticks_in_hive: i32,
	ticks_in_hive: i32,
}

impl CommonBlockEntity for Beehive {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			bees: Vec::new(),
			flower_position: None,
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Item] {
		return &mut [];
	}

	fn get_contained_items_owned(&self) -> Vec<Item> {
		return Vec::new();
	}
}

impl From<Beehive> for Vec<NbtTag> {
	fn from(value: Beehive) -> Self {
		let bees = NbtTag::List(
			"bees".to_string(),
			value
				.bees
				.into_iter()
				.map(|bee| {
					NbtListTag::TagCompound(vec![
						NbtTag::TagCompound("entity_data".to_string(), bee.entity_data),
						NbtTag::Int("min_ticks_in_hive".to_string(), bee.min_ticks_in_hive),
						NbtTag::Int("ticks_in_hive".to_string(), bee.ticks_in_hive),
					])
				})
				.collect(),
		);

		let flower_position =
			value.flower_position.map(|x| NbtTag::IntArray("flower_pos".to_string(), vec![x.x, x.y as i32, x.z])).unwrap_or_default();

		return vec![bees, flower_position];
	}
}

impl TryFrom<NbtListTag> for Beehive {
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

		let bees = value
			.get_child("bees")
			.unwrap()
			.as_list()
			.into_iter()
			.map(|x| Bee {
				entity_data: x.get_child("entity_data").unwrap().as_tag_compound(),
				min_ticks_in_hive: x.get_child("min_ticks_in_hive").unwrap().as_int(),
				ticks_in_hive: x.get_child("ticks_in_hive").unwrap().as_int(),
			})
			.collect();

		let raw_flower_pos = value.get_child("flower_pos").unwrap_or(&NbtTag::IntArray(String::new(), vec![])).as_int_array();
		let flower_position: Option<BlockPosition> = if raw_flower_pos.len() == 3 {
			Some(BlockPosition {
				x: raw_flower_pos[0],
				y: raw_flower_pos[1] as i16,
				z: raw_flower_pos[2],
			})
		} else {
			None
		};

		return Ok(Beehive {
			position,
			components: Vec::new(),
			bees,
			flower_position,
		});
	}
}
