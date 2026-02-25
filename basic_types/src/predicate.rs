use std::collections::HashMap;

use blocks::Property;

use crate::data_component::DataComponent;
use crate::data_component_predicate::DataComponentPredicate;
use crate::enchantment::LevelBasedValue;
use crate::item_modifier::EntityLootContext;
use crate::nbt::NbtTag;
use crate::*;

//https://minecraft.wiki/w/Predicate
#[derive(Debug, Clone, PartialEq)]
pub enum Predicate {
	AllOf(Vec<Predicate>),
	AnyOf(Vec<Predicate>),
	BlockStateProperty(PredicateBlockStateProperty),
	DamageSourceProperties(Box<PredicateDamageSourceProperties>),
	EnchantmentActiveCheck(bool),
	EntityProperties(Box<PredicateEntityProperties>),
	EntityScores(PredicateEntityScores),
	Inverted(Box<Predicate>),
	KilledByPlayer,
	LocationCheck(Box<PredicateLocationCheck>),
	MatchTool(ItemPredicate),
	RandomChance(NumberProvider),
	RandomChanceWithEnchantedBonus(PredicateRandomChanceWithEnchantedBonus),
	Reference(&'static str),
	SurvivesExplosion,
	TableBonus(PredicateTableBonus),
	TimeCheck(PredicateTimeCheck),
	ValueCheck(PredicateValueCheck),
	WeatherCheck(PredicateWeatherCheck),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemPredicate {
	pub items: Vec<&'static str>,
	pub count: Option<i32>,
	pub count_min: Option<i32>,
	pub count_max: Option<i32>,
	pub data_component_predicates: Vec<DataComponentPredicate>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityPredicate {
	pub entity_type: Vec<&'static str>,
	pub components: Vec<DataComponent>,
	pub distance_absolute: Option<f32>,
	pub distance_absolute_min: Option<f32>,
	pub distance_absolute_max: Option<f32>,
	pub distance_horizontal: Option<f32>,
	pub distance_horizontal_min: Option<f32>,
	pub distance_horizontal_max: Option<f32>,
	pub distance_x: Option<f32>,
	pub distance_x_min: Option<f32>,
	pub distance_x_max: Option<f32>,
	pub distance_y: Option<f32>,
	pub distance_y_min: Option<f32>,
	pub distance_y_max: Option<f32>,
	pub distance_z: Option<f32>,
	pub distance_z_min: Option<f32>,
	pub distance_z_max: Option<f32>,
	pub effects: HashMap<&'static str, EffectPredicate>,
	pub equipment: HashMap<&'static str, ItemPredicate>,
	pub flags_is_baby: Option<bool>,
	pub flags_is_on_fire: Option<bool>,
	pub flags_is_sneaking: Option<bool>,
	pub flags_is_sprinting: Option<bool>,
	pub flags_is_swimming: Option<bool>,
	pub flags_is_on_ground: Option<bool>,
	pub flags_is_flying: Option<bool>,
	pub flags_is_fall_flying: Option<bool>,
	pub location: LocationPredicate,
	pub nbt: NbtTag,
	pub passenger: Vec<EntityPredicate>,
	pub slots: HashMap<&'static str, ItemPredicate>,
	pub stepping_on: Option<LocationPredicate>,
	pub movement_affected_by: Option<LocationPredicate>,
	pub team: Option<&'static str>,
	pub targeted_entity: Option<Box<EntityPredicate>>,
	pub vehicle: Option<Box<EntityPredicate>>,
	pub movement_x: Option<f64>,
	pub movement_x_min: Option<f64>,
	pub movement_x_max: Option<f64>,
	pub movement_y: Option<f64>,
	pub movement_y_min: Option<f64>,
	pub movement_y_max: Option<f64>,
	pub movement_z: Option<f64>,
	pub movement_z_min: Option<f64>,
	pub movement_z_max: Option<f64>,
	pub movement_speed: Option<f64>,
	pub movement_speed_min: Option<f64>,
	pub movement_speed_max: Option<f64>,
	pub movement_horizontal_speed: Option<f64>,
	pub movement_horizontal_speed_min: Option<f64>,
	pub movement_horizontal_speed_max: Option<f64>,
	pub movement_vertical_speed: Option<f64>,
	pub movement_vertical_speed_min: Option<f64>,
	pub movement_vertical_speed_max: Option<f64>,
	pub movement_fall_distance: Option<f64>,
	pub movement_fall_distance_min: Option<f64>,
	pub movement_fall_distance_max: Option<f64>,
	pub periodic_tick: Option<i32>,
	pub predicates: Vec<DataComponentPredicate>,
	pub type_specific: EntityPredicateTypeSpecific,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityPredicateTypeSpecific {
	FishingHook(bool),
	Lightning(EntityPredicateTypeSpecificLightning),
	Player(EntityPredicateTypeSpecificPlayer),
	Raider(EntityPredicateTypeSpecificRaider),
	Sheep(bool),
	Slime(EntityPredicateTypeSpecificSlime),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityPredicateTypeSpecificLightning {
	pub blocks_to_set_on_fire: Option<i32>,
	pub blocks_to_set_on_fire_min: Option<i32>,
	pub blocks_to_set_on_fire_max: Option<i32>,
	pub entity_struck: Option<Box<EntityPredicate>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityPredicateTypeSpecificPlayer {
	pub looking_at: Option<Box<EntityPredicate>>,
	pub advancements: HashMap<&'static str, AdvancementPredicate>,
	pub gamemode: Vec<Gamemode>,
	pub level: Option<i32>,
	pub level_min: Option<i32>,
	pub level_max: Option<i32>,
	pub recipes: HashMap<&'static str, bool>,
	pub stats: Vec<StatisticPredicate>,
	pub input_forward: Option<bool>,
	pub input_backward: Option<bool>,
	pub input_left: Option<bool>,
	pub input_right: Option<bool>,
	pub input_jump: Option<bool>,
	pub input_sneak: Option<bool>,
	pub input_sprint: Option<bool>,
	pub food_level: Option<i32>,
	pub food_level_min: Option<i32>,
	pub food_level_max: Option<i32>,
	pub food_saturation: Option<f32>,
	pub food_saturation_min: Option<f32>,
	pub food_saturation_max: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityPredicateTypeSpecificRaider {
	pub is_captain: Option<bool>,
	pub has_raid: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityPredicateTypeSpecificSlime {
	pub size: Option<i32>,
	pub size_min: Option<i32>,
	pub size_max: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdvancementPredicate {
	pub is_granted: Option<bool>,
	pub criterion: HashMap<&'static str, Option<bool>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatisticPredicate {
	pub statistic_type: &'static str,
	pub statistic: &'static str,
	pub value: Option<i32>,
	pub value_min: Option<i32>,
	pub value_max: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EffectPredicate {
	pub amplifier: Option<i32>,
	pub amplifier_min: Option<i32>,
	pub amplifier_max: Option<i32>,
	pub duration: Option<i32>,
	pub duration_min: Option<i32>,
	pub duration_max: Option<i32>,
	pub ambient: Option<bool>,
	pub visible: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocationPredicate {
	pub biomes: Vec<&'static str>,
	pub blocks: Vec<&'static str>,
	pub block_nbt: NbtTag,
	pub block_state: Vec<(Property, Property)>,
	pub block_components: Vec<DataComponent>,
	pub block_predicates: Vec<DataComponentPredicate>,
	pub dimension: Option<&'static str>,
	pub fluids: Vec<&'static str>,
	pub fluid_state: Vec<(Property, Property)>,
	pub fluid_components: Vec<DataComponent>,
	pub fluid_predicates: Vec<DataComponentPredicate>,
	pub light: Option<i32>,
	pub light_max: Option<i32>,
	pub light_min: Option<i32>,
	pub position_x: Option<f64>,
	pub position_x_max: Option<f64>,
	pub position_x_min: Option<f64>,
	pub position_y: Option<f64>,
	pub position_y_max: Option<f64>,
	pub position_y_min: Option<f64>,
	pub position_z: Option<f64>,
	pub position_z_max: Option<f64>,
	pub position_z_min: Option<f64>,
	pub smokey: Option<bool>,
	pub can_see_sky: Option<bool>,
	pub structures: Vec<&'static str>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct PredicateBlockStateProperty {
	pub block: &'static str,
	pub properties: Vec<(Property, Property)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateDamageSourceProperties {
	pub direct_entity: Option<EntityPredicate>,
	pub source_entity: Option<EntityPredicate>,
	pub is_direct: Option<bool>,
	pub tags: Vec<PredicateDamageSourcePropertiesTag>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateDamageSourcePropertiesTag {
	pub id: &'static str,
	pub expected: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateEntityProperties {
	pub entity: EntityLootContext,
	pub predicate: Option<EntityPredicate>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateEntityScores {
	pub entity: EntityLootContext,
	pub scores: HashMap<&'static str, PredicateEntityScoresScore>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateEntityScoresScore {
	pub score: Option<i32>,
	pub score_min: Option<i32>,
	pub score_max: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateLocationCheck {
	pub offset_x: Option<i32>,
	pub offset_y: Option<i32>,
	pub offset_z: Option<i32>,
	pub predicate: Option<LocationPredicate>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateRandomChanceWithEnchantedBonus {
	pub unenchanted_chance: Option<f32>,
	pub enchanted_chance: Option<LevelBasedValue>,
	pub enchantment: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateTableBonus {
	pub enchantment: &'static str,
	pub chances: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateTimeCheck {
	pub clock: Option<&'static str>,
	pub value_min: Option<NumberProvider>,
	pub value_max: Option<NumberProvider>,
	pub value: Option<i32>,
	pub period: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateValueCheck {
	pub value: NumberProvider,
	pub range_min: Option<NumberProvider>,
	pub range_max: Option<NumberProvider>,
	pub range: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredicateWeatherCheck {
	pub raining: Option<bool>,
	pub thundering: Option<bool>,
}
