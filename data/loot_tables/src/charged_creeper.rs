use super::*;
pub fn get_charged_creeper() -> HashMap<&'static str, LootTable> {
	let mut output: HashMap<&'static str, LootTable> = HashMap::new();
	output.insert("minecraft:creeper", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:creeper_head"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:charged_creeper/creeper"),
	});
	output.insert("minecraft:piglin", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:piglin_head"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:charged_creeper/piglin"),
	});
	output.insert("minecraft:root", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Composite(LootTablePoolEntryComposite {
						entry_type: LootTablePoolEntryCompositeType::Alternatives,
						children: vec![
							LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:charged_creeper/piglin"),
							conditions: vec![
									Predicate::EntityProperties(Box::new(PredicateEntityProperties {
									entity: EntityLootContext::This,
									predicate: None
									})),
							],
							functions: vec![],
							weight: None,
							quality: None,
							}),
							LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:charged_creeper/creeper"),
							conditions: vec![
									Predicate::EntityProperties(Box::new(PredicateEntityProperties {
									entity: EntityLootContext::This,
									predicate: None
									})),
							],
							functions: vec![],
							weight: None,
							quality: None,
							}),
							LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:charged_creeper/skeleton"),
							conditions: vec![
									Predicate::EntityProperties(Box::new(PredicateEntityProperties {
									entity: EntityLootContext::This,
									predicate: None
									})),
							],
							functions: vec![],
							weight: None,
							quality: None,
							}),
							LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:charged_creeper/wither_skeleton"),
							conditions: vec![
									Predicate::EntityProperties(Box::new(PredicateEntityProperties {
									entity: EntityLootContext::This,
									predicate: None
									})),
							],
							functions: vec![],
							weight: None,
							quality: None,
							}),
							LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:charged_creeper/zombie"),
							conditions: vec![
									Predicate::EntityProperties(Box::new(PredicateEntityProperties {
									entity: EntityLootContext::This,
									predicate: None
									})),
							],
							functions: vec![],
							weight: None,
							quality: None,
							}),
					],
						conditions: vec![],
					 }),
				],
			},
		],
		random_sequence: Some("minecraft:charged_creeper/root"),
	});
	output.insert("minecraft:skeleton", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:skeleton_skull"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:charged_creeper/skeleton"),
	});
	output.insert("minecraft:wither_skeleton", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wither_skeleton_skull"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:charged_creeper/wither_skeleton"),
	});
	output.insert("minecraft:zombie", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:zombie_head"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:charged_creeper/zombie"),
	});
	return output;
}