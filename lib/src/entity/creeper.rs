use crate::packets::Packet;

use super::*;

static EXPLOSION_TRIGGER_RADIUS: f64 = 4.0;
static STARTING_FUSE: i16 = 30;

#[derive(Debug, PartialEq, Clone)]
pub struct Creeper {
	pub common: CommonEntity,
	pub mob: CommonMob,
	pub explosion_radius: u8,
	pub fuse: i16,
	pub is_ignited: bool,
	pub is_powered: bool,
	pub is_manually_lit: bool,
}

impl CommonEntityTrait for Creeper {
	fn new(data: CommonEntity, extra_nbt: NbtListTag) -> Self {
		let mob = CommonMob::from_nbt(extra_nbt.clone());

		return Self {
			common: data,
			mob,
			explosion_radius: extra_nbt.get_child("ExplosionRadius").unwrap_or(&NbtTag::Byte(String::new(), 3)).as_byte(),
			fuse: extra_nbt.get_child("Fuse").unwrap_or(&NbtTag::Short(String::new(), STARTING_FUSE)).as_short(),
			is_ignited: extra_nbt.get_child("Ignited").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1,
			is_powered: extra_nbt.get_child("powered").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1,
			is_manually_lit: false,
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

		self.mob.has_no_ai = self.is_ignited;

		if self.fuse == 0 {
			return self.explode(dimension, packet_sender, players);
		} else if self.is_ignited {
			self.fuse -= 1;
		}


		if !self.is_manually_lit {
			if are_players_nearby {
				if !self.is_ignited {
					self.fuse = STARTING_FUSE;
					self.is_ignited = true;
					self.resend_metadata_to_players(players, packet_sender, &dimension.name);
				}
			} else {
				self.fuse = STARTING_FUSE;
				self.is_ignited = false;
				self.resend_metadata_to_players(players, packet_sender, &dimension.name);
			}
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
		if self.is_ignited {
			return Vec::new();
		}
		if held_item.count > 0 && held_item.id == data::items::get_item_id_by_name("minecraft:flint_and_steel").unwrap() {
			self.fuse = STARTING_FUSE;
			self.is_ignited = true;
			self.is_manually_lit = true;
			self.resend_metadata_to_players(players_clone, packet_sender, &dimension.name);
		}

		return Vec::new();
	}

	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:creeper");
	}

	fn get_metadata(&self) -> Vec<EntityMetadata> {
		let mut output: Vec<EntityMetadata> = Vec::new();

		if self.is_ignited {
			output.push(EntityMetadata {
				index: 16,
				value: EntityMetadataValue::Varint(1),
			});
			output.push(EntityMetadata {
				index: 18,
				value: EntityMetadataValue::Boolean(true),
			});
		} else {
			output.push(EntityMetadata {
				index: 16,
				value: EntityMetadataValue::Varint(-1),
			});
			output.push(EntityMetadata {
				index: 18,
				value: EntityMetadataValue::Boolean(false),
			});
		}

		output.push(EntityMetadata {
			index: 17,
			value: EntityMetadataValue::Boolean(self.is_powered),
		});

		return output;
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
		self.get_mob_data_mut().drop_items_upon_death = false;

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

		dimension
			.entities
			.iter()
			.filter(|x| x.get_common_entity_data().position.distance_to(self.get_common_entity_data().position) < 2.5)
			.for_each(|x| output.push(EntityTickOutcome::DealDamage(x.get_common_entity_data().entity_id, 20.0)));

		players_clone
			.iter()
			.filter(|x| x.get_gamemode() == Gamemode::Survival || x.get_gamemode() == Gamemode::Adventure)
			.filter(|x| x.get_position().distance_to(self.get_common_entity_data().position) < 2.5)
			.for_each(|x| output.push(EntityTickOutcome::DealDamage(x.entity_id, 20.0)));

		packet_sender.send_packet_to_everyone_in_dimension(
			players_clone,
			&dimension.name,
			crate::packets::clientbound::play::Explosion::PACKET_ID,
			explosion_packet,
		);

		return output;
	}
}
