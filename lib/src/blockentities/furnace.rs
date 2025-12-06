use super::*;

#[derive(Debug, Clone)]
pub struct Furnace {
  pub position: BlockPosition, //global position, NOT within the chunk
  pub components: Option<Vec<SlotComponent>>, //At least I think so?
  pub needs_ticking: bool,
  pub slot_input: Item,
  pub slot_fuel: Item,
  pub slot_output: Item,
  pub lit_time_remaining: i16,
  pub cooking_time_spent: i16,
  pub cooking_total_time: i16,
  pub lit_total_time: i16,
}

impl Furnace {
  pub fn tick(&mut self, players: &[Player], game: Arc<Game>) {
    if self.needs_ticking {
      if self.slot_input.count == 0 {
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

      if (self.slot_fuel.id != "minecraft:coal" && self.lit_time_remaining == 0) || self.slot_input.id != "minecraft:raw_iron" {
        self.needs_ticking = false;
        return;
      }

      let mut can_cook = true;
      if self.lit_time_remaining == 0 {
        if self.slot_fuel.count > 0 {
          self.slot_fuel = Item { count: self.slot_fuel.count - 1, ..self.slot_fuel.clone() };
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

          if self.slot_output.id == "minecraft:iron_ingot" {
            self.slot_output = Item { count: self.slot_output.count + 1, ..self.slot_output.clone() };
          } else {
            self.slot_output = Item { count: 1, id: "minecraft:iron_ingot".to_string(), components: Vec::new() };
          }
          self.slot_input = Item { count: self.slot_input.count - 1, ..self.slot_input.clone() };
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
            slot_data: vec![
              self.slot_input.clone().into(),
              self.slot_fuel.clone().into(),
              self.slot_output.clone().into(),
            ],
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

  pub fn new(position_global: BlockPosition) -> Self {
    Furnace {
      needs_ticking: false,
      position: position_global,
      components: None,
      slot_input: Item::default(),
      slot_fuel: Item::default(),
      slot_output: Item::default(),
      lit_time_remaining: 0,
      cooking_time_spent: 0,
      cooking_total_time: 0,
      lit_total_time: 0,
    }
  }

  pub fn get_contained_items_mut(&mut self) -> Vec<&mut Item> {
    return vec![&mut self.slot_input, &mut self.slot_fuel, &mut self.slot_output];
  }

  pub fn get_contained_items_owned(&self) -> Vec<Item> {
    return vec![self.slot_input.clone(), self.slot_fuel.clone(), self.slot_output.clone()];
  }
}


impl From<Furnace> for Vec<NbtTag> {
  fn from(value: Furnace) -> Self {
    vec![
      vec![
        value.slot_input,
        value.slot_fuel,
        value.slot_output,
      ].into(),
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

    let mut slots = vec![Item::default(); 3];
    if let Some(items) = value.get_child("Items") {
      for entry in items.as_list() {
        slots[entry.get_child("Slot").unwrap().as_byte() as usize] = Item {
          id: entry.get_child("id").unwrap().as_string().to_string(),
          count: entry.get_child("count").unwrap().as_int() as u8,
          components: Vec::new()
        };
      }
    }

    let lit_time_remaining = value.get_child("lit_time_remaining").unwrap_or(&NbtTag::Short("".to_string(), 0)).as_short();
    let slot_input = slots[0].clone();

    return Ok(Furnace {
      lit_time_remaining,
      cooking_time_spent: value.get_child("cooking_time_spent").unwrap_or(&NbtTag::Short("".to_string(), 0)).as_short(),
      cooking_total_time: value.get_child("cooking_total_time").unwrap_or(&NbtTag::Short("".to_string(), 0)).as_short(),
      lit_total_time: value.get_child("lit_total_time").unwrap_or(&NbtTag::Short("".to_string(), 0)).as_short(),
      position,
      components: None,
      needs_ticking: slot_input.count > 0,
      slot_input,
      slot_fuel: slots[1].clone(),
      slot_output: slots[2].clone(),
    });
  }
}
