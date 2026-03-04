use super::*;
pub fn get_harvest() -> HashMap<&'static str, LootTable> {
	let mut output: HashMap<&'static str, LootTable> = HashMap::new();
	output.insert("minecraft:beehive", LootTable {
		loot_table_type: LootTableType::BlockInteract,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:honeycomb"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(3.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:harvest/beehive"),
	});
	output.insert("minecraft:cave_vine", LootTable {
		loot_table_type: LootTableType::BlockInteract,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:glow_berries"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:harvest/cave_vine"),
	});
	output.insert("minecraft:sweet_berry_bush", LootTable {
		loot_table_type: LootTableType::BlockInteract,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:sweet_berries"),
					conditions: vec![
							Predicate::BlockStateProperty(PredicateBlockStateProperty{
								block: "minecraft:sweet_berry_bush",
								properties: vec![(Property::SweetBerryBushAge(SweetBerryBushAge::Num3), Property::SweetBerryBushAge(SweetBerryBushAge::Num3))]
							}),
					],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:sweet_berries"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:harvest/sweet_berry_bush"),
	});
	return output;
}