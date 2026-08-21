#[cfg(test)]
mod test;

mod breedable_mob;
mod common_entity;
mod common_mob;

pub use breedable_mob::*;
pub use common_entity::*;
pub use common_mob::*;

use rand::Rng;
use std::collections::HashMap;

use crate::entity::*;
use crate::packets::Packet;
use crate::packets::clientbound::play::EntityMetadata;
use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Entity {
	Armadillo(Armadillo),
	Cat(Cat),
	ChestMinecart(ChestMinecart),
	Chicken(Chicken),
	Cow(Cow),
	Creeper(Creeper),
	Donkey(Donkey),
	Horse(Horse),
	Item(ItemEntity),
	Parrot(Parrot),
	Pig(Pig),
	Rabbit(Rabbit),
	Sheep(Sheep),
	Player(Player),
	EnderDragon(EnderDragon),
	EndCrystal(EndCrystal),
}

#[derive(Debug, PartialEq)]
pub enum EntityTickOutcome {
	SelfDied,
	RemoveSelf,
	RemoveOthers(Vec<i32>),
	Updated,
	DamageSelf(f32),
	SummonEntity(Box<Entity>),
	DoneBreeding(i32, i32),
	ReplaceBlock(BlockPosition, u16),
	UseNetherPortal(String), //target dimension
	UseEndPortal(String),    //target dimension
	KilledBy(Box<Entity>),
	LoadChunk(i32, i32),
	AddEntity(Box<Entity>),
	DealDamage(i32, f32), //target entity id, damage to deal
	UpdateDebugDataPathfinding(Option<DebugEntityPath>),
}

impl Entity {
	pub fn to_nbt(&self) -> NbtListTag {
		let common_data = self.get_common_entity_data();
		let default_tags = vec![
			NbtTag::String("id".to_string(), data::entities::get_name_from_id(self.get_type())),
			NbtTag::List(
				"Pos".to_string(),
				vec![
					NbtListTag::Double(common_data.position.x),
					NbtListTag::Double(common_data.position.y),
					NbtListTag::Double(common_data.position.z),
				],
			),
			NbtTag::List(
				"Motion".to_string(),
				vec![
					NbtListTag::Double(common_data.velocity.x),
					NbtListTag::Double(common_data.velocity.y),
					NbtListTag::Double(common_data.velocity.z),
				],
			),
			NbtTag::List(
				"Rotation".to_string(),
				vec![NbtListTag::Float(common_data.position.yaw), NbtListTag::Float(common_data.position.pitch)],
			),
			NbtTag::IntArray(
				"UUID".to_string(),
				vec![
					(common_data.uuid >> 96) as i32,
					(common_data.uuid << 32 >> 96) as i32,
					(common_data.uuid << 64 >> 96) as i32,
					(common_data.uuid << 96 >> 96) as i32,
				],
			),
			NbtTag::Short("Air".to_string(), common_data.air),
			common_data.custom_name.clone(),
			common_data.data.clone(),
			NbtTag::Double("fall_distance".to_string(), common_data.fall_distance),
			NbtTag::Short("Fire".to_string(), common_data.ticks_until_fire_is_out),
			NbtTag::Byte("Glowing".to_string(), if common_data.is_glowing { 1 } else { 0 }),
			NbtTag::Byte("HasVisualFire".to_string(), if common_data.has_visual_fire { 1 } else { 0 }),
			NbtTag::Byte("Invulnerable".to_string(), if common_data.invulnerable { 1 } else { 0 }),
			NbtTag::Byte("NoGravity".to_string(), if common_data.no_gravity { 1 } else { 0 }),
			NbtTag::Byte("OnGround".to_string(), if common_data.on_ground { 1 } else { 0 }),
			NbtTag::List("Passengers".to_string(), common_data.passengers.iter().map(|x| x.to_nbt()).collect()),
			NbtTag::Int("PortalCooldown".to_string(), common_data.portal_cooldown),
			NbtTag::Byte("Silent".to_string(), if common_data.is_silent { 1 } else { 0 }),
			NbtTag::List("Tags".to_string(), common_data.scoreboard_tags.clone()),
			NbtTag::Int("TicksFrozen".to_string(), common_data.ticks_frozen),
		];

		return NbtListTag::TagCompound(vec![default_tags, self.to_nbt_extras()].into_iter().flatten().collect());
	}

