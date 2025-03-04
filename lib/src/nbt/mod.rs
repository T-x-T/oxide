
#[allow(unused)]
#[derive(Debug, Clone)]
pub enum NbtTag<'a> {
	Byte(Option<&'a str>, u8),
	Short(Option<&'a str>, i16),
	Int(Option<&'a str>, i32),
	Long(Option<&'a str>, i64),
	Float(Option<&'a str>, f32),
	Double(Option<&'a str>, f64),
	ByteArray(Option<&'a str>, &'a [u8]),
	String(Option<&'a str>, &'a str),
	List(Option<&'a str>, Vec<NbtTag<'a>>),
	TagCompound(Option<&'a str>, Vec<NbtTag<'a>>),
	IntArray(Option<&'a str>, &'a [i32]),
	LongArray(Option<&'a str>, &'a [i64]),
}

impl<'a> Default for NbtTag<'a> {
	fn default() -> Self {
		return NbtTag::TagCompound(None, Vec::new());
	}
}