use std::collections::HashMap;

use rand::Rng;

use crate::packets::Packet;
use crate::types::*;
use crate::entity::*;

#[derive(Debug, PartialEq, Eq)]
pub enum EntityTickOutcome {
  SelfDied,
  RemoveSelf,
  Updated,
  None,
}

#[derive(Debug)]
pub enum AiBehavior {
  Idle,
  MoveTowardsPlayer,
  Wander,
}

#[derive(Debug)]
pub enum AiExecutionResult {
  DoNothing,
  ApplyVelocity(EntityPosition),
}

pub trait CreatableEntity: Entity + Send {
  fn new(data: CommonEntity, extra_nbt: NbtListTag) -> Self;
  fn from_nbt(value: NbtListTag, next_entity_id: &mut i32) -> Box<dyn SaveableEntity + Send> {
    *next_entity_id += 1;
    let mut common_data = CommonEntity {
      entity_id: *next_entity_id,
      ..Default::default()
    };

    let entity_type = value.get_child("id").unwrap().as_string();
    let x = value.get_child("Pos").unwrap().as_list()[0].as_double();
    let y = value.get_child("Pos").unwrap().as_list()[1].as_double();
    let z = value.get_child("Pos").unwrap().as_list()[2].as_double();
    let yaw = value.get_child("Rotation").unwrap().as_list()[0].as_float();
    let pitch = value.get_child("Rotation").unwrap().as_list()[1].as_float();

    common_data.position = EntityPosition { x, y, z, yaw, pitch };

    if value.get_child("Motion").is_some() {
      common_data.velocity = EntityPosition {
        x: value.get_child("Motion").unwrap().as_list()[0].as_double(),
        y: value.get_child("Motion").unwrap().as_list()[1].as_double(),
        z: value.get_child("Motion").unwrap().as_list()[2].as_double(),
        yaw,
        pitch,
      };
    }

    common_data.uuid = value.get_child("UUID").unwrap().as_int_array()
      .into_iter()
      .enumerate()
      .map(|x| (x.1 as u128) << (32 * (3 - x.0)))
      .reduce(|a, b| a | b)
      .unwrap();

    if let Some(value) = value.get_child("Air") {
      common_data.air = value.as_short();
    }

    if let Some(value) = value.get_child("CustomName") {
      common_data.custom_name = value.clone();
    }

    if let Some(value) = value.get_child("CustomNameVisible") {
      common_data.custom_name_visible = value.as_byte() == 1;
    }

    if let Some(value) = value.get_child("data") {
      common_data.data = value.clone();
    }

    if let Some(value) = value.get_child("fall_distance") {
      common_data.fall_distance = value.as_double();
    }

    if let Some(value) = value.get_child("Fire") {
      common_data.ticks_until_fire_is_out = value.as_short();
    }

    if let Some(value) = value.get_child("Glowing") {
      common_data.is_glowing = value.as_byte() == 1;
    }

    if let Some(value) = value.get_child("HasVisualFire") {
      common_data.has_visual_fire = value.as_byte() == 1;
    }

    if let Some(value) = value.get_child("Invulnerable") {
      common_data.invulnerable = value.as_byte() == 1;
    }

    if let Some(value) = value.get_child("NoGravity") {
      common_data.no_gravity = value.as_byte() == 1;
    }

    if let Some(value) = value.get_child("OnGround") {
      common_data.on_ground = value.as_byte() == 1;
    }

    if let Some(_value) = value.get_child("Passengers") {
      //TODO: actually implement this
      common_data.passengers = Vec::new();
    }

    if let Some(value) = value.get_child("PortalCooldown") {
      common_data.portal_cooldown = value.as_int();
    }

    if let Some(value) = value.get_child("Silent") {
      common_data.is_silent = value.as_byte() == 1;
    }

    if let Some(value) = value.get_child("Tags") {
      common_data.scoreboard_tags = value.as_list().clone();
    }

    if let Some(value) = value.get_child("TicksFrozen") {
      common_data.ticks_frozen = value.as_int();
    }

    return self::new(entity_type, common_data, value.clone()).unwrap();
  }
}

pub trait Entity: std::fmt::Debug {
  fn get_common_entity_data(&self) -> &CommonEntity;
  fn get_common_entity_data_mut(&mut self) -> &mut CommonEntity;
  fn set_common_entity_data(&mut self, common_entity_data: CommonEntity);
  fn get_type(&self) -> i32;
  fn get_metadata(&self) -> Vec<crate::packets::clientbound::play::EntityMetadata>;

