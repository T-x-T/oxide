use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};

use crate::packets::Packet;
use crate::types::*;

pub fn handle(parsed_packet: crate::packets::serverbound::play::ClickContainer, chest_items: &mut [Item], player: &mut Player, connections: &HashMap<SocketAddr, Connection>, connection_streams: &HashMap<SocketAddr, TcpStream>, streams_with_container_opened: Vec<TcpStream>) {
  const PLAYER_INVENTORY_STARTING_INDEX: i16 = 9;
  let player_inventory_index = parsed_packet.slot - chest_items.len() as i16 + PLAYER_INVENTORY_STARTING_INDEX;

  let outside_clicked = parsed_packet.slot < 0;
  let chest_inventory_clicked = parsed_packet.slot < chest_items.len() as i16;
  let orig_inventory_item: Option<Slot> = if outside_clicked {
    None
  } else if chest_inventory_clicked {
    if chest_items[parsed_packet.slot as usize].count > 0 {
      Some(chest_items[parsed_packet.slot as usize].clone().into())
    } else {
      None
    }
  } else {
    player.get_inventory()[player_inventory_index as usize].clone()
  };

  //println!("orig item: {orig_inventory_item:?}");
  let orig_cursor_item: Option<Slot> = player.cursor_item.clone();
  //println!("orig cursor: {orig_cursor_item:?}");

  if parsed_packet.mode == 0 {
    let (new_inventory_item, new_cursor_item) = handle_click(parsed_packet.button == 0, orig_inventory_item.clone(), orig_cursor_item.clone());

    //println!("new item: {new_inventory_item:?}");
    if new_inventory_item != orig_inventory_item {
      if outside_clicked {
        //nothing to do
      } else if chest_inventory_clicked {
        //Chest inventory got changed
        chest_items[parsed_packet.slot as usize] = new_inventory_item.clone().into();
        for stream in streams_with_container_opened {
          crate::utils::send_packet(&stream, crate::packets::clientbound::play::SetContainerSlot::PACKET_ID, crate::packets::clientbound::play::SetContainerSlot {
            window_id: 1,
            state_id: 1,
            slot: parsed_packet.slot,
            slot_data: new_inventory_item.clone(),
          }.try_into().unwrap()).unwrap();
        }
      } else {
        //Player inventory got changed
        player.set_inventory_slot(player_inventory_index as u8, new_inventory_item, connections, connection_streams);
      }
    }

    //println!("new cursor: {new_cursor_item:?}");
    if new_cursor_item != orig_cursor_item {
      player.cursor_item = new_cursor_item;
    }
  } else if parsed_packet.mode == 1 {
    let orig_chest_inventory: Vec<Option<Slot>> = chest_items.to_vec().clone().into_iter().map(|x| x.into()).collect();
    let orig_player_inventory: Vec<Option<Slot>> = player.get_inventory().clone();
    let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory.clone(), orig_player_inventory.clone(), parsed_packet.slot);

    if orig_chest_inventory != new_chest_inventory {
      let new_chest_items: Vec<Item> = new_chest_inventory.into_iter().map(|x| x.into()).collect();
      assert_eq!(chest_items.len(), new_chest_items.len());
      chest_items.clone_from_slice(&new_chest_items);

      for connection_stream in connection_streams.iter().clone() {
        let _ = crate::utils::send_packet(connection_stream.1, crate::packets::clientbound::play::SetContainerContent::PACKET_ID, crate::packets::clientbound::play::SetContainerContent {
          window_id: 1,
          state_id: 1,
          slot_data: chest_items.iter().cloned().map(|x| x.into()).collect(),
          carried_item: None,
        }.try_into().unwrap());
      }
    }

    if orig_player_inventory != new_player_inventory {
      player.set_inventory(new_player_inventory, connections, connection_streams);
    }
  } else {
    crate::utils::send_packet(&player.connection_stream, crate::packets::clientbound::play::SetContainerContent::PACKET_ID, crate::packets::clientbound::play::SetContainerContent {
      window_id: 1,
      state_id: 1,
      slot_data: chest_items.to_vec().clone().into_iter().map(|x| x.into()).collect(),
      carried_item: orig_cursor_item.clone(),
    }.try_into().unwrap()).unwrap();
    crate::utils::send_packet(&player.connection_stream, crate::packets::clientbound::play::SetContainerContent::PACKET_ID, crate::packets::clientbound::play::SetContainerContent {
      window_id: 0,
      state_id: 1,
      slot_data: player.get_inventory().clone(),
      carried_item: orig_cursor_item,
    }.try_into().unwrap()).unwrap();
  }
}

