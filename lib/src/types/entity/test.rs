use super::*;

#[derive(Debug, Default)]
struct DefaultMob {
	common: CommonEntity,
	mob: CommonMob,
}

impl CommonEntityTrait for DefaultMob {
	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:creeper");
	}

	fn get_metadata(&self) -> Vec<crate::packets::clientbound::play::EntityMetadata> {
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

	fn new(_data: CommonEntity, _extra_nbt: NbtListTag) -> Self {
		todo!()
	}

	fn to_nbt_extras(&self) -> Vec<NbtTag> {
		todo!()
	}
}

#[derive(Debug, Default)]
struct BigMob {
	common: CommonEntity,
	mob: CommonMob,
}

impl CommonEntityTrait for BigMob {
	fn get_type(&self) -> i32 {
		return data::entities::get_id_from_name("minecraft:creeper");
	}

	fn get_metadata(&self) -> Vec<crate::packets::clientbound::play::EntityMetadata> {
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

	fn get_hitbox(&self) -> (f64, f64) {
		(4.0, 4.0)
	}

	fn new(_data: CommonEntity, _extra_nbt: NbtListTag) -> Self {
		todo!()
	}

	fn to_nbt_extras(&self) -> Vec<NbtTag> {
		todo!()
	}
}

mod get_occupied_block_positions_at_entity_position {
	use super::*;

	#[test]
	fn integer_position() {
		let entity = DefaultMob::default();
		let entity_position = EntityPosition {
			x: 10.0,
			y: 10.0,
			z: 10.0,
			yaw: 0.0,
			pitch: 0.0,
		};

		let mut res = entity.get_occupied_block_positions_at_entity_position(entity_position);

		let mut expected: Vec<BlockPosition> = vec![
			BlockPosition {
				x: 10,
				y: 10,
				z: 10,
			},
			BlockPosition {
				x: 10,
				y: 11,
				z: 10,
			},
		];

		res.sort();
		expected.sort();

		assert_eq!(res, expected);
	}

	#[test]
	fn block_center_pos() {
		let entity = DefaultMob::default();
		let entity_position = EntityPosition {
			x: 10.5,
			y: 10.5,
			z: 10.5,
			yaw: 0.0,
			pitch: 0.0,
		};

		let mut res = entity.get_occupied_block_positions_at_entity_position(entity_position);

		let mut expected: Vec<BlockPosition> = vec![
			BlockPosition {
				x: 10,
				y: 10,
				z: 10,
			},
			BlockPosition {
				x: 10,
				y: 10,
				z: 11,
			},
			BlockPosition {
				x: 10,
				y: 11,
				z: 10,
			},
			BlockPosition {
				x: 10,
				y: 11,
				z: 11,
			},
			BlockPosition {
				x: 10,
				y: 12,
				z: 10,
			},
			BlockPosition {
				x: 10,
				y: 12,
				z: 11,
			},
			BlockPosition {
				x: 11,
				y: 10,
				z: 10,
			},
			BlockPosition {
				x: 11,
				y: 10,
				z: 11,
			},
			BlockPosition {
				x: 11,
				y: 11,
				z: 10,
			},
			BlockPosition {
				x: 11,
				y: 11,
				z: 11,
			},
			BlockPosition {
				x: 11,
				y: 12,
				z: 10,
			},
			BlockPosition {
				x: 11,
				y: 12,
				z: 11,
			},
		];

		res.sort();
		expected.sort();

		assert_eq!(res, expected);
	}


