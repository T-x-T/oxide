use std::error::Error;
use std::sync::Arc;

use data::blocks::Type;

use crate::packets::Packet;

use super::*;

#[derive(Debug, Clone)]
pub enum BlockEntity {
	Furnace(crate::blockentities::furnace::Furnace),
	Chest(crate::blockentities::chest::Chest),
	Sign(crate::blockentities::sign::Sign),
}

pub trait CommonBlockEntity {
	fn tick(&mut self, players: &[Player], game: Arc<Game>);
	fn new(position: BlockPosition) -> Self;
	fn get_contained_items_mut(&mut self) -> &mut [Item];
	fn get_contained_items_owned(&self) -> Vec<Item>;
}

impl BlockEntity {
	pub fn tick(&mut self, players: &[Player], game: Arc<Game>) {
		match self {
			BlockEntity::Furnace(furnace) => furnace.tick(players, game),
			BlockEntity::Chest(chest) => chest.tick(players, game),
			BlockEntity::Sign(sign) => sign.tick(players, game),
		}
	}

	pub fn new_from_block(block_type: data::blocks::Type, position: BlockPosition) -> Option<Self> {
		return match block_type {
			Type::Furnace => Some(BlockEntity::Furnace(crate::blockentities::furnace::Furnace::new(position))),
			Type::Chest => Some(BlockEntity::Chest(crate::blockentities::chest::Chest::new(position))),
			Type::WallSign | Type::StandingSign | Type::WallHangingSign | Type::CeilingHangingSign => {
				Some(BlockEntity::Sign(crate::blockentities::sign::Sign::new(position)))
			}
			_ => None,
		};
	}

	pub fn get_position(&self) -> BlockPosition {
		return match self {
			BlockEntity::Furnace(furnace) => furnace.position,
			BlockEntity::Chest(chest) => chest.position,
			BlockEntity::Sign(sign) => sign.position,
		};
	}

	pub fn get_id(&self) -> String {
		match self {
			BlockEntity::Furnace(_) => "minecraft:furnace".to_string(),
			BlockEntity::Chest(_) => "minecraft:chest".to_string(),
			BlockEntity::Sign(_) => "minecraft:sign".to_string(),
		}
	}

	pub fn get_contained_items_owned(&self) -> Vec<Item> {
		return match self {
			BlockEntity::Furnace(furnace) => furnace.get_contained_items_owned(),
			BlockEntity::Chest(chest) => chest.get_contained_items_owned(),
			_ => Vec::new(),
		};
	}

	pub fn remove_self(&self, entity_id_manager: &EntityIdManager, players: &mut [Player], world: &mut World, game: Arc<Game>) {
		let items = self.get_contained_items_owned();

		let mut entities: Vec<Box<dyn SaveableEntity + Send>> = Vec::new();
		for item in items {
			let new_entity = item.get_entity(self.get_position().into(), entity_id_manager.get_new());
			let spawn_packet = new_entity.to_spawn_entity_packet();

			let metadata_packet = crate::packets::clientbound::play::SetEntityMetadata {
				entity_id: new_entity.get_common_entity_data().entity_id,
				metadata: new_entity.get_metadata(),
			};

			entities.push(Box::new(new_entity));

			players.iter().for_each(|x| {
				game.send_packet(
					&x.peer_socket_address,
					crate::packets::clientbound::play::SpawnEntity::PACKET_ID,
					spawn_packet.clone().try_into().unwrap(),
				);
				game.send_packet(
					&x.peer_socket_address,
					crate::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
					metadata_packet.clone().try_into().unwrap(),
				);
			});
		}

		world.dimensions.get_mut("minecraft:overworld").unwrap().add_entities(entities);

		world
			.dimensions
			.get_mut("minecraft:overworld")
			.unwrap()
			.get_chunk_from_position_mut(self.get_position())
			.unwrap()
			.block_entities
			.retain(|be| be.get_position() != self.get_position());
		players
			.iter_mut()
			.filter(|player| player.opened_inventory_at.is_some_and(|pos| pos == self.get_position()))
			.for_each(|x| x.close_inventory(game.clone()).unwrap());
	}

	pub fn set_needs_ticking(&mut self, new_needs_ticking: bool) {
		match self {
			BlockEntity::Furnace(furnace) => furnace.needs_ticking = new_needs_ticking,
			_ => (),
		}
	}

	pub fn get_needs_ticking(&self) -> bool {
		return match self {
			BlockEntity::Furnace(furnace) => furnace.needs_ticking,
			_ => false,
		};
	}
}

impl TryFrom<NbtListTag> for BlockEntity {
	type Error = Box<dyn Error>;

	fn try_from(value: NbtListTag) -> Result<Self, Self::Error> {
		let id: BlockEntityId = value.get_child("id").unwrap().as_string().try_into()?;

		return Ok(match id {
			BlockEntityId::Furnace => BlockEntity::Furnace(crate::blockentities::furnace::Furnace::try_from(value)?),
			BlockEntityId::Chest => BlockEntity::Chest(crate::blockentities::chest::Chest::try_from(value)?),
			BlockEntityId::Sign => BlockEntity::Sign(crate::blockentities::sign::Sign::try_from(value)?),
			_ => {
				return Err(Box::new(crate::CustomError::TriedParsingUnknown(format!("tried parsing unknown blockentity {id:?}"))));
			}
		});
	}
}

