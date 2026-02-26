use super::*;
pub fn get_entities() -> HashMap<&'static str, LootTable> {
	let mut output: HashMap<&'static str, LootTable> = HashMap::new();
	output.insert("minecraft:allay", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/allay"),
	});
	output.insert("minecraft:armadillo", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/armadillo"),
	});
	output.insert("minecraft:armor_stand", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/armor_stand"),
	});
	output.insert("minecraft:axolotl", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/axolotl"),
	});
	output.insert("minecraft:bat", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/bat"),
	});
	output.insert("minecraft:bee", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/bee"),
	});
	output.insert("minecraft:blaze", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:blaze_rod"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/blaze"),
	});
	output.insert("minecraft:bogged", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: Some(1),
								enchantment:"minecraft:looting"
							}),
							ItemModifier::SetPotion("minecraft:poison"),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/bogged"),
	});
	output.insert("minecraft:breeze", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:breeze_rod"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(2.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/breeze"),
	});
	output.insert("minecraft:camel", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/camel"),
	});
	output.insert("minecraft:cat", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:string"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/cat"),
	});
	output.insert("minecraft:cave_spider", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:string"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:spider_eye"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(-1.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/cave_spider"),
	});
	output.insert("minecraft:chicken", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:feather"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:chicken"),
					conditions: vec![],
					functions: vec![
							ItemModifier::FurnaceSmelt,
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/chicken"),
	});
	output.insert("minecraft:cod", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cod"),
					conditions: vec![],
					functions: vec![
							ItemModifier::FurnaceSmelt,
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::RandomChance(NumberProvider::Constant(0.05),),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone_meal"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/cod"),
	});
	output.insert("minecraft:copper_golem", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:copper_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/copper_golem"),
	});
	output.insert("minecraft:cow", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:beef"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							ItemModifier::FurnaceSmelt,
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/cow"),
	});
	output.insert("minecraft:creaking", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/creaking"),
	});
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gunpowder"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::EntityProperties(Box::new(PredicateEntityProperties {
					entity: EntityLootContext::Attacker,
					predicate: None
					})),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Tag(LootTablePoolEntryTag {
						name: "minecraft:creeper_drop_music_discs",
						expand: true,
						conditions: vec![],
						functions: vec![],
						weight: None,
						quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/creeper"),
	});
	output.insert("minecraft:dolphin", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cod"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
							ItemModifier::FurnaceSmelt,
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/dolphin"),
	});
	output.insert("minecraft:donkey", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/donkey"),
	});
	output.insert("minecraft:drowned", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
					Predicate::RandomChanceWithEnchantedBonus(PredicateRandomChanceWithEnchantedBonus {
					unenchanted_chance: None,
					enchanted_chance: None,
					enchantment: None
					}),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:copper_ingot"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/drowned"),
	});
	output.insert("minecraft:elder_guardian", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:prismarine_shard"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cod"),
					conditions: vec![],
					functions: vec![
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
							ItemModifier::FurnaceSmelt,
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:prismarine_crystals"),
					conditions: vec![],
					functions: vec![
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wet_sponge"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
					Predicate::RandomChanceWithEnchantedBonus(PredicateRandomChanceWithEnchantedBonus {
					unenchanted_chance: None,
					enchanted_chance: None,
					enchantment: None
					}),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:gameplay/fishing/fish"),
					conditions: vec![],
					functions: vec![
							ItemModifier::FurnaceSmelt,
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tide_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/elder_guardian"),
	});
	output.insert("minecraft:ender_dragon", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/ender_dragon"),
	});
	output.insert("minecraft:enderman", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ender_pearl"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/enderman"),
	});
	output.insert("minecraft:endermite", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/endermite"),
	});
	output.insert("minecraft:evoker", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:totem_of_undying"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/evoker"),
	});
	output.insert("minecraft:fox", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/fox"),
	});
	output.insert("minecraft:frog", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/frog"),
	});
	output.insert("minecraft:ghast", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ghast_tear"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gunpowder"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::DamageSourceProperties(Box::new(PredicateDamageSourceProperties {
					direct_entity: None,
					source_entity: None,
					is_direct: None,
					tags: vec![]
					})),
					Predicate::KilledByPlayer,
				],
				functions: vec![
					ItemModifier::SetCount(SetCountData {
						count: NumberProvider::Constant(1.0),
						add: Some(false)
					}),
				],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_tears"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/ghast"),
	});
	output.insert("minecraft:giant", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/giant"),
	});
	output.insert("minecraft:glow_squid", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:glow_ink_sac"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/glow_squid"),
	});
	output.insert("minecraft:goat", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/goat"),
	});
	output.insert("minecraft:guardian", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:prismarine_shard"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cod"),
					conditions: vec![],
					functions: vec![
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
							ItemModifier::FurnaceSmelt,
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:prismarine_crystals"),
					conditions: vec![],
					functions: vec![
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
					Predicate::RandomChanceWithEnchantedBonus(PredicateRandomChanceWithEnchantedBonus {
					unenchanted_chance: None,
					enchanted_chance: None,
					enchantment: None
					}),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:gameplay/fishing/fish"),
					conditions: vec![],
					functions: vec![
							ItemModifier::FurnaceSmelt,
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/guardian"),
	});
	output.insert("minecraft:happy_ghast", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/happy_ghast"),
	});
	output.insert("minecraft:hoglin", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:porkchop"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							ItemModifier::FurnaceSmelt,
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/hoglin"),
	});
	output.insert("minecraft:horse", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/horse"),
	});
	output.insert("minecraft:husk", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
					Predicate::RandomChanceWithEnchantedBonus(PredicateRandomChanceWithEnchantedBonus {
					unenchanted_chance: None,
					enchanted_chance: None,
					enchantment: None
					}),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:carrot"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier::FurnaceSmelt,
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/husk"),
	});
	output.insert("minecraft:illusioner", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/illusioner"),
	});
	output.insert("minecraft:iron_golem", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:poppy"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/iron_golem"),
	});
	output.insert("minecraft:llama", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/llama"),
	});
	output.insert("minecraft:magma_cube", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:magma_cream"),
					conditions: vec![
							Predicate::Inverted(Box::new(Predicate::DamageSourceProperties(Box::new(PredicateDamageSourceProperties {
							direct_entity: None,
							source_entity: None,
							is_direct: None,
							tags: vec![]
							})))),
							Predicate::EntityProperties(Box::new(PredicateEntityProperties {
							entity: EntityLootContext::This,
							predicate: None
							})),
					],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(-2.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:pearlescent_froglight"),
					conditions: vec![
							Predicate::DamageSourceProperties(Box::new(PredicateDamageSourceProperties {
							direct_entity: None,
							source_entity: None,
							is_direct: None,
							tags: vec![]
							})),
					],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:verdant_froglight"),
					conditions: vec![
							Predicate::DamageSourceProperties(Box::new(PredicateDamageSourceProperties {
							direct_entity: None,
							source_entity: None,
							is_direct: None,
							tags: vec![]
							})),
					],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ochre_froglight"),
					conditions: vec![
							Predicate::DamageSourceProperties(Box::new(PredicateDamageSourceProperties {
							direct_entity: None,
							source_entity: None,
							is_direct: None,
							tags: vec![]
							})),
					],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/magma_cube"),
	});
	output.insert("minecraft:mannequin", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/mannequin"),
	});
	output.insert("minecraft:mooshroom", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:beef"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							ItemModifier::FurnaceSmelt,
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/mooshroom"),
	});
	output.insert("minecraft:mule", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/mule"),
	});
	output.insert("minecraft:ocelot", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/ocelot"),
	});
	output.insert("minecraft:panda", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bamboo"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/panda"),
	});
	output.insert("minecraft:parrot", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:feather"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/parrot"),
	});
	output.insert("minecraft:phantom", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:phantom_membrane"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/phantom"),
	});
	output.insert("minecraft:pig", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:porkchop"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							ItemModifier::FurnaceSmelt,
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/pig"),
	});
	output.insert("minecraft:piglin", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/piglin"),
	});
	output.insert("minecraft:piglin_brute", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/piglin_brute"),
	});
	output.insert("minecraft:pillager", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![
					Predicate::EntityProperties(Box::new(PredicateEntityProperties {
					entity: EntityLootContext::This,
					predicate: None
					})),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ominous_bottle"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::SetOminousBottleAmplifier(NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(4.0),)),),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/pillager"),
	});
	output.insert("minecraft:player", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/player"),
	});
	output.insert("minecraft:polar_bear", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cod"),
					conditions: vec![],
					functions: vec![
							ItemModifier::FurnaceSmelt,
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:salmon"),
					conditions: vec![],
					functions: vec![
							ItemModifier::FurnaceSmelt,
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/polar_bear"),
	});
	output.insert("minecraft:pufferfish", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:pufferfish"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::RandomChance(NumberProvider::Constant(0.05),),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone_meal"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/pufferfish"),
	});
	output.insert("minecraft:rabbit", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rabbit_hide"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rabbit"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::FurnaceSmelt,
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
					Predicate::RandomChanceWithEnchantedBonus(PredicateRandomChanceWithEnchantedBonus {
					unenchanted_chance: None,
					enchanted_chance: None,
					enchantment: None
					}),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rabbit_foot"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/rabbit"),
	});
	output.insert("minecraft:ravager", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:saddle"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/ravager"),
	});
	output.insert("minecraft:salmon", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:salmon"),
					conditions: vec![],
					functions: vec![
							ItemModifier::FurnaceSmelt,
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::RandomChance(NumberProvider::Constant(0.05),),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone_meal"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/salmon"),
	});
	output.insert("minecraft:sheep", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:mutton"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::FurnaceSmelt,
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
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
					LootTablePoolEntry::Composite(LootTablePoolEntryComposite {
						entry_type: LootTablePoolEntryCompositeType::Alternatives,
						children: vec![
							LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/white"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/orange"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/magenta"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/light_blue"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/yellow"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/lime"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/pink"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/gray"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/light_gray"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/cyan"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/purple"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/blue"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/brown"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/green"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/red"),
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
							entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:entities/sheep/black"),
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
		random_sequence: Some("minecraft:entities/sheep"),
	});
	output.insert("minecraft:sheep/black", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:black_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/black"),
	});
	output.insert("minecraft:sheep/blue", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:blue_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/blue"),
	});
	output.insert("minecraft:sheep/brown", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:brown_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/brown"),
	});
	output.insert("minecraft:sheep/cyan", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cyan_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/cyan"),
	});
	output.insert("minecraft:sheep/gray", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gray_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/gray"),
	});
	output.insert("minecraft:sheep/green", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:green_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/green"),
	});
	output.insert("minecraft:sheep/light_blue", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:light_blue_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/light_blue"),
	});
	output.insert("minecraft:sheep/light_gray", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:light_gray_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/light_gray"),
	});
	output.insert("minecraft:sheep/lime", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lime_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/lime"),
	});
	output.insert("minecraft:sheep/magenta", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:magenta_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/magenta"),
	});
	output.insert("minecraft:sheep/orange", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:orange_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/orange"),
	});
	output.insert("minecraft:sheep/pink", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:pink_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/pink"),
	});
	output.insert("minecraft:sheep/purple", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:purple_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/purple"),
	});
	output.insert("minecraft:sheep/red", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:red_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/red"),
	});
	output.insert("minecraft:sheep/white", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:white_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/white"),
	});
	output.insert("minecraft:sheep/yellow", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:yellow_wool"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/sheep/yellow"),
	});
	output.insert("minecraft:shulker", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![
					Predicate::RandomChanceWithEnchantedBonus(PredicateRandomChanceWithEnchantedBonus {
					unenchanted_chance: None,
					enchanted_chance: None,
					enchantment: None
					}),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:shulker_shell"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/shulker"),
	});
	output.insert("minecraft:silverfish", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/silverfish"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/skeleton"),
	});
	output.insert("minecraft:skeleton_horse", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/skeleton_horse"),
	});
	output.insert("minecraft:slime", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![
					Predicate::EntityProperties(Box::new(PredicateEntityProperties {
					entity: EntityLootContext::This,
					predicate: None
					})),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:slime_ball"),
					conditions: vec![
							Predicate::Inverted(Box::new(Predicate::DamageSourceProperties(Box::new(PredicateDamageSourceProperties {
							direct_entity: None,
							source_entity: None,
							is_direct: None,
							tags: vec![]
							})))),
					],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:slime_ball"),
					conditions: vec![
							Predicate::DamageSourceProperties(Box::new(PredicateDamageSourceProperties {
							direct_entity: None,
							source_entity: None,
							is_direct: None,
							tags: vec![]
							})),
					],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/slime"),
	});
	output.insert("minecraft:sniffer", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/sniffer"),
	});
	output.insert("minecraft:snow_golem", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:snowball"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(15.0),)),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/snow_golem"),
	});
	output.insert("minecraft:spider", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:string"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:spider_eye"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(-1.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/spider"),
	});
	output.insert("minecraft:squid", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ink_sac"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/squid"),
	});
	output.insert("minecraft:stray", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: Some(1),
								enchantment:"minecraft:looting"
							}),
							ItemModifier::SetPotion("minecraft:slowness"),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/stray"),
	});
	output.insert("minecraft:strider", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:string"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/strider"),
	});
	output.insert("minecraft:tadpole", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/tadpole"),
	});
	output.insert("minecraft:trader_llama", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/trader_llama"),
	});
	output.insert("minecraft:tropical_fish", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tropical_fish"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::RandomChance(NumberProvider::Constant(0.05),),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone_meal"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/tropical_fish"),
	});
	output.insert("minecraft:turtle", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:seagrass"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: Some(3),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::DamageSourceProperties(Box::new(PredicateDamageSourceProperties {
					direct_entity: None,
					source_entity: None,
					is_direct: None,
					tags: vec![]
					})),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bowl"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/turtle"),
	});
	output.insert("minecraft:vex", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/vex"),
	});
	output.insert("minecraft:villager", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/villager"),
	});
	output.insert("minecraft:vindicator", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/vindicator"),
	});
	output.insert("minecraft:wandering_trader", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/wandering_trader"),
	});
	output.insert("minecraft:warden", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:sculk_catalyst"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/warden"),
	});
	output.insert("minecraft:witch", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:glowstone_dust"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:sugar"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:spider_eye"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:glass_bottle"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gunpowder"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stick"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: Some(2),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:redstone"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/witch"),
	});
	output.insert("minecraft:wither", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/wither"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coal"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(-1.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
					Predicate::RandomChanceWithEnchantedBonus(PredicateRandomChanceWithEnchantedBonus {
					unenchanted_chance: None,
					enchanted_chance: None,
					enchantment: None
					}),
				],
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
		random_sequence: Some("minecraft:entities/wither_skeleton"),
	});
	output.insert("minecraft:wolf", LootTable {
		loot_table_type: LootTableType::Entity,
		functions: vec![],
		pools: vec![
		],
		random_sequence: Some("minecraft:entities/wolf"),
	});
	output.insert("minecraft:zoglin", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/zoglin"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
					Predicate::RandomChanceWithEnchantedBonus(PredicateRandomChanceWithEnchantedBonus {
					unenchanted_chance: None,
					enchanted_chance: None,
					enchantment: None
					}),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:carrot"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier::FurnaceSmelt,
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
					Predicate::EntityProperties(Box::new(PredicateEntityProperties {
					entity: EntityLootContext::This,
					predicate: None
					})),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_lava_chicken"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/zombie"),
	});
	output.insert("minecraft:zombie_horse", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/zombie_horse"),
	});
	output.insert("minecraft:zombie_villager", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
					Predicate::RandomChanceWithEnchantedBonus(PredicateRandomChanceWithEnchantedBonus {
					unenchanted_chance: None,
					enchanted_chance: None,
					enchantment: None
					}),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:carrot"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier::FurnaceSmelt,
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/zombie_villager"),
	});
	output.insert("minecraft:zombified_piglin", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_nugget"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							ItemModifier::EnchantedCountIncrease(EnchantCountIncreaseData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
								limit: None,
								enchantment:"minecraft:looting"
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::KilledByPlayer,
					Predicate::RandomChanceWithEnchantedBonus(PredicateRandomChanceWithEnchantedBonus {
					unenchanted_chance: None,
					enchanted_chance: None,
					enchantment: None
					}),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:entities/zombified_piglin"),
	});
	return output;
}