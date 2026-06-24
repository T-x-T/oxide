use crate::packets::clientbound::play::DebugEntityValue;

use super::*;

pub fn get_packets_for_player(dimension: &Dimension) -> Vec<crate::packets::clientbound::play::DebugEntityValue> {
	let events: Vec<DebugEntityValue> = dimension
		.entities
		.iter()
		.filter(|x| x.is_mob())
		.filter(|x| x.get_common_entity_data().debug_data_pathfinding.is_some())
		.map(|entity| DebugEntityValue {
			entity_id: entity.get_common_entity_data().entity_id,
			update: Some(DebugSubscriptionData::EntityPath(entity.get_common_entity_data().debug_data_pathfinding.clone().unwrap())),
		})
		.collect();

	return events;
}

#[derive(Debug, Clone)]
pub enum DebugSubscriptionData {
	DedicatedServerTickTime,
	Bee,
	VillagerBrain,
	Breeze,
	GoalSelector,
	EntityPath(DebugEntityPath),
	EntityBlockIntersection,
	BeeHive,
	Poi,
	RedstoneWireOrientation,
	VillagerSection,
	Raid,
	Structure,
	GameEventListener,
	NeighborUpdate,
	GameEvent,
}

impl TryFrom<Vec<u8>> for DebugSubscriptionData {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Self::Error> {
		if value.is_empty() {
			return Err(Box::new(crate::CustomError::InputEmpty));
		}
		let debug_subscription_type = value.remove(0);
		return match debug_subscription_type {
			5 => Ok(Self::EntityPath(value.try_into()?)),
			_ => Err(Box::new(crate::CustomError::InvalidInput(format!("unknown debug subscription type {debug_subscription_type}")))),
		};
	}
}

impl From<DebugSubscriptionData> for Vec<u8> {
	fn from(value: DebugSubscriptionData) -> Self {
		return match value {
			DebugSubscriptionData::DedicatedServerTickTime => todo!(),
			DebugSubscriptionData::Bee => todo!(),
			DebugSubscriptionData::VillagerBrain => todo!(),
			DebugSubscriptionData::Breeze => todo!(),
			DebugSubscriptionData::GoalSelector => todo!(),
			DebugSubscriptionData::EntityPath(x) => vec![vec![5], x.into()].into_iter().flatten().collect(),
			DebugSubscriptionData::EntityBlockIntersection => todo!(),
			DebugSubscriptionData::BeeHive => todo!(),
			DebugSubscriptionData::Poi => todo!(),
			DebugSubscriptionData::RedstoneWireOrientation => todo!(),
			DebugSubscriptionData::VillagerSection => todo!(),
			DebugSubscriptionData::Raid => todo!(),
			DebugSubscriptionData::Structure => todo!(),
			DebugSubscriptionData::GameEventListener => todo!(),
			DebugSubscriptionData::NeighborUpdate => todo!(),
			DebugSubscriptionData::GameEvent => todo!(),
		};
	}
}


#[derive(Debug, Clone, PartialEq)]
pub struct DebugPathNode {
	pub x: i32,
	pub y: i32,
	pub z: i32,
	pub walked_distance: f32,
	pub cost_malus: f32,
	pub closed: bool,
	pub node_type: u8,
	pub f: f32,
}

impl TryFrom<&mut Vec<u8>> for DebugPathNode {
	type Error = Box<dyn Error>;

	fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
		return Ok(DebugPathNode {
			x: crate::deserialize::int(value)?,
			y: crate::deserialize::int(value)?,
			z: crate::deserialize::int(value)?,
			walked_distance: crate::deserialize::float(value)?,
			cost_malus: crate::deserialize::float(value)?,
			closed: crate::deserialize::boolean(value)?,
			node_type: value.remove(0),
			f: crate::deserialize::float(value)?,
		});
	}
}
impl TryFrom<Vec<u8>> for DebugPathNode {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Self::Error> {
		return (&mut value).try_into();
	}
}

impl From<DebugPathNode> for Vec<u8> {
	fn from(value: DebugPathNode) -> Self {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::int(value.x));
		output.append(&mut crate::serialize::int(value.y));
		output.append(&mut crate::serialize::int(value.z));
		output.append(&mut crate::serialize::float(value.walked_distance));
		output.append(&mut crate::serialize::float(value.cost_malus));
		output.append(&mut crate::serialize::boolean(value.closed));
		output.push(value.node_type);
		output.append(&mut crate::serialize::float(value.f));

		return output;
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct DebugEntityPath {
	pub reached: bool,
	pub next_block_index: i32,
	pub block_position: BlockPosition,
	pub nodes: Vec<DebugPathNode>,
	pub target_nodes: Vec<DebugPathNode>,
	pub open_set: Vec<DebugPathNode>,
	pub closed_set: Vec<DebugPathNode>,
	pub max_node_distance: f32,
}

fn deserialize_path_node_array(value: &mut Vec<u8>) -> Result<Vec<DebugPathNode>, Box<dyn Error>> {
	if value.is_empty() {
		return Err(Box::new(crate::CustomError::InvalidInput("input to deserialize_path_node_array was empty".to_string())));
	}
	let length = crate::deserialize::varint(value)?;
	let mut output: Vec<DebugPathNode> = Vec::new();
	for _ in 0..length {
		output.push(value.try_into()?);
	}

	return Ok(output);
}

impl TryFrom<Vec<u8>> for DebugEntityPath {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Self::Error> {
		return Ok(DebugEntityPath {
			reached: crate::deserialize::boolean(&mut value)?,
			next_block_index: crate::deserialize::int(&mut value)?,
			block_position: crate::deserialize::position(&mut value)?,
			nodes: deserialize_path_node_array(&mut value)?,
			target_nodes: deserialize_path_node_array(&mut value)?,
			open_set: deserialize_path_node_array(&mut value)?,
			closed_set: deserialize_path_node_array(&mut value)?,
			max_node_distance: crate::deserialize::float(&mut value)?,
		});
	}
}

impl From<DebugEntityPath> for Vec<u8> {
	fn from(value: DebugEntityPath) -> Self {
		let mut output: Vec<u8> = Vec::new();

		let nodes_len = value.nodes.len() as i32;
		let target_nodes_len = value.target_nodes.len() as i32;
		let open_set_len = value.open_set.len() as i32;
		let closed_set_len = value.closed_set.len() as i32;

		output.append(&mut crate::serialize::boolean(value.reached));
		output.append(&mut crate::serialize::int(value.next_block_index));
		output.append(&mut crate::serialize::position(&value.block_position));
		output.append(&mut crate::serialize::prefixed_array(value.nodes.into_iter().flat_map(Vec::<u8>::from).collect(), nodes_len));
		output
			.append(&mut crate::serialize::prefixed_array(value.target_nodes.into_iter().flat_map(Vec::<u8>::from).collect(), target_nodes_len));
		output.append(&mut crate::serialize::prefixed_array(value.open_set.into_iter().flat_map(Vec::<u8>::from).collect(), open_set_len));
		output.append(&mut crate::serialize::prefixed_array(value.closed_set.into_iter().flat_map(Vec::<u8>::from).collect(), closed_set_len));
		output.append(&mut crate::serialize::float(value.max_node_distance));

		return output;
	}
}
