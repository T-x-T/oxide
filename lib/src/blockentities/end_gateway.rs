use super::*;

#[derive(Debug, Clone)]
pub struct EndGateway {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub age: i64,
	pub exact_teleport: bool,
	pub exit_portal: BlockPosition,
}

impl CommonBlockEntity for EndGateway {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			age: 0,
			exact_teleport: false,
			exit_portal: BlockPosition {
				x: 0,
				y: 0,
				z: 0,
			},
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Item] {
		return &mut [];
	}

	fn get_contained_items_owned(&self) -> Vec<Item> {
		return Vec::new();
	}
}

impl From<EndGateway> for Vec<NbtTag> {
	fn from(value: EndGateway) -> Self {
		return vec![
			NbtTag::Long("Age".to_string(), value.age),
			NbtTag::Byte("ExactTeleport".to_string(), if value.exact_teleport { 1 } else { 0 }),
			NbtTag::IntArray("exit_portal".to_string(), vec![value.exit_portal.x, value.exit_portal.y as i32, value.exit_portal.z]),
		];
	}
}

impl TryFrom<NbtListTag> for EndGateway {
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

		let age = value.get_child("Age").unwrap_or(&NbtTag::Long(String::new(), 0)).as_long();
		let exact_teleport = value.get_child("ExactTeleport").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1;
		let exit_portal_raw = value.get_child("exit_portal").unwrap_or(&NbtTag::IntArray(String::new(), vec![0, 0, 0])).as_int_array();
		let exit_portal = BlockPosition {
			x: exit_portal_raw[0],
			y: exit_portal_raw[1] as i16,
			z: exit_portal_raw[2],
		};

		return Ok(EndGateway {
			position,
			components: Vec::new(),
			age,
			exact_teleport,
			exit_portal,
		});
	}
}