  fn is_mob(&self) -> bool {
    return false;
  }
  fn get_mob_data(&self) -> &CommonMob {
    panic!("{} is not a mob", data::entities::get_name_from_id(self.get_type()));
  }
  fn get_mob_data_mut(&mut self) -> &mut CommonMob {
    panic!("{} is not a mob", data::entities::get_name_from_id(self.get_type()));
  }
  fn set_mob_data(&mut self, _common_mob_data: CommonMob) {
    panic!("{} is not a mob", data::entities::get_name_from_id(self.get_type()));
  }

  fn get_yaw_u8(&self) -> u8 {
    return if self.get_common_entity_data().position.yaw < 0.0 {
      (((self.get_common_entity_data().position.yaw / 90.0) * 64.0) + 256.0) as u8
    } else {
     	((self.get_common_entity_data().position.yaw / 90.0) * 64.0) as u8
    };
  }

  fn get_pitch_u8(&self) -> u8 {
    return if self.get_common_entity_data().position.pitch < 0.0 {
      (((self.get_common_entity_data().position.pitch / 90.0) * 64.0) + 256.0) as u8
    } else {
     	((self.get_common_entity_data().position.pitch / 90.0) * 64.0) as u8
    };
  }

  fn tick(&mut self, dimension: &Dimension, players: &Vec<Player>, block_state_data: &HashMap<String, data::blocks::Block>) -> EntityTickOutcome {
    if self.is_mob() {
      let mob_data = self.get_mob_data_mut();

      if mob_data.death_time == 20 {
        return EntityTickOutcome::RemoveSelf;
      }

      if mob_data.death_time > 0 {
        mob_data.death_time += 1;
        return EntityTickOutcome::None;
      }

      mob_data.alive_for_ticks += 1;

      if mob_data.hurt_time > 0 {
        mob_data.hurt_time -= 1;
      }

      if mob_data.health <= 0.0 {
        mob_data.death_time = 1;
        return EntityTickOutcome::SelfDied;
      }
    }


    let old_position = self.get_common_entity_data().position;

    if !(self.is_mob() && self.get_mob_data().hurt_time != 0) {
      if self.is_on_ground(dimension, block_state_data) {
        self.get_common_entity_data_mut().position.y = self.get_common_entity_data_mut().position.y.floor();
      } else {
        self.get_common_entity_data_mut().velocity.y -= 0.08;
      }
    }

    //the order in which these are applied differs between different entities, see https://minecraft.wiki/w/Entity#Motion
    let velocity = self.get_common_entity_data().velocity;
    self.get_common_entity_data_mut().velocity = EntityPosition {
      x: velocity.x * 0.91,
      y: velocity.y * 0.98,
      z: velocity.z * 0.91,
      ..velocity
    };


    let mut velocity_from_ai = EntityPosition::default();
    match self.execute_ai(players) {
      AiExecutionResult::DoNothing => (),
      AiExecutionResult::ApplyVelocity(x) => velocity_from_ai = x,
    };

    let mut velocity = self.get_common_entity_data().velocity;
    velocity = velocity + velocity_from_ai;

    let number_of_positions_to_check = velocity.x.abs().max(velocity.y.abs().max(velocity.z).abs()).ceil() as u16 * 16;
    let mut last_velocity = EntityPosition::default();
    for i in 0..=number_of_positions_to_check {
      let velocity_to_check = EntityPosition {
        x: (velocity.x / (number_of_positions_to_check + 1) as f64) * i as f64,
        y: (velocity.y / (number_of_positions_to_check + 1) as f64) * i as f64,
        z: (velocity.z / (number_of_positions_to_check + 1) as f64) * i as f64,
        ..Default::default()
      };

      let entity_position_to_check = EntityPosition {
        x: old_position.x + velocity_to_check.x,
        y: old_position.y + velocity_to_check.y,
        z: old_position.z + velocity_to_check.z,
        ..old_position
      };

      if self.collides_with_blocks_at(dimension, block_state_data, entity_position_to_check) {
        velocity = last_velocity;

        //Check if jumping would help
        if self.is_on_ground(dimension, block_state_data) && !self.collides_with_blocks_at(dimension, block_state_data, EntityPosition { y: entity_position_to_check.y + 1.0, ..entity_position_to_check }) {
            self.get_common_entity_data_mut().velocity.y += 0.025;
          };
        break;
      }

      last_velocity = velocity_to_check;
    }


    let mut next_position = EntityPosition {
      x: old_position.x + velocity.x,
      y: old_position.y + velocity.y,
      z: old_position.z + velocity.z,
      ..old_position
    };
    if self.is_on_ground_at(dimension, block_state_data, next_position) {
      next_position.y = next_position.y.round();
    }

    self.get_common_entity_data_mut().position = next_position;

    if old_position != self.get_common_entity_data().position {
      let packet = crate::packets::clientbound::play::UpdateEntityPosition {
        entity_id: self.get_common_entity_data().entity_id,
        delta_x: ((self.get_common_entity_data().position.x * 4096.0) - (old_position.x * 4096.0)) as i16,
        delta_y: ((self.get_common_entity_data().position.y * 4096.0) - (old_position.y * 4096.0)) as i16,
        delta_z: ((self.get_common_entity_data().position.z * 4096.0) - (old_position.z * 4096.0)) as i16,
        on_ground: self.is_on_ground(dimension, block_state_data),
      };

      for player in players {
        let _ = crate::utils::send_packet(&player.connection_stream, crate::packets::clientbound::play::UpdateEntityPosition::PACKET_ID, packet.clone().try_into().unwrap());
      }

      return EntityTickOutcome::Updated;
    }

    return EntityTickOutcome::None;
  }

