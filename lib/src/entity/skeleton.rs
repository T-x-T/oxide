use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Skeleton {
	pub common: CommonEntity,
	pub mob: CommonMob,
	pub damage_cooldown: u8,
}

impl CommonEntityTrait for Skeleton {
	fn new(data: CommonEntity, extra_nbt: NbtListTag) -> Self {
		let mob = CommonMob::from_nbt(extra_nbt.clone());

		return Self {
			common: data,
			mob,
			damage_cooldown: 80,
		};
	}

	fn to_nbt_extras(&self) -> Vec<NbtTag> {
		let mut output: Vec<NbtTag> = vec![];

		output.append(&mut self.mob.to_nbt());

		return output;
	}

	fn extra_tick(
		&mut self,
		_dimension: &Dimension,
		players: &[Player],
		_packet_sender: &PacketSender,
		entity_id_manager: &EntityIdManager,
		_block_state_data: &HashMap<String, basic_types::blocks::Block>,
	) -> Vec<EntityTickOutcome> {
		let mut output: Vec<EntityTickOutcome> = Vec::new();
		//TODO: light skeleton on fire at daytime

		if self.damage_cooldown > 0 {
			self.damage_cooldown -= 1;
		} else {
			let mut player_distances = players
				.iter()
				.map(|x| (x, self.get_common_entity_data().position.distance_to(x.get_position())))
				.filter(|x| x.1 < 25.0)
				.collect::<Vec<(&Player, f64)>>();

			player_distances.sort_by(|a, b| a.1.total_cmp(&b.1));
			let closest_player = player_distances.first();
			if let Some(closest_player) = closest_player {
				let direction = closest_player.0.get_position() - self.get_common_entity_data().position;
				let normalized_direction = direction / closest_player.0.get_position().distance_to(self.get_common_entity_data().position);
				let direction_with_speed = normalized_direction * 1.5;
				let direction_with_angle = EntityPosition {
					y: direction_with_speed.y + 0.45,
					..direction_with_speed
				};
				let arrow = entity::new(
					"minecraft:arrow",
					CommonEntity {
						position: EntityPosition {
							y: self.get_common_entity_data().position.y + 1.2,
							..self.get_common_entity_data().position
						},
						velocity: direction_with_angle,
						uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
						entity_id: entity_id_manager.get_new(),
						..Default::default()
					},
					NbtListTag::TagCompound(Vec::new()),
				);
				output.push(EntityTickOutcome::AddEntity(Box::new(arrow.unwrap())));
				self.damage_cooldown = 80;
			}
		}


		return output;
	}

	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:skeleton");
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
		return (1.95, 0.6);
	}

	fn get_metadata(&self) -> Vec<crate::packets::clientbound::play::EntityMetadata> {
		return Vec::new();
	}

	fn get_default_ai_behavior(&self) -> AiBehavior {
		return AiBehavior::MoveTowardsPlayerWithMinDistance(8.0);
	}
}
