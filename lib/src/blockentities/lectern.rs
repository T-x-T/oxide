use super::*;

#[derive(Debug, Clone)]
pub struct Lectern {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub book: Option<Slot>,
	pub page: Option<i32>,
}

impl CommonBlockEntity for Lectern {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			book: None,
			page: None,
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Slot] {
		return &mut [];
	}

	fn get_contained_items_owned(&self) -> Vec<Slot> {
		return Vec::new();
	}
}

impl From<Lectern> for Vec<NbtTag> {
	fn from(value: Lectern) -> Self {
		let mut output: Vec<NbtTag> = Vec::new();

		if let Some(book) = value.book {
			output.push(NbtTag::TagCompound(
				"Book".to_string(),
				vec![
					NbtTag::String("id".to_string(), data::items::get_item_name_by_id(book.id).to_string()),
					NbtTag::Int("count".to_string(), book.count),
					NbtTag::TagCompound("components".to_string(), Vec::new()),
				],
			));
			output.push(NbtTag::Int("Page".to_string(), value.page.unwrap()));
		};

		return output;
	}
}

impl TryFrom<NbtListTag> for Lectern {
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

		let book = value.get_child("Book").map(|x| Slot {
			id: data::items::get_item_id_by_name(x.get_child("id").unwrap().as_string()),
			count: x.get_child("count").unwrap().as_int(),
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		});

		let page = value.get_child("Page").map(|x| x.as_int());

		return Ok(Lectern {
			position,
			components: Vec::new(),
			book,
			page,
		});
	}
}