impl From<BlockEntity> for Vec<NbtTag> {
	fn from(value: BlockEntity) -> Self {
		return match value {
			BlockEntity::Furnace(furnace) => furnace.into(),
			BlockEntity::Chest(chest) => chest.into(),
			BlockEntity::Sign(sign) => sign.into(),
		};
	}
}

impl From<BlockEntity> for NbtListTag {
	fn from(value: BlockEntity) -> Self {
		let mut items: Vec<NbtTag> = vec![
			NbtTag::String("id".to_string(), Into::<&str>::into(value.get_id().as_str()).to_string()),
			NbtTag::Int("x".to_string(), value.get_position().x),
			NbtTag::Int("y".to_string(), value.get_position().y as i32),
			NbtTag::Int("z".to_string(), value.get_position().z),
		];

		items.append(&mut value.into());

		return NbtListTag::TagCompound(items);
	}
}

#[derive(Debug, Clone)]
pub struct BlockEntityBaseData {
	pub id: BlockEntityId,
	pub position: BlockPosition,                //global position, NOT within the chunk
	pub components: Option<Vec<SlotComponent>>, //At least I think so?
	pub data: BlockEntityData,
	pub needs_ticking: bool,
}

impl BlockEntityBaseData {
	pub fn get_contained_items(&self) -> Vec<Item> {
		return match &self.data {
			BlockEntityData::Chest(items) => items.clone(),
			BlockEntityData::Furnace(items, _, _, _, _) => items.clone(),
			BlockEntityData::BrewingStand(items) => items.clone(),
			BlockEntityData::Crafter(items) => items.clone(),
			BlockEntityData::Dispenser(items) => items.clone(),
			BlockEntityData::Hopper(items) => items.clone(),
			_ => Vec::new(),
		};
	}
}

#[derive(Debug, Clone, Copy)]
pub enum BlockEntityId {
	Banner,
	Barrel,
	Beacon,
	Bed,
	Beehive,
	Bell,
	BlastFurnace,
	BrewingStand,
	BrushableBlock,
	CalibratedSculkSensor,
	Campfire,
	Chest,
	ChiseledBookshelf,
	Comperator,
	CommandBlock,
	Conduit,
	Crafter,
	CreakingHeart,
	DaylightDetector,
	DecoratedPot,
	Dispenser,
	Dropper,
	EnchantingTable,
	EnderChest,
	EndGateway,
	EndPortal,
	Furnace,
	HangingSign,
	Hopper,
	Jigsaw,
	Jukebox,
	Lectern,
	MobSpawner,
	Piston,
	ShulkerBox,
	Sign,
	Skull,
	SculkCatalyst,
	SculkSensor,
	SculkShrieker,
	Smoker,
	SoulCampfire,
	StructureBlock,
	TrappedChest,
	TrialSpawner,
	Vault,
}

impl From<BlockEntityId> for &str {
	fn from(value: BlockEntityId) -> Self {
		match value {
			BlockEntityId::Banner => "minecraft:banner",
			BlockEntityId::Barrel => "minecraft:barrel",
			BlockEntityId::Beacon => "minecraft:beacon",
			BlockEntityId::Bed => "minecraft:bed",
			BlockEntityId::Beehive => "minecraft:beehive",
			BlockEntityId::Bell => "minecraft:bell",
			BlockEntityId::BlastFurnace => "minecraft:blast_furnace",
			BlockEntityId::BrewingStand => "minecraft:brewing_stand",
			BlockEntityId::BrushableBlock => "minecraft:brushable_block",
			BlockEntityId::CalibratedSculkSensor => "minecraft:calibrated_sculk_sensor",
			BlockEntityId::Campfire => "minecraft:campfire",
			BlockEntityId::Chest => "minecraft:chest",
			BlockEntityId::ChiseledBookshelf => "minecraft:chiseled_bookshelf",
			BlockEntityId::Comperator => "minecraft:comperator",
			BlockEntityId::CommandBlock => "minecraft:command_block",
			BlockEntityId::Conduit => "minecraft:conduit",
			BlockEntityId::Crafter => "minecraft:crafter",
			BlockEntityId::CreakingHeart => "minecraft:creaking_heart",
			BlockEntityId::DaylightDetector => "minecraft:daylight_detector",
			BlockEntityId::DecoratedPot => "minecraft:decorated_pot",
			BlockEntityId::Dispenser => "minecraft:dispenser",
			BlockEntityId::Dropper => "minecraft:dropper",
			BlockEntityId::EnchantingTable => "minecraft:enchanting_table",
			BlockEntityId::EnderChest => "minecraft:ender_chest",
			BlockEntityId::EndGateway => "minecraft:end_gateway",
			BlockEntityId::EndPortal => "minecraft:end_portal",
			BlockEntityId::Furnace => "minecraft:furnace",
			BlockEntityId::HangingSign => "minecraft:hanging_sign",
			BlockEntityId::Hopper => "minecraft:hopper",
			BlockEntityId::Jigsaw => "minecraft:jigsaw",
			BlockEntityId::Jukebox => "minecraft:jukebox",
			BlockEntityId::Lectern => "minecraft:lectern",
			BlockEntityId::MobSpawner => "minecraft:mob_spawner",
			BlockEntityId::Piston => "minecraft:piston",
			BlockEntityId::ShulkerBox => "minecraft:shulker_box",
			BlockEntityId::Sign => "minecraft:sign",
			BlockEntityId::Skull => "minecraft:skull",
			BlockEntityId::SculkCatalyst => "minecraft:sculk_catalyst",
			BlockEntityId::SculkSensor => "minecraft:sculk_sensor",
			BlockEntityId::SculkShrieker => "minecraft:sculk_shrieker",
			BlockEntityId::Smoker => "minecraft:smoker",
			BlockEntityId::SoulCampfire => "minecraft:soul_campfire",
			BlockEntityId::StructureBlock => "minecraft:structure_block",
			BlockEntityId::TrappedChest => "minecraft:trapped_chest",
			BlockEntityId::TrialSpawner => "minecraft:trial_spawner",
			BlockEntityId::Vault => "minecraft:vault",
		}
	}
}

