use crate::*;

//https://minecraft.wiki/w/Predicate
#[derive(Debug, Clone, PartialEq)]
pub enum Predicate {
	AllOf,
	AnyOf,
	BlockStateProperty(PredicateBlockStateProperty),
	DamageSourceProperty,
	EnchantmentActiveCheck,
	EntityProperties,
	EntityScores,
	Inverted,
	KilledByPlayer,
	LocationToCheck,
	MatchTool(PredicateMatchTool),
	RandomChance,
	RandomChanceWithEnchantedBonus,
	Reference,
	SurvivesExplosion,
	TableBonus,
	TimeCheck,
	ValueCheck,
	WeatherCheck,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateBlockStateProperty {
	pub block: &'static str,
	pub properties: Vec<(&'static str, NumberProvider)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateMatchTool {
	pub predicate: ItemPredicate,
}
