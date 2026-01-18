use super::*;
use crate::types::*;

//
// MARK: 0x01 Spawn entity
//

#[derive(Debug, Clone)]
pub struct SpawnEntity {
	pub entity_id: i32,
	pub entity_uuid: u128,
	pub entity_type: i32, //real name in the protocol is just type; enum?
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub pitch: u8,
	pub yaw: u8,
	pub head_yaw: u8,
	pub data: i32,
	pub velocity_x: i16,
	pub velocity_y: i16,
	pub velocity_z: i16,
}

impl Packet for SpawnEntity {
	const PACKET_ID: u8 = 0x01;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<SpawnEntity> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SpawnEntity) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_id));
		output.append(&mut crate::serialize::uuid(&value.entity_uuid));
		output.append(&mut crate::serialize::varint(value.entity_type));
		output.append(&mut crate::serialize::double(value.x));
		output.append(&mut crate::serialize::double(value.y));
		output.append(&mut crate::serialize::double(value.z));
		output.push(0);
		output.push(value.pitch);
		output.push(value.yaw);
		output.push(value.head_yaw);
		output.append(&mut crate::serialize::varint(value.data));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SpawnEntity {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			entity_id: crate::deserialize::varint(&mut value)?,
			entity_uuid: crate::deserialize::uuid(&mut value)?,
			entity_type: crate::deserialize::varint(&mut value)?,
			x: crate::deserialize::double(&mut value)?,
			y: crate::deserialize::double(&mut value)?,
			z: crate::deserialize::double(&mut value)?,
			pitch: value.remove(0),
			yaw: value.remove(0),
			head_yaw: value.remove(0),
			data: crate::deserialize::varint(&mut value)?,
			velocity_x: 0,
			velocity_y: 0,
			velocity_z: 0,
		});
	}
}

//
// MARK: 0x02 entity animation
//

#[derive(Debug, Clone)]
pub struct EntityAnimation {
	pub entity_id: i32,
	pub animation: u8,
}

impl Packet for EntityAnimation {
	const PACKET_ID: u8 = 0x02;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<EntityAnimation> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: EntityAnimation) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_id));
		output.push(value.animation);

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for EntityAnimation {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			entity_id: crate::deserialize::varint(&mut value)?,
			animation: value.remove(0),
		});
	}
}

//
// MARK: 0x04 acknowledge block change
//

#[derive(Debug, Clone)]
pub struct AcknowledgeBlockChange {
	pub sequence_id: i32,
}

impl Packet for AcknowledgeBlockChange {
	const PACKET_ID: u8 = 0x04;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<AcknowledgeBlockChange> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: AcknowledgeBlockChange) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.sequence_id));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for AcknowledgeBlockChange {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			sequence_id: crate::deserialize::varint(&mut value)?,
		});
	}
}

//
// MARK: 0x06 block entity data
//

#[derive(Debug, Clone)]
pub struct BlockEntityData {
	pub location: BlockPosition,
	pub block_entity_type: i32,
	pub nbt_data: NbtTag,
}

impl Packet for BlockEntityData {
	const PACKET_ID: u8 = 0x06;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<BlockEntityData> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: BlockEntityData) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::position(&value.location));
		output.append(&mut crate::serialize::varint(value.block_entity_type));
		output.append(&mut crate::serialize::nbt_network(value.nbt_data));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for BlockEntityData {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			location: crate::deserialize::position(&mut value)?,
			block_entity_type: crate::deserialize::varint(&mut value)?,
			nbt_data: crate::deserialize::nbt_network(&mut value)?,
		});
	}
}

//
// MARK: 0x07 block action
//

#[derive(Debug, Clone)]
pub struct BlockAction {
	pub location: BlockPosition,
	pub action_id: u8, //see https://minecraft.wiki/w/Java_Edition_protocol/Block_actions
	pub action_parameter: u8,
	pub block_type: i32,
}

impl Packet for BlockAction {
	const PACKET_ID: u8 = 0x07;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<BlockAction> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: BlockAction) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::position(&value.location));
		output.push(value.action_id);
		output.push(value.action_parameter);
		output.append(&mut crate::serialize::varint(value.block_type));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for BlockAction {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			location: crate::deserialize::position(&mut value)?,
			action_id: value.remove(0),
			action_parameter: value.remove(0),
			block_type: crate::deserialize::varint(&mut value)?,
		});
	}
}

//
// MARK: 0x08 block update
//

#[derive(Debug, Clone)]
pub struct BlockUpdate {
	pub location: BlockPosition,
	pub block_id: i32,
}

impl Packet for BlockUpdate {
	const PACKET_ID: u8 = 0x08;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<BlockUpdate> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: BlockUpdate) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::position(&value.location));
		output.append(&mut crate::serialize::varint(value.block_id));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for BlockUpdate {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			location: crate::deserialize::position(&mut value)?,
			block_id: crate::deserialize::varint(&mut value)?,
		});
	}
}

//
// MARK: 0x10 commands
//

#[derive(Debug, Clone)]
pub struct Commands {
	pub nodes: Vec<CommandNode>,
	pub root_index: i32,
}

impl Packet for Commands {
	const PACKET_ID: u8 = 0x10;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<Commands> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: Commands) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.nodes.len() as i32));
		value.nodes.into_iter().for_each(|x| output.append(&mut x.try_into().unwrap()));
		output.append(&mut crate::serialize::varint(value.root_index));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for Commands {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let nodes_len = crate::deserialize::varint(&mut value)?;
		let nodes: Vec<CommandNode> = (0..nodes_len).map(|_| CommandNode::try_from(&mut value).unwrap()).collect();
		let root_index = crate::deserialize::varint(&mut value)?;

		return Ok(Self {
			nodes,
			root_index,
		});
	}
}

//
// MARK: 0x11 close container
//

#[derive(Debug, Clone)]
pub struct CloseContainer {
	pub window_id: i32,
}

impl Packet for CloseContainer {
	const PACKET_ID: u8 = 0x11;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<CloseContainer> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: CloseContainer) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.window_id));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for CloseContainer {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			window_id: crate::deserialize::varint(&mut value)?,
		});
	}
}

//
// MARK: 0x12 set container content
//

#[derive(Debug, Clone)]
pub struct SetContainerContent {
	pub window_id: i32,
	pub state_id: i32,
	pub slot_data: Vec<Option<Slot>>,
	pub carried_item: Option<Slot>,
}

impl Packet for SetContainerContent {
	const PACKET_ID: u8 = 0x12;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<SetContainerContent> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetContainerContent) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.window_id));
		output.append(&mut crate::serialize::varint(value.state_id));
		output.append(&mut crate::serialize::varint(value.slot_data.len() as i32));
		value.slot_data.iter().for_each(|x| output.append(&mut crate::slot::serialize_slot(x.as_ref())));
		output.append(&mut crate::slot::serialize_slot(value.carried_item.as_ref()));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SetContainerContent {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let window_id = crate::deserialize::varint(&mut value)?;
		let state_id = crate::deserialize::varint(&mut value)?;
		let slot_data_len = crate::deserialize::varint(&mut value)?;
		let slot_data = (0..slot_data_len).map(|_| crate::slot::deserialize_slot(&mut value).unwrap()).collect();
		let carried_item = crate::slot::deserialize_slot(&mut value)?;

		return Ok(Self {
			window_id,
			state_id,
			slot_data,
			carried_item,
		});
	}
}

//
// MARK: 0x13 set container property
//

#[derive(Debug, Clone)]
pub struct SetContainerProperty {
	pub window_id: i32,
	pub property: i16,
	pub value: i16,
}

impl Packet for SetContainerProperty {
	const PACKET_ID: u8 = 0x13;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<SetContainerProperty> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetContainerProperty) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.window_id));
		output.append(&mut crate::serialize::short(value.property));
		output.append(&mut crate::serialize::short(value.value));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SetContainerProperty {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			window_id: crate::deserialize::varint(&mut value)?,
			property: crate::deserialize::short(&mut value)?,
			value: crate::deserialize::short(&mut value)?,
		});
	}
}

//
// MARK: 0x14 set container slot
//

#[derive(Debug, Clone)]
pub struct SetContainerSlot {
	pub window_id: i32,
	pub state_id: i32,
	pub slot: i16,
	pub slot_data: Option<Slot>,
}

impl Packet for SetContainerSlot {
	const PACKET_ID: u8 = 0x14;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<SetContainerSlot> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetContainerSlot) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.window_id));
		output.append(&mut crate::serialize::varint(value.state_id));
		output.append(&mut crate::serialize::short(value.slot));
		output.append(&mut crate::slot::serialize_slot(value.slot_data.as_ref()));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SetContainerSlot {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			window_id: crate::deserialize::varint(&mut value)?,
			state_id: crate::deserialize::varint(&mut value)?,
			slot: crate::deserialize::short(&mut value)?,
			slot_data: crate::slot::deserialize_slot(&mut value)?,
		});
	}
}

