use std::sync::Arc;

use crate::packets::Packet;

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Cow {
	pub common: CommonEntity,
	pub mob: CommonMob,
	pub breedable_mob: BreedableMob,
}

const FOOD: &str = "minecraft:wheat";

impl CommonEntityTrait for Cow {
	fn new(data: CommonEntity, extra_nbt: NbtListTag) -> Self {
		let mob = CommonMob::from_nbt(extra_nbt.clone());
		let breedable_mob = BreedableMob::from_nbt(extra_nbt);

		return Self {
			common: data,
			mob,
			breedable_mob,
		};
	}

	fn to_nbt_extras(&self) -> Vec<NbtTag> {
		return vec![self.mob.to_nbt(), self.breedable_mob.to_nbt()].into_iter().flatten().collect();
	}

	fn feed(&mut self, held_item: &Slot, game: Arc<Game>, players_clone: &[Player]) -> bool {
		if held_item.id == data::items::get_item_id_by_name(FOOD) {
			self.breedable_mob.in_love = 30 * 20;

			for player in players_clone {
				game.send_packet(
					&player.peer_socket_address,
					crate::packets::clientbound::play::Particle::PACKET_ID,
					crate::packets::clientbound::play::Particle {
						long_distance: false,
						always_visible: false,
						x: self.get_common_entity_data().position.x,
						y: self.get_common_entity_data().position.y + 1.0,
						z: self.get_common_entity_data().position.z,
						offset_x: 0.2,
						offset_y: 0.2,
						offset_z: 0.2,
						max_speed: 1.0,
						particle_count: 8,
						particle_id: 45,
						particle_data: (),
					}
					.try_into()
					.unwrap(),
				);
			}

			return true;
		} else {
			return false;
		}
	}

	fn extra_tick(&mut self, dimension: &Dimension, players: &[Player], game: std::sync::Arc<Game>) -> Vec<EntityTickOutcome> {
		let mut output: Vec<EntityTickOutcome> = Vec::new();

		let in_range_cows_in_love: Vec<&Cow> = if self.breedable_mob.breeding_with.is_some() {
			let breeding_with =
				dimension.entities.iter().find(|x| x.get_common_entity_data().entity_id == self.breedable_mob.breeding_with.unwrap());

			if let Some(breeding_with) = breeding_with {
				vec![match breeding_with {
					Entity::Cow(cow) => cow,
					_ => panic!("cow breeding with entity that is not a cow"),
				}]
			} else {
				vec![]
			}
		} else if self.breedable_mob.in_love > 0 {
			dimension
				.entities
				.iter()
				.filter(|x| x.get_common_entity_data().entity_id != self.get_common_entity_data().entity_id)
				.filter_map(|x| match x {
					Entity::Cow(cow) => Some(cow),
					_ => None,
				})
				.filter(|x| x.breedable_mob.in_love > 0)
				.filter(|x| {
					x.get_common_entity_data().position.distance_to(self.get_common_entity_data().position) <= crate::MOB_BREEDING_ATTRACTION_RADIUS
				})
				.collect()
		} else {
			vec![]
		};

		let in_range_players_with_food: Vec<&Player> = players
			.iter()
			.filter(|x| x.get_position().distance_to(self.get_common_entity_data().position) <= crate::MOB_FOOD_ATTRACTION_RADIUS)
			.filter(|x| x.get_held_item(true).is_some_and(|item| item.id == data::items::get_item_id_by_name(FOOD)))
			.collect();

		let speed = EntityPosition {
			x: 0.025,
			y: 0.001,
			z: 0.025,
			yaw: 1.0,
			pitch: 1.0,
		};

		if !in_range_cows_in_love.is_empty() {
			if self.breedable_mob.breeding_time_left == 0 && self.breedable_mob.breeding_with.is_some() {
				self.breedable_mob.breeding_with = None;
				self.breedable_mob.in_love = 0;
				output.push(EntityTickOutcome::DoneBreeding(
					self.get_common_entity_data().entity_id,
					in_range_cows_in_love.first().unwrap().get_common_entity_data().entity_id,
				));
			} else {
				if self.breedable_mob.breeding_with.is_none() {
					self.breedable_mob.breeding_with = Some(in_range_cows_in_love.first().unwrap().get_common_entity_data().entity_id);
					self.breedable_mob.breeding_time_left = 5 * 20;
				} else {
					self.breedable_mob.breeding_time_left -= 1;
				}
			}
			self.get_common_entity_data_mut().velocity =
				(in_range_cows_in_love.first().unwrap().get_common_entity_data().position - self.get_common_entity_data().position) * speed;
			output.push(EntityTickOutcome::Updated);
		} else if !in_range_players_with_food.is_empty() {
			self.get_common_entity_data_mut().velocity =
				(in_range_players_with_food.first().unwrap().get_position() - self.get_common_entity_data().position) * speed;
			output.push(EntityTickOutcome::Updated);
		}

		if self.breedable_mob.in_love > 0 {
			self.breedable_mob.in_love -= 1;
		}

		if self.breedable_mob.age < -1 && !self.breedable_mob.age_locked {
			self.breedable_mob.age += 1;
		} else if self.breedable_mob.age == -1 && !self.breedable_mob.age_locked {
			self.breedable_mob.age = 0;

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

		return output;
	}

	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:cow");
	}

	fn get_metadata(&self) -> Vec<EntityMetadata> {
		if self.breedable_mob.age < 0 {
			vec![EntityMetadata {
				index: 16,
				value: EntityMetadataValue::Boolean(true),
			}]
		} else {
			vec![EntityMetadata {
				index: 16,
				value: EntityMetadataValue::Boolean(false),
			}]
		}
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
		return (1.4, 0.9);
	}
}
