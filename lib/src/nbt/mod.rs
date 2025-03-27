
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