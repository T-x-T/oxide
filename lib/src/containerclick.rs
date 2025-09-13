use crate::*;

pub fn handle(mode: u8, button: u8, orig_inventory_item: Option<Slot>, orig_cursor_item: Option<Slot>) -> (Option<Slot>, Option<Slot>) {
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
        if mode == 0 && button == 0 {
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
      if (mode == 0 && button == 0) || orig_inventory_item.item_count < 2 {
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
      if mode == 0 && button == 0 {
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn pick_item_up() {
    let orig_inventory_item = Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
    let orig_cursor_item = None;

    let (new_inventory_item, new_cursor_item) = handle(0, 0, orig_inventory_item.clone(), orig_cursor_item.clone());

    assert_eq!(new_inventory_item, orig_cursor_item);
    assert_eq!(new_cursor_item, orig_inventory_item);
  }

  #[test]
  fn pick_empty_item_up() {
    let orig_inventory_item = None;
    let orig_cursor_item = None;

    let (new_inventory_item, new_cursor_item) = handle(0, 0, orig_inventory_item.clone(), orig_cursor_item.clone());

    assert_eq!(new_inventory_item, orig_inventory_item);
    assert_eq!(new_cursor_item, orig_cursor_item);
  }

  #[test]
  fn put_item_down() {
    let orig_inventory_item = None;
    let orig_cursor_item = Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

    let (new_inventory_item, new_cursor_item) = handle(0, 0, orig_inventory_item.clone(), orig_cursor_item.clone());

    assert_eq!(new_inventory_item, orig_cursor_item);
    assert_eq!(new_cursor_item, orig_inventory_item);
  }

  #[test]
  fn put_item_down_on_full_stack() {
    let orig_inventory_item = Some(Slot { item_count: 64, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
    let orig_cursor_item = Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

    let (new_inventory_item, new_cursor_item) = handle(0, 0, orig_inventory_item.clone(), orig_cursor_item.clone());

    assert_eq!(new_inventory_item, orig_inventory_item);
    assert_eq!(new_cursor_item, orig_cursor_item);
  }

  #[test]
  fn swap_different_items() {
    let orig_inventory_item = Some(Slot { item_count: 12, item_id: 2, components_to_add: Vec::new(), components_to_remove: Vec::new() });
    let orig_cursor_item = Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

    let (new_inventory_item, new_cursor_item) = handle(0, 0, orig_inventory_item.clone(), orig_cursor_item.clone());

    assert_eq!(new_inventory_item, orig_cursor_item);
    assert_eq!(new_cursor_item, orig_inventory_item);
  }

  #[test]
  fn stack_up_under_limit() {
    let orig_inventory_item = Some(Slot { item_count: 12, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
    let orig_cursor_item = Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

    let (new_inventory_item, new_cursor_item) = handle(0, 0, orig_inventory_item.clone(), orig_cursor_item.clone());

    assert_eq!(new_inventory_item, Some(Slot { item_count: 13, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    assert_eq!(new_cursor_item, None);
  }

  #[test]
  fn stack_up_to_limit() {
    let orig_inventory_item = Some(Slot { item_count: 12, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
    let orig_cursor_item = Some(Slot { item_count: 52, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

    let (new_inventory_item, new_cursor_item) = handle(0, 0, orig_inventory_item.clone(), orig_cursor_item.clone());

    assert_eq!(new_inventory_item, Some(Slot { item_count: 64, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    assert_eq!(new_cursor_item, None);
  }

  #[test]
  fn stack_up_over_limit() {
    let orig_inventory_item = Some(Slot { item_count: 22, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
    let orig_cursor_item = Some(Slot { item_count: 52, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

    let (new_inventory_item, new_cursor_item) = handle(0, 0, orig_inventory_item.clone(), orig_cursor_item.clone());

    assert_eq!(new_inventory_item, Some(Slot { item_count: 64, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    assert_eq!(new_cursor_item, Some(Slot { item_count: 10, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
  }

  #[test]
  fn pick_half_stack_up_even() {
    let orig_inventory_item = Some(Slot { item_count: 10, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
    let orig_cursor_item = None;

    let (new_inventory_item, new_cursor_item) = handle(0, 1, orig_inventory_item.clone(), orig_cursor_item.clone());

    assert_eq!(new_inventory_item, Some(Slot { item_count: 5, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    assert_eq!(new_cursor_item, Some(Slot { item_count: 5, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
  }

  #[test]
  fn pick_half_stack_up_odd() {
    let orig_inventory_item = Some(Slot { item_count: 11, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
    let orig_cursor_item = None;

    let (new_inventory_item, new_cursor_item) = handle(0, 1, orig_inventory_item.clone(), orig_cursor_item.clone());

    assert_eq!(new_inventory_item, Some(Slot { item_count: 5, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    assert_eq!(new_cursor_item, Some(Slot { item_count: 6, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
  }

  #[test]
  fn pick_half_stack_up_one() {
    let orig_inventory_item = Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
    let orig_cursor_item = None;

    let (new_inventory_item, new_cursor_item) = handle(0, 1, orig_inventory_item.clone(), orig_cursor_item.clone());

    assert_eq!(new_inventory_item, None);
    assert_eq!(new_cursor_item, Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
  }

  #[test]
  fn put_one_item_down() {
    let orig_inventory_item = None;
    let orig_cursor_item = Some(Slot { item_count: 10, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

    let (new_inventory_item, new_cursor_item) = handle(0, 1, orig_inventory_item.clone(), orig_cursor_item.clone());

    assert_eq!(new_inventory_item, Some(Slot { item_count: 1, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    assert_eq!(new_cursor_item, Some(Slot { item_count: 9, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
  }

  #[test]
  fn put_one_item_down_with_same_already_there() {
    let orig_inventory_item = Some(Slot { item_count: 10, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
    let orig_cursor_item = Some(Slot { item_count: 10, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

    let (new_inventory_item, new_cursor_item) = handle(0, 1, orig_inventory_item.clone(), orig_cursor_item.clone());

    assert_eq!(new_inventory_item, Some(Slot { item_count: 11, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    assert_eq!(new_cursor_item, Some(Slot { item_count: 9, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
  }

  #[test]
  fn put_one_item_down_with_stack_full() {
    let orig_inventory_item = Some(Slot { item_count: 64, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });
    let orig_cursor_item = Some(Slot { item_count: 10, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() });

    let (new_inventory_item, new_cursor_item) = handle(0, 1, orig_inventory_item.clone(), orig_cursor_item.clone());

    assert_eq!(new_inventory_item, Some(Slot { item_count: 64, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
    assert_eq!(new_cursor_item, Some(Slot { item_count: 10, item_id: 1, components_to_add: Vec::new(), components_to_remove: Vec::new() }));
  }
}
