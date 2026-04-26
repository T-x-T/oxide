use rand::{Rng, rng};

use crate::packets::Packet;

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Sheep {
	pub common: CommonEntity,
	pub mob: CommonMob,
	pub breedable_mob: BreedableMob,
	pub color: u8,
	pub sheared: bool,
}

const FOOD: &[&str] = &["minecraft:wheat"];

impl CommonEntityTrait for Sheep {
	fn new(data: CommonEntity, extra_nbt: NbtListTag) -> Self {
		let mob = CommonMob::from_nbt(extra_nbt.clone());
		let breedable_mob = BreedableMob::from_nbt(extra_nbt.clone());

		return Self {
			common: data,
			mob,
			breedable_mob,
			color: extra_nbt.get_child("Color").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte(),
			sheared: extra_nbt.get_child("Sheared").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1,
		};
	}

	fn interact(
		&mut self,
		held_item: &Slot,
		game: Arc<Game>,
		_dimension: &mut Dimension,
		players_clone: &[Player],
		_players: &mut [Player],
		player_uuid: u128,
	) -> EntityInteractResult {
		if held_item.count > 0 && held_item.id == data::items::get_item_id_by_name("minecraft:shears") && !self.sheared {
			self.sheared = true;

			let mut rng = rng();
			let wool_drops = rng.random_range(1..=3);
			let wool_slot = Slot {
				count: wool_drops,
				id: data::items::get_item_id_by_name("minecraft:white_wool"),
				components_to_add: Vec::new(),
				components_to_remove: Vec::new(),
			};

			let item_entity = ItemEntity {
				common: CommonEntity {
					position: self.common.position,
					velocity: EntityPosition {
						x: 0.0,
						y: 1.0,
						z: 0.0,
						yaw: 0.0,
						pitch: 0.0,
					},
					uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
					entity_id: game.entity_id_manager.get_new(),
					..Default::default()
				},
				age: 0,
				health: 5,
				item: wool_slot,
				owner: player_uuid,
				pickup_delay: 0,
				thrower: player_uuid,
			};

			let metadata_packet = crate::packets::clientbound::play::SetEntityMetadata {
				entity_id: self.get_common_entity_data().entity_id,
				metadata: self.get_metadata(),
			};

			for player in players_clone {
				game.send_packet(
					&player.peer_socket_address,
					crate::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
					metadata_packet.clone().try_into().unwrap(),
				);
			}

			return EntityInteractResult::AddEntity(Box::new(Entity::Item(item_entity)));
		} else {
			return EntityInteractResult::DoNothing;
		}
	}

	fn to_nbt_extras(&self) -> Vec<NbtTag> {
		return vec![
			self.mob.to_nbt(),
			self.breedable_mob.to_nbt(),
			vec![NbtTag::Byte("Color".to_string(), self.color), NbtTag::Byte("Sheared".to_string(), if self.sheared { 1 } else { 0 })],
		]
		.into_iter()
		.flatten()
		.collect();
	}

	fn feed(&mut self, held_item: &Slot, game: Arc<Game>, players_clone: &[Player]) -> bool {
		return self.feed_breedable_mob(held_item, game, players_clone);
	}

	fn extra_tick(&mut self, dimension: &Dimension, players: &[Player], game: std::sync::Arc<Game>) -> Vec<EntityTickOutcome> {
		let mut output: Vec<EntityTickOutcome> = Vec::new();
		if self.sheared {
			let mut rng = rng();

			if rng.random_ratio(1, 1000) {
				let position = BlockPosition {
					y: self.get_common_entity_data().position.y as i16 - 1,
					..self.get_common_entity_data().position.into()
				};
				let block = dimension.get_block(position).unwrap_or(0);
				let grass_block = data::blocks::get_block_from_name("minecraft:grass_block", &game.block_state_data);
				if block == grass_block.states[grass_block.default_state].id {
					let dirt_block = data::blocks::get_block_from_name("minecraft:dirt", &game.block_state_data);
					output.push(EntityTickOutcome::ReplaceBlock(position, dirt_block.states[dirt_block.default_state].id));

					self.sheared = false;

					let metadata_packet = crate::packets::clientbound::play::SetEntityMetadata {
						entity_id: self.get_common_entity_data().entity_id,
						metadata: self.get_metadata(),
					};

					for player in players {
						game.send_packet(
							&player.peer_socket_address,
							crate::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
							metadata_packet.clone().try_into().unwrap(),
						);
					}
				}
			}
		}

		output.append(&mut self.tick_breedable_mob(dimension, players, game));
		return output;
	}

	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:sheep");
	}

	fn get_metadata(&self) -> Vec<EntityMetadata> {
		let mut output: Vec<EntityMetadata> = Vec::new();

		if self.breedable_mob.age < 0 {
			output.push(EntityMetadata {
				index: 16,
				value: EntityMetadataValue::Boolean(true),
			})
		} else {
			output.push(EntityMetadata {
				index: 16,
				value: EntityMetadataValue::Boolean(false),
			})
		}

		let mut value: u8 = 0;
		value += self.color & 0x0F;
		if self.sheared {
			value += 0x10;
		}
		output.push(EntityMetadata {
			index: 17,
			value: EntityMetadataValue::Byte(value),
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
		return (1.3, 0.9);
	}
}

impl BreedableMobTrait for Sheep {
	fn get_breedable_data(&self) -> &BreedableMob {
		return &self.breedable_mob;
	}

	fn get_breedable_data_mut(&mut self) -> &mut BreedableMob {
		return &mut self.breedable_mob;
	}

	fn get_food(&self) -> &[&'static str] {
		return FOOD;
	}
}
