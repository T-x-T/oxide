use std::error::Error;

use crate::NbtTag;

#[derive(Debug, Clone)]
pub enum ServerLink {
	BuiltIn(BuiltInServerLink, String),
	Custom(NbtTag, String),
}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum BuiltInServerLink {
	BugReport,
	CommunityGuidelines,
	Support,
	Status,
	Feedback,
	Community,
	Website,
	Forums,
	News,
	Announcements,
}

impl TryFrom<u8> for BuiltInServerLink {
	type Error = Box<dyn Error>;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		return match value {
			0 => Ok(Self::BugReport),
			1 => Ok(Self::CommunityGuidelines),
			2 => Ok(Self::Support),
			3 => Ok(Self::Status),
			4 => Ok(Self::Feedback),
			5 => Ok(Self::Community),
			6 => Ok(Self::Website),
			7 => Ok(Self::Forums),
			8 => Ok(Self::News),
			9 => Ok(Self::Announcements),
			_ => Err(Box::new(crate::CustomError::InvalidInput(format!("Value {value} cannot be cast to a BuiltInServerLink")))),
		};
	}
}

impl TryFrom<&mut Vec<u8>> for ServerLink {
	type Error = Box<dyn Error>;

	fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
		if value.is_empty() {
			return Err(Box::new(crate::CustomError::InputEmpty));
		}

		let is_built_in = value.remove(0);
		let result = if is_built_in == 1 {
			let raw_variant = crate::deserialize::varint(value)? as u8;
			let variant = BuiltInServerLink::try_from(raw_variant)?;
			let link = crate::deserialize::string(value)?;

			ServerLink::BuiltIn(variant, link)
		} else {
			let text_component = crate::deserialize::nbt_network(value)?;
			let link = crate::deserialize::string(value)?;

			ServerLink::Custom(text_component, link)
		};

		return Ok(result);
	}
}

impl From<ServerLink> for Vec<u8> {
	fn from(value: ServerLink) -> Self {
		let mut output: Vec<u8> = Vec::new();

		match value {
			ServerLink::BuiltIn(built_in_server_link, link) => {
				output.push(1);
				output.append(&mut crate::serialize::varint(built_in_server_link as i32));
				output.append(&mut crate::serialize::string(&link));
			}
			ServerLink::Custom(nbt_tag, link) => {
				output.push(0);
				output.append(&mut crate::serialize::nbt_network(nbt_tag));
				output.append(&mut crate::serialize::string(&link));
			}
		};

		return output;
	}
}
