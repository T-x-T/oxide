#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
  pub x: i32,
  pub y: i16,
  pub z: i32,
}

impl Position {
  pub fn convert_to_position_in_chunk(&self) -> Position {
    return Position {
      x: if self.x >= 0 {self.x % 16} else if 16 - ((-self.x) % 16) != 16 {16 - ((-self.x) % 16)} else {0},
      y: self.y,
      z: if self.z >= 0 {self.z % 16} else if 16 - ((-self.z) % 16) != 16 {16 - ((-self.z) % 16)} else {0},
    };
  }

  pub fn convert_to_coordinates_of_chunk(&self) -> Position {
    let chunk_x = if self.x >= 0 { self.x / 16 } else { (self.x - 15) / 16 };
    let chunk_z = if self.z >= 0 { self.z / 16 } else { (self.z - 15) / 16 };

    return Position { x: chunk_x, y: 0, z: chunk_z };
  }
}


#[cfg(test)]
mod test {
  use super::*;
  mod convert_to_chunk_position {
    use super::*;

    #[test]
    fn positive_x_under_15() {
      let position = Position {x: 7, y: 0, z: 100};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.x, 7);
    }

    #[test]
    fn positive_z_under_15() {
      let position = Position {x: 7, y: 0, z: 10};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.z, 10);
    }

    #[test]
    fn positive_x_over_15() {
      let position = Position {x: 17, y: 0, z: 100};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.x, 1);
    }

    #[test]
    fn positive_z_over_15() {
      let position = Position {x: 7, y: 0, z: 20};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.z, 4);
    }

    #[test]
    fn negative_x_under_15() {
      let position = Position {x: -7, y: 0, z: 100};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.x, 9);
    }

    #[test]
    fn negative_z_under_15() {
      let position = Position {x: 7, y: 0, z: -10};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.z, 6);
    }

    #[test]
    fn negative_x_over_15() {
      let position = Position {x: -17, y: 0, z: 100};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.x, 15);
    }

    #[test]
    fn negative_z_over_15() {
      let position = Position {x: 7, y: 0, z: -20};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.z, 12);
    }

    #[test]
    fn negative_y() {
      let position = Position {x: 7, y: -33, z: -20};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.y, -33);
    }

    #[test]
    fn positive_y() {
      let position = Position {x: 7, y: 225, z: -20};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.y, 225);
    }

    #[test]
    fn x_0() {
      let position = Position {x: 0, y: 0, z: 100};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.x, 0);
    }

    #[test]
    fn z_0() {
      let position = Position {x: 7, y: 0, z: 0};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.z, 0);
    }

    #[test]
    fn x_15() {
      let position = Position {x: 15, y: 0, z: 100};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.x, 15);
    }

    #[test]
    fn z_15() {
      let position = Position {x: 7, y: 0, z: 15};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.z, 15);
    }

    #[test]
    fn x_16() {
      let position = Position {x: 16, y: 0, z: 100};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.x, 0);
    }

    #[test]
    fn z_16() {
      let position = Position {x: 7, y: 0, z: 16};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.z, 0);
    }

    #[test]
    fn x_negative_16() {
      let position = Position {x: -16, y: 0, z: 100};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.x, 0);
    }

    #[test]
    fn z_negative_16() {
      let position = Position {x: 7, y: 0, z: -16};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.z, 0);
    }

    #[test]
    fn x_negative_17() {
      let position = Position {x: -17, y: 0, z: 100};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.x, 15);
    }

    #[test]
    fn z_negative_17() {
      let position = Position {x: 7, y: 0, z: -17};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.z, 15);
    }

    #[test]
    fn x_negative_14() {
      let position = Position {x: -14, y: 0, z: 100};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.x, 2);
    }

    #[test]
    fn z_negative_14() {
      let position = Position {x: 7, y: 0, z: -14};
      let chunk_position = position.convert_to_position_in_chunk();
      assert_eq!(chunk_position.z, 2);
    }
  }

  mod convert_to_coordinates_of_chunk {
    use super::*;

    #[test]
    fn x_0() {
      let position = Position {x: 7, y: 0, z: 100};
      let chunk_coordinates = position.convert_to_coordinates_of_chunk();
      assert_eq!(chunk_coordinates.x, 0);
    }

    #[test]
    fn z_0() {
      let position = Position {x: 7, y: 0, z: 0};
      let chunk_coordinates = position.convert_to_coordinates_of_chunk();
      assert_eq!(chunk_coordinates.z, 0);
    }

    #[test]
    fn x_24() {
      let position = Position {x: 24, y: 0, z: 100};
      let chunk_coordinates = position.convert_to_coordinates_of_chunk();
      assert_eq!(chunk_coordinates.x, 1);
    }

    #[test]
    fn z_24() {
      let position = Position {x: 7, y: 0, z: 24};
      let chunk_coordinates = position.convert_to_coordinates_of_chunk();
      assert_eq!(chunk_coordinates.z, 1);
    }

    #[test]
    fn x_negative_8() {
      let position = Position {x: -8, y: 0, z: 100};
      let chunk_coordinates = position.convert_to_coordinates_of_chunk();
      assert_eq!(chunk_coordinates.x, -1);
    }

    #[test]
    fn z_negative_8() {
      let position = Position {x: 7, y: 0, z: -8};
      let chunk_coordinates = position.convert_to_coordinates_of_chunk();
      assert_eq!(chunk_coordinates.z, -1);
    }

    #[test]
    fn x_negative_114() {
      let position = Position {x: -114, y: 0, z: 100};
      let chunk_coordinates = position.convert_to_coordinates_of_chunk();
      assert_eq!(chunk_coordinates.x, -8);
    }

    #[test]
    fn z_negative_114() {
      let position = Position {x: 7, y: 0, z: -114};
      let chunk_coordinates = position.convert_to_coordinates_of_chunk();
      assert_eq!(chunk_coordinates.z, -8);
    }

    #[test]
    fn x_negative_112() {
      let position = Position {x: -112, y: 0, z: 100};
      let chunk_coordinates = position.convert_to_coordinates_of_chunk();
      assert_eq!(chunk_coordinates.x, -7);
    }

    #[test]
    fn z_negative_112() {
      let position = Position {x: 7, y: 0, z: -112};
      let chunk_coordinates = position.convert_to_coordinates_of_chunk();
      assert_eq!(chunk_coordinates.z, -7);
    }
  }
}
