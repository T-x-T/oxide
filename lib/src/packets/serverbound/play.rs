use super::*;
use crate::types::*;

//
// MARK: 0x00 confirm teleportation
//

#[derive(Debug, Clone)]
pub struct ConfirmTeleportation {
	pub teleport_id: i32,
}

impl Packet for ConfirmTeleportation {
	const PACKET_ID: u8 = 0x00;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<ConfirmTeleportation> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ConfirmTeleportation) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::varint(value.teleport_id));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for ConfirmTeleportation {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(ConfirmTeleportation {
			teleport_id: crate::deserialize::varint(&mut value)?,
		})
	}
}

//
// MARK: 0x06 chat command
//

#[derive(Debug, Clone)]
pub struct ChatCommand {
	pub command: String,
}

impl Packet for ChatCommand {
	const PACKET_ID: u8 = 0x06;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<ChatCommand> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ChatCommand) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::string(&value.command));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for ChatCommand {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(ChatCommand {
			command: crate::deserialize::string(&mut value)?,
		})
	}
}

//
// MARK: 0x08 chat message
//

#[derive(Debug, Clone)]
pub struct ChatMessage {
	pub message: String,
	pub timestamp: i64,
	pub salt: i64,
	pub signature: Vec<u8>,
	pub message_count: i32,
	pub acknowledged: Vec<u8>, //len should always be 3 for 20 bits
	pub checksum: u8,
}

impl Packet for ChatMessage {
	const PACKET_ID: u8 = 0x08;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<ChatMessage> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ChatMessage) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::string(&value.message));
		result.append(&mut crate::serialize::long(value.timestamp));
		result.append(&mut crate::serialize::long(value.salt));
		if value.signature.is_empty() {
      result.append(&mut crate::serialize::boolean(false));
		} else {
      result.append(&mut crate::serialize::boolean(true));
      result.append(&mut crate::serialize::varint(value.signature.len() as i32));
      value.signature.iter().for_each(|x| result.push(*x));
		}
		result.append(&mut crate::serialize::varint(value.message_count));
		value.acknowledged.iter().for_each(|x| result.push(*x));
		result.push(value.checksum);

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for ChatMessage {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
	  let message = crate::deserialize::string(&mut value)?;
	  let timestamp = crate::deserialize::long(&mut value)?;
	  let salt = crate::deserialize::long(&mut value)?;
		let has_signature = crate::deserialize::boolean(&mut value)?;
		let signature: Vec<u8> = if has_signature {
		  let signature_length = crate::deserialize::varint(&mut value)?;
		  (0..signature_length).map(|_| value.remove(0)).collect()
		} else {
      vec![]
		};
	  let message_count = crate::deserialize::varint(&mut value)?;
		let acknowledged: Vec<u8> = (0..3).map(|_| value.remove(0)).collect();
	  let checksum = value.remove(0);

		return Ok(Self {
      message,
      timestamp,
      salt,
      signature,
      message_count,
      acknowledged,
      checksum,
		});
	}
}

//
// MARK: 0x19 interact
//

#[derive(Debug, Clone)]
pub struct Interact {
	pub entity_id: i32,
	pub interact_type: i32,
	pub target_x: Option<f32>,
	pub target_y: Option<f32>,
	pub target_z: Option<f32>,
	pub hand: Option<i32>,
	pub sneak_key_pressed: bool,
}