//
// MARK: 0x22 entity event
//

#[derive(Debug, Clone)]
pub struct EntityEvent {
	pub entity_id: i32,
	pub entity_status: u8, //see https://minecraft.wiki/w/Java_Edition_protocol/Entity_statuses
}

impl Packet for EntityEvent {
	const PACKET_ID: u8 = 0x22;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<EntityEvent> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: EntityEvent) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::int(value.entity_id));
		output.push(value.entity_status);

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for EntityEvent {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			entity_id: crate::deserialize::int(&mut value)?,
			entity_status: value.remove(0),
		});
	}
}


//
// MARK: 0x23 teleport entity
//

#[derive(Debug, Clone)]
pub struct TeleportEntity {
	pub entity_id: i32,
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub velocity_x: f64,
	pub velocity_y: f64,
	pub velocity_z: f64,
	pub yaw: f32,
	pub pitch: f32,
	pub on_ground: bool,
}

impl Packet for TeleportEntity {
	const PACKET_ID: u8 = 0x23;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<TeleportEntity> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: TeleportEntity) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_id));
		output.append(&mut crate::serialize::double(value.x));
		output.append(&mut crate::serialize::double(value.y));
		output.append(&mut crate::serialize::double(value.z));
		output.append(&mut crate::serialize::double(value.velocity_x));
		output.append(&mut crate::serialize::double(value.velocity_y));
		output.append(&mut crate::serialize::double(value.velocity_z));
		output.append(&mut crate::serialize::float(value.yaw));
		output.append(&mut crate::serialize::float(value.pitch));
		output.append(&mut crate::serialize::boolean(value.on_ground));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for TeleportEntity {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			entity_id: crate::deserialize::varint(&mut value)?,
			x: crate::deserialize::double(&mut value)?,
			y: crate::deserialize::double(&mut value)?,
			z: crate::deserialize::double(&mut value)?,
			velocity_x: crate::deserialize::double(&mut value)?,
			velocity_y: crate::deserialize::double(&mut value)?,
			velocity_z: crate::deserialize::double(&mut value)?,
			yaw: crate::deserialize::float(&mut value)?,
			pitch: crate::deserialize::float(&mut value)?,
			on_ground: crate::deserialize::boolean(&mut value)?,
		});
	}
}

//
// MARK: 0x24 Explosion
//

#[derive(Debug, Clone)]
pub struct Explosion {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub radius: f32,
	pub block_count: i32, //int, not a varint!
	pub player_delta_velocity: Option<(f64, f64, f64)>,
	pub particle_id: i32,
	pub particle_data: (), //see https://minecraft.wiki/w/Java_Edition_protocol/Particles
	pub sound: i32,
}

impl Packet for Explosion {
	const PACKET_ID: u8 = 0x24;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<Explosion> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: Explosion) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::double(value.x));
		output.append(&mut crate::serialize::double(value.y));
		output.append(&mut crate::serialize::double(value.z));
		output.append(&mut crate::serialize::float(value.radius));
		output.append(&mut crate::serialize::int(value.block_count));
		if let Some(player_delta_velocity) = value.player_delta_velocity {
			output.push(1);
			output.append(&mut crate::serialize::double(player_delta_velocity.0));
			output.append(&mut crate::serialize::double(player_delta_velocity.1));
			output.append(&mut crate::serialize::double(player_delta_velocity.2));
		} else {
			output.push(0);
		}
		output.append(&mut crate::serialize::varint(value.particle_id));
		output.append(&mut crate::serialize::varint(value.sound));
		output.push(0); //block particle alternatives

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for Explosion {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let x = crate::deserialize::double(&mut value)?;
		let y = crate::deserialize::double(&mut value)?;
		let z = crate::deserialize::double(&mut value)?;
		let radius = crate::deserialize::float(&mut value)?;
		let block_count = crate::deserialize::int(&mut value)?;
		let player_delta_velocity: Option<(f64, f64, f64)> = if crate::deserialize::boolean(&mut value)? {
			Some((crate::deserialize::double(&mut value)?, crate::deserialize::double(&mut value)?, crate::deserialize::double(&mut value)?))
		} else {
			None
		};

		let particle_id = crate::deserialize::varint(&mut value)?;
		let sound = crate::deserialize::varint(&mut value)?;

		return Ok(Self {
			x,
			y,
			z,
			radius,
			block_count,
			player_delta_velocity,
			particle_id,
			particle_data: (),
			sound,
		});
	}
}

//
// MARK: 0x26 Game event
//

#[derive(Debug, Clone)]
pub struct GameEvent {
	pub event: u8,
	pub value: f32,
}

impl Packet for GameEvent {
	const PACKET_ID: u8 = 0x26;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<GameEvent> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: GameEvent) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.push(value.event);
		output.append(&mut crate::serialize::float(value.value));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for GameEvent {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			event: value.remove(0),
			value: crate::deserialize::float(&mut value)?,
		});
	}
}

//
// MARK: 0x29 Hurt Animation
//

#[derive(Debug, Clone)]
pub struct HurtAnimation {
	pub entity_id: i32,
	pub yaw: f32,
}

impl Packet for HurtAnimation {
	const PACKET_ID: u8 = 0x29;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<HurtAnimation> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: HurtAnimation) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_id));
		output.append(&mut crate::serialize::float(value.yaw));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for HurtAnimation {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			entity_id: crate::deserialize::varint(&mut value)?,
			yaw: crate::deserialize::float(&mut value)?,
		});
	}
}

//
// MARK: 0x2b clientbound keep alive
//

#[derive(Debug, Clone)]
pub struct ClientboundKeepAlive {
	pub keep_alive_id: i64,
}

impl Packet for ClientboundKeepAlive {
	const PACKET_ID: u8 = 0x2b;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<ClientboundKeepAlive> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ClientboundKeepAlive) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::long(value.keep_alive_id));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for ClientboundKeepAlive {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			keep_alive_id: crate::deserialize::long(&mut value)?,
		});
	}
}

//
// MARK: 0x2c Chunk Data and Update Light
//

#[derive(Debug, Clone)]
pub struct ChunkDataAndUpdateLight {
	pub chunk_x: i32,
	pub chunk_z: i32,
	pub heightmaps: Vec<HeightMap>,
	pub data: Vec<ChunkSection>,
	pub block_entities: Vec<BlockEntity>,
	pub sky_light_mask: Bitset,
	pub block_light_mask: Bitset,
	pub empty_sky_light_mask: Bitset,
	pub empty_block_light_mask: Bitset,
	pub sky_light_arrays: Vec<LightArray>,
	pub block_light_arrays: Vec<LightArray>,
}

impl Packet for ChunkDataAndUpdateLight {
	const PACKET_ID: u8 = 0x2c;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

type Bitset = Vec<u64>;
type LightArray = Vec<u8>;


#[derive(Clone, Debug)]
pub struct HeightMap {
	pub data_type: i32,
	pub data: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct BlockEntity {
	pub packed_xz: u8,
	pub y: i16,
	pub block_entity_type: i32,
	pub data: Option<NbtTag>,
}

#[derive(Debug, Clone)]
pub struct ChunkSection {
	pub block_count: u16,
	pub block_states: BlockStatesPalettedContainer,
	pub biomes: BiomesPalettedContainer,
}

#[derive(Debug, Clone)]
pub enum BlockStatesPalettedContainer {
	SingleValued(SingleValued),
	Indirect(Indirect),
	Direct(Direct),
}

#[derive(Debug, Clone)]
pub enum BiomesPalettedContainer {
	SingleValued(SingleValued),
	Indirect(Indirect),
	Direct(Direct),
}

#[derive(Debug, Clone)]
pub struct SingleValued {
	pub bits_per_entry: u8,
	pub value: i32,
}

#[derive(Debug, Clone)]
pub struct Indirect {
	pub bits_per_entry: u8,
	pub palette: Vec<i32>,
	pub data_array: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct Direct {
	pub bits_per_entry: u8,
	pub data_array: Vec<i32>,
}

impl TryFrom<ChunkDataAndUpdateLight> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ChunkDataAndUpdateLight) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::int(value.chunk_x));
		output.append(&mut crate::serialize::int(value.chunk_z));

		output.append(&mut crate::serialize::varint(value.heightmaps.len() as i32));
		for heightmap in value.heightmaps {
			output.append(&mut crate::serialize::varint(heightmap.data_type));
			output.append(&mut crate::serialize::varint(heightmap.data.len() as i32));
			for heightmap_data in heightmap.data {
				output.append(&mut crate::serialize::unsigned_long(heightmap_data));
			}
		}

