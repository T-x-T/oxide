use std::{
	error::Error,
	fmt::{Display, Formatter},
};

#[cfg(test)]
mod test;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Slot {
	item_id: u16,
	count: u8,
}

#[derive(Debug, Clone)]
pub enum SlotError {
	MaxStackSizeExceeded,
}

impl Display for SlotError {
	fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
		return match self {
			SlotError::MaxStackSizeExceeded => write!(f, "max stack size for slot got exceeded"),
		};
	}
}

impl Error for SlotError {}

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

	pub fn get_item_id(&self) -> u16 {
		return self.item_id;
	}

	pub fn get_item_name(&self) -> &'static str {
		return data::items::get_item_name_by_id(self.get_item_id() as i32).expect("Slot::get_item_name encountered an invalid self.item_id");
	}

	pub fn get_count(&self) -> u8 {
		return self.count;
	}

	pub fn get_max_stack_size(&self) -> u8 {
		let item_name = self.get_item_name();
		let all_items = data::items::get_items();
		let item = all_items.get(item_name).expect("Slot::get_max_stack_size couldnt get an Item from item_name");
		return item.max_stack_size;
	}

	pub fn set_count_mut(&mut self, count: u8) -> Result<(), Box<dyn Error>> {
		if count > self.get_max_stack_size() {
			return Err(Box::new(SlotError::MaxStackSizeExceeded));
		}
		self.count = count;
		return Ok(());
	}

	pub fn add_count_mut(&mut self, count: u8) -> Result<(), Box<dyn Error>> {
		let new_count = self.count + count;
		if new_count > self.get_max_stack_size() {
			return Err(Box::new(SlotError::MaxStackSizeExceeded));
		}
		self.count = new_count;
		return Ok(());
	}

	pub fn add_count_mut_with_overflow(&mut self, count: u8) -> Result<Slot, Box<dyn Error>> {
		let new_count = self.count + count;
		let max_stack_size = self.get_max_stack_size();
		let overflow = new_count.saturating_sub(max_stack_size);
		if overflow > max_stack_size {
			return Err(Box::new(SlotError::MaxStackSizeExceeded));
		}
		if overflow > 0 {
			self.count = max_stack_size;
		} else {
			self.count = new_count;
		}
		return Ok(Slot::new(self.get_item_id(), overflow));
	}
}
