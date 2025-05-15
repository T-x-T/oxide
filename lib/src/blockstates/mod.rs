use data::blocks::*;

use crate::{CardinalDirection, Dimension, Position};

pub fn get_block_state_id(face: u8, cardinal_direction: CardinalDirection, dimension: &Dimension, position: Position, used_item_name: String) -> Vec<(i32, Position)> {
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
    _ => (),
  }

  if output.is_empty() {
    output.push((block.states.iter().find(|x| x.default).unwrap().id, position));
  }

  return output;
}