impl Packet for Interact {
	const PACKET_ID: u8 = 0x19;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<Interact> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: Interact) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::varint(value.entity_id));
		result.append(&mut crate::serialize::varint(value.interact_type));
		if value.interact_type == 2 {
      result.push(1);
		  result.append(&mut crate::serialize::float(value.target_x.unwrap()));
      result.push(1);
		  result.append(&mut crate::serialize::float(value.target_y.unwrap()));
      result.push(1);
		  result.append(&mut crate::serialize::float(value.target_z.unwrap()));
		}
		if value.interact_type == 0 || value.interact_type == 2 {
      result.push(1);
      result.append(&mut crate::serialize::varint(value.hand.unwrap()));
		}
		result.append(&mut crate::serialize::boolean(value.sneak_key_pressed));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for Interact {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
	  let entity_id: i32 = crate::deserialize::varint(&mut value)?;
	  let interact_type: i32 = crate::deserialize::varint(&mut value)?;
		let target_x: Option<f32> = if interact_type == 2 {
		  value.remove(0);
		  Some(crate::deserialize::float(&mut value)?)
		} else {
		  None
		};
		let target_y: Option<f32> = if interact_type == 2 {
		  value.remove(0);
		  Some(crate::deserialize::float(&mut value)?)
		} else {
		  None
		};
		let target_z: Option<f32> = if interact_type == 2 {
		  value.remove(0);
		  Some(crate::deserialize::float(&mut value)?)
		} else {
		  None
		};
		let hand: Option<i32> = if interact_type == 0 || interact_type == 2 {
		  value.remove(0);
		  Some(crate::deserialize::varint(&mut value)?)
		} else {
		  None
		};
		let sneak_key_pressed: bool = crate::deserialize::boolean(&mut value)?;

	  return Ok(Interact {
      entity_id,
      interact_type,
      target_x,
      target_y,
      target_z,
      hand,
      sneak_key_pressed,
		});
	}
}

//
// MARK: 0x1d set player position
//

#[derive(Debug, Clone)]
pub struct SetPlayerPosition {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub flags: u8,
}

impl Packet for SetPlayerPosition {
	const PACKET_ID: u8 = 0x1d;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<SetPlayerPosition> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetPlayerPosition) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::double(value.x));
		result.append(&mut crate::serialize::double(value.y));
		result.append(&mut crate::serialize::double(value.z));
		result.push(value.flags);

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for SetPlayerPosition {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(SetPlayerPosition {
			x: crate::deserialize::double(&mut value)?,
			y: crate::deserialize::double(&mut value)?,
			z: crate::deserialize::double(&mut value)?,
			flags: value.remove(0),
		})
	}
}

//
// MARK: 0x1e set player position and rotation
//

#[derive(Debug, Clone)]
pub struct SetPlayerPositionAndRotation {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub yaw: f32,
	pub pitch: f32,
	pub flags: u8,
}

impl Packet for SetPlayerPositionAndRotation {
	const PACKET_ID: u8 = 0x1e;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<SetPlayerPositionAndRotation> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetPlayerPositionAndRotation) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::double(value.x));
		result.append(&mut crate::serialize::double(value.y));
		result.append(&mut crate::serialize::double(value.z));
		result.append(&mut crate::serialize::float(value.yaw));
		result.append(&mut crate::serialize::float(value.pitch));
		result.push(value.flags);

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for SetPlayerPositionAndRotation {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			x: crate::deserialize::double(&mut value)?,
			y: crate::deserialize::double(&mut value)?,
			z: crate::deserialize::double(&mut value)?,
			yaw: crate::deserialize::float(&mut value)?,
			pitch: crate::deserialize::float(&mut value)?,
			flags: value.remove(0),
		})
	}
}

//
// MARK: 0x1f set player rotation
//

#[derive(Debug, Clone)]
pub struct SetPlayerRotation {
	pub yaw: f32,
	pub pitch: f32,
	pub flags: u8,
}

impl Packet for SetPlayerRotation {
	const PACKET_ID: u8 = 0x1f;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<SetPlayerRotation> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetPlayerRotation) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::float(value.yaw));
		result.append(&mut crate::serialize::float(value.pitch));
		result.push(value.flags);

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for SetPlayerRotation {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			yaw: crate::deserialize::float(&mut value)?,
			pitch: crate::deserialize::float(&mut value)?,
			flags: value.remove(0),
		})
	}
}

//
// MARK: 0x23 pick item from block
//

#[derive(Debug, Clone)]
pub struct PickItemFromBlock {
	pub location: Position,
	pub include_data: bool,
}

impl Packet for PickItemFromBlock {
	const PACKET_ID: u8 = 0x23;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<PickItemFromBlock> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: PickItemFromBlock) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::position(&value.location));
		result.append(&mut crate::serialize::boolean(value.include_data));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for PickItemFromBlock {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			location: crate::deserialize::position(&mut value)?,
			include_data: crate::deserialize::boolean(&mut value)?,
		})
	}
}