		let mut chunk_section_data: Vec<u8> = Vec::new();
		for chunk_section in value.data {
			chunk_section_data.append(&mut chunk_section.try_into()?);
		}
		output.append(&mut crate::serialize::varint(chunk_section_data.len() as i32));
		output.append(&mut chunk_section_data);
		output.append(&mut crate::serialize::varint(value.block_entities.len() as i32));
		for x in value.block_entities {
			output.push(x.packed_xz);
			output.append(&mut crate::serialize::short(x.y));
			output.append(&mut crate::serialize::varint(x.block_entity_type));

			if let Some(nbt_tag) = x.data {
				output.append(&mut crate::serialize::nbt_network(nbt_tag));
			}
		}
		output.append(&mut crate::serialize::bitset(&value.sky_light_mask));
		output.append(&mut crate::serialize::bitset(&value.block_light_mask));
		output.append(&mut crate::serialize::bitset(&value.empty_sky_light_mask));
		output.append(&mut crate::serialize::bitset(&value.empty_block_light_mask));

		output.append(&mut crate::serialize::varint(value.sky_light_arrays.len() as i32));
		for x in value.sky_light_arrays {
			output.append(&mut crate::serialize::varint(x.len() as i32));
			output.append(&mut x.clone());
		}
		output.append(&mut crate::serialize::varint(value.block_light_arrays.len() as i32));
		for x in value.block_light_arrays {
			output.append(&mut crate::serialize::varint(x.len() as i32));
			output.append(&mut x.clone());
		}

		return Ok(output);
	}
}

impl TryFrom<ChunkSection> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ChunkSection) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::unsigned_short(value.block_count));
		output.append(&mut value.block_states.try_into()?);
		output.append(&mut value.biomes.try_into()?);

		return Ok(output);
	}
}

impl TryFrom<BlockStatesPalettedContainer> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: BlockStatesPalettedContainer) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		match value {
			BlockStatesPalettedContainer::SingleValued(single_valued) => {
				output.push(single_valued.bits_per_entry);
				output.append(&mut crate::serialize::varint(single_valued.value));
			}
			BlockStatesPalettedContainer::Indirect(indirect) => {
				output.push(indirect.bits_per_entry);
				output.append(&mut crate::serialize::varint(indirect.palette.len() as i32));
				for palette in indirect.palette {
					output.append(&mut crate::serialize::varint(palette));
				}

				let mut data_iter = indirect.data_array.into_iter();
				while data_iter.len() != 0 {
					let entries_per_long = 64 / indirect.bits_per_entry;
					let mut entry: u64 = 0;
					for i in 0..entries_per_long {
						if let Some(next) = data_iter.next() {
							entry += (next as u64) << (i * indirect.bits_per_entry) as u64;
						}
					}
					output.append(&mut crate::serialize::unsigned_long(entry));
				}
			}
			BlockStatesPalettedContainer::Direct(direct) => {
				output.push(direct.bits_per_entry);
				let entries_per_long = 64 / direct.bits_per_entry;

				let mut data_iter = direct.data_array.into_iter();

				while data_iter.len() != 0 {
					let mut entry: u64 = 0;
					for i in 0..entries_per_long {
						if let Some(next) = data_iter.next() {
							entry += (next as u64) << (i * direct.bits_per_entry) as u64;
						}
					}
					output.append(&mut crate::serialize::unsigned_long(entry));
				}
			}
		};

		return Ok(output);
	}
}

impl TryFrom<BiomesPalettedContainer> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: BiomesPalettedContainer) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		match value {
			BiomesPalettedContainer::SingleValued(single_valued) => {
				output.push(single_valued.bits_per_entry);
				output.append(&mut crate::serialize::varint(single_valued.value));
			}
			BiomesPalettedContainer::Indirect(indirect) => {
				output.push(indirect.bits_per_entry);
				output.append(&mut crate::serialize::varint(indirect.palette.len() as i32));
				for palette in indirect.palette {
					output.append(&mut crate::serialize::varint(palette));
				}
				let mut data_array = indirect.data_array.clone();
				while !data_array.is_empty() {
					let entries_per_long = 64 / indirect.bits_per_entry;
					let mut entry: u64 = 0;
					for i in 0..entries_per_long {
						if !data_array.is_empty() {
							let value = data_array.remove(0);
							entry += (value as u64) << (i * indirect.bits_per_entry) as u64;
						}
					}
					output.append(&mut crate::serialize::unsigned_long(entry));
				}
			}
			BiomesPalettedContainer::Direct(direct) => {
				output.push(direct.bits_per_entry);
				let mut data_array = direct.data_array.clone();
				while !data_array.is_empty() {
					let entries_per_long = 64 / direct.bits_per_entry;
					let mut entry: u64 = 0;
					for i in 0..entries_per_long {
						if !data_array.is_empty() {
							let value = data_array.remove(0);
							entry += (value as u64) << (i * direct.bits_per_entry) as u64;
						}
					}
					output.append(&mut crate::serialize::unsigned_long(entry));
				}
			}
		};

		return Ok(output);
	}
}


impl TryFrom<Vec<u8>> for ChunkDataAndUpdateLight {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let chunk_x = crate::deserialize::int(&mut value)?;
		let chunk_z = crate::deserialize::int(&mut value)?;
		let mut heightmaps: Vec<HeightMap> = Vec::new();
		let heightmaps_len = crate::deserialize::varint(&mut value)?;
		for _ in 0..heightmaps_len {
			let data_type = crate::deserialize::varint(&mut value)?;
			let data_len = crate::deserialize::varint(&mut value)?;
			let mut data: Vec<u64> = Vec::new();
			for _ in 0..data_len {
				data.push(crate::deserialize::unsigned_long(&mut value)?);
			}
			heightmaps.push(HeightMap {
				data_type,
				data,
			});
		}
		let _size = crate::deserialize::varint(&mut value)?;
		let mut data: Vec<ChunkSection> = Vec::new();
		let chunk_sections = 24;
		for _ in 0..chunk_sections {
			data.push((&mut value).try_into()?);
		}

		let block_entities_len = crate::deserialize::varint(&mut value)?;
		let mut block_entities: Vec<BlockEntity> = Vec::new();
		for _ in 0..block_entities_len {
			let packed_xz = value.remove(0);
			let y = crate::deserialize::short(&mut value)?;
			let block_entity_type = crate::deserialize::varint(&mut value)?;
			let data = if *value.first().unwrap() == 0 { None } else { Some(crate::deserialize::nbt_network(&mut value)?) };
			block_entities.push(BlockEntity {
				packed_xz,
				y,
				block_entity_type,
				data,
			});
		}
		let sky_light_mask = crate::deserialize::bitset(&mut value)?;
		let block_light_mask = crate::deserialize::bitset(&mut value)?;
		let empty_sky_light_mask = crate::deserialize::bitset(&mut value)?;
		let empty_block_light_mask = crate::deserialize::bitset(&mut value)?;

		let sky_light_arrays_len = crate::deserialize::varint(&mut value)?;
		let mut sky_light_arrays: Vec<LightArray> = Vec::new();
		for _ in 0..sky_light_arrays_len {
			let len = crate::deserialize::varint(&mut value)?;
			let mut light_array: Vec<u8> = Vec::new();
			for _ in 0..len {
				light_array.push(value.remove(0));
			}
			sky_light_arrays.push(light_array);
		}
		let block_light_arrays_len = crate::deserialize::varint(&mut value)?;
		let mut block_light_arrays: Vec<LightArray> = Vec::new();
		for _ in 0..block_light_arrays_len {
			let len = crate::deserialize::varint(&mut value)?;
			let mut light_array: Vec<u8> = Vec::new();
			for _ in 0..len {
				light_array.push(value.remove(0));
			}
			block_light_arrays.push(light_array);
		}

		return Ok(Self {
			chunk_x,
			chunk_z,
			heightmaps,
			data,
			block_entities,
			sky_light_mask,
			block_light_mask,
			empty_sky_light_mask,
			empty_block_light_mask,
			sky_light_arrays,
			block_light_arrays,
		});
	}
}

impl TryFrom<&mut Vec<u8>> for ChunkSection {
	type Error = Box<dyn Error>;

	fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
		return Ok(Self {
			block_count: crate::deserialize::unsigned_short(value)?,
			block_states: value.try_into()?,
			biomes: value.try_into()?,
		});
	}
}

impl TryFrom<&mut Vec<u8>> for BlockStatesPalettedContainer {
	type Error = Box<dyn Error>;

	fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
		let bits_per_entry = value.remove(0);

