#[derive(Debug, Clone, PartialEq)]
pub enum LevelBasedValue {
	Constant(f32),
	Exponent(Box<LevelBasedValue>),
	Linear(LevelBasedValueLinear),
	LevelsSquared(f32),
	Clamped(Box<LevelBasedValueClamped>),
	Fraction(Box<LevelBasedValueFraction>),
	Lookup(Box<LevelBasedValueLookup>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LevelBasedValueExponent {
	pub base: LevelBasedValue,
	pub power: LevelBasedValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LevelBasedValueLinear {
	pub base: f32,
	pub per_level_about_first: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LevelBasedValueClamped {
	pub value: LevelBasedValue,
	pub min: f32,
	pub max: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LevelBasedValueFraction {
	pub numerator: LevelBasedValue,
	pub denominator: LevelBasedValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LevelBasedValueLookup {
	pub values: Vec<f32>,
	pub fallback: LevelBasedValue,
}
