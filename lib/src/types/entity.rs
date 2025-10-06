use crate::packets::Packet;
use crate::types::*;
use crate::entity::*;

pub trait CreatableEntity: Entity + Send {
  fn new(position: EntityPosition, uuid: u128, entity_id: i32, extra_nbt: NbtListTag) -> Self;
  fn from_nbt(value: NbtListTag, next_entity_id: i32) -> Box<dyn SaveableEntity + Send> {
    let entity_type = value.get_child("id").unwrap().as_string();
    let x = value.get_child("Pos").unwrap().as_list()[0].as_double();
    let y = value.get_child("Pos").unwrap().as_list()[1].as_double();
    let z = value.get_child("Pos").unwrap().as_list()[2].as_double();
    let yaw = value.get_child("Rotation").unwrap().as_list()[0].as_float();
    let pitch = value.get_child("Rotation").unwrap().as_list()[1].as_float();
    let uuid = value.get_child("UUID").unwrap().as_int_array()
      .into_iter()
      .enumerate()
      .map(|x| (x.1 as u128) << (32 * (3 - x.0)))
      .reduce(|a, b| a | b)
      .unwrap();

    let position = EntityPosition { x, y, z, yaw, pitch };

    return self::new(entity_type, position, uuid, next_entity_id, value.clone()).unwrap();
  }
}

pub trait Entity: std::fmt::Debug {
  fn get_type(&self) -> i32;
  fn get_position(&self) -> EntityPosition;
  fn set_position(&mut self, position: EntityPosition);
  fn get_uuid(&self) -> u128;
  fn get_id(&self) -> i32;
  fn get_metadata(&self) -> Vec<crate::packets::clientbound::play::EntityMetadata>;
  fn get_yaw_u8(&self) -> u8 {
    return if self.get_position().yaw < 0.0 {
      (((self.get_position().yaw / 90.0) * 64.0) + 256.0) as u8
    } else {
     	((self.get_position().yaw / 90.0) * 64.0) as u8
    };
  }

  fn get_pitch_u8(&self) -> u8 {
    return if self.get_position().pitch < 0.0 {
      (((self.get_position().pitch / 90.0) * 64.0) + 256.0) as u8
    } else {
     	((self.get_position().pitch / 90.0) * 64.0) as u8
    };
  }

  fn tick(&mut self, chunk: &Chunk, players: &Vec<Player>) {
    if !self.is_on_ground(chunk) {
      let old_position = self.get_position();
      let old_z = self.get_position().z;
      self.set_position(EntityPosition { y: old_position.y - 1.0, ..old_position });

      for player in players {
        // crate::utils::send_packet(&player.connection_stream, crate::packets::clientbound::play::UpdateEntityPosition::PACKET_ID, crate::packets::clientbound::play::UpdateEntityPosition {
        //   entity_id: self.get_id(),
        //   delta_x: 0,
        //   delta_y: ((self.get_z() * 4096.0) - (old_z * 4096.0)) as i16,
        //   delta_z: 0,
        //   on_ground: self.is_on_ground(chunk),
        // }.try_into().unwrap()).unwrap();
        crate::utils::send_packet(&player.connection_stream, crate::packets::clientbound::play::TeleportEntity::PACKET_ID, crate::packets::clientbound::play::TeleportEntity {
          entity_id: self.get_id(),
          on_ground: self.is_on_ground(chunk),
          x: self.get_position().x,
          y: self.get_position().y,
          z: self.get_position().z,
          velocity_x: 0.0,
          velocity_y: 0.0,
          velocity_z: 0.0,
          yaw: 0.0,
          pitch: 0.0,
        }.try_into().unwrap()).unwrap();
      }
    }
  }

  fn is_on_ground(&self, chunk: &Chunk) -> bool {
    let position = BlockPosition::from(self.get_position()).convert_to_position_in_chunk();
    let block_at_location = chunk.get_block(BlockPosition {y: position.y - 1, ..position });
    return block_at_location != 0;
  }
}

pub trait SaveableEntity: Entity + Send {
  fn to_nbt_extras(&self) -> Vec<NbtTag>;
  fn to_nbt(&self) -> NbtListTag {
    let default_tags = vec![
      NbtTag::String("id".to_string(), data::entities::get_name_from_id(self.get_type())),
      NbtTag::List("Pos".to_string(), vec![
        NbtListTag::Double(self.get_position().x),
        NbtListTag::Double(self.get_position().y),
        NbtListTag::Double(self.get_position().z),
      ]),
      NbtTag::List("Rotation".to_string(), vec![
        NbtListTag::Float(self.get_position().yaw),
        NbtListTag::Float(self.get_position().pitch),
      ]),
      NbtTag::IntArray("UUID".to_string(), vec![
        (self.get_uuid() >> 96) as i32,
        (self.get_uuid() << 32 >> 96) as i32,
        (self.get_uuid() << 64 >> 96) as i32,
        (self.get_uuid() << 96 >> 96) as i32,
      ])];

    return NbtListTag::TagCompound(vec![default_tags, self.to_nbt_extras()].into_iter().flatten().collect());
  }
}

pub fn new(entity_type: &str, position: EntityPosition, uuid: u128, entity_id: i32, extra_nbt: NbtListTag) -> Option<Box<dyn SaveableEntity + Send>> {
  return match entity_type {
	  "minecraft:armadillo" => Some(Box::new(Armadillo::new(position, uuid, entity_id, extra_nbt))),
	  "minecraft:cat" => Some(Box::new(Cat::new(position, uuid, entity_id, extra_nbt))),
	  "minecraft:chest_minecart" => Some(Box::new(ChestMinecart::new(position, uuid, entity_id, extra_nbt))),
			"minecraft:chicken" => Some(Box::new(Chicken::new(position, uuid, entity_id, extra_nbt))),
			"minecraft:cow" => Some(Box::new(Cow::new(position, uuid, entity_id, extra_nbt))),
			"minecraft:creeper" => Some(Box::new(Creeper::new(position, uuid, entity_id, extra_nbt))),
			"minecraft:donkey" => Some(Box::new(Donkey::new(position, uuid, entity_id, extra_nbt))),
			"minecraft:horse" => Some(Box::new(Horse::new(position, uuid, entity_id, extra_nbt))),
			"minecraft:item" => Some(Box::new(ItemEntity::new(position, uuid, entity_id, extra_nbt))),
			"minecraft:parrot" => Some(Box::new(Parrot::new(position, uuid, entity_id, extra_nbt))),
			"minecraft:pig" => Some(Box::new(Pig::new(position, uuid, entity_id, extra_nbt))),
			"minecraft:rabbit" => Some(Box::new(Rabbit::new(position, uuid, entity_id, extra_nbt))),
			"minecraft:sheep" => Some(Box::new(Sheep::new(position, uuid, entity_id, extra_nbt))),
		_ => None,
  };
}