  fn collides_with_blocks_at(&self, dimension: &Dimension, block_state_data: &HashMap<String, data::blocks::Block>, position_to_check: EntityPosition) -> bool {
    let positions_to_check = self.get_occupied_block_positions_at_entity_position(position_to_check);

    for position_to_check in positions_to_check {
      let block_at_location = dimension.get_block(position_to_check).unwrap_or(0);
      let block_type_at_location = data::blocks::get_type_from_block_state_id(block_at_location, block_state_data);
      if block_type_at_location.is_solid() {
        return true;
      }
    }

    return false;
  }

  fn is_on_ground(&self, dimension: &Dimension, block_state_data: &HashMap<String, data::blocks::Block>) -> bool {
    return self.is_on_ground_at(dimension, block_state_data, self.get_common_entity_data().position);
  }

  fn is_on_ground_at(&self, dimension: &Dimension, block_state_data: &HashMap<String, data::blocks::Block>, mut position_to_check: EntityPosition) -> bool {
    position_to_check.y -= 0.1;

    let positions_to_check = self.get_occupied_block_positions_at_entity_position(position_to_check);

    for position_to_check in positions_to_check {
      let block_at_location = dimension.get_block(position_to_check).unwrap_or(0);
      let block_type_at_location = data::blocks::get_type_from_block_state_id(block_at_location, block_state_data);
      if block_type_at_location.is_solid() {
        return true;
      }
    }

    return false;
  }

  //(height, width) https://minecraft.wiki/w/Hitbox
  fn get_hitbox(&self) -> (f64, f64) {
    return (1.7, 0.6);
  }

  fn get_occupied_block_positions(&self) -> Vec<BlockPosition> {
    return self.get_occupied_block_positions_at_entity_position(self.get_common_entity_data().position);
  }

  fn get_occupied_block_positions_at_entity_position(&self, position_to_check: EntityPosition) -> Vec<BlockPosition> {
    let mut output: Vec<BlockPosition> = Vec::new();

    //seems like the center off the hitbox is offset by half a block from the entity position
    let x_offset = if position_to_check.x.abs() < 1.0 { 0.0 } else if position_to_check.x.is_sign_positive() { 0.5 } else { -0.5 };
    let z_offset = if position_to_check.z.abs() < 1.0 { 0.0 } else if position_to_check.z.is_sign_positive() { 0.5 } else { -0.5 };

    let x_min = position_to_check.x + x_offset - (self.get_hitbox().1 * 0.5);
    let x_max = position_to_check.x + x_offset + (self.get_hitbox().1 * 0.5);
    let x_range = (if x_min.is_sign_positive() {x_min.floor()} else {x_min.ceil()} as i32)..=(if x_max.is_sign_positive() {x_max.floor()} else {x_max.ceil()} as i32);
    let y_min = position_to_check.y;
    let y_max = position_to_check.y + self.get_hitbox().0 - 0.01;
    let y_range = (if y_min.is_sign_positive() {y_min.floor()} else {y_min.ceil()} as i16)..=(if y_max.is_sign_positive() {y_max.floor()} else {y_max.ceil()} as i16);
    let z_min = position_to_check.z + z_offset - (self.get_hitbox().1 * 0.5);
    let z_max = position_to_check.z + z_offset + (self.get_hitbox().1 * 0.5);
    let z_range = (if z_min.is_sign_positive() {z_min.floor()} else {z_min.ceil()} as i32)..=(if z_max.is_sign_positive() {z_max.floor()} else {z_max.ceil()} as i32);

    for x in x_range.clone() {
      for y in y_range.clone() {
        for z in z_range.clone() {
          output.push(BlockPosition { x, y, z });
        }
      }
    }

    return output;
  }

