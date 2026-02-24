use crate::data_component::{AttributeModifiersDataOperation, AttributeModifiersDataSlot, FireworkExplosionShape};
use crate::nbt::NbtTag;
use crate::predicate::ItemPredicate;

//https://minecraft.wiki/w/Data_component_predicate
#[derive(Debug, Clone, PartialEq)]
pub enum DataComponentPredicate {
	AttributeModifiers(AttributeModifiersPredicate),
	BundleContents(PredicateOfItemStack),
	Container(PredicateOfItemStack),
	CustomData(NbtTag),
	Damage(DamagePredicate),
	Enchantments(Vec<EnchantmentsPredicate>),
	FireworkExplosion(PredicateOfExplosionEffect),
	Fireworks(FireworksPredicate),
	JukeboxPlayable(Vec<&'static str>),
	PotionContents(Vec<&'static str>),
	StoredEnchantments(Vec<EnchantmentsPredicate>),
	Trim(TrimPredicate),
	WritableBookContent(WritableBookContentPredicate),
	WrittenBookContent(WrittenBookContentPredicate),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateOfItemStack {
	pub contains: Vec<ItemPredicate>,
	pub size: Option<i32>,
	pub size_min: Option<i32>,
	pub size_max: Option<i32>,
	pub count: Vec<PredicateOfItemStackCount>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateOfItemStackCount {
	pub test: ItemPredicate,
	pub count: Option<i32>,
	pub count_min: Option<i32>,
	pub count_max: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateOfExplosionEffect {
	pub shape: Option<FireworkExplosionShape>,
	pub has_trail: Option<bool>,
	pub has_twinkle: Option<bool>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct AttributeModifiersPredicate {
	pub contains: Vec<PredicateOfAttributeModifier>,
	pub size: Option<i32>,
	pub size_min: Option<i32>,
	pub size_max: Option<i32>,
	pub count: Vec<PredicateOfAttributeModifierCount>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateOfAttributeModifier {
	pub attribute: Vec<&'static str>,
	pub id: Option<&'static str>,
	pub amount: Option<f64>,
	pub amount_max: Option<f64>,
	pub amount_min: Option<f64>,
	pub operation: AttributeModifiersDataOperation,
	pub slot: AttributeModifiersDataSlot,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateOfAttributeModifierCount {
	pub test: PredicateOfAttributeModifier,
	pub count: Option<i32>,
	pub count_min: Option<i32>,
	pub count_max: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DamagePredicate {
	pub damage: Option<i32>,
	pub damage_min: Option<i32>,
	pub damage_max: Option<i32>,
	pub durability: Option<i32>,
	pub durability_min: Option<i32>,
	pub durability_max: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnchantmentsPredicate {
	pub enchantments: Vec<&'static str>,
	pub levels: Option<i32>,
	pub min_level: Option<i32>,
	pub max_level: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FireworksPredicate {
	pub contains: Vec<PredicateOfExplosionEffect>,
	pub size: Option<i32>,
	pub size_min: Option<i32>,
	pub size_max: Option<i32>,
	pub count: Vec<FireworksPredicateCount>,
	pub flight_duration: Option<i32>,
	pub flight_duration_min: Option<i32>,
	pub flight_duration_max: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FireworksPredicateCount {
	pub test: PredicateOfExplosionEffect,
	pub count: Option<i32>,
	pub count_min: Option<i32>,
	pub count_max: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrimPredicate {
	pub material: Vec<&'static str>,
	pub pattern: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WritableBookContentPredicate {
	pub contains: Vec<&'static str>,
	pub size: Option<i32>,
	pub size_min: Option<i32>,
	pub size_max: Option<i32>,
	pub count: Vec<WritableBookContentPredicateCount>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WrittenBookContentPredicate {
	pub contains: Vec<&'static str>,
	pub size: Option<i32>,
	pub size_min: Option<i32>,
	pub size_max: Option<i32>,
	pub count: Vec<WritableBookContentPredicateCount>,
	pub author: Option<&'static str>,
	pub title: Option<&'static str>,
	pub generation: Option<i32>,
	pub generation_min: Option<i32>,
	pub generation_max: Option<i32>,
	pub resolved: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WritableBookContentPredicateCount {
	pub test: &'static str,
	pub count: Option<i32>,
	pub count_min: Option<i32>,
	pub count_max: Option<i32>,
}