fn handle_click(left_click: bool, orig_inventory_item: Option<Slot>, orig_cursor_item: Option<Slot>) -> (Option<Slot>, Option<Slot>) {
  let mut new_inventory_item: Option<Slot> = None;
  let mut new_cursor_item: Option<Slot> = None;

  if let Some(orig_inventory_item) = orig_inventory_item.clone() {
    //Slot has items
    if let Some(orig_cursor_item) = orig_cursor_item.clone() {
      //Swap items or stack up?
      if orig_cursor_item.item_id == orig_inventory_item.item_id {
        //stack up
        let item_count_cursor = orig_cursor_item.item_count;
        let item_count_chest = orig_inventory_item.item_count;
        let max_stack_size = data::items::get_items().get(&data::items::get_item_name_by_id(orig_inventory_item.item_id)).unwrap().max_stack_size as i32;
        if left_click {
          //put all down
          let left_over_item_count = if ((item_count_chest + item_count_cursor) - max_stack_size).is_negative() { 0 } else { (item_count_chest + item_count_cursor) - max_stack_size };
          if left_over_item_count > 0 {
            new_inventory_item = Some(Slot { item_count: max_stack_size, ..orig_cursor_item.clone() });
            new_cursor_item = Some(Slot { item_count: left_over_item_count, ..orig_cursor_item.clone() });
          } else {
            new_inventory_item = Some(Slot { item_count: orig_inventory_item.item_count + item_count_cursor, ..orig_cursor_item.clone() });
            new_cursor_item = None;
          }
        } else {
          //put one down
          if orig_inventory_item.item_count != max_stack_size {
            new_inventory_item = Some(Slot { item_count: orig_inventory_item.item_count + 1, ..orig_inventory_item.clone() });
            new_cursor_item = Some(Slot { item_count: orig_cursor_item.item_count - 1, ..orig_cursor_item.clone() });
          } else {
            new_inventory_item = Some(orig_inventory_item.clone());
            new_cursor_item = Some(orig_cursor_item.clone());
          }
        }
      } else {
        //swap
        let temp_item_from_player = orig_cursor_item.clone();
        new_cursor_item = Some(orig_inventory_item);
        new_inventory_item = temp_item_from_player.clone().into();
      }
    } else {
      //can just take item from slot
      if (left_click) || orig_inventory_item.item_count < 2 {
        new_cursor_item = Some(orig_inventory_item);
        new_inventory_item = None;
      } else {
        new_inventory_item = Some(Slot { item_count: orig_inventory_item.item_count / 2, ..orig_inventory_item.clone() });
        new_cursor_item = Some(Slot { item_count: (f64::from(orig_inventory_item.item_count) / 2.0).ceil() as i32, ..orig_inventory_item.clone() });
      }
    }
  } else {
    //Slot doesnt have items
    if let Some(orig_cursor_item) = orig_cursor_item {
      if left_click {
        new_inventory_item = Some(orig_cursor_item.clone());
        new_cursor_item = None;
      } else {
        new_inventory_item = Some(Slot {item_count: 1, ..orig_cursor_item.clone()});
        new_cursor_item = Some(Slot { item_count: orig_cursor_item.item_count - 1, ..orig_cursor_item.clone() });
      }
    }
  }

  return (new_inventory_item, new_cursor_item);
}