  fn execute_ai(&mut self, players: &[Player]) -> AiExecutionResult {
    let entity_type = data::entities::get_name_from_id(self.get_type());
    let behavior = if entity_type.as_str() == "minecraft:creeper" {
      AiBehavior::MoveTowardsPlayer
    } else if self.is_mob() {
      AiBehavior::Wander
    } else {
      AiBehavior::Idle
    };

    return match behavior {
      AiBehavior::Idle => AiExecutionResult::DoNothing,
      AiBehavior::MoveTowardsPlayer => self.execute_ai_move_towards_player(players),
      AiBehavior::Wander => self.execute_ai_wander(),
    };
  }

  fn execute_ai_wander(&mut self) -> AiExecutionResult {
    if self.get_mob_data().wander_to.is_none() || self.get_mob_data().wandered_for > 200 {
      let mut rng = rand::rng();
      let block_pos_of_entity = BlockPosition::from(self.get_common_entity_data().position);

      self.get_mob_data_mut().wander_to = Some(BlockPosition {
        x: block_pos_of_entity.x + rng.random_range(-10..10),
        y: block_pos_of_entity.y,
        z: block_pos_of_entity.z + rng.random_range(-10..10),
      });

      self.get_mob_data_mut().wandered_for = 0;
    }

    self.get_mob_data_mut().wandered_for += 1;

    let velocity_towards_goal = EntityPosition::from(self.get_mob_data().wander_to.unwrap()) - self.get_common_entity_data().position;
    let distance_towards_goal = self.get_common_entity_data().position.distance_to(self.get_mob_data().wander_to.unwrap().into());
    if distance_towards_goal < 1.0 {
      self.get_mob_data_mut().wander_to = None;
      return AiExecutionResult::DoNothing;
    } else {
      let speed = 0.02;
      return AiExecutionResult::ApplyVelocity(EntityPosition {
        x: (velocity_towards_goal.x / (distance_towards_goal + 1.0)) * speed,
        y: 0.0,
        z: (velocity_towards_goal.z / (distance_towards_goal + 1.0)) * speed,
        yaw: 0.0,
        pitch: 0.0,
      });
    }
  }

  fn execute_ai_move_towards_player(&self, players: &[Player]) -> AiExecutionResult {
    let mut player_distances = players.iter()
      .map(|x| (x, self.get_common_entity_data().position.distance_to(x.get_position())))
      .filter(|x| x.1 < 25.0)
      .collect::<Vec<(&Player, f64)>>();

    player_distances.sort_by(|a, b| a.1.total_cmp(&b.1));
    let closest_player = player_distances.first();

    if let Some(closest_player) = closest_player {
      let velocity_towards_player = closest_player.0.get_position() - self.get_common_entity_data().position;
      let distance_towards_player = self.get_common_entity_data().position.distance_to(closest_player.0.get_position());
      let speed = 0.1;
      return AiExecutionResult::ApplyVelocity(EntityPosition {
        x: (velocity_towards_player.x / (distance_towards_player + 1.0)) * speed,
        y: 0.0,
        z: (velocity_towards_player.z / (distance_towards_player + 1.0)) * speed,
        yaw: 0.0,
        pitch: 0.0,
      });
    } else {
      return AiExecutionResult::DoNothing;
    }
  }
}