		return match bits_per_entry {
			0 => {
				let value_entry = crate::deserialize::varint(value)?;
				Ok(BlockStatesPalettedContainer::SingleValued(SingleValued {
					bits_per_entry,
					value: value_entry,
				}))
			}
			1..=14 => {
				let palette_length = crate::deserialize::varint(value)?;
				let mut palette: Vec<i32> = Vec::new();
				for _ in 0..palette_length {
					palette.push(crate::deserialize::varint(value)?);
				}
				let entries_per_long = 64 / bits_per_entry as i32;
				let long_array_length = (f64::from(16 * 16 * 16) / f64::from(entries_per_long)).ceil() as i32;
				let mut data_array: Vec<i32> = Vec::new();
				for _ in 0..long_array_length {
					let value = crate::deserialize::unsigned_long(value)?;
					for i in 0..entries_per_long {
						let entry = value >> (i * bits_per_entry as i32);
						let entry = entry >> (64 - bits_per_entry);
						data_array.push(entry as i32);
					}
				}
				Ok(BlockStatesPalettedContainer::Indirect(Indirect {
					bits_per_entry,
					data_array,
					palette,
				}))
			}
			_ => {
				let entries_per_long = 64 / bits_per_entry as i32;
				let long_array_length = (f64::from(16 * 16 * 16) / f64::from(entries_per_long)).ceil() as i32;
				let mut data_array: Vec<i32> = Vec::new();
				for _ in 0..long_array_length {
					let value = crate::deserialize::unsigned_long(value)?;
					for i in 0..entries_per_long {
						let entry = value >> (i * bits_per_entry as i32);
						let entry = entry >> (64 - bits_per_entry);
						data_array.push(entry as i32);
					}
				}
				Ok(BlockStatesPalettedContainer::Direct(Direct {
					bits_per_entry,
					data_array,
				}))
			}
		};
	}
}

impl TryFrom<&mut Vec<u8>> for BiomesPalettedContainer {
	type Error = Box<dyn Error>;

	fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
		let bits_per_entry = value.remove(0);

		return match bits_per_entry {
			0 => {
				let value_entry = crate::deserialize::varint(value)?;
				Ok(BiomesPalettedContainer::SingleValued(SingleValued {
					bits_per_entry,
					value: value_entry,
				}))
			}
			1..=5 => {
				let palette_length = crate::deserialize::varint(value)?;
				let mut palette: Vec<i32> = Vec::new();
				for _ in 0..palette_length {
					palette.push(crate::deserialize::varint(value)?);
				}

				let entries_per_long = 64 / bits_per_entry as i32;
				let data_array_length = (f64::from(4 * 4 * 4) / f64::from(entries_per_long)).ceil() as i32;
				let mut data_array: Vec<i32> = Vec::new();
				for _ in 0..data_array_length {
					let value = crate::deserialize::unsigned_long(value)?;
					for i in 0..entries_per_long {
						let entry = value >> (i * bits_per_entry as i32);
						let entry = entry >> (64 - bits_per_entry);
						data_array.push(entry as i32);
					}
				}
				Ok(BiomesPalettedContainer::Indirect(Indirect {
					bits_per_entry,
					data_array,
					palette,
				}))
			}
			_ => {
				let entries_per_long = 64 / bits_per_entry as i32;
				let data_array_length = (f64::from(4 * 4 * 4) / f64::from(entries_per_long)).ceil() as i32;
				let mut data_array: Vec<i32> = Vec::new();
				for _ in 0..data_array_length {
					let value = crate::deserialize::unsigned_long(value)?;
					for i in 0..entries_per_long {
						let entry = value >> (i * bits_per_entry as i32);
						let entry = entry >> (64 - bits_per_entry);
						data_array.push(entry as i32);
					}
				}
				Ok(BiomesPalettedContainer::Direct(Direct {
					bits_per_entry,
					data_array,
				}))
			}
		};
	}
}

//
// MARK: 0x2d world event
//

#[derive(Debug, Clone)]
pub struct WorldEvent {
	pub event: i32,
	pub location: BlockPosition,
	pub data: i32,
}

impl Packet for WorldEvent {
	const PACKET_ID: u8 = 0x2d;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<WorldEvent> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: WorldEvent) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::int(value.event));
		output.append(&mut crate::serialize::position(&value.location));
		output.append(&mut crate::serialize::int(value.data));
		if value.event == 1023 || value.event == 1028 || value.event == 1038 {
			output.push(1);
		} else {
			output.push(0);
		}

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for WorldEvent {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			event: crate::deserialize::int(&mut value)?,
			location: crate::deserialize::position(&mut value)?,
			data: crate::deserialize::int(&mut value)?,
		});
	}
}

//
// MARK: 0x30 Login
//

#[derive(Debug, Clone)]
pub struct Login {
	pub entity_id: i32,
	pub is_hardcore: bool,
	pub dimension_names: Vec<String>,
	pub max_players: i32,
	pub view_distance: i32,
	pub simulation_distance: i32,
	pub reduced_debug_info: bool,
	pub enable_respawn_screen: bool,
	pub do_limited_crafting: bool,
	pub dimension_type: i32,
	pub dimension_name: String,
	pub hashed_seed: i64,
	pub game_mode: u8,
	pub previous_game_mode: i8,
	pub is_debug: bool,
	pub is_flat: bool,
	pub has_death_location: bool,
	pub death_dimension_name: Option<String>,
	pub death_location: Option<u64>,
	pub portal_cooldown: i32,
	pub sea_level: i32,
	pub enforces_secure_chat: bool,
}

impl Packet for Login {
	const PACKET_ID: u8 = 0x30;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<Login> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: Login) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::int(value.entity_id));
		output.append(&mut crate::serialize::boolean(value.is_hardcore));
		output.append(&mut crate::serialize::varint(value.dimension_names.len() as i32));
		for dimension_name in &value.dimension_names {
			output.append(&mut crate::serialize::string(dimension_name));
		}
		output.append(&mut crate::serialize::varint(value.max_players));
		output.append(&mut crate::serialize::varint(value.view_distance));
		output.append(&mut crate::serialize::varint(value.simulation_distance));
		output.append(&mut crate::serialize::boolean(value.reduced_debug_info));
		output.append(&mut crate::serialize::boolean(value.enable_respawn_screen));
		output.append(&mut crate::serialize::boolean(value.do_limited_crafting));
		output.append(&mut crate::serialize::varint(value.dimension_type));
		output.append(&mut crate::serialize::string(&value.dimension_name));
		output.append(&mut crate::serialize::long(value.hashed_seed));
		output.push(value.game_mode);
		output.push(value.previous_game_mode as u8);
		output.append(&mut crate::serialize::boolean(value.is_debug));
		output.append(&mut crate::serialize::boolean(value.is_flat));
		output.append(&mut crate::serialize::boolean(value.has_death_location));
		if value.has_death_location {
			output.append(&mut crate::serialize::string(&value.death_dimension_name.unwrap()));
			output.append(&mut crate::serialize::long(value.death_location.unwrap() as i64)); //probably fucked
		}
		output.append(&mut crate::serialize::varint(value.portal_cooldown));
		output.append(&mut crate::serialize::varint(value.sea_level));
		output.append(&mut crate::serialize::boolean(value.enforces_secure_chat));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for Login {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let entity_id = crate::deserialize::int(&mut value)?;
		let is_hardcore = crate::deserialize::boolean(&mut value)?;

		let dimension_names_len = crate::deserialize::varint(&mut value)?;
		let mut dimension_names: Vec<String> = Vec::new();
		for _ in 0..dimension_names_len {
			dimension_names.push(crate::deserialize::string(&mut value)?);
		}

		let max_players = crate::deserialize::varint(&mut value)?;
		let view_distance = crate::deserialize::varint(&mut value)?;
		let simulation_distance = crate::deserialize::varint(&mut value)?;
		let reduced_debug_info = crate::deserialize::boolean(&mut value)?;
		let enable_respawn_screen = crate::deserialize::boolean(&mut value)?;
		let do_limited_crafting = crate::deserialize::boolean(&mut value)?;
		let dimension_type = crate::deserialize::varint(&mut value)?;
		let dimension_name = crate::deserialize::string(&mut value)?;
		let hashed_seed = crate::deserialize::long(&mut value)?;
		let game_mode = value.remove(0);
		let previous_game_mode = value.remove(0) as i8;
		let is_debug = crate::deserialize::boolean(&mut value)?;
		let is_flat = crate::deserialize::boolean(&mut value)?;
		let has_death_location = crate::deserialize::boolean(&mut value)?;

		let death_dimension_name: Option<String> = if has_death_location { Some(crate::deserialize::string(&mut value)?) } else { None };

		let death_location: Option<u64> = if has_death_location {
			Some(crate::deserialize::long(&mut value)? as u64) //Probably fucked
		} else {
			None
		};

		let portal_cooldown = crate::deserialize::varint(&mut value)?;
		let sea_level = crate::deserialize::varint(&mut value)?;
		let enforces_secure_chat = crate::deserialize::boolean(&mut value)?;


		return Ok(Self {
			entity_id,
			is_hardcore,
			dimension_names,
			max_players,
			view_distance,
			simulation_distance,
			reduced_debug_info,
			enable_respawn_screen,
			do_limited_crafting,
			dimension_type,
			dimension_name,
			hashed_seed,
			game_mode,
			previous_game_mode,
			is_debug,
			is_flat,
			has_death_location,
			death_dimension_name,
			death_location,
			portal_cooldown,
			sea_level,
			enforces_secure_chat,
		});
	}
}

//
// MARK: 0x33 Update entity position
//

