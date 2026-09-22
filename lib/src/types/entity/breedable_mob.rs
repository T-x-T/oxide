use super::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BreedableMob {
	pub age: i32,
	pub age_locked: bool,
	pub forced_age: i32,
	pub in_love: i32,
	pub love_cause: u128,
	pub breeding_with: Option<i32>,
	pub breeding_time_left: i32,
}


impl BreedableMob {
	pub fn from_nbt(data: NbtListTag) -> BreedableMob {
		let mut output = BreedableMob::default();

		if let Some(value) = data.get_child("Age") {
			output.age = value.as_int();
		}
		if let Some(value) = data.get_child("AgeLocked") {
			output.age_locked = value.as_byte() == 1;
		}
		if let Some(value) = data.get_child("ForcedAge") {
			output.forced_age = value.as_int();
		}
		if let Some(value) = data.get_child("InLove") {
			output.in_love = value.as_int();
		}
		if let Some(value) = data.get_child("LoveCause") {
			output.love_cause =
				value.as_int_array().into_iter().enumerate().map(|x| (x.1 as u128) << (32 * (3 - x.0))).reduce(|a, b| a | b).unwrap();
		}

		return output;
	}

	pub fn to_nbt(&self) -> Vec<NbtTag> {
		return vec![
			NbtTag::Int("Age".to_string(), self.age),
			NbtTag::Byte("AgeLocked".to_string(), if self.age_locked { 1 } else { 0 }),
			NbtTag::Int("ForcedAge".to_string(), self.forced_age),
			NbtTag::Int("InLove".to_string(), self.in_love),
			NbtTag::IntArray(
				"LoveCause".to_string(),
				vec![
					(self.love_cause >> 96) as i32,
					(self.love_cause << 32 >> 96) as i32,
					(self.love_cause << 64 >> 96) as i32,
					(self.love_cause << 96 >> 96) as i32,
				],
			),
		];
	}
}

pub trait BreedableMobTrait: CommonEntityTrait {
	fn get_breedable_data(&self) -> &BreedableMob;
	fn get_breedable_data_mut(&mut self) -> &mut BreedableMob;
	fn get_food(&self) -> &[&'static str];
	fn feed_breedable_mob(&mut self, held_item: &Slot, packet_sender: &PacketSender, players_clone: &[Player], dimension_name: &str) -> bool {
		if self.get_breedable_data().age != 0 {
			return false;
		}

		if !self.get_food().contains(&data::items::get_item_name_by_id(held_item.id).unwrap()) {
			return false;
		}

		self.get_breedable_data_mut().in_love = 30 * 20;

		packet_sender.send_packet_to_everyone_in_dimension(
			players_clone,
			dimension_name,
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
			},
		);

		return true;
	}

	fn tick_breedable_mob(
		&mut self,
		dimension: &Dimension,
		players: &[Player],
		packet_sender: &PacketSender,
		_entity_id_manager: &EntityIdManager,
		block_state_data: &HashMap<String, basic_types::blocks::Block>,
	) -> Vec<EntityTickOutcome> {
		let mut output: Vec<EntityTickOutcome> = Vec::new();

		let in_range_peers_in_love: Vec<Box<&dyn BreedableMobTrait>> = if self.get_breedable_data().breeding_with.is_some() {
			let breeding_with =
				dimension.entities.iter().find(|x| x.get_common_entity_data().entity_id == self.get_breedable_data().breeding_with.unwrap());

			if let Some(breeding_with) = breeding_with {
				vec![match breeding_with {
					Entity::Armadillo(x) => Box::new(x),
					Entity::Cat(x) => Box::new(x),
					Entity::Chicken(x) => Box::new(x),
					Entity::Cow(x) => Box::new(x),
					Entity::Donkey(x) => Box::new(x),
					Entity::Horse(x) => Box::new(x),
					Entity::Pig(x) => Box::new(x),
					Entity::Rabbit(x) => Box::new(x),
					Entity::Sheep(x) => Box::new(x),
					_ => panic!("tick_breedable_mob called on a mob that cannot be bred"),
				}]
			} else {
				vec![]
			}
		} else if self.get_breedable_data().in_love > 0 {
			dimension
				.entities
				.iter()
				.filter(|x| x.get_common_entity_data().entity_id != self.get_common_entity_data().entity_id)
				.filter_map(|x| {
					let res: Option<Box<&dyn BreedableMobTrait>> = match x {
						Entity::Armadillo(x) => Some(Box::new(x)),
						Entity::Cat(x) => Some(Box::new(x)),
						Entity::Chicken(x) => Some(Box::new(x)),
						Entity::Cow(x) => Some(Box::new(x)),
						Entity::Donkey(x) => Some(Box::new(x)),
						Entity::Horse(x) => Some(Box::new(x)),
						Entity::Pig(x) => Some(Box::new(x)),
						Entity::Rabbit(x) => Some(Box::new(x)),
						Entity::Sheep(x) => Some(Box::new(x)),
						_ => None,
					};
					return res;
				})
				.filter(|x| x.get_breedable_data().in_love > 0)
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
			.filter(|x| x.get_held_item(true).is_some_and(|item| self.get_food().contains(&data::items::get_item_name_by_id(item.id).unwrap())))
			.collect();


		if !in_range_peers_in_love.is_empty() {
			if self.get_breedable_data().breeding_time_left == 0 && self.get_breedable_data().breeding_with.is_some() {
				self.get_breedable_data_mut().breeding_with = None;
				self.get_breedable_data_mut().in_love = 0;
				self.get_breedable_data_mut().age = crate::MOB_BREEDING_DELAY_AFTER_OFFSPRING_PRODUCED_TICKS;
				output.push(EntityTickOutcome::DoneBreeding(
					self.get_common_entity_data().entity_id,
					in_range_peers_in_love.first().unwrap().get_common_entity_data().entity_id,
				))
			} else {
				if self.get_breedable_data().breeding_with.is_none() {
					self.get_breedable_data_mut().breeding_with = Some(in_range_peers_in_love.first().unwrap().get_common_entity_data().entity_id);
					self.get_breedable_data_mut().breeding_time_left = crate::MOB_TIME_TO_PRODUCE_BABY_TICKS;
				} else {
					self.get_breedable_data_mut().breeding_time_left -= 1;
				}
			}
			let velocity_from_ai =
				self.ai_move_towards_goal(in_range_peers_in_love.first().unwrap().get_common_entity_data().position, dimension, block_state_data).0;
			self.get_common_entity_data_mut().velocity += velocity_from_ai * 0.1;

			output.push(EntityTickOutcome::Updated);
		} else if !in_range_players_with_food.is_empty() {
			let (velocity, mut tick_outcomes) =
				self.ai_move_towards_goal(in_range_players_with_food.first().unwrap().get_position(), dimension, block_state_data);
			self.get_common_entity_data_mut().velocity += velocity * 0.1;
			output.append(&mut tick_outcomes);
			output.push(EntityTickOutcome::Updated);
		}

		if self.get_breedable_data().in_love > 0 {
			self.get_breedable_data_mut().in_love -= 1;
		}

		if self.get_breedable_data().age < -1 && !self.get_breedable_data().age_locked {
			self.get_breedable_data_mut().age += 1;
		} else if self.get_breedable_data().age == -1 && !self.get_breedable_data().age_locked {
			self.get_breedable_data_mut().age = 0;

			self.resend_metadata_to_players(players, packet_sender, &dimension.name);
		} else if self.get_breedable_data().age > 0 {
			self.get_breedable_data_mut().age -= 1;
		}

		return output;
	}
}
