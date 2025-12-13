use super::*;

#[derive(Debug, Clone)]
pub struct Banner {
	pub position: BlockPosition,         //global position, NOT within the chunk
	pub components: Vec<SlotComponent>,  //At least I think so?
	pub patterns: Vec<(String, String)>, //pattern, color
}

impl CommonBlockEntity for Banner {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			patterns: Vec::new(),
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Item] {
		return &mut [];
	}

	fn get_contained_items_owned(&self) -> Vec<Item> {
		return Vec::new();
	}
}

impl From<Banner> for Vec<NbtTag> {
	fn from(value: Banner) -> Self {
		let patterns = NbtTag::List(
			"patterns".to_string(),
			value
				.patterns
				.iter()
				.map(|x| {
					NbtListTag::TagCompound(vec![
						NbtTag::String("color".to_string(), x.1.clone()),
						NbtTag::String("pattern".to_string(), x.0.clone()),
					])
				})
				.collect(),
		);
		return vec![patterns];
	}
}

impl TryFrom<NbtListTag> for Banner {
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

		let mut patterns: Vec<(String, String)> = Vec::new();
		if let Some(raw_patterns) = value.get_child("patterns") {
			for entry in raw_patterns.as_list() {
				patterns
					.push((entry.get_child("color").unwrap().as_string().to_string(), entry.get_child("pattern").unwrap().as_string().to_string()));
			}
		}

		return Ok(Banner {
			position,
			components: Vec::new(),
			patterns,
		});
	}
}
