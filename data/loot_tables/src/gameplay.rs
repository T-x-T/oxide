use super::*;
pub fn get_gameplay() -> HashMap<&'static str, LootTable> {
	let mut output: HashMap<&'static str, LootTable> = HashMap::new();
	output.insert("minecraft:armadillo_shed", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:armadillo_scute"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/armadillo_shed"),
	});
	output.insert("minecraft:cat_morning_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rabbit_hide"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rabbit_foot"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:chicken"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:feather"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:string"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:phantom_membrane"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/cat_morning_gift"),
	});
	output.insert("minecraft:chicken_lay", LootTable {
		loot_table_type: LootTableType::Gift,
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
							entry_type: LootTablePoolEntrySingletonType::Item("minecraft:egg"),
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
							entry_type: LootTablePoolEntrySingletonType::Item("minecraft:brown_egg"),
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
							entry_type: LootTablePoolEntrySingletonType::Item("minecraft:blue_egg"),
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
		random_sequence: Some("minecraft:gameplay/chicken_lay"),
	});
	output.insert("minecraft:fishing", LootTable {
		loot_table_type: LootTableType::Fishing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:gameplay/fishing/junk"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: Some(-2),
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:gameplay/fishing/treasure"),
					conditions: vec![
							Predicate::EntityProperties(Box::new(PredicateEntityProperties {
							entity: EntityLootContext::This,
							predicate: None
							})),
					],
					functions: vec![],
					weight: Some(5),
					quality: Some(2),
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:gameplay/fishing/fish"),
					conditions: vec![],
					functions: vec![],
					weight: Some(85),
					quality: Some(-1),
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/fishing"),
	});
	output.insert("minecraft:fishing/fish", LootTable {
		loot_table_type: LootTableType::Fishing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cod"),
					conditions: vec![],
					functions: vec![],
					weight: Some(60),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:salmon"),
					conditions: vec![],
					functions: vec![],
					weight: Some(25),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tropical_fish"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:pufferfish"),
					conditions: vec![],
					functions: vec![],
					weight: Some(13),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/fishing/fish"),
	});
	output.insert("minecraft:fishing/junk", LootTable {
		loot_table_type: LootTableType::Fishing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lily_pad"),
					conditions: vec![],
					functions: vec![],
					weight: Some(17),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather_boots"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(0.9),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetPotion("minecraft:water"),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:string"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:fishing_rod"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(0.9),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bowl"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stick"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ink_sac"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(10.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tripwire_hook"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bamboo"),
					conditions: vec![
							Predicate::LocationCheck(Box::new(PredicateLocationCheck {
							offset_x: None,
							offset_y: None,
							offset_z: None,
							predicate: Some(LocationPredicate {
								biomes: vec!["minecraft:jungle", "minecraft:sparse_jungle", "minecraft:bamboo_jungle", ],
								blocks: vec![],
								block_nbt: NbtTag::default(),
								block_state: vec![],
								block_components: vec![],
								block_predicates: vec![],
								dimension: None,
								fluids: vec![],
								fluid_state: vec![],
								fluid_components: vec![],
								fluid_predicates: vec![],
								light: None,
								light_max: None,
								light_min: None,
								position_x: None,
								position_x_max: None,
								position_x_min: None,
								position_y: None,
								position_y_max: None,
								position_y_min: None,
								position_z: None,
								position_z_max: None,
								position_z_min: None,
								smokey: None,
								can_see_sky: None,
								structures: vec![],
							})})),
					],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/fishing/junk"),
	});
	output.insert("minecraft:fishing/treasure", LootTable {
		loot_table_type: LootTableType::Fishing,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:name_tag"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:saddle"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(0.25),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Constant(30.0),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:fishing_rod"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(0.25),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Constant(30.0),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:book"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Constant(30.0),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:nautilus_shell"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/fishing/treasure"),
	});
	output.insert("minecraft:hero_of_the_village/armorer_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:chainmail_helmet"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:chainmail_chestplate"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:chainmail_leggings"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:chainmail_boots"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/armorer_gift"),
	});
	output.insert("minecraft:hero_of_the_village/baby_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:poppy"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/baby_gift"),
	});
	output.insert("minecraft:hero_of_the_village/butcher_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cooked_rabbit"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cooked_chicken"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cooked_porkchop"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cooked_beef"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cooked_mutton"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/butcher_gift"),
	});
	output.insert("minecraft:hero_of_the_village/cartographer_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:map"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:paper"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/cartographer_gift"),
	});
	output.insert("minecraft:hero_of_the_village/cleric_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:redstone"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lapis_lazuli"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/cleric_gift"),
	});
	output.insert("minecraft:hero_of_the_village/farmer_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:pumpkin_pie"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cookie"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/farmer_gift"),
	});
	output.insert("minecraft:hero_of_the_village/fisherman_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cod"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:salmon"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/fisherman_gift"),
	});
	output.insert("minecraft:hero_of_the_village/fletcher_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![],
					weight: Some(26),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:swiftness"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:slowness"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:strength"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:healing"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:harming"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:leaping"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:regeneration"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:fire_resistance"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:water_breathing"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:invisibility"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:night_vision"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:weakness"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:poison"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/fletcher_gift"),
	});
	output.insert("minecraft:hero_of_the_village/leatherworker_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/leatherworker_gift"),
	});
	output.insert("minecraft:hero_of_the_village/librarian_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:book"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/librarian_gift"),
	});
	output.insert("minecraft:hero_of_the_village/mason_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:clay"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/mason_gift"),
	});
	output.insert("minecraft:hero_of_the_village/shepherd_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:white_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:orange_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:magenta_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:light_blue_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:yellow_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lime_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:pink_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gray_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:light_gray_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cyan_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:purple_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:blue_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:brown_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:green_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:red_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
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
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/shepherd_gift"),
	});
	output.insert("minecraft:hero_of_the_village/toolsmith_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stone_pickaxe"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stone_axe"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stone_hoe"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stone_shovel"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/toolsmith_gift"),
	});
	output.insert("minecraft:hero_of_the_village/unemployed_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat_seeds"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/unemployed_gift"),
	});
	output.insert("minecraft:hero_of_the_village/weaponsmith_gift", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stone_axe"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_axe"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_axe"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/hero_of_the_village/weaponsmith_gift"),
	});
	output.insert("minecraft:panda_sneeze", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:slime_ball"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(699),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/panda_sneeze"),
	});
	output.insert("minecraft:piglin_bartering", LootTable {
		loot_table_type: LootTableType::Barter,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:book"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_boots"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(8),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetPotion("minecraft:fire_resistance"),
							
									conditions: vec![],
							
							},
					],
					weight: Some(8),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:splash_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetPotion("minecraft:fire_resistance"),
							
									conditions: vec![],
							
							},
					],
					weight: Some(8),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetPotion("minecraft:water"),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_nugget"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(10.0),), Box::new(NumberProvider::Constant(36.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ender_pearl"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:dried_ghast"),
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
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:string"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(9.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:quartz"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(12.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:obsidian"),
					conditions: vec![],
					functions: vec![],
					weight: Some(40),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:crying_obsidian"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(40),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:fire_charge"),
					conditions: vec![],
					functions: vec![],
					weight: Some(40),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(40),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:soul_sand"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(40),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:nether_brick"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(40),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:spectral_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(6.0),), Box::new(NumberProvider::Constant(12.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(40),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gravel"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(8.0),), Box::new(NumberProvider::Constant(16.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(40),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:blackstone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(8.0),), Box::new(NumberProvider::Constant(16.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(40),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/piglin_bartering"),
	});
	output.insert("minecraft:sniffer_digging", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:torchflower_seeds"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:pitcher_pod"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/sniffer_digging"),
	});
	output.insert("minecraft:turtle_grow", LootTable {
		loot_table_type: LootTableType::Gift,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:turtle_scute"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:gameplay/turtle_grow"),
	});
	return output;
}