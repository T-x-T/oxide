use super::*;
pub fn get_chests() -> HashMap<&'static str, LootTable> {
	let mut output: HashMap<&'static str, LootTable> = HashMap::new();
	output.insert("minecraft:abandoned_mineshaft", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:enchanted_golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:name_tag"),
					conditions: vec![],
					functions: vec![],
					weight: Some(30),
					quality: None,
					}),
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
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_pickaxe"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(4.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
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
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:redstone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(9.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lapis_lazuli"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(9.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coal"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:glow_berries"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:melon_seeds"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:pumpkin_seeds"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:beetroot_seeds"),
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
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(3.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rail"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:powered_rail"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:detector_rail"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:activator_rail"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:torch"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(16.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/abandoned_mineshaft"),
	});
	output.insert("minecraft:ancient_city", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(10.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:enchanted_golden_apple"),
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
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_otherside"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:compass"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:sculk_catalyst"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:name_tag"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_hoe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.8),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(30.0),), Box::new(NumberProvider::Constant(50.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lead"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_horse_armor"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_13"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_cat"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_leggings"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(30.0),), Box::new(NumberProvider::Constant(50.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:sculk"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:sculk_sensor"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:candle"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:amethyst_shard"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(15.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:experience_bottle"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:glow_berries"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(15.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_leggings"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:echo_shard"),
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
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:disc_fragment_5"),
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
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:strong_regeneration"),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:book"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(15.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:soul_torch"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(15.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coal"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(6.0),), Box::new(NumberProvider::Constant(15.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(7),
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
					weight: Some(75),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ward_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:silence_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/ancient_city"),
	});
	output.insert("minecraft:ancient_city_ice_box", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(10.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:suspicious_stew"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetStewEffect(	vec![
									SetStewEffectDataEffect {
										effect_type: "minecraft:night_vision",
										duration: NumberProvider::Uniform(Box::new(NumberProvider::Constant(7.0),), Box::new(NumberProvider::Constant(10.0),)),
									},
									SetStewEffectDataEffect {
										effect_type: "minecraft:blindness",
										duration: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(7.0),)),
									},
								]),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_carrot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:baked_potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:packed_ice"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:snowball"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(4),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/ancient_city_ice_box"),
	});
	output.insert("minecraft:bastion_bridge", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lodestone"),
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
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(2.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:crossbow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.1),), Box::new(NumberProvider::Constant(0.5),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:spectral_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(10.0),), Box::new(NumberProvider::Constant(28.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gilded_blackstone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(8.0),), Box::new(NumberProvider::Constant(12.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:crying_obsidian"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_block"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(9.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(9.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_sword"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_chestplate"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_helmet"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_leggings"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_boots"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_axe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
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
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(4.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:string"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(17.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_nugget"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_nugget"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(6.0),)),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(11),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:snout_armor_trim_smithing_template"),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(9),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:netherite_upgrade_smithing_template"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/bastion_bridge"),
	});
	output.insert("minecraft:bastion_hoglin_stable", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_shovel"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.15),), Box::new(NumberProvider::Constant(0.8),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_pickaxe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.15),), Box::new(NumberProvider::Constant(0.95),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(12),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:netherite_scrap"),
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
					weight: Some(8),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ancient_debris"),
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
					weight: Some(12),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ancient_debris"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(2.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:saddle"),
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
					weight: Some(12),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_block"),
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
					weight: Some(16),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_carrot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(8.0),), Box::new(NumberProvider::Constant(17.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_apple"),
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
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(4.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_axe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:crying_obsidian"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:glowstone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gilded_blackstone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:soul_sand"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:crimson_nylium"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_nugget"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(17.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:string"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:porkchop"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cooked_porkchop"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:crimson_fungus"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:crimson_roots"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(7.0),)),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(11),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:snout_armor_trim_smithing_template"),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(9),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:netherite_upgrade_smithing_template"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/bastion_hoglin_stable"),
	});
	output.insert("minecraft:bastion_other", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_pickaxe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_shovel"),
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
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:crossbow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.1),), Box::new(NumberProvider::Constant(0.9),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ancient_debris"),
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
					weight: Some(12),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:netherite_scrap"),
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
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:spectral_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(10.0),), Box::new(NumberProvider::Constant(22.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:piglin_banner_pattern"),
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
					weight: Some(9),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_pigstep"),
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
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_carrot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(6.0),), Box::new(NumberProvider::Constant(17.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(12),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_apple"),
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
					weight: Some(9),
					quality: None,
					}),
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
					weight: Some(10),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(2.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_sword"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.1),), Box::new(NumberProvider::Constant(0.9),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_block"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_boots"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_axe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_block"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:crossbow"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_sword"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_chestplate"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_helmet"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_leggings"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_boots"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:crying_obsidian"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(4.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gilded_blackstone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_chain"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:magma_cream"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone_block"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_nugget"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:obsidian"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_nugget"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:string"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(17.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cooked_porkchop"),
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
					weight: Some(11),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:snout_armor_trim_smithing_template"),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(9),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:netherite_upgrade_smithing_template"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/bastion_other"),
	});
	output.insert("minecraft:bastion_treasure", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(3.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:netherite_ingot"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ancient_debris"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:netherite_scrap"),
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
					weight: Some(8),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ancient_debris"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(2.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_sword"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.8),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_chestplate"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.8),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_helmet"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.8),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_leggings"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.8),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_boots"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.8),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_sword"),
					conditions: vec![],
					functions: vec![],
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_chestplate"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_helmet"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_boots"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_leggings"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:enchanted_golden_apple"),
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
					weight: Some(2),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(4.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:spectral_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(12.0),), Box::new(NumberProvider::Constant(25.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_block"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_block"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:crying_obsidian"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:quartz"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(8.0),), Box::new(NumberProvider::Constant(23.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gilded_blackstone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(15.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:magma_cream"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(8.0),)),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(11),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:snout_armor_trim_smithing_template"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:netherite_upgrade_smithing_template"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/bastion_treasure"),
	});
	output.insert("minecraft:buried_treasure", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:heart_of_the_sea"),
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
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(8.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tnt"),
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
					weight: Some(5),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
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
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:prismarine_crystals"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather_chestplate"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_sword"),
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
				rolls: NumberProvider::Constant(2.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cooked_cod"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cooked_salmon"),
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
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![
					ItemModifier {
						function: Function::SetPotion("minecraft:water_breathing"),
					
							conditions: vec![],
					
					},
				],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(2.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/buried_treasure"),
	});
	output.insert("minecraft:desert_pyramid", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(4.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
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
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(25),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:spider_eye"),
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
					weight: Some(25),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(25),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:copper_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
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
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:enchanted_golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(15),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(4.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gunpowder"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(8.0),)),
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
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:sand"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
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
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:dune_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(2.0),
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
		random_sequence: Some("minecraft:chests/desert_pyramid"),
	});
	output.insert("minecraft:end_city_treasure", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(6.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:beetroot_seeds"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:saddle"),
					conditions: vec![],
					functions: vec![],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:copper_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_sword"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_boots"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_chestplate"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_leggings"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_helmet"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_pickaxe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_shovel"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_sword"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_boots"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_chestplate"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_leggings"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_helmet"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_pickaxe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_shovel"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(20.0),), Box::new(NumberProvider::Constant(39.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
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
					weight: Some(14),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:spire_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/end_city_treasure"),
	});
	output.insert("minecraft:igloo_chest", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(8.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:apple"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coal"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_nugget"),
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
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stone_axe"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/igloo_chest"),
	});
	output.insert("minecraft:jungle_temple", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(6.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bamboo"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(16),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:copper_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_horse_armor"),
					conditions: vec![],
					functions: vec![],
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wild_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(2.0),
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
		random_sequence: Some("minecraft:chests/jungle_temple"),
	});
	output.insert("minecraft:jungle_temple_dispenser", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(2.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(30),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/jungle_temple_dispenser"),
	});
	output.insert("minecraft:nether_bridge", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(4.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
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
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_sword"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_chestplate"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:flint_and_steel"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:nether_wart"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:saddle"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: Some(8),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:copper_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:obsidian"),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(14),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rib_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/nether_bridge"),
	});
	output.insert("minecraft:pillager_outpost", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:crossbow"),
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
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(7),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:carrot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:dark_oak_log"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(3.0),)),
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
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:experience_bottle"),
					conditions: vec![],
					functions: vec![],
					weight: Some(7),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:string"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tripwire_hook"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
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
					weight: Some(3),
					quality: None,
					}),
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
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![
					ItemModifier {
						function: Function::SetInstrument("#minecraft:regular_goat_horns"),
					
							conditions: vec![],
					
					},
				],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:goat_horn"),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:sentry_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(2.0),
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
		random_sequence: Some("minecraft:chests/pillager_outpost"),
	});
	output.insert("minecraft:ruined_portal", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(8.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:obsidian"),
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
					weight: Some(40),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:flint"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(40),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_nugget"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(9.0),), Box::new(NumberProvider::Constant(18.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(40),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:flint_and_steel"),
					conditions: vec![],
					functions: vec![],
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_nugget"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(24.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_sword"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_axe"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_hoe"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_shovel"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_pickaxe"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_boots"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_chestplate"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_helmet"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_leggings"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:glistering_melon_slice"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(12.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:light_weighted_pressure_plate"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_carrot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(12.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:clock"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
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
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bell"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:enchanted_golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_block"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lodestone"),
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
					weight: Some(2),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/ruined_portal"),
	});
	output.insert("minecraft:shipwreck_map", LootTable {
		loot_table_type: LootTableType::Chest,
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
					functions: vec![
							ItemModifier {
								function: Function::ExplorationMap(ExplorationMapData {
								destination: None,
								decoration: Some("minecraft:red_x"),
								zoom: Some(1),
								search_radius: None,
								skip_existing_chunks: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetName(SetNameData {
								name: NbtTag::default(),
								entity: None,
								target: None
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
				rolls: NumberProvider::Constant(3.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:compass"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:map"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:clock"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:paper"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:feather"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:book"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
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
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coast_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(2.0),
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
		random_sequence: Some("minecraft:chests/shipwreck_map"),
	});
	output.insert("minecraft:shipwreck_supply", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(10.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:paper"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(12.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(8),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(7),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:moss_block"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(7),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:poisonous_potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(7),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:carrot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(7),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(8.0),), Box::new(NumberProvider::Constant(21.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(7),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:suspicious_stew"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetStewEffect(	vec![
									SetStewEffectDataEffect {
										effect_type: "minecraft:night_vision",
										duration: NumberProvider::Uniform(Box::new(NumberProvider::Constant(7.0),), Box::new(NumberProvider::Constant(10.0),)),
									},
									SetStewEffectDataEffect {
										effect_type: "minecraft:jump_boost",
										duration: NumberProvider::Uniform(Box::new(NumberProvider::Constant(7.0),), Box::new(NumberProvider::Constant(10.0),)),
									},
									SetStewEffectDataEffect {
										effect_type: "minecraft:weakness",
										duration: NumberProvider::Uniform(Box::new(NumberProvider::Constant(6.0),), Box::new(NumberProvider::Constant(8.0),)),
									},
									SetStewEffectDataEffect {
										effect_type: "minecraft:blindness",
										duration: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(7.0),)),
									},
									SetStewEffectDataEffect {
										effect_type: "minecraft:poison",
										duration: NumberProvider::Uniform(Box::new(NumberProvider::Constant(10.0),), Box::new(NumberProvider::Constant(20.0),)),
									},
									SetStewEffectDataEffect {
										effect_type: "minecraft:saturation",
										duration: NumberProvider::Uniform(Box::new(NumberProvider::Constant(7.0),), Box::new(NumberProvider::Constant(10.0),)),
									},
								]),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coal"),
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
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(24.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:pumpkin"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bamboo"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gunpowder"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tnt"),
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
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather_helmet"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather_chestplate"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather_leggings"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather_boots"),
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
					weight: Some(3),
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
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coast_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(2.0),
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
		random_sequence: Some("minecraft:chests/shipwreck_supply"),
	});
	output.insert("minecraft:shipwreck_treasure", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(6.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(90),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(40),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:experience_bottle"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_nugget"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(50),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_nugget"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lapis_lazuli"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(20),
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
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coast_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(2.0),
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
		random_sequence: Some("minecraft:chests/shipwreck_treasure"),
	});
	output.insert("minecraft:simple_dungeon", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:enchanted_golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_otherside"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_13"),
					conditions: vec![],
					functions: vec![],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_cat"),
					conditions: vec![],
					functions: vec![],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:name_tag"),
					conditions: vec![],
					functions: vec![],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:copper_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
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
					weight: Some(10),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bucket"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:redstone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coal"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:melon_seeds"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:pumpkin_seeds"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:beetroot_seeds"),
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
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(3.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gunpowder"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(8.0),)),
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
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/simple_dungeon"),
	});
	output.insert("minecraft:spawn_bonus_chest", LootTable {
		loot_table_type: LootTableType::Chest,
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wooden_axe"),
					conditions: vec![],
					functions: vec![],
					weight: Some(3),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stone_pickaxe"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wooden_pickaxe"),
					conditions: vec![],
					functions: vec![],
					weight: Some(3),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(3.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:apple"),
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
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:salmon"),
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
					weight: Some(3),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(4.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stick"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(12.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:oak_planks"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(12.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:oak_log"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:spruce_log"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:birch_log"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:jungle_log"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:acacia_log"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:dark_oak_log"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:mangrove_log"),
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
					weight: Some(3),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/spawn_bonus_chest"),
	});
	output.insert("minecraft:stronghold_corridor", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ender_pearl"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
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
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:redstone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(9.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:apple"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_pickaxe"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_sword"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_chestplate"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_helmet"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_leggings"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_boots"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:copper_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_otherside"),
					conditions: vec![],
					functions: vec![],
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
					weight: Some(9),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:eye_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/stronghold_corridor"),
	});
	output.insert("minecraft:stronghold_crossing", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
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
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:redstone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(9.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coal"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:apple"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_pickaxe"),
					conditions: vec![],
					functions: vec![],
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
				],
			},
		],
		random_sequence: Some("minecraft:chests/stronghold_crossing"),
	});
	output.insert("minecraft:stronghold_library", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(10.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:book"),
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
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:paper"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:map"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:compass"),
					conditions: vec![],
					functions: vec![],
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
					weight: Some(10),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:eye_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/stronghold_library"),
	});
	output.insert("minecraft:trial_chambers/corridor", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_axe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.4),), Box::new(NumberProvider::Constant(0.9),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:honeycomb"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stone_axe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.15),), Box::new(NumberProvider::Constant(0.8),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stone_pickaxe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.15),), Box::new(NumberProvider::Constant(0.8),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ender_pearl"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bamboo_hanging_sign"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bamboo_planks"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:scaffolding"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:torch"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tuff"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(8.0),), Box::new(NumberProvider::Constant(20.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/trial_chambers/corridor"),
	});
	output.insert("minecraft:trial_chambers/entrance", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:trial_key"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stick"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wooden_axe"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:honeycomb"),
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
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/trial_chambers/entrance"),
	});
	output.insert("minecraft:trial_chambers/intersection", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_block"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald_block"),
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
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_axe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.1),), Box::new(NumberProvider::Constant(0.5),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_pickaxe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.1),), Box::new(NumberProvider::Constant(0.5),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
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
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cake"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:amethyst_shard"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(8.0),), Box::new(NumberProvider::Constant(20.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_block"),
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
					weight: Some(20),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/trial_chambers/intersection"),
	});
	output.insert("minecraft:trial_chambers/intersection_barrel", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_axe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.4),), Box::new(NumberProvider::Constant(0.9),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_pickaxe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.15),), Box::new(NumberProvider::Constant(0.8),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:compass"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.15),), Box::new(NumberProvider::Constant(0.8),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bucket"),
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
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_axe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.15),), Box::new(NumberProvider::Constant(0.8),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_pickaxe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.15),), Box::new(NumberProvider::Constant(0.8),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bamboo_planks"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(15.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:baked_potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(6.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/trial_chambers/intersection_barrel"),
	});
	output.insert("minecraft:trial_chambers/reward", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:chests/trial_chambers/reward_rare"),
					conditions: vec![],
					functions: vec![],
					weight: Some(8),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:chests/trial_chambers/reward_common"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:chests/trial_chambers/reward_common"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::RandomChance(NumberProvider::Constant(0.25),),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:chests/trial_chambers/reward_unique"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/trial_chambers/reward"),
	});
	output.insert("minecraft:trial_chambers/reward_common", LootTable {
		loot_table_type: LootTableType::Chest,
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
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:poison"),
							
									conditions: vec![],
							
							},
					],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
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
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wind_charge"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:honey_bottle"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ominous_bottle"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetOminousBottleAmplifier(NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(1.0),)),),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wind_charge"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(12.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
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
		random_sequence: Some("minecraft:chests/trial_chambers/reward_common"),
	});
	output.insert("minecraft:trial_chambers/reward_ominous", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:chests/trial_chambers/reward_ominous_rare"),
					conditions: vec![],
					functions: vec![],
					weight: Some(8),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:chests/trial_chambers/reward_ominous_common"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:chests/trial_chambers/reward_ominous_common"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![
					Predicate::RandomChance(NumberProvider::Constant(0.75),),
				],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::LootTableId("minecraft:chests/trial_chambers/reward_ominous_unique"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/trial_chambers/reward_ominous"),
	});
	output.insert("minecraft:trial_chambers/reward_ominous_common", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wind_charge"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(8.0),), Box::new(NumberProvider::Constant(12.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(12.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:strong_slowness"),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ominous_bottle"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetOminousBottleAmplifier(NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(4.0),)),),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/trial_chambers/reward_ominous_common"),
	});
	output.insert("minecraft:trial_chambers/reward_ominous_rare", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald_block"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_block"),
					conditions: vec![],
					functions: vec![],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:crossbow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(20.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_axe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(10.0),), Box::new(NumberProvider::Constant(20.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_chestplate"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(10.0),), Box::new(NumberProvider::Constant(20.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:book"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![
									"minecraft:knockback",
									"minecraft:punch",
									"minecraft:smite",
									"minecraft:looting",
									"minecraft:multishot",
								],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:book"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![
									"minecraft:breach",
									"minecraft:density",
								],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:book"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_block"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/trial_chambers/reward_ominous_rare"),
	});
	output.insert("minecraft:trial_chambers/reward_ominous_unique", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:enchanted_golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:flow_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:flow_banner_pattern"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_creator"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:heavy_core"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/trial_chambers/reward_ominous_unique"),
	});
	output.insert("minecraft:trial_chambers/reward_rare", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:shield"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.5),), Box::new(NumberProvider::Constant(1.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(15.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:crossbow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(20.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_axe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(10.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_chestplate"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.0),), Box::new(NumberProvider::Constant(10.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_carrot"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:book"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![
									"minecraft:sharpness",
									"minecraft:bane_of_arthropods",
									"minecraft:efficiency",
									"minecraft:fortune",
									"minecraft:silk_touch",
									"minecraft:feather_falling",
								],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:book"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantRandomly(EnchantRandomlyData {
								options: vec![
									"minecraft:riptide",
									"minecraft:loyalty",
									"minecraft:channeling",
									"minecraft:impaling",
									"minecraft:mending",
								],
								only_compatible: None,
								include_additional_cost_component: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_chestplate"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(15.0),)),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_axe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::EnchantWithLevels(EnchantWithLevelsData {
								levels: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(15.0),)),
								options: 	vec![],
								include_additional_cost_component: None,
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
		random_sequence: Some("minecraft:chests/trial_chambers/reward_rare"),
	});
	output.insert("minecraft:trial_chambers/reward_unique", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bolt_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:guster_banner_pattern"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_precipice"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:trident"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/trial_chambers/reward_unique"),
	});
	output.insert("minecraft:trial_chambers/supply", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(5.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(14.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(8.0),)),
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
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tipped_arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(8.0),)),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:baked_potato"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:glow_berries"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:acacia_planks"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:moss_block"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone_meal"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tuff"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(5.0),), Box::new(NumberProvider::Constant(10.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:torch"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(2.0),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(2.0),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stone_pickaxe"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetDamage(SetDamageData {
								damage: NumberProvider::Uniform(Box::new(NumberProvider::Constant(0.15),), Box::new(NumberProvider::Constant(0.8),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:milk_bucket"),
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
		random_sequence: Some("minecraft:chests/trial_chambers/supply"),
	});
	output.insert("minecraft:underwater_ruin_big", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(8.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coal"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_nugget"),
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
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather_chestplate"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_helmet"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:fishing_rod"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:map"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::ExplorationMap(ExplorationMapData {
								destination: None,
								decoration: Some("minecraft:red_x"),
								zoom: Some(1),
								search_radius: None,
								skip_existing_chunks: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetName(SetNameData {
								name: NbtTag::default(),
								entity: None,
								target: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/underwater_ruin_big"),
	});
	output.insert("minecraft:underwater_ruin_small", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(8.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coal"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stone_axe"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather_chestplate"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_helmet"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:fishing_rod"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:map"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::ExplorationMap(ExplorationMapData {
								destination: None,
								decoration: Some("minecraft:red_x"),
								zoom: Some(1),
								search_radius: None,
								skip_existing_chunks: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetName(SetNameData {
								name: NbtTag::default(),
								entity: None,
								target: None
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/underwater_ruin_small"),
	});
	output.insert("minecraft:village/village_armorer", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_helmet"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_armorer"),
	});
	output.insert("minecraft:village/village_butcher", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:porkchop"),
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
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat"),
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
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:beef"),
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
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:mutton"),
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
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coal"),
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
					weight: Some(3),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_butcher"),
	});
	output.insert("minecraft:village/village_cartographer", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:map"),
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
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:paper"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:compass"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stick"),
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
					weight: Some(5),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bundle"),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_cartographer"),
	});
	output.insert("minecraft:village/village_desert_house", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(8.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:clay_ball"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:green_dye"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cactus"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:book"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:dead_bush"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bundle"),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_desert_house"),
	});
	output.insert("minecraft:village/village_fisher", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cod"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:salmon"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:water_bucket"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:barrel"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat_seeds"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coal"),
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
					weight: Some(2),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_fisher"),
	});
	output.insert("minecraft:village/village_fletcher", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:feather"),
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
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:egg"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:flint"),
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
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stick"),
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
					weight: Some(6),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_fletcher"),
	});
	output.insert("minecraft:village/village_mason", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:clay_ball"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:flower_pot"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stone"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stone_bricks"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:yellow_dye"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:smooth_stone"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_mason"),
	});
	output.insert("minecraft:village/village_plains_house", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(8.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_nugget"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:dandelion"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:poppy"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:apple"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:book"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:feather"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:oak_sapling"),
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
					weight: Some(5),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bundle"),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_plains_house"),
	});
	output.insert("minecraft:village/village_savanna_house", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(8.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_nugget"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:short_grass"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:tall_grass"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat_seeds"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:acacia_sapling"),
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
					weight: Some(10),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:torch"),
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
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bucket"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bundle"),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_savanna_house"),
	});
	output.insert("minecraft:village/village_shepherd", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:white_wool"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:black_wool"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gray_wool"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:brown_wool"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:light_gray_wool"),
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
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:shears"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(6.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(6),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_shepherd"),
	});
	output.insert("minecraft:village/village_snowy_house", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(8.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:blue_ice"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:snow_block"),
					conditions: vec![],
					functions: vec![],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:beetroot_seeds"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:beetroot_soup"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:furnace"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:snowball"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coal"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bundle"),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_snowy_house"),
	});
	output.insert("minecraft:village/village_taiga_house", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(8.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_nugget"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:fern"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:large_fern"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:sweet_berries"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:pumpkin_seeds"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:spruce_sapling"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:spruce_sign"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:spruce_log"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bundle"),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_taiga_house"),
	});
	output.insert("minecraft:village/village_tannery", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather_chestplate"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather_boots"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather_helmet"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:leather_leggings"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bundle"),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_tannery"),
	});
	output.insert("minecraft:village/village_temple", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(8.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:redstone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(7),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(7),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lapis_lazuli"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
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
		random_sequence: Some("minecraft:chests/village/village_temple"),
	});
	output.insert("minecraft:village/village_toolsmith", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(8.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_pickaxe"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coal"),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:stick"),
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
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_shovel"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_toolsmith"),
	});
	output.insert("minecraft:village/village_weaponsmith", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(8.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
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
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
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
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:apple"),
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
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_pickaxe"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_sword"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_chestplate"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_helmet"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_leggings"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_boots"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:obsidian"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:oak_sapling"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(3.0),), Box::new(NumberProvider::Constant(7.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:saddle"),
					conditions: vec![],
					functions: vec![],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:copper_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_horse_armor"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_horse_armor"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bundle"),
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
					entry_type: LootTablePoolEntrySingletonType::Empty,
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/village/village_weaponsmith"),
	});
	output.insert("minecraft:woodland_mansion", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lead"),
					conditions: vec![],
					functions: vec![],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:enchanted_golden_apple"),
					conditions: vec![],
					functions: vec![],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_13"),
					conditions: vec![],
					functions: vec![],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_cat"),
					conditions: vec![],
					functions: vec![],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:name_tag"),
					conditions: vec![],
					functions: vec![],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:chainmail_chestplate"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_hoe"),
					conditions: vec![],
					functions: vec![],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_chestplate"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
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
					weight: Some(10),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gold_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wheat"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(20),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bucket"),
					conditions: vec![],
					functions: vec![],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:redstone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:coal"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(15),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:melon_seeds"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:pumpkin_seeds"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:beetroot_seeds"),
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:resin_clump"),
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
					weight: Some(50),
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(3.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bone"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:gunpowder"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:rotten_flesh"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(8.0),)),
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
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(10),
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
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:vex_armor_trim_smithing_template"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:chests/woodland_mansion"),
	});
	return output;
}