	pub fn get_common_entity_data(&self) -> &CommonEntity {
		return match self {
			Entity::Armadillo(x) => x.get_common_entity_data(),
			Entity::Cat(x) => x.get_common_entity_data(),
			Entity::ChestMinecart(x) => x.get_common_entity_data(),
			Entity::Chicken(x) => x.get_common_entity_data(),
			Entity::Cow(x) => x.get_common_entity_data(),
			Entity::Creeper(x) => x.get_common_entity_data(),
			Entity::Donkey(x) => x.get_common_entity_data(),
			Entity::Horse(x) => x.get_common_entity_data(),
			Entity::Item(x) => x.get_common_entity_data(),
			Entity::Parrot(x) => x.get_common_entity_data(),
			Entity::Pig(x) => x.get_common_entity_data(),
			Entity::Rabbit(x) => x.get_common_entity_data(),
			Entity::Sheep(x) => x.get_common_entity_data(),
			Entity::Player(x) => x.get_common_entity_data(),
			Entity::EnderDragon(x) => x.get_common_entity_data(),
			Entity::EndCrystal(x) => x.get_common_entity_data(),
		};
	}

	pub fn get_common_entity_data_mut(&mut self) -> &mut CommonEntity {
		return match self {
			Entity::Armadillo(x) => x.get_common_entity_data_mut(),
			Entity::Cat(x) => x.get_common_entity_data_mut(),
			Entity::ChestMinecart(x) => x.get_common_entity_data_mut(),
			Entity::Chicken(x) => x.get_common_entity_data_mut(),
			Entity::Cow(x) => x.get_common_entity_data_mut(),
			Entity::Creeper(x) => x.get_common_entity_data_mut(),
			Entity::Donkey(x) => x.get_common_entity_data_mut(),
			Entity::Horse(x) => x.get_common_entity_data_mut(),
			Entity::Item(x) => x.get_common_entity_data_mut(),
			Entity::Parrot(x) => x.get_common_entity_data_mut(),
			Entity::Pig(x) => x.get_common_entity_data_mut(),
			Entity::Rabbit(x) => x.get_common_entity_data_mut(),
			Entity::Sheep(x) => x.get_common_entity_data_mut(),
			Entity::Player(x) => x.get_common_entity_data_mut(),
			Entity::EnderDragon(x) => x.get_common_entity_data_mut(),
			Entity::EndCrystal(x) => x.get_common_entity_data_mut(),
		};
	}

	pub fn get_mob_data(&self) -> &CommonMob {
		return match self {
			Entity::Armadillo(x) => x.get_mob_data(),
			Entity::Cat(x) => x.get_mob_data(),
			Entity::ChestMinecart(x) => x.get_mob_data(),
			Entity::Chicken(x) => x.get_mob_data(),
			Entity::Cow(x) => x.get_mob_data(),
			Entity::Creeper(x) => x.get_mob_data(),
			Entity::Donkey(x) => x.get_mob_data(),
			Entity::Horse(x) => x.get_mob_data(),
			Entity::Item(x) => x.get_mob_data(),
			Entity::Parrot(x) => x.get_mob_data(),
			Entity::Pig(x) => x.get_mob_data(),
			Entity::Rabbit(x) => x.get_mob_data(),
			Entity::Sheep(x) => x.get_mob_data(),
			Entity::Player(x) => x.get_mob_data(),
			Entity::EnderDragon(x) => x.get_mob_data(),
			Entity::EndCrystal(x) => x.get_mob_data(),
		};
	}

