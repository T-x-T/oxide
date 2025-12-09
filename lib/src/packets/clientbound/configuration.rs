use super::*;
use crate::NbtTag;

//
// MARK: 0x0e ClientBoundKnownPacks
//
#[derive(Debug, Clone, Default)]
pub struct ClientboundKnownPacks {
	pub known_packs: Vec<crate::Datapack>,
}

impl Packet for ClientboundKnownPacks {
	const PACKET_ID: u8 = 0x0e;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Configuration
	}
}

impl TryFrom<ClientboundKnownPacks> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ClientboundKnownPacks) -> Result<Self, Box<dyn Error>> {
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

impl TryFrom<Vec<u8>> for ClientboundKnownPacks {
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
// MARK: 0x07 RegistryData
//

#[derive(Debug, Clone, Default)]
pub struct RegistryData {
	pub registry_id: String,
	pub entries: Vec<RegistryDataEntry>,
}

impl Packet for RegistryData {
	const PACKET_ID: u8 = 0x07;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Configuration
	}
}

#[derive(Debug, Clone, Default)]
pub struct RegistryDataEntry {
	pub entry_id: String,
	pub has_data: bool,
	pub data: Option<crate::nbt::NbtTag>,
}

impl TryFrom<RegistryData> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: RegistryData) -> Result<Self, Box<dyn Error>> {
		let mut data: Vec<u8> = Vec::new();

		data.append(&mut crate::serialize::string(&value.registry_id));
		data.append(&mut crate::serialize::varint(value.entries.len() as i32));
		value.entries.iter().for_each(|x| {
			data.append(&mut crate::serialize::string(&x.entry_id));
			data.append(&mut crate::serialize::boolean(x.has_data));
			if x.has_data {
				data.append(&mut crate::serialize::nbt_network(x.clone().data.unwrap()));
			}
		});

		return Ok(data);
	}
}

impl TryFrom<Vec<u8>> for RegistryData {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let registry_id = crate::deserialize::string(&mut value)?;
		let len = crate::deserialize::varint(&mut value)?;
		let mut output = RegistryData { registry_id, entries: Default::default() };
		for _ in 0..len {
			let entry_id = crate::deserialize::string(&mut value)?;
			let has_data = crate::deserialize::boolean(&mut value)?;
			let data: Option<crate::nbt::NbtTag> = if has_data { Some(crate::deserialize::nbt_network(&mut value)?) } else { None };

			output.entries.push(RegistryDataEntry { entry_id, has_data, data });
		}

		return Ok(output);
	}
}

//
// MARK: 0x03 FinishConfiguration
//

#[derive(Debug, Clone, Default)]
pub struct FinishConfiguration {}

impl Packet for FinishConfiguration {
	const PACKET_ID: u8 = 0x03;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Configuration
	}
}

impl TryFrom<FinishConfiguration> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(_value: FinishConfiguration) -> Result<Self, Box<dyn Error>> {
		return Ok(Vec::new());
	}
}

impl TryFrom<Vec<u8>> for FinishConfiguration {
	type Error = Box<dyn Error>;

	fn try_from(_value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self {});
	}
}

//
// MARK: 0x0d UpdateTags
//

#[derive(Debug, Clone, Default)]
pub struct UpdateTags {
	pub data: Vec<(String, Vec<Tag>)>,
}

#[derive(Debug, Clone, Default)]
pub struct Tag {
	pub name: String,
	pub entries: Vec<i32>,
}

impl Packet for UpdateTags {
	const PACKET_ID: u8 = 0x0d;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Configuration
	}
}

impl TryFrom<UpdateTags> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: UpdateTags) -> Result<Self, Box<dyn Error>> {
		let mut data: Vec<u8> = Vec::new();

		data.append(&mut crate::serialize::varint(value.data.len() as i32));
		for entry in value.data {
			data.append(&mut crate::serialize::string(&entry.0));
			data.append(&mut crate::serialize::varint(entry.1.len() as i32));
			for tag in entry.1 {
				data.append(&mut crate::serialize::string(&tag.name));
				data.append(&mut crate::serialize::varint(tag.entries.len() as i32));
				for tag_entry in tag.entries {
					data.append(&mut crate::serialize::varint(tag_entry));
				}
			}
		}

		return Ok(data);
	}
}

impl TryFrom<Vec<u8>> for UpdateTags {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let data_len = crate::deserialize::varint(&mut value)?;

		let mut data: Vec<(String, Vec<Tag>)> = Vec::new();
		for _ in 0..data_len {
			let registry = crate::deserialize::string(&mut value)?;
			let mut tags: Vec<Tag> = Vec::new();
			let tag_len = crate::deserialize::varint(&mut value)?;
			for _ in 0..tag_len {
				let tag_name = crate::deserialize::string(&mut value)?;
				let mut entries: Vec<i32> = Vec::new();
				let entries_len = crate::deserialize::varint(&mut value)?;
				for _ in 0..entries_len {
					entries.push(crate::deserialize::varint(&mut value)?);
				}
				tags.push(Tag { name: tag_name, entries });
			}
			data.push((registry, tags));
		}

		return Ok(Self { data });
	}
}

//
// MARK: 0x10 server links
//

#[derive(Debug, Clone)]
pub struct ServerLinks {
	pub links: Vec<(NbtTag, String)>,
}

impl Packet for ServerLinks {
	const PACKET_ID: u8 = 0x10;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Configuration
	}
}

impl TryFrom<ServerLinks> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ServerLinks) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.append(&mut crate::serialize::varint(value.links.len() as i32));
		for link in value.links {
			output.append(&mut crate::serialize::boolean(false));
			output.append(&mut crate::serialize::nbt_network(link.0));
			output.append(&mut crate::serialize::string(&link.1));
		}

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for ServerLinks {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let links_len = crate::deserialize::varint(&mut value)?;
		let links: Vec<(NbtTag, String)> = (0..links_len)
			.map(|_| {
				value.remove(0);
				return (crate::deserialize::nbt_network(&mut value).unwrap(), crate::deserialize::string(&mut value).unwrap());
			})
			.collect();

		return Ok(Self { links });
	}
}
//
// MARK: 0x12 show dialog
//

#[derive(Debug, Clone)]
pub struct ShowDialog {
	pub dialog: NbtTag,
}

impl Packet for ShowDialog {
	const PACKET_ID: u8 = 0x12;
	fn get_target() -> PacketTarget {
		PacketTarget::Client
	}
	fn get_state() -> ConnectionState {
		ConnectionState::Configuration
	}
}

impl TryFrom<ShowDialog> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ShowDialog) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();
		output.append(&mut crate::serialize::nbt_network(value.dialog));

		return Ok(output);
	}
}

impl TryFrom<Vec<u8>> for ShowDialog {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(Self { dialog: crate::deserialize::nbt_network(&mut value)? });
	}
}
