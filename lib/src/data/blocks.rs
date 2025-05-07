use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Type {
  Block,
  Slab,
  Sapling,
}

#[derive(Debug, Clone)]
pub struct Block {
  pub block_type: Type,
  pub states: Vec<State>,
  pub properties: Vec<Property>,
}

#[derive(Debug, Clone)]
pub struct State {
  pub id: i32,
  pub default: bool,
  pub properties: Vec<Property>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Property {
  X,
  Y,
  Z,
  Stage(u8),
}

pub fn get_block_from_name(name: String) -> Block {
  return get_blocks().get(&name).cloned().unwrap();
}

pub fn get_blocks() -> HashMap<String, Block> {
  return vec![
    ("minecraft:stone", Block { block_type: Type::Block, properties: vec![], states: vec![State { id: 1, default: true, properties: vec![] }] }),
    ("minecraft:oak_log", Block { block_type: Type::Block, properties: vec![Property::X, Property::Y, Property::Z], states: vec![State { id: 136, default: false, properties: vec![Property::X] }, State { id: 137, default: true, properties: vec![Property::Y] }, State { id: 138, default: false, properties: vec![Property::Z] }] }),
    ("minecraft:oak_sapling", Block { block_type: Type::Sapling, properties: vec![Property::Stage(1), Property::Stage(2)], states: vec![State { id: 29, default: true, properties: vec![Property::Stage(0)] }, State { id: 30, default: false, properties: vec![Property::Stage(1)] }] }),
  ].into_iter().map(|x| (x.0.to_string(), x.1)).collect();
}
