use super::*;
pub fn get_carve() -> HashMap<&'static str, LootTable> {
	let mut output: HashMap<&'static str, LootTable> = HashMap::new();
	output.insert("minecraft:pumpkin", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:pumpkin_seeds"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(4.0),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:carve/pumpkin"),
	});
	return output;
}