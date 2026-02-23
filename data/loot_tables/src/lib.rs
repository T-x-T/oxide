#![allow(clippy::needless_return)]
use basic_types::*;
use std::collections::HashMap;

pub fn get_loot_tables() -> HashMap<LootTableType, Vec<LootTable>> {
	let mut output: HashMap<LootTableType, Vec<LootTable>> = HashMap::new();

	output.insert(
		LootTableType::Block,
		vec![
			LootTable {
				loot_table_type: LootTableType::Block,
				functions: vec![ItemModifier::ExplosionDecay],
				pools: vec![
					LootTablePool {
						conditions: vec![],
						functions: vec![],
						rolls: NumberProvider::Constant(1.0),
						bonus_rolls: NumberProvider::Constant(0.0),
						entries: vec![LootTablePoolEntry::Composite(LootTablePoolEntryComposite {
							entry_type: LootTablePoolEntryCompositeType::Alternatives,
							children: vec![
								LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
									entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat"),
									conditions: vec![Predicate::BlockStateProperty(PredicateBlockStateProperty {
										block: "minecraft:wheat",
										properties: vec![("age", NumberProvider::Constant(7.0))],
									})],
									functions: vec![],
									weight: None,
									quality: None,
								}),
								LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
									entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat_seeds"),
									conditions: vec![],
									functions: vec![],
									weight: None,
									quality: None,
								}),
							],
							conditions: vec![],
						})],
					},
					LootTablePool {
						conditions: vec![Predicate::BlockStateProperty(PredicateBlockStateProperty {
							block: "minecraft:wheat",
							properties: vec![("age", NumberProvider::Constant(7.0))],
						})],
						functions: vec![],
						rolls: NumberProvider::Constant(1.0),
						bonus_rolls: NumberProvider::Constant(0.0),
						entries: vec![LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
							entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat_seeds"),
							conditions: vec![],
							functions: vec![ItemModifier::ApplyBonus(ApplyBonusData {
								enchantment: "minecraft:fortune",
								formula: "minecraft:binomial_with_bonus_count",
								extra: Some(3),
								probability: Some(0.5714286),
								bonus_multiplier: None,
							})],
							weight: None,
							quality: None,
						})],
					},
				],
				random_sequence: Some("minecraft:blocks/wheat"),
			},
			LootTable {
				loot_table_type: LootTableType::Block,
				functions: vec![],
				pools: vec![LootTablePool {
					conditions: vec![Predicate::SurvivesExplosion],
					functions: vec![],
					rolls: NumberProvider::Constant(1.0),
					bonus_rolls: NumberProvider::Constant(0.0),
					entries: vec![LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
						entry_type: LootTablePoolEntrySingletonType::Item("minecraft:acacia_log"),
						conditions: vec![],
						functions: vec![],
						weight: None,
						quality: None,
					})],
				}],
				random_sequence: Some("minecraft:blocks/acacia_log"),
			},
			LootTable {
				loot_table_type: LootTableType::Block,
				functions: vec![],
				pools: vec![LootTablePool {
					conditions: vec![],
					functions: vec![],
					rolls: NumberProvider::Constant(1.0),
					bonus_rolls: NumberProvider::Constant(0.0),
					entries: vec![LootTablePoolEntry::Composite(LootTablePoolEntryComposite {
						entry_type: LootTablePoolEntryCompositeType::Alternatives,
						children: vec![
							LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
								entry_type: LootTablePoolEntrySingletonType::Item("minecraft:glowstone"),
								conditions: vec![Predicate::MatchTool(PredicateMatchTool {
									predicate: ItemPredicate {
										items: vec![],
										count: None,
										data_component_predicates: vec![DataComponentPredicate::Enchantments(vec![DataComponentPredicateEnchantments {
											enchantments: vec!["minecraft:silk_touch"],
											levels: None,
											min_level: Some(1),
											max_level: None,
										}])],
										count_min: None,
										count_max: None,
									},
								})],
								functions: vec![],
								weight: None,
								quality: None,
							}),
							LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
								entry_type: LootTablePoolEntrySingletonType::Item("minecraft:glowstone_dust"),
								conditions: vec![],
								functions: vec![
									ItemModifier::SetCount(SetCountData {
										count: NumberProvider::Uniform(2.0, 4.0),
										add: false,
									}),
									ItemModifier::ApplyBonus(ApplyBonusData {
										enchantment: "minecraft:fortune",
										formula: "minecraft:uniform_bonus_count",
										extra: None,
										probability: None,
										bonus_multiplier: Some(1.0),
									}),
									ItemModifier::LimitCount(NumberProvider::Uniform(1.0, 4.0)),
									ItemModifier::ExplosionDecay,
								],
								weight: None,
								quality: None,
							}),
						],
						conditions: vec![],
					})],
				}],
				random_sequence: Some("minecraft:blocks/glowstone"),
			},
		],
	);

	return output;
}
