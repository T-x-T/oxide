#![allow(unused_mut)]
#![allow(clippy::needless_return)]
use std::collections::HashMap;
pub use block_types::*;
pub use block_get_blocks::*;
pub use block_get_type_from_block_state_id::*;
pub fn get_block_from_block_state_id(block_state_id: u16, block_states: &HashMap<String, Block>) -> Block {
	return block_states.iter().find(|x| x.1.states.iter().any(|y| y.id == block_state_id)).unwrap().1.clone();
}
pub fn get_block_state_from_block_state_id(block_state_id: u16, block_states: &HashMap<String, Block>) -> State {
	return block_states.iter()
		.filter(|x| x.1.states.iter().any(|y| y.id == block_state_id))
		.map(|x| x.1.states.iter().find(|y| y.id == block_state_id).unwrap())
		.collect::<Vec<&State>>().first_mut().unwrap().clone();
}
pub fn get_block_name_from_block_state_id(block_state_id: u16, block_states: &HashMap<String, Block>) -> String {
	return block_states.iter().find(|x| x.1.states.iter().any(|y| y.id == block_state_id)).unwrap().0.clone();
}
 pub fn get_block_from_name(name: &str, block_states: &HashMap<String, Block>) -> Block {
	let air = block_states.get("minecraft:air").unwrap();
	let block = block_states.get(name).unwrap_or(air);
	return block.clone();
}
pub fn get_raw_properties_from_block_state_id(block_states: &HashMap<String, Block>, block_state_id: u16) -> Vec<(String, String)> {
	let state = block_states.iter().find(|x| x.1.states.iter().any(|x| x.id == block_state_id)).unwrap().1.states.iter().find(|x| x.id == block_state_id).unwrap().clone();
	let mut output: Vec<(String, String)> = Vec::new();

	for property in state.properties {
		output.push(get_raw_properties(property));
	}
	return output;
}
pub fn get_raw_properties(property: Property) -> (String, String) {
	return match property {
		Property::AmethystClusterFacing(AmethystClusterFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::AmethystClusterFacing(AmethystClusterFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::AmethystClusterFacing(AmethystClusterFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::AmethystClusterFacing(AmethystClusterFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::AmethystClusterFacing(AmethystClusterFacing::Up) => ("facing".to_string(), "up".to_string()),
		Property::AmethystClusterFacing(AmethystClusterFacing::Down) => ("facing".to_string(), "down".to_string()),
		Property::AmethystClusterWaterlogged(AmethystClusterWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::AmethystClusterWaterlogged(AmethystClusterWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::AnvilFacing(AnvilFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::AnvilFacing(AnvilFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::AnvilFacing(AnvilFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::AnvilFacing(AnvilFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::AttachedStemFacing(AttachedStemFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::AttachedStemFacing(AttachedStemFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::AttachedStemFacing(AttachedStemFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::AttachedStemFacing(AttachedStemFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::BambooStalkAge(BambooStalkAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::BambooStalkAge(BambooStalkAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::BambooStalkLeaves(BambooStalkLeaves::None) => ("leaves".to_string(), "none".to_string()),
		Property::BambooStalkLeaves(BambooStalkLeaves::Small) => ("leaves".to_string(), "small".to_string()),
		Property::BambooStalkLeaves(BambooStalkLeaves::Large) => ("leaves".to_string(), "large".to_string()),
		Property::BambooStalkStage(BambooStalkStage::Num0) => ("stage".to_string(), "0".to_string()),
		Property::BambooStalkStage(BambooStalkStage::Num1) => ("stage".to_string(), "1".to_string()),
		Property::BannerRotation(BannerRotation::Num0) => ("rotation".to_string(), "0".to_string()),
		Property::BannerRotation(BannerRotation::Num1) => ("rotation".to_string(), "1".to_string()),
		Property::BannerRotation(BannerRotation::Num2) => ("rotation".to_string(), "2".to_string()),
		Property::BannerRotation(BannerRotation::Num3) => ("rotation".to_string(), "3".to_string()),
		Property::BannerRotation(BannerRotation::Num4) => ("rotation".to_string(), "4".to_string()),
		Property::BannerRotation(BannerRotation::Num5) => ("rotation".to_string(), "5".to_string()),
		Property::BannerRotation(BannerRotation::Num6) => ("rotation".to_string(), "6".to_string()),
		Property::BannerRotation(BannerRotation::Num7) => ("rotation".to_string(), "7".to_string()),
		Property::BannerRotation(BannerRotation::Num8) => ("rotation".to_string(), "8".to_string()),
		Property::BannerRotation(BannerRotation::Num9) => ("rotation".to_string(), "9".to_string()),
		Property::BannerRotation(BannerRotation::Num10) => ("rotation".to_string(), "10".to_string()),
		Property::BannerRotation(BannerRotation::Num11) => ("rotation".to_string(), "11".to_string()),
		Property::BannerRotation(BannerRotation::Num12) => ("rotation".to_string(), "12".to_string()),
		Property::BannerRotation(BannerRotation::Num13) => ("rotation".to_string(), "13".to_string()),
		Property::BannerRotation(BannerRotation::Num14) => ("rotation".to_string(), "14".to_string()),
		Property::BannerRotation(BannerRotation::Num15) => ("rotation".to_string(), "15".to_string()),
		Property::BarrelFacing(BarrelFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::BarrelFacing(BarrelFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::BarrelFacing(BarrelFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::BarrelFacing(BarrelFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::BarrelFacing(BarrelFacing::Up) => ("facing".to_string(), "up".to_string()),
		Property::BarrelFacing(BarrelFacing::Down) => ("facing".to_string(), "down".to_string()),
		Property::BarrelOpen(BarrelOpen::True) => ("open".to_string(), "true".to_string()),
		Property::BarrelOpen(BarrelOpen::False) => ("open".to_string(), "false".to_string()),
		Property::BarrierWaterlogged(BarrierWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::BarrierWaterlogged(BarrierWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::BaseCoralFanWaterlogged(BaseCoralFanWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::BaseCoralFanWaterlogged(BaseCoralFanWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::BaseCoralPlantWaterlogged(BaseCoralPlantWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::BaseCoralPlantWaterlogged(BaseCoralPlantWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::BaseCoralWallFanFacing(BaseCoralWallFanFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::BaseCoralWallFanFacing(BaseCoralWallFanFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::BaseCoralWallFanFacing(BaseCoralWallFanFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::BaseCoralWallFanFacing(BaseCoralWallFanFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::BaseCoralWallFanWaterlogged(BaseCoralWallFanWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::BaseCoralWallFanWaterlogged(BaseCoralWallFanWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::BedFacing(BedFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::BedFacing(BedFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::BedFacing(BedFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::BedFacing(BedFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::BedOccupied(BedOccupied::True) => ("occupied".to_string(), "true".to_string()),
		Property::BedOccupied(BedOccupied::False) => ("occupied".to_string(), "false".to_string()),
		Property::BedPart(BedPart::Head) => ("part".to_string(), "head".to_string()),
		Property::BedPart(BedPart::Foot) => ("part".to_string(), "foot".to_string()),
		Property::BeehiveFacing(BeehiveFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::BeehiveFacing(BeehiveFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::BeehiveFacing(BeehiveFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::BeehiveFacing(BeehiveFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num0) => ("honey_level".to_string(), "0".to_string()),
		Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num1) => ("honey_level".to_string(), "1".to_string()),
		Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num2) => ("honey_level".to_string(), "2".to_string()),
		Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num3) => ("honey_level".to_string(), "3".to_string()),
		Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num4) => ("honey_level".to_string(), "4".to_string()),
		Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num5) => ("honey_level".to_string(), "5".to_string()),
		Property::BeetrootAge(BeetrootAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::BeetrootAge(BeetrootAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::BeetrootAge(BeetrootAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::BeetrootAge(BeetrootAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::BellAttachment(BellAttachment::Floor) => ("attachment".to_string(), "floor".to_string()),
		Property::BellAttachment(BellAttachment::Ceiling) => ("attachment".to_string(), "ceiling".to_string()),
		Property::BellAttachment(BellAttachment::SingleWall) => ("attachment".to_string(), "single_wall".to_string()),
		Property::BellAttachment(BellAttachment::DoubleWall) => ("attachment".to_string(), "double_wall".to_string()),
		Property::BellFacing(BellFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::BellFacing(BellFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::BellFacing(BellFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::BellFacing(BellFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::BellPowered(BellPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::BellPowered(BellPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::BigDripleafFacing(BigDripleafFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::BigDripleafFacing(BigDripleafFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::BigDripleafFacing(BigDripleafFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::BigDripleafFacing(BigDripleafFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::BigDripleafTilt(BigDripleafTilt::None) => ("tilt".to_string(), "none".to_string()),
		Property::BigDripleafTilt(BigDripleafTilt::Unstable) => ("tilt".to_string(), "unstable".to_string()),
		Property::BigDripleafTilt(BigDripleafTilt::Partial) => ("tilt".to_string(), "partial".to_string()),
		Property::BigDripleafTilt(BigDripleafTilt::Full) => ("tilt".to_string(), "full".to_string()),
		Property::BigDripleafWaterlogged(BigDripleafWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::BigDripleafWaterlogged(BigDripleafWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::BigDripleafStemFacing(BigDripleafStemFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::BigDripleafStemFacing(BigDripleafStemFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::BigDripleafStemFacing(BigDripleafStemFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::BigDripleafStemFacing(BigDripleafStemFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::BigDripleafStemWaterlogged(BigDripleafStemWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::BigDripleafStemWaterlogged(BigDripleafStemWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::BlastFurnaceFacing(BlastFurnaceFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::BlastFurnaceFacing(BlastFurnaceFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::BlastFurnaceFacing(BlastFurnaceFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::BlastFurnaceFacing(BlastFurnaceFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::BlastFurnaceLit(BlastFurnaceLit::True) => ("lit".to_string(), "true".to_string()),
		Property::BlastFurnaceLit(BlastFurnaceLit::False) => ("lit".to_string(), "false".to_string()),
		Property::BrewingStandHasBottle0(BrewingStandHasBottle0::True) => ("has_bottle_0".to_string(), "true".to_string()),
		Property::BrewingStandHasBottle0(BrewingStandHasBottle0::False) => ("has_bottle_0".to_string(), "false".to_string()),
		Property::BrewingStandHasBottle1(BrewingStandHasBottle1::True) => ("has_bottle_1".to_string(), "true".to_string()),
		Property::BrewingStandHasBottle1(BrewingStandHasBottle1::False) => ("has_bottle_1".to_string(), "false".to_string()),
		Property::BrewingStandHasBottle2(BrewingStandHasBottle2::True) => ("has_bottle_2".to_string(), "true".to_string()),
		Property::BrewingStandHasBottle2(BrewingStandHasBottle2::False) => ("has_bottle_2".to_string(), "false".to_string()),
		Property::BrushableDusted(BrushableDusted::Num0) => ("dusted".to_string(), "0".to_string()),
		Property::BrushableDusted(BrushableDusted::Num1) => ("dusted".to_string(), "1".to_string()),
		Property::BrushableDusted(BrushableDusted::Num2) => ("dusted".to_string(), "2".to_string()),
		Property::BrushableDusted(BrushableDusted::Num3) => ("dusted".to_string(), "3".to_string()),
		Property::BubbleColumnDrag(BubbleColumnDrag::True) => ("drag".to_string(), "true".to_string()),
		Property::BubbleColumnDrag(BubbleColumnDrag::False) => ("drag".to_string(), "false".to_string()),
		Property::ButtonFace(ButtonFace::Floor) => ("face".to_string(), "floor".to_string()),
		Property::ButtonFace(ButtonFace::Wall) => ("face".to_string(), "wall".to_string()),
		Property::ButtonFace(ButtonFace::Ceiling) => ("face".to_string(), "ceiling".to_string()),
		Property::ButtonFacing(ButtonFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::ButtonFacing(ButtonFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::ButtonFacing(ButtonFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::ButtonFacing(ButtonFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::ButtonPowered(ButtonPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::ButtonPowered(ButtonPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::CactusAge(CactusAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::CactusAge(CactusAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::CactusAge(CactusAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::CactusAge(CactusAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::CactusAge(CactusAge::Num4) => ("age".to_string(), "4".to_string()),
		Property::CactusAge(CactusAge::Num5) => ("age".to_string(), "5".to_string()),
		Property::CactusAge(CactusAge::Num6) => ("age".to_string(), "6".to_string()),
		Property::CactusAge(CactusAge::Num7) => ("age".to_string(), "7".to_string()),
		Property::CactusAge(CactusAge::Num8) => ("age".to_string(), "8".to_string()),
		Property::CactusAge(CactusAge::Num9) => ("age".to_string(), "9".to_string()),
		Property::CactusAge(CactusAge::Num10) => ("age".to_string(), "10".to_string()),
		Property::CactusAge(CactusAge::Num11) => ("age".to_string(), "11".to_string()),
		Property::CactusAge(CactusAge::Num12) => ("age".to_string(), "12".to_string()),
		Property::CactusAge(CactusAge::Num13) => ("age".to_string(), "13".to_string()),
		Property::CactusAge(CactusAge::Num14) => ("age".to_string(), "14".to_string()),
		Property::CactusAge(CactusAge::Num15) => ("age".to_string(), "15".to_string()),
		Property::CakeBites(CakeBites::Num0) => ("bites".to_string(), "0".to_string()),
		Property::CakeBites(CakeBites::Num1) => ("bites".to_string(), "1".to_string()),
		Property::CakeBites(CakeBites::Num2) => ("bites".to_string(), "2".to_string()),
		Property::CakeBites(CakeBites::Num3) => ("bites".to_string(), "3".to_string()),
		Property::CakeBites(CakeBites::Num4) => ("bites".to_string(), "4".to_string()),
		Property::CakeBites(CakeBites::Num5) => ("bites".to_string(), "5".to_string()),
		Property::CakeBites(CakeBites::Num6) => ("bites".to_string(), "6".to_string()),
		Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num0) => ("power".to_string(), "0".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num1) => ("power".to_string(), "1".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num2) => ("power".to_string(), "2".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num3) => ("power".to_string(), "3".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num4) => ("power".to_string(), "4".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num5) => ("power".to_string(), "5".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num6) => ("power".to_string(), "6".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num7) => ("power".to_string(), "7".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num8) => ("power".to_string(), "8".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num9) => ("power".to_string(), "9".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num10) => ("power".to_string(), "10".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num11) => ("power".to_string(), "11".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num12) => ("power".to_string(), "12".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num13) => ("power".to_string(), "13".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num14) => ("power".to_string(), "14".to_string()),
		Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num15) => ("power".to_string(), "15".to_string()),
		Property::CalibratedSculkSensorSculkSensorPhase(CalibratedSculkSensorSculkSensorPhase::Inactive) => ("sculk_sensor_phase".to_string(), "inactive".to_string()),
		Property::CalibratedSculkSensorSculkSensorPhase(CalibratedSculkSensorSculkSensorPhase::Active) => ("sculk_sensor_phase".to_string(), "active".to_string()),
		Property::CalibratedSculkSensorSculkSensorPhase(CalibratedSculkSensorSculkSensorPhase::Cooldown) => ("sculk_sensor_phase".to_string(), "cooldown".to_string()),
		Property::CalibratedSculkSensorWaterlogged(CalibratedSculkSensorWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::CalibratedSculkSensorWaterlogged(CalibratedSculkSensorWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::CampfireFacing(CampfireFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::CampfireFacing(CampfireFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::CampfireFacing(CampfireFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::CampfireFacing(CampfireFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::CampfireLit(CampfireLit::True) => ("lit".to_string(), "true".to_string()),
		Property::CampfireLit(CampfireLit::False) => ("lit".to_string(), "false".to_string()),
		Property::CampfireSignalFire(CampfireSignalFire::True) => ("signal_fire".to_string(), "true".to_string()),
		Property::CampfireSignalFire(CampfireSignalFire::False) => ("signal_fire".to_string(), "false".to_string()),
		Property::CampfireWaterlogged(CampfireWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::CampfireWaterlogged(CampfireWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::CandleCandles(CandleCandles::Num1) => ("candles".to_string(), "1".to_string()),
		Property::CandleCandles(CandleCandles::Num2) => ("candles".to_string(), "2".to_string()),
		Property::CandleCandles(CandleCandles::Num3) => ("candles".to_string(), "3".to_string()),
		Property::CandleCandles(CandleCandles::Num4) => ("candles".to_string(), "4".to_string()),
		Property::CandleLit(CandleLit::True) => ("lit".to_string(), "true".to_string()),
		Property::CandleLit(CandleLit::False) => ("lit".to_string(), "false".to_string()),
		Property::CandleWaterlogged(CandleWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::CandleWaterlogged(CandleWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::CandleCakeLit(CandleCakeLit::True) => ("lit".to_string(), "true".to_string()),
		Property::CandleCakeLit(CandleCakeLit::False) => ("lit".to_string(), "false".to_string()),
		Property::CarrotAge(CarrotAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::CarrotAge(CarrotAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::CarrotAge(CarrotAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::CarrotAge(CarrotAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::CarrotAge(CarrotAge::Num4) => ("age".to_string(), "4".to_string()),
		Property::CarrotAge(CarrotAge::Num5) => ("age".to_string(), "5".to_string()),
		Property::CarrotAge(CarrotAge::Num6) => ("age".to_string(), "6".to_string()),
		Property::CarrotAge(CarrotAge::Num7) => ("age".to_string(), "7".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num4) => ("age".to_string(), "4".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num5) => ("age".to_string(), "5".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num6) => ("age".to_string(), "6".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num7) => ("age".to_string(), "7".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num8) => ("age".to_string(), "8".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num9) => ("age".to_string(), "9".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num10) => ("age".to_string(), "10".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num11) => ("age".to_string(), "11".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num12) => ("age".to_string(), "12".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num13) => ("age".to_string(), "13".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num14) => ("age".to_string(), "14".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num15) => ("age".to_string(), "15".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num16) => ("age".to_string(), "16".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num17) => ("age".to_string(), "17".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num18) => ("age".to_string(), "18".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num19) => ("age".to_string(), "19".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num20) => ("age".to_string(), "20".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num21) => ("age".to_string(), "21".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num22) => ("age".to_string(), "22".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num23) => ("age".to_string(), "23".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num24) => ("age".to_string(), "24".to_string()),
		Property::CaveVinesAge(CaveVinesAge::Num25) => ("age".to_string(), "25".to_string()),
		Property::CaveVinesBerries(CaveVinesBerries::True) => ("berries".to_string(), "true".to_string()),
		Property::CaveVinesBerries(CaveVinesBerries::False) => ("berries".to_string(), "false".to_string()),
		Property::CaveVinesPlantBerries(CaveVinesPlantBerries::True) => ("berries".to_string(), "true".to_string()),
		Property::CaveVinesPlantBerries(CaveVinesPlantBerries::False) => ("berries".to_string(), "false".to_string()),
		Property::CeilingHangingSignAttached(CeilingHangingSignAttached::True) => ("attached".to_string(), "true".to_string()),
		Property::CeilingHangingSignAttached(CeilingHangingSignAttached::False) => ("attached".to_string(), "false".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num0) => ("rotation".to_string(), "0".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num1) => ("rotation".to_string(), "1".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num2) => ("rotation".to_string(), "2".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num3) => ("rotation".to_string(), "3".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num4) => ("rotation".to_string(), "4".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num5) => ("rotation".to_string(), "5".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num6) => ("rotation".to_string(), "6".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num7) => ("rotation".to_string(), "7".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num8) => ("rotation".to_string(), "8".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num9) => ("rotation".to_string(), "9".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num10) => ("rotation".to_string(), "10".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num11) => ("rotation".to_string(), "11".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num12) => ("rotation".to_string(), "12".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num13) => ("rotation".to_string(), "13".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num14) => ("rotation".to_string(), "14".to_string()),
		Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num15) => ("rotation".to_string(), "15".to_string()),
		Property::CeilingHangingSignWaterlogged(CeilingHangingSignWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::CeilingHangingSignWaterlogged(CeilingHangingSignWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::ChainAxis(ChainAxis::X) => ("axis".to_string(), "x".to_string()),
		Property::ChainAxis(ChainAxis::Y) => ("axis".to_string(), "y".to_string()),
		Property::ChainAxis(ChainAxis::Z) => ("axis".to_string(), "z".to_string()),
		Property::ChainWaterlogged(ChainWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::ChainWaterlogged(ChainWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::ChestFacing(ChestFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::ChestFacing(ChestFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::ChestFacing(ChestFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::ChestFacing(ChestFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::ChestType(ChestType::Single) => ("type".to_string(), "single".to_string()),
		Property::ChestType(ChestType::Left) => ("type".to_string(), "left".to_string()),
		Property::ChestType(ChestType::Right) => ("type".to_string(), "right".to_string()),
		Property::ChestWaterlogged(ChestWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::ChestWaterlogged(ChestWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::ChiseledBookShelfSlot0Occupied(ChiseledBookShelfSlot0Occupied::True) => ("slot_0_occupied".to_string(), "true".to_string()),
		Property::ChiseledBookShelfSlot0Occupied(ChiseledBookShelfSlot0Occupied::False) => ("slot_0_occupied".to_string(), "false".to_string()),
		Property::ChiseledBookShelfSlot1Occupied(ChiseledBookShelfSlot1Occupied::True) => ("slot_1_occupied".to_string(), "true".to_string()),
		Property::ChiseledBookShelfSlot1Occupied(ChiseledBookShelfSlot1Occupied::False) => ("slot_1_occupied".to_string(), "false".to_string()),
		Property::ChiseledBookShelfSlot2Occupied(ChiseledBookShelfSlot2Occupied::True) => ("slot_2_occupied".to_string(), "true".to_string()),
		Property::ChiseledBookShelfSlot2Occupied(ChiseledBookShelfSlot2Occupied::False) => ("slot_2_occupied".to_string(), "false".to_string()),
		Property::ChiseledBookShelfSlot3Occupied(ChiseledBookShelfSlot3Occupied::True) => ("slot_3_occupied".to_string(), "true".to_string()),
		Property::ChiseledBookShelfSlot3Occupied(ChiseledBookShelfSlot3Occupied::False) => ("slot_3_occupied".to_string(), "false".to_string()),
		Property::ChiseledBookShelfSlot4Occupied(ChiseledBookShelfSlot4Occupied::True) => ("slot_4_occupied".to_string(), "true".to_string()),
		Property::ChiseledBookShelfSlot4Occupied(ChiseledBookShelfSlot4Occupied::False) => ("slot_4_occupied".to_string(), "false".to_string()),
		Property::ChiseledBookShelfSlot5Occupied(ChiseledBookShelfSlot5Occupied::True) => ("slot_5_occupied".to_string(), "true".to_string()),
		Property::ChiseledBookShelfSlot5Occupied(ChiseledBookShelfSlot5Occupied::False) => ("slot_5_occupied".to_string(), "false".to_string()),
		Property::ChorusFlowerAge(ChorusFlowerAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::ChorusFlowerAge(ChorusFlowerAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::ChorusFlowerAge(ChorusFlowerAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::ChorusFlowerAge(ChorusFlowerAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::ChorusFlowerAge(ChorusFlowerAge::Num4) => ("age".to_string(), "4".to_string()),
		Property::ChorusFlowerAge(ChorusFlowerAge::Num5) => ("age".to_string(), "5".to_string()),
		Property::ChorusPlantDown(ChorusPlantDown::True) => ("down".to_string(), "true".to_string()),
		Property::ChorusPlantDown(ChorusPlantDown::False) => ("down".to_string(), "false".to_string()),
		Property::ChorusPlantEast(ChorusPlantEast::True) => ("east".to_string(), "true".to_string()),
		Property::ChorusPlantEast(ChorusPlantEast::False) => ("east".to_string(), "false".to_string()),
		Property::ChorusPlantNorth(ChorusPlantNorth::True) => ("north".to_string(), "true".to_string()),
		Property::ChorusPlantNorth(ChorusPlantNorth::False) => ("north".to_string(), "false".to_string()),
		Property::ChorusPlantSouth(ChorusPlantSouth::True) => ("south".to_string(), "true".to_string()),
		Property::ChorusPlantSouth(ChorusPlantSouth::False) => ("south".to_string(), "false".to_string()),
		Property::ChorusPlantUp(ChorusPlantUp::True) => ("up".to_string(), "true".to_string()),
		Property::ChorusPlantUp(ChorusPlantUp::False) => ("up".to_string(), "false".to_string()),
		Property::ChorusPlantWest(ChorusPlantWest::True) => ("west".to_string(), "true".to_string()),
		Property::ChorusPlantWest(ChorusPlantWest::False) => ("west".to_string(), "false".to_string()),
		Property::CocoaAge(CocoaAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::CocoaAge(CocoaAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::CocoaAge(CocoaAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::CocoaFacing(CocoaFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::CocoaFacing(CocoaFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::CocoaFacing(CocoaFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::CocoaFacing(CocoaFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::CommandConditional(CommandConditional::True) => ("conditional".to_string(), "true".to_string()),
		Property::CommandConditional(CommandConditional::False) => ("conditional".to_string(), "false".to_string()),
		Property::CommandFacing(CommandFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::CommandFacing(CommandFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::CommandFacing(CommandFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::CommandFacing(CommandFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::CommandFacing(CommandFacing::Up) => ("facing".to_string(), "up".to_string()),
		Property::CommandFacing(CommandFacing::Down) => ("facing".to_string(), "down".to_string()),
		Property::ComparatorFacing(ComparatorFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::ComparatorFacing(ComparatorFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::ComparatorFacing(ComparatorFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::ComparatorFacing(ComparatorFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::ComparatorMode(ComparatorMode::Compare) => ("mode".to_string(), "compare".to_string()),
		Property::ComparatorMode(ComparatorMode::Subtract) => ("mode".to_string(), "subtract".to_string()),
		Property::ComparatorPowered(ComparatorPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::ComparatorPowered(ComparatorPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::ComposterLevel(ComposterLevel::Num0) => ("level".to_string(), "0".to_string()),
		Property::ComposterLevel(ComposterLevel::Num1) => ("level".to_string(), "1".to_string()),
		Property::ComposterLevel(ComposterLevel::Num2) => ("level".to_string(), "2".to_string()),
		Property::ComposterLevel(ComposterLevel::Num3) => ("level".to_string(), "3".to_string()),
		Property::ComposterLevel(ComposterLevel::Num4) => ("level".to_string(), "4".to_string()),
		Property::ComposterLevel(ComposterLevel::Num5) => ("level".to_string(), "5".to_string()),
		Property::ComposterLevel(ComposterLevel::Num6) => ("level".to_string(), "6".to_string()),
		Property::ComposterLevel(ComposterLevel::Num7) => ("level".to_string(), "7".to_string()),
		Property::ComposterLevel(ComposterLevel::Num8) => ("level".to_string(), "8".to_string()),
		Property::ConduitWaterlogged(ConduitWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::ConduitWaterlogged(ConduitWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::CopperBulbBlockLit(CopperBulbBlockLit::True) => ("lit".to_string(), "true".to_string()),
		Property::CopperBulbBlockLit(CopperBulbBlockLit::False) => ("lit".to_string(), "false".to_string()),
		Property::CopperBulbBlockPowered(CopperBulbBlockPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::CopperBulbBlockPowered(CopperBulbBlockPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::CopperChestFacing(CopperChestFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::CopperChestFacing(CopperChestFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::CopperChestFacing(CopperChestFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::CopperChestFacing(CopperChestFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::CopperChestType(CopperChestType::Single) => ("type".to_string(), "single".to_string()),
		Property::CopperChestType(CopperChestType::Left) => ("type".to_string(), "left".to_string()),
		Property::CopperChestType(CopperChestType::Right) => ("type".to_string(), "right".to_string()),
		Property::CopperChestWaterlogged(CopperChestWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::CopperChestWaterlogged(CopperChestWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::CopperGolemStatueCopperGolemPose(CopperGolemStatueCopperGolemPose::Standing) => ("copper_golem_pose".to_string(), "standing".to_string()),
		Property::CopperGolemStatueCopperGolemPose(CopperGolemStatueCopperGolemPose::Sitting) => ("copper_golem_pose".to_string(), "sitting".to_string()),
		Property::CopperGolemStatueCopperGolemPose(CopperGolemStatueCopperGolemPose::Running) => ("copper_golem_pose".to_string(), "running".to_string()),
		Property::CopperGolemStatueCopperGolemPose(CopperGolemStatueCopperGolemPose::Star) => ("copper_golem_pose".to_string(), "star".to_string()),
		Property::CopperGolemStatueFacing(CopperGolemStatueFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::CopperGolemStatueFacing(CopperGolemStatueFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::CopperGolemStatueFacing(CopperGolemStatueFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::CopperGolemStatueFacing(CopperGolemStatueFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::CopperGolemStatueWaterlogged(CopperGolemStatueWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::CopperGolemStatueWaterlogged(CopperGolemStatueWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::CoralFanWaterlogged(CoralFanWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::CoralFanWaterlogged(CoralFanWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::CoralPlantWaterlogged(CoralPlantWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::CoralPlantWaterlogged(CoralPlantWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::CoralWallFanFacing(CoralWallFanFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::CoralWallFanFacing(CoralWallFanFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::CoralWallFanFacing(CoralWallFanFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::CoralWallFanFacing(CoralWallFanFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::CoralWallFanWaterlogged(CoralWallFanWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::CoralWallFanWaterlogged(CoralWallFanWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::CrafterCrafting(CrafterCrafting::True) => ("crafting".to_string(), "true".to_string()),
		Property::CrafterCrafting(CrafterCrafting::False) => ("crafting".to_string(), "false".to_string()),
		Property::CrafterOrientation(CrafterOrientation::DownEast) => ("orientation".to_string(), "down_east".to_string()),
		Property::CrafterOrientation(CrafterOrientation::DownNorth) => ("orientation".to_string(), "down_north".to_string()),
		Property::CrafterOrientation(CrafterOrientation::DownSouth) => ("orientation".to_string(), "down_south".to_string()),
		Property::CrafterOrientation(CrafterOrientation::DownWest) => ("orientation".to_string(), "down_west".to_string()),
		Property::CrafterOrientation(CrafterOrientation::UpEast) => ("orientation".to_string(), "up_east".to_string()),
		Property::CrafterOrientation(CrafterOrientation::UpNorth) => ("orientation".to_string(), "up_north".to_string()),
		Property::CrafterOrientation(CrafterOrientation::UpSouth) => ("orientation".to_string(), "up_south".to_string()),
		Property::CrafterOrientation(CrafterOrientation::UpWest) => ("orientation".to_string(), "up_west".to_string()),
		Property::CrafterOrientation(CrafterOrientation::WestUp) => ("orientation".to_string(), "west_up".to_string()),
		Property::CrafterOrientation(CrafterOrientation::EastUp) => ("orientation".to_string(), "east_up".to_string()),
		Property::CrafterOrientation(CrafterOrientation::NorthUp) => ("orientation".to_string(), "north_up".to_string()),
		Property::CrafterOrientation(CrafterOrientation::SouthUp) => ("orientation".to_string(), "south_up".to_string()),
		Property::CrafterTriggered(CrafterTriggered::True) => ("triggered".to_string(), "true".to_string()),
		Property::CrafterTriggered(CrafterTriggered::False) => ("triggered".to_string(), "false".to_string()),
		Property::CreakingHeartAxis(CreakingHeartAxis::X) => ("axis".to_string(), "x".to_string()),
		Property::CreakingHeartAxis(CreakingHeartAxis::Y) => ("axis".to_string(), "y".to_string()),
		Property::CreakingHeartAxis(CreakingHeartAxis::Z) => ("axis".to_string(), "z".to_string()),
		Property::CreakingHeartCreakingHeartState(CreakingHeartCreakingHeartState::Uprooted) => ("creaking_heart_state".to_string(), "uprooted".to_string()),
		Property::CreakingHeartCreakingHeartState(CreakingHeartCreakingHeartState::Dormant) => ("creaking_heart_state".to_string(), "dormant".to_string()),
		Property::CreakingHeartCreakingHeartState(CreakingHeartCreakingHeartState::Awake) => ("creaking_heart_state".to_string(), "awake".to_string()),
		Property::CreakingHeartNatural(CreakingHeartNatural::True) => ("natural".to_string(), "true".to_string()),
		Property::CreakingHeartNatural(CreakingHeartNatural::False) => ("natural".to_string(), "false".to_string()),
		Property::CropAge(CropAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::CropAge(CropAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::CropAge(CropAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::CropAge(CropAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::CropAge(CropAge::Num4) => ("age".to_string(), "4".to_string()),
		Property::CropAge(CropAge::Num5) => ("age".to_string(), "5".to_string()),
		Property::CropAge(CropAge::Num6) => ("age".to_string(), "6".to_string()),
		Property::CropAge(CropAge::Num7) => ("age".to_string(), "7".to_string()),
		Property::DaylightDetectorInverted(DaylightDetectorInverted::True) => ("inverted".to_string(), "true".to_string()),
		Property::DaylightDetectorInverted(DaylightDetectorInverted::False) => ("inverted".to_string(), "false".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num0) => ("power".to_string(), "0".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num1) => ("power".to_string(), "1".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num2) => ("power".to_string(), "2".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num3) => ("power".to_string(), "3".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num4) => ("power".to_string(), "4".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num5) => ("power".to_string(), "5".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num6) => ("power".to_string(), "6".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num7) => ("power".to_string(), "7".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num8) => ("power".to_string(), "8".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num9) => ("power".to_string(), "9".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num10) => ("power".to_string(), "10".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num11) => ("power".to_string(), "11".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num12) => ("power".to_string(), "12".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num13) => ("power".to_string(), "13".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num14) => ("power".to_string(), "14".to_string()),
		Property::DaylightDetectorPower(DaylightDetectorPower::Num15) => ("power".to_string(), "15".to_string()),
		Property::DecoratedPotCracked(DecoratedPotCracked::True) => ("cracked".to_string(), "true".to_string()),
		Property::DecoratedPotCracked(DecoratedPotCracked::False) => ("cracked".to_string(), "false".to_string()),
		Property::DecoratedPotFacing(DecoratedPotFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::DecoratedPotFacing(DecoratedPotFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::DecoratedPotFacing(DecoratedPotFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::DecoratedPotFacing(DecoratedPotFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::DecoratedPotWaterlogged(DecoratedPotWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::DecoratedPotWaterlogged(DecoratedPotWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::DetectorRailPowered(DetectorRailPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::DetectorRailPowered(DetectorRailPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::DetectorRailShape(DetectorRailShape::NorthSouth) => ("shape".to_string(), "north_south".to_string()),
		Property::DetectorRailShape(DetectorRailShape::EastWest) => ("shape".to_string(), "east_west".to_string()),
		Property::DetectorRailShape(DetectorRailShape::AscendingEast) => ("shape".to_string(), "ascending_east".to_string()),
		Property::DetectorRailShape(DetectorRailShape::AscendingWest) => ("shape".to_string(), "ascending_west".to_string()),
		Property::DetectorRailShape(DetectorRailShape::AscendingNorth) => ("shape".to_string(), "ascending_north".to_string()),
		Property::DetectorRailShape(DetectorRailShape::AscendingSouth) => ("shape".to_string(), "ascending_south".to_string()),
		Property::DetectorRailWaterlogged(DetectorRailWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::DetectorRailWaterlogged(DetectorRailWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::DispenserFacing(DispenserFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::DispenserFacing(DispenserFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::DispenserFacing(DispenserFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::DispenserFacing(DispenserFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::DispenserFacing(DispenserFacing::Up) => ("facing".to_string(), "up".to_string()),
		Property::DispenserFacing(DispenserFacing::Down) => ("facing".to_string(), "down".to_string()),
		Property::DispenserTriggered(DispenserTriggered::True) => ("triggered".to_string(), "true".to_string()),
		Property::DispenserTriggered(DispenserTriggered::False) => ("triggered".to_string(), "false".to_string()),
		Property::DoorFacing(DoorFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::DoorFacing(DoorFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::DoorFacing(DoorFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::DoorFacing(DoorFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::DoorHalf(DoorHalf::Upper) => ("half".to_string(), "upper".to_string()),
		Property::DoorHalf(DoorHalf::Lower) => ("half".to_string(), "lower".to_string()),
		Property::DoorHinge(DoorHinge::Left) => ("hinge".to_string(), "left".to_string()),
		Property::DoorHinge(DoorHinge::Right) => ("hinge".to_string(), "right".to_string()),
		Property::DoorOpen(DoorOpen::True) => ("open".to_string(), "true".to_string()),
		Property::DoorOpen(DoorOpen::False) => ("open".to_string(), "false".to_string()),
		Property::DoorPowered(DoorPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::DoorPowered(DoorPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::DoublePlantHalf(DoublePlantHalf::Upper) => ("half".to_string(), "upper".to_string()),
		Property::DoublePlantHalf(DoublePlantHalf::Lower) => ("half".to_string(), "lower".to_string()),
		Property::DriedGhastFacing(DriedGhastFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::DriedGhastFacing(DriedGhastFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::DriedGhastFacing(DriedGhastFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::DriedGhastFacing(DriedGhastFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::DriedGhastHydration(DriedGhastHydration::Num0) => ("hydration".to_string(), "0".to_string()),
		Property::DriedGhastHydration(DriedGhastHydration::Num1) => ("hydration".to_string(), "1".to_string()),
		Property::DriedGhastHydration(DriedGhastHydration::Num2) => ("hydration".to_string(), "2".to_string()),
		Property::DriedGhastHydration(DriedGhastHydration::Num3) => ("hydration".to_string(), "3".to_string()),
		Property::DriedGhastWaterlogged(DriedGhastWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::DriedGhastWaterlogged(DriedGhastWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::DropperFacing(DropperFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::DropperFacing(DropperFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::DropperFacing(DropperFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::DropperFacing(DropperFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::DropperFacing(DropperFacing::Up) => ("facing".to_string(), "up".to_string()),
		Property::DropperFacing(DropperFacing::Down) => ("facing".to_string(), "down".to_string()),
		Property::DropperTriggered(DropperTriggered::True) => ("triggered".to_string(), "true".to_string()),
		Property::DropperTriggered(DropperTriggered::False) => ("triggered".to_string(), "false".to_string()),
		Property::EndPortalFrameEye(EndPortalFrameEye::True) => ("eye".to_string(), "true".to_string()),
		Property::EndPortalFrameEye(EndPortalFrameEye::False) => ("eye".to_string(), "false".to_string()),
		Property::EndPortalFrameFacing(EndPortalFrameFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::EndPortalFrameFacing(EndPortalFrameFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::EndPortalFrameFacing(EndPortalFrameFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::EndPortalFrameFacing(EndPortalFrameFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::EndRodFacing(EndRodFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::EndRodFacing(EndRodFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::EndRodFacing(EndRodFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::EndRodFacing(EndRodFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::EndRodFacing(EndRodFacing::Up) => ("facing".to_string(), "up".to_string()),
		Property::EndRodFacing(EndRodFacing::Down) => ("facing".to_string(), "down".to_string()),
		Property::EnderChestFacing(EnderChestFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::EnderChestFacing(EnderChestFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::EnderChestFacing(EnderChestFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::EnderChestFacing(EnderChestFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::EnderChestWaterlogged(EnderChestWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::EnderChestWaterlogged(EnderChestWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::FarmMoisture(FarmMoisture::Num0) => ("moisture".to_string(), "0".to_string()),
		Property::FarmMoisture(FarmMoisture::Num1) => ("moisture".to_string(), "1".to_string()),
		Property::FarmMoisture(FarmMoisture::Num2) => ("moisture".to_string(), "2".to_string()),
		Property::FarmMoisture(FarmMoisture::Num3) => ("moisture".to_string(), "3".to_string()),
		Property::FarmMoisture(FarmMoisture::Num4) => ("moisture".to_string(), "4".to_string()),
		Property::FarmMoisture(FarmMoisture::Num5) => ("moisture".to_string(), "5".to_string()),
		Property::FarmMoisture(FarmMoisture::Num6) => ("moisture".to_string(), "6".to_string()),
		Property::FarmMoisture(FarmMoisture::Num7) => ("moisture".to_string(), "7".to_string()),
		Property::FenceEast(FenceEast::True) => ("east".to_string(), "true".to_string()),
		Property::FenceEast(FenceEast::False) => ("east".to_string(), "false".to_string()),
		Property::FenceNorth(FenceNorth::True) => ("north".to_string(), "true".to_string()),
		Property::FenceNorth(FenceNorth::False) => ("north".to_string(), "false".to_string()),
		Property::FenceSouth(FenceSouth::True) => ("south".to_string(), "true".to_string()),
		Property::FenceSouth(FenceSouth::False) => ("south".to_string(), "false".to_string()),
		Property::FenceWaterlogged(FenceWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::FenceWaterlogged(FenceWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::FenceWest(FenceWest::True) => ("west".to_string(), "true".to_string()),
		Property::FenceWest(FenceWest::False) => ("west".to_string(), "false".to_string()),
		Property::FenceGateFacing(FenceGateFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::FenceGateFacing(FenceGateFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::FenceGateFacing(FenceGateFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::FenceGateFacing(FenceGateFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::FenceGateInWall(FenceGateInWall::True) => ("in_wall".to_string(), "true".to_string()),
		Property::FenceGateInWall(FenceGateInWall::False) => ("in_wall".to_string(), "false".to_string()),
		Property::FenceGateOpen(FenceGateOpen::True) => ("open".to_string(), "true".to_string()),
		Property::FenceGateOpen(FenceGateOpen::False) => ("open".to_string(), "false".to_string()),
		Property::FenceGatePowered(FenceGatePowered::True) => ("powered".to_string(), "true".to_string()),
		Property::FenceGatePowered(FenceGatePowered::False) => ("powered".to_string(), "false".to_string()),
		Property::FireAge(FireAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::FireAge(FireAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::FireAge(FireAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::FireAge(FireAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::FireAge(FireAge::Num4) => ("age".to_string(), "4".to_string()),
		Property::FireAge(FireAge::Num5) => ("age".to_string(), "5".to_string()),
		Property::FireAge(FireAge::Num6) => ("age".to_string(), "6".to_string()),
		Property::FireAge(FireAge::Num7) => ("age".to_string(), "7".to_string()),
		Property::FireAge(FireAge::Num8) => ("age".to_string(), "8".to_string()),
		Property::FireAge(FireAge::Num9) => ("age".to_string(), "9".to_string()),
		Property::FireAge(FireAge::Num10) => ("age".to_string(), "10".to_string()),
		Property::FireAge(FireAge::Num11) => ("age".to_string(), "11".to_string()),
		Property::FireAge(FireAge::Num12) => ("age".to_string(), "12".to_string()),
		Property::FireAge(FireAge::Num13) => ("age".to_string(), "13".to_string()),
		Property::FireAge(FireAge::Num14) => ("age".to_string(), "14".to_string()),
		Property::FireAge(FireAge::Num15) => ("age".to_string(), "15".to_string()),
		Property::FireEast(FireEast::True) => ("east".to_string(), "true".to_string()),
		Property::FireEast(FireEast::False) => ("east".to_string(), "false".to_string()),
		Property::FireNorth(FireNorth::True) => ("north".to_string(), "true".to_string()),
		Property::FireNorth(FireNorth::False) => ("north".to_string(), "false".to_string()),
		Property::FireSouth(FireSouth::True) => ("south".to_string(), "true".to_string()),
		Property::FireSouth(FireSouth::False) => ("south".to_string(), "false".to_string()),
		Property::FireUp(FireUp::True) => ("up".to_string(), "true".to_string()),
		Property::FireUp(FireUp::False) => ("up".to_string(), "false".to_string()),
		Property::FireWest(FireWest::True) => ("west".to_string(), "true".to_string()),
		Property::FireWest(FireWest::False) => ("west".to_string(), "false".to_string()),
		Property::FlowerBedFacing(FlowerBedFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::FlowerBedFacing(FlowerBedFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::FlowerBedFacing(FlowerBedFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::FlowerBedFacing(FlowerBedFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::FlowerBedFlowerAmount(FlowerBedFlowerAmount::Num1) => ("flower_amount".to_string(), "1".to_string()),
		Property::FlowerBedFlowerAmount(FlowerBedFlowerAmount::Num2) => ("flower_amount".to_string(), "2".to_string()),
		Property::FlowerBedFlowerAmount(FlowerBedFlowerAmount::Num3) => ("flower_amount".to_string(), "3".to_string()),
		Property::FlowerBedFlowerAmount(FlowerBedFlowerAmount::Num4) => ("flower_amount".to_string(), "4".to_string()),
		Property::FrostedIceAge(FrostedIceAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::FrostedIceAge(FrostedIceAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::FrostedIceAge(FrostedIceAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::FrostedIceAge(FrostedIceAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::FurnaceFacing(FurnaceFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::FurnaceFacing(FurnaceFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::FurnaceFacing(FurnaceFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::FurnaceFacing(FurnaceFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::FurnaceLit(FurnaceLit::True) => ("lit".to_string(), "true".to_string()),
		Property::FurnaceLit(FurnaceLit::False) => ("lit".to_string(), "false".to_string()),
		Property::GlazedTerracottaFacing(GlazedTerracottaFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::GlazedTerracottaFacing(GlazedTerracottaFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::GlazedTerracottaFacing(GlazedTerracottaFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::GlazedTerracottaFacing(GlazedTerracottaFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::GlowLichenDown(GlowLichenDown::True) => ("down".to_string(), "true".to_string()),
		Property::GlowLichenDown(GlowLichenDown::False) => ("down".to_string(), "false".to_string()),
		Property::GlowLichenEast(GlowLichenEast::True) => ("east".to_string(), "true".to_string()),
		Property::GlowLichenEast(GlowLichenEast::False) => ("east".to_string(), "false".to_string()),
		Property::GlowLichenNorth(GlowLichenNorth::True) => ("north".to_string(), "true".to_string()),
		Property::GlowLichenNorth(GlowLichenNorth::False) => ("north".to_string(), "false".to_string()),
		Property::GlowLichenSouth(GlowLichenSouth::True) => ("south".to_string(), "true".to_string()),
		Property::GlowLichenSouth(GlowLichenSouth::False) => ("south".to_string(), "false".to_string()),
		Property::GlowLichenUp(GlowLichenUp::True) => ("up".to_string(), "true".to_string()),
		Property::GlowLichenUp(GlowLichenUp::False) => ("up".to_string(), "false".to_string()),
		Property::GlowLichenWaterlogged(GlowLichenWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::GlowLichenWaterlogged(GlowLichenWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::GlowLichenWest(GlowLichenWest::True) => ("west".to_string(), "true".to_string()),
		Property::GlowLichenWest(GlowLichenWest::False) => ("west".to_string(), "false".to_string()),
		Property::GrassSnowy(GrassSnowy::True) => ("snowy".to_string(), "true".to_string()),
		Property::GrassSnowy(GrassSnowy::False) => ("snowy".to_string(), "false".to_string()),
		Property::GrindstoneFace(GrindstoneFace::Floor) => ("face".to_string(), "floor".to_string()),
		Property::GrindstoneFace(GrindstoneFace::Wall) => ("face".to_string(), "wall".to_string()),
		Property::GrindstoneFace(GrindstoneFace::Ceiling) => ("face".to_string(), "ceiling".to_string()),
		Property::GrindstoneFacing(GrindstoneFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::GrindstoneFacing(GrindstoneFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::GrindstoneFacing(GrindstoneFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::GrindstoneFacing(GrindstoneFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::HangingMossTip(HangingMossTip::True) => ("tip".to_string(), "true".to_string()),
		Property::HangingMossTip(HangingMossTip::False) => ("tip".to_string(), "false".to_string()),
		Property::HangingRootsWaterlogged(HangingRootsWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::HangingRootsWaterlogged(HangingRootsWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::HayAxis(HayAxis::X) => ("axis".to_string(), "x".to_string()),
		Property::HayAxis(HayAxis::Y) => ("axis".to_string(), "y".to_string()),
		Property::HayAxis(HayAxis::Z) => ("axis".to_string(), "z".to_string()),
		Property::HeavyCoreWaterlogged(HeavyCoreWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::HeavyCoreWaterlogged(HeavyCoreWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::HopperEnabled(HopperEnabled::True) => ("enabled".to_string(), "true".to_string()),
		Property::HopperEnabled(HopperEnabled::False) => ("enabled".to_string(), "false".to_string()),
		Property::HopperFacing(HopperFacing::Down) => ("facing".to_string(), "down".to_string()),
		Property::HopperFacing(HopperFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::HopperFacing(HopperFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::HopperFacing(HopperFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::HopperFacing(HopperFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::HugeMushroomDown(HugeMushroomDown::True) => ("down".to_string(), "true".to_string()),
		Property::HugeMushroomDown(HugeMushroomDown::False) => ("down".to_string(), "false".to_string()),
		Property::HugeMushroomEast(HugeMushroomEast::True) => ("east".to_string(), "true".to_string()),
		Property::HugeMushroomEast(HugeMushroomEast::False) => ("east".to_string(), "false".to_string()),
		Property::HugeMushroomNorth(HugeMushroomNorth::True) => ("north".to_string(), "true".to_string()),
		Property::HugeMushroomNorth(HugeMushroomNorth::False) => ("north".to_string(), "false".to_string()),
		Property::HugeMushroomSouth(HugeMushroomSouth::True) => ("south".to_string(), "true".to_string()),
		Property::HugeMushroomSouth(HugeMushroomSouth::False) => ("south".to_string(), "false".to_string()),
		Property::HugeMushroomUp(HugeMushroomUp::True) => ("up".to_string(), "true".to_string()),
		Property::HugeMushroomUp(HugeMushroomUp::False) => ("up".to_string(), "false".to_string()),
		Property::HugeMushroomWest(HugeMushroomWest::True) => ("west".to_string(), "true".to_string()),
		Property::HugeMushroomWest(HugeMushroomWest::False) => ("west".to_string(), "false".to_string()),
		Property::InfestedRotatedPillarAxis(InfestedRotatedPillarAxis::X) => ("axis".to_string(), "x".to_string()),
		Property::InfestedRotatedPillarAxis(InfestedRotatedPillarAxis::Y) => ("axis".to_string(), "y".to_string()),
		Property::InfestedRotatedPillarAxis(InfestedRotatedPillarAxis::Z) => ("axis".to_string(), "z".to_string()),
		Property::IronBarsEast(IronBarsEast::True) => ("east".to_string(), "true".to_string()),
		Property::IronBarsEast(IronBarsEast::False) => ("east".to_string(), "false".to_string()),
		Property::IronBarsNorth(IronBarsNorth::True) => ("north".to_string(), "true".to_string()),
		Property::IronBarsNorth(IronBarsNorth::False) => ("north".to_string(), "false".to_string()),
		Property::IronBarsSouth(IronBarsSouth::True) => ("south".to_string(), "true".to_string()),
		Property::IronBarsSouth(IronBarsSouth::False) => ("south".to_string(), "false".to_string()),
		Property::IronBarsWaterlogged(IronBarsWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::IronBarsWaterlogged(IronBarsWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::IronBarsWest(IronBarsWest::True) => ("west".to_string(), "true".to_string()),
		Property::IronBarsWest(IronBarsWest::False) => ("west".to_string(), "false".to_string()),
		Property::JackOLanternFacing(JackOLanternFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::JackOLanternFacing(JackOLanternFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::JackOLanternFacing(JackOLanternFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::JackOLanternFacing(JackOLanternFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::JigsawOrientation(JigsawOrientation::DownEast) => ("orientation".to_string(), "down_east".to_string()),
		Property::JigsawOrientation(JigsawOrientation::DownNorth) => ("orientation".to_string(), "down_north".to_string()),
		Property::JigsawOrientation(JigsawOrientation::DownSouth) => ("orientation".to_string(), "down_south".to_string()),
		Property::JigsawOrientation(JigsawOrientation::DownWest) => ("orientation".to_string(), "down_west".to_string()),
		Property::JigsawOrientation(JigsawOrientation::UpEast) => ("orientation".to_string(), "up_east".to_string()),
		Property::JigsawOrientation(JigsawOrientation::UpNorth) => ("orientation".to_string(), "up_north".to_string()),
		Property::JigsawOrientation(JigsawOrientation::UpSouth) => ("orientation".to_string(), "up_south".to_string()),
		Property::JigsawOrientation(JigsawOrientation::UpWest) => ("orientation".to_string(), "up_west".to_string()),
		Property::JigsawOrientation(JigsawOrientation::WestUp) => ("orientation".to_string(), "west_up".to_string()),
		Property::JigsawOrientation(JigsawOrientation::EastUp) => ("orientation".to_string(), "east_up".to_string()),
		Property::JigsawOrientation(JigsawOrientation::NorthUp) => ("orientation".to_string(), "north_up".to_string()),
		Property::JigsawOrientation(JigsawOrientation::SouthUp) => ("orientation".to_string(), "south_up".to_string()),
		Property::JukeboxHasRecord(JukeboxHasRecord::True) => ("has_record".to_string(), "true".to_string()),
		Property::JukeboxHasRecord(JukeboxHasRecord::False) => ("has_record".to_string(), "false".to_string()),
		Property::KelpAge(KelpAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::KelpAge(KelpAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::KelpAge(KelpAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::KelpAge(KelpAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::KelpAge(KelpAge::Num4) => ("age".to_string(), "4".to_string()),
		Property::KelpAge(KelpAge::Num5) => ("age".to_string(), "5".to_string()),
		Property::KelpAge(KelpAge::Num6) => ("age".to_string(), "6".to_string()),
		Property::KelpAge(KelpAge::Num7) => ("age".to_string(), "7".to_string()),
		Property::KelpAge(KelpAge::Num8) => ("age".to_string(), "8".to_string()),
		Property::KelpAge(KelpAge::Num9) => ("age".to_string(), "9".to_string()),
		Property::KelpAge(KelpAge::Num10) => ("age".to_string(), "10".to_string()),
		Property::KelpAge(KelpAge::Num11) => ("age".to_string(), "11".to_string()),
		Property::KelpAge(KelpAge::Num12) => ("age".to_string(), "12".to_string()),
		Property::KelpAge(KelpAge::Num13) => ("age".to_string(), "13".to_string()),
		Property::KelpAge(KelpAge::Num14) => ("age".to_string(), "14".to_string()),
		Property::KelpAge(KelpAge::Num15) => ("age".to_string(), "15".to_string()),
		Property::KelpAge(KelpAge::Num16) => ("age".to_string(), "16".to_string()),
		Property::KelpAge(KelpAge::Num17) => ("age".to_string(), "17".to_string()),
		Property::KelpAge(KelpAge::Num18) => ("age".to_string(), "18".to_string()),
		Property::KelpAge(KelpAge::Num19) => ("age".to_string(), "19".to_string()),
		Property::KelpAge(KelpAge::Num20) => ("age".to_string(), "20".to_string()),
		Property::KelpAge(KelpAge::Num21) => ("age".to_string(), "21".to_string()),
		Property::KelpAge(KelpAge::Num22) => ("age".to_string(), "22".to_string()),
		Property::KelpAge(KelpAge::Num23) => ("age".to_string(), "23".to_string()),
		Property::KelpAge(KelpAge::Num24) => ("age".to_string(), "24".to_string()),
		Property::KelpAge(KelpAge::Num25) => ("age".to_string(), "25".to_string()),
		Property::LadderFacing(LadderFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::LadderFacing(LadderFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::LadderFacing(LadderFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::LadderFacing(LadderFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::LadderWaterlogged(LadderWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::LadderWaterlogged(LadderWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::LanternHanging(LanternHanging::True) => ("hanging".to_string(), "true".to_string()),
		Property::LanternHanging(LanternHanging::False) => ("hanging".to_string(), "false".to_string()),
		Property::LanternWaterlogged(LanternWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::LanternWaterlogged(LanternWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::LayeredCauldronLevel(LayeredCauldronLevel::Num1) => ("level".to_string(), "1".to_string()),
		Property::LayeredCauldronLevel(LayeredCauldronLevel::Num2) => ("level".to_string(), "2".to_string()),
		Property::LayeredCauldronLevel(LayeredCauldronLevel::Num3) => ("level".to_string(), "3".to_string()),
		Property::LeafLitterFacing(LeafLitterFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::LeafLitterFacing(LeafLitterFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::LeafLitterFacing(LeafLitterFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::LeafLitterFacing(LeafLitterFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::LeafLitterSegmentAmount(LeafLitterSegmentAmount::Num1) => ("segment_amount".to_string(), "1".to_string()),
		Property::LeafLitterSegmentAmount(LeafLitterSegmentAmount::Num2) => ("segment_amount".to_string(), "2".to_string()),
		Property::LeafLitterSegmentAmount(LeafLitterSegmentAmount::Num3) => ("segment_amount".to_string(), "3".to_string()),
		Property::LeafLitterSegmentAmount(LeafLitterSegmentAmount::Num4) => ("segment_amount".to_string(), "4".to_string()),
		Property::LecternFacing(LecternFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::LecternFacing(LecternFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::LecternFacing(LecternFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::LecternFacing(LecternFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::LecternHasBook(LecternHasBook::True) => ("has_book".to_string(), "true".to_string()),
		Property::LecternHasBook(LecternHasBook::False) => ("has_book".to_string(), "false".to_string()),
		Property::LecternPowered(LecternPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::LecternPowered(LecternPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::LeverFace(LeverFace::Floor) => ("face".to_string(), "floor".to_string()),
		Property::LeverFace(LeverFace::Wall) => ("face".to_string(), "wall".to_string()),
		Property::LeverFace(LeverFace::Ceiling) => ("face".to_string(), "ceiling".to_string()),
		Property::LeverFacing(LeverFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::LeverFacing(LeverFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::LeverFacing(LeverFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::LeverFacing(LeverFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::LeverPowered(LeverPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::LeverPowered(LeverPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::LightLevel(LightLevel::Num0) => ("level".to_string(), "0".to_string()),
		Property::LightLevel(LightLevel::Num1) => ("level".to_string(), "1".to_string()),
		Property::LightLevel(LightLevel::Num2) => ("level".to_string(), "2".to_string()),
		Property::LightLevel(LightLevel::Num3) => ("level".to_string(), "3".to_string()),
		Property::LightLevel(LightLevel::Num4) => ("level".to_string(), "4".to_string()),
		Property::LightLevel(LightLevel::Num5) => ("level".to_string(), "5".to_string()),
		Property::LightLevel(LightLevel::Num6) => ("level".to_string(), "6".to_string()),
		Property::LightLevel(LightLevel::Num7) => ("level".to_string(), "7".to_string()),
		Property::LightLevel(LightLevel::Num8) => ("level".to_string(), "8".to_string()),
		Property::LightLevel(LightLevel::Num9) => ("level".to_string(), "9".to_string()),
		Property::LightLevel(LightLevel::Num10) => ("level".to_string(), "10".to_string()),
		Property::LightLevel(LightLevel::Num11) => ("level".to_string(), "11".to_string()),
		Property::LightLevel(LightLevel::Num12) => ("level".to_string(), "12".to_string()),
		Property::LightLevel(LightLevel::Num13) => ("level".to_string(), "13".to_string()),
		Property::LightLevel(LightLevel::Num14) => ("level".to_string(), "14".to_string()),
		Property::LightLevel(LightLevel::Num15) => ("level".to_string(), "15".to_string()),
		Property::LightWaterlogged(LightWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::LightWaterlogged(LightWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::LightningRodFacing(LightningRodFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::LightningRodFacing(LightningRodFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::LightningRodFacing(LightningRodFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::LightningRodFacing(LightningRodFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::LightningRodFacing(LightningRodFacing::Up) => ("facing".to_string(), "up".to_string()),
		Property::LightningRodFacing(LightningRodFacing::Down) => ("facing".to_string(), "down".to_string()),
		Property::LightningRodPowered(LightningRodPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::LightningRodPowered(LightningRodPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::LightningRodWaterlogged(LightningRodWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::LightningRodWaterlogged(LightningRodWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::LiquidLevel(LiquidLevel::Num0) => ("level".to_string(), "0".to_string()),
		Property::LiquidLevel(LiquidLevel::Num1) => ("level".to_string(), "1".to_string()),
		Property::LiquidLevel(LiquidLevel::Num2) => ("level".to_string(), "2".to_string()),
		Property::LiquidLevel(LiquidLevel::Num3) => ("level".to_string(), "3".to_string()),
		Property::LiquidLevel(LiquidLevel::Num4) => ("level".to_string(), "4".to_string()),
		Property::LiquidLevel(LiquidLevel::Num5) => ("level".to_string(), "5".to_string()),
		Property::LiquidLevel(LiquidLevel::Num6) => ("level".to_string(), "6".to_string()),
		Property::LiquidLevel(LiquidLevel::Num7) => ("level".to_string(), "7".to_string()),
		Property::LiquidLevel(LiquidLevel::Num8) => ("level".to_string(), "8".to_string()),
		Property::LiquidLevel(LiquidLevel::Num9) => ("level".to_string(), "9".to_string()),
		Property::LiquidLevel(LiquidLevel::Num10) => ("level".to_string(), "10".to_string()),
		Property::LiquidLevel(LiquidLevel::Num11) => ("level".to_string(), "11".to_string()),
		Property::LiquidLevel(LiquidLevel::Num12) => ("level".to_string(), "12".to_string()),
		Property::LiquidLevel(LiquidLevel::Num13) => ("level".to_string(), "13".to_string()),
		Property::LiquidLevel(LiquidLevel::Num14) => ("level".to_string(), "14".to_string()),
		Property::LiquidLevel(LiquidLevel::Num15) => ("level".to_string(), "15".to_string()),
		Property::LoomFacing(LoomFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::LoomFacing(LoomFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::LoomFacing(LoomFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::LoomFacing(LoomFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::MangroveLeavesDistance(MangroveLeavesDistance::Num1) => ("distance".to_string(), "1".to_string()),
		Property::MangroveLeavesDistance(MangroveLeavesDistance::Num2) => ("distance".to_string(), "2".to_string()),
		Property::MangroveLeavesDistance(MangroveLeavesDistance::Num3) => ("distance".to_string(), "3".to_string()),
		Property::MangroveLeavesDistance(MangroveLeavesDistance::Num4) => ("distance".to_string(), "4".to_string()),
		Property::MangroveLeavesDistance(MangroveLeavesDistance::Num5) => ("distance".to_string(), "5".to_string()),
		Property::MangroveLeavesDistance(MangroveLeavesDistance::Num6) => ("distance".to_string(), "6".to_string()),
		Property::MangroveLeavesDistance(MangroveLeavesDistance::Num7) => ("distance".to_string(), "7".to_string()),
		Property::MangroveLeavesPersistent(MangroveLeavesPersistent::True) => ("persistent".to_string(), "true".to_string()),
		Property::MangroveLeavesPersistent(MangroveLeavesPersistent::False) => ("persistent".to_string(), "false".to_string()),
		Property::MangroveLeavesWaterlogged(MangroveLeavesWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::MangroveLeavesWaterlogged(MangroveLeavesWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::MangrovePropaguleAge(MangrovePropaguleAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::MangrovePropaguleAge(MangrovePropaguleAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::MangrovePropaguleAge(MangrovePropaguleAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::MangrovePropaguleAge(MangrovePropaguleAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::MangrovePropaguleAge(MangrovePropaguleAge::Num4) => ("age".to_string(), "4".to_string()),
		Property::MangrovePropaguleHanging(MangrovePropaguleHanging::True) => ("hanging".to_string(), "true".to_string()),
		Property::MangrovePropaguleHanging(MangrovePropaguleHanging::False) => ("hanging".to_string(), "false".to_string()),
		Property::MangrovePropaguleStage(MangrovePropaguleStage::Num0) => ("stage".to_string(), "0".to_string()),
		Property::MangrovePropaguleStage(MangrovePropaguleStage::Num1) => ("stage".to_string(), "1".to_string()),
		Property::MangrovePropaguleWaterlogged(MangrovePropaguleWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::MangrovePropaguleWaterlogged(MangrovePropaguleWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::MangroveRootsWaterlogged(MangroveRootsWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::MangroveRootsWaterlogged(MangroveRootsWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::MossyCarpetBottom(MossyCarpetBottom::True) => ("bottom".to_string(), "true".to_string()),
		Property::MossyCarpetBottom(MossyCarpetBottom::False) => ("bottom".to_string(), "false".to_string()),
		Property::MossyCarpetEast(MossyCarpetEast::None) => ("east".to_string(), "none".to_string()),
		Property::MossyCarpetEast(MossyCarpetEast::Low) => ("east".to_string(), "low".to_string()),
		Property::MossyCarpetEast(MossyCarpetEast::Tall) => ("east".to_string(), "tall".to_string()),
		Property::MossyCarpetNorth(MossyCarpetNorth::None) => ("north".to_string(), "none".to_string()),
		Property::MossyCarpetNorth(MossyCarpetNorth::Low) => ("north".to_string(), "low".to_string()),
		Property::MossyCarpetNorth(MossyCarpetNorth::Tall) => ("north".to_string(), "tall".to_string()),
		Property::MossyCarpetSouth(MossyCarpetSouth::None) => ("south".to_string(), "none".to_string()),
		Property::MossyCarpetSouth(MossyCarpetSouth::Low) => ("south".to_string(), "low".to_string()),
		Property::MossyCarpetSouth(MossyCarpetSouth::Tall) => ("south".to_string(), "tall".to_string()),
		Property::MossyCarpetWest(MossyCarpetWest::None) => ("west".to_string(), "none".to_string()),
		Property::MossyCarpetWest(MossyCarpetWest::Low) => ("west".to_string(), "low".to_string()),
		Property::MossyCarpetWest(MossyCarpetWest::Tall) => ("west".to_string(), "tall".to_string()),
		Property::MovingPistonFacing(MovingPistonFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::MovingPistonFacing(MovingPistonFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::MovingPistonFacing(MovingPistonFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::MovingPistonFacing(MovingPistonFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::MovingPistonFacing(MovingPistonFacing::Up) => ("facing".to_string(), "up".to_string()),
		Property::MovingPistonFacing(MovingPistonFacing::Down) => ("facing".to_string(), "down".to_string()),
		Property::MovingPistonType(MovingPistonType::Normal) => ("type".to_string(), "normal".to_string()),
		Property::MovingPistonType(MovingPistonType::Sticky) => ("type".to_string(), "sticky".to_string()),
		Property::MultifaceDown(MultifaceDown::True) => ("down".to_string(), "true".to_string()),
		Property::MultifaceDown(MultifaceDown::False) => ("down".to_string(), "false".to_string()),
		Property::MultifaceEast(MultifaceEast::True) => ("east".to_string(), "true".to_string()),
		Property::MultifaceEast(MultifaceEast::False) => ("east".to_string(), "false".to_string()),
		Property::MultifaceNorth(MultifaceNorth::True) => ("north".to_string(), "true".to_string()),
		Property::MultifaceNorth(MultifaceNorth::False) => ("north".to_string(), "false".to_string()),
		Property::MultifaceSouth(MultifaceSouth::True) => ("south".to_string(), "true".to_string()),
		Property::MultifaceSouth(MultifaceSouth::False) => ("south".to_string(), "false".to_string()),
		Property::MultifaceUp(MultifaceUp::True) => ("up".to_string(), "true".to_string()),
		Property::MultifaceUp(MultifaceUp::False) => ("up".to_string(), "false".to_string()),
		Property::MultifaceWaterlogged(MultifaceWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::MultifaceWaterlogged(MultifaceWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::MultifaceWest(MultifaceWest::True) => ("west".to_string(), "true".to_string()),
		Property::MultifaceWest(MultifaceWest::False) => ("west".to_string(), "false".to_string()),
		Property::MyceliumSnowy(MyceliumSnowy::True) => ("snowy".to_string(), "true".to_string()),
		Property::MyceliumSnowy(MyceliumSnowy::False) => ("snowy".to_string(), "false".to_string()),
		Property::NetherPortalAxis(NetherPortalAxis::X) => ("axis".to_string(), "x".to_string()),
		Property::NetherPortalAxis(NetherPortalAxis::Z) => ("axis".to_string(), "z".to_string()),
		Property::NetherWartAge(NetherWartAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::NetherWartAge(NetherWartAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::NetherWartAge(NetherWartAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::NetherWartAge(NetherWartAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::NoteInstrument(NoteInstrument::Harp) => ("instrument".to_string(), "harp".to_string()),
		Property::NoteInstrument(NoteInstrument::Basedrum) => ("instrument".to_string(), "basedrum".to_string()),
		Property::NoteInstrument(NoteInstrument::Snare) => ("instrument".to_string(), "snare".to_string()),
		Property::NoteInstrument(NoteInstrument::Hat) => ("instrument".to_string(), "hat".to_string()),
		Property::NoteInstrument(NoteInstrument::Bass) => ("instrument".to_string(), "bass".to_string()),
		Property::NoteInstrument(NoteInstrument::Flute) => ("instrument".to_string(), "flute".to_string()),
		Property::NoteInstrument(NoteInstrument::Bell) => ("instrument".to_string(), "bell".to_string()),
		Property::NoteInstrument(NoteInstrument::Guitar) => ("instrument".to_string(), "guitar".to_string()),
		Property::NoteInstrument(NoteInstrument::Chime) => ("instrument".to_string(), "chime".to_string()),
		Property::NoteInstrument(NoteInstrument::Xylophone) => ("instrument".to_string(), "xylophone".to_string()),
		Property::NoteInstrument(NoteInstrument::IronXylophone) => ("instrument".to_string(), "iron_xylophone".to_string()),
		Property::NoteInstrument(NoteInstrument::CowBell) => ("instrument".to_string(), "cow_bell".to_string()),
		Property::NoteInstrument(NoteInstrument::Didgeridoo) => ("instrument".to_string(), "didgeridoo".to_string()),
		Property::NoteInstrument(NoteInstrument::Bit) => ("instrument".to_string(), "bit".to_string()),
		Property::NoteInstrument(NoteInstrument::Banjo) => ("instrument".to_string(), "banjo".to_string()),
		Property::NoteInstrument(NoteInstrument::Pling) => ("instrument".to_string(), "pling".to_string()),
		Property::NoteInstrument(NoteInstrument::Zombie) => ("instrument".to_string(), "zombie".to_string()),
		Property::NoteInstrument(NoteInstrument::Skeleton) => ("instrument".to_string(), "skeleton".to_string()),
		Property::NoteInstrument(NoteInstrument::Creeper) => ("instrument".to_string(), "creeper".to_string()),
		Property::NoteInstrument(NoteInstrument::Dragon) => ("instrument".to_string(), "dragon".to_string()),
		Property::NoteInstrument(NoteInstrument::WitherSkeleton) => ("instrument".to_string(), "wither_skeleton".to_string()),
		Property::NoteInstrument(NoteInstrument::Piglin) => ("instrument".to_string(), "piglin".to_string()),
		Property::NoteInstrument(NoteInstrument::CustomHead) => ("instrument".to_string(), "custom_head".to_string()),
		Property::NoteNote(NoteNote::Num0) => ("note".to_string(), "0".to_string()),
		Property::NoteNote(NoteNote::Num1) => ("note".to_string(), "1".to_string()),
		Property::NoteNote(NoteNote::Num2) => ("note".to_string(), "2".to_string()),
		Property::NoteNote(NoteNote::Num3) => ("note".to_string(), "3".to_string()),
		Property::NoteNote(NoteNote::Num4) => ("note".to_string(), "4".to_string()),
		Property::NoteNote(NoteNote::Num5) => ("note".to_string(), "5".to_string()),
		Property::NoteNote(NoteNote::Num6) => ("note".to_string(), "6".to_string()),
		Property::NoteNote(NoteNote::Num7) => ("note".to_string(), "7".to_string()),
		Property::NoteNote(NoteNote::Num8) => ("note".to_string(), "8".to_string()),
		Property::NoteNote(NoteNote::Num9) => ("note".to_string(), "9".to_string()),
		Property::NoteNote(NoteNote::Num10) => ("note".to_string(), "10".to_string()),
		Property::NoteNote(NoteNote::Num11) => ("note".to_string(), "11".to_string()),
		Property::NoteNote(NoteNote::Num12) => ("note".to_string(), "12".to_string()),
		Property::NoteNote(NoteNote::Num13) => ("note".to_string(), "13".to_string()),
		Property::NoteNote(NoteNote::Num14) => ("note".to_string(), "14".to_string()),
		Property::NoteNote(NoteNote::Num15) => ("note".to_string(), "15".to_string()),
		Property::NoteNote(NoteNote::Num16) => ("note".to_string(), "16".to_string()),
		Property::NoteNote(NoteNote::Num17) => ("note".to_string(), "17".to_string()),
		Property::NoteNote(NoteNote::Num18) => ("note".to_string(), "18".to_string()),
		Property::NoteNote(NoteNote::Num19) => ("note".to_string(), "19".to_string()),
		Property::NoteNote(NoteNote::Num20) => ("note".to_string(), "20".to_string()),
		Property::NoteNote(NoteNote::Num21) => ("note".to_string(), "21".to_string()),
		Property::NoteNote(NoteNote::Num22) => ("note".to_string(), "22".to_string()),
		Property::NoteNote(NoteNote::Num23) => ("note".to_string(), "23".to_string()),
		Property::NoteNote(NoteNote::Num24) => ("note".to_string(), "24".to_string()),
		Property::NotePowered(NotePowered::True) => ("powered".to_string(), "true".to_string()),
		Property::NotePowered(NotePowered::False) => ("powered".to_string(), "false".to_string()),
		Property::ObserverFacing(ObserverFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::ObserverFacing(ObserverFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::ObserverFacing(ObserverFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::ObserverFacing(ObserverFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::ObserverFacing(ObserverFacing::Up) => ("facing".to_string(), "up".to_string()),
		Property::ObserverFacing(ObserverFacing::Down) => ("facing".to_string(), "down".to_string()),
		Property::ObserverPowered(ObserverPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::ObserverPowered(ObserverPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::PiglinwallskullFacing(PiglinwallskullFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::PiglinwallskullFacing(PiglinwallskullFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::PiglinwallskullFacing(PiglinwallskullFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::PiglinwallskullFacing(PiglinwallskullFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::PiglinwallskullPowered(PiglinwallskullPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::PiglinwallskullPowered(PiglinwallskullPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::PistonBaseExtended(PistonBaseExtended::True) => ("extended".to_string(), "true".to_string()),
		Property::PistonBaseExtended(PistonBaseExtended::False) => ("extended".to_string(), "false".to_string()),
		Property::PistonBaseFacing(PistonBaseFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::PistonBaseFacing(PistonBaseFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::PistonBaseFacing(PistonBaseFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::PistonBaseFacing(PistonBaseFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::PistonBaseFacing(PistonBaseFacing::Up) => ("facing".to_string(), "up".to_string()),
		Property::PistonBaseFacing(PistonBaseFacing::Down) => ("facing".to_string(), "down".to_string()),
		Property::PistonHeadFacing(PistonHeadFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::PistonHeadFacing(PistonHeadFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::PistonHeadFacing(PistonHeadFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::PistonHeadFacing(PistonHeadFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::PistonHeadFacing(PistonHeadFacing::Up) => ("facing".to_string(), "up".to_string()),
		Property::PistonHeadFacing(PistonHeadFacing::Down) => ("facing".to_string(), "down".to_string()),
		Property::PistonHeadShort(PistonHeadShort::True) => ("short".to_string(), "true".to_string()),
		Property::PistonHeadShort(PistonHeadShort::False) => ("short".to_string(), "false".to_string()),
		Property::PistonHeadType(PistonHeadType::Normal) => ("type".to_string(), "normal".to_string()),
		Property::PistonHeadType(PistonHeadType::Sticky) => ("type".to_string(), "sticky".to_string()),
		Property::PitcherCropAge(PitcherCropAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::PitcherCropAge(PitcherCropAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::PitcherCropAge(PitcherCropAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::PitcherCropAge(PitcherCropAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::PitcherCropAge(PitcherCropAge::Num4) => ("age".to_string(), "4".to_string()),
		Property::PitcherCropHalf(PitcherCropHalf::Upper) => ("half".to_string(), "upper".to_string()),
		Property::PitcherCropHalf(PitcherCropHalf::Lower) => ("half".to_string(), "lower".to_string()),
		Property::PlayerHeadPowered(PlayerHeadPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::PlayerHeadPowered(PlayerHeadPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num0) => ("rotation".to_string(), "0".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num1) => ("rotation".to_string(), "1".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num2) => ("rotation".to_string(), "2".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num3) => ("rotation".to_string(), "3".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num4) => ("rotation".to_string(), "4".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num5) => ("rotation".to_string(), "5".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num6) => ("rotation".to_string(), "6".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num7) => ("rotation".to_string(), "7".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num8) => ("rotation".to_string(), "8".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num9) => ("rotation".to_string(), "9".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num10) => ("rotation".to_string(), "10".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num11) => ("rotation".to_string(), "11".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num12) => ("rotation".to_string(), "12".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num13) => ("rotation".to_string(), "13".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num14) => ("rotation".to_string(), "14".to_string()),
		Property::PlayerHeadRotation(PlayerHeadRotation::Num15) => ("rotation".to_string(), "15".to_string()),
		Property::PlayerWallHeadFacing(PlayerWallHeadFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::PlayerWallHeadFacing(PlayerWallHeadFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::PlayerWallHeadFacing(PlayerWallHeadFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::PlayerWallHeadFacing(PlayerWallHeadFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::PlayerWallHeadPowered(PlayerWallHeadPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::PlayerWallHeadPowered(PlayerWallHeadPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::PointedDripstoneThickness(PointedDripstoneThickness::TipMerge) => ("thickness".to_string(), "tip_merge".to_string()),
		Property::PointedDripstoneThickness(PointedDripstoneThickness::Tip) => ("thickness".to_string(), "tip".to_string()),
		Property::PointedDripstoneThickness(PointedDripstoneThickness::Frustum) => ("thickness".to_string(), "frustum".to_string()),
		Property::PointedDripstoneThickness(PointedDripstoneThickness::Middle) => ("thickness".to_string(), "middle".to_string()),
		Property::PointedDripstoneThickness(PointedDripstoneThickness::Base) => ("thickness".to_string(), "base".to_string()),
		Property::PointedDripstoneVerticalDirection(PointedDripstoneVerticalDirection::Up) => ("vertical_direction".to_string(), "up".to_string()),
		Property::PointedDripstoneVerticalDirection(PointedDripstoneVerticalDirection::Down) => ("vertical_direction".to_string(), "down".to_string()),
		Property::PointedDripstoneWaterlogged(PointedDripstoneWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::PointedDripstoneWaterlogged(PointedDripstoneWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::PotatoAge(PotatoAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::PotatoAge(PotatoAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::PotatoAge(PotatoAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::PotatoAge(PotatoAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::PotatoAge(PotatoAge::Num4) => ("age".to_string(), "4".to_string()),
		Property::PotatoAge(PotatoAge::Num5) => ("age".to_string(), "5".to_string()),
		Property::PotatoAge(PotatoAge::Num6) => ("age".to_string(), "6".to_string()),
		Property::PotatoAge(PotatoAge::Num7) => ("age".to_string(), "7".to_string()),
		Property::PoweredRailPowered(PoweredRailPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::PoweredRailPowered(PoweredRailPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::PoweredRailShape(PoweredRailShape::NorthSouth) => ("shape".to_string(), "north_south".to_string()),
		Property::PoweredRailShape(PoweredRailShape::EastWest) => ("shape".to_string(), "east_west".to_string()),
		Property::PoweredRailShape(PoweredRailShape::AscendingEast) => ("shape".to_string(), "ascending_east".to_string()),
		Property::PoweredRailShape(PoweredRailShape::AscendingWest) => ("shape".to_string(), "ascending_west".to_string()),
		Property::PoweredRailShape(PoweredRailShape::AscendingNorth) => ("shape".to_string(), "ascending_north".to_string()),
		Property::PoweredRailShape(PoweredRailShape::AscendingSouth) => ("shape".to_string(), "ascending_south".to_string()),
		Property::PoweredRailWaterlogged(PoweredRailWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::PoweredRailWaterlogged(PoweredRailWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::PressurePlatePowered(PressurePlatePowered::True) => ("powered".to_string(), "true".to_string()),
		Property::PressurePlatePowered(PressurePlatePowered::False) => ("powered".to_string(), "false".to_string()),
		Property::RailShape(RailShape::NorthSouth) => ("shape".to_string(), "north_south".to_string()),
		Property::RailShape(RailShape::EastWest) => ("shape".to_string(), "east_west".to_string()),
		Property::RailShape(RailShape::AscendingEast) => ("shape".to_string(), "ascending_east".to_string()),
		Property::RailShape(RailShape::AscendingWest) => ("shape".to_string(), "ascending_west".to_string()),
		Property::RailShape(RailShape::AscendingNorth) => ("shape".to_string(), "ascending_north".to_string()),
		Property::RailShape(RailShape::AscendingSouth) => ("shape".to_string(), "ascending_south".to_string()),
		Property::RailShape(RailShape::SouthEast) => ("shape".to_string(), "south_east".to_string()),
		Property::RailShape(RailShape::SouthWest) => ("shape".to_string(), "south_west".to_string()),
		Property::RailShape(RailShape::NorthWest) => ("shape".to_string(), "north_west".to_string()),
		Property::RailShape(RailShape::NorthEast) => ("shape".to_string(), "north_east".to_string()),
		Property::RailWaterlogged(RailWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::RailWaterlogged(RailWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::RedstoneLampLit(RedstoneLampLit::True) => ("lit".to_string(), "true".to_string()),
		Property::RedstoneLampLit(RedstoneLampLit::False) => ("lit".to_string(), "false".to_string()),
		Property::RedstoneOreLit(RedstoneOreLit::True) => ("lit".to_string(), "true".to_string()),
		Property::RedstoneOreLit(RedstoneOreLit::False) => ("lit".to_string(), "false".to_string()),
		Property::RedstoneTorchLit(RedstoneTorchLit::True) => ("lit".to_string(), "true".to_string()),
		Property::RedstoneTorchLit(RedstoneTorchLit::False) => ("lit".to_string(), "false".to_string()),
		Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::RedstoneWallTorchLit(RedstoneWallTorchLit::True) => ("lit".to_string(), "true".to_string()),
		Property::RedstoneWallTorchLit(RedstoneWallTorchLit::False) => ("lit".to_string(), "false".to_string()),
		Property::RedstoneWireEast(RedstoneWireEast::Up) => ("east".to_string(), "up".to_string()),
		Property::RedstoneWireEast(RedstoneWireEast::Side) => ("east".to_string(), "side".to_string()),
		Property::RedstoneWireEast(RedstoneWireEast::None) => ("east".to_string(), "none".to_string()),
		Property::RedstoneWireNorth(RedstoneWireNorth::Up) => ("north".to_string(), "up".to_string()),
		Property::RedstoneWireNorth(RedstoneWireNorth::Side) => ("north".to_string(), "side".to_string()),
		Property::RedstoneWireNorth(RedstoneWireNorth::None) => ("north".to_string(), "none".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num0) => ("power".to_string(), "0".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num1) => ("power".to_string(), "1".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num2) => ("power".to_string(), "2".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num3) => ("power".to_string(), "3".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num4) => ("power".to_string(), "4".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num5) => ("power".to_string(), "5".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num6) => ("power".to_string(), "6".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num7) => ("power".to_string(), "7".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num8) => ("power".to_string(), "8".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num9) => ("power".to_string(), "9".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num10) => ("power".to_string(), "10".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num11) => ("power".to_string(), "11".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num12) => ("power".to_string(), "12".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num13) => ("power".to_string(), "13".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num14) => ("power".to_string(), "14".to_string()),
		Property::RedstoneWirePower(RedstoneWirePower::Num15) => ("power".to_string(), "15".to_string()),
		Property::RedstoneWireSouth(RedstoneWireSouth::Up) => ("south".to_string(), "up".to_string()),
		Property::RedstoneWireSouth(RedstoneWireSouth::Side) => ("south".to_string(), "side".to_string()),
		Property::RedstoneWireSouth(RedstoneWireSouth::None) => ("south".to_string(), "none".to_string()),
		Property::RedstoneWireWest(RedstoneWireWest::Up) => ("west".to_string(), "up".to_string()),
		Property::RedstoneWireWest(RedstoneWireWest::Side) => ("west".to_string(), "side".to_string()),
		Property::RedstoneWireWest(RedstoneWireWest::None) => ("west".to_string(), "none".to_string()),
		Property::RepeaterDelay(RepeaterDelay::Num1) => ("delay".to_string(), "1".to_string()),
		Property::RepeaterDelay(RepeaterDelay::Num2) => ("delay".to_string(), "2".to_string()),
		Property::RepeaterDelay(RepeaterDelay::Num3) => ("delay".to_string(), "3".to_string()),
		Property::RepeaterDelay(RepeaterDelay::Num4) => ("delay".to_string(), "4".to_string()),
		Property::RepeaterFacing(RepeaterFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::RepeaterFacing(RepeaterFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::RepeaterFacing(RepeaterFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::RepeaterFacing(RepeaterFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::RepeaterLocked(RepeaterLocked::True) => ("locked".to_string(), "true".to_string()),
		Property::RepeaterLocked(RepeaterLocked::False) => ("locked".to_string(), "false".to_string()),
		Property::RepeaterPowered(RepeaterPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::RepeaterPowered(RepeaterPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::RespawnAnchorCharges(RespawnAnchorCharges::Num0) => ("charges".to_string(), "0".to_string()),
		Property::RespawnAnchorCharges(RespawnAnchorCharges::Num1) => ("charges".to_string(), "1".to_string()),
		Property::RespawnAnchorCharges(RespawnAnchorCharges::Num2) => ("charges".to_string(), "2".to_string()),
		Property::RespawnAnchorCharges(RespawnAnchorCharges::Num3) => ("charges".to_string(), "3".to_string()),
		Property::RespawnAnchorCharges(RespawnAnchorCharges::Num4) => ("charges".to_string(), "4".to_string()),
		Property::RotatedPillarAxis(RotatedPillarAxis::X) => ("axis".to_string(), "x".to_string()),
		Property::RotatedPillarAxis(RotatedPillarAxis::Y) => ("axis".to_string(), "y".to_string()),
		Property::RotatedPillarAxis(RotatedPillarAxis::Z) => ("axis".to_string(), "z".to_string()),
		Property::SaplingStage(SaplingStage::Num0) => ("stage".to_string(), "0".to_string()),
		Property::SaplingStage(SaplingStage::Num1) => ("stage".to_string(), "1".to_string()),
		Property::ScaffoldingBottom(ScaffoldingBottom::True) => ("bottom".to_string(), "true".to_string()),
		Property::ScaffoldingBottom(ScaffoldingBottom::False) => ("bottom".to_string(), "false".to_string()),
		Property::ScaffoldingDistance(ScaffoldingDistance::Num0) => ("distance".to_string(), "0".to_string()),
		Property::ScaffoldingDistance(ScaffoldingDistance::Num1) => ("distance".to_string(), "1".to_string()),
		Property::ScaffoldingDistance(ScaffoldingDistance::Num2) => ("distance".to_string(), "2".to_string()),
		Property::ScaffoldingDistance(ScaffoldingDistance::Num3) => ("distance".to_string(), "3".to_string()),
		Property::ScaffoldingDistance(ScaffoldingDistance::Num4) => ("distance".to_string(), "4".to_string()),
		Property::ScaffoldingDistance(ScaffoldingDistance::Num5) => ("distance".to_string(), "5".to_string()),
		Property::ScaffoldingDistance(ScaffoldingDistance::Num6) => ("distance".to_string(), "6".to_string()),
		Property::ScaffoldingDistance(ScaffoldingDistance::Num7) => ("distance".to_string(), "7".to_string()),
		Property::ScaffoldingWaterlogged(ScaffoldingWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::ScaffoldingWaterlogged(ScaffoldingWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::SculkCatalystBloom(SculkCatalystBloom::True) => ("bloom".to_string(), "true".to_string()),
		Property::SculkCatalystBloom(SculkCatalystBloom::False) => ("bloom".to_string(), "false".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num0) => ("power".to_string(), "0".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num1) => ("power".to_string(), "1".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num2) => ("power".to_string(), "2".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num3) => ("power".to_string(), "3".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num4) => ("power".to_string(), "4".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num5) => ("power".to_string(), "5".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num6) => ("power".to_string(), "6".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num7) => ("power".to_string(), "7".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num8) => ("power".to_string(), "8".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num9) => ("power".to_string(), "9".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num10) => ("power".to_string(), "10".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num11) => ("power".to_string(), "11".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num12) => ("power".to_string(), "12".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num13) => ("power".to_string(), "13".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num14) => ("power".to_string(), "14".to_string()),
		Property::SculkSensorPower(SculkSensorPower::Num15) => ("power".to_string(), "15".to_string()),
		Property::SculkSensorSculkSensorPhase(SculkSensorSculkSensorPhase::Inactive) => ("sculk_sensor_phase".to_string(), "inactive".to_string()),
		Property::SculkSensorSculkSensorPhase(SculkSensorSculkSensorPhase::Active) => ("sculk_sensor_phase".to_string(), "active".to_string()),
		Property::SculkSensorSculkSensorPhase(SculkSensorSculkSensorPhase::Cooldown) => ("sculk_sensor_phase".to_string(), "cooldown".to_string()),
		Property::SculkSensorWaterlogged(SculkSensorWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::SculkSensorWaterlogged(SculkSensorWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::SculkShriekerCanSummon(SculkShriekerCanSummon::True) => ("can_summon".to_string(), "true".to_string()),
		Property::SculkShriekerCanSummon(SculkShriekerCanSummon::False) => ("can_summon".to_string(), "false".to_string()),
		Property::SculkShriekerShrieking(SculkShriekerShrieking::True) => ("shrieking".to_string(), "true".to_string()),
		Property::SculkShriekerShrieking(SculkShriekerShrieking::False) => ("shrieking".to_string(), "false".to_string()),
		Property::SculkShriekerWaterlogged(SculkShriekerWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::SculkShriekerWaterlogged(SculkShriekerWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::SculkVeinDown(SculkVeinDown::True) => ("down".to_string(), "true".to_string()),
		Property::SculkVeinDown(SculkVeinDown::False) => ("down".to_string(), "false".to_string()),
		Property::SculkVeinEast(SculkVeinEast::True) => ("east".to_string(), "true".to_string()),
		Property::SculkVeinEast(SculkVeinEast::False) => ("east".to_string(), "false".to_string()),
		Property::SculkVeinNorth(SculkVeinNorth::True) => ("north".to_string(), "true".to_string()),
		Property::SculkVeinNorth(SculkVeinNorth::False) => ("north".to_string(), "false".to_string()),
		Property::SculkVeinSouth(SculkVeinSouth::True) => ("south".to_string(), "true".to_string()),
		Property::SculkVeinSouth(SculkVeinSouth::False) => ("south".to_string(), "false".to_string()),
		Property::SculkVeinUp(SculkVeinUp::True) => ("up".to_string(), "true".to_string()),
		Property::SculkVeinUp(SculkVeinUp::False) => ("up".to_string(), "false".to_string()),
		Property::SculkVeinWaterlogged(SculkVeinWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::SculkVeinWaterlogged(SculkVeinWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::SculkVeinWest(SculkVeinWest::True) => ("west".to_string(), "true".to_string()),
		Property::SculkVeinWest(SculkVeinWest::False) => ("west".to_string(), "false".to_string()),
		Property::SeaPicklePickles(SeaPicklePickles::Num1) => ("pickles".to_string(), "1".to_string()),
		Property::SeaPicklePickles(SeaPicklePickles::Num2) => ("pickles".to_string(), "2".to_string()),
		Property::SeaPicklePickles(SeaPicklePickles::Num3) => ("pickles".to_string(), "3".to_string()),
		Property::SeaPicklePickles(SeaPicklePickles::Num4) => ("pickles".to_string(), "4".to_string()),
		Property::SeaPickleWaterlogged(SeaPickleWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::SeaPickleWaterlogged(SeaPickleWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::ShelfFacing(ShelfFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::ShelfFacing(ShelfFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::ShelfFacing(ShelfFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::ShelfFacing(ShelfFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::ShelfPowered(ShelfPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::ShelfPowered(ShelfPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::ShelfSideChain(ShelfSideChain::Unconnected) => ("side_chain".to_string(), "unconnected".to_string()),
		Property::ShelfSideChain(ShelfSideChain::Right) => ("side_chain".to_string(), "right".to_string()),
		Property::ShelfSideChain(ShelfSideChain::Center) => ("side_chain".to_string(), "center".to_string()),
		Property::ShelfSideChain(ShelfSideChain::Left) => ("side_chain".to_string(), "left".to_string()),
		Property::ShelfWaterlogged(ShelfWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::ShelfWaterlogged(ShelfWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::ShulkerBoxFacing(ShulkerBoxFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::ShulkerBoxFacing(ShulkerBoxFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::ShulkerBoxFacing(ShulkerBoxFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::ShulkerBoxFacing(ShulkerBoxFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::ShulkerBoxFacing(ShulkerBoxFacing::Up) => ("facing".to_string(), "up".to_string()),
		Property::ShulkerBoxFacing(ShulkerBoxFacing::Down) => ("facing".to_string(), "down".to_string()),
		Property::SkullPowered(SkullPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::SkullPowered(SkullPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::SkullRotation(SkullRotation::Num0) => ("rotation".to_string(), "0".to_string()),
		Property::SkullRotation(SkullRotation::Num1) => ("rotation".to_string(), "1".to_string()),
		Property::SkullRotation(SkullRotation::Num2) => ("rotation".to_string(), "2".to_string()),
		Property::SkullRotation(SkullRotation::Num3) => ("rotation".to_string(), "3".to_string()),
		Property::SkullRotation(SkullRotation::Num4) => ("rotation".to_string(), "4".to_string()),
		Property::SkullRotation(SkullRotation::Num5) => ("rotation".to_string(), "5".to_string()),
		Property::SkullRotation(SkullRotation::Num6) => ("rotation".to_string(), "6".to_string()),
		Property::SkullRotation(SkullRotation::Num7) => ("rotation".to_string(), "7".to_string()),
		Property::SkullRotation(SkullRotation::Num8) => ("rotation".to_string(), "8".to_string()),
		Property::SkullRotation(SkullRotation::Num9) => ("rotation".to_string(), "9".to_string()),
		Property::SkullRotation(SkullRotation::Num10) => ("rotation".to_string(), "10".to_string()),
		Property::SkullRotation(SkullRotation::Num11) => ("rotation".to_string(), "11".to_string()),
		Property::SkullRotation(SkullRotation::Num12) => ("rotation".to_string(), "12".to_string()),
		Property::SkullRotation(SkullRotation::Num13) => ("rotation".to_string(), "13".to_string()),
		Property::SkullRotation(SkullRotation::Num14) => ("rotation".to_string(), "14".to_string()),
		Property::SkullRotation(SkullRotation::Num15) => ("rotation".to_string(), "15".to_string()),
		Property::SlabType(SlabType::Top) => ("type".to_string(), "top".to_string()),
		Property::SlabType(SlabType::Bottom) => ("type".to_string(), "bottom".to_string()),
		Property::SlabType(SlabType::Double) => ("type".to_string(), "double".to_string()),
		Property::SlabWaterlogged(SlabWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::SlabWaterlogged(SlabWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::SmallDripleafFacing(SmallDripleafFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::SmallDripleafFacing(SmallDripleafFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::SmallDripleafFacing(SmallDripleafFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::SmallDripleafFacing(SmallDripleafFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::SmallDripleafHalf(SmallDripleafHalf::Upper) => ("half".to_string(), "upper".to_string()),
		Property::SmallDripleafHalf(SmallDripleafHalf::Lower) => ("half".to_string(), "lower".to_string()),
		Property::SmallDripleafWaterlogged(SmallDripleafWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::SmallDripleafWaterlogged(SmallDripleafWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::SmokerFacing(SmokerFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::SmokerFacing(SmokerFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::SmokerFacing(SmokerFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::SmokerFacing(SmokerFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::SmokerLit(SmokerLit::True) => ("lit".to_string(), "true".to_string()),
		Property::SmokerLit(SmokerLit::False) => ("lit".to_string(), "false".to_string()),
		Property::SnifferEggHatch(SnifferEggHatch::Num0) => ("hatch".to_string(), "0".to_string()),
		Property::SnifferEggHatch(SnifferEggHatch::Num1) => ("hatch".to_string(), "1".to_string()),
		Property::SnifferEggHatch(SnifferEggHatch::Num2) => ("hatch".to_string(), "2".to_string()),
		Property::SnowLayerLayers(SnowLayerLayers::Num1) => ("layers".to_string(), "1".to_string()),
		Property::SnowLayerLayers(SnowLayerLayers::Num2) => ("layers".to_string(), "2".to_string()),
		Property::SnowLayerLayers(SnowLayerLayers::Num3) => ("layers".to_string(), "3".to_string()),
		Property::SnowLayerLayers(SnowLayerLayers::Num4) => ("layers".to_string(), "4".to_string()),
		Property::SnowLayerLayers(SnowLayerLayers::Num5) => ("layers".to_string(), "5".to_string()),
		Property::SnowLayerLayers(SnowLayerLayers::Num6) => ("layers".to_string(), "6".to_string()),
		Property::SnowLayerLayers(SnowLayerLayers::Num7) => ("layers".to_string(), "7".to_string()),
		Property::SnowLayerLayers(SnowLayerLayers::Num8) => ("layers".to_string(), "8".to_string()),
		Property::SnowyDirtSnowy(SnowyDirtSnowy::True) => ("snowy".to_string(), "true".to_string()),
		Property::SnowyDirtSnowy(SnowyDirtSnowy::False) => ("snowy".to_string(), "false".to_string()),
		Property::StainedGlassPaneEast(StainedGlassPaneEast::True) => ("east".to_string(), "true".to_string()),
		Property::StainedGlassPaneEast(StainedGlassPaneEast::False) => ("east".to_string(), "false".to_string()),
		Property::StainedGlassPaneNorth(StainedGlassPaneNorth::True) => ("north".to_string(), "true".to_string()),
		Property::StainedGlassPaneNorth(StainedGlassPaneNorth::False) => ("north".to_string(), "false".to_string()),
		Property::StainedGlassPaneSouth(StainedGlassPaneSouth::True) => ("south".to_string(), "true".to_string()),
		Property::StainedGlassPaneSouth(StainedGlassPaneSouth::False) => ("south".to_string(), "false".to_string()),
		Property::StainedGlassPaneWaterlogged(StainedGlassPaneWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::StainedGlassPaneWaterlogged(StainedGlassPaneWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::StainedGlassPaneWest(StainedGlassPaneWest::True) => ("west".to_string(), "true".to_string()),
		Property::StainedGlassPaneWest(StainedGlassPaneWest::False) => ("west".to_string(), "false".to_string()),
		Property::StairFacing(StairFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::StairFacing(StairFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::StairFacing(StairFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::StairFacing(StairFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::StairHalf(StairHalf::Top) => ("half".to_string(), "top".to_string()),
		Property::StairHalf(StairHalf::Bottom) => ("half".to_string(), "bottom".to_string()),
		Property::StairShape(StairShape::Straight) => ("shape".to_string(), "straight".to_string()),
		Property::StairShape(StairShape::InnerLeft) => ("shape".to_string(), "inner_left".to_string()),
		Property::StairShape(StairShape::InnerRight) => ("shape".to_string(), "inner_right".to_string()),
		Property::StairShape(StairShape::OuterLeft) => ("shape".to_string(), "outer_left".to_string()),
		Property::StairShape(StairShape::OuterRight) => ("shape".to_string(), "outer_right".to_string()),
		Property::StairWaterlogged(StairWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::StairWaterlogged(StairWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num0) => ("rotation".to_string(), "0".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num1) => ("rotation".to_string(), "1".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num2) => ("rotation".to_string(), "2".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num3) => ("rotation".to_string(), "3".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num4) => ("rotation".to_string(), "4".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num5) => ("rotation".to_string(), "5".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num6) => ("rotation".to_string(), "6".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num7) => ("rotation".to_string(), "7".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num8) => ("rotation".to_string(), "8".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num9) => ("rotation".to_string(), "9".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num10) => ("rotation".to_string(), "10".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num11) => ("rotation".to_string(), "11".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num12) => ("rotation".to_string(), "12".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num13) => ("rotation".to_string(), "13".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num14) => ("rotation".to_string(), "14".to_string()),
		Property::StandingSignRotation(StandingSignRotation::Num15) => ("rotation".to_string(), "15".to_string()),
		Property::StandingSignWaterlogged(StandingSignWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::StandingSignWaterlogged(StandingSignWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::StemAge(StemAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::StemAge(StemAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::StemAge(StemAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::StemAge(StemAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::StemAge(StemAge::Num4) => ("age".to_string(), "4".to_string()),
		Property::StemAge(StemAge::Num5) => ("age".to_string(), "5".to_string()),
		Property::StemAge(StemAge::Num6) => ("age".to_string(), "6".to_string()),
		Property::StemAge(StemAge::Num7) => ("age".to_string(), "7".to_string()),
		Property::StonecutterFacing(StonecutterFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::StonecutterFacing(StonecutterFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::StonecutterFacing(StonecutterFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::StonecutterFacing(StonecutterFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::StructureMode(StructureMode::Save) => ("mode".to_string(), "save".to_string()),
		Property::StructureMode(StructureMode::Load) => ("mode".to_string(), "load".to_string()),
		Property::StructureMode(StructureMode::Corner) => ("mode".to_string(), "corner".to_string()),
		Property::StructureMode(StructureMode::Data) => ("mode".to_string(), "data".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num4) => ("age".to_string(), "4".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num5) => ("age".to_string(), "5".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num6) => ("age".to_string(), "6".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num7) => ("age".to_string(), "7".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num8) => ("age".to_string(), "8".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num9) => ("age".to_string(), "9".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num10) => ("age".to_string(), "10".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num11) => ("age".to_string(), "11".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num12) => ("age".to_string(), "12".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num13) => ("age".to_string(), "13".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num14) => ("age".to_string(), "14".to_string()),
		Property::SugarCaneAge(SugarCaneAge::Num15) => ("age".to_string(), "15".to_string()),
		Property::SweetBerryBushAge(SweetBerryBushAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::SweetBerryBushAge(SweetBerryBushAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::SweetBerryBushAge(SweetBerryBushAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::SweetBerryBushAge(SweetBerryBushAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::TallFlowerHalf(TallFlowerHalf::Upper) => ("half".to_string(), "upper".to_string()),
		Property::TallFlowerHalf(TallFlowerHalf::Lower) => ("half".to_string(), "lower".to_string()),
		Property::TallSeagrassHalf(TallSeagrassHalf::Upper) => ("half".to_string(), "upper".to_string()),
		Property::TallSeagrassHalf(TallSeagrassHalf::Lower) => ("half".to_string(), "lower".to_string()),
		Property::TargetPower(TargetPower::Num0) => ("power".to_string(), "0".to_string()),
		Property::TargetPower(TargetPower::Num1) => ("power".to_string(), "1".to_string()),
		Property::TargetPower(TargetPower::Num2) => ("power".to_string(), "2".to_string()),
		Property::TargetPower(TargetPower::Num3) => ("power".to_string(), "3".to_string()),
		Property::TargetPower(TargetPower::Num4) => ("power".to_string(), "4".to_string()),
		Property::TargetPower(TargetPower::Num5) => ("power".to_string(), "5".to_string()),
		Property::TargetPower(TargetPower::Num6) => ("power".to_string(), "6".to_string()),
		Property::TargetPower(TargetPower::Num7) => ("power".to_string(), "7".to_string()),
		Property::TargetPower(TargetPower::Num8) => ("power".to_string(), "8".to_string()),
		Property::TargetPower(TargetPower::Num9) => ("power".to_string(), "9".to_string()),
		Property::TargetPower(TargetPower::Num10) => ("power".to_string(), "10".to_string()),
		Property::TargetPower(TargetPower::Num11) => ("power".to_string(), "11".to_string()),
		Property::TargetPower(TargetPower::Num12) => ("power".to_string(), "12".to_string()),
		Property::TargetPower(TargetPower::Num13) => ("power".to_string(), "13".to_string()),
		Property::TargetPower(TargetPower::Num14) => ("power".to_string(), "14".to_string()),
		Property::TargetPower(TargetPower::Num15) => ("power".to_string(), "15".to_string()),
		Property::TestMode(TestMode::Start) => ("mode".to_string(), "start".to_string()),
		Property::TestMode(TestMode::Log) => ("mode".to_string(), "log".to_string()),
		Property::TestMode(TestMode::Fail) => ("mode".to_string(), "fail".to_string()),
		Property::TestMode(TestMode::Accept) => ("mode".to_string(), "accept".to_string()),
		Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num1) => ("distance".to_string(), "1".to_string()),
		Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num2) => ("distance".to_string(), "2".to_string()),
		Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num3) => ("distance".to_string(), "3".to_string()),
		Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num4) => ("distance".to_string(), "4".to_string()),
		Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num5) => ("distance".to_string(), "5".to_string()),
		Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num6) => ("distance".to_string(), "6".to_string()),
		Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num7) => ("distance".to_string(), "7".to_string()),
		Property::TintedParticleLeavesPersistent(TintedParticleLeavesPersistent::True) => ("persistent".to_string(), "true".to_string()),
		Property::TintedParticleLeavesPersistent(TintedParticleLeavesPersistent::False) => ("persistent".to_string(), "false".to_string()),
		Property::TintedParticleLeavesWaterlogged(TintedParticleLeavesWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::TintedParticleLeavesWaterlogged(TintedParticleLeavesWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::TntUnstable(TntUnstable::True) => ("unstable".to_string(), "true".to_string()),
		Property::TntUnstable(TntUnstable::False) => ("unstable".to_string(), "false".to_string()),
		Property::TorchflowerCropAge(TorchflowerCropAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::TorchflowerCropAge(TorchflowerCropAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::TrapdoorFacing(TrapdoorFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::TrapdoorFacing(TrapdoorFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::TrapdoorFacing(TrapdoorFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::TrapdoorFacing(TrapdoorFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::TrapdoorHalf(TrapdoorHalf::Top) => ("half".to_string(), "top".to_string()),
		Property::TrapdoorHalf(TrapdoorHalf::Bottom) => ("half".to_string(), "bottom".to_string()),
		Property::TrapdoorOpen(TrapdoorOpen::True) => ("open".to_string(), "true".to_string()),
		Property::TrapdoorOpen(TrapdoorOpen::False) => ("open".to_string(), "false".to_string()),
		Property::TrapdoorPowered(TrapdoorPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::TrapdoorPowered(TrapdoorPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::TrapdoorWaterlogged(TrapdoorWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::TrapdoorWaterlogged(TrapdoorWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::TrappedChestFacing(TrappedChestFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::TrappedChestFacing(TrappedChestFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::TrappedChestFacing(TrappedChestFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::TrappedChestFacing(TrappedChestFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::TrappedChestType(TrappedChestType::Single) => ("type".to_string(), "single".to_string()),
		Property::TrappedChestType(TrappedChestType::Left) => ("type".to_string(), "left".to_string()),
		Property::TrappedChestType(TrappedChestType::Right) => ("type".to_string(), "right".to_string()),
		Property::TrappedChestWaterlogged(TrappedChestWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::TrappedChestWaterlogged(TrappedChestWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::TrialSpawnerOminous(TrialSpawnerOminous::True) => ("ominous".to_string(), "true".to_string()),
		Property::TrialSpawnerOminous(TrialSpawnerOminous::False) => ("ominous".to_string(), "false".to_string()),
		Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::Inactive) => ("trial_spawner_state".to_string(), "inactive".to_string()),
		Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::WaitingForPlayers) => ("trial_spawner_state".to_string(), "waiting_for_players".to_string()),
		Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::Active) => ("trial_spawner_state".to_string(), "active".to_string()),
		Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::WaitingForRewardEjection) => ("trial_spawner_state".to_string(), "waiting_for_reward_ejection".to_string()),
		Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::EjectingReward) => ("trial_spawner_state".to_string(), "ejecting_reward".to_string()),
		Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::Cooldown) => ("trial_spawner_state".to_string(), "cooldown".to_string()),
		Property::TripWireHookAttached(TripWireHookAttached::True) => ("attached".to_string(), "true".to_string()),
		Property::TripWireHookAttached(TripWireHookAttached::False) => ("attached".to_string(), "false".to_string()),
		Property::TripWireHookFacing(TripWireHookFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::TripWireHookFacing(TripWireHookFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::TripWireHookFacing(TripWireHookFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::TripWireHookFacing(TripWireHookFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::TripWireHookPowered(TripWireHookPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::TripWireHookPowered(TripWireHookPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::TripwireAttached(TripwireAttached::True) => ("attached".to_string(), "true".to_string()),
		Property::TripwireAttached(TripwireAttached::False) => ("attached".to_string(), "false".to_string()),
		Property::TripwireDisarmed(TripwireDisarmed::True) => ("disarmed".to_string(), "true".to_string()),
		Property::TripwireDisarmed(TripwireDisarmed::False) => ("disarmed".to_string(), "false".to_string()),
		Property::TripwireEast(TripwireEast::True) => ("east".to_string(), "true".to_string()),
		Property::TripwireEast(TripwireEast::False) => ("east".to_string(), "false".to_string()),
		Property::TripwireNorth(TripwireNorth::True) => ("north".to_string(), "true".to_string()),
		Property::TripwireNorth(TripwireNorth::False) => ("north".to_string(), "false".to_string()),
		Property::TripwirePowered(TripwirePowered::True) => ("powered".to_string(), "true".to_string()),
		Property::TripwirePowered(TripwirePowered::False) => ("powered".to_string(), "false".to_string()),
		Property::TripwireSouth(TripwireSouth::True) => ("south".to_string(), "true".to_string()),
		Property::TripwireSouth(TripwireSouth::False) => ("south".to_string(), "false".to_string()),
		Property::TripwireWest(TripwireWest::True) => ("west".to_string(), "true".to_string()),
		Property::TripwireWest(TripwireWest::False) => ("west".to_string(), "false".to_string()),
		Property::TurtleEggEggs(TurtleEggEggs::Num1) => ("eggs".to_string(), "1".to_string()),
		Property::TurtleEggEggs(TurtleEggEggs::Num2) => ("eggs".to_string(), "2".to_string()),
		Property::TurtleEggEggs(TurtleEggEggs::Num3) => ("eggs".to_string(), "3".to_string()),
		Property::TurtleEggEggs(TurtleEggEggs::Num4) => ("eggs".to_string(), "4".to_string()),
		Property::TurtleEggHatch(TurtleEggHatch::Num0) => ("hatch".to_string(), "0".to_string()),
		Property::TurtleEggHatch(TurtleEggHatch::Num1) => ("hatch".to_string(), "1".to_string()),
		Property::TurtleEggHatch(TurtleEggHatch::Num2) => ("hatch".to_string(), "2".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num4) => ("age".to_string(), "4".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num5) => ("age".to_string(), "5".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num6) => ("age".to_string(), "6".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num7) => ("age".to_string(), "7".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num8) => ("age".to_string(), "8".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num9) => ("age".to_string(), "9".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num10) => ("age".to_string(), "10".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num11) => ("age".to_string(), "11".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num12) => ("age".to_string(), "12".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num13) => ("age".to_string(), "13".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num14) => ("age".to_string(), "14".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num15) => ("age".to_string(), "15".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num16) => ("age".to_string(), "16".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num17) => ("age".to_string(), "17".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num18) => ("age".to_string(), "18".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num19) => ("age".to_string(), "19".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num20) => ("age".to_string(), "20".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num21) => ("age".to_string(), "21".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num22) => ("age".to_string(), "22".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num23) => ("age".to_string(), "23".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num24) => ("age".to_string(), "24".to_string()),
		Property::TwistingVinesAge(TwistingVinesAge::Num25) => ("age".to_string(), "25".to_string()),
		Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num1) => ("distance".to_string(), "1".to_string()),
		Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num2) => ("distance".to_string(), "2".to_string()),
		Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num3) => ("distance".to_string(), "3".to_string()),
		Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num4) => ("distance".to_string(), "4".to_string()),
		Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num5) => ("distance".to_string(), "5".to_string()),
		Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num6) => ("distance".to_string(), "6".to_string()),
		Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num7) => ("distance".to_string(), "7".to_string()),
		Property::UntintedParticleLeavesPersistent(UntintedParticleLeavesPersistent::True) => ("persistent".to_string(), "true".to_string()),
		Property::UntintedParticleLeavesPersistent(UntintedParticleLeavesPersistent::False) => ("persistent".to_string(), "false".to_string()),
		Property::UntintedParticleLeavesWaterlogged(UntintedParticleLeavesWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::UntintedParticleLeavesWaterlogged(UntintedParticleLeavesWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::VaultFacing(VaultFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::VaultFacing(VaultFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::VaultFacing(VaultFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::VaultFacing(VaultFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::VaultOminous(VaultOminous::True) => ("ominous".to_string(), "true".to_string()),
		Property::VaultOminous(VaultOminous::False) => ("ominous".to_string(), "false".to_string()),
		Property::VaultVaultState(VaultVaultState::Inactive) => ("vault_state".to_string(), "inactive".to_string()),
		Property::VaultVaultState(VaultVaultState::Active) => ("vault_state".to_string(), "active".to_string()),
		Property::VaultVaultState(VaultVaultState::Unlocking) => ("vault_state".to_string(), "unlocking".to_string()),
		Property::VaultVaultState(VaultVaultState::Ejecting) => ("vault_state".to_string(), "ejecting".to_string()),
		Property::VineEast(VineEast::True) => ("east".to_string(), "true".to_string()),
		Property::VineEast(VineEast::False) => ("east".to_string(), "false".to_string()),
		Property::VineNorth(VineNorth::True) => ("north".to_string(), "true".to_string()),
		Property::VineNorth(VineNorth::False) => ("north".to_string(), "false".to_string()),
		Property::VineSouth(VineSouth::True) => ("south".to_string(), "true".to_string()),
		Property::VineSouth(VineSouth::False) => ("south".to_string(), "false".to_string()),
		Property::VineUp(VineUp::True) => ("up".to_string(), "true".to_string()),
		Property::VineUp(VineUp::False) => ("up".to_string(), "false".to_string()),
		Property::VineWest(VineWest::True) => ("west".to_string(), "true".to_string()),
		Property::VineWest(VineWest::False) => ("west".to_string(), "false".to_string()),
		Property::WallEast(WallEast::None) => ("east".to_string(), "none".to_string()),
		Property::WallEast(WallEast::Low) => ("east".to_string(), "low".to_string()),
		Property::WallEast(WallEast::Tall) => ("east".to_string(), "tall".to_string()),
		Property::WallNorth(WallNorth::None) => ("north".to_string(), "none".to_string()),
		Property::WallNorth(WallNorth::Low) => ("north".to_string(), "low".to_string()),
		Property::WallNorth(WallNorth::Tall) => ("north".to_string(), "tall".to_string()),
		Property::WallSouth(WallSouth::None) => ("south".to_string(), "none".to_string()),
		Property::WallSouth(WallSouth::Low) => ("south".to_string(), "low".to_string()),
		Property::WallSouth(WallSouth::Tall) => ("south".to_string(), "tall".to_string()),
		Property::WallUp(WallUp::True) => ("up".to_string(), "true".to_string()),
		Property::WallUp(WallUp::False) => ("up".to_string(), "false".to_string()),
		Property::WallWaterlogged(WallWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::WallWaterlogged(WallWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::WallWest(WallWest::None) => ("west".to_string(), "none".to_string()),
		Property::WallWest(WallWest::Low) => ("west".to_string(), "low".to_string()),
		Property::WallWest(WallWest::Tall) => ("west".to_string(), "tall".to_string()),
		Property::WallBannerFacing(WallBannerFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::WallBannerFacing(WallBannerFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::WallBannerFacing(WallBannerFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::WallBannerFacing(WallBannerFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::WallHangingSignFacing(WallHangingSignFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::WallHangingSignFacing(WallHangingSignFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::WallHangingSignFacing(WallHangingSignFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::WallHangingSignFacing(WallHangingSignFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::WallHangingSignWaterlogged(WallHangingSignWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::WallHangingSignWaterlogged(WallHangingSignWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::WallSignFacing(WallSignFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::WallSignFacing(WallSignFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::WallSignFacing(WallSignFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::WallSignFacing(WallSignFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::WallSignWaterlogged(WallSignWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::WallSignWaterlogged(WallSignWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::WallSkullFacing(WallSkullFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::WallSkullFacing(WallSkullFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::WallSkullFacing(WallSkullFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::WallSkullFacing(WallSkullFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::WallSkullPowered(WallSkullPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::WallSkullPowered(WallSkullPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::WallTorchFacing(WallTorchFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::WallTorchFacing(WallTorchFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::WallTorchFacing(WallTorchFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::WallTorchFacing(WallTorchFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::WaterloggedTransparentWaterlogged(WaterloggedTransparentWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::WaterloggedTransparentWaterlogged(WaterloggedTransparentWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::WeatheringCopperBarEast(WeatheringCopperBarEast::True) => ("east".to_string(), "true".to_string()),
		Property::WeatheringCopperBarEast(WeatheringCopperBarEast::False) => ("east".to_string(), "false".to_string()),
		Property::WeatheringCopperBarNorth(WeatheringCopperBarNorth::True) => ("north".to_string(), "true".to_string()),
		Property::WeatheringCopperBarNorth(WeatheringCopperBarNorth::False) => ("north".to_string(), "false".to_string()),
		Property::WeatheringCopperBarSouth(WeatheringCopperBarSouth::True) => ("south".to_string(), "true".to_string()),
		Property::WeatheringCopperBarSouth(WeatheringCopperBarSouth::False) => ("south".to_string(), "false".to_string()),
		Property::WeatheringCopperBarWaterlogged(WeatheringCopperBarWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::WeatheringCopperBarWaterlogged(WeatheringCopperBarWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::WeatheringCopperBarWest(WeatheringCopperBarWest::True) => ("west".to_string(), "true".to_string()),
		Property::WeatheringCopperBarWest(WeatheringCopperBarWest::False) => ("west".to_string(), "false".to_string()),
		Property::WeatheringCopperBulbLit(WeatheringCopperBulbLit::True) => ("lit".to_string(), "true".to_string()),
		Property::WeatheringCopperBulbLit(WeatheringCopperBulbLit::False) => ("lit".to_string(), "false".to_string()),
		Property::WeatheringCopperBulbPowered(WeatheringCopperBulbPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::WeatheringCopperBulbPowered(WeatheringCopperBulbPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::WeatheringCopperChainAxis(WeatheringCopperChainAxis::X) => ("axis".to_string(), "x".to_string()),
		Property::WeatheringCopperChainAxis(WeatheringCopperChainAxis::Y) => ("axis".to_string(), "y".to_string()),
		Property::WeatheringCopperChainAxis(WeatheringCopperChainAxis::Z) => ("axis".to_string(), "z".to_string()),
		Property::WeatheringCopperChainWaterlogged(WeatheringCopperChainWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::WeatheringCopperChainWaterlogged(WeatheringCopperChainWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::WeatheringCopperChestFacing(WeatheringCopperChestFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::WeatheringCopperChestFacing(WeatheringCopperChestFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::WeatheringCopperChestFacing(WeatheringCopperChestFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::WeatheringCopperChestFacing(WeatheringCopperChestFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::WeatheringCopperChestType(WeatheringCopperChestType::Single) => ("type".to_string(), "single".to_string()),
		Property::WeatheringCopperChestType(WeatheringCopperChestType::Left) => ("type".to_string(), "left".to_string()),
		Property::WeatheringCopperChestType(WeatheringCopperChestType::Right) => ("type".to_string(), "right".to_string()),
		Property::WeatheringCopperChestWaterlogged(WeatheringCopperChestWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::WeatheringCopperChestWaterlogged(WeatheringCopperChestWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::WeatheringCopperDoorHalf(WeatheringCopperDoorHalf::Upper) => ("half".to_string(), "upper".to_string()),
		Property::WeatheringCopperDoorHalf(WeatheringCopperDoorHalf::Lower) => ("half".to_string(), "lower".to_string()),
		Property::WeatheringCopperDoorHinge(WeatheringCopperDoorHinge::Left) => ("hinge".to_string(), "left".to_string()),
		Property::WeatheringCopperDoorHinge(WeatheringCopperDoorHinge::Right) => ("hinge".to_string(), "right".to_string()),
		Property::WeatheringCopperDoorOpen(WeatheringCopperDoorOpen::True) => ("open".to_string(), "true".to_string()),
		Property::WeatheringCopperDoorOpen(WeatheringCopperDoorOpen::False) => ("open".to_string(), "false".to_string()),
		Property::WeatheringCopperDoorPowered(WeatheringCopperDoorPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::WeatheringCopperDoorPowered(WeatheringCopperDoorPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Standing) => ("copper_golem_pose".to_string(), "standing".to_string()),
		Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Sitting) => ("copper_golem_pose".to_string(), "sitting".to_string()),
		Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Running) => ("copper_golem_pose".to_string(), "running".to_string()),
		Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Star) => ("copper_golem_pose".to_string(), "star".to_string()),
		Property::WeatheringCopperGolemStatueFacing(WeatheringCopperGolemStatueFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::WeatheringCopperGolemStatueFacing(WeatheringCopperGolemStatueFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::WeatheringCopperGolemStatueFacing(WeatheringCopperGolemStatueFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::WeatheringCopperGolemStatueFacing(WeatheringCopperGolemStatueFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::WeatheringCopperGolemStatueWaterlogged(WeatheringCopperGolemStatueWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::WeatheringCopperGolemStatueWaterlogged(WeatheringCopperGolemStatueWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::WeatheringCopperGrateWaterlogged(WeatheringCopperGrateWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::WeatheringCopperGrateWaterlogged(WeatheringCopperGrateWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::WeatheringCopperSlabType(WeatheringCopperSlabType::Top) => ("type".to_string(), "top".to_string()),
		Property::WeatheringCopperSlabType(WeatheringCopperSlabType::Bottom) => ("type".to_string(), "bottom".to_string()),
		Property::WeatheringCopperSlabType(WeatheringCopperSlabType::Double) => ("type".to_string(), "double".to_string()),
		Property::WeatheringCopperSlabWaterlogged(WeatheringCopperSlabWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::WeatheringCopperSlabWaterlogged(WeatheringCopperSlabWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::WeatheringCopperStairFacing(WeatheringCopperStairFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::WeatheringCopperStairFacing(WeatheringCopperStairFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::WeatheringCopperStairFacing(WeatheringCopperStairFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::WeatheringCopperStairFacing(WeatheringCopperStairFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::WeatheringCopperStairHalf(WeatheringCopperStairHalf::Top) => ("half".to_string(), "top".to_string()),
		Property::WeatheringCopperStairHalf(WeatheringCopperStairHalf::Bottom) => ("half".to_string(), "bottom".to_string()),
		Property::WeatheringCopperStairShape(WeatheringCopperStairShape::Straight) => ("shape".to_string(), "straight".to_string()),
		Property::WeatheringCopperStairShape(WeatheringCopperStairShape::InnerLeft) => ("shape".to_string(), "inner_left".to_string()),
		Property::WeatheringCopperStairShape(WeatheringCopperStairShape::InnerRight) => ("shape".to_string(), "inner_right".to_string()),
		Property::WeatheringCopperStairShape(WeatheringCopperStairShape::OuterLeft) => ("shape".to_string(), "outer_left".to_string()),
		Property::WeatheringCopperStairShape(WeatheringCopperStairShape::OuterRight) => ("shape".to_string(), "outer_right".to_string()),
		Property::WeatheringCopperStairWaterlogged(WeatheringCopperStairWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::WeatheringCopperStairWaterlogged(WeatheringCopperStairWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::WeatheringCopperTrapDoorHalf(WeatheringCopperTrapDoorHalf::Top) => ("half".to_string(), "top".to_string()),
		Property::WeatheringCopperTrapDoorHalf(WeatheringCopperTrapDoorHalf::Bottom) => ("half".to_string(), "bottom".to_string()),
		Property::WeatheringCopperTrapDoorOpen(WeatheringCopperTrapDoorOpen::True) => ("open".to_string(), "true".to_string()),
		Property::WeatheringCopperTrapDoorOpen(WeatheringCopperTrapDoorOpen::False) => ("open".to_string(), "false".to_string()),
		Property::WeatheringCopperTrapDoorPowered(WeatheringCopperTrapDoorPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::WeatheringCopperTrapDoorPowered(WeatheringCopperTrapDoorPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::WeatheringCopperTrapDoorWaterlogged(WeatheringCopperTrapDoorWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::WeatheringCopperTrapDoorWaterlogged(WeatheringCopperTrapDoorWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::WeatheringLanternHanging(WeatheringLanternHanging::True) => ("hanging".to_string(), "true".to_string()),
		Property::WeatheringLanternHanging(WeatheringLanternHanging::False) => ("hanging".to_string(), "false".to_string()),
		Property::WeatheringLanternWaterlogged(WeatheringLanternWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::WeatheringLanternWaterlogged(WeatheringLanternWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::Up) => ("facing".to_string(), "up".to_string()),
		Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::Down) => ("facing".to_string(), "down".to_string()),
		Property::WeatheringLightningRodPowered(WeatheringLightningRodPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::WeatheringLightningRodPowered(WeatheringLightningRodPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::WeatheringLightningRodWaterlogged(WeatheringLightningRodWaterlogged::True) => ("waterlogged".to_string(), "true".to_string()),
		Property::WeatheringLightningRodWaterlogged(WeatheringLightningRodWaterlogged::False) => ("waterlogged".to_string(), "false".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num0) => ("age".to_string(), "0".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num1) => ("age".to_string(), "1".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num2) => ("age".to_string(), "2".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num3) => ("age".to_string(), "3".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num4) => ("age".to_string(), "4".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num5) => ("age".to_string(), "5".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num6) => ("age".to_string(), "6".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num7) => ("age".to_string(), "7".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num8) => ("age".to_string(), "8".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num9) => ("age".to_string(), "9".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num10) => ("age".to_string(), "10".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num11) => ("age".to_string(), "11".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num12) => ("age".to_string(), "12".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num13) => ("age".to_string(), "13".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num14) => ("age".to_string(), "14".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num15) => ("age".to_string(), "15".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num16) => ("age".to_string(), "16".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num17) => ("age".to_string(), "17".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num18) => ("age".to_string(), "18".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num19) => ("age".to_string(), "19".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num20) => ("age".to_string(), "20".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num21) => ("age".to_string(), "21".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num22) => ("age".to_string(), "22".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num23) => ("age".to_string(), "23".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num24) => ("age".to_string(), "24".to_string()),
		Property::WeepingVinesAge(WeepingVinesAge::Num25) => ("age".to_string(), "25".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num0) => ("power".to_string(), "0".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num1) => ("power".to_string(), "1".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num2) => ("power".to_string(), "2".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num3) => ("power".to_string(), "3".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num4) => ("power".to_string(), "4".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num5) => ("power".to_string(), "5".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num6) => ("power".to_string(), "6".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num7) => ("power".to_string(), "7".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num8) => ("power".to_string(), "8".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num9) => ("power".to_string(), "9".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num10) => ("power".to_string(), "10".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num11) => ("power".to_string(), "11".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num12) => ("power".to_string(), "12".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num13) => ("power".to_string(), "13".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num14) => ("power".to_string(), "14".to_string()),
		Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num15) => ("power".to_string(), "15".to_string()),
		Property::WitherSkullPowered(WitherSkullPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::WitherSkullPowered(WitherSkullPowered::False) => ("powered".to_string(), "false".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num0) => ("rotation".to_string(), "0".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num1) => ("rotation".to_string(), "1".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num2) => ("rotation".to_string(), "2".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num3) => ("rotation".to_string(), "3".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num4) => ("rotation".to_string(), "4".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num5) => ("rotation".to_string(), "5".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num6) => ("rotation".to_string(), "6".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num7) => ("rotation".to_string(), "7".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num8) => ("rotation".to_string(), "8".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num9) => ("rotation".to_string(), "9".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num10) => ("rotation".to_string(), "10".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num11) => ("rotation".to_string(), "11".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num12) => ("rotation".to_string(), "12".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num13) => ("rotation".to_string(), "13".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num14) => ("rotation".to_string(), "14".to_string()),
		Property::WitherSkullRotation(WitherSkullRotation::Num15) => ("rotation".to_string(), "15".to_string()),
		Property::WitherWallSkullFacing(WitherWallSkullFacing::North) => ("facing".to_string(), "north".to_string()),
		Property::WitherWallSkullFacing(WitherWallSkullFacing::South) => ("facing".to_string(), "south".to_string()),
		Property::WitherWallSkullFacing(WitherWallSkullFacing::West) => ("facing".to_string(), "west".to_string()),
		Property::WitherWallSkullFacing(WitherWallSkullFacing::East) => ("facing".to_string(), "east".to_string()),
		Property::WitherWallSkullPowered(WitherWallSkullPowered::True) => ("powered".to_string(), "true".to_string()),
		Property::WitherWallSkullPowered(WitherWallSkullPowered::False) => ("powered".to_string(), "false".to_string()),
	}
}
pub fn get_block_state_id_from_raw(block_states: &HashMap<String, Block>, block_name: &str, properties: &[(String, String)]) -> u16 {
	let Some(block) = block_states.get(block_name) else {
		println!("get_block_state_id_from_raw couldnt find block {block_name} with properties {properties:?}");
		return 0;
	};
	return match block.block_type {
		Type::Air => {
			block.states.first().unwrap().id
		},
		Type::Amethyst => {
			block.states.first().unwrap().id
		},
		Type::AmethystCluster => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::AmethystClusterFacing(AmethystClusterFacing::North)),
				"east" => x.properties.contains(&Property::AmethystClusterFacing(AmethystClusterFacing::East)),
				"south" => x.properties.contains(&Property::AmethystClusterFacing(AmethystClusterFacing::South)),
				"west" => x.properties.contains(&Property::AmethystClusterFacing(AmethystClusterFacing::West)),
				"up" => x.properties.contains(&Property::AmethystClusterFacing(AmethystClusterFacing::Up)),
				"down" => x.properties.contains(&Property::AmethystClusterFacing(AmethystClusterFacing::Down)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::AmethystClusterWaterlogged(AmethystClusterWaterlogged::True)),
				"false" => x.properties.contains(&Property::AmethystClusterWaterlogged(AmethystClusterWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Anvil => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::AnvilFacing(AnvilFacing::North)),
				"south" => x.properties.contains(&Property::AnvilFacing(AnvilFacing::South)),
				"west" => x.properties.contains(&Property::AnvilFacing(AnvilFacing::West)),
				"east" => x.properties.contains(&Property::AnvilFacing(AnvilFacing::East)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::AttachedStem => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::AttachedStemFacing(AttachedStemFacing::North)),
				"south" => x.properties.contains(&Property::AttachedStemFacing(AttachedStemFacing::South)),
				"west" => x.properties.contains(&Property::AttachedStemFacing(AttachedStemFacing::West)),
				"east" => x.properties.contains(&Property::AttachedStemFacing(AttachedStemFacing::East)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Azalea => {
			block.states.first().unwrap().id
		},
		Type::BambooSapling => {
			block.states.first().unwrap().id
		},
		Type::BambooStalk => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::BambooStalkAge(BambooStalkAge::Num0)),
				"1" => x.properties.contains(&Property::BambooStalkAge(BambooStalkAge::Num1)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "leaves").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::BambooStalkLeaves(BambooStalkLeaves::None)),
				"small" => x.properties.contains(&Property::BambooStalkLeaves(BambooStalkLeaves::Small)),
				"large" => x.properties.contains(&Property::BambooStalkLeaves(BambooStalkLeaves::Large)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "stage").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::BambooStalkStage(BambooStalkStage::Num0)),
				"1" => x.properties.contains(&Property::BambooStalkStage(BambooStalkStage::Num1)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Banner => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "rotation").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num0)),
				"1" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num1)),
				"2" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num2)),
				"3" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num3)),
				"4" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num4)),
				"5" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num5)),
				"6" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num6)),
				"7" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num7)),
				"8" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num8)),
				"9" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num9)),
				"10" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num10)),
				"11" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num11)),
				"12" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num12)),
				"13" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num13)),
				"14" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num14)),
				"15" => x.properties.contains(&Property::BannerRotation(BannerRotation::Num15)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Barrel => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::BarrelFacing(BarrelFacing::North)),
				"east" => x.properties.contains(&Property::BarrelFacing(BarrelFacing::East)),
				"south" => x.properties.contains(&Property::BarrelFacing(BarrelFacing::South)),
				"west" => x.properties.contains(&Property::BarrelFacing(BarrelFacing::West)),
				"up" => x.properties.contains(&Property::BarrelFacing(BarrelFacing::Up)),
				"down" => x.properties.contains(&Property::BarrelFacing(BarrelFacing::Down)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "open").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BarrelOpen(BarrelOpen::True)),
				"false" => x.properties.contains(&Property::BarrelOpen(BarrelOpen::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Barrier => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BarrierWaterlogged(BarrierWaterlogged::True)),
				"false" => x.properties.contains(&Property::BarrierWaterlogged(BarrierWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::BaseCoralFan => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BaseCoralFanWaterlogged(BaseCoralFanWaterlogged::True)),
				"false" => x.properties.contains(&Property::BaseCoralFanWaterlogged(BaseCoralFanWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::BaseCoralPlant => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BaseCoralPlantWaterlogged(BaseCoralPlantWaterlogged::True)),
				"false" => x.properties.contains(&Property::BaseCoralPlantWaterlogged(BaseCoralPlantWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::BaseCoralWallFan => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::BaseCoralWallFanFacing(BaseCoralWallFanFacing::North)),
				"south" => x.properties.contains(&Property::BaseCoralWallFanFacing(BaseCoralWallFanFacing::South)),
				"west" => x.properties.contains(&Property::BaseCoralWallFanFacing(BaseCoralWallFanFacing::West)),
				"east" => x.properties.contains(&Property::BaseCoralWallFanFacing(BaseCoralWallFanFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BaseCoralWallFanWaterlogged(BaseCoralWallFanWaterlogged::True)),
				"false" => x.properties.contains(&Property::BaseCoralWallFanWaterlogged(BaseCoralWallFanWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Beacon => {
			block.states.first().unwrap().id
		},
		Type::Bed => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::BedFacing(BedFacing::North)),
				"south" => x.properties.contains(&Property::BedFacing(BedFacing::South)),
				"west" => x.properties.contains(&Property::BedFacing(BedFacing::West)),
				"east" => x.properties.contains(&Property::BedFacing(BedFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "occupied").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BedOccupied(BedOccupied::True)),
				"false" => x.properties.contains(&Property::BedOccupied(BedOccupied::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "part").unwrap().1.as_str() {
				"head" => x.properties.contains(&Property::BedPart(BedPart::Head)),
				"foot" => x.properties.contains(&Property::BedPart(BedPart::Foot)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Beehive => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::BeehiveFacing(BeehiveFacing::North)),
				"south" => x.properties.contains(&Property::BeehiveFacing(BeehiveFacing::South)),
				"west" => x.properties.contains(&Property::BeehiveFacing(BeehiveFacing::West)),
				"east" => x.properties.contains(&Property::BeehiveFacing(BeehiveFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "honey_level").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num0)),
				"1" => x.properties.contains(&Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num1)),
				"2" => x.properties.contains(&Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num2)),
				"3" => x.properties.contains(&Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num3)),
				"4" => x.properties.contains(&Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num4)),
				"5" => x.properties.contains(&Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num5)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Beetroot => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::BeetrootAge(BeetrootAge::Num0)),
				"1" => x.properties.contains(&Property::BeetrootAge(BeetrootAge::Num1)),
				"2" => x.properties.contains(&Property::BeetrootAge(BeetrootAge::Num2)),
				"3" => x.properties.contains(&Property::BeetrootAge(BeetrootAge::Num3)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Bell => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "attachment").unwrap().1.as_str() {
				"floor" => x.properties.contains(&Property::BellAttachment(BellAttachment::Floor)),
				"ceiling" => x.properties.contains(&Property::BellAttachment(BellAttachment::Ceiling)),
				"single_wall" => x.properties.contains(&Property::BellAttachment(BellAttachment::SingleWall)),
				"double_wall" => x.properties.contains(&Property::BellAttachment(BellAttachment::DoubleWall)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::BellFacing(BellFacing::North)),
				"south" => x.properties.contains(&Property::BellFacing(BellFacing::South)),
				"west" => x.properties.contains(&Property::BellFacing(BellFacing::West)),
				"east" => x.properties.contains(&Property::BellFacing(BellFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BellPowered(BellPowered::True)),
				"false" => x.properties.contains(&Property::BellPowered(BellPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::BigDripleaf => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::BigDripleafFacing(BigDripleafFacing::North)),
				"south" => x.properties.contains(&Property::BigDripleafFacing(BigDripleafFacing::South)),
				"west" => x.properties.contains(&Property::BigDripleafFacing(BigDripleafFacing::West)),
				"east" => x.properties.contains(&Property::BigDripleafFacing(BigDripleafFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "tilt").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::BigDripleafTilt(BigDripleafTilt::None)),
				"unstable" => x.properties.contains(&Property::BigDripleafTilt(BigDripleafTilt::Unstable)),
				"partial" => x.properties.contains(&Property::BigDripleafTilt(BigDripleafTilt::Partial)),
				"full" => x.properties.contains(&Property::BigDripleafTilt(BigDripleafTilt::Full)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BigDripleafWaterlogged(BigDripleafWaterlogged::True)),
				"false" => x.properties.contains(&Property::BigDripleafWaterlogged(BigDripleafWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::BigDripleafStem => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::BigDripleafStemFacing(BigDripleafStemFacing::North)),
				"south" => x.properties.contains(&Property::BigDripleafStemFacing(BigDripleafStemFacing::South)),
				"west" => x.properties.contains(&Property::BigDripleafStemFacing(BigDripleafStemFacing::West)),
				"east" => x.properties.contains(&Property::BigDripleafStemFacing(BigDripleafStemFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BigDripleafStemWaterlogged(BigDripleafStemWaterlogged::True)),
				"false" => x.properties.contains(&Property::BigDripleafStemWaterlogged(BigDripleafStemWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::BlastFurnace => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::BlastFurnaceFacing(BlastFurnaceFacing::North)),
				"south" => x.properties.contains(&Property::BlastFurnaceFacing(BlastFurnaceFacing::South)),
				"west" => x.properties.contains(&Property::BlastFurnaceFacing(BlastFurnaceFacing::West)),
				"east" => x.properties.contains(&Property::BlastFurnaceFacing(BlastFurnaceFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BlastFurnaceLit(BlastFurnaceLit::True)),
				"false" => x.properties.contains(&Property::BlastFurnaceLit(BlastFurnaceLit::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Block => {
			block.states.first().unwrap().id
		},
		Type::BonemealableFeaturePlacer => {
			block.states.first().unwrap().id
		},
		Type::Bottom => {
			block.states.first().unwrap().id
		},
		Type::BrewingStand => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "has_bottle_0").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BrewingStandHasBottle0(BrewingStandHasBottle0::True)),
				"false" => x.properties.contains(&Property::BrewingStandHasBottle0(BrewingStandHasBottle0::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "has_bottle_1").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BrewingStandHasBottle1(BrewingStandHasBottle1::True)),
				"false" => x.properties.contains(&Property::BrewingStandHasBottle1(BrewingStandHasBottle1::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "has_bottle_2").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BrewingStandHasBottle2(BrewingStandHasBottle2::True)),
				"false" => x.properties.contains(&Property::BrewingStandHasBottle2(BrewingStandHasBottle2::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Brushable => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "dusted").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::BrushableDusted(BrushableDusted::Num0)),
				"1" => x.properties.contains(&Property::BrushableDusted(BrushableDusted::Num1)),
				"2" => x.properties.contains(&Property::BrushableDusted(BrushableDusted::Num2)),
				"3" => x.properties.contains(&Property::BrushableDusted(BrushableDusted::Num3)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::BubbleColumn => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "drag").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BubbleColumnDrag(BubbleColumnDrag::True)),
				"false" => x.properties.contains(&Property::BubbleColumnDrag(BubbleColumnDrag::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::BuddingAmethyst => {
			block.states.first().unwrap().id
		},
		Type::Bush => {
			block.states.first().unwrap().id
		},
		Type::Button => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "face").unwrap().1.as_str() {
				"floor" => x.properties.contains(&Property::ButtonFace(ButtonFace::Floor)),
				"wall" => x.properties.contains(&Property::ButtonFace(ButtonFace::Wall)),
				"ceiling" => x.properties.contains(&Property::ButtonFace(ButtonFace::Ceiling)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::ButtonFacing(ButtonFacing::North)),
				"south" => x.properties.contains(&Property::ButtonFacing(ButtonFacing::South)),
				"west" => x.properties.contains(&Property::ButtonFacing(ButtonFacing::West)),
				"east" => x.properties.contains(&Property::ButtonFacing(ButtonFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ButtonPowered(ButtonPowered::True)),
				"false" => x.properties.contains(&Property::ButtonPowered(ButtonPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Cactus => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::CactusAge(CactusAge::Num0)),
				"1" => x.properties.contains(&Property::CactusAge(CactusAge::Num1)),
				"2" => x.properties.contains(&Property::CactusAge(CactusAge::Num2)),
				"3" => x.properties.contains(&Property::CactusAge(CactusAge::Num3)),
				"4" => x.properties.contains(&Property::CactusAge(CactusAge::Num4)),
				"5" => x.properties.contains(&Property::CactusAge(CactusAge::Num5)),
				"6" => x.properties.contains(&Property::CactusAge(CactusAge::Num6)),
				"7" => x.properties.contains(&Property::CactusAge(CactusAge::Num7)),
				"8" => x.properties.contains(&Property::CactusAge(CactusAge::Num8)),
				"9" => x.properties.contains(&Property::CactusAge(CactusAge::Num9)),
				"10" => x.properties.contains(&Property::CactusAge(CactusAge::Num10)),
				"11" => x.properties.contains(&Property::CactusAge(CactusAge::Num11)),
				"12" => x.properties.contains(&Property::CactusAge(CactusAge::Num12)),
				"13" => x.properties.contains(&Property::CactusAge(CactusAge::Num13)),
				"14" => x.properties.contains(&Property::CactusAge(CactusAge::Num14)),
				"15" => x.properties.contains(&Property::CactusAge(CactusAge::Num15)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::CactusFlower => {
			block.states.first().unwrap().id
		},
		Type::Cake => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "bites").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::CakeBites(CakeBites::Num0)),
				"1" => x.properties.contains(&Property::CakeBites(CakeBites::Num1)),
				"2" => x.properties.contains(&Property::CakeBites(CakeBites::Num2)),
				"3" => x.properties.contains(&Property::CakeBites(CakeBites::Num3)),
				"4" => x.properties.contains(&Property::CakeBites(CakeBites::Num4)),
				"5" => x.properties.contains(&Property::CakeBites(CakeBites::Num5)),
				"6" => x.properties.contains(&Property::CakeBites(CakeBites::Num6)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::CalibratedSculkSensor => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::North)),
				"south" => x.properties.contains(&Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::South)),
				"west" => x.properties.contains(&Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::West)),
				"east" => x.properties.contains(&Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "power").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num0)),
				"1" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num1)),
				"2" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num2)),
				"3" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num3)),
				"4" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num4)),
				"5" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num5)),
				"6" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num6)),
				"7" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num7)),
				"8" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num8)),
				"9" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num9)),
				"10" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num10)),
				"11" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num11)),
				"12" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num12)),
				"13" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num13)),
				"14" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num14)),
				"15" => x.properties.contains(&Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num15)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "sculk_sensor_phase").unwrap().1.as_str() {
				"inactive" => x.properties.contains(&Property::CalibratedSculkSensorSculkSensorPhase(CalibratedSculkSensorSculkSensorPhase::Inactive)),
				"active" => x.properties.contains(&Property::CalibratedSculkSensorSculkSensorPhase(CalibratedSculkSensorSculkSensorPhase::Active)),
				"cooldown" => x.properties.contains(&Property::CalibratedSculkSensorSculkSensorPhase(CalibratedSculkSensorSculkSensorPhase::Cooldown)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CalibratedSculkSensorWaterlogged(CalibratedSculkSensorWaterlogged::True)),
				"false" => x.properties.contains(&Property::CalibratedSculkSensorWaterlogged(CalibratedSculkSensorWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Campfire => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::CampfireFacing(CampfireFacing::North)),
				"south" => x.properties.contains(&Property::CampfireFacing(CampfireFacing::South)),
				"west" => x.properties.contains(&Property::CampfireFacing(CampfireFacing::West)),
				"east" => x.properties.contains(&Property::CampfireFacing(CampfireFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CampfireLit(CampfireLit::True)),
				"false" => x.properties.contains(&Property::CampfireLit(CampfireLit::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "signal_fire").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CampfireSignalFire(CampfireSignalFire::True)),
				"false" => x.properties.contains(&Property::CampfireSignalFire(CampfireSignalFire::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CampfireWaterlogged(CampfireWaterlogged::True)),
				"false" => x.properties.contains(&Property::CampfireWaterlogged(CampfireWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Candle => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "candles").unwrap().1.as_str() {
				"1" => x.properties.contains(&Property::CandleCandles(CandleCandles::Num1)),
				"2" => x.properties.contains(&Property::CandleCandles(CandleCandles::Num2)),
				"3" => x.properties.contains(&Property::CandleCandles(CandleCandles::Num3)),
				"4" => x.properties.contains(&Property::CandleCandles(CandleCandles::Num4)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CandleLit(CandleLit::True)),
				"false" => x.properties.contains(&Property::CandleLit(CandleLit::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CandleWaterlogged(CandleWaterlogged::True)),
				"false" => x.properties.contains(&Property::CandleWaterlogged(CandleWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::CandleCake => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CandleCakeLit(CandleCakeLit::True)),
				"false" => x.properties.contains(&Property::CandleCakeLit(CandleCakeLit::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Carpet => {
			block.states.first().unwrap().id
		},
		Type::Carrot => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::CarrotAge(CarrotAge::Num0)),
				"1" => x.properties.contains(&Property::CarrotAge(CarrotAge::Num1)),
				"2" => x.properties.contains(&Property::CarrotAge(CarrotAge::Num2)),
				"3" => x.properties.contains(&Property::CarrotAge(CarrotAge::Num3)),
				"4" => x.properties.contains(&Property::CarrotAge(CarrotAge::Num4)),
				"5" => x.properties.contains(&Property::CarrotAge(CarrotAge::Num5)),
				"6" => x.properties.contains(&Property::CarrotAge(CarrotAge::Num6)),
				"7" => x.properties.contains(&Property::CarrotAge(CarrotAge::Num7)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::CartographyTable => {
			block.states.first().unwrap().id
		},
		Type::Cauldron => {
			block.states.first().unwrap().id
		},
		Type::CaveVines => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num0)),
				"1" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num1)),
				"2" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num2)),
				"3" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num3)),
				"4" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num4)),
				"5" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num5)),
				"6" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num6)),
				"7" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num7)),
				"8" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num8)),
				"9" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num9)),
				"10" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num10)),
				"11" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num11)),
				"12" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num12)),
				"13" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num13)),
				"14" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num14)),
				"15" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num15)),
				"16" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num16)),
				"17" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num17)),
				"18" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num18)),
				"19" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num19)),
				"20" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num20)),
				"21" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num21)),
				"22" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num22)),
				"23" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num23)),
				"24" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num24)),
				"25" => x.properties.contains(&Property::CaveVinesAge(CaveVinesAge::Num25)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "berries").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CaveVinesBerries(CaveVinesBerries::True)),
				"false" => x.properties.contains(&Property::CaveVinesBerries(CaveVinesBerries::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::CaveVinesPlant => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "berries").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CaveVinesPlantBerries(CaveVinesPlantBerries::True)),
				"false" => x.properties.contains(&Property::CaveVinesPlantBerries(CaveVinesPlantBerries::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::CeilingHangingSign => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "attached").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CeilingHangingSignAttached(CeilingHangingSignAttached::True)),
				"false" => x.properties.contains(&Property::CeilingHangingSignAttached(CeilingHangingSignAttached::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "rotation").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num0)),
				"1" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num1)),
				"2" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num2)),
				"3" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num3)),
				"4" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num4)),
				"5" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num5)),
				"6" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num6)),
				"7" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num7)),
				"8" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num8)),
				"9" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num9)),
				"10" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num10)),
				"11" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num11)),
				"12" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num12)),
				"13" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num13)),
				"14" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num14)),
				"15" => x.properties.contains(&Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num15)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CeilingHangingSignWaterlogged(CeilingHangingSignWaterlogged::True)),
				"false" => x.properties.contains(&Property::CeilingHangingSignWaterlogged(CeilingHangingSignWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Chain => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "axis").unwrap().1.as_str() {
				"x" => x.properties.contains(&Property::ChainAxis(ChainAxis::X)),
				"y" => x.properties.contains(&Property::ChainAxis(ChainAxis::Y)),
				"z" => x.properties.contains(&Property::ChainAxis(ChainAxis::Z)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChainWaterlogged(ChainWaterlogged::True)),
				"false" => x.properties.contains(&Property::ChainWaterlogged(ChainWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::CherryLeaves => {
			block.states.first().unwrap().id
		},
		Type::Chest => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::ChestFacing(ChestFacing::North)),
				"south" => x.properties.contains(&Property::ChestFacing(ChestFacing::South)),
				"west" => x.properties.contains(&Property::ChestFacing(ChestFacing::West)),
				"east" => x.properties.contains(&Property::ChestFacing(ChestFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "type").unwrap().1.as_str() {
				"single" => x.properties.contains(&Property::ChestType(ChestType::Single)),
				"left" => x.properties.contains(&Property::ChestType(ChestType::Left)),
				"right" => x.properties.contains(&Property::ChestType(ChestType::Right)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChestWaterlogged(ChestWaterlogged::True)),
				"false" => x.properties.contains(&Property::ChestWaterlogged(ChestWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::ChiseledBookShelf => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::North)),
				"south" => x.properties.contains(&Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::South)),
				"west" => x.properties.contains(&Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::West)),
				"east" => x.properties.contains(&Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "slot_0_occupied").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChiseledBookShelfSlot0Occupied(ChiseledBookShelfSlot0Occupied::True)),
				"false" => x.properties.contains(&Property::ChiseledBookShelfSlot0Occupied(ChiseledBookShelfSlot0Occupied::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "slot_1_occupied").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChiseledBookShelfSlot1Occupied(ChiseledBookShelfSlot1Occupied::True)),
				"false" => x.properties.contains(&Property::ChiseledBookShelfSlot1Occupied(ChiseledBookShelfSlot1Occupied::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "slot_2_occupied").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChiseledBookShelfSlot2Occupied(ChiseledBookShelfSlot2Occupied::True)),
				"false" => x.properties.contains(&Property::ChiseledBookShelfSlot2Occupied(ChiseledBookShelfSlot2Occupied::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "slot_3_occupied").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChiseledBookShelfSlot3Occupied(ChiseledBookShelfSlot3Occupied::True)),
				"false" => x.properties.contains(&Property::ChiseledBookShelfSlot3Occupied(ChiseledBookShelfSlot3Occupied::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "slot_4_occupied").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChiseledBookShelfSlot4Occupied(ChiseledBookShelfSlot4Occupied::True)),
				"false" => x.properties.contains(&Property::ChiseledBookShelfSlot4Occupied(ChiseledBookShelfSlot4Occupied::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "slot_5_occupied").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChiseledBookShelfSlot5Occupied(ChiseledBookShelfSlot5Occupied::True)),
				"false" => x.properties.contains(&Property::ChiseledBookShelfSlot5Occupied(ChiseledBookShelfSlot5Occupied::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::ChorusFlower => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::ChorusFlowerAge(ChorusFlowerAge::Num0)),
				"1" => x.properties.contains(&Property::ChorusFlowerAge(ChorusFlowerAge::Num1)),
				"2" => x.properties.contains(&Property::ChorusFlowerAge(ChorusFlowerAge::Num2)),
				"3" => x.properties.contains(&Property::ChorusFlowerAge(ChorusFlowerAge::Num3)),
				"4" => x.properties.contains(&Property::ChorusFlowerAge(ChorusFlowerAge::Num4)),
				"5" => x.properties.contains(&Property::ChorusFlowerAge(ChorusFlowerAge::Num5)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::ChorusPlant => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "down").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChorusPlantDown(ChorusPlantDown::True)),
				"false" => x.properties.contains(&Property::ChorusPlantDown(ChorusPlantDown::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChorusPlantEast(ChorusPlantEast::True)),
				"false" => x.properties.contains(&Property::ChorusPlantEast(ChorusPlantEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChorusPlantNorth(ChorusPlantNorth::True)),
				"false" => x.properties.contains(&Property::ChorusPlantNorth(ChorusPlantNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChorusPlantSouth(ChorusPlantSouth::True)),
				"false" => x.properties.contains(&Property::ChorusPlantSouth(ChorusPlantSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "up").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChorusPlantUp(ChorusPlantUp::True)),
				"false" => x.properties.contains(&Property::ChorusPlantUp(ChorusPlantUp::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChorusPlantWest(ChorusPlantWest::True)),
				"false" => x.properties.contains(&Property::ChorusPlantWest(ChorusPlantWest::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Cocoa => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::CocoaAge(CocoaAge::Num0)),
				"1" => x.properties.contains(&Property::CocoaAge(CocoaAge::Num1)),
				"2" => x.properties.contains(&Property::CocoaAge(CocoaAge::Num2)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::CocoaFacing(CocoaFacing::North)),
				"south" => x.properties.contains(&Property::CocoaFacing(CocoaFacing::South)),
				"west" => x.properties.contains(&Property::CocoaFacing(CocoaFacing::West)),
				"east" => x.properties.contains(&Property::CocoaFacing(CocoaFacing::East)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::ColoredFalling => {
			block.states.first().unwrap().id
		},
		Type::Command => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "conditional").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CommandConditional(CommandConditional::True)),
				"false" => x.properties.contains(&Property::CommandConditional(CommandConditional::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::CommandFacing(CommandFacing::North)),
				"east" => x.properties.contains(&Property::CommandFacing(CommandFacing::East)),
				"south" => x.properties.contains(&Property::CommandFacing(CommandFacing::South)),
				"west" => x.properties.contains(&Property::CommandFacing(CommandFacing::West)),
				"up" => x.properties.contains(&Property::CommandFacing(CommandFacing::Up)),
				"down" => x.properties.contains(&Property::CommandFacing(CommandFacing::Down)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Comparator => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::ComparatorFacing(ComparatorFacing::North)),
				"south" => x.properties.contains(&Property::ComparatorFacing(ComparatorFacing::South)),
				"west" => x.properties.contains(&Property::ComparatorFacing(ComparatorFacing::West)),
				"east" => x.properties.contains(&Property::ComparatorFacing(ComparatorFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "mode").unwrap().1.as_str() {
				"compare" => x.properties.contains(&Property::ComparatorMode(ComparatorMode::Compare)),
				"subtract" => x.properties.contains(&Property::ComparatorMode(ComparatorMode::Subtract)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ComparatorPowered(ComparatorPowered::True)),
				"false" => x.properties.contains(&Property::ComparatorPowered(ComparatorPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Composter => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "level").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::ComposterLevel(ComposterLevel::Num0)),
				"1" => x.properties.contains(&Property::ComposterLevel(ComposterLevel::Num1)),
				"2" => x.properties.contains(&Property::ComposterLevel(ComposterLevel::Num2)),
				"3" => x.properties.contains(&Property::ComposterLevel(ComposterLevel::Num3)),
				"4" => x.properties.contains(&Property::ComposterLevel(ComposterLevel::Num4)),
				"5" => x.properties.contains(&Property::ComposterLevel(ComposterLevel::Num5)),
				"6" => x.properties.contains(&Property::ComposterLevel(ComposterLevel::Num6)),
				"7" => x.properties.contains(&Property::ComposterLevel(ComposterLevel::Num7)),
				"8" => x.properties.contains(&Property::ComposterLevel(ComposterLevel::Num8)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::ConcretePowder => {
			block.states.first().unwrap().id
		},
		Type::Conduit => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ConduitWaterlogged(ConduitWaterlogged::True)),
				"false" => x.properties.contains(&Property::ConduitWaterlogged(ConduitWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::CopperBulbBlock => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CopperBulbBlockLit(CopperBulbBlockLit::True)),
				"false" => x.properties.contains(&Property::CopperBulbBlockLit(CopperBulbBlockLit::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CopperBulbBlockPowered(CopperBulbBlockPowered::True)),
				"false" => x.properties.contains(&Property::CopperBulbBlockPowered(CopperBulbBlockPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::CopperChest => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::CopperChestFacing(CopperChestFacing::North)),
				"south" => x.properties.contains(&Property::CopperChestFacing(CopperChestFacing::South)),
				"west" => x.properties.contains(&Property::CopperChestFacing(CopperChestFacing::West)),
				"east" => x.properties.contains(&Property::CopperChestFacing(CopperChestFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "type").unwrap().1.as_str() {
				"single" => x.properties.contains(&Property::CopperChestType(CopperChestType::Single)),
				"left" => x.properties.contains(&Property::CopperChestType(CopperChestType::Left)),
				"right" => x.properties.contains(&Property::CopperChestType(CopperChestType::Right)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CopperChestWaterlogged(CopperChestWaterlogged::True)),
				"false" => x.properties.contains(&Property::CopperChestWaterlogged(CopperChestWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::CopperGolemStatue => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "copper_golem_pose").unwrap().1.as_str() {
				"standing" => x.properties.contains(&Property::CopperGolemStatueCopperGolemPose(CopperGolemStatueCopperGolemPose::Standing)),
				"sitting" => x.properties.contains(&Property::CopperGolemStatueCopperGolemPose(CopperGolemStatueCopperGolemPose::Sitting)),
				"running" => x.properties.contains(&Property::CopperGolemStatueCopperGolemPose(CopperGolemStatueCopperGolemPose::Running)),
				"star" => x.properties.contains(&Property::CopperGolemStatueCopperGolemPose(CopperGolemStatueCopperGolemPose::Star)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::CopperGolemStatueFacing(CopperGolemStatueFacing::North)),
				"south" => x.properties.contains(&Property::CopperGolemStatueFacing(CopperGolemStatueFacing::South)),
				"west" => x.properties.contains(&Property::CopperGolemStatueFacing(CopperGolemStatueFacing::West)),
				"east" => x.properties.contains(&Property::CopperGolemStatueFacing(CopperGolemStatueFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CopperGolemStatueWaterlogged(CopperGolemStatueWaterlogged::True)),
				"false" => x.properties.contains(&Property::CopperGolemStatueWaterlogged(CopperGolemStatueWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Coral => {
			block.states.first().unwrap().id
		},
		Type::CoralFan => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CoralFanWaterlogged(CoralFanWaterlogged::True)),
				"false" => x.properties.contains(&Property::CoralFanWaterlogged(CoralFanWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::CoralPlant => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CoralPlantWaterlogged(CoralPlantWaterlogged::True)),
				"false" => x.properties.contains(&Property::CoralPlantWaterlogged(CoralPlantWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::CoralWallFan => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::CoralWallFanFacing(CoralWallFanFacing::North)),
				"south" => x.properties.contains(&Property::CoralWallFanFacing(CoralWallFanFacing::South)),
				"west" => x.properties.contains(&Property::CoralWallFanFacing(CoralWallFanFacing::West)),
				"east" => x.properties.contains(&Property::CoralWallFanFacing(CoralWallFanFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CoralWallFanWaterlogged(CoralWallFanWaterlogged::True)),
				"false" => x.properties.contains(&Property::CoralWallFanWaterlogged(CoralWallFanWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Crafter => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "crafting").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CrafterCrafting(CrafterCrafting::True)),
				"false" => x.properties.contains(&Property::CrafterCrafting(CrafterCrafting::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "orientation").unwrap().1.as_str() {
				"down_east" => x.properties.contains(&Property::CrafterOrientation(CrafterOrientation::DownEast)),
				"down_north" => x.properties.contains(&Property::CrafterOrientation(CrafterOrientation::DownNorth)),
				"down_south" => x.properties.contains(&Property::CrafterOrientation(CrafterOrientation::DownSouth)),
				"down_west" => x.properties.contains(&Property::CrafterOrientation(CrafterOrientation::DownWest)),
				"up_east" => x.properties.contains(&Property::CrafterOrientation(CrafterOrientation::UpEast)),
				"up_north" => x.properties.contains(&Property::CrafterOrientation(CrafterOrientation::UpNorth)),
				"up_south" => x.properties.contains(&Property::CrafterOrientation(CrafterOrientation::UpSouth)),
				"up_west" => x.properties.contains(&Property::CrafterOrientation(CrafterOrientation::UpWest)),
				"west_up" => x.properties.contains(&Property::CrafterOrientation(CrafterOrientation::WestUp)),
				"east_up" => x.properties.contains(&Property::CrafterOrientation(CrafterOrientation::EastUp)),
				"north_up" => x.properties.contains(&Property::CrafterOrientation(CrafterOrientation::NorthUp)),
				"south_up" => x.properties.contains(&Property::CrafterOrientation(CrafterOrientation::SouthUp)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "triggered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CrafterTriggered(CrafterTriggered::True)),
				"false" => x.properties.contains(&Property::CrafterTriggered(CrafterTriggered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::CraftingTable => {
			block.states.first().unwrap().id
		},
		Type::CreakingHeart => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "axis").unwrap().1.as_str() {
				"x" => x.properties.contains(&Property::CreakingHeartAxis(CreakingHeartAxis::X)),
				"y" => x.properties.contains(&Property::CreakingHeartAxis(CreakingHeartAxis::Y)),
				"z" => x.properties.contains(&Property::CreakingHeartAxis(CreakingHeartAxis::Z)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "creaking_heart_state").unwrap().1.as_str() {
				"uprooted" => x.properties.contains(&Property::CreakingHeartCreakingHeartState(CreakingHeartCreakingHeartState::Uprooted)),
				"dormant" => x.properties.contains(&Property::CreakingHeartCreakingHeartState(CreakingHeartCreakingHeartState::Dormant)),
				"awake" => x.properties.contains(&Property::CreakingHeartCreakingHeartState(CreakingHeartCreakingHeartState::Awake)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "natural").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CreakingHeartNatural(CreakingHeartNatural::True)),
				"false" => x.properties.contains(&Property::CreakingHeartNatural(CreakingHeartNatural::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Crop => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::CropAge(CropAge::Num0)),
				"1" => x.properties.contains(&Property::CropAge(CropAge::Num1)),
				"2" => x.properties.contains(&Property::CropAge(CropAge::Num2)),
				"3" => x.properties.contains(&Property::CropAge(CropAge::Num3)),
				"4" => x.properties.contains(&Property::CropAge(CropAge::Num4)),
				"5" => x.properties.contains(&Property::CropAge(CropAge::Num5)),
				"6" => x.properties.contains(&Property::CropAge(CropAge::Num6)),
				"7" => x.properties.contains(&Property::CropAge(CropAge::Num7)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::CryingObsidian => {
			block.states.first().unwrap().id
		},
		Type::DaylightDetector => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "inverted").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DaylightDetectorInverted(DaylightDetectorInverted::True)),
				"false" => x.properties.contains(&Property::DaylightDetectorInverted(DaylightDetectorInverted::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "power").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num0)),
				"1" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num1)),
				"2" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num2)),
				"3" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num3)),
				"4" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num4)),
				"5" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num5)),
				"6" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num6)),
				"7" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num7)),
				"8" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num8)),
				"9" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num9)),
				"10" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num10)),
				"11" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num11)),
				"12" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num12)),
				"13" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num13)),
				"14" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num14)),
				"15" => x.properties.contains(&Property::DaylightDetectorPower(DaylightDetectorPower::Num15)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::DecoratedPot => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "cracked").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DecoratedPotCracked(DecoratedPotCracked::True)),
				"false" => x.properties.contains(&Property::DecoratedPotCracked(DecoratedPotCracked::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::DecoratedPotFacing(DecoratedPotFacing::North)),
				"south" => x.properties.contains(&Property::DecoratedPotFacing(DecoratedPotFacing::South)),
				"west" => x.properties.contains(&Property::DecoratedPotFacing(DecoratedPotFacing::West)),
				"east" => x.properties.contains(&Property::DecoratedPotFacing(DecoratedPotFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DecoratedPotWaterlogged(DecoratedPotWaterlogged::True)),
				"false" => x.properties.contains(&Property::DecoratedPotWaterlogged(DecoratedPotWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::DetectorRail => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DetectorRailPowered(DetectorRailPowered::True)),
				"false" => x.properties.contains(&Property::DetectorRailPowered(DetectorRailPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "shape").unwrap().1.as_str() {
				"north_south" => x.properties.contains(&Property::DetectorRailShape(DetectorRailShape::NorthSouth)),
				"east_west" => x.properties.contains(&Property::DetectorRailShape(DetectorRailShape::EastWest)),
				"ascending_east" => x.properties.contains(&Property::DetectorRailShape(DetectorRailShape::AscendingEast)),
				"ascending_west" => x.properties.contains(&Property::DetectorRailShape(DetectorRailShape::AscendingWest)),
				"ascending_north" => x.properties.contains(&Property::DetectorRailShape(DetectorRailShape::AscendingNorth)),
				"ascending_south" => x.properties.contains(&Property::DetectorRailShape(DetectorRailShape::AscendingSouth)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DetectorRailWaterlogged(DetectorRailWaterlogged::True)),
				"false" => x.properties.contains(&Property::DetectorRailWaterlogged(DetectorRailWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::DirtPath => {
			block.states.first().unwrap().id
		},
		Type::Dispenser => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::DispenserFacing(DispenserFacing::North)),
				"east" => x.properties.contains(&Property::DispenserFacing(DispenserFacing::East)),
				"south" => x.properties.contains(&Property::DispenserFacing(DispenserFacing::South)),
				"west" => x.properties.contains(&Property::DispenserFacing(DispenserFacing::West)),
				"up" => x.properties.contains(&Property::DispenserFacing(DispenserFacing::Up)),
				"down" => x.properties.contains(&Property::DispenserFacing(DispenserFacing::Down)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "triggered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DispenserTriggered(DispenserTriggered::True)),
				"false" => x.properties.contains(&Property::DispenserTriggered(DispenserTriggered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Door => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::DoorFacing(DoorFacing::North)),
				"south" => x.properties.contains(&Property::DoorFacing(DoorFacing::South)),
				"west" => x.properties.contains(&Property::DoorFacing(DoorFacing::West)),
				"east" => x.properties.contains(&Property::DoorFacing(DoorFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"upper" => x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)),
				"lower" => x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hinge").unwrap().1.as_str() {
				"left" => x.properties.contains(&Property::DoorHinge(DoorHinge::Left)),
				"right" => x.properties.contains(&Property::DoorHinge(DoorHinge::Right)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "open").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DoorOpen(DoorOpen::True)),
				"false" => x.properties.contains(&Property::DoorOpen(DoorOpen::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DoorPowered(DoorPowered::True)),
				"false" => x.properties.contains(&Property::DoorPowered(DoorPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Double => {
			block.states.first().unwrap().id
		},
		Type::DoublePlant => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"upper" => x.properties.contains(&Property::DoublePlantHalf(DoublePlantHalf::Upper)),
				"lower" => x.properties.contains(&Property::DoublePlantHalf(DoublePlantHalf::Lower)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::DragonEgg => {
			block.states.first().unwrap().id
		},
		Type::DriedGhast => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::DriedGhastFacing(DriedGhastFacing::North)),
				"south" => x.properties.contains(&Property::DriedGhastFacing(DriedGhastFacing::South)),
				"west" => x.properties.contains(&Property::DriedGhastFacing(DriedGhastFacing::West)),
				"east" => x.properties.contains(&Property::DriedGhastFacing(DriedGhastFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hydration").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::DriedGhastHydration(DriedGhastHydration::Num0)),
				"1" => x.properties.contains(&Property::DriedGhastHydration(DriedGhastHydration::Num1)),
				"2" => x.properties.contains(&Property::DriedGhastHydration(DriedGhastHydration::Num2)),
				"3" => x.properties.contains(&Property::DriedGhastHydration(DriedGhastHydration::Num3)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DriedGhastWaterlogged(DriedGhastWaterlogged::True)),
				"false" => x.properties.contains(&Property::DriedGhastWaterlogged(DriedGhastWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::DropExperience => {
			block.states.first().unwrap().id
		},
		Type::Dropper => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::DropperFacing(DropperFacing::North)),
				"east" => x.properties.contains(&Property::DropperFacing(DropperFacing::East)),
				"south" => x.properties.contains(&Property::DropperFacing(DropperFacing::South)),
				"west" => x.properties.contains(&Property::DropperFacing(DropperFacing::West)),
				"up" => x.properties.contains(&Property::DropperFacing(DropperFacing::Up)),
				"down" => x.properties.contains(&Property::DropperFacing(DropperFacing::Down)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "triggered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DropperTriggered(DropperTriggered::True)),
				"false" => x.properties.contains(&Property::DropperTriggered(DropperTriggered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::DryVegetation => {
			block.states.first().unwrap().id
		},
		Type::EnchantmentTable => {
			block.states.first().unwrap().id
		},
		Type::EndGateway => {
			block.states.first().unwrap().id
		},
		Type::EndPortal => {
			block.states.first().unwrap().id
		},
		Type::EndPortalFrame => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "eye").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::EndPortalFrameEye(EndPortalFrameEye::True)),
				"false" => x.properties.contains(&Property::EndPortalFrameEye(EndPortalFrameEye::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::EndPortalFrameFacing(EndPortalFrameFacing::North)),
				"south" => x.properties.contains(&Property::EndPortalFrameFacing(EndPortalFrameFacing::South)),
				"west" => x.properties.contains(&Property::EndPortalFrameFacing(EndPortalFrameFacing::West)),
				"east" => x.properties.contains(&Property::EndPortalFrameFacing(EndPortalFrameFacing::East)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::EndRod => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::EndRodFacing(EndRodFacing::North)),
				"east" => x.properties.contains(&Property::EndRodFacing(EndRodFacing::East)),
				"south" => x.properties.contains(&Property::EndRodFacing(EndRodFacing::South)),
				"west" => x.properties.contains(&Property::EndRodFacing(EndRodFacing::West)),
				"up" => x.properties.contains(&Property::EndRodFacing(EndRodFacing::Up)),
				"down" => x.properties.contains(&Property::EndRodFacing(EndRodFacing::Down)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::EnderChest => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::EnderChestFacing(EnderChestFacing::North)),
				"south" => x.properties.contains(&Property::EnderChestFacing(EnderChestFacing::South)),
				"west" => x.properties.contains(&Property::EnderChestFacing(EnderChestFacing::West)),
				"east" => x.properties.contains(&Property::EnderChestFacing(EnderChestFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::EnderChestWaterlogged(EnderChestWaterlogged::True)),
				"false" => x.properties.contains(&Property::EnderChestWaterlogged(EnderChestWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Eyeblossom => {
			block.states.first().unwrap().id
		},
		Type::Farm => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "moisture").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num0)),
				"1" => x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num1)),
				"2" => x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num2)),
				"3" => x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num3)),
				"4" => x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num4)),
				"5" => x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num5)),
				"6" => x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num6)),
				"7" => x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num7)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Fence => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FenceEast(FenceEast::True)),
				"false" => x.properties.contains(&Property::FenceEast(FenceEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FenceNorth(FenceNorth::True)),
				"false" => x.properties.contains(&Property::FenceNorth(FenceNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FenceSouth(FenceSouth::True)),
				"false" => x.properties.contains(&Property::FenceSouth(FenceSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FenceWaterlogged(FenceWaterlogged::True)),
				"false" => x.properties.contains(&Property::FenceWaterlogged(FenceWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FenceWest(FenceWest::True)),
				"false" => x.properties.contains(&Property::FenceWest(FenceWest::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::FenceGate => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::North)),
				"south" => x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::South)),
				"west" => x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::West)),
				"east" => x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "in_wall").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::True)),
				"false" => x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "open").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::True)),
				"false" => x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::True)),
				"false" => x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Fire => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::FireAge(FireAge::Num0)),
				"1" => x.properties.contains(&Property::FireAge(FireAge::Num1)),
				"2" => x.properties.contains(&Property::FireAge(FireAge::Num2)),
				"3" => x.properties.contains(&Property::FireAge(FireAge::Num3)),
				"4" => x.properties.contains(&Property::FireAge(FireAge::Num4)),
				"5" => x.properties.contains(&Property::FireAge(FireAge::Num5)),
				"6" => x.properties.contains(&Property::FireAge(FireAge::Num6)),
				"7" => x.properties.contains(&Property::FireAge(FireAge::Num7)),
				"8" => x.properties.contains(&Property::FireAge(FireAge::Num8)),
				"9" => x.properties.contains(&Property::FireAge(FireAge::Num9)),
				"10" => x.properties.contains(&Property::FireAge(FireAge::Num10)),
				"11" => x.properties.contains(&Property::FireAge(FireAge::Num11)),
				"12" => x.properties.contains(&Property::FireAge(FireAge::Num12)),
				"13" => x.properties.contains(&Property::FireAge(FireAge::Num13)),
				"14" => x.properties.contains(&Property::FireAge(FireAge::Num14)),
				"15" => x.properties.contains(&Property::FireAge(FireAge::Num15)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FireEast(FireEast::True)),
				"false" => x.properties.contains(&Property::FireEast(FireEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FireNorth(FireNorth::True)),
				"false" => x.properties.contains(&Property::FireNorth(FireNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FireSouth(FireSouth::True)),
				"false" => x.properties.contains(&Property::FireSouth(FireSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "up").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FireUp(FireUp::True)),
				"false" => x.properties.contains(&Property::FireUp(FireUp::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FireWest(FireWest::True)),
				"false" => x.properties.contains(&Property::FireWest(FireWest::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::FireflyBush => {
			block.states.first().unwrap().id
		},
		Type::Flower => {
			block.states.first().unwrap().id
		},
		Type::FlowerBed => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::FlowerBedFacing(FlowerBedFacing::North)),
				"south" => x.properties.contains(&Property::FlowerBedFacing(FlowerBedFacing::South)),
				"west" => x.properties.contains(&Property::FlowerBedFacing(FlowerBedFacing::West)),
				"east" => x.properties.contains(&Property::FlowerBedFacing(FlowerBedFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "flower_amount").unwrap().1.as_str() {
				"1" => x.properties.contains(&Property::FlowerBedFlowerAmount(FlowerBedFlowerAmount::Num1)),
				"2" => x.properties.contains(&Property::FlowerBedFlowerAmount(FlowerBedFlowerAmount::Num2)),
				"3" => x.properties.contains(&Property::FlowerBedFlowerAmount(FlowerBedFlowerAmount::Num3)),
				"4" => x.properties.contains(&Property::FlowerBedFlowerAmount(FlowerBedFlowerAmount::Num4)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::FlowerPot => {
			block.states.first().unwrap().id
		},
		Type::Frogspawn => {
			block.states.first().unwrap().id
		},
		Type::FrostedIce => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::FrostedIceAge(FrostedIceAge::Num0)),
				"1" => x.properties.contains(&Property::FrostedIceAge(FrostedIceAge::Num1)),
				"2" => x.properties.contains(&Property::FrostedIceAge(FrostedIceAge::Num2)),
				"3" => x.properties.contains(&Property::FrostedIceAge(FrostedIceAge::Num3)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Fungus => {
			block.states.first().unwrap().id
		},
		Type::Furnace => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::FurnaceFacing(FurnaceFacing::North)),
				"south" => x.properties.contains(&Property::FurnaceFacing(FurnaceFacing::South)),
				"west" => x.properties.contains(&Property::FurnaceFacing(FurnaceFacing::West)),
				"east" => x.properties.contains(&Property::FurnaceFacing(FurnaceFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FurnaceLit(FurnaceLit::True)),
				"false" => x.properties.contains(&Property::FurnaceLit(FurnaceLit::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::GlazedTerracotta => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::GlazedTerracottaFacing(GlazedTerracottaFacing::North)),
				"south" => x.properties.contains(&Property::GlazedTerracottaFacing(GlazedTerracottaFacing::South)),
				"west" => x.properties.contains(&Property::GlazedTerracottaFacing(GlazedTerracottaFacing::West)),
				"east" => x.properties.contains(&Property::GlazedTerracottaFacing(GlazedTerracottaFacing::East)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::GlowLichen => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "down").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::GlowLichenDown(GlowLichenDown::True)),
				"false" => x.properties.contains(&Property::GlowLichenDown(GlowLichenDown::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::GlowLichenEast(GlowLichenEast::True)),
				"false" => x.properties.contains(&Property::GlowLichenEast(GlowLichenEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::GlowLichenNorth(GlowLichenNorth::True)),
				"false" => x.properties.contains(&Property::GlowLichenNorth(GlowLichenNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::GlowLichenSouth(GlowLichenSouth::True)),
				"false" => x.properties.contains(&Property::GlowLichenSouth(GlowLichenSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "up").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::GlowLichenUp(GlowLichenUp::True)),
				"false" => x.properties.contains(&Property::GlowLichenUp(GlowLichenUp::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::GlowLichenWaterlogged(GlowLichenWaterlogged::True)),
				"false" => x.properties.contains(&Property::GlowLichenWaterlogged(GlowLichenWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::GlowLichenWest(GlowLichenWest::True)),
				"false" => x.properties.contains(&Property::GlowLichenWest(GlowLichenWest::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Grass => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "snowy").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::GrassSnowy(GrassSnowy::True)),
				"false" => x.properties.contains(&Property::GrassSnowy(GrassSnowy::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Grindstone => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "face").unwrap().1.as_str() {
				"floor" => x.properties.contains(&Property::GrindstoneFace(GrindstoneFace::Floor)),
				"wall" => x.properties.contains(&Property::GrindstoneFace(GrindstoneFace::Wall)),
				"ceiling" => x.properties.contains(&Property::GrindstoneFace(GrindstoneFace::Ceiling)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::GrindstoneFacing(GrindstoneFacing::North)),
				"south" => x.properties.contains(&Property::GrindstoneFacing(GrindstoneFacing::South)),
				"west" => x.properties.contains(&Property::GrindstoneFacing(GrindstoneFacing::West)),
				"east" => x.properties.contains(&Property::GrindstoneFacing(GrindstoneFacing::East)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::HalfTransparent => {
			block.states.first().unwrap().id
		},
		Type::HangingMoss => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "tip").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HangingMossTip(HangingMossTip::True)),
				"false" => x.properties.contains(&Property::HangingMossTip(HangingMossTip::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::HangingRoots => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HangingRootsWaterlogged(HangingRootsWaterlogged::True)),
				"false" => x.properties.contains(&Property::HangingRootsWaterlogged(HangingRootsWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Hay => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "axis").unwrap().1.as_str() {
				"x" => x.properties.contains(&Property::HayAxis(HayAxis::X)),
				"y" => x.properties.contains(&Property::HayAxis(HayAxis::Y)),
				"z" => x.properties.contains(&Property::HayAxis(HayAxis::Z)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::HeavyCore => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HeavyCoreWaterlogged(HeavyCoreWaterlogged::True)),
				"false" => x.properties.contains(&Property::HeavyCoreWaterlogged(HeavyCoreWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Honey => {
			block.states.first().unwrap().id
		},
		Type::Hopper => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "enabled").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HopperEnabled(HopperEnabled::True)),
				"false" => x.properties.contains(&Property::HopperEnabled(HopperEnabled::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"down" => x.properties.contains(&Property::HopperFacing(HopperFacing::Down)),
				"north" => x.properties.contains(&Property::HopperFacing(HopperFacing::North)),
				"south" => x.properties.contains(&Property::HopperFacing(HopperFacing::South)),
				"west" => x.properties.contains(&Property::HopperFacing(HopperFacing::West)),
				"east" => x.properties.contains(&Property::HopperFacing(HopperFacing::East)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::HugeMushroom => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "down").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HugeMushroomDown(HugeMushroomDown::True)),
				"false" => x.properties.contains(&Property::HugeMushroomDown(HugeMushroomDown::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HugeMushroomEast(HugeMushroomEast::True)),
				"false" => x.properties.contains(&Property::HugeMushroomEast(HugeMushroomEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HugeMushroomNorth(HugeMushroomNorth::True)),
				"false" => x.properties.contains(&Property::HugeMushroomNorth(HugeMushroomNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HugeMushroomSouth(HugeMushroomSouth::True)),
				"false" => x.properties.contains(&Property::HugeMushroomSouth(HugeMushroomSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "up").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HugeMushroomUp(HugeMushroomUp::True)),
				"false" => x.properties.contains(&Property::HugeMushroomUp(HugeMushroomUp::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HugeMushroomWest(HugeMushroomWest::True)),
				"false" => x.properties.contains(&Property::HugeMushroomWest(HugeMushroomWest::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Ice => {
			block.states.first().unwrap().id
		},
		Type::Infested => {
			block.states.first().unwrap().id
		},
		Type::InfestedRotatedPillar => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "axis").unwrap().1.as_str() {
				"x" => x.properties.contains(&Property::InfestedRotatedPillarAxis(InfestedRotatedPillarAxis::X)),
				"y" => x.properties.contains(&Property::InfestedRotatedPillarAxis(InfestedRotatedPillarAxis::Y)),
				"z" => x.properties.contains(&Property::InfestedRotatedPillarAxis(InfestedRotatedPillarAxis::Z)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::IronBars => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::IronBarsEast(IronBarsEast::True)),
				"false" => x.properties.contains(&Property::IronBarsEast(IronBarsEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::IronBarsNorth(IronBarsNorth::True)),
				"false" => x.properties.contains(&Property::IronBarsNorth(IronBarsNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::IronBarsSouth(IronBarsSouth::True)),
				"false" => x.properties.contains(&Property::IronBarsSouth(IronBarsSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::IronBarsWaterlogged(IronBarsWaterlogged::True)),
				"false" => x.properties.contains(&Property::IronBarsWaterlogged(IronBarsWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::IronBarsWest(IronBarsWest::True)),
				"false" => x.properties.contains(&Property::IronBarsWest(IronBarsWest::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::JackOLantern => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::JackOLanternFacing(JackOLanternFacing::North)),
				"south" => x.properties.contains(&Property::JackOLanternFacing(JackOLanternFacing::South)),
				"west" => x.properties.contains(&Property::JackOLanternFacing(JackOLanternFacing::West)),
				"east" => x.properties.contains(&Property::JackOLanternFacing(JackOLanternFacing::East)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Jigsaw => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "orientation").unwrap().1.as_str() {
				"down_east" => x.properties.contains(&Property::JigsawOrientation(JigsawOrientation::DownEast)),
				"down_north" => x.properties.contains(&Property::JigsawOrientation(JigsawOrientation::DownNorth)),
				"down_south" => x.properties.contains(&Property::JigsawOrientation(JigsawOrientation::DownSouth)),
				"down_west" => x.properties.contains(&Property::JigsawOrientation(JigsawOrientation::DownWest)),
				"up_east" => x.properties.contains(&Property::JigsawOrientation(JigsawOrientation::UpEast)),
				"up_north" => x.properties.contains(&Property::JigsawOrientation(JigsawOrientation::UpNorth)),
				"up_south" => x.properties.contains(&Property::JigsawOrientation(JigsawOrientation::UpSouth)),
				"up_west" => x.properties.contains(&Property::JigsawOrientation(JigsawOrientation::UpWest)),
				"west_up" => x.properties.contains(&Property::JigsawOrientation(JigsawOrientation::WestUp)),
				"east_up" => x.properties.contains(&Property::JigsawOrientation(JigsawOrientation::EastUp)),
				"north_up" => x.properties.contains(&Property::JigsawOrientation(JigsawOrientation::NorthUp)),
				"south_up" => x.properties.contains(&Property::JigsawOrientation(JigsawOrientation::SouthUp)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Jukebox => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "has_record").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::JukeboxHasRecord(JukeboxHasRecord::True)),
				"false" => x.properties.contains(&Property::JukeboxHasRecord(JukeboxHasRecord::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Kelp => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::KelpAge(KelpAge::Num0)),
				"1" => x.properties.contains(&Property::KelpAge(KelpAge::Num1)),
				"2" => x.properties.contains(&Property::KelpAge(KelpAge::Num2)),
				"3" => x.properties.contains(&Property::KelpAge(KelpAge::Num3)),
				"4" => x.properties.contains(&Property::KelpAge(KelpAge::Num4)),
				"5" => x.properties.contains(&Property::KelpAge(KelpAge::Num5)),
				"6" => x.properties.contains(&Property::KelpAge(KelpAge::Num6)),
				"7" => x.properties.contains(&Property::KelpAge(KelpAge::Num7)),
				"8" => x.properties.contains(&Property::KelpAge(KelpAge::Num8)),
				"9" => x.properties.contains(&Property::KelpAge(KelpAge::Num9)),
				"10" => x.properties.contains(&Property::KelpAge(KelpAge::Num10)),
				"11" => x.properties.contains(&Property::KelpAge(KelpAge::Num11)),
				"12" => x.properties.contains(&Property::KelpAge(KelpAge::Num12)),
				"13" => x.properties.contains(&Property::KelpAge(KelpAge::Num13)),
				"14" => x.properties.contains(&Property::KelpAge(KelpAge::Num14)),
				"15" => x.properties.contains(&Property::KelpAge(KelpAge::Num15)),
				"16" => x.properties.contains(&Property::KelpAge(KelpAge::Num16)),
				"17" => x.properties.contains(&Property::KelpAge(KelpAge::Num17)),
				"18" => x.properties.contains(&Property::KelpAge(KelpAge::Num18)),
				"19" => x.properties.contains(&Property::KelpAge(KelpAge::Num19)),
				"20" => x.properties.contains(&Property::KelpAge(KelpAge::Num20)),
				"21" => x.properties.contains(&Property::KelpAge(KelpAge::Num21)),
				"22" => x.properties.contains(&Property::KelpAge(KelpAge::Num22)),
				"23" => x.properties.contains(&Property::KelpAge(KelpAge::Num23)),
				"24" => x.properties.contains(&Property::KelpAge(KelpAge::Num24)),
				"25" => x.properties.contains(&Property::KelpAge(KelpAge::Num25)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::KelpPlant => {
			block.states.first().unwrap().id
		},
		Type::Ladder => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::LadderFacing(LadderFacing::North)),
				"south" => x.properties.contains(&Property::LadderFacing(LadderFacing::South)),
				"west" => x.properties.contains(&Property::LadderFacing(LadderFacing::West)),
				"east" => x.properties.contains(&Property::LadderFacing(LadderFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::LadderWaterlogged(LadderWaterlogged::True)),
				"false" => x.properties.contains(&Property::LadderWaterlogged(LadderWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Lantern => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hanging").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::LanternHanging(LanternHanging::True)),
				"false" => x.properties.contains(&Property::LanternHanging(LanternHanging::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::LanternWaterlogged(LanternWaterlogged::True)),
				"false" => x.properties.contains(&Property::LanternWaterlogged(LanternWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::LavaCauldron => {
			block.states.first().unwrap().id
		},
		Type::LayeredCauldron => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "level").unwrap().1.as_str() {
				"1" => x.properties.contains(&Property::LayeredCauldronLevel(LayeredCauldronLevel::Num1)),
				"2" => x.properties.contains(&Property::LayeredCauldronLevel(LayeredCauldronLevel::Num2)),
				"3" => x.properties.contains(&Property::LayeredCauldronLevel(LayeredCauldronLevel::Num3)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::LeafLitter => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::LeafLitterFacing(LeafLitterFacing::North)),
				"south" => x.properties.contains(&Property::LeafLitterFacing(LeafLitterFacing::South)),
				"west" => x.properties.contains(&Property::LeafLitterFacing(LeafLitterFacing::West)),
				"east" => x.properties.contains(&Property::LeafLitterFacing(LeafLitterFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "segment_amount").unwrap().1.as_str() {
				"1" => x.properties.contains(&Property::LeafLitterSegmentAmount(LeafLitterSegmentAmount::Num1)),
				"2" => x.properties.contains(&Property::LeafLitterSegmentAmount(LeafLitterSegmentAmount::Num2)),
				"3" => x.properties.contains(&Property::LeafLitterSegmentAmount(LeafLitterSegmentAmount::Num3)),
				"4" => x.properties.contains(&Property::LeafLitterSegmentAmount(LeafLitterSegmentAmount::Num4)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Lectern => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::LecternFacing(LecternFacing::North)),
				"south" => x.properties.contains(&Property::LecternFacing(LecternFacing::South)),
				"west" => x.properties.contains(&Property::LecternFacing(LecternFacing::West)),
				"east" => x.properties.contains(&Property::LecternFacing(LecternFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "has_book").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::LecternHasBook(LecternHasBook::True)),
				"false" => x.properties.contains(&Property::LecternHasBook(LecternHasBook::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::LecternPowered(LecternPowered::True)),
				"false" => x.properties.contains(&Property::LecternPowered(LecternPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Left => {
			block.states.first().unwrap().id
		},
		Type::Lever => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "face").unwrap().1.as_str() {
				"floor" => x.properties.contains(&Property::LeverFace(LeverFace::Floor)),
				"wall" => x.properties.contains(&Property::LeverFace(LeverFace::Wall)),
				"ceiling" => x.properties.contains(&Property::LeverFace(LeverFace::Ceiling)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::LeverFacing(LeverFacing::North)),
				"south" => x.properties.contains(&Property::LeverFacing(LeverFacing::South)),
				"west" => x.properties.contains(&Property::LeverFacing(LeverFacing::West)),
				"east" => x.properties.contains(&Property::LeverFacing(LeverFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::LeverPowered(LeverPowered::True)),
				"false" => x.properties.contains(&Property::LeverPowered(LeverPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Light => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "level").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::LightLevel(LightLevel::Num0)),
				"1" => x.properties.contains(&Property::LightLevel(LightLevel::Num1)),
				"2" => x.properties.contains(&Property::LightLevel(LightLevel::Num2)),
				"3" => x.properties.contains(&Property::LightLevel(LightLevel::Num3)),
				"4" => x.properties.contains(&Property::LightLevel(LightLevel::Num4)),
				"5" => x.properties.contains(&Property::LightLevel(LightLevel::Num5)),
				"6" => x.properties.contains(&Property::LightLevel(LightLevel::Num6)),
				"7" => x.properties.contains(&Property::LightLevel(LightLevel::Num7)),
				"8" => x.properties.contains(&Property::LightLevel(LightLevel::Num8)),
				"9" => x.properties.contains(&Property::LightLevel(LightLevel::Num9)),
				"10" => x.properties.contains(&Property::LightLevel(LightLevel::Num10)),
				"11" => x.properties.contains(&Property::LightLevel(LightLevel::Num11)),
				"12" => x.properties.contains(&Property::LightLevel(LightLevel::Num12)),
				"13" => x.properties.contains(&Property::LightLevel(LightLevel::Num13)),
				"14" => x.properties.contains(&Property::LightLevel(LightLevel::Num14)),
				"15" => x.properties.contains(&Property::LightLevel(LightLevel::Num15)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::LightWaterlogged(LightWaterlogged::True)),
				"false" => x.properties.contains(&Property::LightWaterlogged(LightWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::LightningRod => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::LightningRodFacing(LightningRodFacing::North)),
				"east" => x.properties.contains(&Property::LightningRodFacing(LightningRodFacing::East)),
				"south" => x.properties.contains(&Property::LightningRodFacing(LightningRodFacing::South)),
				"west" => x.properties.contains(&Property::LightningRodFacing(LightningRodFacing::West)),
				"up" => x.properties.contains(&Property::LightningRodFacing(LightningRodFacing::Up)),
				"down" => x.properties.contains(&Property::LightningRodFacing(LightningRodFacing::Down)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::LightningRodPowered(LightningRodPowered::True)),
				"false" => x.properties.contains(&Property::LightningRodPowered(LightningRodPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::LightningRodWaterlogged(LightningRodWaterlogged::True)),
				"false" => x.properties.contains(&Property::LightningRodWaterlogged(LightningRodWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Liquid => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "level").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num0)),
				"1" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num1)),
				"2" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num2)),
				"3" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num3)),
				"4" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num4)),
				"5" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num5)),
				"6" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num6)),
				"7" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num7)),
				"8" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num8)),
				"9" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num9)),
				"10" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num10)),
				"11" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num11)),
				"12" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num12)),
				"13" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num13)),
				"14" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num14)),
				"15" => x.properties.contains(&Property::LiquidLevel(LiquidLevel::Num15)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Loom => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::LoomFacing(LoomFacing::North)),
				"south" => x.properties.contains(&Property::LoomFacing(LoomFacing::South)),
				"west" => x.properties.contains(&Property::LoomFacing(LoomFacing::West)),
				"east" => x.properties.contains(&Property::LoomFacing(LoomFacing::East)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Magma => {
			block.states.first().unwrap().id
		},
		Type::MangroveLeaves => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "distance").unwrap().1.as_str() {
				"1" => x.properties.contains(&Property::MangroveLeavesDistance(MangroveLeavesDistance::Num1)),
				"2" => x.properties.contains(&Property::MangroveLeavesDistance(MangroveLeavesDistance::Num2)),
				"3" => x.properties.contains(&Property::MangroveLeavesDistance(MangroveLeavesDistance::Num3)),
				"4" => x.properties.contains(&Property::MangroveLeavesDistance(MangroveLeavesDistance::Num4)),
				"5" => x.properties.contains(&Property::MangroveLeavesDistance(MangroveLeavesDistance::Num5)),
				"6" => x.properties.contains(&Property::MangroveLeavesDistance(MangroveLeavesDistance::Num6)),
				"7" => x.properties.contains(&Property::MangroveLeavesDistance(MangroveLeavesDistance::Num7)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "persistent").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MangroveLeavesPersistent(MangroveLeavesPersistent::True)),
				"false" => x.properties.contains(&Property::MangroveLeavesPersistent(MangroveLeavesPersistent::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MangroveLeavesWaterlogged(MangroveLeavesWaterlogged::True)),
				"false" => x.properties.contains(&Property::MangroveLeavesWaterlogged(MangroveLeavesWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::MangrovePropagule => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::MangrovePropaguleAge(MangrovePropaguleAge::Num0)),
				"1" => x.properties.contains(&Property::MangrovePropaguleAge(MangrovePropaguleAge::Num1)),
				"2" => x.properties.contains(&Property::MangrovePropaguleAge(MangrovePropaguleAge::Num2)),
				"3" => x.properties.contains(&Property::MangrovePropaguleAge(MangrovePropaguleAge::Num3)),
				"4" => x.properties.contains(&Property::MangrovePropaguleAge(MangrovePropaguleAge::Num4)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hanging").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MangrovePropaguleHanging(MangrovePropaguleHanging::True)),
				"false" => x.properties.contains(&Property::MangrovePropaguleHanging(MangrovePropaguleHanging::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "stage").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::MangrovePropaguleStage(MangrovePropaguleStage::Num0)),
				"1" => x.properties.contains(&Property::MangrovePropaguleStage(MangrovePropaguleStage::Num1)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MangrovePropaguleWaterlogged(MangrovePropaguleWaterlogged::True)),
				"false" => x.properties.contains(&Property::MangrovePropaguleWaterlogged(MangrovePropaguleWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::MangroveRoots => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MangroveRootsWaterlogged(MangroveRootsWaterlogged::True)),
				"false" => x.properties.contains(&Property::MangroveRootsWaterlogged(MangroveRootsWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::MossyCarpet => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "bottom").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MossyCarpetBottom(MossyCarpetBottom::True)),
				"false" => x.properties.contains(&Property::MossyCarpetBottom(MossyCarpetBottom::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::MossyCarpetEast(MossyCarpetEast::None)),
				"low" => x.properties.contains(&Property::MossyCarpetEast(MossyCarpetEast::Low)),
				"tall" => x.properties.contains(&Property::MossyCarpetEast(MossyCarpetEast::Tall)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::MossyCarpetNorth(MossyCarpetNorth::None)),
				"low" => x.properties.contains(&Property::MossyCarpetNorth(MossyCarpetNorth::Low)),
				"tall" => x.properties.contains(&Property::MossyCarpetNorth(MossyCarpetNorth::Tall)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::MossyCarpetSouth(MossyCarpetSouth::None)),
				"low" => x.properties.contains(&Property::MossyCarpetSouth(MossyCarpetSouth::Low)),
				"tall" => x.properties.contains(&Property::MossyCarpetSouth(MossyCarpetSouth::Tall)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::MossyCarpetWest(MossyCarpetWest::None)),
				"low" => x.properties.contains(&Property::MossyCarpetWest(MossyCarpetWest::Low)),
				"tall" => x.properties.contains(&Property::MossyCarpetWest(MossyCarpetWest::Tall)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::MovingPiston => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::MovingPistonFacing(MovingPistonFacing::North)),
				"east" => x.properties.contains(&Property::MovingPistonFacing(MovingPistonFacing::East)),
				"south" => x.properties.contains(&Property::MovingPistonFacing(MovingPistonFacing::South)),
				"west" => x.properties.contains(&Property::MovingPistonFacing(MovingPistonFacing::West)),
				"up" => x.properties.contains(&Property::MovingPistonFacing(MovingPistonFacing::Up)),
				"down" => x.properties.contains(&Property::MovingPistonFacing(MovingPistonFacing::Down)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "type").unwrap().1.as_str() {
				"normal" => x.properties.contains(&Property::MovingPistonType(MovingPistonType::Normal)),
				"sticky" => x.properties.contains(&Property::MovingPistonType(MovingPistonType::Sticky)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Mud => {
			block.states.first().unwrap().id
		},
		Type::Multiface => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "down").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MultifaceDown(MultifaceDown::True)),
				"false" => x.properties.contains(&Property::MultifaceDown(MultifaceDown::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MultifaceEast(MultifaceEast::True)),
				"false" => x.properties.contains(&Property::MultifaceEast(MultifaceEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MultifaceNorth(MultifaceNorth::True)),
				"false" => x.properties.contains(&Property::MultifaceNorth(MultifaceNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MultifaceSouth(MultifaceSouth::True)),
				"false" => x.properties.contains(&Property::MultifaceSouth(MultifaceSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "up").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MultifaceUp(MultifaceUp::True)),
				"false" => x.properties.contains(&Property::MultifaceUp(MultifaceUp::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MultifaceWaterlogged(MultifaceWaterlogged::True)),
				"false" => x.properties.contains(&Property::MultifaceWaterlogged(MultifaceWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MultifaceWest(MultifaceWest::True)),
				"false" => x.properties.contains(&Property::MultifaceWest(MultifaceWest::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Mushroom => {
			block.states.first().unwrap().id
		},
		Type::Mycelium => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "snowy").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MyceliumSnowy(MyceliumSnowy::True)),
				"false" => x.properties.contains(&Property::MyceliumSnowy(MyceliumSnowy::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::NetherPortal => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "axis").unwrap().1.as_str() {
				"x" => x.properties.contains(&Property::NetherPortalAxis(NetherPortalAxis::X)),
				"z" => x.properties.contains(&Property::NetherPortalAxis(NetherPortalAxis::Z)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::NetherSprouts => {
			block.states.first().unwrap().id
		},
		Type::NetherWart => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::NetherWartAge(NetherWartAge::Num0)),
				"1" => x.properties.contains(&Property::NetherWartAge(NetherWartAge::Num1)),
				"2" => x.properties.contains(&Property::NetherWartAge(NetherWartAge::Num2)),
				"3" => x.properties.contains(&Property::NetherWartAge(NetherWartAge::Num3)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Netherrack => {
			block.states.first().unwrap().id
		},
		Type::Normal => {
			block.states.first().unwrap().id
		},
		Type::Note => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "instrument").unwrap().1.as_str() {
				"harp" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Harp)),
				"basedrum" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Basedrum)),
				"snare" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Snare)),
				"hat" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Hat)),
				"bass" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Bass)),
				"flute" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Flute)),
				"bell" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Bell)),
				"guitar" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Guitar)),
				"chime" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Chime)),
				"xylophone" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Xylophone)),
				"iron_xylophone" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::IronXylophone)),
				"cow_bell" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::CowBell)),
				"didgeridoo" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Didgeridoo)),
				"bit" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Bit)),
				"banjo" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Banjo)),
				"pling" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Pling)),
				"zombie" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Zombie)),
				"skeleton" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Skeleton)),
				"creeper" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Creeper)),
				"dragon" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Dragon)),
				"wither_skeleton" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::WitherSkeleton)),
				"piglin" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::Piglin)),
				"custom_head" => x.properties.contains(&Property::NoteInstrument(NoteInstrument::CustomHead)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "note").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::NoteNote(NoteNote::Num0)),
				"1" => x.properties.contains(&Property::NoteNote(NoteNote::Num1)),
				"2" => x.properties.contains(&Property::NoteNote(NoteNote::Num2)),
				"3" => x.properties.contains(&Property::NoteNote(NoteNote::Num3)),
				"4" => x.properties.contains(&Property::NoteNote(NoteNote::Num4)),
				"5" => x.properties.contains(&Property::NoteNote(NoteNote::Num5)),
				"6" => x.properties.contains(&Property::NoteNote(NoteNote::Num6)),
				"7" => x.properties.contains(&Property::NoteNote(NoteNote::Num7)),
				"8" => x.properties.contains(&Property::NoteNote(NoteNote::Num8)),
				"9" => x.properties.contains(&Property::NoteNote(NoteNote::Num9)),
				"10" => x.properties.contains(&Property::NoteNote(NoteNote::Num10)),
				"11" => x.properties.contains(&Property::NoteNote(NoteNote::Num11)),
				"12" => x.properties.contains(&Property::NoteNote(NoteNote::Num12)),
				"13" => x.properties.contains(&Property::NoteNote(NoteNote::Num13)),
				"14" => x.properties.contains(&Property::NoteNote(NoteNote::Num14)),
				"15" => x.properties.contains(&Property::NoteNote(NoteNote::Num15)),
				"16" => x.properties.contains(&Property::NoteNote(NoteNote::Num16)),
				"17" => x.properties.contains(&Property::NoteNote(NoteNote::Num17)),
				"18" => x.properties.contains(&Property::NoteNote(NoteNote::Num18)),
				"19" => x.properties.contains(&Property::NoteNote(NoteNote::Num19)),
				"20" => x.properties.contains(&Property::NoteNote(NoteNote::Num20)),
				"21" => x.properties.contains(&Property::NoteNote(NoteNote::Num21)),
				"22" => x.properties.contains(&Property::NoteNote(NoteNote::Num22)),
				"23" => x.properties.contains(&Property::NoteNote(NoteNote::Num23)),
				"24" => x.properties.contains(&Property::NoteNote(NoteNote::Num24)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::NotePowered(NotePowered::True)),
				"false" => x.properties.contains(&Property::NotePowered(NotePowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Nylium => {
			block.states.first().unwrap().id
		},
		Type::Observer => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::ObserverFacing(ObserverFacing::North)),
				"east" => x.properties.contains(&Property::ObserverFacing(ObserverFacing::East)),
				"south" => x.properties.contains(&Property::ObserverFacing(ObserverFacing::South)),
				"west" => x.properties.contains(&Property::ObserverFacing(ObserverFacing::West)),
				"up" => x.properties.contains(&Property::ObserverFacing(ObserverFacing::Up)),
				"down" => x.properties.contains(&Property::ObserverFacing(ObserverFacing::Down)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ObserverPowered(ObserverPowered::True)),
				"false" => x.properties.contains(&Property::ObserverPowered(ObserverPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::PaleOakLeaves => {
			block.states.first().unwrap().id
		},
		Type::Piglinwallskull => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::PiglinwallskullFacing(PiglinwallskullFacing::North)),
				"south" => x.properties.contains(&Property::PiglinwallskullFacing(PiglinwallskullFacing::South)),
				"west" => x.properties.contains(&Property::PiglinwallskullFacing(PiglinwallskullFacing::West)),
				"east" => x.properties.contains(&Property::PiglinwallskullFacing(PiglinwallskullFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::PiglinwallskullPowered(PiglinwallskullPowered::True)),
				"false" => x.properties.contains(&Property::PiglinwallskullPowered(PiglinwallskullPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::PistonBase => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "extended").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::PistonBaseExtended(PistonBaseExtended::True)),
				"false" => x.properties.contains(&Property::PistonBaseExtended(PistonBaseExtended::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::PistonBaseFacing(PistonBaseFacing::North)),
				"east" => x.properties.contains(&Property::PistonBaseFacing(PistonBaseFacing::East)),
				"south" => x.properties.contains(&Property::PistonBaseFacing(PistonBaseFacing::South)),
				"west" => x.properties.contains(&Property::PistonBaseFacing(PistonBaseFacing::West)),
				"up" => x.properties.contains(&Property::PistonBaseFacing(PistonBaseFacing::Up)),
				"down" => x.properties.contains(&Property::PistonBaseFacing(PistonBaseFacing::Down)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::PistonHead => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::PistonHeadFacing(PistonHeadFacing::North)),
				"east" => x.properties.contains(&Property::PistonHeadFacing(PistonHeadFacing::East)),
				"south" => x.properties.contains(&Property::PistonHeadFacing(PistonHeadFacing::South)),
				"west" => x.properties.contains(&Property::PistonHeadFacing(PistonHeadFacing::West)),
				"up" => x.properties.contains(&Property::PistonHeadFacing(PistonHeadFacing::Up)),
				"down" => x.properties.contains(&Property::PistonHeadFacing(PistonHeadFacing::Down)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "short").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::PistonHeadShort(PistonHeadShort::True)),
				"false" => x.properties.contains(&Property::PistonHeadShort(PistonHeadShort::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "type").unwrap().1.as_str() {
				"normal" => x.properties.contains(&Property::PistonHeadType(PistonHeadType::Normal)),
				"sticky" => x.properties.contains(&Property::PistonHeadType(PistonHeadType::Sticky)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::PitcherCrop => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::PitcherCropAge(PitcherCropAge::Num0)),
				"1" => x.properties.contains(&Property::PitcherCropAge(PitcherCropAge::Num1)),
				"2" => x.properties.contains(&Property::PitcherCropAge(PitcherCropAge::Num2)),
				"3" => x.properties.contains(&Property::PitcherCropAge(PitcherCropAge::Num3)),
				"4" => x.properties.contains(&Property::PitcherCropAge(PitcherCropAge::Num4)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"upper" => x.properties.contains(&Property::PitcherCropHalf(PitcherCropHalf::Upper)),
				"lower" => x.properties.contains(&Property::PitcherCropHalf(PitcherCropHalf::Lower)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::PlayerHead => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::PlayerHeadPowered(PlayerHeadPowered::True)),
				"false" => x.properties.contains(&Property::PlayerHeadPowered(PlayerHeadPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "rotation").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num0)),
				"1" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num1)),
				"2" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num2)),
				"3" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num3)),
				"4" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num4)),
				"5" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num5)),
				"6" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num6)),
				"7" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num7)),
				"8" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num8)),
				"9" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num9)),
				"10" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num10)),
				"11" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num11)),
				"12" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num12)),
				"13" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num13)),
				"14" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num14)),
				"15" => x.properties.contains(&Property::PlayerHeadRotation(PlayerHeadRotation::Num15)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::PlayerWallHead => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::PlayerWallHeadFacing(PlayerWallHeadFacing::North)),
				"south" => x.properties.contains(&Property::PlayerWallHeadFacing(PlayerWallHeadFacing::South)),
				"west" => x.properties.contains(&Property::PlayerWallHeadFacing(PlayerWallHeadFacing::West)),
				"east" => x.properties.contains(&Property::PlayerWallHeadFacing(PlayerWallHeadFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::PlayerWallHeadPowered(PlayerWallHeadPowered::True)),
				"false" => x.properties.contains(&Property::PlayerWallHeadPowered(PlayerWallHeadPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::PointedDripstone => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "thickness").unwrap().1.as_str() {
				"tip_merge" => x.properties.contains(&Property::PointedDripstoneThickness(PointedDripstoneThickness::TipMerge)),
				"tip" => x.properties.contains(&Property::PointedDripstoneThickness(PointedDripstoneThickness::Tip)),
				"frustum" => x.properties.contains(&Property::PointedDripstoneThickness(PointedDripstoneThickness::Frustum)),
				"middle" => x.properties.contains(&Property::PointedDripstoneThickness(PointedDripstoneThickness::Middle)),
				"base" => x.properties.contains(&Property::PointedDripstoneThickness(PointedDripstoneThickness::Base)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "vertical_direction").unwrap().1.as_str() {
				"up" => x.properties.contains(&Property::PointedDripstoneVerticalDirection(PointedDripstoneVerticalDirection::Up)),
				"down" => x.properties.contains(&Property::PointedDripstoneVerticalDirection(PointedDripstoneVerticalDirection::Down)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::PointedDripstoneWaterlogged(PointedDripstoneWaterlogged::True)),
				"false" => x.properties.contains(&Property::PointedDripstoneWaterlogged(PointedDripstoneWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Potato => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::PotatoAge(PotatoAge::Num0)),
				"1" => x.properties.contains(&Property::PotatoAge(PotatoAge::Num1)),
				"2" => x.properties.contains(&Property::PotatoAge(PotatoAge::Num2)),
				"3" => x.properties.contains(&Property::PotatoAge(PotatoAge::Num3)),
				"4" => x.properties.contains(&Property::PotatoAge(PotatoAge::Num4)),
				"5" => x.properties.contains(&Property::PotatoAge(PotatoAge::Num5)),
				"6" => x.properties.contains(&Property::PotatoAge(PotatoAge::Num6)),
				"7" => x.properties.contains(&Property::PotatoAge(PotatoAge::Num7)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::PowderSnow => {
			block.states.first().unwrap().id
		},
		Type::Powered => {
			block.states.first().unwrap().id
		},
		Type::PoweredRail => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::PoweredRailPowered(PoweredRailPowered::True)),
				"false" => x.properties.contains(&Property::PoweredRailPowered(PoweredRailPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "shape").unwrap().1.as_str() {
				"north_south" => x.properties.contains(&Property::PoweredRailShape(PoweredRailShape::NorthSouth)),
				"east_west" => x.properties.contains(&Property::PoweredRailShape(PoweredRailShape::EastWest)),
				"ascending_east" => x.properties.contains(&Property::PoweredRailShape(PoweredRailShape::AscendingEast)),
				"ascending_west" => x.properties.contains(&Property::PoweredRailShape(PoweredRailShape::AscendingWest)),
				"ascending_north" => x.properties.contains(&Property::PoweredRailShape(PoweredRailShape::AscendingNorth)),
				"ascending_south" => x.properties.contains(&Property::PoweredRailShape(PoweredRailShape::AscendingSouth)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::PoweredRailWaterlogged(PoweredRailWaterlogged::True)),
				"false" => x.properties.contains(&Property::PoweredRailWaterlogged(PoweredRailWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::PressurePlate => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::PressurePlatePowered(PressurePlatePowered::True)),
				"false" => x.properties.contains(&Property::PressurePlatePowered(PressurePlatePowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Pumpkin => {
			block.states.first().unwrap().id
		},
		Type::Rail => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "shape").unwrap().1.as_str() {
				"north_south" => x.properties.contains(&Property::RailShape(RailShape::NorthSouth)),
				"east_west" => x.properties.contains(&Property::RailShape(RailShape::EastWest)),
				"ascending_east" => x.properties.contains(&Property::RailShape(RailShape::AscendingEast)),
				"ascending_west" => x.properties.contains(&Property::RailShape(RailShape::AscendingWest)),
				"ascending_north" => x.properties.contains(&Property::RailShape(RailShape::AscendingNorth)),
				"ascending_south" => x.properties.contains(&Property::RailShape(RailShape::AscendingSouth)),
				"south_east" => x.properties.contains(&Property::RailShape(RailShape::SouthEast)),
				"south_west" => x.properties.contains(&Property::RailShape(RailShape::SouthWest)),
				"north_west" => x.properties.contains(&Property::RailShape(RailShape::NorthWest)),
				"north_east" => x.properties.contains(&Property::RailShape(RailShape::NorthEast)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::RailWaterlogged(RailWaterlogged::True)),
				"false" => x.properties.contains(&Property::RailWaterlogged(RailWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::RedstoneLamp => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::RedstoneLampLit(RedstoneLampLit::True)),
				"false" => x.properties.contains(&Property::RedstoneLampLit(RedstoneLampLit::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::RedstoneOre => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::RedstoneOreLit(RedstoneOreLit::True)),
				"false" => x.properties.contains(&Property::RedstoneOreLit(RedstoneOreLit::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::RedstoneTorch => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::RedstoneTorchLit(RedstoneTorchLit::True)),
				"false" => x.properties.contains(&Property::RedstoneTorchLit(RedstoneTorchLit::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::RedstoneWallTorch => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::North)),
				"south" => x.properties.contains(&Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::South)),
				"west" => x.properties.contains(&Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::West)),
				"east" => x.properties.contains(&Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::RedstoneWallTorchLit(RedstoneWallTorchLit::True)),
				"false" => x.properties.contains(&Property::RedstoneWallTorchLit(RedstoneWallTorchLit::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::RedstoneWire => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"up" => x.properties.contains(&Property::RedstoneWireEast(RedstoneWireEast::Up)),
				"side" => x.properties.contains(&Property::RedstoneWireEast(RedstoneWireEast::Side)),
				"none" => x.properties.contains(&Property::RedstoneWireEast(RedstoneWireEast::None)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"up" => x.properties.contains(&Property::RedstoneWireNorth(RedstoneWireNorth::Up)),
				"side" => x.properties.contains(&Property::RedstoneWireNorth(RedstoneWireNorth::Side)),
				"none" => x.properties.contains(&Property::RedstoneWireNorth(RedstoneWireNorth::None)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "power").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num0)),
				"1" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num1)),
				"2" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num2)),
				"3" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num3)),
				"4" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num4)),
				"5" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num5)),
				"6" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num6)),
				"7" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num7)),
				"8" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num8)),
				"9" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num9)),
				"10" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num10)),
				"11" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num11)),
				"12" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num12)),
				"13" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num13)),
				"14" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num14)),
				"15" => x.properties.contains(&Property::RedstoneWirePower(RedstoneWirePower::Num15)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"up" => x.properties.contains(&Property::RedstoneWireSouth(RedstoneWireSouth::Up)),
				"side" => x.properties.contains(&Property::RedstoneWireSouth(RedstoneWireSouth::Side)),
				"none" => x.properties.contains(&Property::RedstoneWireSouth(RedstoneWireSouth::None)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"up" => x.properties.contains(&Property::RedstoneWireWest(RedstoneWireWest::Up)),
				"side" => x.properties.contains(&Property::RedstoneWireWest(RedstoneWireWest::Side)),
				"none" => x.properties.contains(&Property::RedstoneWireWest(RedstoneWireWest::None)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Repeater => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "delay").unwrap().1.as_str() {
				"1" => x.properties.contains(&Property::RepeaterDelay(RepeaterDelay::Num1)),
				"2" => x.properties.contains(&Property::RepeaterDelay(RepeaterDelay::Num2)),
				"3" => x.properties.contains(&Property::RepeaterDelay(RepeaterDelay::Num3)),
				"4" => x.properties.contains(&Property::RepeaterDelay(RepeaterDelay::Num4)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::RepeaterFacing(RepeaterFacing::North)),
				"south" => x.properties.contains(&Property::RepeaterFacing(RepeaterFacing::South)),
				"west" => x.properties.contains(&Property::RepeaterFacing(RepeaterFacing::West)),
				"east" => x.properties.contains(&Property::RepeaterFacing(RepeaterFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "locked").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::RepeaterLocked(RepeaterLocked::True)),
				"false" => x.properties.contains(&Property::RepeaterLocked(RepeaterLocked::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::RepeaterPowered(RepeaterPowered::True)),
				"false" => x.properties.contains(&Property::RepeaterPowered(RepeaterPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::RespawnAnchor => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "charges").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::RespawnAnchorCharges(RespawnAnchorCharges::Num0)),
				"1" => x.properties.contains(&Property::RespawnAnchorCharges(RespawnAnchorCharges::Num1)),
				"2" => x.properties.contains(&Property::RespawnAnchorCharges(RespawnAnchorCharges::Num2)),
				"3" => x.properties.contains(&Property::RespawnAnchorCharges(RespawnAnchorCharges::Num3)),
				"4" => x.properties.contains(&Property::RespawnAnchorCharges(RespawnAnchorCharges::Num4)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Right => {
			block.states.first().unwrap().id
		},
		Type::RootedDirt => {
			block.states.first().unwrap().id
		},
		Type::Roots => {
			block.states.first().unwrap().id
		},
		Type::RotatedPillar => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "axis").unwrap().1.as_str() {
				"x" => x.properties.contains(&Property::RotatedPillarAxis(RotatedPillarAxis::X)),
				"y" => x.properties.contains(&Property::RotatedPillarAxis(RotatedPillarAxis::Y)),
				"z" => x.properties.contains(&Property::RotatedPillarAxis(RotatedPillarAxis::Z)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Sand => {
			block.states.first().unwrap().id
		},
		Type::Sapling => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "stage").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::SaplingStage(SaplingStage::Num0)),
				"1" => x.properties.contains(&Property::SaplingStage(SaplingStage::Num1)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Scaffolding => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "bottom").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ScaffoldingBottom(ScaffoldingBottom::True)),
				"false" => x.properties.contains(&Property::ScaffoldingBottom(ScaffoldingBottom::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "distance").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::ScaffoldingDistance(ScaffoldingDistance::Num0)),
				"1" => x.properties.contains(&Property::ScaffoldingDistance(ScaffoldingDistance::Num1)),
				"2" => x.properties.contains(&Property::ScaffoldingDistance(ScaffoldingDistance::Num2)),
				"3" => x.properties.contains(&Property::ScaffoldingDistance(ScaffoldingDistance::Num3)),
				"4" => x.properties.contains(&Property::ScaffoldingDistance(ScaffoldingDistance::Num4)),
				"5" => x.properties.contains(&Property::ScaffoldingDistance(ScaffoldingDistance::Num5)),
				"6" => x.properties.contains(&Property::ScaffoldingDistance(ScaffoldingDistance::Num6)),
				"7" => x.properties.contains(&Property::ScaffoldingDistance(ScaffoldingDistance::Num7)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ScaffoldingWaterlogged(ScaffoldingWaterlogged::True)),
				"false" => x.properties.contains(&Property::ScaffoldingWaterlogged(ScaffoldingWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Sculk => {
			block.states.first().unwrap().id
		},
		Type::SculkCatalyst => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "bloom").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkCatalystBloom(SculkCatalystBloom::True)),
				"false" => x.properties.contains(&Property::SculkCatalystBloom(SculkCatalystBloom::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::SculkSensor => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "power").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num0)),
				"1" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num1)),
				"2" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num2)),
				"3" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num3)),
				"4" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num4)),
				"5" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num5)),
				"6" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num6)),
				"7" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num7)),
				"8" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num8)),
				"9" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num9)),
				"10" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num10)),
				"11" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num11)),
				"12" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num12)),
				"13" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num13)),
				"14" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num14)),
				"15" => x.properties.contains(&Property::SculkSensorPower(SculkSensorPower::Num15)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "sculk_sensor_phase").unwrap().1.as_str() {
				"inactive" => x.properties.contains(&Property::SculkSensorSculkSensorPhase(SculkSensorSculkSensorPhase::Inactive)),
				"active" => x.properties.contains(&Property::SculkSensorSculkSensorPhase(SculkSensorSculkSensorPhase::Active)),
				"cooldown" => x.properties.contains(&Property::SculkSensorSculkSensorPhase(SculkSensorSculkSensorPhase::Cooldown)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkSensorWaterlogged(SculkSensorWaterlogged::True)),
				"false" => x.properties.contains(&Property::SculkSensorWaterlogged(SculkSensorWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::SculkShrieker => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "can_summon").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkShriekerCanSummon(SculkShriekerCanSummon::True)),
				"false" => x.properties.contains(&Property::SculkShriekerCanSummon(SculkShriekerCanSummon::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "shrieking").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkShriekerShrieking(SculkShriekerShrieking::True)),
				"false" => x.properties.contains(&Property::SculkShriekerShrieking(SculkShriekerShrieking::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkShriekerWaterlogged(SculkShriekerWaterlogged::True)),
				"false" => x.properties.contains(&Property::SculkShriekerWaterlogged(SculkShriekerWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::SculkVein => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "down").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkVeinDown(SculkVeinDown::True)),
				"false" => x.properties.contains(&Property::SculkVeinDown(SculkVeinDown::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkVeinEast(SculkVeinEast::True)),
				"false" => x.properties.contains(&Property::SculkVeinEast(SculkVeinEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkVeinNorth(SculkVeinNorth::True)),
				"false" => x.properties.contains(&Property::SculkVeinNorth(SculkVeinNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkVeinSouth(SculkVeinSouth::True)),
				"false" => x.properties.contains(&Property::SculkVeinSouth(SculkVeinSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "up").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkVeinUp(SculkVeinUp::True)),
				"false" => x.properties.contains(&Property::SculkVeinUp(SculkVeinUp::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkVeinWaterlogged(SculkVeinWaterlogged::True)),
				"false" => x.properties.contains(&Property::SculkVeinWaterlogged(SculkVeinWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkVeinWest(SculkVeinWest::True)),
				"false" => x.properties.contains(&Property::SculkVeinWest(SculkVeinWest::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::SeaPickle => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "pickles").unwrap().1.as_str() {
				"1" => x.properties.contains(&Property::SeaPicklePickles(SeaPicklePickles::Num1)),
				"2" => x.properties.contains(&Property::SeaPicklePickles(SeaPicklePickles::Num2)),
				"3" => x.properties.contains(&Property::SeaPicklePickles(SeaPicklePickles::Num3)),
				"4" => x.properties.contains(&Property::SeaPicklePickles(SeaPicklePickles::Num4)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SeaPickleWaterlogged(SeaPickleWaterlogged::True)),
				"false" => x.properties.contains(&Property::SeaPickleWaterlogged(SeaPickleWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Seagrass => {
			block.states.first().unwrap().id
		},
		Type::Shelf => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::ShelfFacing(ShelfFacing::North)),
				"south" => x.properties.contains(&Property::ShelfFacing(ShelfFacing::South)),
				"west" => x.properties.contains(&Property::ShelfFacing(ShelfFacing::West)),
				"east" => x.properties.contains(&Property::ShelfFacing(ShelfFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ShelfPowered(ShelfPowered::True)),
				"false" => x.properties.contains(&Property::ShelfPowered(ShelfPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "side_chain").unwrap().1.as_str() {
				"unconnected" => x.properties.contains(&Property::ShelfSideChain(ShelfSideChain::Unconnected)),
				"right" => x.properties.contains(&Property::ShelfSideChain(ShelfSideChain::Right)),
				"center" => x.properties.contains(&Property::ShelfSideChain(ShelfSideChain::Center)),
				"left" => x.properties.contains(&Property::ShelfSideChain(ShelfSideChain::Left)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ShelfWaterlogged(ShelfWaterlogged::True)),
				"false" => x.properties.contains(&Property::ShelfWaterlogged(ShelfWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::ShortDryGrass => {
			block.states.first().unwrap().id
		},
		Type::ShulkerBox => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::ShulkerBoxFacing(ShulkerBoxFacing::North)),
				"east" => x.properties.contains(&Property::ShulkerBoxFacing(ShulkerBoxFacing::East)),
				"south" => x.properties.contains(&Property::ShulkerBoxFacing(ShulkerBoxFacing::South)),
				"west" => x.properties.contains(&Property::ShulkerBoxFacing(ShulkerBoxFacing::West)),
				"up" => x.properties.contains(&Property::ShulkerBoxFacing(ShulkerBoxFacing::Up)),
				"down" => x.properties.contains(&Property::ShulkerBoxFacing(ShulkerBoxFacing::Down)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Single => {
			block.states.first().unwrap().id
		},
		Type::Skull => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SkullPowered(SkullPowered::True)),
				"false" => x.properties.contains(&Property::SkullPowered(SkullPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "rotation").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num0)),
				"1" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num1)),
				"2" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num2)),
				"3" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num3)),
				"4" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num4)),
				"5" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num5)),
				"6" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num6)),
				"7" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num7)),
				"8" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num8)),
				"9" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num9)),
				"10" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num10)),
				"11" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num11)),
				"12" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num12)),
				"13" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num13)),
				"14" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num14)),
				"15" => x.properties.contains(&Property::SkullRotation(SkullRotation::Num15)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Slab => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "type").unwrap().1.as_str() {
				"top" => x.properties.contains(&Property::SlabType(SlabType::Top)),
				"bottom" => x.properties.contains(&Property::SlabType(SlabType::Bottom)),
				"double" => x.properties.contains(&Property::SlabType(SlabType::Double)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::True)),
				"false" => x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Slime => {
			block.states.first().unwrap().id
		},
		Type::SmallDripleaf => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::SmallDripleafFacing(SmallDripleafFacing::North)),
				"south" => x.properties.contains(&Property::SmallDripleafFacing(SmallDripleafFacing::South)),
				"west" => x.properties.contains(&Property::SmallDripleafFacing(SmallDripleafFacing::West)),
				"east" => x.properties.contains(&Property::SmallDripleafFacing(SmallDripleafFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"upper" => x.properties.contains(&Property::SmallDripleafHalf(SmallDripleafHalf::Upper)),
				"lower" => x.properties.contains(&Property::SmallDripleafHalf(SmallDripleafHalf::Lower)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SmallDripleafWaterlogged(SmallDripleafWaterlogged::True)),
				"false" => x.properties.contains(&Property::SmallDripleafWaterlogged(SmallDripleafWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::SmithingTable => {
			block.states.first().unwrap().id
		},
		Type::Smoker => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::SmokerFacing(SmokerFacing::North)),
				"south" => x.properties.contains(&Property::SmokerFacing(SmokerFacing::South)),
				"west" => x.properties.contains(&Property::SmokerFacing(SmokerFacing::West)),
				"east" => x.properties.contains(&Property::SmokerFacing(SmokerFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SmokerLit(SmokerLit::True)),
				"false" => x.properties.contains(&Property::SmokerLit(SmokerLit::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::SnifferEgg => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hatch").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::SnifferEggHatch(SnifferEggHatch::Num0)),
				"1" => x.properties.contains(&Property::SnifferEggHatch(SnifferEggHatch::Num1)),
				"2" => x.properties.contains(&Property::SnifferEggHatch(SnifferEggHatch::Num2)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::SnowLayer => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "layers").unwrap().1.as_str() {
				"1" => x.properties.contains(&Property::SnowLayerLayers(SnowLayerLayers::Num1)),
				"2" => x.properties.contains(&Property::SnowLayerLayers(SnowLayerLayers::Num2)),
				"3" => x.properties.contains(&Property::SnowLayerLayers(SnowLayerLayers::Num3)),
				"4" => x.properties.contains(&Property::SnowLayerLayers(SnowLayerLayers::Num4)),
				"5" => x.properties.contains(&Property::SnowLayerLayers(SnowLayerLayers::Num5)),
				"6" => x.properties.contains(&Property::SnowLayerLayers(SnowLayerLayers::Num6)),
				"7" => x.properties.contains(&Property::SnowLayerLayers(SnowLayerLayers::Num7)),
				"8" => x.properties.contains(&Property::SnowLayerLayers(SnowLayerLayers::Num8)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::SnowyDirt => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "snowy").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SnowyDirtSnowy(SnowyDirtSnowy::True)),
				"false" => x.properties.contains(&Property::SnowyDirtSnowy(SnowyDirtSnowy::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::SoulFire => {
			block.states.first().unwrap().id
		},
		Type::SoulSand => {
			block.states.first().unwrap().id
		},
		Type::Spawner => {
			block.states.first().unwrap().id
		},
		Type::Sponge => {
			block.states.first().unwrap().id
		},
		Type::SporeBlossom => {
			block.states.first().unwrap().id
		},
		Type::StainedGlass => {
			block.states.first().unwrap().id
		},
		Type::StainedGlassPane => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::StainedGlassPaneEast(StainedGlassPaneEast::True)),
				"false" => x.properties.contains(&Property::StainedGlassPaneEast(StainedGlassPaneEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::StainedGlassPaneNorth(StainedGlassPaneNorth::True)),
				"false" => x.properties.contains(&Property::StainedGlassPaneNorth(StainedGlassPaneNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::StainedGlassPaneSouth(StainedGlassPaneSouth::True)),
				"false" => x.properties.contains(&Property::StainedGlassPaneSouth(StainedGlassPaneSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::StainedGlassPaneWaterlogged(StainedGlassPaneWaterlogged::True)),
				"false" => x.properties.contains(&Property::StainedGlassPaneWaterlogged(StainedGlassPaneWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::StainedGlassPaneWest(StainedGlassPaneWest::True)),
				"false" => x.properties.contains(&Property::StainedGlassPaneWest(StainedGlassPaneWest::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Stair => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::StairFacing(StairFacing::North)),
				"south" => x.properties.contains(&Property::StairFacing(StairFacing::South)),
				"west" => x.properties.contains(&Property::StairFacing(StairFacing::West)),
				"east" => x.properties.contains(&Property::StairFacing(StairFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"top" => x.properties.contains(&Property::StairHalf(StairHalf::Top)),
				"bottom" => x.properties.contains(&Property::StairHalf(StairHalf::Bottom)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "shape").unwrap().1.as_str() {
				"straight" => x.properties.contains(&Property::StairShape(StairShape::Straight)),
				"inner_left" => x.properties.contains(&Property::StairShape(StairShape::InnerLeft)),
				"inner_right" => x.properties.contains(&Property::StairShape(StairShape::InnerRight)),
				"outer_left" => x.properties.contains(&Property::StairShape(StairShape::OuterLeft)),
				"outer_right" => x.properties.contains(&Property::StairShape(StairShape::OuterRight)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::True)),
				"false" => x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::StandingSign => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "rotation").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num0)),
				"1" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num1)),
				"2" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num2)),
				"3" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num3)),
				"4" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num4)),
				"5" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num5)),
				"6" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num6)),
				"7" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num7)),
				"8" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num8)),
				"9" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num9)),
				"10" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num10)),
				"11" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num11)),
				"12" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num12)),
				"13" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num13)),
				"14" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num14)),
				"15" => x.properties.contains(&Property::StandingSignRotation(StandingSignRotation::Num15)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::StandingSignWaterlogged(StandingSignWaterlogged::True)),
				"false" => x.properties.contains(&Property::StandingSignWaterlogged(StandingSignWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Stem => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::StemAge(StemAge::Num0)),
				"1" => x.properties.contains(&Property::StemAge(StemAge::Num1)),
				"2" => x.properties.contains(&Property::StemAge(StemAge::Num2)),
				"3" => x.properties.contains(&Property::StemAge(StemAge::Num3)),
				"4" => x.properties.contains(&Property::StemAge(StemAge::Num4)),
				"5" => x.properties.contains(&Property::StemAge(StemAge::Num5)),
				"6" => x.properties.contains(&Property::StemAge(StemAge::Num6)),
				"7" => x.properties.contains(&Property::StemAge(StemAge::Num7)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Sticky => {
			block.states.first().unwrap().id
		},
		Type::Stonecutter => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::StonecutterFacing(StonecutterFacing::North)),
				"south" => x.properties.contains(&Property::StonecutterFacing(StonecutterFacing::South)),
				"west" => x.properties.contains(&Property::StonecutterFacing(StonecutterFacing::West)),
				"east" => x.properties.contains(&Property::StonecutterFacing(StonecutterFacing::East)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Structure => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "mode").unwrap().1.as_str() {
				"save" => x.properties.contains(&Property::StructureMode(StructureMode::Save)),
				"load" => x.properties.contains(&Property::StructureMode(StructureMode::Load)),
				"corner" => x.properties.contains(&Property::StructureMode(StructureMode::Corner)),
				"data" => x.properties.contains(&Property::StructureMode(StructureMode::Data)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::StructureVoid => {
			block.states.first().unwrap().id
		},
		Type::SugarCane => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num0)),
				"1" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num1)),
				"2" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num2)),
				"3" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num3)),
				"4" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num4)),
				"5" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num5)),
				"6" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num6)),
				"7" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num7)),
				"8" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num8)),
				"9" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num9)),
				"10" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num10)),
				"11" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num11)),
				"12" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num12)),
				"13" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num13)),
				"14" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num14)),
				"15" => x.properties.contains(&Property::SugarCaneAge(SugarCaneAge::Num15)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::SweetBerryBush => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::SweetBerryBushAge(SweetBerryBushAge::Num0)),
				"1" => x.properties.contains(&Property::SweetBerryBushAge(SweetBerryBushAge::Num1)),
				"2" => x.properties.contains(&Property::SweetBerryBushAge(SweetBerryBushAge::Num2)),
				"3" => x.properties.contains(&Property::SweetBerryBushAge(SweetBerryBushAge::Num3)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::TallDryGrass => {
			block.states.first().unwrap().id
		},
		Type::TallFlower => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"upper" => x.properties.contains(&Property::TallFlowerHalf(TallFlowerHalf::Upper)),
				"lower" => x.properties.contains(&Property::TallFlowerHalf(TallFlowerHalf::Lower)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::TallGrass => {
			block.states.first().unwrap().id
		},
		Type::TallSeagrass => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"upper" => x.properties.contains(&Property::TallSeagrassHalf(TallSeagrassHalf::Upper)),
				"lower" => x.properties.contains(&Property::TallSeagrassHalf(TallSeagrassHalf::Lower)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Target => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "power").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::TargetPower(TargetPower::Num0)),
				"1" => x.properties.contains(&Property::TargetPower(TargetPower::Num1)),
				"2" => x.properties.contains(&Property::TargetPower(TargetPower::Num2)),
				"3" => x.properties.contains(&Property::TargetPower(TargetPower::Num3)),
				"4" => x.properties.contains(&Property::TargetPower(TargetPower::Num4)),
				"5" => x.properties.contains(&Property::TargetPower(TargetPower::Num5)),
				"6" => x.properties.contains(&Property::TargetPower(TargetPower::Num6)),
				"7" => x.properties.contains(&Property::TargetPower(TargetPower::Num7)),
				"8" => x.properties.contains(&Property::TargetPower(TargetPower::Num8)),
				"9" => x.properties.contains(&Property::TargetPower(TargetPower::Num9)),
				"10" => x.properties.contains(&Property::TargetPower(TargetPower::Num10)),
				"11" => x.properties.contains(&Property::TargetPower(TargetPower::Num11)),
				"12" => x.properties.contains(&Property::TargetPower(TargetPower::Num12)),
				"13" => x.properties.contains(&Property::TargetPower(TargetPower::Num13)),
				"14" => x.properties.contains(&Property::TargetPower(TargetPower::Num14)),
				"15" => x.properties.contains(&Property::TargetPower(TargetPower::Num15)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Test => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "mode").unwrap().1.as_str() {
				"start" => x.properties.contains(&Property::TestMode(TestMode::Start)),
				"log" => x.properties.contains(&Property::TestMode(TestMode::Log)),
				"fail" => x.properties.contains(&Property::TestMode(TestMode::Fail)),
				"accept" => x.properties.contains(&Property::TestMode(TestMode::Accept)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::TestInstance => {
			block.states.first().unwrap().id
		},
		Type::TintedGlass => {
			block.states.first().unwrap().id
		},
		Type::TintedLeaves => {
			block.states.first().unwrap().id
		},
		Type::TintedParticleLeaves => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "distance").unwrap().1.as_str() {
				"1" => x.properties.contains(&Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num1)),
				"2" => x.properties.contains(&Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num2)),
				"3" => x.properties.contains(&Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num3)),
				"4" => x.properties.contains(&Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num4)),
				"5" => x.properties.contains(&Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num5)),
				"6" => x.properties.contains(&Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num6)),
				"7" => x.properties.contains(&Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num7)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "persistent").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TintedParticleLeavesPersistent(TintedParticleLeavesPersistent::True)),
				"false" => x.properties.contains(&Property::TintedParticleLeavesPersistent(TintedParticleLeavesPersistent::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TintedParticleLeavesWaterlogged(TintedParticleLeavesWaterlogged::True)),
				"false" => x.properties.contains(&Property::TintedParticleLeavesWaterlogged(TintedParticleLeavesWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Tnt => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "unstable").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TntUnstable(TntUnstable::True)),
				"false" => x.properties.contains(&Property::TntUnstable(TntUnstable::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Top => {
			block.states.first().unwrap().id
		},
		Type::Torch => {
			block.states.first().unwrap().id
		},
		Type::TorchflowerCrop => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::TorchflowerCropAge(TorchflowerCropAge::Num0)),
				"1" => x.properties.contains(&Property::TorchflowerCropAge(TorchflowerCropAge::Num1)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Transparent => {
			block.states.first().unwrap().id
		},
		Type::Trapdoor => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::TrapdoorFacing(TrapdoorFacing::North)),
				"south" => x.properties.contains(&Property::TrapdoorFacing(TrapdoorFacing::South)),
				"west" => x.properties.contains(&Property::TrapdoorFacing(TrapdoorFacing::West)),
				"east" => x.properties.contains(&Property::TrapdoorFacing(TrapdoorFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"top" => x.properties.contains(&Property::TrapdoorHalf(TrapdoorHalf::Top)),
				"bottom" => x.properties.contains(&Property::TrapdoorHalf(TrapdoorHalf::Bottom)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "open").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TrapdoorOpen(TrapdoorOpen::True)),
				"false" => x.properties.contains(&Property::TrapdoorOpen(TrapdoorOpen::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TrapdoorPowered(TrapdoorPowered::True)),
				"false" => x.properties.contains(&Property::TrapdoorPowered(TrapdoorPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TrapdoorWaterlogged(TrapdoorWaterlogged::True)),
				"false" => x.properties.contains(&Property::TrapdoorWaterlogged(TrapdoorWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::TrappedChest => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::TrappedChestFacing(TrappedChestFacing::North)),
				"south" => x.properties.contains(&Property::TrappedChestFacing(TrappedChestFacing::South)),
				"west" => x.properties.contains(&Property::TrappedChestFacing(TrappedChestFacing::West)),
				"east" => x.properties.contains(&Property::TrappedChestFacing(TrappedChestFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "type").unwrap().1.as_str() {
				"single" => x.properties.contains(&Property::TrappedChestType(TrappedChestType::Single)),
				"left" => x.properties.contains(&Property::TrappedChestType(TrappedChestType::Left)),
				"right" => x.properties.contains(&Property::TrappedChestType(TrappedChestType::Right)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TrappedChestWaterlogged(TrappedChestWaterlogged::True)),
				"false" => x.properties.contains(&Property::TrappedChestWaterlogged(TrappedChestWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::TrialSpawner => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "ominous").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TrialSpawnerOminous(TrialSpawnerOminous::True)),
				"false" => x.properties.contains(&Property::TrialSpawnerOminous(TrialSpawnerOminous::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "trial_spawner_state").unwrap().1.as_str() {
				"inactive" => x.properties.contains(&Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::Inactive)),
				"waiting_for_players" => x.properties.contains(&Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::WaitingForPlayers)),
				"active" => x.properties.contains(&Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::Active)),
				"waiting_for_reward_ejection" => x.properties.contains(&Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::WaitingForRewardEjection)),
				"ejecting_reward" => x.properties.contains(&Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::EjectingReward)),
				"cooldown" => x.properties.contains(&Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::Cooldown)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::TripWireHook => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "attached").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripWireHookAttached(TripWireHookAttached::True)),
				"false" => x.properties.contains(&Property::TripWireHookAttached(TripWireHookAttached::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::TripWireHookFacing(TripWireHookFacing::North)),
				"south" => x.properties.contains(&Property::TripWireHookFacing(TripWireHookFacing::South)),
				"west" => x.properties.contains(&Property::TripWireHookFacing(TripWireHookFacing::West)),
				"east" => x.properties.contains(&Property::TripWireHookFacing(TripWireHookFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripWireHookPowered(TripWireHookPowered::True)),
				"false" => x.properties.contains(&Property::TripWireHookPowered(TripWireHookPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Tripwire => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "attached").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripwireAttached(TripwireAttached::True)),
				"false" => x.properties.contains(&Property::TripwireAttached(TripwireAttached::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "disarmed").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripwireDisarmed(TripwireDisarmed::True)),
				"false" => x.properties.contains(&Property::TripwireDisarmed(TripwireDisarmed::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripwireEast(TripwireEast::True)),
				"false" => x.properties.contains(&Property::TripwireEast(TripwireEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripwireNorth(TripwireNorth::True)),
				"false" => x.properties.contains(&Property::TripwireNorth(TripwireNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripwirePowered(TripwirePowered::True)),
				"false" => x.properties.contains(&Property::TripwirePowered(TripwirePowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripwireSouth(TripwireSouth::True)),
				"false" => x.properties.contains(&Property::TripwireSouth(TripwireSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripwireWest(TripwireWest::True)),
				"false" => x.properties.contains(&Property::TripwireWest(TripwireWest::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::TurtleEgg => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "eggs").unwrap().1.as_str() {
				"1" => x.properties.contains(&Property::TurtleEggEggs(TurtleEggEggs::Num1)),
				"2" => x.properties.contains(&Property::TurtleEggEggs(TurtleEggEggs::Num2)),
				"3" => x.properties.contains(&Property::TurtleEggEggs(TurtleEggEggs::Num3)),
				"4" => x.properties.contains(&Property::TurtleEggEggs(TurtleEggEggs::Num4)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hatch").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::TurtleEggHatch(TurtleEggHatch::Num0)),
				"1" => x.properties.contains(&Property::TurtleEggHatch(TurtleEggHatch::Num1)),
				"2" => x.properties.contains(&Property::TurtleEggHatch(TurtleEggHatch::Num2)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::TwistingVines => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num0)),
				"1" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num1)),
				"2" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num2)),
				"3" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num3)),
				"4" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num4)),
				"5" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num5)),
				"6" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num6)),
				"7" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num7)),
				"8" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num8)),
				"9" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num9)),
				"10" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num10)),
				"11" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num11)),
				"12" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num12)),
				"13" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num13)),
				"14" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num14)),
				"15" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num15)),
				"16" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num16)),
				"17" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num17)),
				"18" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num18)),
				"19" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num19)),
				"20" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num20)),
				"21" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num21)),
				"22" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num22)),
				"23" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num23)),
				"24" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num24)),
				"25" => x.properties.contains(&Property::TwistingVinesAge(TwistingVinesAge::Num25)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::TwistingVinesPlant => {
			block.states.first().unwrap().id
		},
		Type::Uniform => {
			block.states.first().unwrap().id
		},
		Type::UntintedParticleLeaves => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "distance").unwrap().1.as_str() {
				"1" => x.properties.contains(&Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num1)),
				"2" => x.properties.contains(&Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num2)),
				"3" => x.properties.contains(&Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num3)),
				"4" => x.properties.contains(&Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num4)),
				"5" => x.properties.contains(&Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num5)),
				"6" => x.properties.contains(&Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num6)),
				"7" => x.properties.contains(&Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num7)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "persistent").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::UntintedParticleLeavesPersistent(UntintedParticleLeavesPersistent::True)),
				"false" => x.properties.contains(&Property::UntintedParticleLeavesPersistent(UntintedParticleLeavesPersistent::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::UntintedParticleLeavesWaterlogged(UntintedParticleLeavesWaterlogged::True)),
				"false" => x.properties.contains(&Property::UntintedParticleLeavesWaterlogged(UntintedParticleLeavesWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Vault => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::VaultFacing(VaultFacing::North)),
				"south" => x.properties.contains(&Property::VaultFacing(VaultFacing::South)),
				"west" => x.properties.contains(&Property::VaultFacing(VaultFacing::West)),
				"east" => x.properties.contains(&Property::VaultFacing(VaultFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "ominous").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::VaultOminous(VaultOminous::True)),
				"false" => x.properties.contains(&Property::VaultOminous(VaultOminous::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "vault_state").unwrap().1.as_str() {
				"inactive" => x.properties.contains(&Property::VaultVaultState(VaultVaultState::Inactive)),
				"active" => x.properties.contains(&Property::VaultVaultState(VaultVaultState::Active)),
				"unlocking" => x.properties.contains(&Property::VaultVaultState(VaultVaultState::Unlocking)),
				"ejecting" => x.properties.contains(&Property::VaultVaultState(VaultVaultState::Ejecting)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Vine => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::VineEast(VineEast::True)),
				"false" => x.properties.contains(&Property::VineEast(VineEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::VineNorth(VineNorth::True)),
				"false" => x.properties.contains(&Property::VineNorth(VineNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::VineSouth(VineSouth::True)),
				"false" => x.properties.contains(&Property::VineSouth(VineSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "up").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::VineUp(VineUp::True)),
				"false" => x.properties.contains(&Property::VineUp(VineUp::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::VineWest(VineWest::True)),
				"false" => x.properties.contains(&Property::VineWest(VineWest::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Wall => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::WallEast(WallEast::None)),
				"low" => x.properties.contains(&Property::WallEast(WallEast::Low)),
				"tall" => x.properties.contains(&Property::WallEast(WallEast::Tall)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::WallNorth(WallNorth::None)),
				"low" => x.properties.contains(&Property::WallNorth(WallNorth::Low)),
				"tall" => x.properties.contains(&Property::WallNorth(WallNorth::Tall)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::WallSouth(WallSouth::None)),
				"low" => x.properties.contains(&Property::WallSouth(WallSouth::Low)),
				"tall" => x.properties.contains(&Property::WallSouth(WallSouth::Tall)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "up").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WallUp(WallUp::True)),
				"false" => x.properties.contains(&Property::WallUp(WallUp::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WallWaterlogged(WallWaterlogged::True)),
				"false" => x.properties.contains(&Property::WallWaterlogged(WallWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::WallWest(WallWest::None)),
				"low" => x.properties.contains(&Property::WallWest(WallWest::Low)),
				"tall" => x.properties.contains(&Property::WallWest(WallWest::Tall)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WallBanner => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WallBannerFacing(WallBannerFacing::North)),
				"south" => x.properties.contains(&Property::WallBannerFacing(WallBannerFacing::South)),
				"west" => x.properties.contains(&Property::WallBannerFacing(WallBannerFacing::West)),
				"east" => x.properties.contains(&Property::WallBannerFacing(WallBannerFacing::East)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WallHangingSign => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WallHangingSignFacing(WallHangingSignFacing::North)),
				"south" => x.properties.contains(&Property::WallHangingSignFacing(WallHangingSignFacing::South)),
				"west" => x.properties.contains(&Property::WallHangingSignFacing(WallHangingSignFacing::West)),
				"east" => x.properties.contains(&Property::WallHangingSignFacing(WallHangingSignFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WallHangingSignWaterlogged(WallHangingSignWaterlogged::True)),
				"false" => x.properties.contains(&Property::WallHangingSignWaterlogged(WallHangingSignWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WallSign => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WallSignFacing(WallSignFacing::North)),
				"south" => x.properties.contains(&Property::WallSignFacing(WallSignFacing::South)),
				"west" => x.properties.contains(&Property::WallSignFacing(WallSignFacing::West)),
				"east" => x.properties.contains(&Property::WallSignFacing(WallSignFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WallSignWaterlogged(WallSignWaterlogged::True)),
				"false" => x.properties.contains(&Property::WallSignWaterlogged(WallSignWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WallSkull => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WallSkullFacing(WallSkullFacing::North)),
				"south" => x.properties.contains(&Property::WallSkullFacing(WallSkullFacing::South)),
				"west" => x.properties.contains(&Property::WallSkullFacing(WallSkullFacing::West)),
				"east" => x.properties.contains(&Property::WallSkullFacing(WallSkullFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WallSkullPowered(WallSkullPowered::True)),
				"false" => x.properties.contains(&Property::WallSkullPowered(WallSkullPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WallTorch => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WallTorchFacing(WallTorchFacing::North)),
				"south" => x.properties.contains(&Property::WallTorchFacing(WallTorchFacing::South)),
				"west" => x.properties.contains(&Property::WallTorchFacing(WallTorchFacing::West)),
				"east" => x.properties.contains(&Property::WallTorchFacing(WallTorchFacing::East)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Waterlily => {
			block.states.first().unwrap().id
		},
		Type::WaterloggedTransparent => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WaterloggedTransparentWaterlogged(WaterloggedTransparentWaterlogged::True)),
				"false" => x.properties.contains(&Property::WaterloggedTransparentWaterlogged(WaterloggedTransparentWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringCopperBar => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperBarEast(WeatheringCopperBarEast::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperBarEast(WeatheringCopperBarEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperBarNorth(WeatheringCopperBarNorth::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperBarNorth(WeatheringCopperBarNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperBarSouth(WeatheringCopperBarSouth::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperBarSouth(WeatheringCopperBarSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperBarWaterlogged(WeatheringCopperBarWaterlogged::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperBarWaterlogged(WeatheringCopperBarWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperBarWest(WeatheringCopperBarWest::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperBarWest(WeatheringCopperBarWest::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringCopperBulb => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperBulbLit(WeatheringCopperBulbLit::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperBulbLit(WeatheringCopperBulbLit::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperBulbPowered(WeatheringCopperBulbPowered::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperBulbPowered(WeatheringCopperBulbPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringCopperChain => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "axis").unwrap().1.as_str() {
				"x" => x.properties.contains(&Property::WeatheringCopperChainAxis(WeatheringCopperChainAxis::X)),
				"y" => x.properties.contains(&Property::WeatheringCopperChainAxis(WeatheringCopperChainAxis::Y)),
				"z" => x.properties.contains(&Property::WeatheringCopperChainAxis(WeatheringCopperChainAxis::Z)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperChainWaterlogged(WeatheringCopperChainWaterlogged::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperChainWaterlogged(WeatheringCopperChainWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringCopperChest => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WeatheringCopperChestFacing(WeatheringCopperChestFacing::North)),
				"south" => x.properties.contains(&Property::WeatheringCopperChestFacing(WeatheringCopperChestFacing::South)),
				"west" => x.properties.contains(&Property::WeatheringCopperChestFacing(WeatheringCopperChestFacing::West)),
				"east" => x.properties.contains(&Property::WeatheringCopperChestFacing(WeatheringCopperChestFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "type").unwrap().1.as_str() {
				"single" => x.properties.contains(&Property::WeatheringCopperChestType(WeatheringCopperChestType::Single)),
				"left" => x.properties.contains(&Property::WeatheringCopperChestType(WeatheringCopperChestType::Left)),
				"right" => x.properties.contains(&Property::WeatheringCopperChestType(WeatheringCopperChestType::Right)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperChestWaterlogged(WeatheringCopperChestWaterlogged::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperChestWaterlogged(WeatheringCopperChestWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringCopperDoor => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::North)),
				"south" => x.properties.contains(&Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::South)),
				"west" => x.properties.contains(&Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::West)),
				"east" => x.properties.contains(&Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"upper" => x.properties.contains(&Property::WeatheringCopperDoorHalf(WeatheringCopperDoorHalf::Upper)),
				"lower" => x.properties.contains(&Property::WeatheringCopperDoorHalf(WeatheringCopperDoorHalf::Lower)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hinge").unwrap().1.as_str() {
				"left" => x.properties.contains(&Property::WeatheringCopperDoorHinge(WeatheringCopperDoorHinge::Left)),
				"right" => x.properties.contains(&Property::WeatheringCopperDoorHinge(WeatheringCopperDoorHinge::Right)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "open").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperDoorOpen(WeatheringCopperDoorOpen::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperDoorOpen(WeatheringCopperDoorOpen::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperDoorPowered(WeatheringCopperDoorPowered::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperDoorPowered(WeatheringCopperDoorPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringCopperFull => {
			block.states.first().unwrap().id
		},
		Type::WeatheringCopperGolemStatue => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "copper_golem_pose").unwrap().1.as_str() {
				"standing" => x.properties.contains(&Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Standing)),
				"sitting" => x.properties.contains(&Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Sitting)),
				"running" => x.properties.contains(&Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Running)),
				"star" => x.properties.contains(&Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Star)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WeatheringCopperGolemStatueFacing(WeatheringCopperGolemStatueFacing::North)),
				"south" => x.properties.contains(&Property::WeatheringCopperGolemStatueFacing(WeatheringCopperGolemStatueFacing::South)),
				"west" => x.properties.contains(&Property::WeatheringCopperGolemStatueFacing(WeatheringCopperGolemStatueFacing::West)),
				"east" => x.properties.contains(&Property::WeatheringCopperGolemStatueFacing(WeatheringCopperGolemStatueFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperGolemStatueWaterlogged(WeatheringCopperGolemStatueWaterlogged::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperGolemStatueWaterlogged(WeatheringCopperGolemStatueWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringCopperGrate => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperGrateWaterlogged(WeatheringCopperGrateWaterlogged::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperGrateWaterlogged(WeatheringCopperGrateWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringCopperSlab => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "type").unwrap().1.as_str() {
				"top" => x.properties.contains(&Property::WeatheringCopperSlabType(WeatheringCopperSlabType::Top)),
				"bottom" => x.properties.contains(&Property::WeatheringCopperSlabType(WeatheringCopperSlabType::Bottom)),
				"double" => x.properties.contains(&Property::WeatheringCopperSlabType(WeatheringCopperSlabType::Double)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperSlabWaterlogged(WeatheringCopperSlabWaterlogged::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperSlabWaterlogged(WeatheringCopperSlabWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringCopperStair => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WeatheringCopperStairFacing(WeatheringCopperStairFacing::North)),
				"south" => x.properties.contains(&Property::WeatheringCopperStairFacing(WeatheringCopperStairFacing::South)),
				"west" => x.properties.contains(&Property::WeatheringCopperStairFacing(WeatheringCopperStairFacing::West)),
				"east" => x.properties.contains(&Property::WeatheringCopperStairFacing(WeatheringCopperStairFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"top" => x.properties.contains(&Property::WeatheringCopperStairHalf(WeatheringCopperStairHalf::Top)),
				"bottom" => x.properties.contains(&Property::WeatheringCopperStairHalf(WeatheringCopperStairHalf::Bottom)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "shape").unwrap().1.as_str() {
				"straight" => x.properties.contains(&Property::WeatheringCopperStairShape(WeatheringCopperStairShape::Straight)),
				"inner_left" => x.properties.contains(&Property::WeatheringCopperStairShape(WeatheringCopperStairShape::InnerLeft)),
				"inner_right" => x.properties.contains(&Property::WeatheringCopperStairShape(WeatheringCopperStairShape::InnerRight)),
				"outer_left" => x.properties.contains(&Property::WeatheringCopperStairShape(WeatheringCopperStairShape::OuterLeft)),
				"outer_right" => x.properties.contains(&Property::WeatheringCopperStairShape(WeatheringCopperStairShape::OuterRight)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperStairWaterlogged(WeatheringCopperStairWaterlogged::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperStairWaterlogged(WeatheringCopperStairWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringCopperTrapDoor => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::North)),
				"south" => x.properties.contains(&Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::South)),
				"west" => x.properties.contains(&Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::West)),
				"east" => x.properties.contains(&Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"top" => x.properties.contains(&Property::WeatheringCopperTrapDoorHalf(WeatheringCopperTrapDoorHalf::Top)),
				"bottom" => x.properties.contains(&Property::WeatheringCopperTrapDoorHalf(WeatheringCopperTrapDoorHalf::Bottom)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "open").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperTrapDoorOpen(WeatheringCopperTrapDoorOpen::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperTrapDoorOpen(WeatheringCopperTrapDoorOpen::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperTrapDoorPowered(WeatheringCopperTrapDoorPowered::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperTrapDoorPowered(WeatheringCopperTrapDoorPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperTrapDoorWaterlogged(WeatheringCopperTrapDoorWaterlogged::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperTrapDoorWaterlogged(WeatheringCopperTrapDoorWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringLantern => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hanging").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringLanternHanging(WeatheringLanternHanging::True)),
				"false" => x.properties.contains(&Property::WeatheringLanternHanging(WeatheringLanternHanging::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringLanternWaterlogged(WeatheringLanternWaterlogged::True)),
				"false" => x.properties.contains(&Property::WeatheringLanternWaterlogged(WeatheringLanternWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringLightningRod => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::North)),
				"east" => x.properties.contains(&Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::East)),
				"south" => x.properties.contains(&Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::South)),
				"west" => x.properties.contains(&Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::West)),
				"up" => x.properties.contains(&Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::Up)),
				"down" => x.properties.contains(&Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::Down)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringLightningRodPowered(WeatheringLightningRodPowered::True)),
				"false" => x.properties.contains(&Property::WeatheringLightningRodPowered(WeatheringLightningRodPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringLightningRodWaterlogged(WeatheringLightningRodWaterlogged::True)),
				"false" => x.properties.contains(&Property::WeatheringLightningRodWaterlogged(WeatheringLightningRodWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Web => {
			block.states.first().unwrap().id
		},
		Type::WeepingVines => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num0)),
				"1" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num1)),
				"2" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num2)),
				"3" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num3)),
				"4" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num4)),
				"5" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num5)),
				"6" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num6)),
				"7" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num7)),
				"8" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num8)),
				"9" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num9)),
				"10" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num10)),
				"11" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num11)),
				"12" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num12)),
				"13" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num13)),
				"14" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num14)),
				"15" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num15)),
				"16" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num16)),
				"17" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num17)),
				"18" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num18)),
				"19" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num19)),
				"20" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num20)),
				"21" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num21)),
				"22" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num22)),
				"23" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num23)),
				"24" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num24)),
				"25" => x.properties.contains(&Property::WeepingVinesAge(WeepingVinesAge::Num25)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeepingVinesPlant => {
			block.states.first().unwrap().id
		},
		Type::WeightedPressurePlate => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "power").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num0)),
				"1" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num1)),
				"2" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num2)),
				"3" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num3)),
				"4" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num4)),
				"5" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num5)),
				"6" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num6)),
				"7" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num7)),
				"8" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num8)),
				"9" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num9)),
				"10" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num10)),
				"11" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num11)),
				"12" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num12)),
				"13" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num13)),
				"14" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num14)),
				"15" => x.properties.contains(&Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num15)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WetSponge => {
			block.states.first().unwrap().id
		},
		Type::WitherRose => {
			block.states.first().unwrap().id
		},
		Type::WitherSkull => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WitherSkullPowered(WitherSkullPowered::True)),
				"false" => x.properties.contains(&Property::WitherSkullPowered(WitherSkullPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "rotation").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num0)),
				"1" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num1)),
				"2" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num2)),
				"3" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num3)),
				"4" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num4)),
				"5" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num5)),
				"6" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num6)),
				"7" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num7)),
				"8" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num8)),
				"9" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num9)),
				"10" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num10)),
				"11" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num11)),
				"12" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num12)),
				"13" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num13)),
				"14" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num14)),
				"15" => x.properties.contains(&Property::WitherSkullRotation(WitherSkullRotation::Num15)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WitherWallSkull => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WitherWallSkullFacing(WitherWallSkullFacing::North)),
				"south" => x.properties.contains(&Property::WitherWallSkullFacing(WitherWallSkullFacing::South)),
				"west" => x.properties.contains(&Property::WitherWallSkullFacing(WitherWallSkullFacing::West)),
				"east" => x.properties.contains(&Property::WitherWallSkullFacing(WitherWallSkullFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WitherWallSkullPowered(WitherWallSkullPowered::True)),
				"false" => x.properties.contains(&Property::WitherWallSkullPowered(WitherWallSkullPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WoolCarpet => {
			block.states.first().unwrap().id
		},
	};
}