#[derive(Debug, Clone)]
pub struct UpdateEntityPosition {
	pub entity_id: i32,
	pub delta_x: i16,
	pub delta_y: i16,
	pub delta_z: i16,
	pub on_ground: bool,
}

impl Packet for UpdateEntityPosition {
	const PACKET_ID: u8 = 0x33;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<UpdateEntityPosition> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: UpdateEntityPosition) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_id));
		output.append(&mut crate::serialize::short(value.delta_x));
		output.append(&mut crate::serialize::short(value.delta_y));
		output.append(&mut crate::serialize::short(value.delta_z));
		output.append(&mut crate::serialize::boolean(value.on_ground));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for UpdateEntityPosition {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			entity_id: crate::deserialize::varint(&mut value)?,
			delta_x: crate::deserialize::short(&mut value)?,
			delta_y: crate::deserialize::short(&mut value)?,
			delta_z: crate::deserialize::short(&mut value)?,
			on_ground: crate::deserialize::boolean(&mut value)?,
		});
	}
}

//
// MARK: 0x34 Update entity position and rotation
//

#[derive(Debug, Clone)]
pub struct UpdateEntityPositionAndRotation {
	pub entity_id: i32,
	pub delta_x: i16,
	pub delta_y: i16,
	pub delta_z: i16,
	pub yaw: u8,
	pub pitch: u8,
	pub on_ground: bool,
}

impl Packet for UpdateEntityPositionAndRotation {
	const PACKET_ID: u8 = 0x34;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<UpdateEntityPositionAndRotation> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: UpdateEntityPositionAndRotation) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_id));
		output.append(&mut crate::serialize::short(value.delta_x));
		output.append(&mut crate::serialize::short(value.delta_y));
		output.append(&mut crate::serialize::short(value.delta_z));
		output.push(value.yaw);
		output.push(value.pitch);
		output.append(&mut crate::serialize::boolean(value.on_ground));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for UpdateEntityPositionAndRotation {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			entity_id: crate::deserialize::varint(&mut value)?,
			delta_x: crate::deserialize::short(&mut value)?,
			delta_y: crate::deserialize::short(&mut value)?,
			delta_z: crate::deserialize::short(&mut value)?,
			yaw: value.remove(0),
			pitch: value.remove(0),
			on_ground: crate::deserialize::boolean(&mut value)?,
		});
	}
}

//
// MARK: 0x36 Update entity rotation
//

#[derive(Debug, Clone)]
pub struct UpdateEntityRotation {
	pub entity_id: i32,
	pub yaw: u8,
	pub pitch: u8,
	pub on_ground: bool,
}

impl Packet for UpdateEntityRotation {
	const PACKET_ID: u8 = 0x36;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<UpdateEntityRotation> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: UpdateEntityRotation) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_id));
		output.push(value.yaw);
		output.push(value.pitch);
		output.append(&mut crate::serialize::boolean(value.on_ground));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for UpdateEntityRotation {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			entity_id: crate::deserialize::varint(&mut value)?,
			yaw: value.remove(0),
			pitch: value.remove(0),
			on_ground: crate::deserialize::boolean(&mut value)?,
		});
	}
}

//
// MARK: 0x39 Open Screen
//

#[derive(Debug, Clone)]
pub struct OpenScreen {
	pub window_id: i32,
	pub window_type: i32,
	pub window_title: NbtTag,
}

impl Packet for OpenScreen {
	const PACKET_ID: u8 = 0x39;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<OpenScreen> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: OpenScreen) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.window_id));
		output.append(&mut crate::serialize::varint(value.window_type));
		output.append(&mut crate::serialize::nbt_network(value.window_title));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for OpenScreen {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			window_id: crate::deserialize::varint(&mut value)?,
			window_type: crate::deserialize::varint(&mut value)?,
			window_title: crate::deserialize::nbt_network(&mut value)?,
		});
	}
}

//
// MARK: 0x3a Open Sign Editor
//

#[derive(Debug, Clone)]
pub struct OpenSignEditor {
	pub location: BlockPosition,
	pub is_front_text: bool,
}

impl Packet for OpenSignEditor {
	const PACKET_ID: u8 = 0x3a;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<OpenSignEditor> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: OpenSignEditor) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::position(&value.location));
		output.append(&mut crate::serialize::boolean(value.is_front_text));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for OpenSignEditor {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			location: crate::deserialize::position(&mut value)?,
			is_front_text: crate::deserialize::boolean(&mut value)?,
		});
	}
}

//
// MARK: 0x3f player chat message
//

#[derive(Debug, Clone)]
pub struct PlayerChatMessage {
	pub global_index: i32,
	pub sender: u128,
	pub index: i32,
	pub message_signature_bytes: Vec<u8>,
	pub message: String,
	pub timestamp: i64,
	pub salt: i64,
	pub signature_array: Vec<(i32, Vec<u8>)>,
	pub unsigned_content: Option<NbtTag>,
	pub filter_type: i32,
	pub filter_type_bits: Vec<u64>, //only contains data if filter type is 2
	pub chat_type: i32,
	pub sender_name: NbtTag,
	pub target_name: Option<NbtTag>,
}

impl Packet for PlayerChatMessage {
	const PACKET_ID: u8 = 0x3f;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<PlayerChatMessage> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: PlayerChatMessage) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.global_index));
		output.append(&mut crate::serialize::uuid(&value.sender));
		output.append(&mut crate::serialize::varint(value.index));
		if value.message_signature_bytes.is_empty() {
			output.append(&mut crate::serialize::boolean(false));
		} else {
			output.append(&mut crate::serialize::boolean(true));
			value.message_signature_bytes.iter().for_each(|x| output.push(*x));
		}
		output.append(&mut crate::serialize::string(&value.message));
		output.append(&mut crate::serialize::long(value.timestamp));
		output.append(&mut crate::serialize::long(value.salt));
		output.append(&mut crate::serialize::varint(value.signature_array.len() as i32));
		value.signature_array.iter().for_each(|x| {
			output.append(&mut crate::serialize::varint(x.0));
			if x.0 == 0 {
				x.1.iter().for_each(|x| output.push(*x));
			}
		});
		if let Some(nbt_tag) = value.unsigned_content {
			output.append(&mut crate::serialize::boolean(true));
			output.append(&mut crate::serialize::nbt_network(nbt_tag));
		} else {
			output.append(&mut crate::serialize::boolean(false));
		}
		output.append(&mut crate::serialize::varint(value.filter_type));
		if value.filter_type == 2 {
			output.append(&mut crate::serialize::bitset(&value.filter_type_bits));
		}
		output.append(&mut crate::serialize::varint(value.chat_type));
		output.append(&mut crate::serialize::nbt_network(value.sender_name));
		if let Some(nbt_tag) = value.target_name {
			output.append(&mut crate::serialize::nbt_network(nbt_tag));
		}
		output.push(0); //not sure why this is needed

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for PlayerChatMessage {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let global_index = crate::deserialize::varint(&mut value)?;
		let sender = crate::deserialize::uuid(&mut value)?;
		let index = crate::deserialize::varint(&mut value)?;
		let message_signature_bytes_present = crate::deserialize::boolean(&mut value)?;
		let message_signature_bytes = if message_signature_bytes_present { (0..255).map(|_| value.remove(0)).collect() } else { Vec::new() };
		let message = crate::deserialize::string(&mut value)?;
		let timestamp = crate::deserialize::long(&mut value)?;
		let salt = crate::deserialize::long(&mut value)?;
		let signature_array_len = crate::deserialize::varint(&mut value)?;
		let signature_array: Vec<(i32, Vec<u8>)> = (0..signature_array_len)
			.map(|_| {
				let message_id = crate::deserialize::varint(&mut value).unwrap();
				let signature = if message_id == 0 { (0..255).map(|_| value.remove(0)).collect() } else { Vec::new() };
				(message_id, signature)
			})
			.collect();
		let unsigned_content_present = crate::deserialize::boolean(&mut value)?;
		let unsigned_content = if unsigned_content_present { Some(crate::deserialize::nbt_network(&mut value)?) } else { None };
		let filter_type = crate::deserialize::varint(&mut value)?;
		let filter_type_bits = if filter_type == 2 { crate::deserialize::bitset(&mut value)? } else { Vec::new() };
		let chat_type = crate::deserialize::varint(&mut value)?;
		let sender_name = crate::deserialize::nbt_network(&mut value)?;
		let target_name_present = crate::deserialize::boolean(&mut value)?;
		let target_name = if target_name_present { Some(crate::deserialize::nbt_network(&mut value)?) } else { None };

		return Ok(Self {
			global_index,
			sender,
			index,
			message_signature_bytes,
			message,
			timestamp,
			salt,
			signature_array,
			unsigned_content,
			filter_type,
			filter_type_bits,
			chat_type,
			sender_name,
			target_name,
		});
	}
}

//
// MARK: 0x43 player info remove
//

#[derive(Debug, Clone)]
pub struct PlayerInfoRemove {
	pub uuids: Vec<u128>,
}

