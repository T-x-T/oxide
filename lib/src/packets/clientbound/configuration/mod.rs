use super::*;

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
			println!("aaaaaaa");
			output.known_packs.push(crate::Datapack {
				namespace: crate::deserialize::string(&mut value)?,
				id: crate::deserialize::string(&mut value)?,
				version: crate::deserialize::string(&mut value)?,
			});
		}

		return Ok(output);
	}
}

#[derive(Debug, Clone, Default)]
pub struct RegistryData<'a> {
	pub registry_id: String,
	pub entries: Vec<RegistryDataEntry<'a>>,
}

#[derive(Debug, Clone, Default)]
pub struct RegistryDataEntry<'a> {
	pub entry_id: String,
	pub has_data: bool,
	pub data: crate::nbt::NbtTag<'a>,
}

impl<'a> TryFrom<RegistryData<'a>> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: RegistryData) -> Result<Self, Box<dyn Error>> {
		let mut data: Vec<u8> = Vec::new();

		data.append(&mut crate::serialize::string(&value.registry_id));
		value.entries.iter().for_each(|x| {
			data.append(&mut crate::serialize::string(&x.entry_id));
			data.append(&mut crate::serialize::bool(&x.has_data));
			data.append(&mut crate::serialize::nbt(x.clone().data));
		});
		
		return Ok(data);
	}
}

//TODO: missing deserialization for nbt
impl<'a> TryFrom<Vec<u8>> for RegistryData<'a> {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let registry_id = crate::deserialize::string(&mut value)?;
		let len = crate::deserialize::varint(&mut value)?;

		let mut output = RegistryData {
			registry_id,
			entries: Default::default(),
		};
		// for _ in 0..=len {
		// 	output.entries.push(RegistryDataEntry {
		// 		entry_id: crate::deserialize::string(&mut value)?,
		// 		has_data: crate::deserialize::boolean(&mut value)?,
		// 		data: crate::deserialize::nbt(&mut value)?,
		// 	});
		// }

		return Ok(output);
	}
}