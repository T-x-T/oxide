#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NbtTag {
	Byte(Option<String>, u8),
	Short(Option<String>, i16),
	Int(Option<String>, i32),
	Long(Option<String>, i64),
	Float(Option<String>, f32),
	Double(Option<String>, f64),
	ByteArray(Option<String>, Vec<u8>),
	String(Option<String>, String),
	List(Option<String>, Vec<NbtTag>),
	TagCompound(Option<String>, Vec<NbtTag>),
	IntArray(Option<String>, Vec<i32>),
	LongArray(Option<String>, Vec<i64>),
}

impl Default for NbtTag {
	fn default() -> Self {
		return NbtTag::TagCompound(None, Vec::new());
	}
}

impl NbtTag {
	pub fn get_children(&self) -> Vec<NbtTag> {
		match self {
			NbtTag::TagCompound(_, p) => return p.clone(),
			NbtTag::List(_, p) => return p.clone(),
			_ => return Vec::new(),
		}
	}

	pub fn get_child(&self, description: &str) -> Option<NbtTag> {
		match self {
			NbtTag::TagCompound(_, p) => {
				for tag in p {
					if tag.get_description().unwrap_or_default().as_str() == description {
						return Some(tag.clone());
					}
				}
				return None;
			},
			_ => return None,
		}
	}

	pub fn get_description(&self) -> Option<String> {
		return match self {
	    NbtTag::Byte(d, _) => d.clone(),
	    NbtTag::Short(d, _) => d.clone(),
	    NbtTag::Int(d, _) => d.clone(),
	    NbtTag::Long(d, _) => d.clone(),
	    NbtTag::Float(d, _) => d.clone(),
	    NbtTag::Double(d, _) => d.clone(),
	    NbtTag::ByteArray(d, _) => d.clone(),
	    NbtTag::String(d, _) => d.clone(),
	    NbtTag::List(d, _) => d.clone(),
	    NbtTag::TagCompound(d, _) => d.clone(),
	    NbtTag::IntArray(d, _) => d.clone(),
	    NbtTag::LongArray(d, _) => d.clone(),
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

	pub fn as_string(&self) -> String {
		match self {
			NbtTag::String(_, p) => return p.clone(),
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_list(&self) -> Vec<NbtTag> {
		match self {
			NbtTag::List(_, p) => return p.clone(),
			_ => panic!("wrong type of Tag!"),
		}
	}

	pub fn as_tag_compound(&self) -> Vec<NbtTag> {
		match self {
			NbtTag::TagCompound(_, p) => return p.clone(),
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
