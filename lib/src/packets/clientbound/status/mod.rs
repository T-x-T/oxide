use super::*;


#[derive(Debug, Clone)]
pub struct StatusResponse {
	pub status: String,
}

impl TryFrom<StatusResponse> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: StatusResponse) -> Result<Self, Box<dyn Error>> {
		let mut result: Vec<u8> = Vec::new();

		result.append(&mut crate::serialize::string(&value.status));

		return Ok(result);
	}
}

impl TryFrom<Vec<u8>> for StatusResponse {
	type Error = Box<dyn Error>;

	fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
		return Ok(StatusResponse {
			status: crate::deserialize::string(&mut value)?,
		})
	}
}