fn handle_shift_click(orig_chest_inventory: Vec<Option<Slot>>, orig_player_inventory: Vec<Option<Slot>>, clicked_slot: i16) -> (Vec<Option<Slot>>, Vec<Option<Slot>>) {
  const PLAYER_INVENTORY_STARTING_INDEX: i16 = 9;
  let player_inventory_index = (clicked_slot - orig_chest_inventory.len() as i16 + PLAYER_INVENTORY_STARTING_INDEX) as usize;
  let clicked_slot = clicked_slot as usize;

  let mut new_chest_inventory = orig_chest_inventory.clone();
  let mut new_player_inventory = orig_player_inventory.clone();

  if clicked_slot > orig_chest_inventory.len() {
    //player inventory clicked
    if orig_player_inventory[player_inventory_index].is_none() {
      return (new_chest_inventory, new_player_inventory);
    }
    let max_stack_size = data::items::get_items().get(&data::items::get_item_name_by_id(orig_player_inventory[player_inventory_index].clone().unwrap().item_id)).unwrap().max_stack_size as i32;

    orig_chest_inventory.iter()
      .enumerate()
      .filter(|x|
        x.1.as_ref().is_some_and(|slot| slot.item_id == orig_player_inventory[player_inventory_index].as_ref().unwrap().item_id)
      ).for_each(|(i, slot)| {
        if new_player_inventory[player_inventory_index].is_some() {
          let left_over_count = max_stack_size - slot.as_ref().unwrap().item_count;
          if left_over_count >= new_player_inventory[player_inventory_index].as_ref().unwrap().item_count {
            new_chest_inventory[i] = Some(Slot { item_count: slot.as_ref().unwrap().item_count + new_player_inventory[player_inventory_index].as_ref().unwrap().item_count, ..orig_chest_inventory[i].clone().unwrap()});
            new_player_inventory[player_inventory_index] = None;
          } else {
            new_chest_inventory[i] = Some(Slot { item_count: max_stack_size, ..orig_chest_inventory[i].clone().unwrap()});
            new_player_inventory[player_inventory_index] = Some(Slot { item_count: (slot.as_ref().unwrap().item_count + new_player_inventory[player_inventory_index].as_ref().unwrap().item_count) - max_stack_size, ..new_player_inventory[player_inventory_index].clone().unwrap()});
          }
        }
      });

    if new_player_inventory[player_inventory_index].is_none() {
      return (new_chest_inventory, new_player_inventory);
    }

    let first_free_slot = orig_chest_inventory.iter().enumerate().find(|x| x.1.is_none()).map(|x| x.0);
    if let Some(first_free_slot) = first_free_slot {
      new_chest_inventory[first_free_slot] = new_player_inventory[player_inventory_index].clone();
      new_player_inventory[player_inventory_index] = None;
    }
  } else {
    //chest inventory clicked
    if orig_chest_inventory[clicked_slot].is_none() {
      return (new_chest_inventory, new_player_inventory);
    }

    let max_stack_size = data::items::get_items().get(&data::items::get_item_name_by_id(orig_chest_inventory[clicked_slot].clone().unwrap().item_id)).unwrap().max_stack_size as i32;

    orig_player_inventory.iter()
      .enumerate()
      .filter(|x|
        x.1.as_ref().is_some_and(|slot| slot.item_id == orig_chest_inventory[clicked_slot].as_ref().unwrap().item_id)
      ).for_each(|(i, slot)| {
        if new_chest_inventory[clicked_slot].is_some() {
          let left_over_count = max_stack_size - slot.as_ref().unwrap().item_count;
          if left_over_count >= new_chest_inventory[clicked_slot].as_ref().unwrap().item_count {
            new_player_inventory[i] = Some(Slot { item_count: slot.as_ref().unwrap().item_count + new_chest_inventory[clicked_slot].as_ref().unwrap().item_count, ..new_player_inventory[i].clone().unwrap()});
            new_chest_inventory[clicked_slot] = None;
          } else {
            new_player_inventory[i] = Some(Slot { item_count: max_stack_size, ..new_player_inventory[i].clone().unwrap()});
            new_chest_inventory[clicked_slot] = Some(Slot { item_count: (slot.as_ref().unwrap().item_count + new_chest_inventory[clicked_slot].as_ref().unwrap().item_count) - max_stack_size, ..new_chest_inventory[clicked_slot].clone().unwrap()});
          }
        }
      });

    if new_chest_inventory[clicked_slot].is_none() {
      return (new_chest_inventory, new_player_inventory);
    }

    let slot_order: Vec<usize> = vec![(36..=44).rev().map(|x| x as usize).collect::<Vec<usize>>(),(9..=35).map(|x| x as usize).collect()].into_iter().flatten().collect();
    let first_free_slot = slot_order.into_iter().find(|x| new_player_inventory[*x].is_none());
    if let Some(first_free_slot) = first_free_slot {
      new_player_inventory[first_free_slot] = new_chest_inventory[clicked_slot].clone();
      new_chest_inventory[clicked_slot] = None;
    }
  }

  return (new_chest_inventory, new_player_inventory);
}

