use super::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CollisionShape {
	cuboids: Vec<Cuboid>,
	base_coordinates: EntityPosition,
}

impl CollisionShape {
	pub fn new_from_cuboid(cuboid: Cuboid, base_coordinates: EntityPosition) -> CollisionShape {
		return CollisionShape {
			cuboids: vec![cuboid],
			base_coordinates,
		};
	}

	pub fn add_cuboid(&mut self, cuboid: Cuboid) -> &mut CollisionShape {
		self.cuboids.push(cuboid);
		return self;
	}

	pub fn set_base_coordinates(&mut self, new_base_coordinates: EntityPosition) -> &mut CollisionShape {
		self.base_coordinates = new_base_coordinates;
		return self;
	}

	pub fn collides_with(&self, other: &CollisionShape) -> bool {
		if self.cuboids.is_empty() || other.cuboids.is_empty() {
			return false;
		}

		let self_cuboids_position_adjusted: Vec<Cuboid> = self
			.cuboids
			.iter()
			.map(|x| Cuboid {
				x1: x.x1 + self.base_coordinates.x,
				x2: x.x2 + self.base_coordinates.x,
				y1: x.y1 + self.base_coordinates.y,
				y2: x.y2 + self.base_coordinates.y,
				z1: x.z1 + self.base_coordinates.z,
				z2: x.z2 + self.base_coordinates.z,
			})
			.collect();

		let other_cuboids_position_adjusted: Vec<Cuboid> = other
			.cuboids
			.iter()
			.map(|x| Cuboid {
				x1: x.x1 + other.base_coordinates.x,
				x2: x.x2 + other.base_coordinates.x,
				y1: x.y1 + other.base_coordinates.y,
				y2: x.y2 + other.base_coordinates.y,
				z1: x.z1 + other.base_coordinates.z,
				z2: x.z2 + other.base_coordinates.z,
			})
			.collect();

		for cuboid in &self_cuboids_position_adjusted {
			for other_cuboid in &other_cuboids_position_adjusted {
				if cuboid.intersects_with_other(other_cuboid) {
					return true;
				}
			}
		}

		return false;
	}
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Cuboid {
	pub x1: f64,
	pub y1: f64,
	pub z1: f64,
	pub x2: f64,
	pub y2: f64,
	pub z2: f64,
}

impl Cuboid {
	pub fn intersects_with_other(&self, other: &Cuboid) -> bool {
		assert!(self.x1 < self.x2 && self.y1 < self.y2 && self.z1 < self.z2);
		assert!(other.x1 < other.x2 && other.y1 < other.y2 && other.z1 < other.z2);

		let mut x_intersects = false;
		if (self.x2 - self.x1) > (other.x2 - other.x1) {
			if (self.x1 <= other.x1 && self.x2 >= other.x1) || (self.x1 <= other.x2 && self.x2 >= other.x2) {
				x_intersects = true;
			}
		} else {
			if (other.x1 <= self.x1 && other.x2 >= self.x1) || (other.x1 <= self.x2 && other.x2 >= self.x2) {
				x_intersects = true;
			}
		}

		let mut y_intersects = false;
		if (self.y2 - self.y1) > (other.y2 - other.y1) {
			if (self.y1 <= other.y1 && self.y2 >= other.y1) || (self.y1 <= other.y2 && self.y2 >= other.y2) {
				y_intersects = true;
			}
		} else {
			if (other.y1 <= self.y1 && other.y2 >= self.y1) || (other.y1 <= self.y2 && other.y2 >= self.y2) {
				y_intersects = true;
			}
		}

		let mut z_intersects = false;
		if (self.z2 - self.z1) > (other.z2 - other.z1) {
			if (self.z1 <= other.z1 && self.z2 >= other.z1) || (self.z1 <= other.z2 && self.z2 >= other.z2) {
				z_intersects = true;
			}
		} else {
			if (other.z1 <= self.z1 && other.z2 >= self.z1) || (other.z1 <= self.z2 && other.z2 >= self.z2) {
				z_intersects = true;
			}
		}

		return x_intersects && y_intersects && z_intersects;
	}
}


#[rustfmt::skip]
#[cfg(test)]
mod tests {
	use super::*;

