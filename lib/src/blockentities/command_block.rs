use super::*;

#[derive(Debug, Clone)]
pub struct CommandBlock {
	pub position: BlockPosition,        //global position, NOT within the chunk
	pub components: Vec<SlotComponent>, //At least I think so?
	pub auto: bool,
	pub command: String,
	pub condition_met: bool, //true if not a conditional command block
	pub custom_name: Option<String>,
	pub last_execution: i64,
	pub last_output: String,
	pub powered: bool,
	pub success_count: i32,
	pub track_output: bool,
	pub update_last_execution: bool,
}

impl CommonBlockEntity for CommandBlock {
	fn tick(&mut self, _players: &[Player], _game: Arc<Game>) {
		return;
	}

	fn new(position: BlockPosition) -> Self {
		return Self {
			position,
			components: Vec::new(),
			auto: false,
			command: String::new(),
			condition_met: true,
			custom_name: None,
			last_execution: 0,
			last_output: String::new(),
			powered: false,
			success_count: 0,
			track_output: true,
			update_last_execution: true,
		};
	}

	fn get_contained_items_mut(&mut self) -> &mut [Item] {
		return &mut [];
	}

	fn get_contained_items_owned(&self) -> Vec<Item> {
		return Vec::new();
	}
}

impl From<CommandBlock> for Vec<NbtTag> {
	fn from(value: CommandBlock) -> Self {
		let mut output = vec![
			NbtTag::Byte("auto".to_string(), if value.auto { 1 } else { 0 }),
			NbtTag::String("Command".to_string(), value.command),
			NbtTag::Byte("conditionMet".to_string(), if value.condition_met { 1 } else { 0 }),
			NbtTag::Long("LastExecution".to_string(), value.last_execution),
			NbtTag::String("LastOutput".to_string(), value.last_output),
			NbtTag::Byte("powered".to_string(), if value.powered { 1 } else { 0 }),
			NbtTag::Int("SuccessCount".to_string(), value.success_count),
			NbtTag::Byte("TrackOutput".to_string(), if value.track_output { 1 } else { 0 }),
			NbtTag::Byte("UpdateLastExecution".to_string(), if value.update_last_execution { 1 } else { 0 }),
		];

		if let Some(custom_name) = value.custom_name {
			output.push(NbtTag::String("CustomName".to_string(), custom_name));
		}

		return output;
	}
}

impl TryFrom<NbtListTag> for CommandBlock {
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

		let auto = value.get_child("auto").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1;
		let command = value.get_child("Command").unwrap_or(&NbtTag::String(String::new(), String::new())).as_string().to_string();
		let condition_met = value.get_child("conditionMet").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1;
		let last_execution = value.get_child("LastExecution").unwrap_or(&NbtTag::Long(String::new(), 0)).as_long();
		let last_output = value.get_child("LastOutput").unwrap_or(&NbtTag::String(String::new(), String::new())).as_string().to_string();
		let powered = value.get_child("powered").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1;
		let success_count = value.get_child("SuccessCount").unwrap_or(&NbtTag::Int(String::new(), 0)).as_int();
		let track_output = value.get_child("TrackOutput").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1;
		let update_last_execution = value.get_child("UpdateLastExecution").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1;
		let custom_name = value.get_child("CustomName").map(|x| x.as_string().to_string());

		return Ok(CommandBlock {
			position,
			components: Vec::new(),
			auto,
			command,
			condition_met,
			custom_name,
			last_execution,
			last_output,
			powered,
			success_count,
			track_output,
			update_last_execution,
		});
	}
}
