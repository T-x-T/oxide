use super::*;

//
// MARK: 0x07 ServerboundKnownPackets
//

#[derive(Debug, Clone, Default)]
pub struct ServerboundKnownPackets {
	pub known_packs: Vec<crate::Datapack>,
}

impl Packet for ServerboundKnownPackets {
	const PACKET_ID: u8 = 0x07;
	fn get_target() -> PacketTarget {
		PacketTarget::Server
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Configuration
	}
}

impl TryFrom<ServerboundKnownPackets> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ServerboundKnownPackets) -> Result<Self, Box<dyn Error>> {
		let data: Vec<u8> = value
			.known_packs
			.clone()
			.into_iter()
			.flat_map(|x| {
				vec![
					crate::serialize::string(x.namespace.as_str()),
					crate::serialize::string(x.id.as_str()),
					crate::serialize::string(x.version.as_str()),
				]
			})
			.flatten()
			.collect();

		return Ok(crate::serialize::prefixed_array(data, value.known_packs.len() as i32));
	}
}

impl TryFrom<Vec<u8>> for ServerboundKnownPackets {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let len = crate::deserialize::varint(&mut value)?;

		let mut output = Self::default();
		for _ in 0..len {
			output.known_packs.push(crate::Datapack {
				namespace: crate::deserialize::string(&mut value)?,
				id: crate::deserialize::string(&mut value)?,
				version: crate::deserialize::string(&mut value)?,
			});
		}

		return Ok(output);
	}
}

//
// MARK: 0x03 AcknowledgeFinishConfiguration
//

#[derive(Debug, Clone, Default)]
pub struct AcknowledgeFinishConfiguration {}

impl Packet for AcknowledgeFinishConfiguration {
	const PACKET_ID: u8 = 0x03;
	fn get_target() -> PacketTarget {
		PacketTarget::Server
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Configuration
	}
}

impl TryFrom<AcknowledgeFinishConfiguration> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(_value: AcknowledgeFinishConfiguration) -> Result<Self, Box<dyn Error>> {
		return Ok(Vec::new());
	}
}

impl TryFrom<Vec<u8>> for AcknowledgeFinishConfiguration {
	type Error = Box<dyn Error>;

	fn try_from(_value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {});
	}
}
