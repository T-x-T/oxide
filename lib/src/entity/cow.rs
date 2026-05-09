use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Cow {
	pub common: CommonEntity,
	pub mob: CommonMob,
	pub breedable_mob: BreedableMob,
}

const FOOD: &[&str] = &["minecraft:wheat"];

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

	fn interact(
		&mut self,
		held_item: &Slot,
		game: Arc<Game>,
		_dimension: &mut Dimension,
		players_clone: &[Player],
		players: &mut [Player],
		player_uuid: u128,
	) -> EntityInteractResult {
		if held_item.count <= 0 || held_item.id != data::items::get_item_id_by_name("minecraft:bucket").unwrap() {
			return EntityInteractResult::DoNothing;
		}

		let player = players.iter_mut().find(|x| x.uuid == player_uuid).unwrap();

		let held_item = player.get_held_item(true).unwrap();
		if held_item.count > 1 {
			let slot = Slot {
				count: held_item.count - 1,
				..held_item.clone()
			};
			player.set_selected_inventory_slot(Some(slot), players_clone, game.clone());
		} else {
			player.set_selected_inventory_slot(None, players_clone, game.clone());
		}

		let milk_bucket_slot = Slot {
			count: 1,
			id: data::items::get_item_id_by_name("minecraft:milk_bucket").unwrap(),
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};
		player.add_item_to_inventory(milk_bucket_slot, players_clone, game.clone());

		return EntityInteractResult::DoNothing;
	}

	fn to_nbt_extras(&self) -> Vec<NbtTag> {
		return vec![self.mob.to_nbt(), self.breedable_mob.to_nbt()].into_iter().flatten().collect();
	}

	fn feed(&mut self, held_item: &Slot, game: Arc<Game>, players_clone: &[Player], dimension_name: &str) -> bool {
		return self.feed_breedable_mob(held_item, game, players_clone, dimension_name);
	}

	fn extra_tick(&mut self, dimension: &Dimension, players: &[Player], game: std::sync::Arc<Game>) -> Vec<EntityTickOutcome> {
		return self.tick_breedable_mob(dimension, players, game);
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

impl BreedableMobTrait for Cow {
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
