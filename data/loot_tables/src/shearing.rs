use super::*;
pub fn get_shearing() -> HashMap<&'static str, LootTable> {
	let mut output: HashMap<&'static str, LootTable> = HashMap::new();
	output.insert("minecraft:bogged", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(2.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:brown_mushroom"),
					conditions: vec![],
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
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:red_mushroom"),
					conditions: vec![],
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
		],
		random_sequence: Some("minecraft:shearing/bogged"),
	});
	output.insert("minecraft:mooshroom", LootTable {
		loot_table_type: LootTableType::Shearing,
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/mooshroom/red"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/mooshroom/brown"),
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
		random_sequence: Some("minecraft:shearing/mooshroom"),
	});
	output.insert("minecraft:mooshroom/brown", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(5.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:brown_mushroom"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/mooshroom/brown"),
	});
	output.insert("minecraft:mooshroom/red", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(5.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:red_mushroom"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/mooshroom/red"),
	});
	output.insert("minecraft:sheep", LootTable {
		loot_table_type: LootTableType::Shearing,
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/white"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/orange"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/magenta"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/light_blue"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/yellow"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/lime"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/pink"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/gray"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/light_gray"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/cyan"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/purple"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/blue"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/brown"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/green"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/red"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:shearing/sheep/black"),
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
		random_sequence: Some("minecraft:shearing/sheep"),
	});
	output.insert("minecraft:sheep/black", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:black_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/black"),
	});
	output.insert("minecraft:sheep/blue", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:blue_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/blue"),
	});
	output.insert("minecraft:sheep/brown", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:brown_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/brown"),
	});
	output.insert("minecraft:sheep/cyan", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cyan_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/cyan"),
	});
	output.insert("minecraft:sheep/gray", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gray_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/gray"),
	});
	output.insert("minecraft:sheep/green", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:green_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/green"),
	});
	output.insert("minecraft:sheep/light_blue", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:light_blue_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/light_blue"),
	});
	output.insert("minecraft:sheep/light_gray", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:light_gray_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/light_gray"),
	});
	output.insert("minecraft:sheep/lime", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lime_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/lime"),
	});
	output.insert("minecraft:sheep/magenta", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:magenta_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/magenta"),
	});
	output.insert("minecraft:sheep/orange", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:orange_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/orange"),
	});
	output.insert("minecraft:sheep/pink", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:pink_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/pink"),
	});
	output.insert("minecraft:sheep/purple", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:purple_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/purple"),
	});
	output.insert("minecraft:sheep/red", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:red_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/red"),
	});
	output.insert("minecraft:sheep/white", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:white_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/white"),
	});
	output.insert("minecraft:sheep/yellow", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:yellow_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/sheep/yellow"),
	});
	output.insert("minecraft:snow_golem", LootTable {
		loot_table_type: LootTableType::Shearing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:carved_pumpkin"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:shearing/snow_golem"),
	});
	return output;
}