	#[test]
	fn integer_position_big_mob() {
		let entity = BigMob::default();
		let entity_position = EntityPosition {
			x: 10.5,
			y: 10.0,
			z: 10.5,
			yaw: 0.0,
			pitch: 0.0,
		};

		let mut res = entity.get_occupied_block_positions_at_entity_position(entity_position);

		let mut expected: Vec<BlockPosition> = vec![
			BlockPosition {
				x: 9,
				y: 10,
				z: 9,
			},
			BlockPosition {
				x: 9,
				y: 10,
				z: 10,
			},
			BlockPosition {
				x: 9,
				y: 10,
				z: 11,
			},
			BlockPosition {
				x: 9,
				y: 10,
				z: 12,
			},
			BlockPosition {
				x: 9,
				y: 10,
				z: 13,
			},
			BlockPosition {
				x: 9,
				y: 11,
				z: 9,
			},
			BlockPosition {
				x: 9,
				y: 11,
				z: 10,
			},
			BlockPosition {
				x: 9,
				y: 11,
				z: 11,
			},
			BlockPosition {
				x: 9,
				y: 11,
				z: 12,
			},
			BlockPosition {
				x: 9,
				y: 11,
				z: 13,
			},
			BlockPosition {
				x: 9,
				y: 12,
				z: 9,
			},
			BlockPosition {
				x: 9,
				y: 12,
				z: 10,
			},
			BlockPosition {
				x: 9,
				y: 12,
				z: 11,
			},
			BlockPosition {
				x: 9,
				y: 12,
				z: 12,
			},
			BlockPosition {
				x: 9,
				y: 12,
				z: 13,
			},
			BlockPosition {
				x: 9,
				y: 13,
				z: 9,
			},
			BlockPosition {
				x: 9,
				y: 13,
				z: 10,
			},
			BlockPosition {
				x: 9,
				y: 13,
				z: 11,
			},
			BlockPosition {
				x: 9,
				y: 13,
				z: 12,
			},
			BlockPosition {
				x: 9,
				y: 13,
				z: 13,
			},
			BlockPosition {
				x: 10,
				y: 10,
				z: 9,
			},
			BlockPosition {
				x: 10,
				y: 10,
				z: 10,
			},
			BlockPosition {
				x: 10,
				y: 10,
				z: 11,
			},
			BlockPosition {
				x: 10,
				y: 10,
				z: 12,
			},
			BlockPosition {
				x: 10,
				y: 10,
				z: 13,
			},
			BlockPosition {
				x: 10,
				y: 11,
				z: 9,
			},
			BlockPosition {
				x: 10,
				y: 11,
				z: 10,
			},
			BlockPosition {
				x: 10,
				y: 11,
				z: 11,
			},
			BlockPosition {
				x: 10,
				y: 11,
				z: 12,
			},
			BlockPosition {
				x: 10,
				y: 11,
				z: 13,
			},
			BlockPosition {
				x: 10,
				y: 12,
				z: 9,
			},
			BlockPosition {
				x: 10,
				y: 12,
				z: 10,
			},
			BlockPosition {
				x: 10,
				y: 12,
				z: 11,
			},
			BlockPosition {
				x: 10,
				y: 12,
				z: 12,
			},
			BlockPosition {
				x: 10,
				y: 12,
				z: 13,
			},
			BlockPosition {
				x: 10,
				y: 13,
				z: 9,
			},
			BlockPosition {
				x: 10,
				y: 13,
				z: 10,
			},
			BlockPosition {
				x: 10,
				y: 13,
				z: 11,
			},
			BlockPosition {
				x: 10,
				y: 13,
				z: 12,
			},
			BlockPosition {
				x: 10,
				y: 13,
				z: 13,
			},
			BlockPosition {
				x: 11,
				y: 10,
				z: 9,
			},
			BlockPosition {
				x: 11,
				y: 10,
				z: 10,
			},
			BlockPosition {
				x: 11,
				y: 10,
				z: 11,
			},
			BlockPosition {
				x: 11,
				y: 10,
				z: 12,
			},
			BlockPosition {
				x: 11,
				y: 10,
				z: 13,
			},
			BlockPosition {
				x: 11,
				y: 11,
				z: 9,
			},
			BlockPosition {
				x: 11,
				y: 11,
				z: 10,
			},
			BlockPosition {
				x: 11,
				y: 11,
				z: 11,
			},
			BlockPosition {
				x: 11,
				y: 11,
				z: 12,
			},
			BlockPosition {
				x: 11,
				y: 11,
				z: 13,
			},
			BlockPosition {
				x: 11,
				y: 12,
				z: 9,
			},
			BlockPosition {
				x: 11,
				y: 12,
				z: 10,
			},
			BlockPosition {
				x: 11,
				y: 12,
				z: 11,
			},
			BlockPosition {
				x: 11,
				y: 12,
				z: 12,
			},
			BlockPosition {
				x: 11,
				y: 12,
				z: 13,
			},
			BlockPosition {
				x: 11,
				y: 13,
				z: 9,
			},
			BlockPosition {
				x: 11,
				y: 13,
				z: 10,
			},
			BlockPosition {
				x: 11,
				y: 13,
				z: 11,
			},
			BlockPosition {
				x: 11,
				y: 13,
				z: 12,
			},
			BlockPosition {
				x: 11,
				y: 13,
				z: 13,
			},
			BlockPosition {
				x: 12,
				y: 10,
				z: 9,
			},
			BlockPosition {
				x: 12,
				y: 10,
				z: 10,
			},
			BlockPosition {
				x: 12,
				y: 10,
				z: 11,
			},
			BlockPosition {
				x: 12,
				y: 10,
				z: 12,
			},
			BlockPosition {
				x: 12,
				y: 10,
				z: 13,
			},
			BlockPosition {
				x: 12,
				y: 11,
				z: 9,
			},
			BlockPosition {
				x: 12,
				y: 11,
				z: 10,
			},
			BlockPosition {
				x: 12,
				y: 11,
				z: 11,
			},
			BlockPosition {
				x: 12,
				y: 11,
				z: 12,
			},
			BlockPosition {
				x: 12,
				y: 11,
				z: 13,
			},
			BlockPosition {
				x: 12,
				y: 12,
				z: 9,
			},
			BlockPosition {
				x: 12,
				y: 12,
				z: 10,
			},
			BlockPosition {
				x: 12,
				y: 12,
				z: 11,
			},
			BlockPosition {
				x: 12,
				y: 12,
				z: 12,
			},
			BlockPosition {
				x: 12,
				y: 12,
				z: 13,
			},
			BlockPosition {
				x: 12,
				y: 13,
				z: 9,
			},
			BlockPosition {
				x: 12,
				y: 13,
				z: 10,
			},
			BlockPosition {
				x: 12,
				y: 13,
				z: 11,
			},
			BlockPosition {
				x: 12,
				y: 13,
				z: 12,
			},
			BlockPosition {
				x: 12,
				y: 13,
				z: 13,
			},
			BlockPosition {
				x: 13,
				y: 10,
				z: 9,
			},
			BlockPosition {
				x: 13,
				y: 10,
				z: 10,
			},
			BlockPosition {
				x: 13,
				y: 10,
				z: 11,
			},
			BlockPosition {
				x: 13,
				y: 10,
				z: 12,
			},
			BlockPosition {
				x: 13,
				y: 10,
				z: 13,
			},
			BlockPosition {
				x: 13,
				y: 11,
				z: 9,
			},
			BlockPosition {
				x: 13,
				y: 11,
				z: 10,
			},
			BlockPosition {
				x: 13,
				y: 11,
				z: 11,
			},
			BlockPosition {
				x: 13,
				y: 11,
				z: 12,
			},
			BlockPosition {
				x: 13,
				y: 11,
				z: 13,
			},
			BlockPosition {
				x: 13,
				y: 12,
				z: 9,
			},
			BlockPosition {
				x: 13,
				y: 12,
				z: 10,
			},
			BlockPosition {
				x: 13,
				y: 12,
				z: 11,
			},
			BlockPosition {
				x: 13,
				y: 12,
				z: 12,
			},
			BlockPosition {
				x: 13,
				y: 12,
				z: 13,
			},
			BlockPosition {
				x: 13,
				y: 13,
				z: 9,
			},
			BlockPosition {
				x: 13,
				y: 13,
				z: 10,
			},
			BlockPosition {
				x: 13,
				y: 13,
				z: 11,
			},
			BlockPosition {
				x: 13,
				y: 13,
				z: 12,
			},
			BlockPosition {
				x: 13,
				y: 13,
				z: 13,
			},
		];

		res.sort();
		expected.sort();

		assert_eq!(res, expected);
	}
}
