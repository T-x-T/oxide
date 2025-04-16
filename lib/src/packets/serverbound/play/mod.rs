use super::*;

//
// MARK: 0x1C SetPlayerPosition
//

#[derive(Debug, Clone)]
pub struct SetPlayerPosition {
	pub x: f64,
	pub feet_y: f64,
	pub z: f64,
	pub flags: u8,
}

impl TryFrom<SetPlayerPosition> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: SetPlayerPosition) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::double(value.x));
		result.append(&mut crate::serialize::double(value.feet_y));
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
			feet_y: crate::deserialize::double(&mut value)?,
			z: crate::deserialize::double(&mut value)?,
			flags: value.remove(0),
		})
	}
}