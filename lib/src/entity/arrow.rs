use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Arrow {
	pub common: CommonEntity,
	pub left_owner: bool,
	pub owner: u128,
	pub in_ground: bool,
	pub life: i16,
}

impl CommonEntityTrait for Arrow {
	fn new(data: CommonEntity, extra_nbt: NbtListTag) -> Self {
		return Self {
			common: data,
			life: extra_nbt.get_child("life").unwrap_or(&NbtTag::Short(String::new(), 0)).as_short(),
			owner: extra_nbt
				.get_child("Owner")
				.unwrap_or(&NbtTag::IntArray(String::new(), vec![0; 4]))
				.as_int_array()
				.into_iter()
				.enumerate()
				.map(|x| (x.1 as u128) << (32 * (3 - x.0)))
				.reduce(|a, b| a | b)
				.unwrap(),
			left_owner: extra_nbt.get_child("LeftOwner").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1,
			in_ground: extra_nbt.get_child("InGround").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1,
		};
	}

	fn to_nbt_extras(&self) -> Vec<NbtTag> {
		return vec![
			NbtTag::Short("Life".to_string(), self.life),
			NbtTag::IntArray(
				"Owner".to_string(),
				vec![(self.owner >> 96) as i32, (self.owner << 32 >> 96) as i32, (self.owner << 64 >> 96) as i32, (self.owner << 96 >> 96) as i32],
			),
			NbtTag::Byte("LeftOwner".to_string(), self.left_owner.into()),
			NbtTag::Byte("InGround".to_string(), self.in_ground.into()),
		];
	}

	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:arrow");
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

	//(height, width) https://minecraft.wiki/w/Hitbox
	fn get_hitbox(&self) -> (f64, f64) {
		return (1.95, 0.6);
	}

	fn get_metadata(&self) -> Vec<crate::packets::clientbound::play::EntityMetadata> {
		return Vec::new();
	}

	fn get_default_ai_behavior(&self) -> AiBehavior {
		return AiBehavior::Idle;
	}

	#[allow(clippy::inspect_for_each)]
	fn extra_tick(
		&mut self,
		dimension: &Dimension,
		players: &[Player],
		_packet_sender: &PacketSender,
		_entity_id_manager: &EntityIdManager,
		block_state_data: &HashMap<String, basic_types::blocks::Block>,
	) -> Vec<EntityTickOutcome> {
		let mut output: Vec<EntityTickOutcome> = vec![EntityTickOutcome::Updated];

		//Despawn arrow thats too long stuck in ground
		if self.in_ground {
			self.life += 1;
			if self.life > 1200 {
				output.push(EntityTickOutcome::RemoveSelf);
			}
		}

		//get picked up?
		if self.in_ground {
			let picking_up_player = players
				.iter()
				.map(|x| (x, self.get_common_entity_data().position.distance_to(x.get_position())))
				.find(|x| x.1 < crate::ITEM_PICKUP_DISTANCE);

			if let Some((picking_up_player, _)) = picking_up_player {
				output.push(EntityTickOutcome::GetPickedUpByPlayer(
					Slot {
						count: 1,
						id: data::items::get_item_id_by_name("minecraft:arrow").unwrap(),
						components_to_add: Vec::new(),
						components_to_remove: Vec::new(),
					},
					self.get_common_entity_data().entity_id,
					picking_up_player.uuid,
				));

				return output;
			}
		}

		//check if arrow left owners collision box, if not we dont have to check for collisions with other entities to deal damage
		if !self.left_owner {
			let mut owner_collision_shape: Option<CollisionShape> = None;
			let player_owner = players.iter().find(|x| x.uuid == self.owner);
			if let Some(player_owner) = player_owner {
				owner_collision_shape = Some(player_owner.get_common_entity_data_cloned().collision_shape);
			} else {
				let entity_owner = dimension.entities.iter().find(|x| x.get_common_entity_data().uuid == self.owner);
				if let Some(entity_owner) = entity_owner {
					owner_collision_shape = Some(entity_owner.get_common_entity_data().collision_shape.clone());
				}
			}

			if let Some(owner_collision_shape) = owner_collision_shape {
				if !owner_collision_shape.collides_with(&self.get_common_entity_data().collision_shape) {
					self.left_owner = true;
				}
			} else {
				self.left_owner = true;
			}
		}

		//if the arrow left the owners collision box, check if collided with other entities, players or the ground
		if self.left_owner {
			if self.collides_with_blocks_at(dimension, self.get_common_entity_data().position, block_state_data) {
				self.in_ground = true;
			} else {
				self.in_ground = false;
				let mut hit_something = false;
				players
					.iter()
					.map(|x| (x, self.get_common_entity_data().position.distance_to(x.get_position())))
					.filter(|x| x.1 < 2.0)
					.map(|x| x.0)
					.filter(|x| x.get_common_entity_data_cloned().collision_shape.collides_with(&self.get_common_entity_data().collision_shape))
					.inspect(|_| hit_something = true)
					.for_each(|x| output.push(EntityTickOutcome::DealDamage(x.entity_id, 5.0)));

				dimension
					.entities
					.iter()
					.filter(|x| x.get_common_entity_data().entity_id != self.get_common_entity_data().entity_id)
					.map(|x| (x, self.get_common_entity_data().position.distance_to(x.get_common_entity_data().position)))
					.filter(|x| x.1 < 10.0)
					.map(|x| x.0)
					.filter(|x| x.get_common_entity_data().collision_shape.collides_with(&self.get_common_entity_data().collision_shape))
					.inspect(|_| hit_something = true)
					.for_each(|x| output.push(EntityTickOutcome::DealDamage(x.get_common_entity_data().entity_id, 5.0)));

				if hit_something {
					output.push(EntityTickOutcome::RemoveSelf);
				}
			}
		}

		return output;
	}
}