impl Packet for PlayerInfoRemove {
	const PACKET_ID: u8 = 0x43;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<PlayerInfoRemove> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: PlayerInfoRemove) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.uuids.len() as i32));
		for uuid in value.uuids {
			output.append(&mut crate::serialize::uuid(&uuid));
		}

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for PlayerInfoRemove {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let mut uuids: Vec<u128> = Vec::new();
		for _ in 0..crate::deserialize::varint(&mut value)? {
			uuids.push(crate::deserialize::uuid(&mut value)?);
		}

		return Ok(Self {
			uuids,
		});
	}
}

//
// MARK: 0x44 PlayerInfoUpdate
//

#[derive(Debug, Clone)]
pub struct PlayerInfoUpdate {
	pub actions: u8,
	pub players: Vec<(u128, Vec<PlayerAction>)>,
}

impl Packet for PlayerInfoUpdate {
	const PACKET_ID: u8 = 0x44;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

#[derive(Debug, Clone)]
pub enum PlayerAction {
	AddPlayer(String, Vec<(String, String, Option<String>)>), //Name, Property<Name, Value, Signature>
	InitializeChat(Option<(u128, i64, Vec<u8>, Vec<u8>)>), //Chat session ID, Public key expiry time, encoded public key, public key signature
	UpdateGameMode(i32),                                   //Gamemode
	UpdateListed(bool),                                    //Listed
	UpdateLatency(i32),                                    //Ping
	UpdateDisplayName(Option<NbtTag>),                     //Display Name
	UpdateListPriority(i32),                               //Priority
	UpdateHat(bool),                                       //Visible
}

impl TryFrom<PlayerInfoUpdate> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: PlayerInfoUpdate) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.push(value.actions);
		output.append(&mut crate::serialize::varint(value.players.len() as i32));
		for player in value.players {
			output.append(&mut crate::serialize::uuid(&player.0));

			for action in player.1 {
				match action {
					PlayerAction::AddPlayer(name, properties) => {
						output.append(&mut crate::serialize::string(&name));
						output.append(&mut crate::serialize::varint(properties.len() as i32));
						for property in properties {
							output.append(&mut crate::serialize::string(&property.0));
							output.append(&mut crate::serialize::string(&property.1));
							output.append(&mut crate::serialize::boolean(property.2.is_some()));
							if let Some(prop) = property.2 {
								output.append(&mut crate::serialize::string(&prop));
							}
						}
					}
					PlayerAction::InitializeChat(data) => match data {
						Some(data) => {
							output.push(1);
							output.append(&mut crate::serialize::uuid(&data.clone().0));
							output.append(&mut crate::serialize::long(data.clone().1));
							output.append(&mut data.clone().2.clone());
							output.append(&mut data.3.clone());
						}
						None => {
							output.push(0);
						}
					},
					PlayerAction::UpdateGameMode(game_mode) => output.append(&mut crate::serialize::varint(game_mode)),
					PlayerAction::UpdateListed(listed) => output.append(&mut crate::serialize::boolean(listed)),
					PlayerAction::UpdateLatency(ping) => output.append(&mut crate::serialize::varint(ping)),
					PlayerAction::UpdateDisplayName(display_name) => {
						output.append(&mut crate::serialize::boolean(display_name.is_some()));
						if let Some(display_name) = display_name {
							output.append(&mut crate::serialize::nbt_network(display_name));
						}
					}
					PlayerAction::UpdateListPriority(priority) => output.append(&mut crate::serialize::varint(priority)),
					PlayerAction::UpdateHat(visible) => output.append(&mut crate::serialize::boolean(visible)),
				}
			}
		}

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for PlayerInfoUpdate {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let actions = value.remove(0);
		let player_length = crate::deserialize::varint(&mut value)?;
		let mut players: Vec<(u128, Vec<PlayerAction>)> = Vec::new();
		for _ in 0..player_length {
			let uuid = crate::deserialize::uuid(&mut value)?;

			let mut player_actions: Vec<PlayerAction> = Vec::new();
			if actions & 0x01 != 0 {
				let name = crate::deserialize::string(&mut value)?;
				let mut properties: Vec<(String, String, Option<String>)> = Vec::new();
				let properties_len = crate::deserialize::varint(&mut value)?;
				for _ in 0..properties_len {
					let name = crate::deserialize::string(&mut value)?;
					let value_prop = crate::deserialize::string(&mut value)?;
					let signature: Option<String> =
						if crate::deserialize::boolean(&mut value)? { Some(crate::deserialize::string(&mut value)?) } else { None };
					properties.push((name, value_prop, signature));
				}
				player_actions.push(PlayerAction::AddPlayer(name, properties));
			}

			if actions & 0x02 != 0 {
				if crate::deserialize::boolean(&mut value)? {
					let chat_session_id = crate::deserialize::uuid(&mut value)?;
					let public_key_expiry_time = crate::deserialize::long(&mut value)?;
					let encoded_public_key_length = crate::deserialize::varint(&mut value)?;
					let mut encoded_public_key: Vec<u8> = Vec::new();
					for _ in 0..encoded_public_key_length {
						encoded_public_key.push(value.remove(0));
					}
					let public_key_signature_length = crate::deserialize::varint(&mut value)?;
					let mut public_key_signature: Vec<u8> = Vec::new();
					for _ in 0..public_key_signature_length {
						public_key_signature.push(value.remove(0));
					}

					player_actions.push(PlayerAction::InitializeChat(Some((
						chat_session_id,
						public_key_expiry_time,
						encoded_public_key,
						public_key_signature,
					))));
				} else {
					player_actions.push(PlayerAction::InitializeChat(None));
				}
			}

			if actions & 0x04 != 0 {
				player_actions.push(PlayerAction::UpdateGameMode(crate::deserialize::varint(&mut value)?));
			}

			if actions & 0x08 != 0 {
				player_actions.push(PlayerAction::UpdateListed(crate::deserialize::boolean(&mut value)?));
			}

			if actions & 0x10 != 0 {
				player_actions.push(PlayerAction::UpdateLatency(crate::deserialize::varint(&mut value)?));
			}

			if actions & 0x20 != 0 {
				let display_name = if crate::deserialize::boolean(&mut value)? { Some(crate::deserialize::nbt_network(&mut value)?) } else { None };
				player_actions.push(PlayerAction::UpdateDisplayName(display_name));
			}

			if actions & 0x40 != 0 {
				player_actions.push(PlayerAction::UpdateListPriority(crate::deserialize::varint(&mut value)?));
			}

			if actions & 0x80 != 0 {
				player_actions.push(PlayerAction::UpdateHat(crate::deserialize::boolean(&mut value)?));
			}

			players.push((uuid, player_actions));
		}

		return Ok(Self {
			actions,
			players,
		});
	}
}

//
// MARK: 0x46 synchronize player position
//

#[derive(Debug, Clone)]
pub struct SynchronizePlayerPosition {
	pub teleport_id: i32,
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub velocity_x: f64,
	pub velocity_y: f64,
	pub velocity_z: f64,
	pub yaw: f32,
	pub pitch: f32,
	pub flags: i32,
}

impl Packet for SynchronizePlayerPosition {
	const PACKET_ID: u8 = 0x46;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<SynchronizePlayerPosition> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SynchronizePlayerPosition) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.teleport_id));
		output.append(&mut crate::serialize::double(value.x));
		output.append(&mut crate::serialize::double(value.y));
		output.append(&mut crate::serialize::double(value.z));
		output.append(&mut crate::serialize::double(value.velocity_x));
		output.append(&mut crate::serialize::double(value.velocity_y));
		output.append(&mut crate::serialize::double(value.velocity_z));
		output.append(&mut crate::serialize::float(value.yaw));
		output.append(&mut crate::serialize::float(value.pitch));
		output.append(&mut crate::serialize::int(value.flags));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SynchronizePlayerPosition {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			teleport_id: crate::deserialize::varint(&mut value)?,
			x: crate::deserialize::double(&mut value)?,
			y: crate::deserialize::double(&mut value)?,
			z: crate::deserialize::double(&mut value)?,
			velocity_x: crate::deserialize::double(&mut value)?,
			velocity_y: crate::deserialize::double(&mut value)?,
			velocity_z: crate::deserialize::double(&mut value)?,
			yaw: crate::deserialize::float(&mut value)?,
			pitch: crate::deserialize::float(&mut value)?,
			flags: crate::deserialize::int(&mut value)?,
		});
	}
}

//
// MARK: 0x4b remove entities
//

#[derive(Debug, Clone)]
pub struct RemoveEntities {
	pub entity_ids: Vec<i32>,
}

impl Packet for RemoveEntities {
	const PACKET_ID: u8 = 0x4b;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<RemoveEntities> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: RemoveEntities) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_ids.len() as i32));
		for entity_id in value.entity_ids {
			output.append(&mut crate::serialize::varint(entity_id));
		}

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for RemoveEntities {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let mut entity_ids: Vec<i32> = Vec::new();
		for _ in 0..crate::deserialize::varint(&mut value)? {
			entity_ids.push(crate::deserialize::varint(&mut value)?);
		}

		return Ok(Self {
			entity_ids,
		});
	}
}

