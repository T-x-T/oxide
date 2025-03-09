use super::*;

//
// MARK: ClientBoundKnownPacks
//
#[derive(Debug, Clone, Default)]
pub struct ClientboundKnownPacks {
	pub known_packs: Vec<crate::Datapack>,
}


impl TryFrom<ClientboundKnownPacks> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: ClientboundKnownPacks) -> Result<Self, Box<dyn Error>> {
		let data: Vec<u8> = value.known_packs.clone().into_iter()
			.map(|x| vec![crate::serialize::string(x.namespace.as_str()), crate::serialize::string(x.id.as_str()), crate::serialize::string(x.version.as_str())])
			.flatten()
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
// MARK: RegistryData
//

#[derive(Debug, Clone, Default)]
pub struct RegistryData {
	pub registry_id: String,
	pub entry_count: i32,
	pub entries: Vec<RegistryDataEntry>,
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
		data.append(&mut crate::serialize::varint(value.entry_count));
		value.entries.iter().for_each(|x| {
			data.append(&mut crate::serialize::string(&x.entry_id));
			data.append(&mut crate::serialize::bool(&x.has_data));
			if x.has_data {
				data.append(&mut crate::serialize::nbt(x.clone().data.unwrap()));
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
		let mut output = RegistryData {
			registry_id,
			entries: Default::default(),
			entry_count: len,
		};
		for _ in 0..len {
			let entry_id = crate::deserialize::string(&mut value)?;
			let has_data = crate::deserialize::boolean(&mut value)?;
			let data: Option<crate::nbt::NbtTag> = if has_data {
				Some(crate::deserialize::nbt(&mut value)?)
			} else {
				None
			};

			output.entries.push(RegistryDataEntry {
				entry_id,
				has_data,
				data,
			});
		}

		return Ok(output);
	}
}

//
// MARK: FinishConfiguration
//

#[derive(Debug, Clone, Default)]
pub struct FinishConfiguration {

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
		return Ok(Self {  });
	}
}