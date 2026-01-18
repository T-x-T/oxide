#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NbtTag {
	Byte(String, u8),
	Short(String, i16),
	Int(String, i32),
	Long(String, i64),
	Float(String, f32),
	Double(String, f64),
	ByteArray(String, Vec<u8>),
	String(String, String),
	List(String, Vec<NbtListTag>),
	TagCompound(String, Vec<NbtTag>),
	Root(Vec<NbtTag>),
	IntArray(String, Vec<i32>),
	LongArray(String, Vec<i64>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NbtListTag {
	Byte(u8),
	Short(i16),
	Int(i32),
	Long(i64),
	Float(f32),
	Double(f64),
	ByteArray(Vec<u8>),
	String(String),
	List(Vec<NbtListTag>),
	TagCompound(Vec<NbtTag>),
	IntArray(Vec<i32>),
	LongArray(Vec<i64>),
}

impl Default for NbtTag {
	fn default() -> Self {
		return NbtTag::TagCompound(String::new(), Vec::new());
	}
}

impl Default for NbtListTag {
	fn default() -> Self {
		return NbtListTag::TagCompound(Vec::new());
	}
}

impl NbtTag {
	pub fn get_compound_children(&self) -> Vec<NbtTag> {
		match self {
			NbtTag::TagCompound(_, p) => return p.clone(),
			_ => return Vec::new(),
		}
	}

	pub fn get_list_children(&self) -> Vec<NbtListTag> {
		match self {
			NbtTag::List(_, p) => return p.clone(),
			_ => return Vec::new(),
		}
	}

	pub fn get_child(&self, description: &str) -> Option<&NbtTag> {
		match self {
			NbtTag::TagCompound(_, p) => {
				for tag in p {
					if tag.get_description() == description {
						return Some(tag);
					}
				}
				return None;
			}
			NbtTag::Root(p) => {
				for tag in p {
					if tag.get_description() == description {
						return Some(tag);
					}
				}
				return None;
			}
			_ => return None,
		}
	}

	pub fn get_child_mut(&mut self, description: &str) -> Option<&mut NbtTag> {
		match self {
			NbtTag::TagCompound(_, p) => {
				for tag in p {
					if tag.get_description() == description {
						return Some(tag);
					}
				}
				return None;
			}
			NbtTag::Root(p) => {
				for tag in p {
					if tag.get_description() == description {
						return Some(tag);
					}
				}
				return None;
			}
			_ => return None,
		}
	}

	pub fn get_description(&self) -> &str {
		return match self {
			NbtTag::Byte(d, _) => d,
			NbtTag::Short(d, _) => d,
			NbtTag::Int(d, _) => d,
			NbtTag::Long(d, _) => d,
			NbtTag::Float(d, _) => d,
			NbtTag::Double(d, _) => d,
			NbtTag::ByteArray(d, _) => d,
			NbtTag::String(d, _) => d,
			NbtTag::List(d, _) => d,
			NbtTag::TagCompound(d, _) => d,
			NbtTag::IntArray(d, _) => d,
			NbtTag::LongArray(d, _) => d,
			NbtTag::Root(_) => "",
		};
	}

	pub fn as_byte(&self) -> u8 {
		match self {
			NbtTag::Byte(_, p) => return *p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_short(&self) -> i16 {
		match self {
			NbtTag::Short(_, p) => return *p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_int(&self) -> i32 {
		match self {
			NbtTag::Int(_, p) => return *p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_long(&self) -> i64 {
		match self {
			NbtTag::Long(_, p) => return *p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_float(&self) -> f32 {
		match self {
			NbtTag::Float(_, p) => return *p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_double(&self) -> f64 {
		match self {
			NbtTag::Double(_, p) => return *p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_byte_array(&self) -> Vec<u8> {
		match self {
			NbtTag::ByteArray(_, p) => return p.clone(),
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_string(&self) -> &str {
		match self {
			NbtTag::String(_, p) => return p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_list(&self) -> Vec<NbtListTag> {
		match self {
			NbtTag::List(_, p) => return p.clone(),
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_list_mut(&mut self) -> &mut Vec<NbtListTag> {
		match self {
			NbtTag::List(_, p) => return p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_tag_compound(&self) -> Vec<NbtTag> {
		match self {
			NbtTag::TagCompound(_, p) => return p.clone(),
			NbtTag::Root(p) => return p.clone(),
			_ => panic!("wrong type of Tag!"),
		}
	}
	pub fn as_tag_compound_mut(&mut self) -> &mut Vec<NbtTag> {
		match self {
			NbtTag::TagCompound(_, p) => return p,
			NbtTag::Root(p) => return p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_int_array(&self) -> Vec<i32> {
		match self {
			NbtTag::IntArray(_, p) => return p.clone(),
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_long_array(&self) -> Vec<i64> {
		match self {
			NbtTag::LongArray(_, p) => return p.clone(),
			_ => panic!("wrong type of Tag!"),
		}
	}
}

impl NbtListTag {
	pub fn get_compound_children(&self) -> Vec<NbtTag> {
		match self {
			NbtListTag::TagCompound(p) => return p.clone(),
			_ => return Vec::new(),
		}
	}

	pub fn get_list_children(&self) -> Vec<NbtListTag> {
		match self {
			NbtListTag::List(p) => return p.clone(),
			_ => return Vec::new(),
		}
	}

	pub fn get_child(&self, description: &str) -> Option<&NbtTag> {
		match self {
			NbtListTag::TagCompound(p) => {
				for tag in p {
					if tag.get_description() == description {
						return Some(tag);
					}
				}
				return None;
			}
			_ => return None,
		}
	}

	pub fn get_child_mut(&mut self, description: &str) -> Option<&mut NbtTag> {
		match self {
			NbtListTag::TagCompound(p) => {
				for tag in p {
					if tag.get_description() == description {
						return Some(tag);
					}
				}
				return None;
			}
			_ => return None,
		}
	}

	pub fn as_byte(&self) -> u8 {
		match self {
			NbtListTag::Byte(p) => return *p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_short(&self) -> i16 {
		match self {
			NbtListTag::Short(p) => return *p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_int(&self) -> i32 {
		match self {
			NbtListTag::Int(p) => return *p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_long(&self) -> i64 {
		match self {
			NbtListTag::Long(p) => return *p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_float(&self) -> f32 {
		match self {
			NbtListTag::Float(p) => return *p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_double(&self) -> f64 {
		match self {
			NbtListTag::Double(p) => return *p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_byte_array(&self) -> Vec<u8> {
		match self {
			NbtListTag::ByteArray(p) => return p.clone(),
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_string(&self) -> &str {
		match self {
			NbtListTag::String(p) => return p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_list(&self) -> Vec<NbtListTag> {
		match self {
			NbtListTag::List(p) => return p.clone(),
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_list_mut(&mut self) -> &mut Vec<NbtListTag> {
		match self {
			NbtListTag::List(p) => return p,
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_tag_compound(&self) -> Vec<NbtTag> {
		match self {
			NbtListTag::TagCompound(p) => return p.clone(),
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_int_array(&self) -> Vec<i32> {
		match self {
			NbtListTag::IntArray(p) => return p.clone(),
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_long_array(&self) -> Vec<i64> {
		match self {
			NbtListTag::LongArray(p) => return p.clone(),
			_ => panic!("wrong type of Tag!"),
		}
	}
}
