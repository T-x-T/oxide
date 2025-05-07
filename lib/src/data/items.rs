use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ItemRarity {
  Common
}

#[derive(Debug, Clone)]
pub struct Item {
  pub max_stack_size: u8,
  pub rarity: ItemRarity,
  pub repair_cost: u8,
  pub id: i32,
}

pub fn get_item_name_by_id(id: i32) -> String {
  return get_items().into_iter().find(|x| x.1.id == id).unwrap_or(get_items().into_iter().next().unwrap()).0;
}

pub fn get_items() -> HashMap<String, Item> {
  return vec![
    ("minecraft:stone", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 1 }),
    ("minecraft:oak_log", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 134 }),
    ("minecraft:oak_sapling", Item { max_stack_size: 64, rarity: ItemRarity::Common, repair_cost: 0, id: 49 }),
  ].into_iter().map(|x| (x.0.to_string(), x.1)).collect();
}
