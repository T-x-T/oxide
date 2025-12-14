use std::error::Error;
use std::sync::Arc;

use data::blocks::Type;

use crate::packets::Packet;

use super::*;

#[derive(Debug, Clone)]
pub enum BlockEntity {
	Banner(crate::blockentities::banner::Banner),
	Barrel(crate::blockentities::barrel::Barrel),
	Beacon(crate::blockentities::beacon::Beacon),
	Bed(crate::blockentities::bed::Bed),
	Beehive(crate::blockentities::beehive::Beehive),
	Bell(crate::blockentities::bell::Bell),
	BlastFurnace(crate::blockentities::blast_furnace::BlastFurnace),
	BrewingStand(crate::blockentities::brewing_stand::BrewingStand),
	BrushableBlock(crate::blockentities::brushable_block::BrushableBlock),
	CalibratedSculkSensor(crate::blockentities::calibrated_sculk_sensor::CalibratedSculkSensor),
	Campfire(crate::blockentities::campfire::Campfire),
	Chest(crate::blockentities::chest::Chest),
	ChiseledBookshelf(crate::blockentities::chiseled_bookshelf::ChiseledBookshelf),
	CommandBlock(crate::blockentities::command_block::CommandBlock),
	Comparator(crate::blockentities::comparator::Comparator),
	Conduit(crate::blockentities::conduit::Conduit),
	Crafter(crate::blockentities::crafter::Crafter),
	CreakingHeart(crate::blockentities::creaking_heart::CreakingHeart),
	DaylightDetector(crate::blockentities::daylight_detector::DaylightDetector),
	DecoratedPot(crate::blockentities::decorated_pot::DecoratedPot),
	Dispenser(crate::blockentities::dispenser::Dispenser),
	Dropper(crate::blockentities::dropper::Dropper),
	EnchantingTable(crate::blockentities::enchanting_table::EnchantingTable),
	EndGateway(crate::blockentities::end_gateway::EndGateway),
	EndPortal(crate::blockentities::end_portal::EndPortal),
	EnderChest(crate::blockentities::ender_chest::EnderChest),
	Furnace(crate::blockentities::furnace::Furnace),
	HangingSign(crate::blockentities::hanging_sign::HangingSign),
	Hopper(crate::blockentities::hopper::Hopper),
	Jigsaw(crate::blockentities::jigsaw::Jigsaw),
	Jukebox(crate::blockentities::jukebox::Jukebox),
	Lectern(crate::blockentities::lectern::Lectern),
	MobSpawner(crate::blockentities::mob_spawner::MobSpawner),
	Piston(crate::blockentities::piston::Piston),
	SculkCatalyst(crate::blockentities::sculk_catalyst::SculkCatalyst),
	SculkSensor(crate::blockentities::sculk_sensor::SculkSensor),
	SculkShrieker(crate::blockentities::sculk_shrieker::SculkShrieker),
	ShulkerBox(crate::blockentities::shulker_box::ShulkerBox),
	Sign(crate::blockentities::sign::Sign),
	Skull(crate::blockentities::skull::Skull),
	Smoker(crate::blockentities::smoker::Smoker),
	SoulCampfire(crate::blockentities::soul_campfire::SoulCampfire),
	StructureBlock(crate::blockentities::structure_block::StructureBlock),
	TrappedChest(crate::blockentities::trapped_chest::TrappedChest),
	TrialSpawner(crate::blockentities::trial_spawner::TrialSpawner),
	Vault(crate::blockentities::vault::Vault),
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
			BlockEntity::Barrel(barrel) => barrel.tick(players, game),
			BlockEntity::Bed(bed) => bed.tick(players, game),
			BlockEntity::MobSpawner(mob_spawner) => mob_spawner.tick(players, game),
			BlockEntity::Beacon(beacon) => beacon.tick(players, game),
			BlockEntity::BlastFurnace(blast_furnace) => blast_furnace.tick(players, game),
			BlockEntity::BrewingStand(brewing_stand) => brewing_stand.tick(players, game),
			BlockEntity::CommandBlock(command_block) => command_block.tick(players, game),
			BlockEntity::Crafter(crafter) => crafter.tick(players, game),
			BlockEntity::Dispenser(dispenser) => dispenser.tick(players, game),
			BlockEntity::Dropper(dropper) => dropper.tick(players, game),
			BlockEntity::EnchantingTable(enchanting_table) => enchanting_table.tick(players, game),
			BlockEntity::EnderChest(ender_chest) => ender_chest.tick(players, game),
			BlockEntity::HangingSign(hanging_sign) => hanging_sign.tick(players, game),
			BlockEntity::Hopper(hopper) => hopper.tick(players, game),
			BlockEntity::Jigsaw(jigsaw) => jigsaw.tick(players, game),
			BlockEntity::Smoker(smoker) => smoker.tick(players, game),
			BlockEntity::TrappedChest(trapped_chest) => trapped_chest.tick(players, game),
			BlockEntity::Banner(banner) => banner.tick(players, game),
			BlockEntity::Beehive(beehive) => beehive.tick(players, game),
			BlockEntity::Bell(bell) => bell.tick(players, game),
			BlockEntity::BrushableBlock(brushable_block) => brushable_block.tick(players, game),
			BlockEntity::CalibratedSculkSensor(calibrated_sculk_sensor) => calibrated_sculk_sensor.tick(players, game),
			BlockEntity::Campfire(campfire) => campfire.tick(players, game),
			BlockEntity::ChiseledBookshelf(chiseled_bookshelf) => chiseled_bookshelf.tick(players, game),
			BlockEntity::Comparator(comperator) => comperator.tick(players, game),
			BlockEntity::Conduit(conduit) => conduit.tick(players, game),
			BlockEntity::CreakingHeart(creaking_heart) => creaking_heart.tick(players, game),
			BlockEntity::DaylightDetector(daylight_detector) => daylight_detector.tick(players, game),
			BlockEntity::DecoratedPot(decorated_pot) => decorated_pot.tick(players, game),
			BlockEntity::EndGateway(end_gateway) => end_gateway.tick(players, game),
			BlockEntity::EndPortal(end_portal) => end_portal.tick(players, game),
			BlockEntity::Jukebox(jukebox) => jukebox.tick(players, game),
			BlockEntity::Lectern(lectern) => lectern.tick(players, game),
			BlockEntity::Piston(piston) => piston.tick(players, game),
			BlockEntity::SculkCatalyst(sculk_catalyst) => sculk_catalyst.tick(players, game),
			BlockEntity::SculkSensor(sculk_sensor) => sculk_sensor.tick(players, game),
			BlockEntity::SculkShrieker(sculk_shrieker) => sculk_shrieker.tick(players, game),
			BlockEntity::ShulkerBox(shulker_box) => shulker_box.tick(players, game),
			BlockEntity::Skull(skull) => skull.tick(players, game),
			BlockEntity::SoulCampfire(soul_campfire) => soul_campfire.tick(players, game),
			BlockEntity::StructureBlock(structure_block) => structure_block.tick(players, game),
			BlockEntity::TrialSpawner(trial_spawner) => trial_spawner.tick(players, game),
			BlockEntity::Vault(vault) => vault.tick(players, game),
		}
	}

	pub fn new_from_block(block_type: data::blocks::Type, position: BlockPosition) -> Option<Self> {
		return match block_type {
			Type::Furnace => Some(BlockEntity::Furnace(crate::blockentities::furnace::Furnace::new(position))),
			Type::Chest => Some(BlockEntity::Chest(crate::blockentities::chest::Chest::new(position))),
			Type::WallSign | Type::StandingSign => Some(BlockEntity::Sign(crate::blockentities::sign::Sign::new(position))),
			Type::Barrel => Some(BlockEntity::Barrel(crate::blockentities::barrel::Barrel::new(position))),
			Type::Bed => Some(BlockEntity::Bed(crate::blockentities::bed::Bed::new(position))),
			Type::Spawner => Some(BlockEntity::MobSpawner(crate::blockentities::mob_spawner::MobSpawner::new(position))),
			Type::Beacon => Some(BlockEntity::Beacon(crate::blockentities::beacon::Beacon::new(position))),
			Type::BlastFurnace => Some(BlockEntity::BlastFurnace(crate::blockentities::blast_furnace::BlastFurnace::new(position))),
			Type::BrewingStand => Some(BlockEntity::BrewingStand(crate::blockentities::brewing_stand::BrewingStand::new(position))),
			Type::Command => Some(BlockEntity::CommandBlock(crate::blockentities::command_block::CommandBlock::new(position))),
			Type::Crafter => Some(BlockEntity::Crafter(crate::blockentities::crafter::Crafter::new(position))),
			Type::Dispenser => Some(BlockEntity::Dispenser(crate::blockentities::dispenser::Dispenser::new(position))),
			Type::Dropper => Some(BlockEntity::Dropper(crate::blockentities::dropper::Dropper::new(position))),
			Type::EnchantmentTable => Some(BlockEntity::EnchantingTable(crate::blockentities::enchanting_table::EnchantingTable::new(position))),
			Type::EnderChest => Some(BlockEntity::EnderChest(crate::blockentities::ender_chest::EnderChest::new(position))),
			Type::WallHangingSign | Type::CeilingHangingSign => {
				Some(BlockEntity::HangingSign(crate::blockentities::hanging_sign::HangingSign::new(position)))
			}
			Type::Hopper => Some(BlockEntity::Hopper(crate::blockentities::hopper::Hopper::new(position))),
			Type::Jigsaw => Some(BlockEntity::Jigsaw(crate::blockentities::jigsaw::Jigsaw::new(position))),
			Type::Smoker => Some(BlockEntity::Smoker(crate::blockentities::smoker::Smoker::new(position))),
			Type::TrappedChest => Some(BlockEntity::TrappedChest(crate::blockentities::trapped_chest::TrappedChest::new(position))),
			Type::Banner | Type::WallBanner => Some(BlockEntity::Banner(crate::blockentities::banner::Banner::new(position))),
			Type::Beehive => Some(BlockEntity::Beehive(crate::blockentities::beehive::Beehive::new(position))),
			Type::Bell => Some(BlockEntity::Bell(crate::blockentities::bell::Bell::new(position))),
			Type::Brushable => Some(BlockEntity::BrushableBlock(crate::blockentities::brushable_block::BrushableBlock::new(position))),
			Type::CalibratedSculkSensor => {
				Some(BlockEntity::CalibratedSculkSensor(crate::blockentities::calibrated_sculk_sensor::CalibratedSculkSensor::new(position)))
			}
			Type::Campfire => Some(BlockEntity::Campfire(crate::blockentities::campfire::Campfire::new(position))), //TODO: check if this is a soul campfire
			Type::ChiseledBookShelf => {
				Some(BlockEntity::ChiseledBookshelf(crate::blockentities::chiseled_bookshelf::ChiseledBookshelf::new(position)))
			}
			Type::Comparator => Some(BlockEntity::Comparator(crate::blockentities::comparator::Comparator::new(position))),
			Type::Conduit => Some(BlockEntity::Conduit(crate::blockentities::conduit::Conduit::new(position))),
			Type::CreakingHeart => Some(BlockEntity::CreakingHeart(crate::blockentities::creaking_heart::CreakingHeart::new(position))),
			Type::DaylightDetector => {
				Some(BlockEntity::DaylightDetector(crate::blockentities::daylight_detector::DaylightDetector::new(position)))
			}
			Type::DecoratedPot => Some(BlockEntity::DecoratedPot(crate::blockentities::decorated_pot::DecoratedPot::new(position))),
			Type::EndGateway => Some(BlockEntity::EndGateway(crate::blockentities::end_gateway::EndGateway::new(position))),
			Type::EndPortal => Some(BlockEntity::EndPortal(crate::blockentities::end_portal::EndPortal::new(position))),
			Type::Jukebox => Some(BlockEntity::Jukebox(crate::blockentities::jukebox::Jukebox::new(position))),
			Type::Lectern => Some(BlockEntity::Lectern(crate::blockentities::lectern::Lectern::new(position))),
			Type::MovingPiston => Some(BlockEntity::Piston(crate::blockentities::piston::Piston::new(position))),
			Type::SculkCatalyst => Some(BlockEntity::SculkCatalyst(crate::blockentities::sculk_catalyst::SculkCatalyst::new(position))),
			Type::SculkSensor => Some(BlockEntity::SculkSensor(crate::blockentities::sculk_sensor::SculkSensor::new(position))),
			Type::SculkShrieker => Some(BlockEntity::SculkShrieker(crate::blockentities::sculk_shrieker::SculkShrieker::new(position))),
			Type::ShulkerBox => Some(BlockEntity::ShulkerBox(crate::blockentities::shulker_box::ShulkerBox::new(position))),
			Type::Skull | Type::WallSkull => Some(BlockEntity::Skull(crate::blockentities::skull::Skull::new(position))),
			Type::Structure => Some(BlockEntity::StructureBlock(crate::blockentities::structure_block::StructureBlock::new(position))),
			Type::TrialSpawner => Some(BlockEntity::TrialSpawner(crate::blockentities::trial_spawner::TrialSpawner::new(position))),
			Type::Vault => Some(BlockEntity::Vault(crate::blockentities::vault::Vault::new(position))),
			_ => None,
		};
	}

	pub fn get_position(&self) -> BlockPosition {
		return match self {
			BlockEntity::Furnace(furnace) => furnace.position,
			BlockEntity::Chest(chest) => chest.position,
			BlockEntity::Sign(sign) => sign.position,
			BlockEntity::Barrel(barrel) => barrel.position,
			BlockEntity::Bed(bed) => bed.position,
			BlockEntity::MobSpawner(mob_spawner) => mob_spawner.position,
			BlockEntity::Beacon(beacon) => beacon.position,
			BlockEntity::BlastFurnace(blast_furnace) => blast_furnace.position,
			BlockEntity::BrewingStand(brewing_stand) => brewing_stand.position,
			BlockEntity::CommandBlock(command_block) => command_block.position,
			BlockEntity::Crafter(crafter) => crafter.position,
			BlockEntity::Dispenser(dispenser) => dispenser.position,
			BlockEntity::Dropper(dropper) => dropper.position,
			BlockEntity::EnchantingTable(enchanting_table) => enchanting_table.position,
			BlockEntity::EnderChest(ender_chest) => ender_chest.position,
			BlockEntity::HangingSign(hanging_sign) => hanging_sign.position,
			BlockEntity::Hopper(hopper) => hopper.position,
			BlockEntity::Jigsaw(jigsaw) => jigsaw.position,
			BlockEntity::Smoker(smoker) => smoker.position,
			BlockEntity::TrappedChest(trapped_chest) => trapped_chest.position,
			BlockEntity::Banner(banner) => banner.position,
			BlockEntity::Beehive(beehive) => beehive.position,
			BlockEntity::Bell(bell) => bell.position,
			BlockEntity::BrushableBlock(brushable_block) => brushable_block.position,
			BlockEntity::CalibratedSculkSensor(calibrated_sculk_sensor) => calibrated_sculk_sensor.position,
			BlockEntity::Campfire(campfire) => campfire.position,
			BlockEntity::ChiseledBookshelf(chiseled_bookshelf) => chiseled_bookshelf.position,
			BlockEntity::Comparator(comparator) => comparator.position,
			BlockEntity::Conduit(conduit) => conduit.position,
			BlockEntity::CreakingHeart(creaking_heart) => creaking_heart.position,
			BlockEntity::DaylightDetector(daylight_detector) => daylight_detector.position,
			BlockEntity::DecoratedPot(decorated_pot) => decorated_pot.position,
			BlockEntity::EndGateway(end_gateway) => end_gateway.position,
			BlockEntity::EndPortal(end_portal) => end_portal.position,
			BlockEntity::Jukebox(jukebox) => jukebox.position,
			BlockEntity::Lectern(lectern) => lectern.position,
			BlockEntity::Piston(piston) => piston.position,
			BlockEntity::SculkCatalyst(sculk_catalyst) => sculk_catalyst.position,
			BlockEntity::SculkSensor(sculk_sensor) => sculk_sensor.position,
			BlockEntity::SculkShrieker(sculk_shrieker) => sculk_shrieker.position,
			BlockEntity::ShulkerBox(shulker_box) => shulker_box.position,
			BlockEntity::Skull(skull) => skull.position,
			BlockEntity::SoulCampfire(soul_campfire) => soul_campfire.position,
			BlockEntity::StructureBlock(structure_block) => structure_block.position,
			BlockEntity::TrialSpawner(trial_spawner) => trial_spawner.position,
			BlockEntity::Vault(vault) => vault.position,
		};
	}

	pub fn get_id(&self) -> String {
		match self {
			BlockEntity::Furnace(_) => "minecraft:furnace".to_string(),
			BlockEntity::Chest(_) => "minecraft:chest".to_string(),
			BlockEntity::Sign(_) => "minecraft:sign".to_string(),
			BlockEntity::Barrel(_) => "minecraft:barrel".to_string(),
			BlockEntity::Bed(_) => "minecraft:bed".to_string(),
			BlockEntity::MobSpawner(_) => "minecraft:mob_spawner".to_string(),
			BlockEntity::Beacon(_) => "minecraft:beacon".to_string(),
			BlockEntity::BlastFurnace(_) => "minecraft:blast_furnace".to_string(),
			BlockEntity::BrewingStand(_) => "minecraft:brewing_stand".to_string(),
			BlockEntity::CommandBlock(_) => "minecraft:command_block".to_string(),
			BlockEntity::Crafter(_) => "minecraft:crafter".to_string(),
			BlockEntity::Dispenser(_) => "minecraft:dispenser".to_string(),
			BlockEntity::Dropper(_) => "minecraft:dropper".to_string(),
			BlockEntity::EnchantingTable(_) => "minecraft:enchanting_table".to_string(),
			BlockEntity::EnderChest(_) => "minecraft:ender_chest".to_string(),
			BlockEntity::HangingSign(_) => "minecraft:hanging_sign".to_string(),
			BlockEntity::Hopper(_) => "minecraft:hopper".to_string(),
			BlockEntity::Jigsaw(_) => "minecraft:jigsaw".to_string(),
			BlockEntity::Smoker(_) => "minecraft:smoker".to_string(),
			BlockEntity::TrappedChest(_) => "minecraft:trapped_chest".to_string(),
			BlockEntity::Banner(_) => "minecraft:banner".to_string(),
			BlockEntity::Beehive(_) => "minecraft:beehive".to_string(),
			BlockEntity::Bell(_) => "minecraft:bell".to_string(),
			BlockEntity::BrushableBlock(_) => "minecraft:brushable_block".to_string(),
			BlockEntity::CalibratedSculkSensor(_) => "minecraft:calibrated_sculk_sensor".to_string(),
			BlockEntity::Campfire(_) => "minecraft:campfire".to_string(),
			BlockEntity::ChiseledBookshelf(_) => "minecraft:chiseled_bookshelf".to_string(),
			BlockEntity::Comparator(_) => "minecraft:comparator".to_string(),
			BlockEntity::Conduit(_) => "minecraft:conduit".to_string(),
			BlockEntity::CreakingHeart(_) => "minecraft:creaking_heart".to_string(),
			BlockEntity::DaylightDetector(_) => "minecraft:daylight_detector".to_string(),
			BlockEntity::DecoratedPot(_) => "minecraft:decorated_pot".to_string(),
			BlockEntity::EndGateway(_) => "minecraft:end_gateway".to_string(),
			BlockEntity::EndPortal(_) => "minecraft:end_portal".to_string(),
			BlockEntity::Jukebox(_) => "minecraft:jukebox".to_string(),
			BlockEntity::Lectern(_) => "minecraft:lectern".to_string(),
			BlockEntity::Piston(_) => "minecraft:piston".to_string(),
			BlockEntity::SculkCatalyst(_) => "minecraft:sculk_catalyst".to_string(),
			BlockEntity::SculkSensor(_) => "minecraft:sculk_sensor".to_string(),
			BlockEntity::SculkShrieker(_) => "minecraft:sculk_shrieker".to_string(),
			BlockEntity::ShulkerBox(_) => "minecraft:shulker_box".to_string(),
			BlockEntity::Skull(_) => "minecraft:skull".to_string(),
			BlockEntity::SoulCampfire(_) => "minecraft:soul_campfire".to_string(),
			BlockEntity::StructureBlock(_) => "minecraft:structure_block".to_string(),
			BlockEntity::TrialSpawner(_) => "minecraft:trial_spawner".to_string(),
			BlockEntity::Vault(_) => "minecraft:vault".to_string(),
		}
	}

	pub fn get_contained_items_owned(&self) -> Vec<Item> {
		return match self {
			BlockEntity::Furnace(furnace) => furnace.get_contained_items_owned(),
			BlockEntity::Chest(chest) => chest.get_contained_items_owned(),
			BlockEntity::Barrel(barrel) => barrel.get_contained_items_owned(),
			BlockEntity::BlastFurnace(blast_furnace) => blast_furnace.get_contained_items_owned(),
			BlockEntity::BrewingStand(brewing_stand) => brewing_stand.get_contained_items_owned(),
			BlockEntity::Crafter(crafter) => crafter.get_contained_items_owned(),
			BlockEntity::Dispenser(dispenser) => dispenser.get_contained_items_owned(),
			BlockEntity::Dropper(dropper) => dropper.get_contained_items_owned(),
			BlockEntity::Hopper(hopper) => hopper.get_contained_items_owned(),
			BlockEntity::Smoker(smoker) => smoker.get_contained_items_owned(),
			BlockEntity::ShulkerBox(shulker_box) => shulker_box.get_contained_items_owned(),
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
		#[allow(clippy::single_match)]
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
			BlockEntityId::Barrel => BlockEntity::Barrel(crate::blockentities::barrel::Barrel::try_from(value)?),
			BlockEntityId::Beacon => BlockEntity::Beacon(crate::blockentities::beacon::Beacon::try_from(value)?),
			BlockEntityId::Bed => BlockEntity::Bed(crate::blockentities::bed::Bed::try_from(value)?),
			BlockEntityId::BlastFurnace => BlockEntity::BlastFurnace(crate::blockentities::blast_furnace::BlastFurnace::try_from(value)?),
			BlockEntityId::BrewingStand => BlockEntity::BrewingStand(crate::blockentities::brewing_stand::BrewingStand::try_from(value)?),
			BlockEntityId::Chest => BlockEntity::Chest(crate::blockentities::chest::Chest::try_from(value)?),
			BlockEntityId::CommandBlock => BlockEntity::CommandBlock(crate::blockentities::command_block::CommandBlock::try_from(value)?),
			BlockEntityId::Crafter => BlockEntity::Crafter(crate::blockentities::crafter::Crafter::try_from(value)?),
			BlockEntityId::Dispenser => BlockEntity::Dispenser(crate::blockentities::dispenser::Dispenser::try_from(value)?),
			BlockEntityId::Dropper => BlockEntity::Dropper(crate::blockentities::dropper::Dropper::try_from(value)?),
			BlockEntityId::EnchantingTable => {
				BlockEntity::EnchantingTable(crate::blockentities::enchanting_table::EnchantingTable::try_from(value)?)
			}
			BlockEntityId::EnderChest => BlockEntity::EnderChest(crate::blockentities::ender_chest::EnderChest::try_from(value)?),
			BlockEntityId::Furnace => BlockEntity::Furnace(crate::blockentities::furnace::Furnace::try_from(value)?),
			BlockEntityId::HangingSign => BlockEntity::HangingSign(crate::blockentities::hanging_sign::HangingSign::try_from(value)?),
			BlockEntityId::Hopper => BlockEntity::Hopper(crate::blockentities::hopper::Hopper::try_from(value)?),
			BlockEntityId::Jigsaw => BlockEntity::Jigsaw(crate::blockentities::jigsaw::Jigsaw::try_from(value)?),
			BlockEntityId::MobSpawner => BlockEntity::MobSpawner(crate::blockentities::mob_spawner::MobSpawner::try_from(value)?),
			BlockEntityId::Sign => BlockEntity::Sign(crate::blockentities::sign::Sign::try_from(value)?),
			BlockEntityId::Smoker => BlockEntity::Smoker(crate::blockentities::smoker::Smoker::try_from(value)?),
			BlockEntityId::TrappedChest => BlockEntity::TrappedChest(crate::blockentities::trapped_chest::TrappedChest::try_from(value)?),
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
			BlockEntity::Barrel(barrel) => barrel.into(),
			BlockEntity::Bed(bed) => bed.into(),
			BlockEntity::MobSpawner(mob_spawner) => mob_spawner.into(),
			BlockEntity::Beacon(beacon) => beacon.into(),
			BlockEntity::BlastFurnace(blast_furnace) => blast_furnace.into(),
			BlockEntity::BrewingStand(brewing_stand) => brewing_stand.into(),
			BlockEntity::CommandBlock(command_block) => command_block.into(),
			BlockEntity::Crafter(crafter) => crafter.into(),
			BlockEntity::Dispenser(dispenser) => dispenser.into(),
			BlockEntity::Dropper(dropper) => dropper.into(),
			BlockEntity::EnchantingTable(enchanting_table) => enchanting_table.into(),
			BlockEntity::EnderChest(ender_chest) => ender_chest.into(),
			BlockEntity::HangingSign(hanging_sign) => hanging_sign.into(),
			BlockEntity::Hopper(hopper) => hopper.into(),
			BlockEntity::Jigsaw(jigsaw) => jigsaw.into(),
			BlockEntity::Smoker(smoker) => smoker.into(),
			BlockEntity::TrappedChest(trapped_chest) => trapped_chest.into(),
			BlockEntity::Banner(banner) => banner.into(),
			BlockEntity::Beehive(beehive) => beehive.into(),
			BlockEntity::Bell(bell) => bell.into(),
			BlockEntity::BrushableBlock(brushable_block) => brushable_block.into(),
			BlockEntity::CalibratedSculkSensor(calibrated_sculk_sensor) => calibrated_sculk_sensor.into(),
			BlockEntity::Campfire(campfire) => campfire.into(),
			BlockEntity::ChiseledBookshelf(chiseled_bookshelf) => chiseled_bookshelf.into(),
			BlockEntity::Comparator(comparator) => comparator.into(),
			BlockEntity::Conduit(conduit) => conduit.into(),
			BlockEntity::CreakingHeart(creaking_heart) => creaking_heart.into(),
			BlockEntity::DaylightDetector(daylight_detector) => daylight_detector.into(),
			BlockEntity::DecoratedPot(decorated_pot) => decorated_pot.into(),
			BlockEntity::EndGateway(end_gateway) => end_gateway.into(),
			BlockEntity::EndPortal(end_portal) => end_portal.into(),
			BlockEntity::Jukebox(jukebox) => jukebox.into(),
			BlockEntity::Lectern(lectern) => lectern.into(),
			BlockEntity::Piston(piston) => piston.into(),
			BlockEntity::SculkCatalyst(sculk_catalyst) => sculk_catalyst.into(),
			BlockEntity::SculkSensor(sculk_sensor) => sculk_sensor.into(),
			BlockEntity::SculkShrieker(sculk_shrieker) => sculk_shrieker.into(),
			BlockEntity::ShulkerBox(shulker_box) => shulker_box.into(),
			BlockEntity::Skull(skull) => skull.into(),
			BlockEntity::SoulCampfire(soul_campfire) => soul_campfire.into(),
			BlockEntity::StructureBlock(structure_block) => structure_block.into(),
			BlockEntity::TrialSpawner(trial_spawner) => trial_spawner.into(),
			BlockEntity::Vault(vault) => vault.into(),
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
