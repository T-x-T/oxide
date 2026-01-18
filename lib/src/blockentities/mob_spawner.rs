use super::*;

#[derive(Debug, Clone)]
pub struct MobSpawner {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub delay: i16,
	pub max_nearby_entities: i16,
	pub max_spawn_delay: i16,
	pub min_spawn_delay: i16,
	pub required_player_range: i16,
	pub spawn_count: i16,
	pub spawn_range: i16,
	pub spawn_data: NbtTag,
	pub spawn_potentials: Vec<NbtListTag>,
}

impl CommonBlockEntity for MobSpawner {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			delay: 0,
			max_nearby_entities: 8,
			max_spawn_delay: 20,
			min_spawn_delay: 0,
			required_player_range: 10,
			spawn_count: 4,
			spawn_range: 4,
			spawn_data: NbtTag::TagCompound("SpawnData".to_string(), Vec::new()),
			spawn_potentials: Vec::new(),
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Item] {
		return &mut [];
	}

	fn get_contained_items_owned(&self) -> Vec<Item> {
		return Vec::new();
	}
}

impl From<MobSpawner> for Vec<NbtTag> {
	fn from(value: MobSpawner) -> Self {
		let mut output = vec![
			NbtTag::Short("Delay".to_string(), value.delay),
			NbtTag::Short("MaxNearbyEntities".to_string(), value.max_nearby_entities),
			NbtTag::Short("MaxSpawnDelay".to_string(), value.max_spawn_delay),
			NbtTag::Short("MinSpawnDelay".to_string(), value.min_spawn_delay),
			NbtTag::Short("RequiredPlayerRange".to_string(), value.required_player_range),
			NbtTag::Short("SpawnCount".to_string(), value.spawn_count),
			NbtTag::Short("SpawnRange".to_string(), value.spawn_range),
			value.spawn_data,
		];

		if !value.spawn_potentials.is_empty() {
			output.push(NbtTag::List("SpawnPotentials".to_string(), value.spawn_potentials));
		}

		return output;
	}
}

impl TryFrom<NbtListTag> for MobSpawner {
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

		let delay = value.get_child("Delay").unwrap_or(&NbtTag::Short(String::new(), 0)).as_short();
		let max_nearby_entities = value.get_child("MaxNearbyEntities").unwrap_or(&NbtTag::Short(String::new(), 8)).as_short();
		let max_spawn_delay = value.get_child("MaxSpawnDelay").unwrap_or(&NbtTag::Short(String::new(), 20)).as_short();
		let min_spawn_delay = value.get_child("MinSpawnDelay").unwrap_or(&NbtTag::Short(String::new(), 0)).as_short();
		let required_player_range = value.get_child("RequiredPlayerRange").unwrap_or(&NbtTag::Short(String::new(), 10)).as_short();
		let spawn_count = value.get_child("SpawnCount").unwrap_or(&NbtTag::Short(String::new(), 4)).as_short();
		let spawn_range = value.get_child("SpawnRange").unwrap_or(&NbtTag::Short(String::new(), 4)).as_short();

		let spawn_data = value.get_child("SpawnData").unwrap_or(&NbtTag::TagCompound("SpawnData".to_string(), Vec::new())).clone();
		let spawn_potentials = value.get_child("SpawnPotentials").unwrap_or(&NbtTag::List("SpawnPotentials".to_string(), Vec::new())).as_list();

		return Ok(MobSpawner {
			position,
			components: Vec::new(),
			delay,
			max_nearby_entities,
			max_spawn_delay,
			min_spawn_delay,
			required_player_range,
			spawn_count,
			spawn_range,
			spawn_data,
			spawn_potentials,
		});
	}
}
