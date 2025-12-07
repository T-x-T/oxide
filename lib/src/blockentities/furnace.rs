use super::*;

#[derive(Debug, Clone)]
pub struct Furnace {
  pub position: BlockPosition, //global position, NOT within the chunk
  pub components: Vec<SlotComponent>, //At least I think so?
  pub needs_ticking: bool,
  pub inventory: Vec<Item>, //input, fuel, output
  pub lit_time_remaining: i16,
  pub cooking_time_spent: i16,
  pub cooking_total_time: i16,
  pub lit_total_time: i16,
}

impl CommonBlockEntity for Furnace {
  fn tick(&mut self, players: &[Player], game: Arc<Game>) {
    if self.needs_ticking {
      if self.inventory[0].count == 0 {
        self.needs_ticking = false;

        self.lit_time_remaining = 0;
        self.cooking_time_spent = 0;
        self.cooking_total_time = 0;
        self.lit_total_time = 0;

        players.iter()
          .filter(|x| x.opened_inventory_at.is_some_and(|y| y == self.position))
          .for_each(|x| {
            game.send_packet(&x.peer_socket_address, crate::packets::clientbound::play::SetContainerProperty::PACKET_ID, crate::packets::clientbound::play::SetContainerProperty {
              window_id: 1,
              property: 0, //fuel left
              value: self.lit_time_remaining, //ticks of fuel left
            }.try_into().unwrap());

            game.send_packet(&x.peer_socket_address, crate::packets::clientbound::play::SetContainerProperty::PACKET_ID, crate::packets::clientbound::play::SetContainerProperty {
              window_id: 1,
              property: 2, //progress
              value: self.cooking_time_spent, //progress from 0-200
            }.try_into().unwrap());
          });

        return;
      }

      if (self.inventory[1].id != "minecraft:coal" && self.lit_time_remaining == 0) || self.inventory[0].id != "minecraft:raw_iron" {
        self.needs_ticking = false;
        return;
      }

      let mut can_cook = true;
      if self.lit_time_remaining == 0 {
        if self.inventory[1].count > 0 {
          self.inventory[1] = Item { count: self.inventory[1].count - 1, ..self.inventory[1].clone() };
          self.lit_time_remaining = 1600;
        } else {
          self.cooking_time_spent = 0;
          can_cook = false;
        }
      }

      if can_cook {
        if self.cooking_time_spent == 0 {
          self.cooking_time_spent = 1;
        } else if self.cooking_time_spent == 200 {
          self.cooking_time_spent = 0;

          if self.inventory[2].id == "minecraft:iron_ingot" {
            self.inventory[2] = Item { count: self.inventory[2].count + 1, ..self.inventory[2].clone() };
          } else {
            self.inventory[2] = Item { count: 1, id: "minecraft:iron_ingot".to_string(), components: Vec::new() };
          }
          self.inventory[0] = Item { count: self.inventory[0].count - 1, ..self.inventory[0].clone() };
        } else {
          self.cooking_time_spent += 1;
        }

        self.lit_time_remaining -= 1;
      }

      players.iter()
        .filter(|x| x.opened_inventory_at.is_some_and(|y| y == self.position))
        .for_each(|x| {
          game.send_packet(&x.peer_socket_address, crate::packets::clientbound::play::SetContainerContent::PACKET_ID, crate::packets::clientbound::play::SetContainerContent {
            window_id: 1,
            state_id: 1,
            slot_data: self.inventory.iter().cloned().map(Into::into).collect(),
            carried_item: None,
          }.try_into().unwrap());

          game.send_packet(&x.peer_socket_address, crate::packets::clientbound::play::SetContainerProperty::PACKET_ID, crate::packets::clientbound::play::SetContainerProperty {
            window_id: 1,
            property: 0, //fuel left
            value: self.lit_time_remaining, //ticks of fuel left
          }.try_into().unwrap());

          game.send_packet(&x.peer_socket_address, crate::packets::clientbound::play::SetContainerProperty::PACKET_ID, crate::packets::clientbound::play::SetContainerProperty {
            window_id: 1,
            property: 1, //max fuel burn time
            value: 1600, //ticks fuel should burn for
          }.try_into().unwrap());

          game.send_packet(&x.peer_socket_address, crate::packets::clientbound::play::SetContainerProperty::PACKET_ID, crate::packets::clientbound::play::SetContainerProperty {
            window_id: 1,
            property: 3, //max progress
            value: 200, //progress from 0-200
          }.try_into().unwrap());

          game.send_packet(&x.peer_socket_address, crate::packets::clientbound::play::SetContainerProperty::PACKET_ID, crate::packets::clientbound::play::SetContainerProperty {
            window_id: 1,
            property: 2, //progress
            value: self.cooking_time_spent, //progress from 0-200
          }.try_into().unwrap());
        });
    } else {
      println!("Im a furnace that doesnt need ticking, but got ticked regardless");
    }
  }

  fn new(position: BlockPosition) -> Self {
    Furnace {
      needs_ticking: false,
      position,
      components: Vec::new(),
      inventory: vec![Item::default();3],
      lit_time_remaining: 0,
      cooking_time_spent: 0,
      cooking_total_time: 0,
      lit_total_time: 0,
    }
  }

  fn get_contained_items_mut(&mut self) -> &mut[Item] {
    return &mut self.inventory;
  }

  fn get_contained_items_owned(&self) -> Vec<Item> {
    return self.inventory.clone();
  }
}


impl From<Furnace> for Vec<NbtTag> {
  fn from(value: Furnace) -> Self {
    vec![
      value.inventory.into(),
      NbtTag::Short("lit_time_remaining".to_string(), value.lit_time_remaining),
      NbtTag::Short("cooking_time_spent".to_string(), value.cooking_time_spent),
      NbtTag::Short("cooking_total_time".to_string(), value.cooking_total_time),
      NbtTag::Short("lit_total_time".to_string(), value.lit_total_time),
    ]
  }
}

impl TryFrom<NbtListTag> for Furnace {
  type Error = Box<dyn Error>;

  fn try_from(value: NbtListTag) -> Result<Self, Self::Error> {
    let x = value.get_child("x").unwrap().as_int();
    let y = value.get_child("y").unwrap().as_int() as i16;
    let z = value.get_child("z").unwrap().as_int();
    let position = BlockPosition { x, y, z };

    let mut inventory = vec![Item::default(); 3];
    if let Some(items) = value.get_child("Items") {
      for entry in items.as_list() {
        inventory[entry.get_child("Slot").unwrap().as_byte() as usize] = Item {
          id: entry.get_child("id").unwrap().as_string().to_string(),
          count: entry.get_child("count").unwrap().as_int() as u8,
          components: Vec::new()
        };
      }
    }

    let lit_time_remaining = value.get_child("lit_time_remaining").unwrap_or(&NbtTag::Short("".to_string(), 0)).as_short();
    let slot_input = inventory[0].clone();

    return Ok(Furnace {
      lit_time_remaining,
      cooking_time_spent: value.get_child("cooking_time_spent").unwrap_or(&NbtTag::Short("".to_string(), 0)).as_short(),
      cooking_total_time: value.get_child("cooking_total_time").unwrap_or(&NbtTag::Short("".to_string(), 0)).as_short(),
      lit_total_time: value.get_child("lit_total_time").unwrap_or(&NbtTag::Short("".to_string(), 0)).as_short(),
      position,
      components: Vec::new(),
      needs_ticking: slot_input.count > 0,
      inventory,
    });
  }
}
