use std::error::Error;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Slot {
	item_id: u16,
	count: u8,
}

impl Slot {
	pub fn new(item_id: u16, count: u8) -> Slot {
		return Slot { item_id, count };
	}

	pub fn new_from_name(item_name: &str, count: u8) -> Result<Slot, Box<dyn Error>> {
		let Some(item_id) = data::items::get_item_id_by_name(item_name) else {
			return Err(Box::new(crate::CustomError::InvalidInput(format!("Slot::new_from_name got an invalid item_name: {item_name}"))));
		};
		let item_id = u16::try_from(item_id)?;
		return Ok(Slot { item_id, count });
	}
}