pub trait SaveableEntity: Entity + Send {
  fn to_nbt_extras(&self) -> Vec<NbtTag>;
  fn to_nbt(&self) -> NbtListTag {
    let common_data = self.get_common_entity_data();
    let default_tags = vec![
      NbtTag::String("id".to_string(), data::entities::get_name_from_id(self.get_type())),
      NbtTag::List("Pos".to_string(), vec![
        NbtListTag::Double(common_data.position.x),
        NbtListTag::Double(common_data.position.y),
        NbtListTag::Double(common_data.position.z),
      ]),
      NbtTag::List("Motion".to_string(), vec![
        NbtListTag::Double(common_data.velocity.x),
        NbtListTag::Double(common_data.velocity.y),
        NbtListTag::Double(common_data.velocity.z),
      ]),
      NbtTag::List("Rotation".to_string(), vec![
        NbtListTag::Float(common_data.position.yaw),
        NbtListTag::Float(common_data.position.pitch),
      ]),
      NbtTag::IntArray("UUID".to_string(), vec![
        (common_data.uuid >> 96) as i32,
        (common_data.uuid << 32 >> 96) as i32,
        (common_data.uuid << 64 >> 96) as i32,
        (common_data.uuid << 96 >> 96) as i32,
      ]),
      NbtTag::Short("Air".to_string(), common_data.air),
      common_data.custom_name.clone(),
      common_data.data.clone(),
      NbtTag::Double("fall_distance".to_string(), common_data.fall_distance),
      NbtTag::Short("Fire".to_string(), common_data.ticks_until_fire_is_out),
      NbtTag::Byte("Glowing".to_string(), if common_data.is_glowing { 1 } else { 0 }),
      NbtTag::Byte("HasVisualFire".to_string(), if common_data.has_visual_fire { 1 } else { 0 }),
      NbtTag::Byte("Invulnerable".to_string(), if common_data.invulnerable { 1 } else { 0 }),
      NbtTag::Byte("NoGravity".to_string(), if common_data.no_gravity { 1 } else { 0 }),
      NbtTag::Byte("OnGround".to_string(), if common_data.on_ground { 1 } else { 0 }),
      NbtTag::List("Passengers".to_string(), common_data.passengers.iter().map(|x| x.to_nbt()).collect()),
      NbtTag::Int("PortalCooldown".to_string(), common_data.portal_cooldown),
      NbtTag::Byte("Silent".to_string(), if common_data.is_silent { 1 } else { 0 }),
      NbtTag::List("Tags".to_string(), common_data.scoreboard_tags.clone()),
      NbtTag::Int("TicksFrozen".to_string(), common_data.ticks_frozen),
    ];

    return NbtListTag::TagCompound(vec![default_tags, self.to_nbt_extras()].into_iter().flatten().collect());
  }
}

impl CommonMob {
  pub fn from_nbt(data: NbtListTag) -> CommonMob {
    let mut output = CommonMob::default();

    if let Some(value) = data.get_child("AbsorptionAmount") { output.absorption_amount = value.as_float(); }
    if let Some(value) = data.get_child("active_effects") { output.active_effects = value.as_list(); }
    if let Some(value) = data.get_child("attributes") { output.attributes = value.as_list(); }
    if let Some(value) = data.get_child("brain") { output.brain = value.clone(); }
    if let Some(value) = data.get_child("CanPickUpLoot") { output.can_pick_up_loot = value.as_byte() == 1; }
    if let Some(value) = data.get_child("DeathLootTable") { output.death_loot_table = Some(value.as_string().to_string()); }
    if let Some(value) = data.get_child("DeathLootTableSeed") { output.death_loot_table_seed = Some(value.as_long()); }
    if let Some(value) = data.get_child("DeathTime") { output.death_time = value.as_short(); }
    if let Some(value) = data.get_child("drop_chances") { output.drop_chances = value.clone(); }
    if let Some(value) = data.get_child("equipment") { let mut equipment: HashMap<String, Item> = HashMap::new();
      if let Some(x) = value.get_child("head") { equipment.insert("head".to_string(), x.clone().into()); }
      if let Some(x) = value.get_child("chest") { equipment.insert("chest".to_string(), x.clone().into()); }
      if let Some(x) = value.get_child("legs") { equipment.insert("legs".to_string(), x.clone().into()); }
      if let Some(x) = value.get_child("feet") { equipment.insert("feet".to_string(), x.clone().into()); }
      if let Some(x) = value.get_child("mainhand") { equipment.insert("mainhand".to_string(), x.clone().into()); }
      if let Some(x) = value.get_child("offhand") { equipment.insert("offhand".to_string(), x.clone().into()); }
      if let Some(x) = value.get_child("body") { equipment.insert("body".to_string(), x.clone().into()); }
      if let Some(x) = value.get_child("saddle") { equipment.insert("saddle".to_string(), x.clone().into()); }
    }
    if let Some(value) = data.get_child("fall_flying") { output.fall_flying = value.as_byte(); }
    if let Some(value) = data.get_child("health") { output.health = value.as_float(); } else { output.health = 20.0 } //TODO: find a proper way to assign default values like health
    if let Some(value) = data.get_child("home_pos") { output.home_location = ( value.as_int_array()[0], value.as_int_array()[1], value.as_int_array()[2] ); }
    if let Some(value) = data.get_child("home_radius") { output.home_radius = value.as_int(); }
    if let Some(value) = data.get_child("HurtByTimestamp") { output.hurt_by_timestamp = value.as_int(); }
    if let Some(value) = data.get_child("HurtTime") { output.hurt_time = value.as_short(); }
    if let Some(value) = data.get_child("leash") {
      match value {
        NbtTag::TagCompound(_, _) => output.leashed_block = Some((value.as_int_array()[0], value.as_int_array()[1], value.as_int_array()[2])),
        NbtTag::IntArray(_, _) => output.leashed_entity = Some(value.get_child("UUID").unwrap()
          .as_int_array()
          .into_iter()
          .enumerate()
          .map(|x| (x.1 as u128) << (32 * (3 - x.0)))
          .reduce(|a, b| a | b)
          .unwrap()
        ),
        _ => ()
      }
    }
    if let Some(value) = data.get_child("LeftHanded") {  output.is_left_handed = value.as_byte() == 1; }
    if let Some(value) = data.get_child("locator_bar_icon") {
      output.locator_bar_icon_color = Some(value.get_child("color").unwrap().as_int());
      output.locator_bar_icon_style = Some(value.get_child("color").unwrap().as_string().to_string());
    }
    if let Some(value) = data.get_child("NoAI") { output.has_no_ai = value.as_byte() == 1; }
    if let Some(value) = data.get_child("PersistanceRequired") { output.is_persistance_required = value.as_byte() == 1; }
    if let Some(value) = data.get_child("sleeping_pos") {
      output.sleeping_location = Some((
        value.as_int_array()[0],
        value.as_int_array()[1],
        value.as_int_array()[2]
      ));
    }

    return output;
  }