#[cfg(test)]
mod test {
  use super::*;

  mod handle_click {
    use super::*;

    #[test]
    fn pick_item_up() {
      let orig_inventory_item = Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
      let orig_cursor_item = None;

      let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

      assert_eq!(new_inventory_item, orig_cursor_item);
      assert_eq!(new_cursor_item, orig_inventory_item);
    }

    #[test]
    fn pick_empty_item_up() {
      let orig_inventory_item = None;
      let orig_cursor_item = None;

      let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

      assert_eq!(new_inventory_item, orig_inventory_item);
      assert_eq!(new_cursor_item, orig_cursor_item);
    }

    #[test]
    fn put_item_down() {
      let orig_inventory_item = None;
      let orig_cursor_item = Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

      let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

      assert_eq!(new_inventory_item, orig_cursor_item);
      assert_eq!(new_cursor_item, orig_inventory_item);
    }

    #[test]
    fn put_item_down_on_full_stack() {
      let orig_inventory_item = Some(Slot { item_count: 64, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
      let orig_cursor_item = Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

      let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

      assert_eq!(new_inventory_item, orig_inventory_item);
      assert_eq!(new_cursor_item, orig_cursor_item);
    }

    #[test]
    fn swap_different_items() {
      let orig_inventory_item = Some(Slot { item_count: 12, item_id: 2, components_to_add: Vec::new(), components_to_remove: Vec::new() });
      let orig_cursor_item = Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

      let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

      assert_eq!(new_inventory_item, orig_cursor_item);
      assert_eq!(new_cursor_item, orig_inventory_item);
    }

    #[test]
    fn stack_up_under_limit() {
      let orig_inventory_item = Some(Slot { item_count: 12, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
      let orig_cursor_item = Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

      let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

      assert_eq!(new_inventory_item, Some(Slot { item_count: 13, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
      assert_eq!(new_cursor_item, None);
    }

    #[test]
    fn stack_up_to_limit() {
      let orig_inventory_item = Some(Slot { item_count: 12, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
      let orig_cursor_item = Some(Slot { item_count: 52, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

      let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

      assert_eq!(new_inventory_item, Some(Slot { item_count: 64, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
      assert_eq!(new_cursor_item, None);
    }

    #[test]
    fn stack_up_over_limit() {
      let orig_inventory_item = Some(Slot { item_count: 22, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
      let orig_cursor_item = Some(Slot { item_count: 52, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

      let (new_inventory_item, new_cursor_item) = handle_click(true, orig_inventory_item.clone(), orig_cursor_item.clone());

      assert_eq!(new_inventory_item, Some(Slot { item_count: 64, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
      assert_eq!(new_cursor_item, Some(Slot { item_count: 10, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    }

    #[test]
    fn pick_half_stack_up_even() {
      let orig_inventory_item = Some(Slot { item_count: 10, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
      let orig_cursor_item = None;

      let (new_inventory_item, new_cursor_item) = handle_click(false, orig_inventory_item.clone(), orig_cursor_item.clone());

      assert_eq!(new_inventory_item, Some(Slot { item_count: 5, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
      assert_eq!(new_cursor_item, Some(Slot { item_count: 5, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    }

    #[test]
    fn pick_half_stack_up_odd() {
      let orig_inventory_item = Some(Slot { item_count: 11, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
      let orig_cursor_item = None;

      let (new_inventory_item, new_cursor_item) = handle_click(false, orig_inventory_item.clone(), orig_cursor_item.clone());

      assert_eq!(new_inventory_item, Some(Slot { item_count: 5, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
      assert_eq!(new_cursor_item, Some(Slot { item_count: 6, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    }

    #[test]
    fn pick_half_stack_up_one() {
      let orig_inventory_item = Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
      let orig_cursor_item = None;

      let (new_inventory_item, new_cursor_item) = handle_click(false, orig_inventory_item.clone(), orig_cursor_item.clone());

      assert_eq!(new_inventory_item, None);
      assert_eq!(new_cursor_item, Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    }

    #[test]
    fn put_one_item_down() {
      let orig_inventory_item = None;
      let orig_cursor_item = Some(Slot { item_count: 10, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

      let (new_inventory_item, new_cursor_item) = handle_click(false, orig_inventory_item.clone(), orig_cursor_item.clone());

      assert_eq!(new_inventory_item, Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
      assert_eq!(new_cursor_item, Some(Slot { item_count: 9, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    }

    #[test]
    fn put_one_item_down_with_same_already_there() {
      let orig_inventory_item = Some(Slot { item_count: 10, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
      let orig_cursor_item = Some(Slot { item_count: 10, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

      let (new_inventory_item, new_cursor_item) = handle_click(false, orig_inventory_item.clone(), orig_cursor_item.clone());

      assert_eq!(new_inventory_item, Some(Slot { item_count: 11, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
      assert_eq!(new_cursor_item, Some(Slot { item_count: 9, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    }

    #[test]
    fn put_one_item_down_with_stack_full() {
      let orig_inventory_item = Some(Slot { item_count: 64, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
      let orig_cursor_item = Some(Slot { item_count: 10, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

      let (new_inventory_item, new_cursor_item) = handle_click(false, orig_inventory_item.clone(), orig_cursor_item.clone());

      assert_eq!(new_inventory_item, Some(Slot { item_count: 64, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
      assert_eq!(new_cursor_item, Some(Slot { item_count: 10, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    }
  }

  mod handle_shift_click {
    use super::*;

    #[test]
    fn shift_click_on_empty_player_slot() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![None;27];
      let orig_player_inventory: Vec<Option<Slot>> = vec![None;46];
      let clicked_slot = 54;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![None;27]);
      assert_eq!(new_player_inventory, vec![None;46]);
    }

    #[test]
    fn shift_click_on_empty_chest_slot() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![None;27];
      let orig_player_inventory: Vec<Option<Slot>> = vec![None;46];
      let clicked_slot = 1;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![None;27]);
      assert_eq!(new_player_inventory, vec![None;46]);
    }

    #[test]
    fn shift_click_to_empty_chest() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![None;27];
      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![None;36],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect();
      let clicked_slot = 54;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;26],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());

      assert_eq!(new_player_inventory, vec![None;46]);
    }

    #[test]
    fn shift_click_to_chest_with_items() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;2],
        vec![Some(Slot {item_id: 2, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;23]
      ].into_iter().flatten().collect();
      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![None;36],
        vec![Some(Slot {item_id: 3, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect();
      let clicked_slot = 54;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![Some(Slot {item_id: 3, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None],
        vec![Some(Slot {item_id: 2, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;23],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());

      assert_eq!(new_player_inventory, vec![None;46]);
    }

    #[test]
    fn shift_click_to_chest_with_items_stack_up() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;2],
        vec![Some(Slot {item_id: 2, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;23]
      ].into_iter().flatten().collect();
      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![None;36],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect();
      let clicked_slot = 54;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![
        vec![Some(Slot {item_id: 1, item_count: 24, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;2],
        vec![Some(Slot {item_id: 2, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;23],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());

      assert_eq!(new_player_inventory, vec![None;46]);
    }

    #[test]
    fn shift_click_to_chest_with_items_stack_up_first_stack() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;2],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;23]
      ].into_iter().flatten().collect();
      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![None;36],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect();
      let clicked_slot = 54;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![
        vec![Some(Slot {item_id: 1, item_count: 24, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;2],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;23],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());

      assert_eq!(new_player_inventory, vec![None;46]);
    }

    #[test]
    fn shift_click_to_chest_with_items_stack_up_multiple() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 60, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;2],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;23]
      ].into_iter().flatten().collect();
      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![None;36],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect();
      let clicked_slot = 54;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![
        vec![Some(Slot {item_id: 1, item_count: 64, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;2],
        vec![Some(Slot {item_id: 1, item_count: 20, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;23],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());

      assert_eq!(new_player_inventory, vec![None;46]);
    }

    #[test]
    fn shift_click_to_chest_with_items_stack_up_lots() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 63, components_to_add: Vec::new(), components_to_remove: Vec::new()});20],
        vec![None;7]
      ].into_iter().flatten().collect();
      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![None;36],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect();
      let clicked_slot = 54;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![
        vec![Some(Slot {item_id: 1, item_count: 64, components_to_add: Vec::new(), components_to_remove: Vec::new()});12],
        vec![Some(Slot {item_id: 1, item_count: 63, components_to_add: Vec::new(), components_to_remove: Vec::new()});8],
        vec![None;7],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());

      assert_eq!(new_player_inventory, vec![None;46]);
    }

    #[test]
    fn shift_click_to_chest_with_items_stack_up_lots_overflow() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 63, components_to_add: Vec::new(), components_to_remove: Vec::new()});20],
        vec![None;7]
      ].into_iter().flatten().collect();
      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![None;36],
        vec![Some(Slot {item_id: 1, item_count: 24, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect();
      let clicked_slot = 54;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![
        vec![Some(Slot {item_id: 1, item_count: 64, components_to_add: Vec::new(), components_to_remove: Vec::new()});20],
        vec![Some(Slot {item_id: 1, item_count: 4, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;6],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());

      assert_eq!(new_player_inventory, vec![None;46]);
    }

    #[test]
    fn shift_click_to_chest_full() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()});27],
      ].into_iter().flatten().collect();
      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![None;36],
        vec![Some(Slot {item_id: 3, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect();
      let clicked_slot = 54;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()});27],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());

      assert_eq!(new_player_inventory, vec![
        vec![None;36],
        vec![Some(Slot {item_id: 3, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());
    }

    #[test]
    fn shift_click_to_empty_inventory() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;26]
      ].into_iter().flatten().collect();

      let orig_player_inventory: Vec<Option<Slot>> = vec![None;46];
      let clicked_slot = 0;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![None;27]);

      assert_eq!(new_player_inventory, vec![
        vec![None;44],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;1],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());
    }

    #[test]
    fn shift_click_to_inventory_with_items() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;26]
      ].into_iter().flatten().collect();

      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![None;43],
        vec![Some(Slot {item_id: 2, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![Some(Slot {item_id: 3, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;1],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>();
      let clicked_slot = 0;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![None;27]);

      assert_eq!(new_player_inventory, vec![
        vec![None;42],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![Some(Slot {item_id: 2, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![Some(Slot {item_id: 3, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;1],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());
    }

    #[test]
    fn shift_click_to_inventory_with_full_hotbar() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;26]
      ].into_iter().flatten().collect();

      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![None;9],
        vec![Some(Slot {item_id: 2, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;26],
        vec![Some(Slot {item_id: 3, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()});9],
        vec![None;1],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>();
      let clicked_slot = 0;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![None;27]);

      assert_eq!(new_player_inventory, vec![
        vec![None;9],
        vec![Some(Slot {item_id: 2, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;25],
        vec![Some(Slot {item_id: 3, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()});9],
        vec![None;1],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());
    }

    #[test]
    fn shift_click_to_inventory_full() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;26]
      ].into_iter().flatten().collect();

      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 2, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()});46],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>();
      let clicked_slot = 0;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;26]
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());

      assert_eq!(new_player_inventory, vec![
        vec![Some(Slot {item_id: 2, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()});46],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());
    }

    #[test]
    fn shift_click_to_inventory_with_items_stack_up() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;26],
      ].into_iter().flatten().collect();
      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 2, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;36],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect();
      let clicked_slot = 0;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![None;27]);

      assert_eq!(new_player_inventory, vec![
        vec![Some(Slot {item_id: 2, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;36],
        vec![Some(Slot {item_id: 1, item_count: 24, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());
    }

    #[test]
    fn shift_click_to_inventory_with_items_stack_up_first_stack() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;26],
      ].into_iter().flatten().collect();
      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;36],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect();
      let clicked_slot = 0;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![None;27]);

      assert_eq!(new_player_inventory, vec![
        vec![Some(Slot {item_id: 1, item_count: 24, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;36],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());
    }

    #[test]
    fn shift_click_to_inventory_with_items_stack_up_multiple() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;26],
      ].into_iter().flatten().collect();
      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 60, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;36],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect();
      let clicked_slot = 0;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![None;27]);

      assert_eq!(new_player_inventory, vec![
        vec![Some(Slot {item_id: 1, item_count: 64, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;36],
        vec![Some(Slot {item_id: 1, item_count: 20, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());
    }

    #[test]
    fn shift_click_to_inventory_with_items_stack_up_lots() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 10, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;26],
      ].into_iter().flatten().collect();
      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 63, components_to_add: Vec::new(), components_to_remove: Vec::new()});20],
        vec![None;16],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect();
      let clicked_slot = 0;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory, clicked_slot);

      assert_eq!(new_chest_inventory, vec![None;27]);

      assert_eq!(new_player_inventory, vec![
        vec![Some(Slot {item_id: 1, item_count: 64, components_to_add: Vec::new(), components_to_remove: Vec::new()});10],
        vec![Some(Slot {item_id: 1, item_count: 63, components_to_add: Vec::new(), components_to_remove: Vec::new()});10],
        vec![None;16],
        vec![Some(Slot {item_id: 1, item_count: 12, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;9]
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());
    }

    #[test]
    fn shift_click_to_inventory_with_items_stack_up_lots_with_overflow() {
      let orig_chest_inventory: Vec<Option<Slot>> = vec![
        vec![Some(Slot {item_id: 1, item_count: 24, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;26],
      ].into_iter().flatten().collect();
      let orig_player_inventory: Vec<Option<Slot>> = vec![
        vec![None;9],
        vec![Some(Slot {item_id: 1, item_count: 63, components_to_add: Vec::new(), components_to_remove: Vec::new()});20],
        vec![None;17],
      ].into_iter().flatten().collect();
      let clicked_slot = 0;

      let (new_chest_inventory, new_player_inventory) = handle_shift_click(orig_chest_inventory, orig_player_inventory.clone(), clicked_slot);

      assert_eq!(new_chest_inventory, vec![None;27]);

      assert_eq!(new_player_inventory, vec![
        vec![None;9],
        vec![Some(Slot {item_id: 1, item_count: 64, components_to_add: Vec::new(), components_to_remove: Vec::new()});20],
        vec![None;15],
        vec![Some(Slot {item_id: 1, item_count: 4, components_to_add: Vec::new(), components_to_remove: Vec::new()})],
        vec![None;1],
      ].into_iter().flatten().collect::<Vec<Option<Slot>>>());
    }
  }
}
