use data::blocks::*;

use crate::{CardinalDirection, Dimension, Position};

pub fn get_block_state_id(face: u8, cardinal_direction: CardinalDirection, dimension: &mut Dimension, position: Position, used_item_name: String) -> Vec<(i32, Position)> {
  let block = data::blocks::get_block_from_name(used_item_name);
  let mut output: Vec<(i32, Position)> = Vec::new();

  for property in block.properties.clone() {
    match property {
      Property::RotatedPillarAxis(rotated_pillar_axis) => {
        match rotated_pillar_axis {
          data::blocks::RotatedPillarAxis::X => if face == 4 || face == 5 { output.push((block.states.iter().find(|x| x.properties.contains(&Property::RotatedPillarAxis(rotated_pillar_axis.clone()))).unwrap().id, position)) },
          data::blocks::RotatedPillarAxis::Y => if face == 0 || face == 1 { output.push((block.states.iter().find(|x| x.properties.contains(&Property::RotatedPillarAxis(rotated_pillar_axis.clone()))).unwrap().id, position)) },
          data::blocks::RotatedPillarAxis::Z => if face == 2 || face == 3 { output.push((block.states.iter().find(|x| x.properties.contains(&Property::RotatedPillarAxis(rotated_pillar_axis.clone()))).unwrap().id, position)) },
        };
      },
      Property::DoorFacing(door_facing) => {
        match door_facing {
          data::blocks::DoorFacing::North => if cardinal_direction == CardinalDirection::North {
            output.push((block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(door_facing.clone())) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id, position));
            output.push((block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(door_facing.clone())) && x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id, Position { x: position.x, y: position.y + 1, z: position.z }))
          },
          data::blocks::DoorFacing::East => if cardinal_direction == CardinalDirection::East {
            output.push((block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(door_facing.clone())) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id, position));
            output.push((block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(door_facing.clone())) && x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id, Position { x: position.x, y: position.y + 1, z: position.z }))
          },
          data::blocks::DoorFacing::South => if cardinal_direction == CardinalDirection::South {
            output.push((block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(door_facing.clone())) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id, position));
            output.push((block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(door_facing.clone())) && x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id, Position { x: position.x, y: position.y + 1, z: position.z }))
          },
          data::blocks::DoorFacing::West => if cardinal_direction == CardinalDirection::West {
            output.push((block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(door_facing.clone())) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id, position));
            output.push((block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(door_facing.clone())) && x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id, Position { x: position.x, y: position.y + 1, z: position.z }))
          },
        }
      },
      _ => (),
    }
  }

  if output.is_empty() {
    output.push((block.states.iter().find(|x| x.default).unwrap().id, position));
  }

  return output;
}
