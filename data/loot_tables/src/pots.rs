use super::*;
pub fn get_pots() -> HashMap<&'static str, LootTable> {
	let mut output: HashMap<&'static str, LootTable> = HashMap::new();
	output.insert("minecraft:trial_chambers/corridor", LootTable {
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
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
					],
					weight: Some(125),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
					],
					weight: Some(100),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:iron_ingot"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
					],
					weight: Some(100),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:trial_key"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
					],
					weight: Some(10),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:music_disc_creator_music_box"),
					conditions: vec![],
					functions: vec![],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:emerald_block"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
					],
					weight: Some(5),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:diamond_block"),
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
		random_sequence: Some("minecraft:pots/trial_chambers/corridor"),
	});
	return output;
}