  pub fn to_nbt(&self) -> Vec<NbtTag> {
    let mut output: Vec<NbtTag> = vec![
      NbtTag::Float("AbsorptionAmount".to_string(), self.absorption_amount),
      NbtTag::List("active_effects".to_string(), self.active_effects.clone()),
      NbtTag::List("attributes".to_string(), self.attributes.clone()),
      self.brain.clone(),
      NbtTag::Byte("CanPickUpLoot".to_string(), if self.can_pick_up_loot { 1 } else { 0 }),
      NbtTag::Short("DeathTime".to_string(), self.death_time),
      self.drop_chances.clone(),
      NbtTag::Byte("FallFlying".to_string(), self.fall_flying),
      NbtTag::Float("Health".to_string(), self.health),
      NbtTag::IntArray("home_pos".to_string(), vec![self.home_location.0, self.home_location.1, self.home_location.2]),
      NbtTag::Int("home_radius".to_string(), self.home_radius),
      NbtTag::Int("HurtByTimestamp".to_string(), self.hurt_by_timestamp),
      NbtTag::Short("HurtTime".to_string(), self.hurt_time),
      NbtTag::Byte("LeftHanded".to_string(), if self.is_left_handed { 1 } else { 0 }),
      NbtTag::Byte("NoAI".to_string(), if self.has_no_ai { 1 } else { 0 }),
      NbtTag::Byte("PersitanceRequired".to_string(), if self.is_persistance_required { 1 } else { 0 }),
    ];

    if let Some(value) = self.death_loot_table.clone() { output.push(NbtTag::String("DeathLootTable".to_string(), value)); }
    if let Some(value) = self.death_loot_table_seed { output.push(NbtTag::Long("DeathLootTableSeed".to_string(), value)); }
    if !self.equipment.is_empty() {
      let mut entries: Vec<NbtTag> = Vec::new();
      for x in &self.equipment {
        entries.push(NbtTag::TagCompound(x.0.clone(), x.1.clone().into()));
      }
    }
    if let Some(value) = self.leashed_block { output.push(NbtTag::IntArray("leash".to_string(), vec![value.0, value.1, value.2])); }
    if let Some(value) = self.leashed_entity { output.push(NbtTag::TagCompound("leash".to_string(), vec![
      NbtTag::IntArray("UUID".to_string(), vec![
        (value >> 96) as i32,
        (value << 32 >> 96) as i32,
        (value << 64 >> 96) as i32,
        (value << 96 >> 96) as i32,
      ]),
    ])); }
    if self.locator_bar_icon_color.is_some() { output.push(NbtTag::TagCompound("locator_bar_icon".to_string(), vec![
      NbtTag::Int("color".to_string(), self.locator_bar_icon_color.unwrap()),
      NbtTag::String("style".to_string(), self.locator_bar_icon_style.clone().unwrap())
    ])); }
    if let Some(value) = self.sleeping_location { output.push(NbtTag::IntArray("sleeping_pos".to_string(), vec![value.0, value.1, value.2])); }

    return output;
  }
}

