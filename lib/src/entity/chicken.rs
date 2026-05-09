use rand::{Rng, rng};

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Chicken {
	pub common: CommonEntity,
	pub mob: CommonMob,
	pub breedable_mob: BreedableMob,
}

const FOOD: &[&str] = &[
	"minecraft:wheat_seeds",
	"minecraft:pumpkin_seeds",
	"minecraft:melon_seeds",
	"minecraft:beetroot_seeds",
	"minecraft:torchflower_seeds",
	"minecraft:pitcher_pod",
];

impl CommonEntityTrait for Chicken {
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

	fn feed(&mut self, held_item: &Slot, game: Arc<Game>, players_clone: &[Player], dimension_name: &str) -> bool {
		return self.feed_breedable_mob(held_item, game, players_clone, dimension_name);
	}

	fn extra_tick(&mut self, dimension: &Dimension, players: &[Player], game: std::sync::Arc<Game>) -> Vec<EntityTickOutcome> {
		let mut output: Vec<EntityTickOutcome> = Vec::new();
		let mut rng = rng();

		if rng.random_ratio(1, 10_000) {
			let egg_slot = Slot {
				count: 1,
				id: data::items::get_item_id_by_name("minecraft:egg").unwrap(),
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
				item: egg_slot,
				owner: 0,
				pickup_delay: 0,
				thrower: 0,
			};

			output.push(EntityTickOutcome::SummonEntity(Box::new(Entity::Item(item_entity))));
		}


		output.append(&mut self.tick_breedable_mob(dimension, players, game));
		return output;
	}

	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:chicken");
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
		return (0.7, 0.4);
	}
}

impl BreedableMobTrait for Chicken {
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
