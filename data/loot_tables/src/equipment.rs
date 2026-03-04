use super::*;
pub fn get_equipment() -> HashMap<&'static str, LootTable> {
	let mut output: HashMap<&'static str, LootTable> = HashMap::new();
	output.insert("minecraft:trial_chamber", LootTable {
		loot_table_type: LootTableType::Equipment,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableCustom(LootTable {
							loot_table_type: LootTableType::Custom,
							functions: vec![],
							pools: vec![
								LootTablePool {
									conditions: vec![
										Predicate::RandomChance(NumberProvider::Constant(0.5),),
									],
									functions: vec![],
									rolls: NumberProvider::Constant(1.0),
									bonus_rolls: NumberProvider::Constant(0.0),
									entries: vec![
										LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
										entry_type: LootTablePoolEntrySingletonType::Item("minecraft:chainmail_helmet"),
										conditions: vec![],
										functions: vec![
												ItemModifier {
													function: Function::SetComponents(vec![]),
												
														conditions: vec![],
												
												},
												ItemModifier {
													function: Function::SetEnchantments(SetEnchantmentsData {
													enchantments: HashMap::new(),
													add: None
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
									conditions: vec![
										Predicate::RandomChance(NumberProvider::Constant(0.5),),
									],
									functions: vec![],
									rolls: NumberProvider::Constant(1.0),
									bonus_rolls: NumberProvider::Constant(0.0),
									entries: vec![
										LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
										entry_type: LootTablePoolEntrySingletonType::Item("minecraft:chainmail_chestplate"),
										conditions: vec![],
										functions: vec![
												ItemModifier {
													function: Function::SetComponents(vec![]),
												
														conditions: vec![],
												
												},
												ItemModifier {
													function: Function::SetEnchantments(SetEnchantmentsData {
													enchantments: HashMap::new(),
													add: None
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
							random_sequence: None,
						}),
					conditions: vec![],
					functions: vec![],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableCustom(LootTable {
							loot_table_type: LootTableType::Custom,
							functions: vec![],
							pools: vec![
								LootTablePool {
									conditions: vec![
										Predicate::RandomChance(NumberProvider::Constant(0.5),),
									],
									functions: vec![],
									rolls: NumberProvider::Constant(1.0),
									bonus_rolls: NumberProvider::Constant(0.0),
									entries: vec![
										LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
										entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_helmet"),
										conditions: vec![],
										functions: vec![
												ItemModifier {
													function: Function::SetComponents(vec![]),
												
														conditions: vec![],
												
												},
												ItemModifier {
													function: Function::SetEnchantments(SetEnchantmentsData {
													enchantments: HashMap::new(),
													add: None
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
									conditions: vec![
										Predicate::RandomChance(NumberProvider::Constant(0.5),),
									],
									functions: vec![],
									rolls: NumberProvider::Constant(1.0),
									bonus_rolls: NumberProvider::Constant(0.0),
									entries: vec![
										LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
										entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_chestplate"),
										conditions: vec![],
										functions: vec![
												ItemModifier {
													function: Function::SetComponents(vec![]),
												
														conditions: vec![],
												
												},
												ItemModifier {
													function: Function::SetEnchantments(SetEnchantmentsData {
													enchantments: HashMap::new(),
													add: None
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
							random_sequence: None,
						}),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableCustom(LootTable {
							loot_table_type: LootTableType::Custom,
							functions: vec![],
							pools: vec![
								LootTablePool {
									conditions: vec![
										Predicate::RandomChance(NumberProvider::Constant(0.5),),
									],
									functions: vec![],
									rolls: NumberProvider::Constant(1.0),
									bonus_rolls: NumberProvider::Constant(0.0),
									entries: vec![
										LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
										entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_helmet"),
										conditions: vec![],
										functions: vec![
												ItemModifier {
													function: Function::SetComponents(vec![]),
												
														conditions: vec![],
												
												},
												ItemModifier {
													function: Function::SetEnchantments(SetEnchantmentsData {
													enchantments: HashMap::new(),
													add: None
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
									conditions: vec![
										Predicate::RandomChance(NumberProvider::Constant(0.5),),
									],
									functions: vec![],
									rolls: NumberProvider::Constant(1.0),
									bonus_rolls: NumberProvider::Constant(0.0),
									entries: vec![
										LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
										entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_chestplate"),
										conditions: vec![],
										functions: vec![
												ItemModifier {
													function: Function::SetComponents(vec![]),
												
														conditions: vec![],
												
												},
												ItemModifier {
													function: Function::SetEnchantments(SetEnchantmentsData {
													enchantments: HashMap::new(),
													add: None
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
							random_sequence: None,
						}),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:equipment/trial_chamber"),
	});
	output.insert("minecraft:trial_chamber_melee", LootTable {
		loot_table_type: LootTableType::Equipment,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:equipment/trial_chamber"),
					conditions: vec![],
					functions: vec![],
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_sword"),
					conditions: vec![],
					functions: vec![],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_sword"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetEnchantments(SetEnchantmentsData {
								enchantments: HashMap::new(),
								add: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_sword"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetEnchantments(SetEnchantmentsData {
								enchantments: HashMap::new(),
								add: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_sword"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:equipment/trial_chamber_melee"),
	});
	output.insert("minecraft:trial_chamber_ranged", LootTable {
		loot_table_type: LootTableType::Equipment,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:equipment/trial_chamber"),
					conditions: vec![],
					functions: vec![],
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bow"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetEnchantments(SetEnchantmentsData {
								enchantments: HashMap::new(),
								add: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetEnchantments(SetEnchantmentsData {
								enchantments: HashMap::new(),
								add: None
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
		random_sequence: Some("minecraft:equipment/trial_chamber_ranged"),
	});
	return output;
}