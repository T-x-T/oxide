use data::blocks::*;

use crate::{CardinalDirection, Dimension, Position};

pub fn get_block_state_id(face: u8, cardinal_direction: CardinalDirection, dimension: &Dimension, position: Position, used_item_name: String, cursor_position_x: f32, cursor_position_y: f32, cursor_position_z: f32) -> Vec<(i32, Position)> {
  let block = data::blocks::get_block_from_name(used_item_name);
  let mut output: Vec<(i32, Position)> = Vec::new();

  match block.block_type {
    Type::RotatedPillar => {
      let axis = match face {
        0|1 => RotatedPillarAxis::Y,
        2|3 => RotatedPillarAxis::Z,
        _ => RotatedPillarAxis::X,
      };
      output.push((block.states.iter().find(|x| x.properties.contains(&Property::RotatedPillarAxis(axis.clone()))).unwrap().id, position));
    },
    Type::Door => {
      let facing = match cardinal_direction {
        CardinalDirection::North => DoorFacing::North,
        CardinalDirection::East => DoorFacing::East,
        CardinalDirection::South => DoorFacing::South,
        CardinalDirection::West => DoorFacing::West,
      };

      let open = DoorOpen::False;
      let powered = DoorPowered::False;

      let position_to_check = match cardinal_direction {
        CardinalDirection::North => Position { x: position.x - 1, ..position },
        CardinalDirection::East => Position { z: position.z - 1, ..position },
        CardinalDirection::South => Position { x: position.x + 1, ..position },
        CardinalDirection::West => Position { z: position.z + 1, ..position },
      };
      let block_id_next_to_door = dimension.get_block(position_to_check).unwrap_or(0);
      let block_id_of_valid_door = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(facing.clone())) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(open.clone())) && x.properties.contains(&Property::DoorPowered(powered.clone()))).unwrap().id;
      let we_must_go_double_door_mode = block_id_next_to_door == block_id_of_valid_door;
      let hinge = if we_must_go_double_door_mode { DoorHinge::Right } else { DoorHinge::Left };

      output.push((block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(facing.clone())) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(hinge.clone())) && x.properties.contains(&Property::DoorOpen(open.clone())) && x.properties.contains(&Property::DoorPowered(powered.clone()))).unwrap().id, position));
      output.push((block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(facing.clone())) && x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)) && x.properties.contains(&Property::DoorHinge(hinge.clone())) && x.properties.contains(&Property::DoorOpen(open.clone())) && x.properties.contains(&Property::DoorPowered(powered.clone()))).unwrap().id, Position { x: position.x, y: position.y + 1, z: position.z }));
    },
    Type::Slab => {
      //TODO: logic of doubling slabs is still kinda wonky, for example it gets doubled when placing against the bottom face of a lower slab
      let position_to_check = if face == 0 {
        Position { y: position.y + 1, ..position }
      } else if face == 1 {
        Position { y: position.y - 1, ..position }
      } else {
        position
      };
      let existing_block_id = dimension.get_block(position_to_check).unwrap_or(0);
      let block_ids_of_slabs: Vec<i32> = block.states.iter().filter(|x| !x.properties.contains(&Property::SlabType(SlabType::Double))).map(|x| x.id).collect();
      let double_it_up = block_ids_of_slabs.contains(&existing_block_id);
      let place_top = (face >= 2 && cursor_position_y > 0.5) || face == 0;
      let slab_type_to_place = if double_it_up { SlabType::Double } else if place_top { SlabType::Top } else { SlabType::Bottom };

      let block_ids_of_double_slab: Vec<i32> = block.states.iter().filter(|x| x.properties.contains(&Property::SlabType(SlabType::Double))).map(|x| x.id).collect();
      let its_already_doubled = block_ids_of_double_slab.contains(&existing_block_id);
      let position_to_place = if its_already_doubled { position } else if double_it_up { position_to_check } else { position };

      output.push((block.states.iter().find(|x| x.properties.contains(&Property::SlabType(slab_type_to_place.clone())) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id, position_to_place));
     }
    _ => (),
  }

  if output.is_empty() {
    output.push((block.states.iter().find(|x| x.default).unwrap().id, position));
  }

  return output;
}
