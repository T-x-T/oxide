use super::*;

#[derive(Debug)]
pub struct Creeper {
	pub common: CommonEntity,
	pub mob: CommonMob,
	pub explosion_radius: u8,
	pub fuse: i16,
	pub is_ignited: bool,
	pub is_powered: bool,
}

impl CreatableEntity for Creeper {
	fn new(data: CommonEntity, extra_nbt: NbtListTag) -> Self {
		let mob = CommonMob::from_nbt(extra_nbt.clone());

		return Self {
			common: data,
			mob,
			explosion_radius: extra_nbt.get_child("ExplosionRadius").unwrap_or(&NbtTag::Byte(String::new(), 3)).as_byte(),
			fuse: extra_nbt.get_child("Fuse").unwrap_or(&NbtTag::Short(String::new(), 30)).as_short(),
			is_ignited: extra_nbt.get_child("Ignited").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1,
			is_powered: extra_nbt.get_child("powered").unwrap_or(&NbtTag::Byte(String::new(), 0)).as_byte() == 1,
		};
	}
}

impl SaveableEntity for Creeper {
	fn to_nbt_extras(&self) -> Vec<NbtTag> {
		let mut output: Vec<NbtTag> = vec![
			NbtTag::Byte("ExplosionRadius".to_string(), self.explosion_radius),
			NbtTag::Short("Fuse".to_string(), self.fuse),
			NbtTag::Byte("Ignited".to_string(), if self.is_ignited { 1 } else { 0 }),
			NbtTag::Byte("powered".to_string(), if self.is_powered { 1 } else { 0 }),
		];

		output.append(&mut self.mob.to_nbt());

		return output;
	}
}

impl Entity for Creeper {
	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:creeper");
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
		return (1.7, 0.6);
	}
}