pub fn new(entity_type: &str, common_data: CommonEntity, extra_nbt: NbtListTag) -> Option<Box<dyn SaveableEntity + Send>> {
  return match entity_type {
    "minecraft:armadillo" => Some(Box::new(Armadillo::new(common_data, extra_nbt))),
    "minecraft:cat" => Some(Box::new(Cat::new(common_data, extra_nbt))),
    "minecraft:chest_minecart" => Some(Box::new(ChestMinecart::new(common_data, extra_nbt))),
    "minecraft:chicken" => Some(Box::new(Chicken::new(common_data, extra_nbt))),
    "minecraft:cow" => Some(Box::new(Cow::new(common_data, extra_nbt))),
    "minecraft:creeper" => Some(Box::new(Creeper::new(common_data, extra_nbt))),
    "minecraft:donkey" => Some(Box::new(Donkey::new(common_data, extra_nbt))),
    "minecraft:horse" => Some(Box::new(Horse::new(common_data, extra_nbt))),
    "minecraft:item" => Some(Box::new(ItemEntity::new(common_data, extra_nbt))),
    "minecraft:parrot" => Some(Box::new(Parrot::new(common_data, extra_nbt))),
    "minecraft:pig" => Some(Box::new(Pig::new(common_data, extra_nbt))),
    "minecraft:rabbit" => Some(Box::new(Rabbit::new(common_data, extra_nbt))),
    "minecraft:sheep" => Some(Box::new(Sheep::new(common_data, extra_nbt))),
		_ => None,
  };
}


#[cfg(test)]
mod test {
  use super::*;

  #[derive(Debug, Default)]
  struct DefaultMob {
    common: CommonEntity,
    mob: CommonMob,
  }

  impl Entity for DefaultMob {
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
  }

  #[derive(Debug, Default)]
  struct BigMob {
    common: CommonEntity,
    mob: CommonMob,
  }

