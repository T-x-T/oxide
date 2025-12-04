#![allow(
  clippy::needless_return,
  clippy::format_collect,
  unused
)]

use std::collections::HashMap;
use std::io::prelude::*;

mod blockentity;
mod entities;
mod items;
mod blocks;

fn main() {
  blockentity::generate();
  entities::generate();
  items::generate();
  blocks::generate();
}

fn convert_to_upper_camel_case(input: &str) -> String {
  let mut found_underscore = false;
  return input
    .replace("minecraft:", "")
    .chars()
    .enumerate()
    .map(|i| {
      if i.1 == '_' {
        found_underscore = true;
        return i.1;
      }
      if i.0 == 0 || found_underscore {
        found_underscore = false;
        return i.1.to_ascii_uppercase();
      } else {
        return i.1;
      }
    }
  )
  .filter(|i| *i != '_')
  .collect();
}
