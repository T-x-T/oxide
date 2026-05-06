use crate::packets::Packet;

use super::*;

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

	fn interact(
		&mut self,
		held_item: &Slot,
		game: Arc<Game>,
		dimension: &mut Dimension,
		players_clone: &[Player],
		players: &mut [Player],
		_player_uuid: u128,
	) -> EntityInteractResult {
		if held_item.count > 0 && held_item.id == data::items::get_item_id_by_name("minecraft:flint_and_steel").unwrap() {
			//right clicked a creeper with flint and steel -> explode!
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
						let res = dimension
							.overwrite_block(
								BlockPosition {
									x,
									y,
									z,
								},
								0,
							)
							.unwrap();
						if let Some(BlockOverwriteOutcome::DestroyBlockentity) = res {
							let block_entity = dimension
								.get_chunk_from_position(BlockPosition {
									x,
									y,
									z,
								})
								.unwrap()
								.block_entities
								.iter()
								.find(|a| {
									a.get_position()
										== BlockPosition {
											x,
											y,
											z,
										}
								})
								.unwrap();
							let block_entity = block_entity.clone(); //So we get rid of the immutable borrow, so we can borrow world mutably again
							block_entity.remove_self(&game.entity_id_manager, players, dimension, game.clone());
						}

						for player in players_clone {
							game.send_packet(
								&player.peer_socket_address,
								crate::packets::clientbound::play::BlockUpdate::PACKET_ID,
								crate::packets::clientbound::play::BlockUpdate {
									location: BlockPosition {
										x,
										y,
										z,
									},
									block_id: 0,
								}
								.try_into()
								.unwrap(),
							);
						}
					}
				}
			}

			players_clone.iter().for_each(|x| {
				game.send_packet(
					&x.peer_socket_address,
					crate::packets::clientbound::play::Explosion::PACKET_ID,
					explosion_packet.clone().try_into().unwrap(),
				);
			});
		}

		return EntityInteractResult::DoNothing;
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
