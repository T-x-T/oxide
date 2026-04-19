use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Donkey {
	pub common: CommonEntity,
	pub mob: CommonMob,
	pub breedable_mob: BreedableMob,
}

const FOOD: &[&str] = &["minecraft:golden_apple", "minecraft:enchanted_golden_apple", "minecraft:golden_carrot"];

impl CommonEntityTrait for Donkey {
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
		return self.feed_breedable_mob(held_item, game, players_clone);
	}

	fn extra_tick(&mut self, dimension: &Dimension, players: &[Player], game: std::sync::Arc<Game>) -> Vec<EntityTickOutcome> {
		return self.tick_breedable_mob(dimension, players, game);
	}

	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:donkey");
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
		return (1.5, 0.3965);
	}
}

impl BreedableMobTrait for Donkey {
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
