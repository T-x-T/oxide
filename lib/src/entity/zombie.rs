use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Zombie {
	pub common: CommonEntity,
	pub mob: CommonMob,
	pub damage_cooldown: u8,
}

impl CommonEntityTrait for Zombie {
	fn new(data: CommonEntity, extra_nbt: NbtListTag) -> Self {
		let mob = CommonMob::from_nbt(extra_nbt.clone());

		return Self {
			common: data,
			mob,
			damage_cooldown: 0,
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
		_entity_id_manager: &EntityIdManager,
		_block_state_data: &HashMap<String, basic_types::blocks::Block>,
	) -> Vec<EntityTickOutcome> {
		let mut output: Vec<EntityTickOutcome> = Vec::new();
		//TODO: light zombie on fire at daytime

		if self.damage_cooldown > 0 {
			self.damage_cooldown -= 1;
		} else {
			let mut player_distances = players
				.iter()
				.map(|x| (x, self.get_common_entity_data().position.distance_to(x.get_position())))
				.filter(|x| x.1 < 1.0)
				.collect::<Vec<(&Player, f64)>>();

			player_distances.sort_by(|a, b| a.1.total_cmp(&b.1));
			let closest_player = player_distances.first();
			if let Some(closest_player) = closest_player {
				output.push(EntityTickOutcome::DealDamage(closest_player.0.entity_id, 5.0));
				self.damage_cooldown = 20;
			}
		}


		return output;
	}

	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:zombie");
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
		return AiBehavior::MoveTowardsPlayer;
	}
}
