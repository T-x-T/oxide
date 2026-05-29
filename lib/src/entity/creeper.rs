use crate::packets::Packet;

use super::*;

static EXPLOSION_TRIGGER_RADIUS: f64 = 4.0;

#[derive(Debug, PartialEq, Clone)]
pub struct Creeper {
	pub common: CommonEntity,
	pub mob: CommonMob,
	pub explosion_radius: u8,
	pub fuse: i16,
	pub is_ignited: bool,
	pub is_powered: bool,
}

impl CommonEntityTrait for Creeper {
	fn new(data: CommonEntity, extra_nbt: NbtListTag) -> Self {
		let mob = CommonMob::from_nbt(extra_nbt.clone());

		return Self {
			common: data,
			mob,
			explosion_radius: extra_nbt.get_child("ExplosionRadius").unwrap_or(&NbtTag::Byte(String::new(), 3)).as_byte(),
			fuse: extra_nbt.get_child("Fuse").unwrap_or(&NbtTag::Short(String::new(), 30)).as_short(),
			is_ignited: extra_nbt.get_child("Ignited").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1,
			is_powered: extra_nbt.get_child("powered").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1,
		};
	}

	fn to_nbt_extras(&self) -> Vec<NbtTag> {
		let mut output: Vec<NbtTag> = vec![
			NbtTag::Byte("ExplosionRadius".to_string(), self.explosion_radius),
			NbtTag::Short("Fuse".to_string(), self.fuse),
			NbtTag::Byte("Ignited".to_string(), if self.is_ignited { 1 } else { 0 }),
			NbtTag::Byte("powered".to_string(), if self.is_powered { 1 } else { 0 }),
		];

		output.append(&mut self.mob.to_nbt());

		return output;
	}

	fn extra_tick(
		&mut self,
		dimension: &Dimension,
		players: &[Player],
		packet_sender: &PacketSender,
		_entity_id_manager: &EntityIdManager,
		_block_state_data: &HashMap<String, basic_types::blocks::Block>,
	) -> Vec<EntityTickOutcome> {
		let are_players_nearby = players
			.iter()
			.filter(|x| x.get_gamemode() == Gamemode::Survival)
			.any(|x| x.get_position().distance_to(self.get_common_entity_data().position) <= EXPLOSION_TRIGGER_RADIUS);

		if are_players_nearby {
			return self.explode(dimension, packet_sender, players);
		}

		return Vec::new();
	}

	fn interact(
		&mut self,
		held_item: &Slot,
		dimension: &mut Dimension,
		players_clone: &[Player],
		_players: &mut [Player],
		_player_uuid: u128,
		packet_sender: &PacketSender,
		_entity_id_manager: &EntityIdManager,
		_block_state_data: &HashMap<String, basic_types::blocks::Block>,
	) -> Vec<EntityTickOutcome> {
		if held_item.count > 0 && held_item.id == data::items::get_item_id_by_name("minecraft:flint_and_steel").unwrap() {
			//right clicked a creeper with flint and steel -> explode!
			return self.explode(dimension, packet_sender, players_clone);
		}

		return Vec::new();
	}

	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:creeper");
	}

	fn get_metadata(&self) -> Vec<EntityMetadata> {
		return Vec::new();
	}

	fn get_common_entity_data(&self) -> &CommonEntity {
		return &self.common;
	}

	fn get_common_entity_data_mut(&mut self) -> &mut CommonEntity {
		return &mut self.common;
	}

	fn set_common_entity_data(&mut self, common_entity_data: CommonEntity) {
		self.common = common_entity_data;
	}

	fn is_mob(&self) -> bool {
		return true;
	}

	fn get_mob_data(&self) -> &CommonMob {
		return &self.mob;
	}

	fn get_mob_data_mut(&mut self) -> &mut CommonMob {
		return &mut self.mob;
	}

	fn set_mob_data(&mut self, common_mob_data: CommonMob) {
		self.mob = common_mob_data;
	}

	//(height, width) https://minecraft.wiki/w/Hitbox
	fn get_hitbox(&self) -> (f64, f64) {
		return (1.7, 0.6);
	}
}

impl Creeper {
	fn explode(&mut self, dimension: &Dimension, packet_sender: &PacketSender, players_clone: &[Player]) -> Vec<EntityTickOutcome> {
		let mut output: Vec<EntityTickOutcome> = Vec::new();

		self.get_mob_data_mut().health = 0.0;

		let explosion_packet = crate::packets::clientbound::play::Explosion {
			x: self.get_common_entity_data().position.x,
			y: self.get_common_entity_data().position.y,
			z: self.get_common_entity_data().position.z,
			radius: 2.0,
			block_count: 64,
			player_delta_velocity: None,
			particle_id: 23,
			particle_data: (),
			sound: 616,
		};

		let creeper_position = BlockPosition::from(self.get_common_entity_data().position);
		for x in (creeper_position.x - 2)..creeper_position.x + 2 {
			for y in (creeper_position.y - 2)..creeper_position.y + 2 {
				for z in (creeper_position.z - 2)..creeper_position.z + 2 {
					output.push(EntityTickOutcome::ReplaceBlock(
						BlockPosition {
							x,
							y,
							z,
						},
						0,
					));
				}
			}
		}

		packet_sender.send_packet_to_everyone_in_dimension(
			players_clone,
			&dimension.name,
			crate::packets::clientbound::play::Explosion::PACKET_ID,
			explosion_packet,
		);

		return output;
	}
}