	mod collision_shape_collides_with {
		use super::*;

		#[test]
		fn one_each_collide_at_zero_coords() {
			let one = CollisionShape::new_from_cuboid(
				Cuboid { x1: 0.0,	y1: 0.0, z1: 0.0,	x2: 1.0, y2: 1.0,	z2: 1.0 },
				EntityPosition::default(),
			);

			let other = CollisionShape::new_from_cuboid(
				Cuboid { x1: 0.5,	y1: 0.0, z1: 0.0, x2: 1.5, y2: 1.0, z2: 1.0 },
				EntityPosition::default(),
			);

			assert!(one.collides_with(&other));
		}

		#[test]
		fn two_each_collide_at_zero_coords() {
			let mut one = CollisionShape::new_from_cuboid(
				Cuboid { x1: 0.0, y1: 0.0,	z1: 0.0, x2: 1.0,	y2: 1.0, z2: 1.0 },
				EntityPosition::default(),
			);
			one.add_cuboid(Cuboid { x1: 0.0,	y1: 1.0, z1: 0.0,	x2: 1.5, y2: 2.0,	z2: 1.5 });

			let mut other = CollisionShape::new_from_cuboid(
				Cuboid { x1: 1.5,	y1: 0.0, z1: 0.0,	x2: 2.5, y2: 1.0,	z2: 1.0 },
				EntityPosition::default(),
			);
			other.add_cuboid(Cuboid {	x1: 1.0, y1: 1.0,	z1: 1.5, x2: 2.5,	y2: 2.0, z2: 2.0	});

			assert!(one.collides_with(&other));
		}

		#[test]
		fn two_each_collide_at_nonzero_coords() {
			let mut one = CollisionShape::new_from_cuboid(
				Cuboid { x1: 0.0, y1: 0.0,	z1: 0.0, x2: 1.0,	y2: 1.0, z2: 1.0 },
				EntityPosition { x: 1.0, y: 1.0, z: 1.0, ..Default::default() },
			);
			one.add_cuboid(Cuboid { x1: 0.0,	y1: 1.0, z1: 0.0,	x2: 1.5, y2: 2.0,	z2: 1.5 });

			let mut other = CollisionShape::new_from_cuboid(
				Cuboid { x1: 1.5,	y1: 0.0, z1: 0.0,	x2: 2.5, y2: 1.0,	z2: 1.0 },
				EntityPosition { x: -1.0, y: -1.0, z: -1.0, ..Default::default() },
			);
			other.add_cuboid(Cuboid {	x1: 1.0, y1: 1.0,	z1: 1.5, x2: 2.5,	y2: 2.0, z2: 2.0	});

			assert!(one.collides_with(&other));
		}

		#[test]
		fn two_each_dont_collide_at_zero_coords() {
			let mut one = CollisionShape::new_from_cuboid(
				Cuboid { x1: 0.0, y1: 0.0,	z1: 0.0, x2: 1.0,	y2: 1.0, z2: 1.0 },
				EntityPosition::default(),
			);
			one.add_cuboid(Cuboid { x1: 0.0,	y1: 1.0, z1: 0.0,	x2: 1.5, y2: 2.0,	z2: 1.5 });

			let mut other = CollisionShape::new_from_cuboid(
				Cuboid { x1: 10.5, y1: 10.0, z1: 10.0, x2: 12.5, y2: 11.0, z2: 11.0 },
				EntityPosition::default(),
			);
			other.add_cuboid(Cuboid {	x1: 10.0, y1: 10.0,	z1: 10.5, x2: 12.5,	y2: 12.0, z2: 12.0 });

			assert!(!one.collides_with(&other));
		}

		#[test]
		fn two_each_dont_collide_at_nonzero_coords() {
			let mut one = CollisionShape::new_from_cuboid(
				Cuboid { x1: 0.0, y1: 0.0,	z1: 0.0, x2: 1.0,	y2: 1.0, z2: 1.0 },
				EntityPosition { x: 1.0, y: 1.0, z: 1.0, ..Default::default() },
			);
			one.add_cuboid(Cuboid { x1: 0.0,	y1: 1.0, z1: 0.0,	x2: 1.5, y2: 2.0,	z2: 1.5 });

			let mut other = CollisionShape::new_from_cuboid(
				Cuboid { x1: 1.5,	y1: 0.0, z1: 0.0,	x2: 2.5, y2: 1.0,	z2: 1.0 },
				EntityPosition { x: -10.0, y: -1.0, z: -1.0, ..Default::default() },
			);
			other.add_cuboid(Cuboid {	x1: 1.0, y1: 1.0,	z1: 1.5, x2: 2.5,	y2: 2.0, z2: 2.0	});

			assert!(!one.collides_with(&other));
		}