	pub fn get_mob_data_mut(&mut self) -> &mut CommonMob {
		return match self {
			Entity::Armadillo(x) => x.get_mob_data_mut(),
			Entity::Cat(x) => x.get_mob_data_mut(),
			Entity::ChestMinecart(x) => x.get_mob_data_mut(),
			Entity::Chicken(x) => x.get_mob_data_mut(),
			Entity::Cow(x) => x.get_mob_data_mut(),
			Entity::Creeper(x) => x.get_mob_data_mut(),
			Entity::Donkey(x) => x.get_mob_data_mut(),
			Entity::Horse(x) => x.get_mob_data_mut(),
			Entity::Item(x) => x.get_mob_data_mut(),
			Entity::Parrot(x) => x.get_mob_data_mut(),
			Entity::Pig(x) => x.get_mob_data_mut(),
			Entity::Rabbit(x) => x.get_mob_data_mut(),
			Entity::Sheep(x) => x.get_mob_data_mut(),
			Entity::Player(x) => x.get_mob_data_mut(),
			Entity::EnderDragon(x) => x.get_mob_data_mut(),
			Entity::EndCrystal(x) => x.get_mob_data_mut(),
		};
	}

	pub fn get_type(&self) -> i32 {
		return match self {
			Entity::Armadillo(x) => x.get_type(),
			Entity::Cat(x) => x.get_type(),
			Entity::ChestMinecart(x) => x.get_type(),
			Entity::Chicken(x) => x.get_type(),
			Entity::Cow(x) => x.get_type(),
			Entity::Creeper(x) => x.get_type(),
			Entity::Donkey(x) => x.get_type(),
			Entity::Horse(x) => x.get_type(),
			Entity::Item(x) => x.get_type(),
			Entity::Parrot(x) => x.get_type(),
			Entity::Pig(x) => x.get_type(),
			Entity::Rabbit(x) => x.get_type(),
			Entity::Sheep(x) => x.get_type(),
			Entity::Player(x) => x.get_type(),
			Entity::EnderDragon(x) => x.get_type(),
			Entity::EndCrystal(x) => x.get_type(),
		};
	}

	pub fn to_nbt_extras(&self) -> Vec<NbtTag> {
		return match self {
			Entity::Armadillo(x) => x.to_nbt_extras(),
			Entity::Cat(x) => x.to_nbt_extras(),
			Entity::ChestMinecart(x) => x.to_nbt_extras(),
			Entity::Chicken(x) => x.to_nbt_extras(),
			Entity::Cow(x) => x.to_nbt_extras(),
			Entity::Creeper(x) => x.to_nbt_extras(),
			Entity::Donkey(x) => x.to_nbt_extras(),
			Entity::Horse(x) => x.to_nbt_extras(),
			Entity::Item(x) => x.to_nbt_extras(),
			Entity::Parrot(x) => x.to_nbt_extras(),
			Entity::Pig(x) => x.to_nbt_extras(),
			Entity::Rabbit(x) => x.to_nbt_extras(),
			Entity::Sheep(x) => x.to_nbt_extras(),
			Entity::Player(x) => x.to_nbt_extras(),
			Entity::EnderDragon(x) => x.to_nbt_extras(),
			Entity::EndCrystal(x) => x.to_nbt_extras(),
		};
	}

	pub fn to_spawn_entity_packet(&self) -> crate::packets::clientbound::play::SpawnEntity {
		return crate::packets::clientbound::play::SpawnEntity {
			entity_id: self.get_common_entity_data().entity_id,
			entity_uuid: self.get_common_entity_data().uuid,
			entity_type: self.get_type(),
			x: self.get_common_entity_data().position.x,
			y: self.get_common_entity_data().position.y,
			z: self.get_common_entity_data().position.z,
			pitch: self.get_pitch_u8(),
			yaw: self.get_yaw_u8(),
			head_yaw: 0,
			data: 0,
			velocity_x: 0,
			velocity_y: 0,
			velocity_z: 0,
		};
	}

