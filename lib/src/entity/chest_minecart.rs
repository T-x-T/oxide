use super::*;

#[derive(Debug, PartialEq)]
pub struct ChestMinecart {
	pub common: CommonEntity,
}

impl CommonEntityTrait for ChestMinecart {
	fn new(data: CommonEntity, _extra_nbt: NbtListTag) -> Self {
		return Self {
			common: data,
		};
	}

	fn to_nbt_extras(&self) -> Vec<NbtTag> {
		return vec![];
	}

	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:chest_minecart");
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

	//(height, width) https://minecraft.wiki/w/Hitbox
	fn get_hitbox(&self) -> (f64, f64) {
		return (0.7, 0.98);
	}
}