impl TryFrom<&str> for BlockEntityId {
	type Error = Box<dyn Error>;

	fn try_from(value: &str) -> Result<Self, Box<dyn Error>> {
		match value {
			"minecraft:banner" => Ok(BlockEntityId::Banner),
			"minecraft:barrel" => Ok(BlockEntityId::Barrel),
			"minecraft:beacon" => Ok(BlockEntityId::Beacon),
			"minecraft:bed" => Ok(BlockEntityId::Bed),
			"minecraft:beehive" => Ok(BlockEntityId::Beehive),
			"minecraft:bell" => Ok(BlockEntityId::Bell),
			"minecraft:blast_furnace" => Ok(BlockEntityId::BlastFurnace),
			"minecraft:brewing_stand" => Ok(BlockEntityId::BrewingStand),
			"minecraft:brushable_block" => Ok(BlockEntityId::BrushableBlock),
			"minecraft:calibrated_sculk_sensor" => Ok(BlockEntityId::CalibratedSculkSensor),
			"minecraft:campfire" => Ok(BlockEntityId::Campfire),
			"minecraft:chest" => Ok(BlockEntityId::Chest),
			"minecraft:chiseled_bookshelf" => Ok(BlockEntityId::ChiseledBookshelf),
			"minecraft:comperator" => Ok(BlockEntityId::Comperator),
			"minecraft:command_block" => Ok(BlockEntityId::CommandBlock),
			"minecraft:conduit" => Ok(BlockEntityId::Conduit),
			"minecraft:crafter" => Ok(BlockEntityId::Crafter),
			"minecraft:creaking_heart" => Ok(BlockEntityId::CreakingHeart),
			"minecraft:daylight_detector" => Ok(BlockEntityId::DaylightDetector),
			"minecraft:decorated_pot" => Ok(BlockEntityId::DecoratedPot),
			"minecraft:dispenser" => Ok(BlockEntityId::Dispenser),
			"minecraft:dropper" => Ok(BlockEntityId::Dropper),
			"minecraft:enchanting_table" => Ok(BlockEntityId::EnchantingTable),
			"minecraft:ender_chest" => Ok(BlockEntityId::EnderChest),
			"minecraft:end_gateway" => Ok(BlockEntityId::EndGateway),
			"minecraft:end_portal" => Ok(BlockEntityId::EndPortal),
			"minecraft:furnace" => Ok(BlockEntityId::Furnace),
			"minecraft:hanging_sign" => Ok(BlockEntityId::HangingSign),
			"minecraft:hopper" => Ok(BlockEntityId::Hopper),
			"minecraft:jigsaw" => Ok(BlockEntityId::Jigsaw),
			"minecraft:jukebox" => Ok(BlockEntityId::Jukebox),
			"minecraft:lectern" => Ok(BlockEntityId::Lectern),
			"minecraft:mob_spawner" => Ok(BlockEntityId::MobSpawner),
			"minecraft:piston" => Ok(BlockEntityId::Piston),
			"minecraft:shulker_box" => Ok(BlockEntityId::ShulkerBox),
			"minecraft:sign" => Ok(BlockEntityId::Sign),
			"minecraft:skull" => Ok(BlockEntityId::Skull),
			"minecraft:sculk_catalyst" => Ok(BlockEntityId::SculkCatalyst),
			"minecraft:sculk_sensor" => Ok(BlockEntityId::SculkSensor),
			"minecraft:sculk_shrieker" => Ok(BlockEntityId::SculkShrieker),
			"minecraft:smoker" => Ok(BlockEntityId::Smoker),
			"minecraft:soul_campfire" => Ok(BlockEntityId::SoulCampfire),
			"minecraft:structure_block" => Ok(BlockEntityId::StructureBlock),
			"minecraft:trapped_chest" => Ok(BlockEntityId::TrappedChest),
			"minecraft:trial_spawner" => Ok(BlockEntityId::TrialSpawner),
			"minecraft:vault" => Ok(BlockEntityId::Vault),
			_ => Err(Box::new(crate::CustomError::InvalidInput(value.to_string()))),
		}
	}
}