//
// MARK: 0x51 set head rotation
//

#[derive(Debug, Clone)]
pub struct SetHeadRotation {
	pub entity_id: i32,
	pub head_yaw: u8,
}

impl Packet for SetHeadRotation {
	const PACKET_ID: u8 = 0x51;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<SetHeadRotation> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetHeadRotation) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_id));
		output.push(value.head_yaw);

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SetHeadRotation {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			entity_id: crate::deserialize::varint(&mut value)?,
			head_yaw: value.remove(0),
		});
	}
}

//
// MARK: 0x5c set center chunk
//

#[derive(Debug, Clone)]
pub struct SetCenterChunk {
	pub chunk_x: i32,
	pub chunk_z: i32,
}

impl Packet for SetCenterChunk {
	const PACKET_ID: u8 = 0x5c;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<SetCenterChunk> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetCenterChunk) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.chunk_x));
		output.append(&mut crate::serialize::varint(value.chunk_z));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SetCenterChunk {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			chunk_x: crate::deserialize::varint(&mut value)?,
			chunk_z: crate::deserialize::varint(&mut value)?,
		});
	}
}

//
// MARK: 0x61 Set Entity Metadata
//

#[derive(Debug, Clone)]
pub struct SetEntityMetadata {
	pub entity_id: i32,
	pub metadata: Vec<EntityMetadata>,
}