//
// MARK: 0x28 player action
//

#[derive(Debug, Clone)]
pub struct PlayerAction {
  pub status: i32,
  pub location: crate::Position,
  pub face: u8,
  pub sequence: i32,
}

impl Packet for PlayerAction {
	const PACKET_ID: u8 = 0x28;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<PlayerAction> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: PlayerAction) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::varint(value.status));
		result.append(&mut crate::serialize::position(&value.location));
		result.push(value.face);
		result.append(&mut crate::serialize::varint(value.sequence));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for PlayerAction {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			status: crate::deserialize::varint(&mut value)?,
			location: crate::deserialize::position(&mut value)?,
			face: value.remove(0),
			sequence: crate::deserialize::varint(&mut value)?,
		})
	}
}

//
// MARK: 0x34 set hand item
//

#[derive(Debug, Clone)]
pub struct SetHandItem {
  pub slot: i16,
}

impl Packet for SetHandItem {
	const PACKET_ID: u8 = 0x34;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<SetHandItem> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetHandItem) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::short(value.slot));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for SetHandItem {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			slot: crate::deserialize::short(&mut value)?,
		})
	}
}

//
// MARK: 0x37 set creative mode slot
//

#[derive(Debug, Clone)]
pub struct SetCreativeModeSlot {
  pub slot: i16,
  pub item: Slot,
}

impl Packet for SetCreativeModeSlot {
	const PACKET_ID: u8 = 0x37;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<SetCreativeModeSlot> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetCreativeModeSlot) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::short(value.slot));
		result.append(&mut crate::serialize::slot(&value.item));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for SetCreativeModeSlot {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			slot: crate::deserialize::short(&mut value)?,
			item: crate::deserialize::slot(&mut value)?,
		})
	}
}

//
// MARK: 0x3c swing arm
//

#[derive(Debug, Clone)]
pub struct SwingArm {
  pub hand: i32,
}

impl Packet for SwingArm {
	const PACKET_ID: u8 = 0x3c;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<SwingArm> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SwingArm) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::varint(value.hand));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for SwingArm {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			hand: crate::deserialize::varint(&mut value)?,
		})
	}
}

//
// MARK: 0x3f use item on
//

#[derive(Debug, Clone)]
pub struct UseItemOn {
  pub hand: i32,
  pub location: crate::Position,
  pub face: u8,
  pub cursor_position_x: f32,
  pub cursor_position_y: f32,
  pub cursor_position_z: f32,
  pub inside_block: bool,
  pub world_border_hit: bool,
  pub sequence: i32,
}

impl Packet for UseItemOn {
	const PACKET_ID: u8 = 0x3f;
  fn get_target() -> PacketTarget { PacketTarget::Server }
  fn get_state() -> ConnectionState { ConnectionState::Play }
}

impl TryFrom<UseItemOn> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: UseItemOn) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::varint(value.hand));
		result.append(&mut crate::serialize::position(&value.location));
		result.push(value.face);
		result.append(&mut crate::serialize::float(value.cursor_position_x));
		result.append(&mut crate::serialize::float(value.cursor_position_y));
		result.append(&mut crate::serialize::float(value.cursor_position_z));
		result.append(&mut crate::serialize::boolean(value.inside_block));
		result.append(&mut crate::serialize::boolean(value.world_border_hit));
		result.append(&mut crate::serialize::varint(value.sequence));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for UseItemOn {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			hand: crate::deserialize::varint(&mut value)?,
			location: crate::deserialize::position(&mut value)?,
			face: value.remove(0),
			cursor_position_x: crate::deserialize::float(&mut value)?,
			cursor_position_y: crate::deserialize::float(&mut value)?,
			cursor_position_z: crate::deserialize::float(&mut value)?,
			inside_block: crate::deserialize::boolean(&mut value)?,
			world_border_hit: crate::deserialize::boolean(&mut value)?,
			sequence: crate::deserialize::varint(&mut value)?,
		})
	}
}