	pub fn interact(
		&mut self,
		held_item: &Slot,
		dim: &mut Dimension,
		players_clone: &[Player],
		players: &mut [Player],
		player_uuid: u128,
		packet_sndr: &PacketSender,
		entity_id_mgr: &EntityIdManager,
		block_states: &HashMap<String, basic_types::blocks::Block>,
	) -> Vec<EntityTickOutcome> {
		return match self {
			Entity::Armadillo(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::Cat(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::ChestMinecart(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::Chicken(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::Cow(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::Creeper(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::Donkey(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::Horse(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::Item(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::Parrot(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::Pig(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::Rabbit(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::Sheep(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::Player(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::EnderDragon(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
			Entity::EndCrystal(x) => x.interact(held_item, dim, players_clone, players, player_uuid, packet_sndr, entity_id_mgr, block_states),
		};
	}

	pub fn get_yaw_u8(&self) -> u8 {
		return if self.get_common_entity_data().position.yaw < 0.0 {
			(((self.get_common_entity_data().position.yaw / 90.0) * 64.0) + 256.0) as u8
		} else {
			((self.get_common_entity_data().position.yaw / 90.0) * 64.0) as u8
		};
	}

	pub fn get_pitch_u8(&self) -> u8 {
		return if self.get_common_entity_data().position.pitch < 0.0 {
			(((self.get_common_entity_data().position.pitch / 90.0) * 64.0) + 256.0) as u8
		} else {
			((self.get_common_entity_data().position.pitch / 90.0) * 64.0) as u8
		};
	}

	pub fn is_mob(&self) -> bool {
		return match self {
			Entity::Armadillo(x) => x.is_mob(),
			Entity::Cat(x) => x.is_mob(),
			Entity::ChestMinecart(x) => x.is_mob(),
			Entity::Chicken(x) => x.is_mob(),
			Entity::Cow(x) => x.is_mob(),
			Entity::Creeper(x) => x.is_mob(),
			Entity::Donkey(x) => x.is_mob(),
			Entity::Horse(x) => x.is_mob(),
			Entity::Item(x) => x.is_mob(),
			Entity::Parrot(x) => x.is_mob(),
			Entity::Pig(x) => x.is_mob(),
			Entity::Rabbit(x) => x.is_mob(),
			Entity::Sheep(x) => x.is_mob(),
			Entity::Player(x) => x.is_mob(),
			Entity::EnderDragon(x) => x.is_mob(),
			Entity::EndCrystal(x) => x.is_mob(),
		};
	}

	pub fn get_metadata(&self) -> Vec<EntityMetadata> {
		return match self {
			Entity::Armadillo(x) => x.get_metadata(),
			Entity::Cat(x) => x.get_metadata(),
			Entity::ChestMinecart(x) => x.get_metadata(),
			Entity::Chicken(x) => x.get_metadata(),
			Entity::Cow(x) => x.get_metadata(),
			Entity::Creeper(x) => x.get_metadata(),
			Entity::Donkey(x) => x.get_metadata(),
			Entity::Horse(x) => x.get_metadata(),
			Entity::Item(x) => x.get_metadata(),
			Entity::Parrot(x) => x.get_metadata(),
			Entity::Pig(x) => x.get_metadata(),
			Entity::Rabbit(x) => x.get_metadata(),
			Entity::Sheep(x) => x.get_metadata(),
			Entity::Player(x) => x.get_metadata(),
			Entity::EnderDragon(x) => x.get_metadata(),
			Entity::EndCrystal(x) => x.get_metadata(),
		};
	}

	pub fn get_hitbox(&self) -> (f64, f64) {
		return match self {
			Entity::Armadillo(x) => x.get_hitbox(),
			Entity::Cat(x) => x.get_hitbox(),
			Entity::ChestMinecart(x) => x.get_hitbox(),
			Entity::Chicken(x) => x.get_hitbox(),
			Entity::Cow(x) => x.get_hitbox(),
			Entity::Creeper(x) => x.get_hitbox(),
			Entity::Donkey(x) => x.get_hitbox(),
			Entity::Horse(x) => x.get_hitbox(),
			Entity::Item(x) => x.get_hitbox(),
			Entity::Parrot(x) => x.get_hitbox(),
			Entity::Pig(x) => x.get_hitbox(),
			Entity::Rabbit(x) => x.get_hitbox(),
			Entity::Sheep(x) => x.get_hitbox(),
			Entity::Player(x) => x.get_hitbox(),
			Entity::EnderDragon(x) => x.get_hitbox(),
			Entity::EndCrystal(x) => x.get_hitbox(),
		};
	}

	pub fn tick(
		&mut self,
		dimension: &Dimension,
		players: &[Player],
		packet_sender: &PacketSender,
		entity_id_manager: &EntityIdManager,
		block_state_data: &HashMap<String, basic_types::blocks::Block>,
	) -> Vec<EntityTickOutcome> {
		return match self {
			Entity::Armadillo(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::Cat(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::ChestMinecart(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::Chicken(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::Cow(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::Creeper(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::Donkey(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::Horse(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::Item(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::Parrot(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::Pig(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::Rabbit(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::Sheep(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::Player(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::EnderDragon(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
			Entity::EndCrystal(x) => x.tick(dimension, players, packet_sender, entity_id_manager, block_state_data),
		};
	}
	pub fn damage(&mut self, damage: f32, packet_sender: &PacketSender, players: &[Player]) {
		return match self {
			Entity::Armadillo(x) => x.damage(damage, packet_sender, players),
			Entity::Cat(x) => x.damage(damage, packet_sender, players),
			Entity::ChestMinecart(x) => x.damage(damage, packet_sender, players),
			Entity::Chicken(x) => x.damage(damage, packet_sender, players),
			Entity::Cow(x) => x.damage(damage, packet_sender, players),
			Entity::Creeper(x) => x.damage(damage, packet_sender, players),
			Entity::Donkey(x) => x.damage(damage, packet_sender, players),
			Entity::Horse(x) => x.damage(damage, packet_sender, players),
			Entity::Item(x) => x.damage(damage, packet_sender, players),
			Entity::Parrot(x) => x.damage(damage, packet_sender, players),
			Entity::Pig(x) => x.damage(damage, packet_sender, players),
			Entity::Rabbit(x) => x.damage(damage, packet_sender, players),
			Entity::Sheep(x) => x.damage(damage, packet_sender, players),
			Entity::Player(x) => x.damage(damage, packet_sender, players),
			Entity::EnderDragon(x) => x.damage(damage, packet_sender, players),
			Entity::EndCrystal(x) => x.damage(damage, packet_sender, players),
		};
	}

	pub fn feed(&mut self, held_item: &Slot, packet_sender: &PacketSender, players_clone: &[Player], dimension_name: &str) -> bool {
		return match self {
			Entity::Armadillo(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::Cat(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::ChestMinecart(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::Chicken(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::Cow(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::Creeper(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::Donkey(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::Horse(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::Item(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::Parrot(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::Pig(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::Rabbit(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::Sheep(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::Player(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::EnderDragon(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
			Entity::EndCrystal(x) => x.feed(held_item, packet_sender, players_clone, dimension_name),
		};
	}

	pub fn set_age(&mut self, new_age: i32) {
		match self {
			Entity::Armadillo(x) => x.breedable_mob.age = new_age,
			Entity::Cat(x) => x.breedable_mob.age = new_age,
			Entity::Chicken(x) => x.breedable_mob.age = new_age,
			Entity::Cow(x) => x.breedable_mob.age = new_age,
			Entity::Donkey(x) => x.breedable_mob.age = new_age,
			Entity::Horse(x) => x.breedable_mob.age = new_age,
			Entity::Pig(x) => x.breedable_mob.age = new_age,
			Entity::Rabbit(x) => x.breedable_mob.age = new_age,
			Entity::Sheep(x) => x.breedable_mob.age = new_age,
			_ => println!("tried setting age on entity that doesnt support it: {self:?}"),
		};
	}

	pub fn resend_metadata_to_players(&self, players_clone: &[Player], packet_sender: &PacketSender, dimension_name: &str) {
		return match self {
			Entity::Armadillo(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::Cat(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::ChestMinecart(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::Chicken(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::Cow(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::Creeper(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::Donkey(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::Horse(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::Item(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::Parrot(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::Pig(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::Rabbit(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::Sheep(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::Player(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::EnderDragon(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
			Entity::EndCrystal(x) => x.resend_metadata_to_players(players_clone, packet_sender, dimension_name),
		};
	}

	pub fn change_dimension(
		&mut self,
		new_dimension_name: &str,
		players_clone: &[Player],
		dimension: &mut Dimension,
		packet_sender: &PacketSender,
		position: BlockPosition,
		block_states: &HashMap<String, basic_types::blocks::Block>,
	) {
		return match self {
			Entity::Armadillo(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::Cat(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::ChestMinecart(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::Chicken(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::Cow(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::Creeper(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::Donkey(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::Horse(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::Item(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::Parrot(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::Pig(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::Rabbit(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::Sheep(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::Player(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::EnderDragon(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
			Entity::EndCrystal(x) => x.change_dimension(new_dimension_name, players_clone, dimension, packet_sender, position, block_states),
		};
	}
}


pub fn new(entity_type: &str, common_data: CommonEntity, extra_nbt: NbtListTag) -> Option<Entity> {
	let new_entity = match entity_type {
		"minecraft:armadillo" => Some(Entity::Armadillo(Armadillo::new(common_data, extra_nbt))),
		"minecraft:cat" => Some(Entity::Cat(Cat::new(common_data, extra_nbt))),
		"minecraft:chest_minecart" => Some(Entity::ChestMinecart(ChestMinecart::new(common_data, extra_nbt))),
		"minecraft:chicken" => Some(Entity::Chicken(Chicken::new(common_data, extra_nbt))),
		"minecraft:cow" => Some(Entity::Cow(Cow::new(common_data, extra_nbt))),
		"minecraft:creeper" => Some(Entity::Creeper(Creeper::new(common_data, extra_nbt))),
		"minecraft:donkey" => Some(Entity::Donkey(Donkey::new(common_data, extra_nbt))),
		"minecraft:horse" => Some(Entity::Horse(Horse::new(common_data, extra_nbt))),
		"minecraft:item" => Some(Entity::Item(ItemEntity::new(common_data, extra_nbt))),
		"minecraft:parrot" => Some(Entity::Parrot(Parrot::new(common_data, extra_nbt))),
		"minecraft:pig" => Some(Entity::Pig(Pig::new(common_data, extra_nbt))),
		"minecraft:rabbit" => Some(Entity::Rabbit(Rabbit::new(common_data, extra_nbt))),
		"minecraft:sheep" => Some(Entity::Sheep(Sheep::new(common_data, extra_nbt))),
		"minecraft:ender_dragon" => Some(Entity::EnderDragon(EnderDragon::new(common_data, extra_nbt))),
		"minecraft:end_crystal" => Some(Entity::EndCrystal(EndCrystal::new(common_data, extra_nbt))),
		_ => None,
	};

	if let Some(mut new_entity) = new_entity {
		use crate::types::entity::*;
		let (height, width) = new_entity.get_hitbox();
		let cuboid = Cuboid {
			x1: -(width / 2.0),
			y1: 0.0,
			z1: -(width / 2.0),
			x2: width / 2.0,
			y2: height,
			z2: width / 2.0,
		};
		new_entity.get_common_entity_data_mut().collision_shape =
			CollisionShape::new_from_cuboid(cuboid, new_entity.get_common_entity_data().position);
		return Some(new_entity);
	}

	return new_entity;
}

pub fn create_and_spawn_entity_from_egg(
	spawn_egg_name: &str,
	entity_id: i32,
	position: BlockPosition,
	dimension: &mut Dimension,
	players: &[Player],
	packet_sender: &PacketSender,
) {
	let entity_type = spawn_egg_name.replace("_spawn_egg", "");
	let entity_position = EntityPosition {
		x: position.x as f64 + 0.5,
		y: position.y as f64,
		z: position.z as f64 + 0.5,
		yaw: 0.0,
		pitch: 0.0,
	};
	create_and_spawn_entity(&entity_type, entity_id, entity_position, dimension, players, packet_sender);
}

pub fn create_and_spawn_entity(
	entity_type: &str,
	entity_id: i32,
	position: EntityPosition,
	dimension: &mut Dimension,
	players: &[Player],
	packet_sender: &PacketSender,
) {
	let new_entity = entity::new(
		entity_type,
		CommonEntity {
			position,
			velocity: EntityPosition::default(),
			uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
			entity_id,
			..Default::default()
		},
		NbtListTag::TagCompound(Vec::new()),
	);

	if let Some(new_entity) = new_entity {
		let packet = new_entity.to_spawn_entity_packet();

		dimension.add_entity(new_entity);

		packet_sender.send_packet_to_everyone_in_dimension(
			players,
			&dimension.name,
			crate::packets::clientbound::play::SpawnEntity::PACKET_ID,
			packet,
		);
	};
}