  impl Entity for BigMob {
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
        BlockPosition { x: 10, y: 10, z: 10 }, BlockPosition { x: 10, y: 11, z: 10 }
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
        BlockPosition { x: 10, y: 10, z: 10 }, BlockPosition { x: 10, y: 10, z: 11 }, BlockPosition { x: 10, y: 11, z: 10 }, BlockPosition { x: 10, y: 11, z: 11 }, BlockPosition { x: 10, y: 12, z: 10 }, BlockPosition { x: 10, y: 12, z: 11 }, BlockPosition { x: 11, y: 10, z: 10 }, BlockPosition { x: 11, y: 10, z: 11 }, BlockPosition { x: 11, y: 11, z: 10 }, BlockPosition { x: 11, y: 11, z: 11 }, BlockPosition { x: 11, y: 12, z: 10 }, BlockPosition { x: 11, y: 12, z: 11 }
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
        BlockPosition { x: 9, y: 10, z: 9 }, BlockPosition { x: 9, y: 10, z: 10 }, BlockPosition { x: 9, y: 10, z: 11 }, BlockPosition { x: 9, y: 10, z: 12 }, BlockPosition { x: 9, y: 10, z: 13 }, BlockPosition { x: 9, y: 11, z: 9 }, BlockPosition { x: 9, y: 11, z: 10 }, BlockPosition { x: 9, y: 11, z: 11 }, BlockPosition { x: 9, y: 11, z: 12 }, BlockPosition { x: 9, y: 11, z: 13 }, BlockPosition { x: 9, y: 12, z: 9 }, BlockPosition { x: 9, y: 12, z: 10 }, BlockPosition { x: 9, y: 12, z: 11 }, BlockPosition { x: 9, y: 12, z: 12 }, BlockPosition { x: 9, y: 12, z: 13 }, BlockPosition { x: 9, y: 13, z: 9 }, BlockPosition { x: 9, y: 13, z: 10 }, BlockPosition { x: 9, y: 13, z: 11 }, BlockPosition { x: 9, y: 13, z: 12 }, BlockPosition { x: 9, y: 13, z: 13 }, BlockPosition { x: 10, y: 10, z: 9 }, BlockPosition { x: 10, y: 10, z: 10 }, BlockPosition { x: 10, y: 10, z: 11 }, BlockPosition { x: 10, y: 10, z: 12 }, BlockPosition { x: 10, y: 10, z: 13 }, BlockPosition { x: 10, y: 11, z: 9 }, BlockPosition { x: 10, y: 11, z: 10 }, BlockPosition { x: 10, y: 11, z: 11 }, BlockPosition { x: 10, y: 11, z: 12 }, BlockPosition { x: 10, y: 11, z: 13 }, BlockPosition { x: 10, y: 12, z: 9 }, BlockPosition { x: 10, y: 12, z: 10 }, BlockPosition { x: 10, y: 12, z: 11 }, BlockPosition { x: 10, y: 12, z: 12 }, BlockPosition { x: 10, y: 12, z: 13 }, BlockPosition { x: 10, y: 13, z: 9 }, BlockPosition { x: 10, y: 13, z: 10 }, BlockPosition { x: 10, y: 13, z: 11 }, BlockPosition { x: 10, y: 13, z: 12 }, BlockPosition { x: 10, y: 13, z: 13 }, BlockPosition { x: 11, y: 10, z: 9 }, BlockPosition { x: 11, y: 10, z: 10 }, BlockPosition { x: 11, y: 10, z: 11 }, BlockPosition { x: 11, y: 10, z: 12 }, BlockPosition { x: 11, y: 10, z: 13 }, BlockPosition { x: 11, y: 11, z: 9 }, BlockPosition { x: 11, y: 11, z: 10 }, BlockPosition { x: 11, y: 11, z: 11 }, BlockPosition { x: 11, y: 11, z: 12 }, BlockPosition { x: 11, y: 11, z: 13 }, BlockPosition { x: 11, y: 12, z: 9 }, BlockPosition { x: 11, y: 12, z: 10 }, BlockPosition { x: 11, y: 12, z: 11 }, BlockPosition { x: 11, y: 12, z: 12 }, BlockPosition { x: 11, y: 12, z: 13 }, BlockPosition { x: 11, y: 13, z: 9 }, BlockPosition { x: 11, y: 13, z: 10 }, BlockPosition { x: 11, y: 13, z: 11 }, BlockPosition { x: 11, y: 13, z: 12 }, BlockPosition { x: 11, y: 13, z: 13 }, BlockPosition { x: 12, y: 10, z: 9 }, BlockPosition { x: 12, y: 10, z: 10 }, BlockPosition { x: 12, y: 10, z: 11 }, BlockPosition { x: 12, y: 10, z: 12 }, BlockPosition { x: 12, y: 10, z: 13 }, BlockPosition { x: 12, y: 11, z: 9 }, BlockPosition { x: 12, y: 11, z: 10 }, BlockPosition { x: 12, y: 11, z: 11 }, BlockPosition { x: 12, y: 11, z: 12 }, BlockPosition { x: 12, y: 11, z: 13 }, BlockPosition { x: 12, y: 12, z: 9 }, BlockPosition { x: 12, y: 12, z: 10 }, BlockPosition { x: 12, y: 12, z: 11 }, BlockPosition { x: 12, y: 12, z: 12 }, BlockPosition { x: 12, y: 12, z: 13 }, BlockPosition { x: 12, y: 13, z: 9 }, BlockPosition { x: 12, y: 13, z: 10 }, BlockPosition { x: 12, y: 13, z: 11 }, BlockPosition { x: 12, y: 13, z: 12 }, BlockPosition { x: 12, y: 13, z: 13 }, BlockPosition { x: 13, y: 10, z: 9 }, BlockPosition { x: 13, y: 10, z: 10 }, BlockPosition { x: 13, y: 10, z: 11 }, BlockPosition { x: 13, y: 10, z: 12 }, BlockPosition { x: 13, y: 10, z: 13 }, BlockPosition { x: 13, y: 11, z: 9 }, BlockPosition { x: 13, y: 11, z: 10 }, BlockPosition { x: 13, y: 11, z: 11 }, BlockPosition { x: 13, y: 11, z: 12 }, BlockPosition { x: 13, y: 11, z: 13 }, BlockPosition { x: 13, y: 12, z: 9 }, BlockPosition { x: 13, y: 12, z: 10 }, BlockPosition { x: 13, y: 12, z: 11 }, BlockPosition { x: 13, y: 12, z: 12 }, BlockPosition { x: 13, y: 12, z: 13 }, BlockPosition { x: 13, y: 13, z: 9 }, BlockPosition { x: 13, y: 13, z: 10 }, BlockPosition { x: 13, y: 13, z: 11 }, BlockPosition { x: 13, y: 13, z: 12 }, BlockPosition { x: 13, y: 13, z: 13 }
      ];

      res.sort();
      expected.sort();

      assert_eq!(res, expected);
    }
  }
}
