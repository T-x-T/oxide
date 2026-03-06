#![allow(clippy::needless_return)]

pub mod data_component;
pub mod data_component_predicate;
pub mod enchantment;
pub mod item_modifier;
pub mod loot_table;
pub mod nbt;
pub mod predicate;
pub mod recipe;


#[derive(Debug, Clone, PartialEq)]
pub enum NumberProvider {
	Constant(f32),
	Uniform(Box<NumberProvider>, Box<NumberProvider>),
	Binomial(Box<NumberProvider>, Box<NumberProvider>),
	Score(&'static str, &'static str, &'static str, &'static str, f32),
	Storage(&'static str, &'static str),
	EnchantmentLevel(&'static str),
	Sum(Vec<NumberProvider>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Gamemode {
	Survival = 0,
	Creative = 1,
	Adventure = 2,
	Spectator = 3,
}

impl TryFrom<u8> for Gamemode {
	type Error = String;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		return match value {
			0 => Ok(Gamemode::Survival),
			1 => Ok(Gamemode::Creative),
			2 => Ok(Gamemode::Adventure),
			3 => Ok(Gamemode::Spectator),
			x => Err(format!("I dont know what a gamemode of {x} is supposed to be")),
		};
	}
}
