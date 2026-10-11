use super::*;

pub mod acacia;
pub mod birch;
pub mod dark_oak;
pub mod jungle;
pub mod oak;
pub mod spruce;


fn gen_tree(height: i16, log_id: u16, leaves_id: u16) -> Vec<(BlockPosition, u16)> {
	let stem: Vec<(BlockPosition, u16)> = (0..height).into_iter().map(|y| (BlockPosition { y, ..Default::default() }, log_id)).collect();
	let crown = vec![
		(BlockPosition { y: height - 2, x: -2, z: -2 }, leaves_id),
		(BlockPosition { y: height - 2, x: -1, z: -2 }, leaves_id),
		(BlockPosition { y: height - 2, x: 0, z: -2 }, leaves_id),
		(BlockPosition { y: height - 2, x: 1, z: -2 }, leaves_id),
		(BlockPosition { y: height - 2, x: 2, z: -2 }, leaves_id),
		(BlockPosition { y: height - 2, x: -2, z: -1 }, leaves_id),
		(BlockPosition { y: height - 2, x: -1, z: -1 }, leaves_id),
		(BlockPosition { y: height - 2, x: 0, z: -1 }, leaves_id),
		(BlockPosition { y: height - 2, x: 1, z: -1 }, leaves_id),
		(BlockPosition { y: height - 2, x: 2, z: -1 }, leaves_id),
		(BlockPosition { y: height - 2, x: -2, z: 0 }, leaves_id),
		(BlockPosition { y: height - 2, x: -1, z: 0 }, leaves_id),
		(BlockPosition { y: height - 2, x: 1, z: 0 }, leaves_id),
		(BlockPosition { y: height - 2, x: 2, z: 0 }, leaves_id),
		(BlockPosition { y: height - 2, x: -2, z: 1 }, leaves_id),
		(BlockPosition { y: height - 2, x: -1, z: 1 }, leaves_id),
		(BlockPosition { y: height - 2, x: 0, z: 1 }, leaves_id),
		(BlockPosition { y: height - 2, x: 1, z: 1 }, leaves_id),
		(BlockPosition { y: height - 2, x: 2, z: 1 }, leaves_id),
		(BlockPosition { y: height - 2, x: -2, z: 2 }, leaves_id),
		(BlockPosition { y: height - 2, x: -1, z: 2 }, leaves_id),
		(BlockPosition { y: height - 2, x: 0, z: 2 }, leaves_id),
		(BlockPosition { y: height - 2, x: 1, z: 2 }, leaves_id),
		(BlockPosition { y: height - 2, x: 2, z: 2 }, leaves_id),
		(BlockPosition { y: height - 1, x: -2, z: -2 }, leaves_id),
		(BlockPosition { y: height - 1, x: -1, z: -2 }, leaves_id),
		(BlockPosition { y: height - 1, x: 0, z: -2 }, leaves_id),
		(BlockPosition { y: height - 1, x: 1, z: -2 }, leaves_id),
		(BlockPosition { y: height - 1, x: 2, z: -2 }, leaves_id),
		(BlockPosition { y: height - 1, x: -2, z: -1 }, leaves_id),
		(BlockPosition { y: height - 1, x: -1, z: -1 }, leaves_id),
		(BlockPosition { y: height - 1, x: 0, z: -1 }, leaves_id),
		(BlockPosition { y: height - 1, x: 1, z: -1 }, leaves_id),
		(BlockPosition { y: height - 1, x: 2, z: -1 }, leaves_id),
		(BlockPosition { y: height - 1, x: -2, z: 0 }, leaves_id),
		(BlockPosition { y: height - 1, x: -1, z: 0 }, leaves_id),
		(BlockPosition { y: height - 1, x: 1, z: 0 }, leaves_id),
		(BlockPosition { y: height - 1, x: 2, z: 0 }, leaves_id),
		(BlockPosition { y: height - 1, x: -2, z: 1 }, leaves_id),
		(BlockPosition { y: height - 1, x: -1, z: 1 }, leaves_id),
		(BlockPosition { y: height - 1, x: 0, z: 1 }, leaves_id),
		(BlockPosition { y: height - 1, x: 1, z: 1 }, leaves_id),
		(BlockPosition { y: height - 1, x: 2, z: 1 }, leaves_id),
		(BlockPosition { y: height - 1, x: -2, z: 2 }, leaves_id),
		(BlockPosition { y: height - 1, x: -1, z: 2 }, leaves_id),
		(BlockPosition { y: height - 1, x: 0, z: 2 }, leaves_id),
		(BlockPosition { y: height - 1, x: 1, z: 2 }, leaves_id),
		(BlockPosition { y: height - 1, x: 2, z: 2 }, leaves_id),
		(BlockPosition { y: height, x: -1, z: -1 }, leaves_id),
		(BlockPosition { y: height, x: 0, z: -1 }, leaves_id),
		(BlockPosition { y: height, x: 1, z: -1 }, leaves_id),
		(BlockPosition { y: height, x: -1, z: 0 }, leaves_id),
		(BlockPosition { y: height, x: 0, z: 0 }, leaves_id),
		(BlockPosition { y: height, x: 1, z: 0 }, leaves_id),
		(BlockPosition { y: height, x: -1, z: 1 }, leaves_id),
		(BlockPosition { y: height, x: 0, z: 1 }, leaves_id),
		(BlockPosition { y: height, x: 1, z: 1 }, leaves_id),
		(BlockPosition { y: height + 1, x: 0, z: 0 }, leaves_id),
	];

	return vec![stem, crown].into_iter().flatten().collect();
}
