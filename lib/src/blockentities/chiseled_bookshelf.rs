use super::*;

#[derive(Debug, Clone)]
pub struct ChiseledBookshelf {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub items: Vec<Item>,               //len 6
	pub last_interacted_slot: i32,      //0-5 or -1 if none
}

impl CommonBlockEntity for ChiseledBookshelf {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			items: vec![Item::default(); 6],
			last_interacted_slot: -1,
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Item] {
		return &mut self.items;
	}

	fn get_contained_items_owned(&self) -> Vec<Item> {
		return self.items.clone();
	}
}

impl From<ChiseledBookshelf> for Vec<NbtTag> {
	fn from(value: ChiseledBookshelf) -> Self {
		return vec![value.items.into(), NbtTag::Int("last_interacted_slot".to_string(), value.last_interacted_slot)];
	}
}

impl TryFrom<NbtListTag> for ChiseledBookshelf {
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

		let mut items = vec![Item::default(); 27];
		if let Some(raw_items) = value.get_child("Items") {
			for entry in raw_items.as_list() {
				items[entry.get_child("Slot").unwrap().as_byte() as usize] = Item {
					id: entry.get_child("id").unwrap().as_string().to_string(),
					count: entry.get_child("count").unwrap().as_int() as u8,
					components: Vec::new(),
				};
			}
		}

		let last_interacted_slot = value.get_child("last_interacted_slot").unwrap_or(&NbtTag::Int(String::new(), -1)).as_int();

		return Ok(ChiseledBookshelf {
			position,
			components: Vec::new(),
			items,
			last_interacted_slot,
		});
	}
}