		#[test]
		fn pig_doesnt_collide_with_slab() {
			let pig = CollisionShape::new_from_cuboid(
			  Cuboid { x1: -0.45, y1: 0.0, z1: -0.45, x2: 0.45, y2: 0.9, z2: 0.45 },
				EntityPosition { x: -25.65522739210799, y: 69.9, z: 42.476654394084896, yaw: -143.42871, pitch: 90.0 },
			);

			let slab = CollisionShape::new_from_cuboid(
			  Cuboid { x1: 0.0, y1: 0.0, z1: 0.0, x2: 1.0, y2: 0.5, z2: 1.0 },
				EntityPosition { x: -26.0, y: 69.0, z: 42.0, yaw: 0.0, pitch: 0.0 },
			);

			assert!(!pig.collides_with(&slab));
		}
	}

	mod cuboid_intersects_with_other_x {
		use super::*;

		#[test]
		fn intersecting_positive() {
			let cuboid1 = Cuboid { x1: 0.0, y1: 0.0, z1: 0.0, x2: 1.0, y2: 1.0,	z2: 1.0 };
			let cuboid2 = Cuboid { x1: 0.5,	y1: 0.5, z1: 0.5,	x2: 1.5, y2: 1.5,	z2: 1.5	};
			assert!(cuboid1.intersects_with_other(&cuboid2));
		}

		#[test]
		fn intersecting_negative() {
		  let cuboid1 = Cuboid { x1: 0.0, y1: 0.0, z1: 0.0, x2: 1.0, y2: 1.0,	z2: 1.0 };
			let cuboid2 = Cuboid { x1: -0.5, y1: -0.5, z1: -0.5, x2: 0.5,	y2: 0.5, z2: 0.5 };
			assert!(cuboid1.intersects_with_other(&cuboid2));
		}

		#[test]
		fn intersecting_smaller() {
		  let cuboid1 = Cuboid { x1: 0.0, y1: 0.0, z1: 0.0, x2: 1.0, y2: 1.0,	z2: 1.0 };
			let cuboid2 = Cuboid { x1: 0.25, y1: 0.25, z1: 0.25, x2: 0.5, y2: 0.5, z2: 0.5 };
			assert!(cuboid1.intersects_with_other(&cuboid2));
		}

		#[test]
		fn intersecting_bigger() {
		  let cuboid1 = Cuboid { x1: 0.0, y1: 0.0, z1: 0.0, x2: 1.0, y2: 1.0,	z2: 1.0 };
			let cuboid2 = Cuboid { x1: -0.25, y1: -0.25, z1: -0.25, x2: 1.5, y2: 1.5, z2: 1.5 };
			assert!(cuboid1.intersects_with_other(&cuboid2));
		}

		#[test]
		fn not_intersecting() {
		  let cuboid1 = Cuboid { x1: 0.0, y1: 0.0, z1: 0.0, x2: 1.0, y2: 1.0,	z2: 1.0 };
			let cuboid2 = Cuboid { x1: 2.0,	y1: 2.0, z1: 2.0,	x2: 2.5, y2: 2.5,	z2: 2.5	};
			assert!(!cuboid1.intersects_with_other(&cuboid2));
		}

		#[test]
		fn not_intersecting_touching() {
		  let cuboid1 = Cuboid { x1: 0.0, y1: 0.0, z1: 0.0, x2: 1.0, y2: 1.0,	z2: 1.0 };
			let cuboid2 = Cuboid { x1: 1.0,	y1: 1.5, z1: 1.5, x2: 2.0, y2: 2.5,	z2: 2.5 };
			assert!(!cuboid1.intersects_with_other(&cuboid2));
		}
	}
}
