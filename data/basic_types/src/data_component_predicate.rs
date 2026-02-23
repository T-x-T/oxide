//https://minecraft.wiki/w/Data_component_predicate
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataComponentPredicate {
	AttributeModifiers,
	BundleContents,
	Container,
	CustomData,
	Damage,
	Enchantments(Vec<DataComponentPredicateEnchantments>),
	FireworkExplosion,
	Fireworks,
	JukeboxPlayable,
	PotionContents,
	StoredEnchantments,
	Trim,
	WritableBookContent,
	WrittenBookContent,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataComponentPredicateEnchantments {
	pub enchantments: Vec<&'static str>,
	pub levels: Option<i32>,
	pub min_level: Option<i32>,
	pub max_level: Option<i32>,
}
