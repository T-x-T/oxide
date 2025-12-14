use super::*;

#[derive(Debug, Clone)]
pub struct Jukebox {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub record_item: Item,
	pub ticks_since_song_started: Option<i64>,
}

impl CommonBlockEntity for Jukebox {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			record_item: Item::default(),
			ticks_since_song_started: None,
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Item] {
		return &mut [];
	}

	fn get_contained_items_owned(&self) -> Vec<Item> {
		return vec![self.record_item.clone()];
	}
}

impl From<Jukebox> for Vec<NbtTag> {
	fn from(value: Jukebox) -> Self {
		let mut output = vec![NbtTag::TagCompound(
			"RecordItem".to_string(),
			vec![
				NbtTag::String("id".to_string(), value.record_item.id.clone()),
				NbtTag::Int("count".to_string(), value.record_item.count as i32),
				NbtTag::TagCompound("components".to_string(), Vec::new()), //missing SlotComponent to nbt conversion]
			],
		)];

		if let Some(ticks_since_song_started) = value.ticks_since_song_started {
			output.push(NbtTag::Long("ticks_since_song_started".to_string(), ticks_since_song_started));
		}

		return output;
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

		let record_item_raw = value.get_child("RecordItem").unwrap_or(&NbtTag::TagCompound(String::new(), Vec::new())).clone();
		let record_item = Item {
			id: record_item_raw.get_child("id").unwrap_or(&NbtTag::String(String::new(), "minecraft:air".to_string())).as_string().to_string(),
			count: record_item_raw.get_child("count").unwrap_or(&NbtTag::Int(String::new(), 0)).as_int() as u8,
			components: Vec::new(),
		};

		let ticks_since_song_started = value.get_child("ticks_since_song_started").map(|x| x.as_long());

		return Ok(Jukebox {
			position,
			components: Vec::new(),
			record_item,
			ticks_since_song_started,
		});
	}
}
