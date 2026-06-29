use super::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommonEntity {
	pub position: EntityPosition,
	pub velocity: EntityPosition,
	pub uuid: u128,
	pub entity_id: i32,
	pub air: i16,
	pub custom_name: NbtTag,
	pub custom_name_visible: bool,
	pub data: NbtTag,
	pub fall_distance: f64,
	pub ticks_until_fire_is_out: i16,
	pub is_glowing: bool,
	pub has_visual_fire: bool,
	pub invulnerable: bool,
	pub no_gravity: bool,
	pub on_ground: bool,
	pub passengers: Vec<Entity>,
	pub portal_cooldown: i32,
	pub is_silent: bool,
	pub scoreboard_tags: Vec<NbtListTag>,
	pub ticks_frozen: i32,
	pub debug_data_pathfinding: Option<DebugEntityPath>,
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

pub trait CommonEntityTrait {
	fn new(data: CommonEntity, extra_nbt: NbtListTag) -> Self
	where
		Self: Sized;
	fn from_nbt(value: NbtListTag, entity_id_manager: &EntityIdManager) -> Self
	where
		Self: std::marker::Sized,
	{
		let mut common_data = CommonEntity {
			entity_id: entity_id_manager.get_new(),
			..Default::default()
		};

		let x = value.get_child("Pos").unwrap().as_list()[0].as_double();
		let y = value.get_child("Pos").unwrap().as_list()[1].as_double();
		let z = value.get_child("Pos").unwrap().as_list()[2].as_double();
		let yaw = value.get_child("Rotation").unwrap().as_list()[0].as_float();
		let pitch = value.get_child("Rotation").unwrap().as_list()[1].as_float();

		common_data.position = EntityPosition {
			x,
			y,
			z,
			yaw,
			pitch,
		};

		if value.get_child("Motion").is_some() {
			common_data.velocity = EntityPosition {
				x: value.get_child("Motion").unwrap().as_list()[0].as_double(),
				y: value.get_child("Motion").unwrap().as_list()[1].as_double(),
				z: value.get_child("Motion").unwrap().as_list()[2].as_double(),
				yaw,
				pitch,
			};
		}

		common_data.uuid = value
			.get_child("UUID")
			.unwrap()
			.as_int_array()
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

		return Self::new(common_data, value.clone());
	}

	fn get_common_entity_data(&self) -> &CommonEntity;
	fn get_common_entity_data_mut(&mut self) -> &mut CommonEntity;
	fn get_common_entity_data_cloned(&self) -> CommonEntity {
		return self.get_common_entity_data().clone();
	}
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

	fn tick(
		&mut self,
		dimension: &Dimension,
		players: &[Player],
		packet_sender: &PacketSender,
		entity_id_manager: &EntityIdManager,
		block_state_data: &HashMap<String, basic_types::blocks::Block>,
	) -> Vec<EntityTickOutcome> {
		let mut output: Vec<EntityTickOutcome> = Vec::new();

		if self.is_mob() {
			let mob_data = self.get_mob_data_mut();

			if mob_data.death_time == 20 {
				return vec![EntityTickOutcome::RemoveSelf];
			}

			if mob_data.death_time > 0 {
				mob_data.death_time += 1;
				return Vec::new();
			}

			mob_data.alive_for_ticks += 1;

			if mob_data.hurt_time > 0 {
				mob_data.hurt_time -= 1;
			}

			if mob_data.health <= 0.0 {
				mob_data.death_time = 1;
				return vec![EntityTickOutcome::SelfDied];
			}
		}


		let old_position = self.get_common_entity_data().position;

		if !(self.is_mob() && self.get_mob_data().hurt_time != 0) {
			if self.is_on_ground(dimension) {
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
		let (ai_result, mut ai_tick_outcome) = self.execute_ai(players, dimension);
		match ai_result {
			AiExecutionResult::DoNothing => (),
			AiExecutionResult::ApplyVelocity(x) => velocity_from_ai = x,
		};
		output.append(&mut ai_tick_outcome);

		let mut velocity = self.get_common_entity_data().velocity;
		velocity += velocity_from_ai;

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

			if self.collides_with_blocks_at(dimension, entity_position_to_check) {
				velocity = last_velocity;

				//Check if jumping would help
				if self.is_on_ground(dimension)
					&& !self.collides_with_blocks_at(
						dimension,
						EntityPosition {
							y: entity_position_to_check.y + 1.0,
							..entity_position_to_check
						},
					) {
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
		if self.is_on_ground_at(dimension, next_position) {
			next_position.y = next_position.y.round();
		}

		self.get_common_entity_data_mut().position = next_position;

		if old_position != self.get_common_entity_data().position {
			let packet = crate::packets::clientbound::play::UpdateEntityPosition {
				entity_id: self.get_common_entity_data().entity_id,
				delta_x: ((self.get_common_entity_data().position.x * 4096.0) - (old_position.x * 4096.0)) as i16,
				delta_y: ((self.get_common_entity_data().position.y * 4096.0) - (old_position.y * 4096.0)) as i16,
				delta_z: ((self.get_common_entity_data().position.z * 4096.0) - (old_position.z * 4096.0)) as i16,
				on_ground: self.is_on_ground(dimension),
			};

			packet_sender.send_packet_to_everyone_in_dimension(
				players,
				&dimension.name,
				crate::packets::clientbound::play::UpdateEntityPosition::PACKET_ID,
				packet,
			);

			output.push(EntityTickOutcome::Updated);
		}

		output.append(&mut self.extra_tick(dimension, players, packet_sender, entity_id_manager, block_state_data));

		return output;
	}

	fn extra_tick(
		&mut self,
		_dimension: &Dimension,
		_players: &[Player],
		_packet_sender: &PacketSender,
		_entity_id_manager: &EntityIdManager,
		_block_state_data: &HashMap<String, basic_types::blocks::Block>,
	) -> Vec<EntityTickOutcome> {
		return Vec::new();
	}

	fn collides_with_blocks_at(&self, dimension: &Dimension, position_to_check: EntityPosition) -> bool {
		let positions_to_check = self.get_occupied_block_positions_at_entity_position(position_to_check);

		for position_to_check in positions_to_check {
			let block_at_location = dimension.get_block(position_to_check).unwrap_or(0);
			let block_type_at_location = data::blocks::get_type_from_block_state_id(block_at_location);
			if !block_type_at_location.has_no_collision_box() {
				return true;
			}
		}

		return false;
	}

	fn is_on_ground(&self, dimension: &Dimension) -> bool {
		return self.is_on_ground_at(dimension, self.get_common_entity_data().position);
	}

	fn is_on_ground_at(&self, dimension: &Dimension, mut position_to_check: EntityPosition) -> bool {
		position_to_check.y -= 0.1;

		let positions_to_check = self.get_occupied_block_positions_at_entity_position(position_to_check);

		for position_to_check in positions_to_check {
			let block_at_location = dimension.get_block(position_to_check).unwrap_or(0);
			let block_type_at_location = data::blocks::get_type_from_block_state_id(block_at_location);
			if !block_type_at_location.has_no_collision_box() {
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
		//seems like the center off the hitbox is offset by half a block from the entity position
		let x_offset = if position_to_check.x.abs() < 1.0 {
			0.0
		} else if position_to_check.x.is_sign_positive() {
			0.5
		} else {
			-0.5
		};
		let z_offset = if position_to_check.z.abs() < 1.0 {
			0.0
		} else if position_to_check.z.is_sign_positive() {
			0.5
		} else {
			-0.5
		};

		let x_min = position_to_check.x + x_offset - (self.get_hitbox().1 * 0.5);
		let x_max = position_to_check.x + x_offset + (self.get_hitbox().1 * 0.5);
		let x_range = (if x_min.is_sign_positive() { x_min.floor() } else { x_min.ceil() } as i32)..=(if x_max.is_sign_positive() {
			x_max.floor()
		} else {
			x_max.ceil()
		} as i32);
		let y_min = position_to_check.y;
		let y_max = position_to_check.y + self.get_hitbox().0 - 0.01;
		let y_range = (if y_min.is_sign_positive() { y_min.floor() } else { y_min.ceil() } as i16)..=(if y_max.is_sign_positive() {
			y_max.floor()
		} else {
			y_max.ceil()
		} as i16);
		let z_min = position_to_check.z + z_offset - (self.get_hitbox().1 * 0.5);
		let z_max = position_to_check.z + z_offset + (self.get_hitbox().1 * 0.5);
		let z_range = (if z_min.is_sign_positive() { z_min.floor() } else { z_min.ceil() } as i32)..=(if z_max.is_sign_positive() {
			z_max.floor()
		} else {
			z_max.ceil()
		} as i32);

		let mut output: Vec<BlockPosition> = Vec::new();

		for x in x_range.clone() {
			for y in y_range.clone() {
				for z in z_range.clone() {
					output.push(BlockPosition {
						x,
						y,
						z,
					});
				}
			}
		}

		return output;
	}

	fn execute_ai(&mut self, players: &[Player], dimension: &Dimension) -> (AiExecutionResult, Vec<EntityTickOutcome>) {
		if self.is_mob() && self.get_mob_data().has_no_ai {
			return (AiExecutionResult::DoNothing, Vec::new());
		}
		let entity_type = data::entities::get_name_from_id(self.get_type());
		let behavior = if entity_type.as_str() == "minecraft:creeper" {
			AiBehavior::MoveTowardsPlayer
		} else if self.is_mob() {
			AiBehavior::Wander
		} else {
			AiBehavior::Idle
		};

		return match behavior {
			AiBehavior::Idle => (AiExecutionResult::DoNothing, Vec::new()),
			AiBehavior::MoveTowardsPlayer => self.execute_ai_move_towards_player(players, dimension),
			AiBehavior::Wander => (self.execute_ai_wander(), Vec::new()),
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

	fn execute_ai_move_towards_player(&self, players: &[Player], dimension: &Dimension) -> (AiExecutionResult, Vec<EntityTickOutcome>) {
		let mut player_distances = players
			.iter()
			.map(|x| (x, self.get_common_entity_data().position.distance_to(x.get_position())))
			.filter(|x| x.1 < 25.0)
			.collect::<Vec<(&Player, f64)>>();

		player_distances.sort_by(|a, b| a.1.total_cmp(&b.1));
		let closest_player = player_distances.first();

		if let Some(closest_player) = closest_player {
			let ai_move_result = self.ai_move_towards_goal(closest_player.0.get_position(), dimension);
			return (AiExecutionResult::ApplyVelocity(ai_move_result.0), ai_move_result.1);
		} else {
			return (AiExecutionResult::DoNothing, Vec::new());
		}
	}

	fn ai_move_towards_goal(&self, goal: EntityPosition, dimension: &Dimension) -> (EntityPosition, Vec<EntityTickOutcome>) {
		let distance_from_start_to_goal = self.get_common_entity_data().position.distance_to(goal);
		let goal_block = BlockPosition::from(goal);
		let starting_point = BlockPosition::from(self.get_common_entity_data().position);
		//println!("starting on block: {starting_point:?}");
		let mut open: Vec<(BlockPosition, f64, BlockPosition)> = vec![(starting_point, 0.0, starting_point)]; //own pos, cost, parent pos
		let mut closed: Vec<(BlockPosition, f64, BlockPosition)> = Vec::new(); //own pos, parent pos

		for _ in 0..100 {
			if open.is_empty() {
				println!("open was empty, aborting");
				break;
			}
			let mut lowest_cost_index = 0;
			let mut lowest_cost = f64::INFINITY;
			for (i, (_, cost, _)) in open.iter().enumerate() {
				if *cost < lowest_cost {
					lowest_cost = *cost;
					lowest_cost_index = i;
				}
			}
			let (node, cost, parent) = open.remove(lowest_cost_index);
			//println!("node: {node:?}, cost: {cost}, goal: {goal_block:?}");
			if node == goal_block {
				//println!("reached goal, tracing backwards...");
				let mut current_node = node;
				let mut current_cost = cost;
				let mut current_parent = parent;
				let mut last_node = node;
				let mut nodes: Vec<(BlockPosition, f64)> = Vec::new();
				loop {
					nodes.push((current_node, current_cost));
					//println!("current_node: {current_node:?} current_parent: {current_parent:?} last_node: {last_node:?}");
					if current_node == starting_point {
						//println!("next block towards goal: {last_node:?}");

						//Update debug data
						let debug_pathfinding_data = {
							let mut lowest_cost_index = 0;
							let mut lowest_cost = f64::INFINITY;
							for (i, (_, cost, _)) in open.iter().enumerate() {
								if *cost < lowest_cost {
									lowest_cost = *cost;
									lowest_cost_index = i;
								}
							}

							DebugEntityPath {
								reached: true,
								next_block_index: lowest_cost_index as i32,
								block_position: self.get_common_entity_data().position.into(),
								nodes: nodes
									.iter()
									.map(|(node, cost)| DebugPathNode {
										x: node.x,
										y: node.y as i32,
										z: node.z,
										walked_distance: *cost as f32,
										cost_malus: 0.0,
										closed: true,
										node_type: 0,
										f: 0.0,
									})
									.collect(),
								target_nodes: vec![DebugPathNode {
									x: goal_block.x,
									y: goal_block.y as i32,
									z: goal_block.z,
									walked_distance: 0.0,
									cost_malus: 0.0,
									closed: false,
									node_type: 0,
									f: 0.0,
								}],
								open_set: open
									.iter()
									.map(|x| DebugPathNode {
										x: x.0.x,
										y: x.0.y as i32,
										z: x.0.z,
										walked_distance: x.1 as f32,
										cost_malus: 0.0,
										closed: false,
										node_type: 0,
										f: 0.0,
									})
									.collect(),
								closed_set: closed
									.iter()
									.map(|x| DebugPathNode {
										x: x.0.x,
										y: x.0.y as i32,
										z: x.0.z,
										walked_distance: x.1 as f32,
										cost_malus: 0.0,
										closed: true,
										node_type: 0,
										f: 0.0,
									})
									.collect(),
								max_node_distance: 0.0,
							}
						};
						// END Update debug data

						let speed = 0.075;
						return (
							EntityPosition {
								x: (last_node.x - starting_point.x) as f64 * speed,
								y: 0.0,
								z: (last_node.z - starting_point.z) as f64 * speed,
								yaw: 0.0,
								pitch: 0.0,
							},
							vec![EntityTickOutcome::UpdateDebugDataPathfinding(Some(debug_pathfinding_data))],
						);
					}

					let (next_node, next_cost, next_parent) = closed.iter().find(|(node, _, _)| *node == current_parent).unwrap();
					last_node = current_node;
					current_node = *next_node;
					current_cost = *next_cost;
					current_parent = *next_parent;
				}
			}

			closed.push((node, cost, parent));

			let neighbours = [
				BlockPosition {
					x: node.x - 1,
					y: node.y,
					z: node.z - 1,
				},
				BlockPosition {
					x: node.x - 1,
					y: node.y,
					z: node.z,
				},
				BlockPosition {
					x: node.x - 1,
					y: node.y,
					z: node.z + 1,
				},
				BlockPosition {
					x: node.x,
					y: node.y,
					z: node.z - 1,
				},
				BlockPosition {
					x: node.x,
					y: node.y,
					z: node.z + 1,
				},
				BlockPosition {
					x: node.x + 1,
					y: node.y,
					z: node.z - 1,
				},
				BlockPosition {
					x: node.x + 1,
					y: node.y,
					z: node.z,
				},
				BlockPosition {
					x: node.x + 1,
					y: node.y,
					z: node.z + 1,
				},
			];

			for neighbour in neighbours {
				if closed.iter().any(|x| x.0 == neighbour) {
					continue;
				}
				//TODO: more sophisticated check if block is valid to step on
				let neighbour_block = dimension.get_block(neighbour);
				if neighbour_block.is_err() || neighbour_block.unwrap() != 0 {
					continue;
				}

				let distance_to_goal = EntityPosition::from(neighbour).distance_to(goal);
				let cost = distance_to_goal - distance_from_start_to_goal;
				if let Some(node) = open.iter_mut().find(|x| x.0 == neighbour && x.1 > cost) {
					//We found a lower cost way to get here
					node.1 = cost;
				} else if !open.iter().any(|x| x.0 == neighbour && x.1 >= cost) {
					open.push((neighbour, cost, node));
				}
			}
		}

		println!("didnt find way to goal, aborting");

		return (EntityPosition::default(), vec![EntityTickOutcome::UpdateDebugDataPathfinding(None)]);
	}

	fn to_nbt_extras(&self) -> Vec<NbtTag>;

	fn to_spawn_entity_packet(&self) -> crate::packets::clientbound::play::SpawnEntity {
		return crate::packets::clientbound::play::SpawnEntity {
			entity_id: self.get_common_entity_data().entity_id,
			entity_uuid: self.get_common_entity_data().uuid,
			entity_type: self.get_type(),
			x: self.get_common_entity_data().position.x,
			y: self.get_common_entity_data().position.y,
			z: self.get_common_entity_data().position.z,
			pitch: self.get_pitch_u8(),
			yaw: self.get_yaw_u8(),
			head_yaw: 0,
			data: 0,
			velocity_x: 0,
			velocity_y: 0,
			velocity_z: 0,
		};
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

	fn damage(&mut self, damage: f32, _packet_sender: &PacketSender, _players: &[Player]) {
		if self.is_mob() {
			self.get_mob_data_mut().health -= damage;
		}
	}

	fn is_in_liquid(&self, dimension: &Dimension) -> bool {
		//need to use self.get_common_entity_data_cloned() because Player doesnt implement self.get_common_entity_data()
		let block_at_pos = dimension.get_block(self.get_common_entity_data_cloned().position.into()).unwrap_or_default();
		return data::blocks::get_type_from_block_state_id(block_at_pos) == basic_types::blocks::Type::Liquid;
	}

	//returns true if feeding was successfull, to signal to caller that players inventory needs updating
	fn feed(&mut self, _held_item: &Slot, _packet_sender: &PacketSender, _players_clone: &[Player], _dimension_name: &str) -> bool {
		return false;
	}

	fn interact(
		&mut self,
		_held_item: &Slot,
		_dimension: &mut Dimension,
		_players_clone: &[Player],
		_players: &mut [Player],
		_player_uuid: u128,
		_packet_sender: &PacketSender,
		_entity_id_manager: &EntityIdManager,
		_block_state_data: &HashMap<String, basic_types::blocks::Block>,
	) -> Vec<EntityTickOutcome> {
		return Vec::new();
	}

	fn resend_metadata_to_players(&self, players_clone: &[Player], packet_sender: &PacketSender, dimension_name: &str) {
		let metadata_packet = crate::packets::clientbound::play::SetEntityMetadata {
			entity_id: self.get_common_entity_data().entity_id,
			metadata: self.get_metadata(),
		};

		packet_sender.send_packet_to_everyone_in_dimension(
			players_clone,
			dimension_name,
			crate::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
			metadata_packet,
		);
	}

	fn change_dimension(
		&mut self,
		_new_dimension_name: &str,
		_players_clone: &[Player],
		_dimension: &mut Dimension,
		_packet_sender: &PacketSender,
		_position: BlockPosition,
	) {
		return;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	mod ai_move_towards_goal {
		use super::*;

		#[test]
		fn default_chunk_towards_pos_x() {
			let creeper = Creeper::new(
				CommonEntity {
					position: EntityPosition {
						x: 0.0,
						y: 16.0,
						z: 0.0,
						yaw: 0.0,
						pitch: 0.0,
					},
					..Default::default()
				},
				NbtListTag::default(),
			);
			let dimension = Dimension::new("oxide:test");
			let goal = EntityPosition {
				x: 10.0,
				y: 16.0,
				z: 0.0,
				yaw: 0.0,
				pitch: 0.0,
			};

			let res = creeper.ai_move_towards_goal(goal, &dimension);
			println!("{res:?}");
			assert!(res.0.x > 0.07 && res.0.x < 0.08);
		}

		#[test]
		fn default_chunk_towards_neg_x() {
			let creeper = Creeper::new(
				CommonEntity {
					position: EntityPosition {
						x: 0.0,
						y: 16.0,
						z: 0.0,
						yaw: 0.0,
						pitch: 0.0,
					},
					..Default::default()
				},
				NbtListTag::default(),
			);
			let dimension = Dimension::new("oxide:test");
			let goal = EntityPosition {
				x: -10.0,
				y: 16.0,
				z: 0.0,
				yaw: 0.0,
				pitch: 0.0,
			};

			let res = creeper.ai_move_towards_goal(goal, &dimension);
			println!("{res:?}");
			assert!(res.0.x < -0.07 && res.0.x > -0.08);
		}

		#[test]
		fn default_chunk_towards_pos_z() {
			let creeper = Creeper::new(
				CommonEntity {
					position: EntityPosition {
						x: 0.0,
						y: 16.0,
						z: 0.0,
						yaw: 0.0,
						pitch: 0.0,
					},
					..Default::default()
				},
				NbtListTag::default(),
			);
			let dimension = Dimension::new("oxide:test");
			let goal = EntityPosition {
				x: 0.0,
				y: 16.0,
				z: 10.0,
				yaw: 0.0,
				pitch: 0.0,
			};

			let res = creeper.ai_move_towards_goal(goal, &dimension);
			println!("{res:?}");
			assert!(res.0.z > 0.07 && res.0.z < 0.08);
		}

		#[test]
		fn default_chunk_towards_neg_z() {
			let creeper = Creeper::new(
				CommonEntity {
					position: EntityPosition {
						x: 0.0,
						y: 16.0,
						z: 0.0,
						yaw: 0.0,
						pitch: 0.0,
					},
					..Default::default()
				},
				NbtListTag::default(),
			);
			let dimension = Dimension::new("oxide:test");
			let goal = EntityPosition {
				x: 0.0,
				y: 16.0,
				z: -10.0,
				yaw: 0.0,
				pitch: 0.0,
			};

			let res = creeper.ai_move_towards_goal(goal, &dimension);
			println!("{res:?}");
			assert!(res.0.z < -0.07 && res.0.z > -0.08);
		}
	}

	#[test]
	fn default_chunk_towards_pos_x_and_z() {
		let creeper = Creeper::new(
			CommonEntity {
				position: EntityPosition {
					x: 0.0,
					y: 16.0,
					z: 0.0,
					yaw: 0.0,
					pitch: 0.0,
				},
				..Default::default()
			},
			NbtListTag::default(),
		);
		let dimension = Dimension::new("oxide:test");
		let goal = EntityPosition {
			x: 10.0,
			y: 16.0,
			z: 10.0,
			yaw: 0.0,
			pitch: 0.0,
		};

		let res = creeper.ai_move_towards_goal(goal, &dimension);
		println!("{res:?}");
		assert!(res.0.x > 0.07 && res.0.x < 0.08);
		assert!(res.0.z > 0.07 && res.0.z < 0.08);
	}

	#[test]
	fn obstacle_towards_pos_x() {
		let creeper = Creeper::new(
			CommonEntity {
				position: EntityPosition {
					x: 0.0,
					y: 16.0,
					z: 0.0,
					yaw: 0.0,
					pitch: 0.0,
				},
				..Default::default()
			},
			NbtListTag::default(),
		);
		let mut dimension = Dimension::new("oxide:test");
		dimension
			.overwrite_block(
				BlockPosition {
					x: 1,
					y: 16,
					z: 0,
				},
				1,
			)
			.unwrap();

		let goal = EntityPosition {
			x: 10.0,
			y: 16.0,
			z: 0.0,
			yaw: 0.0,
			pitch: 0.0,
		};

		let res = creeper.ai_move_towards_goal(goal, &dimension);
		println!("{res:?}");
		assert!(res.0.x < 0.1);
		assert!((res.0.z > 0.07 && res.0.z < 0.08) || (res.0.z < -0.07 && res.0.z > -0.08));
	}

	#[test]
	fn labyrinth() {
		let mut creeper = Creeper::new(
			CommonEntity {
				position: EntityPosition {
					x: OBSTACLE_COURSE_START.x as f64 + 0.5,
					y: OBSTACLE_COURSE_START.y as f64,
					z: OBSTACLE_COURSE_START.z as f64 + 0.5,
					yaw: 0.0,
					pitch: 0.0,
				},
				..Default::default()
			},
			NbtListTag::default(),
		);
		let mut dimension = Dimension::new("oxide:test");
		for block in OBSTACLE_COURSE_BLOCKS {
			dimension.overwrite_block(block, 1).unwrap();
		}

		let goal = EntityPosition {
			x: OBSTACLE_COURSE_GOAL.x as f64,
			y: OBSTACLE_COURSE_GOAL.y as f64,
			z: OBSTACLE_COURSE_GOAL.z as f64,
			yaw: 0.0,
			pitch: 0.0,
		};

		let mut reached_goal = false;
		for _ in 0..1000 {
			let res = creeper.ai_move_towards_goal(goal, &dimension);
			creeper.get_common_entity_data_mut().position += res.0;

			if BlockPosition::from(creeper.get_common_entity_data().position) == BlockPosition::from(goal) {
				reached_goal = true;
				break;
			}
		}

		assert!(reached_goal);
	}

	const OBSTACLE_COURSE_START: BlockPosition = BlockPosition {
		x: 0,
		y: 16,
		z: 0,
	};
	const OBSTACLE_COURSE_GOAL: BlockPosition = BlockPosition {
		x: -9,
		y: 16,
		z: -11,
	};
	#[rustfmt::skip]
	const OBSTACLE_COURSE_BLOCKS: [BlockPosition; 216] = [
    BlockPosition {x: -6, y: 16, z: -17},
    BlockPosition {x: -5, y: 16, z: -17},
    BlockPosition {x: -4, y: 16, z: -17},
    BlockPosition {x: -14, y: 16, z: -16},
    BlockPosition {x: -13, y: 16, z: -16},
    BlockPosition {x: -12, y: 16, z: -16},
    BlockPosition {x: -11, y: 16, z: -16},
    BlockPosition {x: -10, y: 16, z: -16},
    BlockPosition {x: -9, y: 16, z: -16},
    BlockPosition {x: -8, y: 16, z: -16},
    BlockPosition {x: -7, y: 16, z: -16},
    BlockPosition {x: -6, y: 16, z: -16},
    BlockPosition {x: -4, y: 16, z: -16},
    BlockPosition {x: -15, y: 16, z: -15},
    BlockPosition {x: -14, y: 16, z: -15},
    BlockPosition {x: -4, y: 16, z: -15},
    BlockPosition {x: -3, y: 16, z: -15},
    BlockPosition {x: -15, y: 16, z: -14},
    BlockPosition {x: -12, y: 16, z: -14},
    BlockPosition {x: -11, y: 16, z: -14},
    BlockPosition {x: -10, y: 16, z: -14},
    BlockPosition {x: -9, y: 16, z: -14},
    BlockPosition {x: -8, y: 16, z: -14},
    BlockPosition {x: -7, y: 16, z: -14},
    BlockPosition {x: -6, y: 16, z: -14},
    BlockPosition {x: -3, y: 16, z: -14},
    BlockPosition {x: -3, y: 16, z: -14},
    BlockPosition {x: -17, y: 16, z: -13},
    BlockPosition {x: -16, y: 16, z: -13},
    BlockPosition {x: -15, y: 16, z: -13},
    BlockPosition {x: -13, y: 16, z: -13},
    BlockPosition {x: -12, y: 16, z: -13},
    BlockPosition {x: -6, y: 16, z: -13},
    BlockPosition {x: -5, y: 16, z: -13},
    BlockPosition {x: -3, y: 16, z: -13},
    BlockPosition {x: -2, y: 16, z: -13},
    BlockPosition {x: -1, y: 16, z: -13},
    BlockPosition {x: 0, y: 16, z: -13},
    BlockPosition {x: 1, y: 16, z: -13},
    BlockPosition {x: 2, y: 16, z: -13},
    BlockPosition {x: 3, y: 16, z: -13},
    BlockPosition {x: 4, y: 16, z: -13},
    BlockPosition {x: 5, y: 16, z: -13},
    BlockPosition {x: 6, y: 16, z: -13},
    BlockPosition {x: 7, y: 16, z: -13},
    BlockPosition {x: 8, y: 16, z: -13},
    BlockPosition {x: 9, y: 16, z: -13},
    BlockPosition {x: -17, y: 16, z: -12},
    BlockPosition {x: -15, y: 16, z: -12},
    BlockPosition {x: -13, y: 16, z: -12},
    BlockPosition {x: -12, y: 16, z: -12},
    BlockPosition {x: -10, y: 16, z: -12},
    BlockPosition {x: -8, y: 16, z: -12},
    BlockPosition {x: -5, y: 16, z: -12},
    BlockPosition {x: 6, y: 16, z: -12},
    BlockPosition {x: 9, y: 16, z: -12},
    BlockPosition {x: -17, y: 16, z: -11},
    BlockPosition {x: -10, y: 16, z: -11},
    BlockPosition {x: -8, y: 16, z: -11},
    BlockPosition {x: -5, y: 16, z: -11},
    BlockPosition {x: -4, y: 16, z: -11},
    BlockPosition {x: -2, y: 16, z: -11},
    BlockPosition {x: -1, y: 16, z: -11},
    BlockPosition {x: 0, y: 16, z: -11},
    BlockPosition {x: 1, y: 16, z: -11},
    BlockPosition {x: 2, y: 16, z: -11},
    BlockPosition {x: 3, y: 16, z: -11},
    BlockPosition {x: 4, y: 16, z: -11},
    BlockPosition {x: 6, y: 16, z: -11},
    BlockPosition {x: 7, y: 16, z: -11},
    BlockPosition {x: 9, y: 16, z: -11},
    BlockPosition {x: -17, y: 16, z: -10},
    BlockPosition {x: -15, y: 16, z: -10},
    BlockPosition {x: -14, y: 16, z: -10},
    BlockPosition {x: -13, y: 16, z: -10},
    BlockPosition {x: -12, y: 16, z: -10},
    BlockPosition {x: -10, y: 16, z: -10},
    BlockPosition {x: -9, y: 16, z: -10},
    BlockPosition {x: -8, y: 16, z: -10},
    BlockPosition {x: -5, y: 16, z: -10},
    BlockPosition {x: -2, y: 16, z: -10},
    BlockPosition {x: 9, y: 16, z: -10},
    BlockPosition {x: -17, y: 16, z: -9},
    BlockPosition {x: -5, y: 16, z: -9},
    BlockPosition {x: -4, y: 16, z: -9},
    BlockPosition {x: -3, y: 16, z: -9},
    BlockPosition {x: -2, y: 16, z: -9},
    BlockPosition {x: 6, y: 16, z: -9},
    BlockPosition {x: 9, y: 16, z: -9},
    BlockPosition {x: -17, y: 16, z: -8},
    BlockPosition {x: -16, y: 16, z: -8},
    BlockPosition {x: -15, y: 16, z: -8},
    BlockPosition {x: -14, y: 16, z: -8},
    BlockPosition {x: -13, y: 16, z: -8},
    BlockPosition {x: -12, y: 16, z: -8},
    BlockPosition {x: -11, y: 16, z: -8},
    BlockPosition {x: -10, y: 16, z: -8},
    BlockPosition {x: -9, y: 16, z: -8},
    BlockPosition {x: -8, y: 16, z: -8},
    BlockPosition {x: -7, y: 16, z: -8},
    BlockPosition {x: -6, y: 16, z: -8},
    BlockPosition {x: -5, y: 16, z: -8},
    BlockPosition {x: -2, y: 16, z: -8},
    BlockPosition {x: 0, y: 16, z: -8},
    BlockPosition {x: 1, y: 16, z: -8},
    BlockPosition {x: 2, y: 16, z: -8},
    BlockPosition {x: 3, y: 16, z: -8},
    BlockPosition {x: 4, y: 16, z: -8},
    BlockPosition {x: 6, y: 16, z: -8},
    BlockPosition {x: 7, y: 16, z: -8},
    BlockPosition {x: 9, y: 16, z: -8},
    BlockPosition {x: -5, y: 16, z: -7},
    BlockPosition {x: -4, y: 16, z: -7},
    BlockPosition {x: -2, y: 16, z: -7},
    BlockPosition {x: 0, y: 16, z: -7},
    BlockPosition {x: 4, y: 16, z: -7},
    BlockPosition {x: 6, y: 16, z: -7},
    BlockPosition {x: 9, y: 16, z: -7},
    BlockPosition {x: -5, y: 16, z: -6},
    BlockPosition {x: -4, y: 16, z: -6},
    BlockPosition {x: -2, y: 16, z: -6},
    BlockPosition {x: -1, y: 16, z: -6},
    BlockPosition {x: 0, y: 16, z: -6},
    BlockPosition {x: 1, y: 16, z: -6},
    BlockPosition {x: 2, y: 16, z: -6},
    BlockPosition {x: 4, y: 16, z: -6},
    BlockPosition {x: 6, y: 16, z: -6},
    BlockPosition {x: 8, y: 16, z: -6},
    BlockPosition {x: 9, y: 16, z: -6},
    BlockPosition {x: -5, y: 16, z: -5},
    BlockPosition {x: -4, y: 16, z: -5},
    BlockPosition {x: -2, y: 16, z: -5},
    BlockPosition {x: -1, y: 16, z: -5},
    BlockPosition {x: 6, y: 16, z: -5},
    BlockPosition {x: 9, y: 16, z: -5},
    BlockPosition {x: -5, y: 16, z: -4},
    BlockPosition {x: -2, y: 16, z: -4},
    BlockPosition {x: -1, y: 16, z: -4},
    BlockPosition {x: 1, y: 16, z: -4},
    BlockPosition {x: 3, y: 16, z: -4},
    BlockPosition {x: 4, y: 16, z: -4},
    BlockPosition {x: 5, y: 16, z: -4},
    BlockPosition {x: 6, y: 16, z: -4},
    BlockPosition {x: 7, y: 16, z: -4},
    BlockPosition {x: 10, y: 16, z: -4},
    BlockPosition {x: 11, y: 16, z: -4},
    BlockPosition {x: -5, y: 16, z: -3},
    BlockPosition {x: -5, y: 16, z: -3},
    BlockPosition {x: -3, y: 16, z: -3},
    BlockPosition {x: -2, y: 16, z: -3},
    BlockPosition {x: -1, y: 16, z: -3},
    BlockPosition {x: 1, y: 16, z: -3},
    BlockPosition {x: 2, y: 16, z: -3},
    BlockPosition {x: 3, y: 16, z: -3},
    BlockPosition {x: 7, y: 16, z: -3},
    BlockPosition {x: 8, y: 16, z: -3},
    BlockPosition {x: 11, y: 16, z: -3},
    BlockPosition {x: -5, y: 16, z: -2},
    BlockPosition {x: -2, y: 16, z: -2},
    BlockPosition {x: 8, y: 16, z: -2},
    BlockPosition {x: 9, y: 16, z: -2},
    BlockPosition {x: 11, y: 16, z: -2},
    BlockPosition {x: -5, y: 16, z: -1},
    BlockPosition {x: -1, y: 16, z: -1},
    BlockPosition {x: 0, y: 16, z: -1},
    BlockPosition {x: 1, y: 16, z: -1},
    BlockPosition {x: 3, y: 16, z: -1},
    BlockPosition {x: 4, y: 16, z: -1},
    BlockPosition {x: 6, y: 16, z: -1},
    BlockPosition {x: 9, y: 16, z: -1},
    BlockPosition {x: 11, y: 16, z: -1},
    BlockPosition {x: -5, y: 16, z: 0},
    BlockPosition {x: -4, y: 16, z: 0},
    BlockPosition {x: -3, y: 16, z: 0},
    BlockPosition {x: -1, y: 16, z: 0},
    BlockPosition {x: 1, y: 16, z: 0},
    BlockPosition {x: 6, y: 16, z: 0},
    BlockPosition {x: 7, y: 16, z: 0},
    BlockPosition {x: 11, y: 16, z: 0},
    BlockPosition {x: -3, y: 16, z: 1},
    BlockPosition {x: -1, y: 16, z: 1},
    BlockPosition {x: 1, y: 16, z: 1},
    BlockPosition {x: 3, y: 16, z: 1},
    BlockPosition {x: 4, y: 16, z: 1},
    BlockPosition {x: 5, y: 16, z: 1},
    BlockPosition {x: 6, y: 16, z: 1},
    BlockPosition {x: 7, y: 16, z: 1},
    BlockPosition {x: 8, y: 16, z: 1},
    BlockPosition {x: 9, y: 16, z: 1},
    BlockPosition {x: 10, y: 16, z: 1},
    BlockPosition {x: 11, y: 16, z: 1},
    BlockPosition {x: -3, y: 16, z: 2},
    BlockPosition {x: 3, y: 16, z: 2},
    BlockPosition {x: -3, y: 16, z: 3},
    BlockPosition {x: -1, y: 16, z: 3},
    BlockPosition {x: 0, y: 16, z: 3},
    BlockPosition {x: 1, y: 16, z: 3},
    BlockPosition {x: 2, y: 16, z: 3},
    BlockPosition {x: 3, y: 16, z: 3},
    BlockPosition {x: -3, y: 16, z: 4},
    BlockPosition {x: 3, y: 16, z: 4},
    BlockPosition {x: -3, y: 16, z: 5},
    BlockPosition {x: -2, y: 16, z: 5},
    BlockPosition {x: -1, y: 16, z: 5},
    BlockPosition {x: 0, y: 16, z: 5},
    BlockPosition {x: 1, y: 16, z: 5},
    BlockPosition {x: 3, y: 16, z: 5},
    BlockPosition {x: -3, y: 16, z: 6},
    BlockPosition {x: 3, y: 16, z: 6},
    BlockPosition {x: -3, y: 16, z: 7},
    BlockPosition {x: -2, y: 16, z: 7},
    BlockPosition {x: -1, y: 16, z: 7},
    BlockPosition {x: 0, y: 16, z: 7},
    BlockPosition {x: 1, y: 16, z: 7},
    BlockPosition {x: 2, y: 16, z: 7},
    BlockPosition {x: 3, y: 16, z: 7},
	];
}
