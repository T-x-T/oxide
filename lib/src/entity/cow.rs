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

	fn extra_tick(&mut self, _dimension: &Dimension, players: &[Player], _game: std::sync::Arc<Game>) -> Vec<EntityTickOutcome> {
		let mut output: Vec<EntityTickOutcome> = Vec::new();

		let in_range_players_with_food: Vec<&Player> = players
			.iter()
			.filter(|x| x.get_position().distance_to(self.get_common_entity_data().position) <= crate::MOB_FOOD_ATTRACTION_RADIUS)
			.filter(|x| x.get_held_item(true).is_some_and(|item| item.id == data::items::get_item_id_by_name(FOOD)))
			.collect();

		if !in_range_players_with_food.is_empty() {
			let speed = EntityPosition {
				x: 0.025,
				y: 0.001,
				z: 0.025,
				yaw: 1.0,
				pitch: 1.0,
			};
			self.get_common_entity_data_mut().velocity =
				(in_range_players_with_food.first().unwrap().get_position() - self.get_common_entity_data().position) * speed;
			output.push(EntityTickOutcome::Updated);
		}

		return output;
	}

	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:cow");
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
		return (1.4, 0.9);
	}
}