impl Packet for SetEntityMetadata {
	const PACKET_ID: u8 = 0x61;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

#[derive(Debug, Clone)]
pub struct EntityMetadata {
	pub index: u8,
	pub value: EntityMetadataValue,
}

#[derive(Debug, Clone)]
pub enum EntityMetadataValue {
	Byte(u8),
	Varint(i32),
	Varlong(i64),
	Float(f32),
	String(String),
	TextComponent(NbtTag),
	OptionalTextComponent(Option<NbtTag>), //absence of value indicated by a 0x00, if present append 0x01 byte
	Slot(Slot),
	Boolean(bool),
	Rotations(f32, f32, f32),
	Position(i64),
	OptionalPosition(Option<i64>), //absence of value indicated by a 0x00, if present append 0x01 byte
	Direction(i32),
	OptionalUuid(bool, u128),
	BlockState(i32),
	OptionalBlockState(i32),
	Particle(i32),            //Missing a type that varies, whatever the fuck that means
	Particles(i32, Vec<i32>), //Missing a type that varies, whatever the fuck that means
	VillagerData(i32, i32, i32),
	OptionalVarint(i32),
	Pose(i32),
	CatVariant(i32),
	CowVariant(i32),
	WolfVariant(i32),
	WolfSoundVariant(i32),
	FrogVariant(i32),
	PigVariant(i32),
	ChickenVariant(i32),
	OptionalGlobalPosition(bool, bool, String, bool, i64),
	PaintingVariant(i32),
	SnifferState(i32),
	ArmadilloState(i32),
	CopperGolemState(i32),
	WeatheringCopperState(i32),
	Vector3(f32, f32, f32),
	Quaternion(f32, f32, f32, f32),
	ResolvableProfile(i32), //kind; theres more that I havent bothered with yet
}

#[allow(clippy::from_over_into)]
impl Into<i32> for EntityMetadataValue {
	fn into(self) -> i32 {
		match self {
			EntityMetadataValue::Byte(_) => 0,
			EntityMetadataValue::Varint(_) => 1,
			EntityMetadataValue::Varlong(_) => 2,
			EntityMetadataValue::Float(_) => 3,
			EntityMetadataValue::String(_) => 4,
			EntityMetadataValue::TextComponent(_) => 5,
			EntityMetadataValue::OptionalTextComponent(_) => 6,
			EntityMetadataValue::Slot(_) => 7,
			EntityMetadataValue::Boolean(_) => 8,
			EntityMetadataValue::Rotations(_, _, _) => 9,
			EntityMetadataValue::Position(_) => 10,
			EntityMetadataValue::OptionalPosition(_) => 11,
			EntityMetadataValue::Direction(_) => 12,
			EntityMetadataValue::OptionalUuid(_, _) => 13,
			EntityMetadataValue::BlockState(_) => 14,
			EntityMetadataValue::OptionalBlockState(_) => 15,
			EntityMetadataValue::Particle(_) => 16,
			EntityMetadataValue::Particles(_, _) => 17,
			EntityMetadataValue::VillagerData(_, _, _) => 18,
			EntityMetadataValue::OptionalVarint(_) => 19,
			EntityMetadataValue::Pose(_) => 20,
			EntityMetadataValue::CatVariant(_) => 21,
			EntityMetadataValue::CowVariant(_) => 22,
			EntityMetadataValue::WolfVariant(_) => 23,
			EntityMetadataValue::WolfSoundVariant(_) => 24,
			EntityMetadataValue::FrogVariant(_) => 25,
			EntityMetadataValue::PigVariant(_) => 26,
			EntityMetadataValue::ChickenVariant(_) => 27,
			EntityMetadataValue::OptionalGlobalPosition(_, _, _, _, _) => 28,
			EntityMetadataValue::PaintingVariant(_) => 29,
			EntityMetadataValue::SnifferState(_) => 30,
			EntityMetadataValue::ArmadilloState(_) => 31,
			EntityMetadataValue::CopperGolemState(_) => 32,
			EntityMetadataValue::WeatheringCopperState(_) => 33,
			EntityMetadataValue::Vector3(_, _, _) => 34,
			EntityMetadataValue::Quaternion(_, _, _, _) => 35,
			EntityMetadataValue::ResolvableProfile(_) => 36,
		}
	}
}

impl TryFrom<SetEntityMetadata> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetEntityMetadata) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_id));

		for metadata in value.metadata {
			output.push(metadata.index);
			output.append(&mut crate::serialize::varint(metadata.value.clone().into()));

			match metadata.value {
				EntityMetadataValue::Byte(a) => output.push(a),
				EntityMetadataValue::Varint(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::Varlong(_) => todo!(),
				EntityMetadataValue::Float(a) => output.append(&mut crate::serialize::float(a)),
				EntityMetadataValue::String(a) => output.append(&mut crate::serialize::string(&a)),
				EntityMetadataValue::TextComponent(a) => output.append(&mut crate::serialize::nbt_network(a)),
				EntityMetadataValue::OptionalTextComponent(a) => match a {
					Some(a) => {
						output.push(0x01);
						output.append(&mut crate::serialize::nbt_network(a));
					}
					None => {
						output.push(0x00);
					}
				},
				EntityMetadataValue::Slot(a) => output.append(&mut crate::slot::serialize_slot(Some(&a))),
				EntityMetadataValue::Boolean(a) => output.append(&mut crate::serialize::boolean(a)),
				EntityMetadataValue::Rotations(a, b, c) => {
					output.append(&mut crate::serialize::float(a));
					output.append(&mut crate::serialize::float(b));
					output.append(&mut crate::serialize::float(c));
				}
				EntityMetadataValue::Position(a) => output.append(&mut crate::serialize::long(a)),
				EntityMetadataValue::OptionalPosition(a) => match a {
					Some(a) => {
						output.push(0x01);
						output.append(&mut crate::serialize::long(a));
					}
					None => {
						output.push(0x00);
					}
				},
				EntityMetadataValue::Direction(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::OptionalUuid(a, b) => {
					output.append(&mut crate::serialize::boolean(a));
					if a {
						output.append(&mut crate::serialize::uuid(&b));
					}
				}
				EntityMetadataValue::BlockState(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::OptionalBlockState(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::Particle(_) => todo!(),
				EntityMetadataValue::Particles(_, _) => output.push(0),
				EntityMetadataValue::VillagerData(_, _, _) => todo!(),
				EntityMetadataValue::OptionalVarint(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::Pose(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::CatVariant(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::WolfVariant(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::FrogVariant(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::OptionalGlobalPosition(a, b, c, _d, _e) => {
					output.append(&mut crate::serialize::boolean(a));
					output.append(&mut crate::serialize::boolean(b));
					if b {
						output.append(&mut crate::serialize::string(&c));
					}
				}
				EntityMetadataValue::CowVariant(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::WolfSoundVariant(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::PigVariant(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::ChickenVariant(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::CopperGolemState(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::WeatheringCopperState(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::ResolvableProfile(_) => todo!(),
				EntityMetadataValue::PaintingVariant(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::SnifferState(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::ArmadilloState(a) => output.append(&mut crate::serialize::varint(a)),
				EntityMetadataValue::Vector3(a, b, c) => {
					output.append(&mut crate::serialize::float(a));
					output.append(&mut crate::serialize::float(b));
					output.append(&mut crate::serialize::float(c));
				}
				EntityMetadataValue::Quaternion(a, b, c, d) => {
					output.append(&mut crate::serialize::float(a));
					output.append(&mut crate::serialize::float(b));
					output.append(&mut crate::serialize::float(c));
					output.append(&mut crate::serialize::float(d));
				}
			}
		}

		output.push(255);

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SetEntityMetadata {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let entity_id = crate::deserialize::varint(&mut value)?;
		let mut metadata: Vec<EntityMetadata> = Vec::new();

		loop {
			let index = value.remove(0);
			if index == 0xff {
				break;
			}

			let type_id = crate::deserialize::varint(&mut value)?;
			let metadata_value = match type_id {
				0 => EntityMetadataValue::Byte(value.remove(0)),
				1 => EntityMetadataValue::Varint(crate::deserialize::varint(&mut value)?),
				2 => todo!(),
				3 => EntityMetadataValue::Float(crate::deserialize::float(&mut value)?),
				4 => EntityMetadataValue::String(crate::deserialize::string(&mut value)?),
				5 => EntityMetadataValue::TextComponent(crate::deserialize::nbt_network(&mut value)?),
				6 => {
					let nbt_present = crate::deserialize::boolean(&mut value)?;
					let nbt = if nbt_present { Some(crate::deserialize::nbt_network(&mut value)?) } else { None };
					EntityMetadataValue::OptionalTextComponent(nbt)
				}
				7 => EntityMetadataValue::Slot(crate::slot::deserialize_slot(&mut value)?.unwrap_or(Slot {
					item_count: 0,
					item_id: 0,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				})),
				8 => EntityMetadataValue::Boolean(crate::deserialize::boolean(&mut value)?),
				9 => EntityMetadataValue::Rotations(
					crate::deserialize::float(&mut value)?,
					crate::deserialize::float(&mut value)?,
					crate::deserialize::float(&mut value)?,
				),
				10 => EntityMetadataValue::Position(crate::deserialize::long(&mut value)?),
				11 => {
					let position_present = crate::deserialize::boolean(&mut value)?;
					let position = if position_present { Some(crate::deserialize::long(&mut value)?) } else { None };
					EntityMetadataValue::OptionalPosition(position)
				}
				12 => EntityMetadataValue::Direction(crate::deserialize::varint(&mut value)?),
				13 => todo!(),
				14 => EntityMetadataValue::BlockState(crate::deserialize::varint(&mut value)?),
				15 => EntityMetadataValue::OptionalBlockState(crate::deserialize::varint(&mut value)?),
				16 => todo!(),
				17 => todo!(),
				18 => EntityMetadataValue::VillagerData(
					crate::deserialize::varint(&mut value)?,
					crate::deserialize::varint(&mut value)?,
					crate::deserialize::varint(&mut value)?,
				),
				19 => EntityMetadataValue::OptionalVarint(crate::deserialize::varint(&mut value)?),
				20 => EntityMetadataValue::Pose(crate::deserialize::varint(&mut value)?),
				21 => EntityMetadataValue::CatVariant(crate::deserialize::varint(&mut value)?),
				23 => EntityMetadataValue::CowVariant(crate::deserialize::varint(&mut value)?),
				22 => EntityMetadataValue::WolfVariant(crate::deserialize::varint(&mut value)?),
				24 => EntityMetadataValue::WolfSoundVariant(crate::deserialize::varint(&mut value)?),
				25 => EntityMetadataValue::FrogVariant(crate::deserialize::varint(&mut value)?),
				26 => EntityMetadataValue::PigVariant(crate::deserialize::varint(&mut value)?),
				27 => EntityMetadataValue::ChickenVariant(crate::deserialize::varint(&mut value)?),
				28 => todo!(),
				29 => EntityMetadataValue::PaintingVariant(crate::deserialize::varint(&mut value)?),
				30 => EntityMetadataValue::SnifferState(crate::deserialize::varint(&mut value)?),
				31 => EntityMetadataValue::ArmadilloState(crate::deserialize::varint(&mut value)?),
				32 => EntityMetadataValue::CopperGolemState(crate::deserialize::varint(&mut value)?),
				33 => EntityMetadataValue::WeatheringCopperState(crate::deserialize::varint(&mut value)?),
				34 => EntityMetadataValue::Vector3(
					crate::deserialize::float(&mut value)?,
					crate::deserialize::float(&mut value)?,
					crate::deserialize::float(&mut value)?,
				),
				35 => EntityMetadataValue::Quaternion(
					crate::deserialize::float(&mut value)?,
					crate::deserialize::float(&mut value)?,
					crate::deserialize::float(&mut value)?,
					crate::deserialize::float(&mut value)?,
				),
				36 => todo!(),
				id => panic!("type_id {id} is not a recognized entity type"),
			};

			metadata.push(EntityMetadata {
				index,
				value: metadata_value,
			});
		}

		return Ok(Self {
			entity_id,
			metadata,
		});
	}
}

//
// MARK: 0x64 set equipment
//

#[derive(Debug, Clone)]
pub struct SetEquipment {
	pub entity_id: i32,
	pub equipment: Vec<(u8, Option<Slot>)>,
}

impl Packet for SetEquipment {
	const PACKET_ID: u8 = 0x64;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<SetEquipment> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetEquipment) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.entity_id));
		value.equipment.iter().enumerate().for_each(|x| {
			let mask = if x.0 < value.equipment.len() - 1 { 0b1000_0000 } else { 0b0000_0000 };
			output.push(x.1.0 + mask);
			output.append(&mut crate::slot::serialize_slot(x.1.1.as_ref()));
		});

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SetEquipment {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let entity_id = crate::deserialize::varint(&mut value)?;
		let mut equipment: Vec<(u8, Option<Slot>)> = Vec::new();

		loop {
			let slot = value.remove(0);
			let item = crate::slot::deserialize_slot(&mut value)?;
			equipment.push((slot, item));

			if slot & 0b1000_0000 == 0b1000_0000 {
				break;
			}
		}

		return Ok(Self {
			entity_id,
			equipment,
		});
	}
}

//
// MARK: 0x67 set held item
//

#[derive(Debug, Clone)]
pub struct SetHeldItem {
	pub slot: u8,
}

impl Packet for SetHeldItem {
	const PACKET_ID: u8 = 0x67;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<SetHeldItem> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetHeldItem) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.slot as i32));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SetHeldItem {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			slot: crate::deserialize::varint(&mut value)? as u8,
		});
	}
}

//
// MARK: 0x6a set player inventory slot
//

#[derive(Debug, Clone)]
pub struct SetPlayerInventorySlot {
	pub slot: i32,
	pub slot_data: Option<Slot>,
}

impl Packet for SetPlayerInventorySlot {
	const PACKET_ID: u8 = 0x6a;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<SetPlayerInventorySlot> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetPlayerInventorySlot) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.slot));
		output.append(&mut crate::slot::serialize_slot(value.slot_data.as_ref()));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SetPlayerInventorySlot {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			slot: crate::deserialize::varint(&mut value)?,
			slot_data: crate::slot::deserialize_slot(&mut value)?,
		});
	}
}

//
// MARK: 0x77 system chat message
//

#[derive(Debug, Clone)]
pub struct SystemChatMessage {
	pub content: NbtTag,
	pub overlay: bool,
}

impl Packet for SystemChatMessage {
	const PACKET_ID: u8 = 0x77;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<SystemChatMessage> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SystemChatMessage) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::nbt_network(value.content));
		output.append(&mut crate::serialize::boolean(value.overlay));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SystemChatMessage {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			content: crate::deserialize::nbt_network(&mut value)?,
			overlay: crate::deserialize::boolean(&mut value)?,
		});
	}
}

//
// MARK: 0x78 set tab list header and footer
//

#[derive(Debug, Clone)]
pub struct SetTabListHeaderAndFooter {
	pub header: NbtTag,
	pub footer: NbtTag,
}

impl Packet for SetTabListHeaderAndFooter {
	const PACKET_ID: u8 = 0x78;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<SetTabListHeaderAndFooter> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetTabListHeaderAndFooter) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::nbt_network(value.header));
		output.append(&mut crate::serialize::nbt_network(value.footer));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for SetTabListHeaderAndFooter {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			header: crate::deserialize::nbt_network(&mut value)?,
			footer: crate::deserialize::nbt_network(&mut value)?,
		});
	}
}

//
// MARK: 0x87 server links
//

#[derive(Debug, Clone)]
pub struct ServerLinks {
	pub links: Vec<ServerLink>,
}

impl Packet for ServerLinks {
	const PACKET_ID: u8 = 0x87;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Play
	}
}

impl TryFrom<ServerLinks> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ServerLinks) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.links.len() as i32));
		for link in value.links {
			output.append(&mut link.into());
		}

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for ServerLinks {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let links_len = crate::deserialize::varint(&mut value)?;
		let links: Vec<ServerLink> = (0..links_len).map(|_| ServerLink::try_from(&mut value).unwrap()).collect();

		return Ok(Self {
			links,
		});
	}
}