#[derive(Debug, Clone, Default)]
pub enum BlockEntityData {
	Banner(Vec<(String, String)>), //patterns: <pattern, color>
	Chest(Vec<Item>),
	Furnace(Vec<Item>, i16, i16, i16, i16), //0: item being smelted 1: fuel 2: result; lit_time_remaining, cooking_time_spent, cooking_total_time, lit_total_time
	BrewingStand(Vec<Item>),                //0: left, 1: middle, 2: right, 3: ingredient, 4: fuel
	Crafter(Vec<Item>),                     //len 9
	Dispenser(Vec<Item>),                   //len 9
	Hopper(Vec<Item>),                      //len 5
	Beacon(Option<String>, Option<String>), //Primary and Secondary effect as potion ID
	Beehive(Vec<Bee>, Vec<i32>),            //Int vec is 3 long and stores flower_pos
	Brushable(Option<Item>),
	Campfire(Vec<i32>, Vec<i32>, Vec<Item>), //CookingTimes, CookingTotalTimes, Items
	ChiseledBookShelf(Vec<Item>, i32),       //i32 is the index of the last selected slot or -1; valid slots go from 0-5
	Comparator(i32),                         //OutputSignal
	Conduit(Option<Vec<i32>>),               //may have 4 four ints representing UUID of mob that gets currently attacked
	CreakingHeart(Vec<i32>),                 //has 4 four ints for the UUID of the associated creaking
	DecoratedPot(Vec<String>, Item),         //Vector of items IDs for each side or something
	EndGateway(i64, u8, Vec<i32>),           //Age, ExactTeleport, exit_portal (coords in 3 ints)
	Sign(u8, NbtTag, NbtTag),                //is_waxed, front_text, back_text
	JukeBox(Item, i64),                      //RecordItem, ticks_since_song_started
	Lectern(Option<Item>, Option<i32>),      //Book, Page
	#[default]
	NoData,
}

#[derive(Debug, Clone)]
pub struct Bee {
	entity_data: Vec<NbtTag>,
	min_ticks_in_hive: i32,
	ticks_in_hive: i32,
}

impl From<BlockEntityBaseData> for NbtListTag {
	fn from(value: BlockEntityBaseData) -> Self {
		let mut items: Vec<NbtTag> = vec![
			NbtTag::String("id".to_string(), Into::<&str>::into(value.id).to_string()),
			NbtTag::Int("x".to_string(), value.position.x),
			NbtTag::Int("y".to_string(), value.position.y as i32),
			NbtTag::Int("z".to_string(), value.position.z),
		];

		items.append(&mut value.data.into());

		return NbtListTag::TagCompound(items);
	}
}

impl From<BlockEntityData> for Vec<NbtTag> {
	fn from(value: BlockEntityData) -> Self {
		return match value {
			BlockEntityData::Banner(data) => vec![NbtTag::List(
				"patterns".to_string(),
				data
					.iter()
					.map(|x| {
						NbtListTag::TagCompound(vec![
							NbtTag::String("color".to_string(), x.1.clone()),
							NbtTag::String("pattern".to_string(), x.0.clone()),
						])
					})
					.collect(),
			)],
			BlockEntityData::Chest(block_entity_data_items) => {
				vec![block_entity_data_items.into()]
			}
			BlockEntityData::Furnace(block_entity_data_items, lit_time_remaining, cooking_time_spent, cooking_total_time, lit_total_time) => {
				vec![
					block_entity_data_items.into(),
					NbtTag::Short("lit_time_remaining".to_string(), lit_time_remaining),
					NbtTag::Short("cooking_time_spent".to_string(), cooking_time_spent),
					NbtTag::Short("cooking_total_time".to_string(), cooking_total_time),
					NbtTag::Short("lit_total_time".to_string(), lit_total_time),
				]
			}
			BlockEntityData::BrewingStand(block_entity_data_items) => {
				vec![block_entity_data_items.into()]
			}
			BlockEntityData::Crafter(block_entity_data_items) => {
				vec![block_entity_data_items.into()]
			}
			BlockEntityData::Dispenser(block_entity_data_items) => {
				vec![block_entity_data_items.into()]
			}
			BlockEntityData::Hopper(block_entity_data_items) => {
				vec![block_entity_data_items.into()]
			}
			BlockEntityData::Beacon(primary_effect, secondary_effect) => {
				let mut output: Vec<NbtTag> = Vec::new();

				if let Some(primary_effect) = primary_effect {
					output.push(NbtTag::String("primary_effect".to_string(), primary_effect));
				}
				if let Some(secondary_effect) = secondary_effect {
					output.push(NbtTag::String("secondary_effect".to_string(), secondary_effect));
				}

				output
			}
			BlockEntityData::Beehive(bees, flower_pos) => {
				vec![
					NbtTag::List(
						"bees".to_string(),
						bees
							.into_iter()
							.map(|bee| {
								NbtListTag::TagCompound(vec![
									NbtTag::TagCompound("entity_data".to_string(), bee.entity_data),
									NbtTag::Int("min_ticks_in_hive".to_string(), bee.min_ticks_in_hive),
									NbtTag::Int("ticks_in_hive".to_string(), bee.ticks_in_hive),
								])
							})
							.collect(),
					),
					NbtTag::IntArray("flower_pos".to_string(), flower_pos),
				]
			}
			BlockEntityData::Brushable(item) => {
				let mut output: Vec<NbtTag> = Vec::new();

				if let Some(item) = item {
					output.push(NbtTag::TagCompound(
						"item".to_string(),
						vec![
							NbtTag::Byte("Slot".to_string(), 0),
							NbtTag::String("id".to_string(), item.id.clone()),
							NbtTag::Int("count".to_string(), item.count as i32),
							NbtTag::TagCompound("components".to_string(), Vec::new()), //missing SlotComponent to nbt conversion
						],
					));
				}

				return output;
			}
			BlockEntityData::Campfire(cooking_times, cooking_total_times, items) => {
				vec![
					items.into(),
					NbtTag::IntArray("CookingTimes".to_string(), cooking_times),
					NbtTag::IntArray("CookingTotalTimes".to_string(), cooking_total_times),
				]
			}
			BlockEntityData::ChiseledBookShelf(items, last_interacted_slot) => {
				vec![items.into(), NbtTag::Int("last_interacted_slot".to_string(), last_interacted_slot)]
			}
			BlockEntityData::Comparator(output_signal) => {
				vec![NbtTag::Int("OutputSignal".to_string(), output_signal)]
			}
			BlockEntityData::Conduit(mob) => {
				if let Some(mob) = mob {
					vec![NbtTag::IntArray("Target".to_string(), mob)]
				} else {
					Vec::new()
				}
			}
			BlockEntityData::CreakingHeart(creaking) => {
				vec![NbtTag::IntArray("Target".to_string(), creaking)]
			}
			BlockEntityData::DecoratedPot(sherds, item) => {
				vec![
					NbtTag::List("sherds".to_string(), sherds.into_iter().map(NbtListTag::String).collect()),
					NbtTag::TagCompound(
						"item".to_string(),
						vec![
							NbtTag::String("id".to_string(), item.id.clone()),
							NbtTag::Int("count".to_string(), item.count as i32),
							NbtTag::TagCompound("components".to_string(), Vec::new()),
						],
					),
				]
			}
			BlockEntityData::EndGateway(age, exact_teleport, exit_portal) => {
				vec![
					NbtTag::Long("Age".to_string(), age),
					NbtTag::Byte("ExactTeleport".to_string(), exact_teleport),
					NbtTag::IntArray("exit_portal".to_string(), exit_portal),
				]
			}
			BlockEntityData::Sign(is_waxed, front_text, back_text) => {
				vec![NbtTag::Byte("is_waxed".to_string(), is_waxed), front_text, back_text]
			}
			BlockEntityData::JukeBox(record_item, ticks_since_song_started) => {
				vec![
					NbtTag::TagCompound(
						"RecordItem".to_string(),
						vec![
							NbtTag::String("id".to_string(), record_item.id.clone()),
							NbtTag::Int("count".to_string(), record_item.count as i32),
							NbtTag::TagCompound("components".to_string(), Vec::new()),
						],
					),
					NbtTag::Long("ticks_since_song_started".to_string(), ticks_since_song_started),
				]
			}
			BlockEntityData::Lectern(book, page) => {
				let mut output: Vec<NbtTag> = Vec::new();

				if let Some(book) = book {
					output.push(NbtTag::TagCompound(
						"Book".to_string(),
						vec![
							NbtTag::String("id".to_string(), book.id.clone()),
							NbtTag::Int("count".to_string(), book.count as i32),
							NbtTag::TagCompound("components".to_string(), Vec::new()),
						],
					));
					output.push(NbtTag::Int("Page".to_string(), page.unwrap()));
				};

				output
			}
			BlockEntityData::NoData => Vec::new(),
		};
	}
}

pub fn get_blockentity_for_placed_block(position_global: BlockPosition, block_state_id: u16) -> Option<BlockEntityBaseData> {
	return match data::blocks::get_type_from_block_state_id(block_state_id) {
		Type::Chest => Some(BlockEntityBaseData {
			id: BlockEntityId::Chest,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Chest(vec![Item::default(); 27]),
		}),
		Type::TrappedChest => Some(BlockEntityBaseData {
			id: BlockEntityId::TrappedChest,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Chest(vec![Item::default(); 27]),
		}),
		Type::Barrel => Some(BlockEntityBaseData {
			id: BlockEntityId::Barrel,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Chest(vec![Item::default(); 27]),
		}),
		Type::Banner => Some(BlockEntityBaseData {
			id: BlockEntityId::Banner,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Banner(Vec::new()),
		}),
		Type::WallBanner => Some(BlockEntityBaseData {
			id: BlockEntityId::Banner,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Banner(Vec::new()),
		}),
		Type::Beacon => Some(BlockEntityBaseData {
			id: BlockEntityId::Beacon,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Beacon(None, None),
		}),
		Type::Bed => Some(BlockEntityBaseData {
			id: BlockEntityId::Bed,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::Beehive => Some(BlockEntityBaseData {
			id: BlockEntityId::Beehive,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Beehive(Vec::new(), Vec::new()),
		}),
		Type::Bell => Some(BlockEntityBaseData {
			id: BlockEntityId::Bell,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::BlastFurnace => Some(BlockEntityBaseData {
			id: BlockEntityId::BlastFurnace,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Furnace(vec![Item::default(); 3], 0, 0, 0, 0),
		}),
		Type::BrewingStand => Some(BlockEntityBaseData {
			id: BlockEntityId::BrewingStand,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::BrewingStand(vec![Item::default(); 5]),
		}),
		Type::Brushable => Some(BlockEntityBaseData {
			id: BlockEntityId::BrushableBlock,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Brushable(None),
		}),
		Type::CalibratedSculkSensor => Some(BlockEntityBaseData {
			id: BlockEntityId::CalibratedSculkSensor,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::Campfire => Some(BlockEntityBaseData {
			id: BlockEntityId::Campfire,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Campfire(Vec::new(), Vec::new(), Vec::new()),
		}), //could technically also be a soulcampfire, but probably doesn't matter
		Type::ChiseledBookShelf => Some(BlockEntityBaseData {
			id: BlockEntityId::ChiseledBookshelf,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::ChiseledBookShelf(Vec::new(), -1),
		}),
		Type::Comparator => Some(BlockEntityBaseData {
			id: BlockEntityId::Comperator,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Comparator(0),
		}),
		Type::Command => Some(BlockEntityBaseData {
			id: BlockEntityId::CommandBlock,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::Conduit => Some(BlockEntityBaseData {
			id: BlockEntityId::Conduit,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Conduit(None),
		}),
		Type::Crafter => Some(BlockEntityBaseData {
			id: BlockEntityId::Crafter,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Crafter(vec![Item::default(); 9]),
		}),
		Type::CreakingHeart => Some(BlockEntityBaseData {
			id: BlockEntityId::CreakingHeart,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::CreakingHeart(Vec::new()),
		}), //is supposed to spawn a creaking or something???
		Type::DaylightDetector => Some(BlockEntityBaseData {
			id: BlockEntityId::DaylightDetector,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::DecoratedPot => Some(BlockEntityBaseData {
			id: BlockEntityId::DecoratedPot,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::DecoratedPot(Vec::new(), Item::default()),
		}),
		Type::Dispenser => Some(BlockEntityBaseData {
			id: BlockEntityId::Dispenser,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Dispenser(vec![Item::default(); 9]),
		}),
		Type::Dropper => Some(BlockEntityBaseData {
			id: BlockEntityId::Dropper,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Dispenser(vec![Item::default(); 9]),
		}),
		Type::EnchantmentTable => Some(BlockEntityBaseData {
			id: BlockEntityId::EnchantingTable,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::EnderChest => Some(BlockEntityBaseData {
			id: BlockEntityId::EnderChest,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::EndGateway => Some(BlockEntityBaseData {
			id: BlockEntityId::EndGateway,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::EndGateway(0, 0, vec![0, 100, 0]),
		}),
		Type::EndPortal => Some(BlockEntityBaseData {
			id: BlockEntityId::EndPortal,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::Furnace => Some(BlockEntityBaseData {
			id: BlockEntityId::Furnace,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Furnace(vec![Item::default(); 3], 0, 0, 0, 0),
		}),
		Type::WallHangingSign => Some(BlockEntityBaseData {
			id: BlockEntityId::HangingSign,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Sign(
				0,
				NbtTag::TagCompound("front_text".to_string(), Vec::new()),
				NbtTag::TagCompound("back_text".to_string(), Vec::new()),
			),
		}),
		Type::CeilingHangingSign => Some(BlockEntityBaseData {
			id: BlockEntityId::HangingSign,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Sign(
				0,
				NbtTag::TagCompound("front_text".to_string(), Vec::new()),
				NbtTag::TagCompound("back_text".to_string(), Vec::new()),
			),
		}),
		Type::Hopper => Some(BlockEntityBaseData {
			id: BlockEntityId::Hopper,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Hopper(vec![Item::default(); 5]),
		}),
		Type::Jigsaw => Some(BlockEntityBaseData {
			id: BlockEntityId::Jigsaw,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::Jukebox => Some(BlockEntityBaseData {
			id: BlockEntityId::Jukebox,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::JukeBox(Item::default(), 0),
		}),
		Type::Lectern => Some(BlockEntityBaseData {
			id: BlockEntityId::Lectern,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Lectern(None, None),
		}),
		Type::Spawner => Some(BlockEntityBaseData {
			id: BlockEntityId::MobSpawner,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}), //Will implement once entities are implemented
		Type::MovingPiston => Some(BlockEntityBaseData {
			id: BlockEntityId::Piston,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}), //Implement when doing redstone
		Type::ShulkerBox => Some(BlockEntityBaseData {
			id: BlockEntityId::ShulkerBox,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Chest(vec![Item::default(); 27]),
		}),
		Type::WallSign => Some(BlockEntityBaseData {
			id: BlockEntityId::Sign,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Sign(
				0,
				NbtTag::TagCompound("front_text".to_string(), Vec::new()),
				NbtTag::TagCompound("back_text".to_string(), Vec::new()),
			),
		}),
		Type::StandingSign => Some(BlockEntityBaseData {
			id: BlockEntityId::Sign,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Sign(
				0,
				NbtTag::TagCompound("front_text".to_string(), Vec::new()),
				NbtTag::TagCompound("back_text".to_string(), Vec::new()),
			),
		}),
		Type::Skull => Some(BlockEntityBaseData {
			id: BlockEntityId::Skull,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::WallSkull => Some(BlockEntityBaseData {
			id: BlockEntityId::Skull,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::SculkCatalyst => Some(BlockEntityBaseData {
			id: BlockEntityId::SculkCatalyst,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::SculkSensor => Some(BlockEntityBaseData {
			id: BlockEntityId::SculkSensor,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::SculkShrieker => Some(BlockEntityBaseData {
			id: BlockEntityId::SculkShrieker,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::Smoker => Some(BlockEntityBaseData {
			id: BlockEntityId::Smoker,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::Furnace(vec![Item::default(); 3], 0, 0, 0, 0),
		}),
		Type::Structure => Some(BlockEntityBaseData {
			id: BlockEntityId::StructureBlock,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::TrialSpawner => Some(BlockEntityBaseData {
			id: BlockEntityId::TrialSpawner,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		Type::Vault => Some(BlockEntityBaseData {
			id: BlockEntityId::Vault,
			needs_ticking: false,
			position: position_global,
			components: None,
			data: BlockEntityData::NoData,
		}),
		_ => None,
	};
}

impl TryFrom<NbtListTag> for BlockEntityBaseData {
	type Error = Box<dyn Error>;

	fn try_from(value: NbtListTag) -> Result<Self, Self::Error> {
		let id: BlockEntityId = value.get_child("id").unwrap().as_string().try_into()?;
		let x = value.get_child("x").unwrap().as_int();
		let y = value.get_child("y").unwrap().as_int() as i16;
		let z = value.get_child("z").unwrap().as_int();
		let position = BlockPosition {
			x,
			y,
			z,
		};

		let data: BlockEntityData = match id {
			BlockEntityId::Banner => {
				let mut data: Vec<(String, String)> = Vec::new();

				if let Some(patterns) = value.get_child("patterns") {
					for entry in patterns.as_list() {
						data.push((
							entry.get_child("color").unwrap().as_string().to_string(),
							entry.get_child("pattern").unwrap().as_string().to_string(),
						));
					}
				}

				BlockEntityData::Banner(data)
			}
			BlockEntityId::Barrel | BlockEntityId::Chest | BlockEntityId::TrappedChest => {
				let mut data = vec![Item::default(); 27];

				if let Some(items) = value.get_child("Items") {
					for entry in items.as_list() {
						data[entry.get_child("Slot").unwrap().as_byte() as usize] = Item {
							id: entry.get_child("id").unwrap().as_string().to_string(),
							count: entry.get_child("count").unwrap().as_int() as u8,
							components: Vec::new(),
						};
					}
				}

				BlockEntityData::Chest(data)
			}
			BlockEntityId::Furnace | BlockEntityId::BlastFurnace | BlockEntityId::Smoker => {
				let mut data = vec![Item::default(); 3];

				if let Some(items) = value.get_child("Items") {
					for entry in items.as_list() {
						data[entry.get_child("Slot").unwrap().as_byte() as usize] = Item {
							id: entry.get_child("id").unwrap().as_string().to_string(),
							count: entry.get_child("count").unwrap().as_int() as u8,
							components: Vec::new(),
						};
					}
				}

				BlockEntityData::Furnace(
					data,
					value.get_child("lit_time_remaining").unwrap_or(&NbtTag::Short("".to_string(), 0)).as_short(),
					value.get_child("cooking_time_spent").unwrap_or(&NbtTag::Short("".to_string(), 0)).as_short(),
					value.get_child("cooking_total_time").unwrap_or(&NbtTag::Short("".to_string(), 0)).as_short(),
					value.get_child("lit_total_time").unwrap_or(&NbtTag::Short("".to_string(), 0)).as_short(),
				)
			}
			BlockEntityId::BrewingStand => {
				let mut data = vec![Item::default(); 5];

				if let Some(items) = value.get_child("Items") {
					for entry in items.as_list() {
						data[entry.get_child("Slot").unwrap().as_byte() as usize] = Item {
							id: entry.get_child("id").unwrap().as_string().to_string(),
							count: entry.get_child("count").unwrap().as_int() as u8,
							components: Vec::new(),
						};
					}
				}

				BlockEntityData::BrewingStand(data)
			}
			BlockEntityId::Crafter => {
				let mut data = vec![Item::default(); 9];

				if let Some(items) = value.get_child("Items") {
					for entry in items.as_list() {
						data[entry.get_child("Slot").unwrap().as_byte() as usize] = Item {
							id: entry.get_child("id").unwrap().as_string().to_string(),
							count: entry.get_child("count").unwrap().as_int() as u8,
							components: Vec::new(),
						};
					}
				}

				BlockEntityData::Crafter(data)
			}
			BlockEntityId::Dispenser | BlockEntityId::Dropper => {
				let mut data = vec![Item::default(); 9];

				if let Some(items) = value.get_child("Items") {
					for entry in items.as_list() {
						data[entry.get_child("Slot").unwrap().as_byte() as usize] = Item {
							id: entry.get_child("id").unwrap().as_string().to_string(),
							count: entry.get_child("count").unwrap().as_int() as u8,
							components: Vec::new(),
						};
					}
				}

				BlockEntityData::Dispenser(data)
			}
			BlockEntityId::Hopper => {
				let mut data = vec![Item::default(); 5];

				if let Some(items) = value.get_child("Items") {
					for entry in items.as_list() {
						data[entry.get_child("Slot").unwrap().as_byte() as usize] = Item {
							id: entry.get_child("id").unwrap().as_string().to_string(),
							count: entry.get_child("count").unwrap().as_int() as u8,
							components: Vec::new(),
						};
					}
				}

				BlockEntityData::Hopper(data)
			}
			BlockEntityId::Beacon => BlockEntityData::Beacon(
				value.get_child("primary_effect").map(|x| x.as_string().to_string()),
				value.get_child("secondary_effect").map(|x| x.as_string().to_string()),
			),
			BlockEntityId::Beehive => BlockEntityData::Beehive(
				value
					.get_child("bees")
					.unwrap()
					.as_list()
					.into_iter()
					.map(|x| Bee {
						entity_data: x.get_child("entity_data").unwrap().as_tag_compound(),
						min_ticks_in_hive: x.get_child("min_ticks_in_hive").unwrap().as_int(),
						ticks_in_hive: x.get_child("ticks_in_hive").unwrap().as_int(),
					})
					.collect(),
				value.get_child("flower_pos").unwrap_or(&NbtTag::IntArray(String::new(), vec![])).as_int_array(),
			),
			BlockEntityId::BrushableBlock => {
				if let Some(item) = value.get_child("item") {
					BlockEntityData::Brushable(Some(Item {
						id: item.get_child("id").unwrap().as_string().to_string(),
						count: item.get_child("count").unwrap().as_int() as u8,
						components: Vec::new(),
					}))
				} else {
					BlockEntityData::Brushable(None)
				}
			}
			BlockEntityId::Campfire | BlockEntityId::SoulCampfire => {
				let items: Vec<Item> = value
					.get_child("Items")
					.unwrap()
					.as_list()
					.iter()
					.map(|entry| Item {
						id: entry.get_child("id").unwrap().as_string().to_string(),
						count: entry.get_child("count").unwrap().as_int() as u8,
						components: Vec::new(),
					})
					.collect();

				BlockEntityData::Campfire(
					value.get_child("CookingTimes").unwrap().as_int_array(),
					value.get_child("CookingTotalTimes").unwrap().as_int_array(),
					items,
				)
			}
			BlockEntityId::ChiseledBookshelf => {
				let items: Vec<Item> = value
					.get_child("Items")
					.unwrap()
					.as_list()
					.iter()
					.map(|entry| Item {
						id: entry.get_child("id").unwrap().as_string().to_string(),
						count: entry.get_child("count").unwrap().as_int() as u8,
						components: Vec::new(),
					})
					.collect();

				BlockEntityData::ChiseledBookShelf(items, value.get_child("last_interacted_slot").unwrap().as_int())
			}
			BlockEntityId::Comperator => BlockEntityData::Comparator(value.get_child("OutputSignal").unwrap().as_int()),
			BlockEntityId::Conduit => BlockEntityData::Conduit(value.get_child("Target").map(|x| x.as_int_array())),
			BlockEntityId::CreakingHeart => BlockEntityData::CreakingHeart(value.get_child("Target").unwrap().as_int_array()),
			BlockEntityId::DecoratedPot => BlockEntityData::DecoratedPot(
				value
					.get_child("sherds")
					.unwrap_or(&NbtTag::List(String::new(), Vec::new()))
					.as_list()
					.iter()
					.map(|x| x.as_string().to_string())
					.collect(),
				if value.get_child("item").is_some() {
					Item {
						id: value.get_child("item").unwrap().get_child("id").unwrap().as_string().to_string(),
						count: value.get_child("item").unwrap().get_child("count").unwrap().as_int() as u8,
						components: Vec::new(),
					}
				} else {
					Item {
						id: "minecraft:air".to_string(),
						count: 0,
						components: Vec::new(),
					}
				},
			),
			BlockEntityId::EndGateway => BlockEntityData::EndGateway(
				value.get_child("Age").unwrap().as_long(),
				value.get_child("ExactTeleport").unwrap().as_byte(),
				value.get_child("exit_portal").unwrap().as_int_array(),
			),
			BlockEntityId::Sign | BlockEntityId::HangingSign => BlockEntityData::Sign(
				value.get_child("is_waxed").unwrap().as_byte(),
				value.get_child("front_text").unwrap().clone(),
				value.get_child("back_text").unwrap().clone(),
			),
			BlockEntityId::Jukebox => BlockEntityData::JukeBox(
				Item {
					id: value.get_child("RecordItem").unwrap().get_child("id").unwrap().as_string().to_string(),
					count: value.get_child("RecordItem").unwrap().get_child("count").unwrap().as_int() as u8,
					components: Vec::new(),
				},
				value.get_child("ticks_since_song_started").unwrap().as_long(),
			),
			BlockEntityId::Lectern => {
				if value.get_child("Book").is_some() {
					BlockEntityData::Lectern(
						Some(Item {
							id: value.get_child("Book").unwrap().get_child("id").unwrap().as_string().to_string(),
							count: value.get_child("Book").unwrap().get_child("count").unwrap().as_int() as u8,
							components: Vec::new(),
						}),
						Some(value.get_child("Page").unwrap().as_int()),
					)
				} else {
					BlockEntityData::Lectern(None, None)
				}
			}

			BlockEntityId::Bed
			| BlockEntityId::Bell
			| BlockEntityId::CalibratedSculkSensor
			| BlockEntityId::CommandBlock
			| BlockEntityId::DaylightDetector
			| BlockEntityId::EnchantingTable
			| BlockEntityId::EnderChest
			| BlockEntityId::EndPortal
			| BlockEntityId::Jigsaw
			| BlockEntityId::MobSpawner
			| BlockEntityId::Piston
			| BlockEntityId::ShulkerBox
			| BlockEntityId::Skull
			| BlockEntityId::SculkCatalyst
			| BlockEntityId::SculkSensor
			| BlockEntityId::SculkShrieker
			| BlockEntityId::StructureBlock
			| BlockEntityId::TrialSpawner
			| BlockEntityId::Vault => BlockEntityData::NoData,
		};

		return Ok(BlockEntityBaseData {
			id,
			position,
			components: None,
			data,
			needs_ticking: false,
		});
	}
}
