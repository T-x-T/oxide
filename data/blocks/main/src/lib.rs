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
		match property {
			Property::SculkShriekerWaterlogged(SculkShriekerWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::SculkShriekerWaterlogged(SculkShriekerWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::JigsawOrientation(JigsawOrientation::DownEast) => output.push(("orientation".to_string(), "down_east".to_string())),
			Property::JigsawOrientation(JigsawOrientation::DownNorth) => output.push(("orientation".to_string(), "down_north".to_string())),
			Property::JigsawOrientation(JigsawOrientation::DownSouth) => output.push(("orientation".to_string(), "down_south".to_string())),
			Property::JigsawOrientation(JigsawOrientation::DownWest) => output.push(("orientation".to_string(), "down_west".to_string())),
			Property::JigsawOrientation(JigsawOrientation::UpEast) => output.push(("orientation".to_string(), "up_east".to_string())),
			Property::JigsawOrientation(JigsawOrientation::UpNorth) => output.push(("orientation".to_string(), "up_north".to_string())),
			Property::JigsawOrientation(JigsawOrientation::UpSouth) => output.push(("orientation".to_string(), "up_south".to_string())),
			Property::JigsawOrientation(JigsawOrientation::UpWest) => output.push(("orientation".to_string(), "up_west".to_string())),
			Property::JigsawOrientation(JigsawOrientation::WestUp) => output.push(("orientation".to_string(), "west_up".to_string())),
			Property::JigsawOrientation(JigsawOrientation::EastUp) => output.push(("orientation".to_string(), "east_up".to_string())),
			Property::JigsawOrientation(JigsawOrientation::NorthUp) => output.push(("orientation".to_string(), "north_up".to_string())),
			Property::JigsawOrientation(JigsawOrientation::SouthUp) => output.push(("orientation".to_string(), "south_up".to_string())),
			Property::CommandFacing(CommandFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::CommandFacing(CommandFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::CommandFacing(CommandFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::CommandFacing(CommandFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::CommandFacing(CommandFacing::Up) => output.push(("facing".to_string(), "up".to_string())),
			Property::CommandFacing(CommandFacing::Down) => output.push(("facing".to_string(), "down".to_string())),
			Property::MultifaceNorth(MultifaceNorth::True) => output.push(("north".to_string(), "true".to_string())),
			Property::MultifaceNorth(MultifaceNorth::False) => output.push(("north".to_string(), "false".to_string())),
			Property::RedstoneWireWest(RedstoneWireWest::Up) => output.push(("west".to_string(), "up".to_string())),
			Property::RedstoneWireWest(RedstoneWireWest::Side) => output.push(("west".to_string(), "side".to_string())),
			Property::RedstoneWireWest(RedstoneWireWest::None) => output.push(("west".to_string(), "none".to_string())),
			Property::CopperBulbBlockPowered(CopperBulbBlockPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::CopperBulbBlockPowered(CopperBulbBlockPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::BellPowered(BellPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::BellPowered(BellPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::TrapdoorHalf(TrapdoorHalf::Top) => output.push(("half".to_string(), "top".to_string())),
			Property::TrapdoorHalf(TrapdoorHalf::Bottom) => output.push(("half".to_string(), "bottom".to_string())),
			Property::DecoratedPotCracked(DecoratedPotCracked::True) => output.push(("cracked".to_string(), "true".to_string())),
			Property::DecoratedPotCracked(DecoratedPotCracked::False) => output.push(("cracked".to_string(), "false".to_string())),
			Property::PoweredRailShape(PoweredRailShape::NorthSouth) => output.push(("shape".to_string(), "north_south".to_string())),
			Property::PoweredRailShape(PoweredRailShape::EastWest) => output.push(("shape".to_string(), "east_west".to_string())),
			Property::PoweredRailShape(PoweredRailShape::AscendingEast) => output.push(("shape".to_string(), "ascending_east".to_string())),
			Property::PoweredRailShape(PoweredRailShape::AscendingWest) => output.push(("shape".to_string(), "ascending_west".to_string())),
			Property::PoweredRailShape(PoweredRailShape::AscendingNorth) => output.push(("shape".to_string(), "ascending_north".to_string())),
			Property::PoweredRailShape(PoweredRailShape::AscendingSouth) => output.push(("shape".to_string(), "ascending_south".to_string())),
			Property::WeatheringCopperSlabWaterlogged(WeatheringCopperSlabWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::WeatheringCopperSlabWaterlogged(WeatheringCopperSlabWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::TintedParticleLeavesWaterlogged(TintedParticleLeavesWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::TintedParticleLeavesWaterlogged(TintedParticleLeavesWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::AttachedStemFacing(AttachedStemFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::AttachedStemFacing(AttachedStemFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::AttachedStemFacing(AttachedStemFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::AttachedStemFacing(AttachedStemFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::CalibratedSculkSensorWaterlogged(CalibratedSculkSensorWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::CalibratedSculkSensorWaterlogged(CalibratedSculkSensorWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::WeatheringCopperBulbLit(WeatheringCopperBulbLit::True) => output.push(("lit".to_string(), "true".to_string())),
			Property::WeatheringCopperBulbLit(WeatheringCopperBulbLit::False) => output.push(("lit".to_string(), "false".to_string())),
			Property::ChorusPlantEast(ChorusPlantEast::True) => output.push(("east".to_string(), "true".to_string())),
			Property::ChorusPlantEast(ChorusPlantEast::False) => output.push(("east".to_string(), "false".to_string())),
			Property::ShelfPowered(ShelfPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::ShelfPowered(ShelfPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::StairHalf(StairHalf::Top) => output.push(("half".to_string(), "top".to_string())),
			Property::StairHalf(StairHalf::Bottom) => output.push(("half".to_string(), "bottom".to_string())),
			Property::StainedGlassPaneNorth(StainedGlassPaneNorth::True) => output.push(("north".to_string(), "true".to_string())),
			Property::StainedGlassPaneNorth(StainedGlassPaneNorth::False) => output.push(("north".to_string(), "false".to_string())),
			Property::LeafLitterSegmentAmount(LeafLitterSegmentAmount::Num1) => output.push(("segment_amount".to_string(), "1".to_string())),
			Property::LeafLitterSegmentAmount(LeafLitterSegmentAmount::Num2) => output.push(("segment_amount".to_string(), "2".to_string())),
			Property::LeafLitterSegmentAmount(LeafLitterSegmentAmount::Num3) => output.push(("segment_amount".to_string(), "3".to_string())),
			Property::LeafLitterSegmentAmount(LeafLitterSegmentAmount::Num4) => output.push(("segment_amount".to_string(), "4".to_string())),
			Property::LightLevel(LightLevel::Num0) => output.push(("level".to_string(), "0".to_string())),
			Property::LightLevel(LightLevel::Num1) => output.push(("level".to_string(), "1".to_string())),
			Property::LightLevel(LightLevel::Num2) => output.push(("level".to_string(), "2".to_string())),
			Property::LightLevel(LightLevel::Num3) => output.push(("level".to_string(), "3".to_string())),
			Property::LightLevel(LightLevel::Num4) => output.push(("level".to_string(), "4".to_string())),
			Property::LightLevel(LightLevel::Num5) => output.push(("level".to_string(), "5".to_string())),
			Property::LightLevel(LightLevel::Num6) => output.push(("level".to_string(), "6".to_string())),
			Property::LightLevel(LightLevel::Num7) => output.push(("level".to_string(), "7".to_string())),
			Property::LightLevel(LightLevel::Num8) => output.push(("level".to_string(), "8".to_string())),
			Property::LightLevel(LightLevel::Num9) => output.push(("level".to_string(), "9".to_string())),
			Property::LightLevel(LightLevel::Num10) => output.push(("level".to_string(), "10".to_string())),
			Property::LightLevel(LightLevel::Num11) => output.push(("level".to_string(), "11".to_string())),
			Property::LightLevel(LightLevel::Num12) => output.push(("level".to_string(), "12".to_string())),
			Property::LightLevel(LightLevel::Num13) => output.push(("level".to_string(), "13".to_string())),
			Property::LightLevel(LightLevel::Num14) => output.push(("level".to_string(), "14".to_string())),
			Property::LightLevel(LightLevel::Num15) => output.push(("level".to_string(), "15".to_string())),
			Property::BrewingStandHasBottle1(BrewingStandHasBottle1::True) => output.push(("has_bottle_1".to_string(), "true".to_string())),
			Property::BrewingStandHasBottle1(BrewingStandHasBottle1::False) => output.push(("has_bottle_1".to_string(), "false".to_string())),
			Property::ChiseledBookShelfSlot2Occupied(ChiseledBookShelfSlot2Occupied::True) => output.push(("slot_2_occupied".to_string(), "true".to_string())),
			Property::ChiseledBookShelfSlot2Occupied(ChiseledBookShelfSlot2Occupied::False) => output.push(("slot_2_occupied".to_string(), "false".to_string())),
			Property::WeatheringCopperBarSouth(WeatheringCopperBarSouth::True) => output.push(("south".to_string(), "true".to_string())),
			Property::WeatheringCopperBarSouth(WeatheringCopperBarSouth::False) => output.push(("south".to_string(), "false".to_string())),
			Property::HopperFacing(HopperFacing::Down) => output.push(("facing".to_string(), "down".to_string())),
			Property::HopperFacing(HopperFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::HopperFacing(HopperFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::HopperFacing(HopperFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::HopperFacing(HopperFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::LightWaterlogged(LightWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::LightWaterlogged(LightWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::MossyCarpetEast(MossyCarpetEast::None) => output.push(("east".to_string(), "none".to_string())),
			Property::MossyCarpetEast(MossyCarpetEast::Low) => output.push(("east".to_string(), "low".to_string())),
			Property::MossyCarpetEast(MossyCarpetEast::Tall) => output.push(("east".to_string(), "tall".to_string())),
			Property::SmokerLit(SmokerLit::True) => output.push(("lit".to_string(), "true".to_string())),
			Property::SmokerLit(SmokerLit::False) => output.push(("lit".to_string(), "false".to_string())),
			Property::SculkVeinWest(SculkVeinWest::True) => output.push(("west".to_string(), "true".to_string())),
			Property::SculkVeinWest(SculkVeinWest::False) => output.push(("west".to_string(), "false".to_string())),
			Property::CampfireSignalFire(CampfireSignalFire::True) => output.push(("signal_fire".to_string(), "true".to_string())),
			Property::CampfireSignalFire(CampfireSignalFire::False) => output.push(("signal_fire".to_string(), "false".to_string())),
			Property::DropperFacing(DropperFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::DropperFacing(DropperFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::DropperFacing(DropperFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::DropperFacing(DropperFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::DropperFacing(DropperFacing::Up) => output.push(("facing".to_string(), "up".to_string())),
			Property::DropperFacing(DropperFacing::Down) => output.push(("facing".to_string(), "down".to_string())),
			Property::LeafLitterFacing(LeafLitterFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::LeafLitterFacing(LeafLitterFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::LeafLitterFacing(LeafLitterFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::LeafLitterFacing(LeafLitterFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::WeatheringCopperStairShape(WeatheringCopperStairShape::Straight) => output.push(("shape".to_string(), "straight".to_string())),
			Property::WeatheringCopperStairShape(WeatheringCopperStairShape::InnerLeft) => output.push(("shape".to_string(), "inner_left".to_string())),
			Property::WeatheringCopperStairShape(WeatheringCopperStairShape::InnerRight) => output.push(("shape".to_string(), "inner_right".to_string())),
			Property::WeatheringCopperStairShape(WeatheringCopperStairShape::OuterLeft) => output.push(("shape".to_string(), "outer_left".to_string())),
			Property::WeatheringCopperStairShape(WeatheringCopperStairShape::OuterRight) => output.push(("shape".to_string(), "outer_right".to_string())),
			Property::PistonBaseFacing(PistonBaseFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::PistonBaseFacing(PistonBaseFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::PistonBaseFacing(PistonBaseFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::PistonBaseFacing(PistonBaseFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::PistonBaseFacing(PistonBaseFacing::Up) => output.push(("facing".to_string(), "up".to_string())),
			Property::PistonBaseFacing(PistonBaseFacing::Down) => output.push(("facing".to_string(), "down".to_string())),
			Property::EnderChestFacing(EnderChestFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::EnderChestFacing(EnderChestFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::EnderChestFacing(EnderChestFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::EnderChestFacing(EnderChestFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::CreakingHeartCreakingHeartState(CreakingHeartCreakingHeartState::Uprooted) => output.push(("creaking_heart_state".to_string(), "uprooted".to_string())),
			Property::CreakingHeartCreakingHeartState(CreakingHeartCreakingHeartState::Dormant) => output.push(("creaking_heart_state".to_string(), "dormant".to_string())),
			Property::CreakingHeartCreakingHeartState(CreakingHeartCreakingHeartState::Awake) => output.push(("creaking_heart_state".to_string(), "awake".to_string())),
			Property::LoomFacing(LoomFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::LoomFacing(LoomFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::LoomFacing(LoomFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::LoomFacing(LoomFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::MultifaceEast(MultifaceEast::True) => output.push(("east".to_string(), "true".to_string())),
			Property::MultifaceEast(MultifaceEast::False) => output.push(("east".to_string(), "false".to_string())),
			Property::SeaPicklePickles(SeaPicklePickles::Num1) => output.push(("pickles".to_string(), "1".to_string())),
			Property::SeaPicklePickles(SeaPicklePickles::Num2) => output.push(("pickles".to_string(), "2".to_string())),
			Property::SeaPicklePickles(SeaPicklePickles::Num3) => output.push(("pickles".to_string(), "3".to_string())),
			Property::SeaPicklePickles(SeaPicklePickles::Num4) => output.push(("pickles".to_string(), "4".to_string())),
			Property::WeatheringCopperDoorOpen(WeatheringCopperDoorOpen::True) => output.push(("open".to_string(), "true".to_string())),
			Property::WeatheringCopperDoorOpen(WeatheringCopperDoorOpen::False) => output.push(("open".to_string(), "false".to_string())),
			Property::ComposterLevel(ComposterLevel::Num0) => output.push(("level".to_string(), "0".to_string())),
			Property::ComposterLevel(ComposterLevel::Num1) => output.push(("level".to_string(), "1".to_string())),
			Property::ComposterLevel(ComposterLevel::Num2) => output.push(("level".to_string(), "2".to_string())),
			Property::ComposterLevel(ComposterLevel::Num3) => output.push(("level".to_string(), "3".to_string())),
			Property::ComposterLevel(ComposterLevel::Num4) => output.push(("level".to_string(), "4".to_string())),
			Property::ComposterLevel(ComposterLevel::Num5) => output.push(("level".to_string(), "5".to_string())),
			Property::ComposterLevel(ComposterLevel::Num6) => output.push(("level".to_string(), "6".to_string())),
			Property::ComposterLevel(ComposterLevel::Num7) => output.push(("level".to_string(), "7".to_string())),
			Property::ComposterLevel(ComposterLevel::Num8) => output.push(("level".to_string(), "8".to_string())),
			Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::IronBarsNorth(IronBarsNorth::True) => output.push(("north".to_string(), "true".to_string())),
			Property::IronBarsNorth(IronBarsNorth::False) => output.push(("north".to_string(), "false".to_string())),
			Property::AmethystClusterFacing(AmethystClusterFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::AmethystClusterFacing(AmethystClusterFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::AmethystClusterFacing(AmethystClusterFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::AmethystClusterFacing(AmethystClusterFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::AmethystClusterFacing(AmethystClusterFacing::Up) => output.push(("facing".to_string(), "up".to_string())),
			Property::AmethystClusterFacing(AmethystClusterFacing::Down) => output.push(("facing".to_string(), "down".to_string())),
			Property::GlowLichenUp(GlowLichenUp::True) => output.push(("up".to_string(), "true".to_string())),
			Property::GlowLichenUp(GlowLichenUp::False) => output.push(("up".to_string(), "false".to_string())),
			Property::PointedDripstoneThickness(PointedDripstoneThickness::TipMerge) => output.push(("thickness".to_string(), "tip_merge".to_string())),
			Property::PointedDripstoneThickness(PointedDripstoneThickness::Tip) => output.push(("thickness".to_string(), "tip".to_string())),
			Property::PointedDripstoneThickness(PointedDripstoneThickness::Frustum) => output.push(("thickness".to_string(), "frustum".to_string())),
			Property::PointedDripstoneThickness(PointedDripstoneThickness::Middle) => output.push(("thickness".to_string(), "middle".to_string())),
			Property::PointedDripstoneThickness(PointedDripstoneThickness::Base) => output.push(("thickness".to_string(), "base".to_string())),
			Property::ScaffoldingWaterlogged(ScaffoldingWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::ScaffoldingWaterlogged(ScaffoldingWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::BrushableDusted(BrushableDusted::Num0) => output.push(("dusted".to_string(), "0".to_string())),
			Property::BrushableDusted(BrushableDusted::Num1) => output.push(("dusted".to_string(), "1".to_string())),
			Property::BrushableDusted(BrushableDusted::Num2) => output.push(("dusted".to_string(), "2".to_string())),
			Property::BrushableDusted(BrushableDusted::Num3) => output.push(("dusted".to_string(), "3".to_string())),
			Property::LanternWaterlogged(LanternWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::LanternWaterlogged(LanternWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::WeatheringCopperChestFacing(WeatheringCopperChestFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::WeatheringCopperChestFacing(WeatheringCopperChestFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::WeatheringCopperChestFacing(WeatheringCopperChestFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::WeatheringCopperChestFacing(WeatheringCopperChestFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::SweetBerryBushAge(SweetBerryBushAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::SweetBerryBushAge(SweetBerryBushAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::SweetBerryBushAge(SweetBerryBushAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::SweetBerryBushAge(SweetBerryBushAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::TripwirePowered(TripwirePowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::TripwirePowered(TripwirePowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::RepeaterPowered(RepeaterPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::RepeaterPowered(RepeaterPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::TrialSpawnerOminous(TrialSpawnerOminous::True) => output.push(("ominous".to_string(), "true".to_string())),
			Property::TrialSpawnerOminous(TrialSpawnerOminous::False) => output.push(("ominous".to_string(), "false".to_string())),
			Property::CandleLit(CandleLit::True) => output.push(("lit".to_string(), "true".to_string())),
			Property::CandleLit(CandleLit::False) => output.push(("lit".to_string(), "false".to_string())),
			Property::ChorusPlantNorth(ChorusPlantNorth::True) => output.push(("north".to_string(), "true".to_string())),
			Property::ChorusPlantNorth(ChorusPlantNorth::False) => output.push(("north".to_string(), "false".to_string())),
			Property::DropperTriggered(DropperTriggered::True) => output.push(("triggered".to_string(), "true".to_string())),
			Property::DropperTriggered(DropperTriggered::False) => output.push(("triggered".to_string(), "false".to_string())),
			Property::SmallDripleafFacing(SmallDripleafFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::SmallDripleafFacing(SmallDripleafFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::SmallDripleafFacing(SmallDripleafFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::SmallDripleafFacing(SmallDripleafFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::WeatheringCopperChainAxis(WeatheringCopperChainAxis::X) => output.push(("axis".to_string(), "x".to_string())),
			Property::WeatheringCopperChainAxis(WeatheringCopperChainAxis::Y) => output.push(("axis".to_string(), "y".to_string())),
			Property::WeatheringCopperChainAxis(WeatheringCopperChainAxis::Z) => output.push(("axis".to_string(), "z".to_string())),
			Property::TripwireSouth(TripwireSouth::True) => output.push(("south".to_string(), "true".to_string())),
			Property::TripwireSouth(TripwireSouth::False) => output.push(("south".to_string(), "false".to_string())),
			Property::HangingMossTip(HangingMossTip::True) => output.push(("tip".to_string(), "true".to_string())),
			Property::HangingMossTip(HangingMossTip::False) => output.push(("tip".to_string(), "false".to_string())),
			Property::SkullPowered(SkullPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::SkullPowered(SkullPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::StairWaterlogged(StairWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::StairWaterlogged(StairWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::BubbleColumnDrag(BubbleColumnDrag::True) => output.push(("drag".to_string(), "true".to_string())),
			Property::BubbleColumnDrag(BubbleColumnDrag::False) => output.push(("drag".to_string(), "false".to_string())),
			Property::ChiseledBookShelfSlot5Occupied(ChiseledBookShelfSlot5Occupied::True) => output.push(("slot_5_occupied".to_string(), "true".to_string())),
			Property::ChiseledBookShelfSlot5Occupied(ChiseledBookShelfSlot5Occupied::False) => output.push(("slot_5_occupied".to_string(), "false".to_string())),
			Property::GlowLichenSouth(GlowLichenSouth::True) => output.push(("south".to_string(), "true".to_string())),
			Property::GlowLichenSouth(GlowLichenSouth::False) => output.push(("south".to_string(), "false".to_string())),
			Property::WallUp(WallUp::True) => output.push(("up".to_string(), "true".to_string())),
			Property::WallUp(WallUp::False) => output.push(("up".to_string(), "false".to_string())),
			Property::FarmMoisture(FarmMoisture::Num0) => output.push(("moisture".to_string(), "0".to_string())),
			Property::FarmMoisture(FarmMoisture::Num1) => output.push(("moisture".to_string(), "1".to_string())),
			Property::FarmMoisture(FarmMoisture::Num2) => output.push(("moisture".to_string(), "2".to_string())),
			Property::FarmMoisture(FarmMoisture::Num3) => output.push(("moisture".to_string(), "3".to_string())),
			Property::FarmMoisture(FarmMoisture::Num4) => output.push(("moisture".to_string(), "4".to_string())),
			Property::FarmMoisture(FarmMoisture::Num5) => output.push(("moisture".to_string(), "5".to_string())),
			Property::FarmMoisture(FarmMoisture::Num6) => output.push(("moisture".to_string(), "6".to_string())),
			Property::FarmMoisture(FarmMoisture::Num7) => output.push(("moisture".to_string(), "7".to_string())),
			Property::JukeboxHasRecord(JukeboxHasRecord::True) => output.push(("has_record".to_string(), "true".to_string())),
			Property::JukeboxHasRecord(JukeboxHasRecord::False) => output.push(("has_record".to_string(), "false".to_string())),
			Property::StructureMode(StructureMode::Save) => output.push(("mode".to_string(), "save".to_string())),
			Property::StructureMode(StructureMode::Load) => output.push(("mode".to_string(), "load".to_string())),
			Property::StructureMode(StructureMode::Corner) => output.push(("mode".to_string(), "corner".to_string())),
			Property::StructureMode(StructureMode::Data) => output.push(("mode".to_string(), "data".to_string())),
			Property::CactusAge(CactusAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::CactusAge(CactusAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::CactusAge(CactusAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::CactusAge(CactusAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::CactusAge(CactusAge::Num4) => output.push(("age".to_string(), "4".to_string())),
			Property::CactusAge(CactusAge::Num5) => output.push(("age".to_string(), "5".to_string())),
			Property::CactusAge(CactusAge::Num6) => output.push(("age".to_string(), "6".to_string())),
			Property::CactusAge(CactusAge::Num7) => output.push(("age".to_string(), "7".to_string())),
			Property::CactusAge(CactusAge::Num8) => output.push(("age".to_string(), "8".to_string())),
			Property::CactusAge(CactusAge::Num9) => output.push(("age".to_string(), "9".to_string())),
			Property::CactusAge(CactusAge::Num10) => output.push(("age".to_string(), "10".to_string())),
			Property::CactusAge(CactusAge::Num11) => output.push(("age".to_string(), "11".to_string())),
			Property::CactusAge(CactusAge::Num12) => output.push(("age".to_string(), "12".to_string())),
			Property::CactusAge(CactusAge::Num13) => output.push(("age".to_string(), "13".to_string())),
			Property::CactusAge(CactusAge::Num14) => output.push(("age".to_string(), "14".to_string())),
			Property::CactusAge(CactusAge::Num15) => output.push(("age".to_string(), "15".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num4) => output.push(("age".to_string(), "4".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num5) => output.push(("age".to_string(), "5".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num6) => output.push(("age".to_string(), "6".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num7) => output.push(("age".to_string(), "7".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num8) => output.push(("age".to_string(), "8".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num9) => output.push(("age".to_string(), "9".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num10) => output.push(("age".to_string(), "10".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num11) => output.push(("age".to_string(), "11".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num12) => output.push(("age".to_string(), "12".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num13) => output.push(("age".to_string(), "13".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num14) => output.push(("age".to_string(), "14".to_string())),
			Property::SugarCaneAge(SugarCaneAge::Num15) => output.push(("age".to_string(), "15".to_string())),
			Property::BaseCoralFanWaterlogged(BaseCoralFanWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::BaseCoralFanWaterlogged(BaseCoralFanWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::InfestedRotatedPillarAxis(InfestedRotatedPillarAxis::X) => output.push(("axis".to_string(), "x".to_string())),
			Property::InfestedRotatedPillarAxis(InfestedRotatedPillarAxis::Y) => output.push(("axis".to_string(), "y".to_string())),
			Property::InfestedRotatedPillarAxis(InfestedRotatedPillarAxis::Z) => output.push(("axis".to_string(), "z".to_string())),
			Property::MangroveRootsWaterlogged(MangroveRootsWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::MangroveRootsWaterlogged(MangroveRootsWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::VaultFacing(VaultFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::VaultFacing(VaultFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::VaultFacing(VaultFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::VaultFacing(VaultFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::VineSouth(VineSouth::True) => output.push(("south".to_string(), "true".to_string())),
			Property::VineSouth(VineSouth::False) => output.push(("south".to_string(), "false".to_string())),
			Property::BambooStalkAge(BambooStalkAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::BambooStalkAge(BambooStalkAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::WeatheringCopperTrapDoorWaterlogged(WeatheringCopperTrapDoorWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::WeatheringCopperTrapDoorWaterlogged(WeatheringCopperTrapDoorWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::DetectorRailWaterlogged(DetectorRailWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::DetectorRailWaterlogged(DetectorRailWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::TripwireWest(TripwireWest::True) => output.push(("west".to_string(), "true".to_string())),
			Property::TripwireWest(TripwireWest::False) => output.push(("west".to_string(), "false".to_string())),
			Property::ButtonPowered(ButtonPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::ButtonPowered(ButtonPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num0) => output.push(("power".to_string(), "0".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num1) => output.push(("power".to_string(), "1".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num2) => output.push(("power".to_string(), "2".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num3) => output.push(("power".to_string(), "3".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num4) => output.push(("power".to_string(), "4".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num5) => output.push(("power".to_string(), "5".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num6) => output.push(("power".to_string(), "6".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num7) => output.push(("power".to_string(), "7".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num8) => output.push(("power".to_string(), "8".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num9) => output.push(("power".to_string(), "9".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num10) => output.push(("power".to_string(), "10".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num11) => output.push(("power".to_string(), "11".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num12) => output.push(("power".to_string(), "12".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num13) => output.push(("power".to_string(), "13".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num14) => output.push(("power".to_string(), "14".to_string())),
			Property::CalibratedSculkSensorPower(CalibratedSculkSensorPower::Num15) => output.push(("power".to_string(), "15".to_string())),
			Property::SnowyDirtSnowy(SnowyDirtSnowy::True) => output.push(("snowy".to_string(), "true".to_string())),
			Property::SnowyDirtSnowy(SnowyDirtSnowy::False) => output.push(("snowy".to_string(), "false".to_string())),
			Property::WeatheringCopperGolemStatueFacing(WeatheringCopperGolemStatueFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::WeatheringCopperGolemStatueFacing(WeatheringCopperGolemStatueFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::WeatheringCopperGolemStatueFacing(WeatheringCopperGolemStatueFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::WeatheringCopperGolemStatueFacing(WeatheringCopperGolemStatueFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num0) => output.push(("rotation".to_string(), "0".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num1) => output.push(("rotation".to_string(), "1".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num2) => output.push(("rotation".to_string(), "2".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num3) => output.push(("rotation".to_string(), "3".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num4) => output.push(("rotation".to_string(), "4".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num5) => output.push(("rotation".to_string(), "5".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num6) => output.push(("rotation".to_string(), "6".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num7) => output.push(("rotation".to_string(), "7".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num8) => output.push(("rotation".to_string(), "8".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num9) => output.push(("rotation".to_string(), "9".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num10) => output.push(("rotation".to_string(), "10".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num11) => output.push(("rotation".to_string(), "11".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num12) => output.push(("rotation".to_string(), "12".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num13) => output.push(("rotation".to_string(), "13".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num14) => output.push(("rotation".to_string(), "14".to_string())),
			Property::PlayerHeadRotation(PlayerHeadRotation::Num15) => output.push(("rotation".to_string(), "15".to_string())),
			Property::PotatoAge(PotatoAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::PotatoAge(PotatoAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::PotatoAge(PotatoAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::PotatoAge(PotatoAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::PotatoAge(PotatoAge::Num4) => output.push(("age".to_string(), "4".to_string())),
			Property::PotatoAge(PotatoAge::Num5) => output.push(("age".to_string(), "5".to_string())),
			Property::PotatoAge(PotatoAge::Num6) => output.push(("age".to_string(), "6".to_string())),
			Property::PotatoAge(PotatoAge::Num7) => output.push(("age".to_string(), "7".to_string())),
			Property::GlowLichenDown(GlowLichenDown::True) => output.push(("down".to_string(), "true".to_string())),
			Property::GlowLichenDown(GlowLichenDown::False) => output.push(("down".to_string(), "false".to_string())),
			Property::PistonHeadType(PistonHeadType::Normal) => output.push(("type".to_string(), "normal".to_string())),
			Property::PistonHeadType(PistonHeadType::Sticky) => output.push(("type".to_string(), "sticky".to_string())),
			Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num0) => output.push(("honey_level".to_string(), "0".to_string())),
			Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num1) => output.push(("honey_level".to_string(), "1".to_string())),
			Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num2) => output.push(("honey_level".to_string(), "2".to_string())),
			Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num3) => output.push(("honey_level".to_string(), "3".to_string())),
			Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num4) => output.push(("honey_level".to_string(), "4".to_string())),
			Property::BeehiveHoneyLevel(BeehiveHoneyLevel::Num5) => output.push(("honey_level".to_string(), "5".to_string())),
			Property::RedstoneLampLit(RedstoneLampLit::True) => output.push(("lit".to_string(), "true".to_string())),
			Property::RedstoneLampLit(RedstoneLampLit::False) => output.push(("lit".to_string(), "false".to_string())),
			Property::WallBannerFacing(WallBannerFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::WallBannerFacing(WallBannerFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::WallBannerFacing(WallBannerFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::WallBannerFacing(WallBannerFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::SculkSensorSculkSensorPhase(SculkSensorSculkSensorPhase::Inactive) => output.push(("sculk_sensor_phase".to_string(), "inactive".to_string())),
			Property::SculkSensorSculkSensorPhase(SculkSensorSculkSensorPhase::Active) => output.push(("sculk_sensor_phase".to_string(), "active".to_string())),
			Property::SculkSensorSculkSensorPhase(SculkSensorSculkSensorPhase::Cooldown) => output.push(("sculk_sensor_phase".to_string(), "cooldown".to_string())),
			Property::SnifferEggHatch(SnifferEggHatch::Num0) => output.push(("hatch".to_string(), "0".to_string())),
			Property::SnifferEggHatch(SnifferEggHatch::Num1) => output.push(("hatch".to_string(), "1".to_string())),
			Property::SnifferEggHatch(SnifferEggHatch::Num2) => output.push(("hatch".to_string(), "2".to_string())),
			Property::WeatheringCopperBarWest(WeatheringCopperBarWest::True) => output.push(("west".to_string(), "true".to_string())),
			Property::WeatheringCopperBarWest(WeatheringCopperBarWest::False) => output.push(("west".to_string(), "false".to_string())),
			Property::PistonHeadFacing(PistonHeadFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::PistonHeadFacing(PistonHeadFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::PistonHeadFacing(PistonHeadFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::PistonHeadFacing(PistonHeadFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::PistonHeadFacing(PistonHeadFacing::Up) => output.push(("facing".to_string(), "up".to_string())),
			Property::PistonHeadFacing(PistonHeadFacing::Down) => output.push(("facing".to_string(), "down".to_string())),
			Property::VineUp(VineUp::True) => output.push(("up".to_string(), "true".to_string())),
			Property::VineUp(VineUp::False) => output.push(("up".to_string(), "false".to_string())),
			Property::TallSeagrassHalf(TallSeagrassHalf::Upper) => output.push(("half".to_string(), "upper".to_string())),
			Property::TallSeagrassHalf(TallSeagrassHalf::Lower) => output.push(("half".to_string(), "lower".to_string())),
			Property::TripwireDisarmed(TripwireDisarmed::True) => output.push(("disarmed".to_string(), "true".to_string())),
			Property::TripwireDisarmed(TripwireDisarmed::False) => output.push(("disarmed".to_string(), "false".to_string())),
			Property::CopperGolemStatueFacing(CopperGolemStatueFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::CopperGolemStatueFacing(CopperGolemStatueFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::CopperGolemStatueFacing(CopperGolemStatueFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::CopperGolemStatueFacing(CopperGolemStatueFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::IronBarsWaterlogged(IronBarsWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::IronBarsWaterlogged(IronBarsWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::TrappedChestWaterlogged(TrappedChestWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::TrappedChestWaterlogged(TrappedChestWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::BaseCoralWallFanFacing(BaseCoralWallFanFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::BaseCoralWallFanFacing(BaseCoralWallFanFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::BaseCoralWallFanFacing(BaseCoralWallFanFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::BaseCoralWallFanFacing(BaseCoralWallFanFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::RedstoneWireNorth(RedstoneWireNorth::Up) => output.push(("north".to_string(), "up".to_string())),
			Property::RedstoneWireNorth(RedstoneWireNorth::Side) => output.push(("north".to_string(), "side".to_string())),
			Property::RedstoneWireNorth(RedstoneWireNorth::None) => output.push(("north".to_string(), "none".to_string())),
			Property::VaultOminous(VaultOminous::True) => output.push(("ominous".to_string(), "true".to_string())),
			Property::VaultOminous(VaultOminous::False) => output.push(("ominous".to_string(), "false".to_string())),
			Property::LightningRodWaterlogged(LightningRodWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::LightningRodWaterlogged(LightningRodWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::ButtonFace(ButtonFace::Floor) => output.push(("face".to_string(), "floor".to_string())),
			Property::ButtonFace(ButtonFace::Wall) => output.push(("face".to_string(), "wall".to_string())),
			Property::ButtonFace(ButtonFace::Ceiling) => output.push(("face".to_string(), "ceiling".to_string())),
			Property::WitherWallSkullPowered(WitherWallSkullPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::WitherWallSkullPowered(WitherWallSkullPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::BedOccupied(BedOccupied::True) => output.push(("occupied".to_string(), "true".to_string())),
			Property::BedOccupied(BedOccupied::False) => output.push(("occupied".to_string(), "false".to_string())),
			Property::RedstoneWireEast(RedstoneWireEast::Up) => output.push(("east".to_string(), "up".to_string())),
			Property::RedstoneWireEast(RedstoneWireEast::Side) => output.push(("east".to_string(), "side".to_string())),
			Property::RedstoneWireEast(RedstoneWireEast::None) => output.push(("east".to_string(), "none".to_string())),
			Property::StemAge(StemAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::StemAge(StemAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::StemAge(StemAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::StemAge(StemAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::StemAge(StemAge::Num4) => output.push(("age".to_string(), "4".to_string())),
			Property::StemAge(StemAge::Num5) => output.push(("age".to_string(), "5".to_string())),
			Property::StemAge(StemAge::Num6) => output.push(("age".to_string(), "6".to_string())),
			Property::StemAge(StemAge::Num7) => output.push(("age".to_string(), "7".to_string())),
			Property::BambooStalkStage(BambooStalkStage::Num0) => output.push(("stage".to_string(), "0".to_string())),
			Property::BambooStalkStage(BambooStalkStage::Num1) => output.push(("stage".to_string(), "1".to_string())),
			Property::FireNorth(FireNorth::True) => output.push(("north".to_string(), "true".to_string())),
			Property::FireNorth(FireNorth::False) => output.push(("north".to_string(), "false".to_string())),
			Property::BarrelOpen(BarrelOpen::True) => output.push(("open".to_string(), "true".to_string())),
			Property::BarrelOpen(BarrelOpen::False) => output.push(("open".to_string(), "false".to_string())),
			Property::GlowLichenEast(GlowLichenEast::True) => output.push(("east".to_string(), "true".to_string())),
			Property::GlowLichenEast(GlowLichenEast::False) => output.push(("east".to_string(), "false".to_string())),
			Property::LecternPowered(LecternPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::LecternPowered(LecternPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::PointedDripstoneVerticalDirection(PointedDripstoneVerticalDirection::Up) => output.push(("vertical_direction".to_string(), "up".to_string())),
			Property::PointedDripstoneVerticalDirection(PointedDripstoneVerticalDirection::Down) => output.push(("vertical_direction".to_string(), "down".to_string())),
			Property::GlowLichenWaterlogged(GlowLichenWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::GlowLichenWaterlogged(GlowLichenWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::VineEast(VineEast::True) => output.push(("east".to_string(), "true".to_string())),
			Property::VineEast(VineEast::False) => output.push(("east".to_string(), "false".to_string())),
			Property::StainedGlassPaneEast(StainedGlassPaneEast::True) => output.push(("east".to_string(), "true".to_string())),
			Property::StainedGlassPaneEast(StainedGlassPaneEast::False) => output.push(("east".to_string(), "false".to_string())),
			Property::ComparatorPowered(ComparatorPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::ComparatorPowered(ComparatorPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::PlayerWallHeadPowered(PlayerWallHeadPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::PlayerWallHeadPowered(PlayerWallHeadPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::FenceGateFacing(FenceGateFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::FenceGateFacing(FenceGateFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::FenceGateFacing(FenceGateFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::FenceGateFacing(FenceGateFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::BannerRotation(BannerRotation::Num0) => output.push(("rotation".to_string(), "0".to_string())),
			Property::BannerRotation(BannerRotation::Num1) => output.push(("rotation".to_string(), "1".to_string())),
			Property::BannerRotation(BannerRotation::Num2) => output.push(("rotation".to_string(), "2".to_string())),
			Property::BannerRotation(BannerRotation::Num3) => output.push(("rotation".to_string(), "3".to_string())),
			Property::BannerRotation(BannerRotation::Num4) => output.push(("rotation".to_string(), "4".to_string())),
			Property::BannerRotation(BannerRotation::Num5) => output.push(("rotation".to_string(), "5".to_string())),
			Property::BannerRotation(BannerRotation::Num6) => output.push(("rotation".to_string(), "6".to_string())),
			Property::BannerRotation(BannerRotation::Num7) => output.push(("rotation".to_string(), "7".to_string())),
			Property::BannerRotation(BannerRotation::Num8) => output.push(("rotation".to_string(), "8".to_string())),
			Property::BannerRotation(BannerRotation::Num9) => output.push(("rotation".to_string(), "9".to_string())),
			Property::BannerRotation(BannerRotation::Num10) => output.push(("rotation".to_string(), "10".to_string())),
			Property::BannerRotation(BannerRotation::Num11) => output.push(("rotation".to_string(), "11".to_string())),
			Property::BannerRotation(BannerRotation::Num12) => output.push(("rotation".to_string(), "12".to_string())),
			Property::BannerRotation(BannerRotation::Num13) => output.push(("rotation".to_string(), "13".to_string())),
			Property::BannerRotation(BannerRotation::Num14) => output.push(("rotation".to_string(), "14".to_string())),
			Property::BannerRotation(BannerRotation::Num15) => output.push(("rotation".to_string(), "15".to_string())),
			Property::HugeMushroomEast(HugeMushroomEast::True) => output.push(("east".to_string(), "true".to_string())),
			Property::HugeMushroomEast(HugeMushroomEast::False) => output.push(("east".to_string(), "false".to_string())),
			Property::AnvilFacing(AnvilFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::AnvilFacing(AnvilFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::AnvilFacing(AnvilFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::AnvilFacing(AnvilFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::BaseCoralWallFanWaterlogged(BaseCoralWallFanWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::BaseCoralWallFanWaterlogged(BaseCoralWallFanWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::FireUp(FireUp::True) => output.push(("up".to_string(), "true".to_string())),
			Property::FireUp(FireUp::False) => output.push(("up".to_string(), "false".to_string())),
			Property::PistonBaseExtended(PistonBaseExtended::True) => output.push(("extended".to_string(), "true".to_string())),
			Property::PistonBaseExtended(PistonBaseExtended::False) => output.push(("extended".to_string(), "false".to_string())),
			Property::FenceNorth(FenceNorth::True) => output.push(("north".to_string(), "true".to_string())),
			Property::FenceNorth(FenceNorth::False) => output.push(("north".to_string(), "false".to_string())),
			Property::RepeaterDelay(RepeaterDelay::Num1) => output.push(("delay".to_string(), "1".to_string())),
			Property::RepeaterDelay(RepeaterDelay::Num2) => output.push(("delay".to_string(), "2".to_string())),
			Property::RepeaterDelay(RepeaterDelay::Num3) => output.push(("delay".to_string(), "3".to_string())),
			Property::RepeaterDelay(RepeaterDelay::Num4) => output.push(("delay".to_string(), "4".to_string())),
			Property::VineWest(VineWest::True) => output.push(("west".to_string(), "true".to_string())),
			Property::VineWest(VineWest::False) => output.push(("west".to_string(), "false".to_string())),
			Property::DoublePlantHalf(DoublePlantHalf::Upper) => output.push(("half".to_string(), "upper".to_string())),
			Property::DoublePlantHalf(DoublePlantHalf::Lower) => output.push(("half".to_string(), "lower".to_string())),
			Property::CandleCakeLit(CandleCakeLit::True) => output.push(("lit".to_string(), "true".to_string())),
			Property::CandleCakeLit(CandleCakeLit::False) => output.push(("lit".to_string(), "false".to_string())),
			Property::BigDripleafStemWaterlogged(BigDripleafStemWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::BigDripleafStemWaterlogged(BigDripleafStemWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::RespawnAnchorCharges(RespawnAnchorCharges::Num0) => output.push(("charges".to_string(), "0".to_string())),
			Property::RespawnAnchorCharges(RespawnAnchorCharges::Num1) => output.push(("charges".to_string(), "1".to_string())),
			Property::RespawnAnchorCharges(RespawnAnchorCharges::Num2) => output.push(("charges".to_string(), "2".to_string())),
			Property::RespawnAnchorCharges(RespawnAnchorCharges::Num3) => output.push(("charges".to_string(), "3".to_string())),
			Property::RespawnAnchorCharges(RespawnAnchorCharges::Num4) => output.push(("charges".to_string(), "4".to_string())),
			Property::RedstoneOreLit(RedstoneOreLit::True) => output.push(("lit".to_string(), "true".to_string())),
			Property::RedstoneOreLit(RedstoneOreLit::False) => output.push(("lit".to_string(), "false".to_string())),
			Property::MangrovePropaguleStage(MangrovePropaguleStage::Num0) => output.push(("stage".to_string(), "0".to_string())),
			Property::MangrovePropaguleStage(MangrovePropaguleStage::Num1) => output.push(("stage".to_string(), "1".to_string())),
			Property::IronBarsSouth(IronBarsSouth::True) => output.push(("south".to_string(), "true".to_string())),
			Property::IronBarsSouth(IronBarsSouth::False) => output.push(("south".to_string(), "false".to_string())),
			Property::WallHangingSignWaterlogged(WallHangingSignWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::WallHangingSignWaterlogged(WallHangingSignWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::SculkShriekerCanSummon(SculkShriekerCanSummon::True) => output.push(("can_summon".to_string(), "true".to_string())),
			Property::SculkShriekerCanSummon(SculkShriekerCanSummon::False) => output.push(("can_summon".to_string(), "false".to_string())),
			Property::StainedGlassPaneWest(StainedGlassPaneWest::True) => output.push(("west".to_string(), "true".to_string())),
			Property::StainedGlassPaneWest(StainedGlassPaneWest::False) => output.push(("west".to_string(), "false".to_string())),
			Property::CampfireWaterlogged(CampfireWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::CampfireWaterlogged(CampfireWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::CandleCandles(CandleCandles::Num1) => output.push(("candles".to_string(), "1".to_string())),
			Property::CandleCandles(CandleCandles::Num2) => output.push(("candles".to_string(), "2".to_string())),
			Property::CandleCandles(CandleCandles::Num3) => output.push(("candles".to_string(), "3".to_string())),
			Property::CandleCandles(CandleCandles::Num4) => output.push(("candles".to_string(), "4".to_string())),
			Property::BedPart(BedPart::Head) => output.push(("part".to_string(), "head".to_string())),
			Property::BedPart(BedPart::Foot) => output.push(("part".to_string(), "foot".to_string())),
			Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::MangroveLeavesWaterlogged(MangroveLeavesWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::MangroveLeavesWaterlogged(MangroveLeavesWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::RedstoneTorchLit(RedstoneTorchLit::True) => output.push(("lit".to_string(), "true".to_string())),
			Property::RedstoneTorchLit(RedstoneTorchLit::False) => output.push(("lit".to_string(), "false".to_string())),
			Property::BigDripleafTilt(BigDripleafTilt::None) => output.push(("tilt".to_string(), "none".to_string())),
			Property::BigDripleafTilt(BigDripleafTilt::Unstable) => output.push(("tilt".to_string(), "unstable".to_string())),
			Property::BigDripleafTilt(BigDripleafTilt::Partial) => output.push(("tilt".to_string(), "partial".to_string())),
			Property::BigDripleafTilt(BigDripleafTilt::Full) => output.push(("tilt".to_string(), "full".to_string())),
			Property::BrewingStandHasBottle2(BrewingStandHasBottle2::True) => output.push(("has_bottle_2".to_string(), "true".to_string())),
			Property::BrewingStandHasBottle2(BrewingStandHasBottle2::False) => output.push(("has_bottle_2".to_string(), "false".to_string())),
			Property::ChestWaterlogged(ChestWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::ChestWaterlogged(ChestWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::SculkShriekerShrieking(SculkShriekerShrieking::True) => output.push(("shrieking".to_string(), "true".to_string())),
			Property::SculkShriekerShrieking(SculkShriekerShrieking::False) => output.push(("shrieking".to_string(), "false".to_string())),
			Property::RotatedPillarAxis(RotatedPillarAxis::X) => output.push(("axis".to_string(), "x".to_string())),
			Property::RotatedPillarAxis(RotatedPillarAxis::Y) => output.push(("axis".to_string(), "y".to_string())),
			Property::RotatedPillarAxis(RotatedPillarAxis::Z) => output.push(("axis".to_string(), "z".to_string())),
			Property::BarrierWaterlogged(BarrierWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::BarrierWaterlogged(BarrierWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::CaveVinesBerries(CaveVinesBerries::True) => output.push(("berries".to_string(), "true".to_string())),
			Property::CaveVinesBerries(CaveVinesBerries::False) => output.push(("berries".to_string(), "false".to_string())),
			Property::DecoratedPotFacing(DecoratedPotFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::DecoratedPotFacing(DecoratedPotFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::DecoratedPotFacing(DecoratedPotFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::DecoratedPotFacing(DecoratedPotFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::LiquidLevel(LiquidLevel::Num0) => output.push(("level".to_string(), "0".to_string())),
			Property::LiquidLevel(LiquidLevel::Num1) => output.push(("level".to_string(), "1".to_string())),
			Property::LiquidLevel(LiquidLevel::Num2) => output.push(("level".to_string(), "2".to_string())),
			Property::LiquidLevel(LiquidLevel::Num3) => output.push(("level".to_string(), "3".to_string())),
			Property::LiquidLevel(LiquidLevel::Num4) => output.push(("level".to_string(), "4".to_string())),
			Property::LiquidLevel(LiquidLevel::Num5) => output.push(("level".to_string(), "5".to_string())),
			Property::LiquidLevel(LiquidLevel::Num6) => output.push(("level".to_string(), "6".to_string())),
			Property::LiquidLevel(LiquidLevel::Num7) => output.push(("level".to_string(), "7".to_string())),
			Property::LiquidLevel(LiquidLevel::Num8) => output.push(("level".to_string(), "8".to_string())),
			Property::LiquidLevel(LiquidLevel::Num9) => output.push(("level".to_string(), "9".to_string())),
			Property::LiquidLevel(LiquidLevel::Num10) => output.push(("level".to_string(), "10".to_string())),
			Property::LiquidLevel(LiquidLevel::Num11) => output.push(("level".to_string(), "11".to_string())),
			Property::LiquidLevel(LiquidLevel::Num12) => output.push(("level".to_string(), "12".to_string())),
			Property::LiquidLevel(LiquidLevel::Num13) => output.push(("level".to_string(), "13".to_string())),
			Property::LiquidLevel(LiquidLevel::Num14) => output.push(("level".to_string(), "14".to_string())),
			Property::LiquidLevel(LiquidLevel::Num15) => output.push(("level".to_string(), "15".to_string())),
			Property::CoralWallFanFacing(CoralWallFanFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::CoralWallFanFacing(CoralWallFanFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::CoralWallFanFacing(CoralWallFanFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::CoralWallFanFacing(CoralWallFanFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::IronBarsEast(IronBarsEast::True) => output.push(("east".to_string(), "true".to_string())),
			Property::IronBarsEast(IronBarsEast::False) => output.push(("east".to_string(), "false".to_string())),
			Property::UntintedParticleLeavesWaterlogged(UntintedParticleLeavesWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::UntintedParticleLeavesWaterlogged(UntintedParticleLeavesWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::WeatheringCopperGrateWaterlogged(WeatheringCopperGrateWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::WeatheringCopperGrateWaterlogged(WeatheringCopperGrateWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::NoteInstrument(NoteInstrument::Harp) => output.push(("instrument".to_string(), "harp".to_string())),
			Property::NoteInstrument(NoteInstrument::Basedrum) => output.push(("instrument".to_string(), "basedrum".to_string())),
			Property::NoteInstrument(NoteInstrument::Snare) => output.push(("instrument".to_string(), "snare".to_string())),
			Property::NoteInstrument(NoteInstrument::Hat) => output.push(("instrument".to_string(), "hat".to_string())),
			Property::NoteInstrument(NoteInstrument::Bass) => output.push(("instrument".to_string(), "bass".to_string())),
			Property::NoteInstrument(NoteInstrument::Flute) => output.push(("instrument".to_string(), "flute".to_string())),
			Property::NoteInstrument(NoteInstrument::Bell) => output.push(("instrument".to_string(), "bell".to_string())),
			Property::NoteInstrument(NoteInstrument::Guitar) => output.push(("instrument".to_string(), "guitar".to_string())),
			Property::NoteInstrument(NoteInstrument::Chime) => output.push(("instrument".to_string(), "chime".to_string())),
			Property::NoteInstrument(NoteInstrument::Xylophone) => output.push(("instrument".to_string(), "xylophone".to_string())),
			Property::NoteInstrument(NoteInstrument::IronXylophone) => output.push(("instrument".to_string(), "iron_xylophone".to_string())),
			Property::NoteInstrument(NoteInstrument::CowBell) => output.push(("instrument".to_string(), "cow_bell".to_string())),
			Property::NoteInstrument(NoteInstrument::Didgeridoo) => output.push(("instrument".to_string(), "didgeridoo".to_string())),
			Property::NoteInstrument(NoteInstrument::Bit) => output.push(("instrument".to_string(), "bit".to_string())),
			Property::NoteInstrument(NoteInstrument::Banjo) => output.push(("instrument".to_string(), "banjo".to_string())),
			Property::NoteInstrument(NoteInstrument::Pling) => output.push(("instrument".to_string(), "pling".to_string())),
			Property::NoteInstrument(NoteInstrument::Zombie) => output.push(("instrument".to_string(), "zombie".to_string())),
			Property::NoteInstrument(NoteInstrument::Skeleton) => output.push(("instrument".to_string(), "skeleton".to_string())),
			Property::NoteInstrument(NoteInstrument::Creeper) => output.push(("instrument".to_string(), "creeper".to_string())),
			Property::NoteInstrument(NoteInstrument::Dragon) => output.push(("instrument".to_string(), "dragon".to_string())),
			Property::NoteInstrument(NoteInstrument::WitherSkeleton) => output.push(("instrument".to_string(), "wither_skeleton".to_string())),
			Property::NoteInstrument(NoteInstrument::Piglin) => output.push(("instrument".to_string(), "piglin".to_string())),
			Property::NoteInstrument(NoteInstrument::CustomHead) => output.push(("instrument".to_string(), "custom_head".to_string())),
			Property::CrafterTriggered(CrafterTriggered::True) => output.push(("triggered".to_string(), "true".to_string())),
			Property::CrafterTriggered(CrafterTriggered::False) => output.push(("triggered".to_string(), "false".to_string())),
			Property::CocoaFacing(CocoaFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::CocoaFacing(CocoaFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::CocoaFacing(CocoaFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::CocoaFacing(CocoaFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::CaveVinesPlantBerries(CaveVinesPlantBerries::True) => output.push(("berries".to_string(), "true".to_string())),
			Property::CaveVinesPlantBerries(CaveVinesPlantBerries::False) => output.push(("berries".to_string(), "false".to_string())),
			Property::RepeaterLocked(RepeaterLocked::True) => output.push(("locked".to_string(), "true".to_string())),
			Property::RepeaterLocked(RepeaterLocked::False) => output.push(("locked".to_string(), "false".to_string())),
			Property::FenceGatePowered(FenceGatePowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::FenceGatePowered(FenceGatePowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::DetectorRailShape(DetectorRailShape::NorthSouth) => output.push(("shape".to_string(), "north_south".to_string())),
			Property::DetectorRailShape(DetectorRailShape::EastWest) => output.push(("shape".to_string(), "east_west".to_string())),
			Property::DetectorRailShape(DetectorRailShape::AscendingEast) => output.push(("shape".to_string(), "ascending_east".to_string())),
			Property::DetectorRailShape(DetectorRailShape::AscendingWest) => output.push(("shape".to_string(), "ascending_west".to_string())),
			Property::DetectorRailShape(DetectorRailShape::AscendingNorth) => output.push(("shape".to_string(), "ascending_north".to_string())),
			Property::DetectorRailShape(DetectorRailShape::AscendingSouth) => output.push(("shape".to_string(), "ascending_south".to_string())),
			Property::WeatheringCopperTrapDoorHalf(WeatheringCopperTrapDoorHalf::Top) => output.push(("half".to_string(), "top".to_string())),
			Property::WeatheringCopperTrapDoorHalf(WeatheringCopperTrapDoorHalf::Bottom) => output.push(("half".to_string(), "bottom".to_string())),
			Property::BaseCoralPlantWaterlogged(BaseCoralPlantWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::BaseCoralPlantWaterlogged(BaseCoralPlantWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::WeatheringCopperBarEast(WeatheringCopperBarEast::True) => output.push(("east".to_string(), "true".to_string())),
			Property::WeatheringCopperBarEast(WeatheringCopperBarEast::False) => output.push(("east".to_string(), "false".to_string())),
			Property::WeatheringCopperTrapDoorOpen(WeatheringCopperTrapDoorOpen::True) => output.push(("open".to_string(), "true".to_string())),
			Property::WeatheringCopperTrapDoorOpen(WeatheringCopperTrapDoorOpen::False) => output.push(("open".to_string(), "false".to_string())),
			Property::WeatheringCopperStairWaterlogged(WeatheringCopperStairWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::WeatheringCopperStairWaterlogged(WeatheringCopperStairWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::MangrovePropaguleAge(MangrovePropaguleAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::MangrovePropaguleAge(MangrovePropaguleAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::MangrovePropaguleAge(MangrovePropaguleAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::MangrovePropaguleAge(MangrovePropaguleAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::MangrovePropaguleAge(MangrovePropaguleAge::Num4) => output.push(("age".to_string(), "4".to_string())),
			Property::FlowerBedFacing(FlowerBedFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::FlowerBedFacing(FlowerBedFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::FlowerBedFacing(FlowerBedFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::FlowerBedFacing(FlowerBedFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::FireAge(FireAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::FireAge(FireAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::FireAge(FireAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::FireAge(FireAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::FireAge(FireAge::Num4) => output.push(("age".to_string(), "4".to_string())),
			Property::FireAge(FireAge::Num5) => output.push(("age".to_string(), "5".to_string())),
			Property::FireAge(FireAge::Num6) => output.push(("age".to_string(), "6".to_string())),
			Property::FireAge(FireAge::Num7) => output.push(("age".to_string(), "7".to_string())),
			Property::FireAge(FireAge::Num8) => output.push(("age".to_string(), "8".to_string())),
			Property::FireAge(FireAge::Num9) => output.push(("age".to_string(), "9".to_string())),
			Property::FireAge(FireAge::Num10) => output.push(("age".to_string(), "10".to_string())),
			Property::FireAge(FireAge::Num11) => output.push(("age".to_string(), "11".to_string())),
			Property::FireAge(FireAge::Num12) => output.push(("age".to_string(), "12".to_string())),
			Property::FireAge(FireAge::Num13) => output.push(("age".to_string(), "13".to_string())),
			Property::FireAge(FireAge::Num14) => output.push(("age".to_string(), "14".to_string())),
			Property::FireAge(FireAge::Num15) => output.push(("age".to_string(), "15".to_string())),
			Property::CoralPlantWaterlogged(CoralPlantWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::CoralPlantWaterlogged(CoralPlantWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::PistonHeadShort(PistonHeadShort::True) => output.push(("short".to_string(), "true".to_string())),
			Property::PistonHeadShort(PistonHeadShort::False) => output.push(("short".to_string(), "false".to_string())),
			Property::ButtonFacing(ButtonFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::ButtonFacing(ButtonFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::ButtonFacing(ButtonFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::ButtonFacing(ButtonFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::PointedDripstoneWaterlogged(PointedDripstoneWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::PointedDripstoneWaterlogged(PointedDripstoneWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::WallWaterlogged(WallWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::WallWaterlogged(WallWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::LanternHanging(LanternHanging::True) => output.push(("hanging".to_string(), "true".to_string())),
			Property::LanternHanging(LanternHanging::False) => output.push(("hanging".to_string(), "false".to_string())),
			Property::PiglinwallskullPowered(PiglinwallskullPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::PiglinwallskullPowered(PiglinwallskullPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::DoorPowered(DoorPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::DoorPowered(DoorPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::SculkVeinEast(SculkVeinEast::True) => output.push(("east".to_string(), "true".to_string())),
			Property::SculkVeinEast(SculkVeinEast::False) => output.push(("east".to_string(), "false".to_string())),
			Property::BigDripleafWaterlogged(BigDripleafWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::BigDripleafWaterlogged(BigDripleafWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::WallSouth(WallSouth::None) => output.push(("south".to_string(), "none".to_string())),
			Property::WallSouth(WallSouth::Low) => output.push(("south".to_string(), "low".to_string())),
			Property::WallSouth(WallSouth::Tall) => output.push(("south".to_string(), "tall".to_string())),
			Property::SculkVeinSouth(SculkVeinSouth::True) => output.push(("south".to_string(), "true".to_string())),
			Property::SculkVeinSouth(SculkVeinSouth::False) => output.push(("south".to_string(), "false".to_string())),
			Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::Up) => output.push(("facing".to_string(), "up".to_string())),
			Property::WeatheringLightningRodFacing(WeatheringLightningRodFacing::Down) => output.push(("facing".to_string(), "down".to_string())),
			Property::WallSkullPowered(WallSkullPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::WallSkullPowered(WallSkullPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::StonecutterFacing(StonecutterFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::StonecutterFacing(StonecutterFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::StonecutterFacing(StonecutterFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::StonecutterFacing(StonecutterFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::CarrotAge(CarrotAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::CarrotAge(CarrotAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::CarrotAge(CarrotAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::CarrotAge(CarrotAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::CarrotAge(CarrotAge::Num4) => output.push(("age".to_string(), "4".to_string())),
			Property::CarrotAge(CarrotAge::Num5) => output.push(("age".to_string(), "5".to_string())),
			Property::CarrotAge(CarrotAge::Num6) => output.push(("age".to_string(), "6".to_string())),
			Property::CarrotAge(CarrotAge::Num7) => output.push(("age".to_string(), "7".to_string())),
			Property::WitherSkullPowered(WitherSkullPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::WitherSkullPowered(WitherSkullPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::CopperChestFacing(CopperChestFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::CopperChestFacing(CopperChestFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::CopperChestFacing(CopperChestFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::CopperChestFacing(CopperChestFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::SculkVeinDown(SculkVeinDown::True) => output.push(("down".to_string(), "true".to_string())),
			Property::SculkVeinDown(SculkVeinDown::False) => output.push(("down".to_string(), "false".to_string())),
			Property::LightningRodFacing(LightningRodFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::LightningRodFacing(LightningRodFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::LightningRodFacing(LightningRodFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::LightningRodFacing(LightningRodFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::LightningRodFacing(LightningRodFacing::Up) => output.push(("facing".to_string(), "up".to_string())),
			Property::LightningRodFacing(LightningRodFacing::Down) => output.push(("facing".to_string(), "down".to_string())),
			Property::WeatheringLightningRodPowered(WeatheringLightningRodPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::WeatheringLightningRodPowered(WeatheringLightningRodPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::IronBarsWest(IronBarsWest::True) => output.push(("west".to_string(), "true".to_string())),
			Property::IronBarsWest(IronBarsWest::False) => output.push(("west".to_string(), "false".to_string())),
			Property::ObserverPowered(ObserverPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::ObserverPowered(ObserverPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num0) => output.push(("power".to_string(), "0".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num1) => output.push(("power".to_string(), "1".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num2) => output.push(("power".to_string(), "2".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num3) => output.push(("power".to_string(), "3".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num4) => output.push(("power".to_string(), "4".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num5) => output.push(("power".to_string(), "5".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num6) => output.push(("power".to_string(), "6".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num7) => output.push(("power".to_string(), "7".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num8) => output.push(("power".to_string(), "8".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num9) => output.push(("power".to_string(), "9".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num10) => output.push(("power".to_string(), "10".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num11) => output.push(("power".to_string(), "11".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num12) => output.push(("power".to_string(), "12".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num13) => output.push(("power".to_string(), "13".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num14) => output.push(("power".to_string(), "14".to_string())),
			Property::RedstoneWirePower(RedstoneWirePower::Num15) => output.push(("power".to_string(), "15".to_string())),
			Property::WallEast(WallEast::None) => output.push(("east".to_string(), "none".to_string())),
			Property::WallEast(WallEast::Low) => output.push(("east".to_string(), "low".to_string())),
			Property::WallEast(WallEast::Tall) => output.push(("east".to_string(), "tall".to_string())),
			Property::CopperGolemStatueCopperGolemPose(CopperGolemStatueCopperGolemPose::Standing) => output.push(("copper_golem_pose".to_string(), "standing".to_string())),
			Property::CopperGolemStatueCopperGolemPose(CopperGolemStatueCopperGolemPose::Sitting) => output.push(("copper_golem_pose".to_string(), "sitting".to_string())),
			Property::CopperGolemStatueCopperGolemPose(CopperGolemStatueCopperGolemPose::Running) => output.push(("copper_golem_pose".to_string(), "running".to_string())),
			Property::CopperGolemStatueCopperGolemPose(CopperGolemStatueCopperGolemPose::Star) => output.push(("copper_golem_pose".to_string(), "star".to_string())),
			Property::PiglinwallskullFacing(PiglinwallskullFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::PiglinwallskullFacing(PiglinwallskullFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::PiglinwallskullFacing(PiglinwallskullFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::PiglinwallskullFacing(PiglinwallskullFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::FenceWest(FenceWest::True) => output.push(("west".to_string(), "true".to_string())),
			Property::FenceWest(FenceWest::False) => output.push(("west".to_string(), "false".to_string())),
			Property::MultifaceSouth(MultifaceSouth::True) => output.push(("south".to_string(), "true".to_string())),
			Property::MultifaceSouth(MultifaceSouth::False) => output.push(("south".to_string(), "false".to_string())),
			Property::TrappedChestFacing(TrappedChestFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::TrappedChestFacing(TrappedChestFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::TrappedChestFacing(TrappedChestFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::TrappedChestFacing(TrappedChestFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::DoorHalf(DoorHalf::Upper) => output.push(("half".to_string(), "upper".to_string())),
			Property::DoorHalf(DoorHalf::Lower) => output.push(("half".to_string(), "lower".to_string())),
			Property::BeetrootAge(BeetrootAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::BeetrootAge(BeetrootAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::BeetrootAge(BeetrootAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::BeetrootAge(BeetrootAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::ChorusPlantUp(ChorusPlantUp::True) => output.push(("up".to_string(), "true".to_string())),
			Property::ChorusPlantUp(ChorusPlantUp::False) => output.push(("up".to_string(), "false".to_string())),
			Property::CocoaAge(CocoaAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::CocoaAge(CocoaAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::CocoaAge(CocoaAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::EndRodFacing(EndRodFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::EndRodFacing(EndRodFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::EndRodFacing(EndRodFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::EndRodFacing(EndRodFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::EndRodFacing(EndRodFacing::Up) => output.push(("facing".to_string(), "up".to_string())),
			Property::EndRodFacing(EndRodFacing::Down) => output.push(("facing".to_string(), "down".to_string())),
			Property::EnderChestWaterlogged(EnderChestWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::EnderChestWaterlogged(EnderChestWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num1) => output.push(("distance".to_string(), "1".to_string())),
			Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num2) => output.push(("distance".to_string(), "2".to_string())),
			Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num3) => output.push(("distance".to_string(), "3".to_string())),
			Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num4) => output.push(("distance".to_string(), "4".to_string())),
			Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num5) => output.push(("distance".to_string(), "5".to_string())),
			Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num6) => output.push(("distance".to_string(), "6".to_string())),
			Property::TintedParticleLeavesDistance(TintedParticleLeavesDistance::Num7) => output.push(("distance".to_string(), "7".to_string())),
			Property::BambooStalkLeaves(BambooStalkLeaves::None) => output.push(("leaves".to_string(), "none".to_string())),
			Property::BambooStalkLeaves(BambooStalkLeaves::Small) => output.push(("leaves".to_string(), "small".to_string())),
			Property::BambooStalkLeaves(BambooStalkLeaves::Large) => output.push(("leaves".to_string(), "large".to_string())),
			Property::LayeredCauldronLevel(LayeredCauldronLevel::Num1) => output.push(("level".to_string(), "1".to_string())),
			Property::LayeredCauldronLevel(LayeredCauldronLevel::Num2) => output.push(("level".to_string(), "2".to_string())),
			Property::LayeredCauldronLevel(LayeredCauldronLevel::Num3) => output.push(("level".to_string(), "3".to_string())),
			Property::CeilingHangingSignAttached(CeilingHangingSignAttached::True) => output.push(("attached".to_string(), "true".to_string())),
			Property::CeilingHangingSignAttached(CeilingHangingSignAttached::False) => output.push(("attached".to_string(), "false".to_string())),
			Property::GlowLichenNorth(GlowLichenNorth::True) => output.push(("north".to_string(), "true".to_string())),
			Property::GlowLichenNorth(GlowLichenNorth::False) => output.push(("north".to_string(), "false".to_string())),
			Property::LecternHasBook(LecternHasBook::True) => output.push(("has_book".to_string(), "true".to_string())),
			Property::LecternHasBook(LecternHasBook::False) => output.push(("has_book".to_string(), "false".to_string())),
			Property::LeverFace(LeverFace::Floor) => output.push(("face".to_string(), "floor".to_string())),
			Property::LeverFace(LeverFace::Wall) => output.push(("face".to_string(), "wall".to_string())),
			Property::LeverFace(LeverFace::Ceiling) => output.push(("face".to_string(), "ceiling".to_string())),
			Property::FenceWaterlogged(FenceWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::FenceWaterlogged(FenceWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::WallSignFacing(WallSignFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::WallSignFacing(WallSignFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::WallSignFacing(WallSignFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::WallSignFacing(WallSignFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::MovingPistonType(MovingPistonType::Normal) => output.push(("type".to_string(), "normal".to_string())),
			Property::MovingPistonType(MovingPistonType::Sticky) => output.push(("type".to_string(), "sticky".to_string())),
			Property::BigDripleafStemFacing(BigDripleafStemFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::BigDripleafStemFacing(BigDripleafStemFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::BigDripleafStemFacing(BigDripleafStemFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::BigDripleafStemFacing(BigDripleafStemFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::TripWireHookFacing(TripWireHookFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::TripWireHookFacing(TripWireHookFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::TripWireHookFacing(TripWireHookFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::TripWireHookFacing(TripWireHookFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::TrapdoorWaterlogged(TrapdoorWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::TrapdoorWaterlogged(TrapdoorWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num0) => output.push(("power".to_string(), "0".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num1) => output.push(("power".to_string(), "1".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num2) => output.push(("power".to_string(), "2".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num3) => output.push(("power".to_string(), "3".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num4) => output.push(("power".to_string(), "4".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num5) => output.push(("power".to_string(), "5".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num6) => output.push(("power".to_string(), "6".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num7) => output.push(("power".to_string(), "7".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num8) => output.push(("power".to_string(), "8".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num9) => output.push(("power".to_string(), "9".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num10) => output.push(("power".to_string(), "10".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num11) => output.push(("power".to_string(), "11".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num12) => output.push(("power".to_string(), "12".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num13) => output.push(("power".to_string(), "13".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num14) => output.push(("power".to_string(), "14".to_string())),
			Property::WeightedPressurePlatePower(WeightedPressurePlatePower::Num15) => output.push(("power".to_string(), "15".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num4) => output.push(("age".to_string(), "4".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num5) => output.push(("age".to_string(), "5".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num6) => output.push(("age".to_string(), "6".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num7) => output.push(("age".to_string(), "7".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num8) => output.push(("age".to_string(), "8".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num9) => output.push(("age".to_string(), "9".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num10) => output.push(("age".to_string(), "10".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num11) => output.push(("age".to_string(), "11".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num12) => output.push(("age".to_string(), "12".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num13) => output.push(("age".to_string(), "13".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num14) => output.push(("age".to_string(), "14".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num15) => output.push(("age".to_string(), "15".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num16) => output.push(("age".to_string(), "16".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num17) => output.push(("age".to_string(), "17".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num18) => output.push(("age".to_string(), "18".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num19) => output.push(("age".to_string(), "19".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num20) => output.push(("age".to_string(), "20".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num21) => output.push(("age".to_string(), "21".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num22) => output.push(("age".to_string(), "22".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num23) => output.push(("age".to_string(), "23".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num24) => output.push(("age".to_string(), "24".to_string())),
			Property::TwistingVinesAge(TwistingVinesAge::Num25) => output.push(("age".to_string(), "25".to_string())),
			Property::VineNorth(VineNorth::True) => output.push(("north".to_string(), "true".to_string())),
			Property::VineNorth(VineNorth::False) => output.push(("north".to_string(), "false".to_string())),
			Property::SaplingStage(SaplingStage::Num0) => output.push(("stage".to_string(), "0".to_string())),
			Property::SaplingStage(SaplingStage::Num1) => output.push(("stage".to_string(), "1".to_string())),
			Property::DispenserTriggered(DispenserTriggered::True) => output.push(("triggered".to_string(), "true".to_string())),
			Property::DispenserTriggered(DispenserTriggered::False) => output.push(("triggered".to_string(), "false".to_string())),
			Property::DetectorRailPowered(DetectorRailPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::DetectorRailPowered(DetectorRailPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::ChorusPlantDown(ChorusPlantDown::True) => output.push(("down".to_string(), "true".to_string())),
			Property::ChorusPlantDown(ChorusPlantDown::False) => output.push(("down".to_string(), "false".to_string())),
			Property::ObserverFacing(ObserverFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::ObserverFacing(ObserverFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::ObserverFacing(ObserverFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::ObserverFacing(ObserverFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::ObserverFacing(ObserverFacing::Up) => output.push(("facing".to_string(), "up".to_string())),
			Property::ObserverFacing(ObserverFacing::Down) => output.push(("facing".to_string(), "down".to_string())),
			Property::FlowerBedFlowerAmount(FlowerBedFlowerAmount::Num1) => output.push(("flower_amount".to_string(), "1".to_string())),
			Property::FlowerBedFlowerAmount(FlowerBedFlowerAmount::Num2) => output.push(("flower_amount".to_string(), "2".to_string())),
			Property::FlowerBedFlowerAmount(FlowerBedFlowerAmount::Num3) => output.push(("flower_amount".to_string(), "3".to_string())),
			Property::FlowerBedFlowerAmount(FlowerBedFlowerAmount::Num4) => output.push(("flower_amount".to_string(), "4".to_string())),
			Property::MultifaceUp(MultifaceUp::True) => output.push(("up".to_string(), "true".to_string())),
			Property::MultifaceUp(MultifaceUp::False) => output.push(("up".to_string(), "false".to_string())),
			Property::BellFacing(BellFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::BellFacing(BellFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::BellFacing(BellFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::BellFacing(BellFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::MultifaceWaterlogged(MultifaceWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::MultifaceWaterlogged(MultifaceWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::HugeMushroomSouth(HugeMushroomSouth::True) => output.push(("south".to_string(), "true".to_string())),
			Property::HugeMushroomSouth(HugeMushroomSouth::False) => output.push(("south".to_string(), "false".to_string())),
			Property::WeatheringCopperStairHalf(WeatheringCopperStairHalf::Top) => output.push(("half".to_string(), "top".to_string())),
			Property::WeatheringCopperStairHalf(WeatheringCopperStairHalf::Bottom) => output.push(("half".to_string(), "bottom".to_string())),
			Property::ChorusPlantWest(ChorusPlantWest::True) => output.push(("west".to_string(), "true".to_string())),
			Property::ChorusPlantWest(ChorusPlantWest::False) => output.push(("west".to_string(), "false".to_string())),
			Property::WeatheringLanternHanging(WeatheringLanternHanging::True) => output.push(("hanging".to_string(), "true".to_string())),
			Property::WeatheringLanternHanging(WeatheringLanternHanging::False) => output.push(("hanging".to_string(), "false".to_string())),
			Property::TrapdoorFacing(TrapdoorFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::TrapdoorFacing(TrapdoorFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::TrapdoorFacing(TrapdoorFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::TrapdoorFacing(TrapdoorFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::SmokerFacing(SmokerFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::SmokerFacing(SmokerFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::SmokerFacing(SmokerFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::SmokerFacing(SmokerFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::TripwireEast(TripwireEast::True) => output.push(("east".to_string(), "true".to_string())),
			Property::TripwireEast(TripwireEast::False) => output.push(("east".to_string(), "false".to_string())),
			Property::GrindstoneFacing(GrindstoneFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::GrindstoneFacing(GrindstoneFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::GrindstoneFacing(GrindstoneFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::GrindstoneFacing(GrindstoneFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::CopperChestWaterlogged(CopperChestWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::CopperChestWaterlogged(CopperChestWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::StainedGlassPaneWaterlogged(StainedGlassPaneWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::StainedGlassPaneWaterlogged(StainedGlassPaneWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::LightningRodPowered(LightningRodPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::LightningRodPowered(LightningRodPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::TrapdoorOpen(TrapdoorOpen::True) => output.push(("open".to_string(), "true".to_string())),
			Property::TrapdoorOpen(TrapdoorOpen::False) => output.push(("open".to_string(), "false".to_string())),
			Property::BellAttachment(BellAttachment::Floor) => output.push(("attachment".to_string(), "floor".to_string())),
			Property::BellAttachment(BellAttachment::Ceiling) => output.push(("attachment".to_string(), "ceiling".to_string())),
			Property::BellAttachment(BellAttachment::SingleWall) => output.push(("attachment".to_string(), "single_wall".to_string())),
			Property::BellAttachment(BellAttachment::DoubleWall) => output.push(("attachment".to_string(), "double_wall".to_string())),
			Property::WeatheringCopperDoorPowered(WeatheringCopperDoorPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::WeatheringCopperDoorPowered(WeatheringCopperDoorPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::WallSkullFacing(WallSkullFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::WallSkullFacing(WallSkullFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::WallSkullFacing(WallSkullFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::WallSkullFacing(WallSkullFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::AmethystClusterWaterlogged(AmethystClusterWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::AmethystClusterWaterlogged(AmethystClusterWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::ShulkerBoxFacing(ShulkerBoxFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::ShulkerBoxFacing(ShulkerBoxFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::ShulkerBoxFacing(ShulkerBoxFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::ShulkerBoxFacing(ShulkerBoxFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::ShulkerBoxFacing(ShulkerBoxFacing::Up) => output.push(("facing".to_string(), "up".to_string())),
			Property::ShulkerBoxFacing(ShulkerBoxFacing::Down) => output.push(("facing".to_string(), "down".to_string())),
			Property::HugeMushroomDown(HugeMushroomDown::True) => output.push(("down".to_string(), "true".to_string())),
			Property::HugeMushroomDown(HugeMushroomDown::False) => output.push(("down".to_string(), "false".to_string())),
			Property::MangroveLeavesDistance(MangroveLeavesDistance::Num1) => output.push(("distance".to_string(), "1".to_string())),
			Property::MangroveLeavesDistance(MangroveLeavesDistance::Num2) => output.push(("distance".to_string(), "2".to_string())),
			Property::MangroveLeavesDistance(MangroveLeavesDistance::Num3) => output.push(("distance".to_string(), "3".to_string())),
			Property::MangroveLeavesDistance(MangroveLeavesDistance::Num4) => output.push(("distance".to_string(), "4".to_string())),
			Property::MangroveLeavesDistance(MangroveLeavesDistance::Num5) => output.push(("distance".to_string(), "5".to_string())),
			Property::MangroveLeavesDistance(MangroveLeavesDistance::Num6) => output.push(("distance".to_string(), "6".to_string())),
			Property::MangroveLeavesDistance(MangroveLeavesDistance::Num7) => output.push(("distance".to_string(), "7".to_string())),
			Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::SmallDripleafHalf(SmallDripleafHalf::Upper) => output.push(("half".to_string(), "upper".to_string())),
			Property::SmallDripleafHalf(SmallDripleafHalf::Lower) => output.push(("half".to_string(), "lower".to_string())),
			Property::SnowLayerLayers(SnowLayerLayers::Num1) => output.push(("layers".to_string(), "1".to_string())),
			Property::SnowLayerLayers(SnowLayerLayers::Num2) => output.push(("layers".to_string(), "2".to_string())),
			Property::SnowLayerLayers(SnowLayerLayers::Num3) => output.push(("layers".to_string(), "3".to_string())),
			Property::SnowLayerLayers(SnowLayerLayers::Num4) => output.push(("layers".to_string(), "4".to_string())),
			Property::SnowLayerLayers(SnowLayerLayers::Num5) => output.push(("layers".to_string(), "5".to_string())),
			Property::SnowLayerLayers(SnowLayerLayers::Num6) => output.push(("layers".to_string(), "6".to_string())),
			Property::SnowLayerLayers(SnowLayerLayers::Num7) => output.push(("layers".to_string(), "7".to_string())),
			Property::SnowLayerLayers(SnowLayerLayers::Num8) => output.push(("layers".to_string(), "8".to_string())),
			Property::TntUnstable(TntUnstable::True) => output.push(("unstable".to_string(), "true".to_string())),
			Property::TntUnstable(TntUnstable::False) => output.push(("unstable".to_string(), "false".to_string())),
			Property::RailShape(RailShape::NorthSouth) => output.push(("shape".to_string(), "north_south".to_string())),
			Property::RailShape(RailShape::EastWest) => output.push(("shape".to_string(), "east_west".to_string())),
			Property::RailShape(RailShape::AscendingEast) => output.push(("shape".to_string(), "ascending_east".to_string())),
			Property::RailShape(RailShape::AscendingWest) => output.push(("shape".to_string(), "ascending_west".to_string())),
			Property::RailShape(RailShape::AscendingNorth) => output.push(("shape".to_string(), "ascending_north".to_string())),
			Property::RailShape(RailShape::AscendingSouth) => output.push(("shape".to_string(), "ascending_south".to_string())),
			Property::RailShape(RailShape::SouthEast) => output.push(("shape".to_string(), "south_east".to_string())),
			Property::RailShape(RailShape::SouthWest) => output.push(("shape".to_string(), "south_west".to_string())),
			Property::RailShape(RailShape::NorthWest) => output.push(("shape".to_string(), "north_west".to_string())),
			Property::RailShape(RailShape::NorthEast) => output.push(("shape".to_string(), "north_east".to_string())),
			Property::FurnaceFacing(FurnaceFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::FurnaceFacing(FurnaceFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::FurnaceFacing(FurnaceFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::FurnaceFacing(FurnaceFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::ConduitWaterlogged(ConduitWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::ConduitWaterlogged(ConduitWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::ChiseledBookShelfSlot1Occupied(ChiseledBookShelfSlot1Occupied::True) => output.push(("slot_1_occupied".to_string(), "true".to_string())),
			Property::ChiseledBookShelfSlot1Occupied(ChiseledBookShelfSlot1Occupied::False) => output.push(("slot_1_occupied".to_string(), "false".to_string())),
			Property::WeatheringLanternWaterlogged(WeatheringLanternWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::WeatheringLanternWaterlogged(WeatheringLanternWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::CopperBulbBlockLit(CopperBulbBlockLit::True) => output.push(("lit".to_string(), "true".to_string())),
			Property::CopperBulbBlockLit(CopperBulbBlockLit::False) => output.push(("lit".to_string(), "false".to_string())),
			Property::ChiseledBookShelfSlot4Occupied(ChiseledBookShelfSlot4Occupied::True) => output.push(("slot_4_occupied".to_string(), "true".to_string())),
			Property::ChiseledBookShelfSlot4Occupied(ChiseledBookShelfSlot4Occupied::False) => output.push(("slot_4_occupied".to_string(), "false".to_string())),
			Property::TripWireHookAttached(TripWireHookAttached::True) => output.push(("attached".to_string(), "true".to_string())),
			Property::TripWireHookAttached(TripWireHookAttached::False) => output.push(("attached".to_string(), "false".to_string())),
			Property::WeatheringLightningRodWaterlogged(WeatheringLightningRodWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::WeatheringLightningRodWaterlogged(WeatheringLightningRodWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::FrostedIceAge(FrostedIceAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::FrostedIceAge(FrostedIceAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::FrostedIceAge(FrostedIceAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::FrostedIceAge(FrostedIceAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::RepeaterFacing(RepeaterFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::RepeaterFacing(RepeaterFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::RepeaterFacing(RepeaterFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::RepeaterFacing(RepeaterFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::WeatheringCopperChainWaterlogged(WeatheringCopperChainWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::WeatheringCopperChainWaterlogged(WeatheringCopperChainWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::ComparatorMode(ComparatorMode::Compare) => output.push(("mode".to_string(), "compare".to_string())),
			Property::ComparatorMode(ComparatorMode::Subtract) => output.push(("mode".to_string(), "subtract".to_string())),
			Property::FurnaceLit(FurnaceLit::True) => output.push(("lit".to_string(), "true".to_string())),
			Property::FurnaceLit(FurnaceLit::False) => output.push(("lit".to_string(), "false".to_string())),
			Property::GrassSnowy(GrassSnowy::True) => output.push(("snowy".to_string(), "true".to_string())),
			Property::GrassSnowy(GrassSnowy::False) => output.push(("snowy".to_string(), "false".to_string())),
			Property::WeatheringCopperStairFacing(WeatheringCopperStairFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::WeatheringCopperStairFacing(WeatheringCopperStairFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::WeatheringCopperStairFacing(WeatheringCopperStairFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::WeatheringCopperStairFacing(WeatheringCopperStairFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num0) => output.push(("rotation".to_string(), "0".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num1) => output.push(("rotation".to_string(), "1".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num2) => output.push(("rotation".to_string(), "2".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num3) => output.push(("rotation".to_string(), "3".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num4) => output.push(("rotation".to_string(), "4".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num5) => output.push(("rotation".to_string(), "5".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num6) => output.push(("rotation".to_string(), "6".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num7) => output.push(("rotation".to_string(), "7".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num8) => output.push(("rotation".to_string(), "8".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num9) => output.push(("rotation".to_string(), "9".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num10) => output.push(("rotation".to_string(), "10".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num11) => output.push(("rotation".to_string(), "11".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num12) => output.push(("rotation".to_string(), "12".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num13) => output.push(("rotation".to_string(), "13".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num14) => output.push(("rotation".to_string(), "14".to_string())),
			Property::CeilingHangingSignRotation(CeilingHangingSignRotation::Num15) => output.push(("rotation".to_string(), "15".to_string())),
			Property::BlastFurnaceFacing(BlastFurnaceFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::BlastFurnaceFacing(BlastFurnaceFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::BlastFurnaceFacing(BlastFurnaceFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::BlastFurnaceFacing(BlastFurnaceFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::DriedGhastWaterlogged(DriedGhastWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::DriedGhastWaterlogged(DriedGhastWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::StairFacing(StairFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::StairFacing(StairFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::StairFacing(StairFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::StairFacing(StairFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::CoralFanWaterlogged(CoralFanWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::CoralFanWaterlogged(CoralFanWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::ChiseledBookShelfSlot3Occupied(ChiseledBookShelfSlot3Occupied::True) => output.push(("slot_3_occupied".to_string(), "true".to_string())),
			Property::ChiseledBookShelfSlot3Occupied(ChiseledBookShelfSlot3Occupied::False) => output.push(("slot_3_occupied".to_string(), "false".to_string())),
			Property::HugeMushroomNorth(HugeMushroomNorth::True) => output.push(("north".to_string(), "true".to_string())),
			Property::HugeMushroomNorth(HugeMushroomNorth::False) => output.push(("north".to_string(), "false".to_string())),
			Property::SkullRotation(SkullRotation::Num0) => output.push(("rotation".to_string(), "0".to_string())),
			Property::SkullRotation(SkullRotation::Num1) => output.push(("rotation".to_string(), "1".to_string())),
			Property::SkullRotation(SkullRotation::Num2) => output.push(("rotation".to_string(), "2".to_string())),
			Property::SkullRotation(SkullRotation::Num3) => output.push(("rotation".to_string(), "3".to_string())),
			Property::SkullRotation(SkullRotation::Num4) => output.push(("rotation".to_string(), "4".to_string())),
			Property::SkullRotation(SkullRotation::Num5) => output.push(("rotation".to_string(), "5".to_string())),
			Property::SkullRotation(SkullRotation::Num6) => output.push(("rotation".to_string(), "6".to_string())),
			Property::SkullRotation(SkullRotation::Num7) => output.push(("rotation".to_string(), "7".to_string())),
			Property::SkullRotation(SkullRotation::Num8) => output.push(("rotation".to_string(), "8".to_string())),
			Property::SkullRotation(SkullRotation::Num9) => output.push(("rotation".to_string(), "9".to_string())),
			Property::SkullRotation(SkullRotation::Num10) => output.push(("rotation".to_string(), "10".to_string())),
			Property::SkullRotation(SkullRotation::Num11) => output.push(("rotation".to_string(), "11".to_string())),
			Property::SkullRotation(SkullRotation::Num12) => output.push(("rotation".to_string(), "12".to_string())),
			Property::SkullRotation(SkullRotation::Num13) => output.push(("rotation".to_string(), "13".to_string())),
			Property::SkullRotation(SkullRotation::Num14) => output.push(("rotation".to_string(), "14".to_string())),
			Property::SkullRotation(SkullRotation::Num15) => output.push(("rotation".to_string(), "15".to_string())),
			Property::WallWest(WallWest::None) => output.push(("west".to_string(), "none".to_string())),
			Property::WallWest(WallWest::Low) => output.push(("west".to_string(), "low".to_string())),
			Property::WallWest(WallWest::Tall) => output.push(("west".to_string(), "tall".to_string())),
			Property::PoweredRailPowered(PoweredRailPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::PoweredRailPowered(PoweredRailPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::MultifaceDown(MultifaceDown::True) => output.push(("down".to_string(), "true".to_string())),
			Property::MultifaceDown(MultifaceDown::False) => output.push(("down".to_string(), "false".to_string())),
			Property::WallHangingSignFacing(WallHangingSignFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::WallHangingSignFacing(WallHangingSignFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::WallHangingSignFacing(WallHangingSignFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::WallHangingSignFacing(WallHangingSignFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::SculkVeinNorth(SculkVeinNorth::True) => output.push(("north".to_string(), "true".to_string())),
			Property::SculkVeinNorth(SculkVeinNorth::False) => output.push(("north".to_string(), "false".to_string())),
			Property::CreakingHeartAxis(CreakingHeartAxis::X) => output.push(("axis".to_string(), "x".to_string())),
			Property::CreakingHeartAxis(CreakingHeartAxis::Y) => output.push(("axis".to_string(), "y".to_string())),
			Property::CreakingHeartAxis(CreakingHeartAxis::Z) => output.push(("axis".to_string(), "z".to_string())),
			Property::WallNorth(WallNorth::None) => output.push(("north".to_string(), "none".to_string())),
			Property::WallNorth(WallNorth::Low) => output.push(("north".to_string(), "low".to_string())),
			Property::WallNorth(WallNorth::Tall) => output.push(("north".to_string(), "tall".to_string())),
			Property::CampfireLit(CampfireLit::True) => output.push(("lit".to_string(), "true".to_string())),
			Property::CampfireLit(CampfireLit::False) => output.push(("lit".to_string(), "false".to_string())),
			Property::ChorusPlantSouth(ChorusPlantSouth::True) => output.push(("south".to_string(), "true".to_string())),
			Property::ChorusPlantSouth(ChorusPlantSouth::False) => output.push(("south".to_string(), "false".to_string())),
			Property::GlazedTerracottaFacing(GlazedTerracottaFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::GlazedTerracottaFacing(GlazedTerracottaFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::GlazedTerracottaFacing(GlazedTerracottaFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::GlazedTerracottaFacing(GlazedTerracottaFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::FenceSouth(FenceSouth::True) => output.push(("south".to_string(), "true".to_string())),
			Property::FenceSouth(FenceSouth::False) => output.push(("south".to_string(), "false".to_string())),
			Property::BlastFurnaceLit(BlastFurnaceLit::True) => output.push(("lit".to_string(), "true".to_string())),
			Property::BlastFurnaceLit(BlastFurnaceLit::False) => output.push(("lit".to_string(), "false".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num0) => output.push(("power".to_string(), "0".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num1) => output.push(("power".to_string(), "1".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num2) => output.push(("power".to_string(), "2".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num3) => output.push(("power".to_string(), "3".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num4) => output.push(("power".to_string(), "4".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num5) => output.push(("power".to_string(), "5".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num6) => output.push(("power".to_string(), "6".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num7) => output.push(("power".to_string(), "7".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num8) => output.push(("power".to_string(), "8".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num9) => output.push(("power".to_string(), "9".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num10) => output.push(("power".to_string(), "10".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num11) => output.push(("power".to_string(), "11".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num12) => output.push(("power".to_string(), "12".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num13) => output.push(("power".to_string(), "13".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num14) => output.push(("power".to_string(), "14".to_string())),
			Property::DaylightDetectorPower(DaylightDetectorPower::Num15) => output.push(("power".to_string(), "15".to_string())),
			Property::BeehiveFacing(BeehiveFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::BeehiveFacing(BeehiveFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::BeehiveFacing(BeehiveFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::BeehiveFacing(BeehiveFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::StainedGlassPaneSouth(StainedGlassPaneSouth::True) => output.push(("south".to_string(), "true".to_string())),
			Property::StainedGlassPaneSouth(StainedGlassPaneSouth::False) => output.push(("south".to_string(), "false".to_string())),
			Property::EndPortalFrameFacing(EndPortalFrameFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::EndPortalFrameFacing(EndPortalFrameFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::EndPortalFrameFacing(EndPortalFrameFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::EndPortalFrameFacing(EndPortalFrameFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::WeatheringCopperChestWaterlogged(WeatheringCopperChestWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::WeatheringCopperChestWaterlogged(WeatheringCopperChestWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::SculkVeinUp(SculkVeinUp::True) => output.push(("up".to_string(), "true".to_string())),
			Property::SculkVeinUp(SculkVeinUp::False) => output.push(("up".to_string(), "false".to_string())),
			Property::TargetPower(TargetPower::Num0) => output.push(("power".to_string(), "0".to_string())),
			Property::TargetPower(TargetPower::Num1) => output.push(("power".to_string(), "1".to_string())),
			Property::TargetPower(TargetPower::Num2) => output.push(("power".to_string(), "2".to_string())),
			Property::TargetPower(TargetPower::Num3) => output.push(("power".to_string(), "3".to_string())),
			Property::TargetPower(TargetPower::Num4) => output.push(("power".to_string(), "4".to_string())),
			Property::TargetPower(TargetPower::Num5) => output.push(("power".to_string(), "5".to_string())),
			Property::TargetPower(TargetPower::Num6) => output.push(("power".to_string(), "6".to_string())),
			Property::TargetPower(TargetPower::Num7) => output.push(("power".to_string(), "7".to_string())),
			Property::TargetPower(TargetPower::Num8) => output.push(("power".to_string(), "8".to_string())),
			Property::TargetPower(TargetPower::Num9) => output.push(("power".to_string(), "9".to_string())),
			Property::TargetPower(TargetPower::Num10) => output.push(("power".to_string(), "10".to_string())),
			Property::TargetPower(TargetPower::Num11) => output.push(("power".to_string(), "11".to_string())),
			Property::TargetPower(TargetPower::Num12) => output.push(("power".to_string(), "12".to_string())),
			Property::TargetPower(TargetPower::Num13) => output.push(("power".to_string(), "13".to_string())),
			Property::TargetPower(TargetPower::Num14) => output.push(("power".to_string(), "14".to_string())),
			Property::TargetPower(TargetPower::Num15) => output.push(("power".to_string(), "15".to_string())),
			Property::ShelfWaterlogged(ShelfWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::ShelfWaterlogged(ShelfWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::CakeBites(CakeBites::Num0) => output.push(("bites".to_string(), "0".to_string())),
			Property::CakeBites(CakeBites::Num1) => output.push(("bites".to_string(), "1".to_string())),
			Property::CakeBites(CakeBites::Num2) => output.push(("bites".to_string(), "2".to_string())),
			Property::CakeBites(CakeBites::Num3) => output.push(("bites".to_string(), "3".to_string())),
			Property::CakeBites(CakeBites::Num4) => output.push(("bites".to_string(), "4".to_string())),
			Property::CakeBites(CakeBites::Num5) => output.push(("bites".to_string(), "5".to_string())),
			Property::CakeBites(CakeBites::Num6) => output.push(("bites".to_string(), "6".to_string())),
			Property::CalibratedSculkSensorSculkSensorPhase(CalibratedSculkSensorSculkSensorPhase::Inactive) => output.push(("sculk_sensor_phase".to_string(), "inactive".to_string())),
			Property::CalibratedSculkSensorSculkSensorPhase(CalibratedSculkSensorSculkSensorPhase::Active) => output.push(("sculk_sensor_phase".to_string(), "active".to_string())),
			Property::CalibratedSculkSensorSculkSensorPhase(CalibratedSculkSensorSculkSensorPhase::Cooldown) => output.push(("sculk_sensor_phase".to_string(), "cooldown".to_string())),
			Property::CrafterCrafting(CrafterCrafting::True) => output.push(("crafting".to_string(), "true".to_string())),
			Property::CrafterCrafting(CrafterCrafting::False) => output.push(("crafting".to_string(), "false".to_string())),
			Property::SculkCatalystBloom(SculkCatalystBloom::True) => output.push(("bloom".to_string(), "true".to_string())),
			Property::SculkCatalystBloom(SculkCatalystBloom::False) => output.push(("bloom".to_string(), "false".to_string())),
			Property::TripwireAttached(TripwireAttached::True) => output.push(("attached".to_string(), "true".to_string())),
			Property::TripwireAttached(TripwireAttached::False) => output.push(("attached".to_string(), "false".to_string())),
			Property::TripWireHookPowered(TripWireHookPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::TripWireHookPowered(TripWireHookPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::ScaffoldingBottom(ScaffoldingBottom::True) => output.push(("bottom".to_string(), "true".to_string())),
			Property::ScaffoldingBottom(ScaffoldingBottom::False) => output.push(("bottom".to_string(), "false".to_string())),
			Property::WeatheringCopperDoorHalf(WeatheringCopperDoorHalf::Upper) => output.push(("half".to_string(), "upper".to_string())),
			Property::WeatheringCopperDoorHalf(WeatheringCopperDoorHalf::Lower) => output.push(("half".to_string(), "lower".to_string())),
			Property::MangrovePropaguleHanging(MangrovePropaguleHanging::True) => output.push(("hanging".to_string(), "true".to_string())),
			Property::MangrovePropaguleHanging(MangrovePropaguleHanging::False) => output.push(("hanging".to_string(), "false".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num4) => output.push(("age".to_string(), "4".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num5) => output.push(("age".to_string(), "5".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num6) => output.push(("age".to_string(), "6".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num7) => output.push(("age".to_string(), "7".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num8) => output.push(("age".to_string(), "8".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num9) => output.push(("age".to_string(), "9".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num10) => output.push(("age".to_string(), "10".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num11) => output.push(("age".to_string(), "11".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num12) => output.push(("age".to_string(), "12".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num13) => output.push(("age".to_string(), "13".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num14) => output.push(("age".to_string(), "14".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num15) => output.push(("age".to_string(), "15".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num16) => output.push(("age".to_string(), "16".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num17) => output.push(("age".to_string(), "17".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num18) => output.push(("age".to_string(), "18".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num19) => output.push(("age".to_string(), "19".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num20) => output.push(("age".to_string(), "20".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num21) => output.push(("age".to_string(), "21".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num22) => output.push(("age".to_string(), "22".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num23) => output.push(("age".to_string(), "23".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num24) => output.push(("age".to_string(), "24".to_string())),
			Property::CaveVinesAge(CaveVinesAge::Num25) => output.push(("age".to_string(), "25".to_string())),
			Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::SculkVeinWaterlogged(SculkVeinWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::SculkVeinWaterlogged(SculkVeinWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::MossyCarpetNorth(MossyCarpetNorth::None) => output.push(("north".to_string(), "none".to_string())),
			Property::MossyCarpetNorth(MossyCarpetNorth::Low) => output.push(("north".to_string(), "low".to_string())),
			Property::MossyCarpetNorth(MossyCarpetNorth::Tall) => output.push(("north".to_string(), "tall".to_string())),
			Property::RedstoneWallTorchLit(RedstoneWallTorchLit::True) => output.push(("lit".to_string(), "true".to_string())),
			Property::RedstoneWallTorchLit(RedstoneWallTorchLit::False) => output.push(("lit".to_string(), "false".to_string())),
			Property::PitcherCropAge(PitcherCropAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::PitcherCropAge(PitcherCropAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::PitcherCropAge(PitcherCropAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::PitcherCropAge(PitcherCropAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::PitcherCropAge(PitcherCropAge::Num4) => output.push(("age".to_string(), "4".to_string())),
			Property::BigDripleafFacing(BigDripleafFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::BigDripleafFacing(BigDripleafFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::BigDripleafFacing(BigDripleafFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::BigDripleafFacing(BigDripleafFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::UntintedParticleLeavesPersistent(UntintedParticleLeavesPersistent::True) => output.push(("persistent".to_string(), "true".to_string())),
			Property::UntintedParticleLeavesPersistent(UntintedParticleLeavesPersistent::False) => output.push(("persistent".to_string(), "false".to_string())),
			Property::WeatheringCopperDoorHinge(WeatheringCopperDoorHinge::Left) => output.push(("hinge".to_string(), "left".to_string())),
			Property::WeatheringCopperDoorHinge(WeatheringCopperDoorHinge::Right) => output.push(("hinge".to_string(), "right".to_string())),
			Property::PitcherCropHalf(PitcherCropHalf::Upper) => output.push(("half".to_string(), "upper".to_string())),
			Property::PitcherCropHalf(PitcherCropHalf::Lower) => output.push(("half".to_string(), "lower".to_string())),
			Property::MultifaceWest(MultifaceWest::True) => output.push(("west".to_string(), "true".to_string())),
			Property::MultifaceWest(MultifaceWest::False) => output.push(("west".to_string(), "false".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num0) => output.push(("power".to_string(), "0".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num1) => output.push(("power".to_string(), "1".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num2) => output.push(("power".to_string(), "2".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num3) => output.push(("power".to_string(), "3".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num4) => output.push(("power".to_string(), "4".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num5) => output.push(("power".to_string(), "5".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num6) => output.push(("power".to_string(), "6".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num7) => output.push(("power".to_string(), "7".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num8) => output.push(("power".to_string(), "8".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num9) => output.push(("power".to_string(), "9".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num10) => output.push(("power".to_string(), "10".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num11) => output.push(("power".to_string(), "11".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num12) => output.push(("power".to_string(), "12".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num13) => output.push(("power".to_string(), "13".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num14) => output.push(("power".to_string(), "14".to_string())),
			Property::SculkSensorPower(SculkSensorPower::Num15) => output.push(("power".to_string(), "15".to_string())),
			Property::WaterloggedTransparentWaterlogged(WaterloggedTransparentWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::WaterloggedTransparentWaterlogged(WaterloggedTransparentWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::NetherPortalAxis(NetherPortalAxis::X) => output.push(("axis".to_string(), "x".to_string())),
			Property::NetherPortalAxis(NetherPortalAxis::Z) => output.push(("axis".to_string(), "z".to_string())),
			Property::CreakingHeartNatural(CreakingHeartNatural::True) => output.push(("natural".to_string(), "true".to_string())),
			Property::CreakingHeartNatural(CreakingHeartNatural::False) => output.push(("natural".to_string(), "false".to_string())),
			Property::TestMode(TestMode::Start) => output.push(("mode".to_string(), "start".to_string())),
			Property::TestMode(TestMode::Log) => output.push(("mode".to_string(), "log".to_string())),
			Property::TestMode(TestMode::Fail) => output.push(("mode".to_string(), "fail".to_string())),
			Property::TestMode(TestMode::Accept) => output.push(("mode".to_string(), "accept".to_string())),
			Property::WeatheringCopperBulbPowered(WeatheringCopperBulbPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::WeatheringCopperBulbPowered(WeatheringCopperBulbPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::PoweredRailWaterlogged(PoweredRailWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::PoweredRailWaterlogged(PoweredRailWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::ChainAxis(ChainAxis::X) => output.push(("axis".to_string(), "x".to_string())),
			Property::ChainAxis(ChainAxis::Y) => output.push(("axis".to_string(), "y".to_string())),
			Property::ChainAxis(ChainAxis::Z) => output.push(("axis".to_string(), "z".to_string())),
			Property::DriedGhastHydration(DriedGhastHydration::Num0) => output.push(("hydration".to_string(), "0".to_string())),
			Property::DriedGhastHydration(DriedGhastHydration::Num1) => output.push(("hydration".to_string(), "1".to_string())),
			Property::DriedGhastHydration(DriedGhastHydration::Num2) => output.push(("hydration".to_string(), "2".to_string())),
			Property::DriedGhastHydration(DriedGhastHydration::Num3) => output.push(("hydration".to_string(), "3".to_string())),
			Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::Inactive) => output.push(("trial_spawner_state".to_string(), "inactive".to_string())),
			Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::WaitingForPlayers) => output.push(("trial_spawner_state".to_string(), "waiting_for_players".to_string())),
			Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::Active) => output.push(("trial_spawner_state".to_string(), "active".to_string())),
			Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::WaitingForRewardEjection) => output.push(("trial_spawner_state".to_string(), "waiting_for_reward_ejection".to_string())),
			Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::EjectingReward) => output.push(("trial_spawner_state".to_string(), "ejecting_reward".to_string())),
			Property::TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState::Cooldown) => output.push(("trial_spawner_state".to_string(), "cooldown".to_string())),
			Property::BedFacing(BedFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::BedFacing(BedFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::BedFacing(BedFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::BedFacing(BedFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::WitherWallSkullFacing(WitherWallSkullFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::WitherWallSkullFacing(WitherWallSkullFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::WitherWallSkullFacing(WitherWallSkullFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::WitherWallSkullFacing(WitherWallSkullFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::CeilingHangingSignWaterlogged(CeilingHangingSignWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::CeilingHangingSignWaterlogged(CeilingHangingSignWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::ChiseledBookShelfSlot0Occupied(ChiseledBookShelfSlot0Occupied::True) => output.push(("slot_0_occupied".to_string(), "true".to_string())),
			Property::ChiseledBookShelfSlot0Occupied(ChiseledBookShelfSlot0Occupied::False) => output.push(("slot_0_occupied".to_string(), "false".to_string())),
			Property::WeatheringCopperChestType(WeatheringCopperChestType::Single) => output.push(("type".to_string(), "single".to_string())),
			Property::WeatheringCopperChestType(WeatheringCopperChestType::Left) => output.push(("type".to_string(), "left".to_string())),
			Property::WeatheringCopperChestType(WeatheringCopperChestType::Right) => output.push(("type".to_string(), "right".to_string())),
			Property::WeatheringCopperGolemStatueWaterlogged(WeatheringCopperGolemStatueWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::WeatheringCopperGolemStatueWaterlogged(WeatheringCopperGolemStatueWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::TintedParticleLeavesPersistent(TintedParticleLeavesPersistent::True) => output.push(("persistent".to_string(), "true".to_string())),
			Property::TintedParticleLeavesPersistent(TintedParticleLeavesPersistent::False) => output.push(("persistent".to_string(), "false".to_string())),
			Property::LadderFacing(LadderFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::LadderFacing(LadderFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::LadderFacing(LadderFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::LadderFacing(LadderFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::BrewingStandHasBottle0(BrewingStandHasBottle0::True) => output.push(("has_bottle_0".to_string(), "true".to_string())),
			Property::BrewingStandHasBottle0(BrewingStandHasBottle0::False) => output.push(("has_bottle_0".to_string(), "false".to_string())),
			Property::LeverPowered(LeverPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::LeverPowered(LeverPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::ChainWaterlogged(ChainWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::ChainWaterlogged(ChainWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::WeatheringCopperTrapDoorPowered(WeatheringCopperTrapDoorPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::WeatheringCopperTrapDoorPowered(WeatheringCopperTrapDoorPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::CampfireFacing(CampfireFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::CampfireFacing(CampfireFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::CampfireFacing(CampfireFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::CampfireFacing(CampfireFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::GrindstoneFace(GrindstoneFace::Floor) => output.push(("face".to_string(), "floor".to_string())),
			Property::GrindstoneFace(GrindstoneFace::Wall) => output.push(("face".to_string(), "wall".to_string())),
			Property::GrindstoneFace(GrindstoneFace::Ceiling) => output.push(("face".to_string(), "ceiling".to_string())),
			Property::PressurePlatePowered(PressurePlatePowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::PressurePlatePowered(PressurePlatePowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::HangingRootsWaterlogged(HangingRootsWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::HangingRootsWaterlogged(HangingRootsWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::FireEast(FireEast::True) => output.push(("east".to_string(), "true".to_string())),
			Property::FireEast(FireEast::False) => output.push(("east".to_string(), "false".to_string())),
			Property::EndPortalFrameEye(EndPortalFrameEye::True) => output.push(("eye".to_string(), "true".to_string())),
			Property::EndPortalFrameEye(EndPortalFrameEye::False) => output.push(("eye".to_string(), "false".to_string())),
			Property::RailWaterlogged(RailWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::RailWaterlogged(RailWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::SmallDripleafWaterlogged(SmallDripleafWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::SmallDripleafWaterlogged(SmallDripleafWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::LecternFacing(LecternFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::LecternFacing(LecternFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::LecternFacing(LecternFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::LecternFacing(LecternFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::LeverFacing(LeverFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::LeverFacing(LeverFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::LeverFacing(LeverFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::LeverFacing(LeverFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::MyceliumSnowy(MyceliumSnowy::True) => output.push(("snowy".to_string(), "true".to_string())),
			Property::MyceliumSnowy(MyceliumSnowy::False) => output.push(("snowy".to_string(), "false".to_string())),
			Property::FenceGateInWall(FenceGateInWall::True) => output.push(("in_wall".to_string(), "true".to_string())),
			Property::FenceGateInWall(FenceGateInWall::False) => output.push(("in_wall".to_string(), "false".to_string())),
			Property::DoorHinge(DoorHinge::Left) => output.push(("hinge".to_string(), "left".to_string())),
			Property::DoorHinge(DoorHinge::Right) => output.push(("hinge".to_string(), "right".to_string())),
			Property::HugeMushroomUp(HugeMushroomUp::True) => output.push(("up".to_string(), "true".to_string())),
			Property::HugeMushroomUp(HugeMushroomUp::False) => output.push(("up".to_string(), "false".to_string())),
			Property::NetherWartAge(NetherWartAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::NetherWartAge(NetherWartAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::NetherWartAge(NetherWartAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::NetherWartAge(NetherWartAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::HugeMushroomWest(HugeMushroomWest::True) => output.push(("west".to_string(), "true".to_string())),
			Property::HugeMushroomWest(HugeMushroomWest::False) => output.push(("west".to_string(), "false".to_string())),
			Property::VaultVaultState(VaultVaultState::Inactive) => output.push(("vault_state".to_string(), "inactive".to_string())),
			Property::VaultVaultState(VaultVaultState::Active) => output.push(("vault_state".to_string(), "active".to_string())),
			Property::VaultVaultState(VaultVaultState::Unlocking) => output.push(("vault_state".to_string(), "unlocking".to_string())),
			Property::VaultVaultState(VaultVaultState::Ejecting) => output.push(("vault_state".to_string(), "ejecting".to_string())),
			Property::CropAge(CropAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::CropAge(CropAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::CropAge(CropAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::CropAge(CropAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::CropAge(CropAge::Num4) => output.push(("age".to_string(), "4".to_string())),
			Property::CropAge(CropAge::Num5) => output.push(("age".to_string(), "5".to_string())),
			Property::CropAge(CropAge::Num6) => output.push(("age".to_string(), "6".to_string())),
			Property::CropAge(CropAge::Num7) => output.push(("age".to_string(), "7".to_string())),
			Property::ChestType(ChestType::Single) => output.push(("type".to_string(), "single".to_string())),
			Property::ChestType(ChestType::Left) => output.push(("type".to_string(), "left".to_string())),
			Property::ChestType(ChestType::Right) => output.push(("type".to_string(), "right".to_string())),
			Property::DoorFacing(DoorFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::DoorFacing(DoorFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::DoorFacing(DoorFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::DoorFacing(DoorFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::ShelfFacing(ShelfFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::ShelfFacing(ShelfFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::ShelfFacing(ShelfFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::ShelfFacing(ShelfFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::MangrovePropaguleWaterlogged(MangrovePropaguleWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::MangrovePropaguleWaterlogged(MangrovePropaguleWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::KelpAge(KelpAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::KelpAge(KelpAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::KelpAge(KelpAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::KelpAge(KelpAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::KelpAge(KelpAge::Num4) => output.push(("age".to_string(), "4".to_string())),
			Property::KelpAge(KelpAge::Num5) => output.push(("age".to_string(), "5".to_string())),
			Property::KelpAge(KelpAge::Num6) => output.push(("age".to_string(), "6".to_string())),
			Property::KelpAge(KelpAge::Num7) => output.push(("age".to_string(), "7".to_string())),
			Property::KelpAge(KelpAge::Num8) => output.push(("age".to_string(), "8".to_string())),
			Property::KelpAge(KelpAge::Num9) => output.push(("age".to_string(), "9".to_string())),
			Property::KelpAge(KelpAge::Num10) => output.push(("age".to_string(), "10".to_string())),
			Property::KelpAge(KelpAge::Num11) => output.push(("age".to_string(), "11".to_string())),
			Property::KelpAge(KelpAge::Num12) => output.push(("age".to_string(), "12".to_string())),
			Property::KelpAge(KelpAge::Num13) => output.push(("age".to_string(), "13".to_string())),
			Property::KelpAge(KelpAge::Num14) => output.push(("age".to_string(), "14".to_string())),
			Property::KelpAge(KelpAge::Num15) => output.push(("age".to_string(), "15".to_string())),
			Property::KelpAge(KelpAge::Num16) => output.push(("age".to_string(), "16".to_string())),
			Property::KelpAge(KelpAge::Num17) => output.push(("age".to_string(), "17".to_string())),
			Property::KelpAge(KelpAge::Num18) => output.push(("age".to_string(), "18".to_string())),
			Property::KelpAge(KelpAge::Num19) => output.push(("age".to_string(), "19".to_string())),
			Property::KelpAge(KelpAge::Num20) => output.push(("age".to_string(), "20".to_string())),
			Property::KelpAge(KelpAge::Num21) => output.push(("age".to_string(), "21".to_string())),
			Property::KelpAge(KelpAge::Num22) => output.push(("age".to_string(), "22".to_string())),
			Property::KelpAge(KelpAge::Num23) => output.push(("age".to_string(), "23".to_string())),
			Property::KelpAge(KelpAge::Num24) => output.push(("age".to_string(), "24".to_string())),
			Property::KelpAge(KelpAge::Num25) => output.push(("age".to_string(), "25".to_string())),
			Property::SlabType(SlabType::Top) => output.push(("type".to_string(), "top".to_string())),
			Property::SlabType(SlabType::Bottom) => output.push(("type".to_string(), "bottom".to_string())),
			Property::SlabType(SlabType::Double) => output.push(("type".to_string(), "double".to_string())),
			Property::BarrelFacing(BarrelFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::BarrelFacing(BarrelFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::BarrelFacing(BarrelFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::BarrelFacing(BarrelFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::BarrelFacing(BarrelFacing::Up) => output.push(("facing".to_string(), "up".to_string())),
			Property::BarrelFacing(BarrelFacing::Down) => output.push(("facing".to_string(), "down".to_string())),
			Property::CrafterOrientation(CrafterOrientation::DownEast) => output.push(("orientation".to_string(), "down_east".to_string())),
			Property::CrafterOrientation(CrafterOrientation::DownNorth) => output.push(("orientation".to_string(), "down_north".to_string())),
			Property::CrafterOrientation(CrafterOrientation::DownSouth) => output.push(("orientation".to_string(), "down_south".to_string())),
			Property::CrafterOrientation(CrafterOrientation::DownWest) => output.push(("orientation".to_string(), "down_west".to_string())),
			Property::CrafterOrientation(CrafterOrientation::UpEast) => output.push(("orientation".to_string(), "up_east".to_string())),
			Property::CrafterOrientation(CrafterOrientation::UpNorth) => output.push(("orientation".to_string(), "up_north".to_string())),
			Property::CrafterOrientation(CrafterOrientation::UpSouth) => output.push(("orientation".to_string(), "up_south".to_string())),
			Property::CrafterOrientation(CrafterOrientation::UpWest) => output.push(("orientation".to_string(), "up_west".to_string())),
			Property::CrafterOrientation(CrafterOrientation::WestUp) => output.push(("orientation".to_string(), "west_up".to_string())),
			Property::CrafterOrientation(CrafterOrientation::EastUp) => output.push(("orientation".to_string(), "east_up".to_string())),
			Property::CrafterOrientation(CrafterOrientation::NorthUp) => output.push(("orientation".to_string(), "north_up".to_string())),
			Property::CrafterOrientation(CrafterOrientation::SouthUp) => output.push(("orientation".to_string(), "south_up".to_string())),
			Property::DaylightDetectorInverted(DaylightDetectorInverted::True) => output.push(("inverted".to_string(), "true".to_string())),
			Property::DaylightDetectorInverted(DaylightDetectorInverted::False) => output.push(("inverted".to_string(), "false".to_string())),
			Property::FenceEast(FenceEast::True) => output.push(("east".to_string(), "true".to_string())),
			Property::FenceEast(FenceEast::False) => output.push(("east".to_string(), "false".to_string())),
			Property::NotePowered(NotePowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::NotePowered(NotePowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::SeaPickleWaterlogged(SeaPickleWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::SeaPickleWaterlogged(SeaPickleWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num1) => output.push(("distance".to_string(), "1".to_string())),
			Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num2) => output.push(("distance".to_string(), "2".to_string())),
			Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num3) => output.push(("distance".to_string(), "3".to_string())),
			Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num4) => output.push(("distance".to_string(), "4".to_string())),
			Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num5) => output.push(("distance".to_string(), "5".to_string())),
			Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num6) => output.push(("distance".to_string(), "6".to_string())),
			Property::UntintedParticleLeavesDistance(UntintedParticleLeavesDistance::Num7) => output.push(("distance".to_string(), "7".to_string())),
			Property::JackOLanternFacing(JackOLanternFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::JackOLanternFacing(JackOLanternFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::JackOLanternFacing(JackOLanternFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::JackOLanternFacing(JackOLanternFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::WeatheringCopperBarWaterlogged(WeatheringCopperBarWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::WeatheringCopperBarWaterlogged(WeatheringCopperBarWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::SculkSensorWaterlogged(SculkSensorWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::SculkSensorWaterlogged(SculkSensorWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::MossyCarpetWest(MossyCarpetWest::None) => output.push(("west".to_string(), "none".to_string())),
			Property::MossyCarpetWest(MossyCarpetWest::Low) => output.push(("west".to_string(), "low".to_string())),
			Property::MossyCarpetWest(MossyCarpetWest::Tall) => output.push(("west".to_string(), "tall".to_string())),
			Property::CommandConditional(CommandConditional::True) => output.push(("conditional".to_string(), "true".to_string())),
			Property::CommandConditional(CommandConditional::False) => output.push(("conditional".to_string(), "false".to_string())),
			Property::FireWest(FireWest::True) => output.push(("west".to_string(), "true".to_string())),
			Property::FireWest(FireWest::False) => output.push(("west".to_string(), "false".to_string())),
			Property::GlowLichenWest(GlowLichenWest::True) => output.push(("west".to_string(), "true".to_string())),
			Property::GlowLichenWest(GlowLichenWest::False) => output.push(("west".to_string(), "false".to_string())),
			Property::DoorOpen(DoorOpen::True) => output.push(("open".to_string(), "true".to_string())),
			Property::DoorOpen(DoorOpen::False) => output.push(("open".to_string(), "false".to_string())),
			Property::ComparatorFacing(ComparatorFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::ComparatorFacing(ComparatorFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::ComparatorFacing(ComparatorFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::ComparatorFacing(ComparatorFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::MangroveLeavesPersistent(MangroveLeavesPersistent::True) => output.push(("persistent".to_string(), "true".to_string())),
			Property::MangroveLeavesPersistent(MangroveLeavesPersistent::False) => output.push(("persistent".to_string(), "false".to_string())),
			Property::PlayerHeadPowered(PlayerHeadPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::PlayerHeadPowered(PlayerHeadPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::PlayerWallHeadFacing(PlayerWallHeadFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::PlayerWallHeadFacing(PlayerWallHeadFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::PlayerWallHeadFacing(PlayerWallHeadFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::PlayerWallHeadFacing(PlayerWallHeadFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::TorchflowerCropAge(TorchflowerCropAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::TorchflowerCropAge(TorchflowerCropAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::StandingSignWaterlogged(StandingSignWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::StandingSignWaterlogged(StandingSignWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::TrappedChestType(TrappedChestType::Single) => output.push(("type".to_string(), "single".to_string())),
			Property::TrappedChestType(TrappedChestType::Left) => output.push(("type".to_string(), "left".to_string())),
			Property::TrappedChestType(TrappedChestType::Right) => output.push(("type".to_string(), "right".to_string())),
			Property::CopperGolemStatueWaterlogged(CopperGolemStatueWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::CopperGolemStatueWaterlogged(CopperGolemStatueWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::LadderWaterlogged(LadderWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::LadderWaterlogged(LadderWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::WeatheringCopperBarNorth(WeatheringCopperBarNorth::True) => output.push(("north".to_string(), "true".to_string())),
			Property::WeatheringCopperBarNorth(WeatheringCopperBarNorth::False) => output.push(("north".to_string(), "false".to_string())),
			Property::FireSouth(FireSouth::True) => output.push(("south".to_string(), "true".to_string())),
			Property::FireSouth(FireSouth::False) => output.push(("south".to_string(), "false".to_string())),
			Property::ChorusFlowerAge(ChorusFlowerAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::ChorusFlowerAge(ChorusFlowerAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::ChorusFlowerAge(ChorusFlowerAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::ChorusFlowerAge(ChorusFlowerAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::ChorusFlowerAge(ChorusFlowerAge::Num4) => output.push(("age".to_string(), "4".to_string())),
			Property::ChorusFlowerAge(ChorusFlowerAge::Num5) => output.push(("age".to_string(), "5".to_string())),
			Property::MovingPistonFacing(MovingPistonFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::MovingPistonFacing(MovingPistonFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::MovingPistonFacing(MovingPistonFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::MovingPistonFacing(MovingPistonFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::MovingPistonFacing(MovingPistonFacing::Up) => output.push(("facing".to_string(), "up".to_string())),
			Property::MovingPistonFacing(MovingPistonFacing::Down) => output.push(("facing".to_string(), "down".to_string())),
			Property::MossyCarpetSouth(MossyCarpetSouth::None) => output.push(("south".to_string(), "none".to_string())),
			Property::MossyCarpetSouth(MossyCarpetSouth::Low) => output.push(("south".to_string(), "low".to_string())),
			Property::MossyCarpetSouth(MossyCarpetSouth::Tall) => output.push(("south".to_string(), "tall".to_string())),
			Property::TurtleEggHatch(TurtleEggHatch::Num0) => output.push(("hatch".to_string(), "0".to_string())),
			Property::TurtleEggHatch(TurtleEggHatch::Num1) => output.push(("hatch".to_string(), "1".to_string())),
			Property::TurtleEggHatch(TurtleEggHatch::Num2) => output.push(("hatch".to_string(), "2".to_string())),
			Property::WallTorchFacing(WallTorchFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::WallTorchFacing(WallTorchFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::WallTorchFacing(WallTorchFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::WallTorchFacing(WallTorchFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::CandleWaterlogged(CandleWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::CandleWaterlogged(CandleWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::DispenserFacing(DispenserFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::DispenserFacing(DispenserFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::DispenserFacing(DispenserFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::DispenserFacing(DispenserFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::DispenserFacing(DispenserFacing::Up) => output.push(("facing".to_string(), "up".to_string())),
			Property::DispenserFacing(DispenserFacing::Down) => output.push(("facing".to_string(), "down".to_string())),
			Property::ChestFacing(ChestFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::ChestFacing(ChestFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::ChestFacing(ChestFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::ChestFacing(ChestFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::HeavyCoreWaterlogged(HeavyCoreWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::HeavyCoreWaterlogged(HeavyCoreWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::StairShape(StairShape::Straight) => output.push(("shape".to_string(), "straight".to_string())),
			Property::StairShape(StairShape::InnerLeft) => output.push(("shape".to_string(), "inner_left".to_string())),
			Property::StairShape(StairShape::InnerRight) => output.push(("shape".to_string(), "inner_right".to_string())),
			Property::StairShape(StairShape::OuterLeft) => output.push(("shape".to_string(), "outer_left".to_string())),
			Property::StairShape(StairShape::OuterRight) => output.push(("shape".to_string(), "outer_right".to_string())),
			Property::ShelfSideChain(ShelfSideChain::Unconnected) => output.push(("side_chain".to_string(), "unconnected".to_string())),
			Property::ShelfSideChain(ShelfSideChain::Right) => output.push(("side_chain".to_string(), "right".to_string())),
			Property::ShelfSideChain(ShelfSideChain::Center) => output.push(("side_chain".to_string(), "center".to_string())),
			Property::ShelfSideChain(ShelfSideChain::Left) => output.push(("side_chain".to_string(), "left".to_string())),
			Property::SlabWaterlogged(SlabWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::SlabWaterlogged(SlabWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::WeatheringCopperSlabType(WeatheringCopperSlabType::Top) => output.push(("type".to_string(), "top".to_string())),
			Property::WeatheringCopperSlabType(WeatheringCopperSlabType::Bottom) => output.push(("type".to_string(), "bottom".to_string())),
			Property::WeatheringCopperSlabType(WeatheringCopperSlabType::Double) => output.push(("type".to_string(), "double".to_string())),
			Property::TrapdoorPowered(TrapdoorPowered::True) => output.push(("powered".to_string(), "true".to_string())),
			Property::TrapdoorPowered(TrapdoorPowered::False) => output.push(("powered".to_string(), "false".to_string())),
			Property::WallSignWaterlogged(WallSignWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::WallSignWaterlogged(WallSignWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::CoralWallFanWaterlogged(CoralWallFanWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::CoralWallFanWaterlogged(CoralWallFanWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::HayAxis(HayAxis::X) => output.push(("axis".to_string(), "x".to_string())),
			Property::HayAxis(HayAxis::Y) => output.push(("axis".to_string(), "y".to_string())),
			Property::HayAxis(HayAxis::Z) => output.push(("axis".to_string(), "z".to_string())),
			Property::HopperEnabled(HopperEnabled::True) => output.push(("enabled".to_string(), "true".to_string())),
			Property::HopperEnabled(HopperEnabled::False) => output.push(("enabled".to_string(), "false".to_string())),
			Property::TurtleEggEggs(TurtleEggEggs::Num1) => output.push(("eggs".to_string(), "1".to_string())),
			Property::TurtleEggEggs(TurtleEggEggs::Num2) => output.push(("eggs".to_string(), "2".to_string())),
			Property::TurtleEggEggs(TurtleEggEggs::Num3) => output.push(("eggs".to_string(), "3".to_string())),
			Property::TurtleEggEggs(TurtleEggEggs::Num4) => output.push(("eggs".to_string(), "4".to_string())),
			Property::TripwireNorth(TripwireNorth::True) => output.push(("north".to_string(), "true".to_string())),
			Property::TripwireNorth(TripwireNorth::False) => output.push(("north".to_string(), "false".to_string())),
			Property::TallFlowerHalf(TallFlowerHalf::Upper) => output.push(("half".to_string(), "upper".to_string())),
			Property::TallFlowerHalf(TallFlowerHalf::Lower) => output.push(("half".to_string(), "lower".to_string())),
			Property::NoteNote(NoteNote::Num0) => output.push(("note".to_string(), "0".to_string())),
			Property::NoteNote(NoteNote::Num1) => output.push(("note".to_string(), "1".to_string())),
			Property::NoteNote(NoteNote::Num2) => output.push(("note".to_string(), "2".to_string())),
			Property::NoteNote(NoteNote::Num3) => output.push(("note".to_string(), "3".to_string())),
			Property::NoteNote(NoteNote::Num4) => output.push(("note".to_string(), "4".to_string())),
			Property::NoteNote(NoteNote::Num5) => output.push(("note".to_string(), "5".to_string())),
			Property::NoteNote(NoteNote::Num6) => output.push(("note".to_string(), "6".to_string())),
			Property::NoteNote(NoteNote::Num7) => output.push(("note".to_string(), "7".to_string())),
			Property::NoteNote(NoteNote::Num8) => output.push(("note".to_string(), "8".to_string())),
			Property::NoteNote(NoteNote::Num9) => output.push(("note".to_string(), "9".to_string())),
			Property::NoteNote(NoteNote::Num10) => output.push(("note".to_string(), "10".to_string())),
			Property::NoteNote(NoteNote::Num11) => output.push(("note".to_string(), "11".to_string())),
			Property::NoteNote(NoteNote::Num12) => output.push(("note".to_string(), "12".to_string())),
			Property::NoteNote(NoteNote::Num13) => output.push(("note".to_string(), "13".to_string())),
			Property::NoteNote(NoteNote::Num14) => output.push(("note".to_string(), "14".to_string())),
			Property::NoteNote(NoteNote::Num15) => output.push(("note".to_string(), "15".to_string())),
			Property::NoteNote(NoteNote::Num16) => output.push(("note".to_string(), "16".to_string())),
			Property::NoteNote(NoteNote::Num17) => output.push(("note".to_string(), "17".to_string())),
			Property::NoteNote(NoteNote::Num18) => output.push(("note".to_string(), "18".to_string())),
			Property::NoteNote(NoteNote::Num19) => output.push(("note".to_string(), "19".to_string())),
			Property::NoteNote(NoteNote::Num20) => output.push(("note".to_string(), "20".to_string())),
			Property::NoteNote(NoteNote::Num21) => output.push(("note".to_string(), "21".to_string())),
			Property::NoteNote(NoteNote::Num22) => output.push(("note".to_string(), "22".to_string())),
			Property::NoteNote(NoteNote::Num23) => output.push(("note".to_string(), "23".to_string())),
			Property::NoteNote(NoteNote::Num24) => output.push(("note".to_string(), "24".to_string())),
			Property::MossyCarpetBottom(MossyCarpetBottom::True) => output.push(("bottom".to_string(), "true".to_string())),
			Property::MossyCarpetBottom(MossyCarpetBottom::False) => output.push(("bottom".to_string(), "false".to_string())),
			Property::CopperChestType(CopperChestType::Single) => output.push(("type".to_string(), "single".to_string())),
			Property::CopperChestType(CopperChestType::Left) => output.push(("type".to_string(), "left".to_string())),
			Property::CopperChestType(CopperChestType::Right) => output.push(("type".to_string(), "right".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num0) => output.push(("age".to_string(), "0".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num1) => output.push(("age".to_string(), "1".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num2) => output.push(("age".to_string(), "2".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num3) => output.push(("age".to_string(), "3".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num4) => output.push(("age".to_string(), "4".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num5) => output.push(("age".to_string(), "5".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num6) => output.push(("age".to_string(), "6".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num7) => output.push(("age".to_string(), "7".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num8) => output.push(("age".to_string(), "8".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num9) => output.push(("age".to_string(), "9".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num10) => output.push(("age".to_string(), "10".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num11) => output.push(("age".to_string(), "11".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num12) => output.push(("age".to_string(), "12".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num13) => output.push(("age".to_string(), "13".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num14) => output.push(("age".to_string(), "14".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num15) => output.push(("age".to_string(), "15".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num16) => output.push(("age".to_string(), "16".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num17) => output.push(("age".to_string(), "17".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num18) => output.push(("age".to_string(), "18".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num19) => output.push(("age".to_string(), "19".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num20) => output.push(("age".to_string(), "20".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num21) => output.push(("age".to_string(), "21".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num22) => output.push(("age".to_string(), "22".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num23) => output.push(("age".to_string(), "23".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num24) => output.push(("age".to_string(), "24".to_string())),
			Property::WeepingVinesAge(WeepingVinesAge::Num25) => output.push(("age".to_string(), "25".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num0) => output.push(("rotation".to_string(), "0".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num1) => output.push(("rotation".to_string(), "1".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num2) => output.push(("rotation".to_string(), "2".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num3) => output.push(("rotation".to_string(), "3".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num4) => output.push(("rotation".to_string(), "4".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num5) => output.push(("rotation".to_string(), "5".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num6) => output.push(("rotation".to_string(), "6".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num7) => output.push(("rotation".to_string(), "7".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num8) => output.push(("rotation".to_string(), "8".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num9) => output.push(("rotation".to_string(), "9".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num10) => output.push(("rotation".to_string(), "10".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num11) => output.push(("rotation".to_string(), "11".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num12) => output.push(("rotation".to_string(), "12".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num13) => output.push(("rotation".to_string(), "13".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num14) => output.push(("rotation".to_string(), "14".to_string())),
			Property::WitherSkullRotation(WitherSkullRotation::Num15) => output.push(("rotation".to_string(), "15".to_string())),
			Property::FenceGateOpen(FenceGateOpen::True) => output.push(("open".to_string(), "true".to_string())),
			Property::FenceGateOpen(FenceGateOpen::False) => output.push(("open".to_string(), "false".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num0) => output.push(("rotation".to_string(), "0".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num1) => output.push(("rotation".to_string(), "1".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num2) => output.push(("rotation".to_string(), "2".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num3) => output.push(("rotation".to_string(), "3".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num4) => output.push(("rotation".to_string(), "4".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num5) => output.push(("rotation".to_string(), "5".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num6) => output.push(("rotation".to_string(), "6".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num7) => output.push(("rotation".to_string(), "7".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num8) => output.push(("rotation".to_string(), "8".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num9) => output.push(("rotation".to_string(), "9".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num10) => output.push(("rotation".to_string(), "10".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num11) => output.push(("rotation".to_string(), "11".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num12) => output.push(("rotation".to_string(), "12".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num13) => output.push(("rotation".to_string(), "13".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num14) => output.push(("rotation".to_string(), "14".to_string())),
			Property::StandingSignRotation(StandingSignRotation::Num15) => output.push(("rotation".to_string(), "15".to_string())),
			Property::DecoratedPotWaterlogged(DecoratedPotWaterlogged::True) => output.push(("waterlogged".to_string(), "true".to_string())),
			Property::DecoratedPotWaterlogged(DecoratedPotWaterlogged::False) => output.push(("waterlogged".to_string(), "false".to_string())),
			Property::DriedGhastFacing(DriedGhastFacing::North) => output.push(("facing".to_string(), "north".to_string())),
			Property::DriedGhastFacing(DriedGhastFacing::South) => output.push(("facing".to_string(), "south".to_string())),
			Property::DriedGhastFacing(DriedGhastFacing::West) => output.push(("facing".to_string(), "west".to_string())),
			Property::DriedGhastFacing(DriedGhastFacing::East) => output.push(("facing".to_string(), "east".to_string())),
			Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Standing) => output.push(("copper_golem_pose".to_string(), "standing".to_string())),
			Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Sitting) => output.push(("copper_golem_pose".to_string(), "sitting".to_string())),
			Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Running) => output.push(("copper_golem_pose".to_string(), "running".to_string())),
			Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Star) => output.push(("copper_golem_pose".to_string(), "star".to_string())),
			Property::RedstoneWireSouth(RedstoneWireSouth::Up) => output.push(("south".to_string(), "up".to_string())),
			Property::RedstoneWireSouth(RedstoneWireSouth::Side) => output.push(("south".to_string(), "side".to_string())),
			Property::RedstoneWireSouth(RedstoneWireSouth::None) => output.push(("south".to_string(), "none".to_string())),
			Property::ScaffoldingDistance(ScaffoldingDistance::Num0) => output.push(("distance".to_string(), "0".to_string())),
			Property::ScaffoldingDistance(ScaffoldingDistance::Num1) => output.push(("distance".to_string(), "1".to_string())),
			Property::ScaffoldingDistance(ScaffoldingDistance::Num2) => output.push(("distance".to_string(), "2".to_string())),
			Property::ScaffoldingDistance(ScaffoldingDistance::Num3) => output.push(("distance".to_string(), "3".to_string())),
			Property::ScaffoldingDistance(ScaffoldingDistance::Num4) => output.push(("distance".to_string(), "4".to_string())),
			Property::ScaffoldingDistance(ScaffoldingDistance::Num5) => output.push(("distance".to_string(), "5".to_string())),
			Property::ScaffoldingDistance(ScaffoldingDistance::Num6) => output.push(("distance".to_string(), "6".to_string())),
			Property::ScaffoldingDistance(ScaffoldingDistance::Num7) => output.push(("distance".to_string(), "7".to_string())),
		}
	}
	return output;
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::AmethystClusterWaterlogged(AmethystClusterWaterlogged::True)),
				"false" => x.properties.contains(&Property::AmethystClusterWaterlogged(AmethystClusterWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::AmethystClusterFacing(AmethystClusterFacing::North)),
				"east" => x.properties.contains(&Property::AmethystClusterFacing(AmethystClusterFacing::East)),
				"south" => x.properties.contains(&Property::AmethystClusterFacing(AmethystClusterFacing::South)),
				"west" => x.properties.contains(&Property::AmethystClusterFacing(AmethystClusterFacing::West)),
				"up" => x.properties.contains(&Property::AmethystClusterFacing(AmethystClusterFacing::Up)),
				"down" => x.properties.contains(&Property::AmethystClusterFacing(AmethystClusterFacing::Down)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::BambooStalkAge(BambooStalkAge::Num0)),
				"1" => x.properties.contains(&Property::BambooStalkAge(BambooStalkAge::Num1)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "open").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BarrelOpen(BarrelOpen::True)),
				"false" => x.properties.contains(&Property::BarrelOpen(BarrelOpen::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::BarrelFacing(BarrelFacing::North)),
				"east" => x.properties.contains(&Property::BarrelFacing(BarrelFacing::East)),
				"south" => x.properties.contains(&Property::BarrelFacing(BarrelFacing::South)),
				"west" => x.properties.contains(&Property::BarrelFacing(BarrelFacing::West)),
				"up" => x.properties.contains(&Property::BarrelFacing(BarrelFacing::Up)),
				"down" => x.properties.contains(&Property::BarrelFacing(BarrelFacing::Down)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "occupied").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BedOccupied(BedOccupied::True)),
				"false" => x.properties.contains(&Property::BedOccupied(BedOccupied::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::BedFacing(BedFacing::North)),
				"south" => x.properties.contains(&Property::BedFacing(BedFacing::South)),
				"west" => x.properties.contains(&Property::BedFacing(BedFacing::West)),
				"east" => x.properties.contains(&Property::BedFacing(BedFacing::East)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BellPowered(BellPowered::True)),
				"false" => x.properties.contains(&Property::BellPowered(BellPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::BellFacing(BellFacing::North)),
				"south" => x.properties.contains(&Property::BellFacing(BellFacing::South)),
				"west" => x.properties.contains(&Property::BellFacing(BellFacing::West)),
				"east" => x.properties.contains(&Property::BellFacing(BellFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "attachment").unwrap().1.as_str() {
				"floor" => x.properties.contains(&Property::BellAttachment(BellAttachment::Floor)),
				"ceiling" => x.properties.contains(&Property::BellAttachment(BellAttachment::Ceiling)),
				"single_wall" => x.properties.contains(&Property::BellAttachment(BellAttachment::SingleWall)),
				"double_wall" => x.properties.contains(&Property::BellAttachment(BellAttachment::DoubleWall)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::BigDripleaf => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BigDripleafWaterlogged(BigDripleafWaterlogged::True)),
				"false" => x.properties.contains(&Property::BigDripleafWaterlogged(BigDripleafWaterlogged::False)),
				_ => false,
				})
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::BlastFurnaceLit(BlastFurnaceLit::True)),
				"false" => x.properties.contains(&Property::BlastFurnaceLit(BlastFurnaceLit::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::BlastFurnaceFacing(BlastFurnaceFacing::North)),
				"south" => x.properties.contains(&Property::BlastFurnaceFacing(BlastFurnaceFacing::South)),
				"west" => x.properties.contains(&Property::BlastFurnaceFacing(BlastFurnaceFacing::West)),
				"east" => x.properties.contains(&Property::BlastFurnaceFacing(BlastFurnaceFacing::East)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::ButtonFacing(ButtonFacing::North)),
				"south" => x.properties.contains(&Property::ButtonFacing(ButtonFacing::South)),
				"west" => x.properties.contains(&Property::ButtonFacing(ButtonFacing::West)),
				"east" => x.properties.contains(&Property::ButtonFacing(ButtonFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "face").unwrap().1.as_str() {
				"floor" => x.properties.contains(&Property::ButtonFace(ButtonFace::Floor)),
				"wall" => x.properties.contains(&Property::ButtonFace(ButtonFace::Wall)),
				"ceiling" => x.properties.contains(&Property::ButtonFace(ButtonFace::Ceiling)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::North)),
				"south" => x.properties.contains(&Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::South)),
				"west" => x.properties.contains(&Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::West)),
				"east" => x.properties.contains(&Property::CalibratedSculkSensorFacing(CalibratedSculkSensorFacing::East)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CampfireWaterlogged(CampfireWaterlogged::True)),
				"false" => x.properties.contains(&Property::CampfireWaterlogged(CampfireWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "signal_fire").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CampfireSignalFire(CampfireSignalFire::True)),
				"false" => x.properties.contains(&Property::CampfireSignalFire(CampfireSignalFire::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CampfireLit(CampfireLit::True)),
				"false" => x.properties.contains(&Property::CampfireLit(CampfireLit::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::CampfireFacing(CampfireFacing::North)),
				"south" => x.properties.contains(&Property::CampfireFacing(CampfireFacing::South)),
				"west" => x.properties.contains(&Property::CampfireFacing(CampfireFacing::West)),
				"east" => x.properties.contains(&Property::CampfireFacing(CampfireFacing::East)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "attached").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CeilingHangingSignAttached(CeilingHangingSignAttached::True)),
				"false" => x.properties.contains(&Property::CeilingHangingSignAttached(CeilingHangingSignAttached::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChainWaterlogged(ChainWaterlogged::True)),
				"false" => x.properties.contains(&Property::ChainWaterlogged(ChainWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "axis").unwrap().1.as_str() {
				"x" => x.properties.contains(&Property::ChainAxis(ChainAxis::X)),
				"y" => x.properties.contains(&Property::ChainAxis(ChainAxis::Y)),
				"z" => x.properties.contains(&Property::ChainAxis(ChainAxis::Z)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChestWaterlogged(ChestWaterlogged::True)),
				"false" => x.properties.contains(&Property::ChestWaterlogged(ChestWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "type").unwrap().1.as_str() {
				"single" => x.properties.contains(&Property::ChestType(ChestType::Single)),
				"left" => x.properties.contains(&Property::ChestType(ChestType::Left)),
				"right" => x.properties.contains(&Property::ChestType(ChestType::Right)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::ChiseledBookShelf => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "slot_5_occupied").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChiseledBookShelfSlot5Occupied(ChiseledBookShelfSlot5Occupied::True)),
				"false" => x.properties.contains(&Property::ChiseledBookShelfSlot5Occupied(ChiseledBookShelfSlot5Occupied::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "slot_0_occupied").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChiseledBookShelfSlot0Occupied(ChiseledBookShelfSlot0Occupied::True)),
				"false" => x.properties.contains(&Property::ChiseledBookShelfSlot0Occupied(ChiseledBookShelfSlot0Occupied::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "slot_4_occupied").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChiseledBookShelfSlot4Occupied(ChiseledBookShelfSlot4Occupied::True)),
				"false" => x.properties.contains(&Property::ChiseledBookShelfSlot4Occupied(ChiseledBookShelfSlot4Occupied::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::North)),
				"south" => x.properties.contains(&Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::South)),
				"west" => x.properties.contains(&Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::West)),
				"east" => x.properties.contains(&Property::ChiseledBookShelfFacing(ChiseledBookShelfFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "slot_3_occupied").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChiseledBookShelfSlot3Occupied(ChiseledBookShelfSlot3Occupied::True)),
				"false" => x.properties.contains(&Property::ChiseledBookShelfSlot3Occupied(ChiseledBookShelfSlot3Occupied::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChorusPlantNorth(ChorusPlantNorth::True)),
				"false" => x.properties.contains(&Property::ChorusPlantNorth(ChorusPlantNorth::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChorusPlantEast(ChorusPlantEast::True)),
				"false" => x.properties.contains(&Property::ChorusPlantEast(ChorusPlantEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ChorusPlantSouth(ChorusPlantSouth::True)),
				"false" => x.properties.contains(&Property::ChorusPlantSouth(ChorusPlantSouth::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Cocoa => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::CocoaFacing(CocoaFacing::North)),
				"south" => x.properties.contains(&Property::CocoaFacing(CocoaFacing::South)),
				"west" => x.properties.contains(&Property::CocoaFacing(CocoaFacing::West)),
				"east" => x.properties.contains(&Property::CocoaFacing(CocoaFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::CocoaAge(CocoaAge::Num0)),
				"1" => x.properties.contains(&Property::CocoaAge(CocoaAge::Num1)),
				"2" => x.properties.contains(&Property::CocoaAge(CocoaAge::Num2)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ComparatorPowered(ComparatorPowered::True)),
				"false" => x.properties.contains(&Property::ComparatorPowered(ComparatorPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "mode").unwrap().1.as_str() {
				"compare" => x.properties.contains(&Property::ComparatorMode(ComparatorMode::Compare)),
				"subtract" => x.properties.contains(&Property::ComparatorMode(ComparatorMode::Subtract)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CopperChestWaterlogged(CopperChestWaterlogged::True)),
				"false" => x.properties.contains(&Property::CopperChestWaterlogged(CopperChestWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "type").unwrap().1.as_str() {
				"single" => x.properties.contains(&Property::CopperChestType(CopperChestType::Single)),
				"left" => x.properties.contains(&Property::CopperChestType(CopperChestType::Left)),
				"right" => x.properties.contains(&Property::CopperChestType(CopperChestType::Right)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::CopperChestFacing(CopperChestFacing::North)),
				"south" => x.properties.contains(&Property::CopperChestFacing(CopperChestFacing::South)),
				"west" => x.properties.contains(&Property::CopperChestFacing(CopperChestFacing::West)),
				"east" => x.properties.contains(&Property::CopperChestFacing(CopperChestFacing::East)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "triggered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CrafterTriggered(CrafterTriggered::True)),
				"false" => x.properties.contains(&Property::CrafterTriggered(CrafterTriggered::False)),
				_ => false,
				})
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "natural").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::CreakingHeartNatural(CreakingHeartNatural::True)),
				"false" => x.properties.contains(&Property::CreakingHeartNatural(CreakingHeartNatural::False)),
				_ => false,
				})
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "inverted").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DaylightDetectorInverted(DaylightDetectorInverted::True)),
				"false" => x.properties.contains(&Property::DaylightDetectorInverted(DaylightDetectorInverted::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::DecoratedPot => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DecoratedPotWaterlogged(DecoratedPotWaterlogged::True)),
				"false" => x.properties.contains(&Property::DecoratedPotWaterlogged(DecoratedPotWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::DecoratedPotFacing(DecoratedPotFacing::North)),
				"south" => x.properties.contains(&Property::DecoratedPotFacing(DecoratedPotFacing::South)),
				"west" => x.properties.contains(&Property::DecoratedPotFacing(DecoratedPotFacing::West)),
				"east" => x.properties.contains(&Property::DecoratedPotFacing(DecoratedPotFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "cracked").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DecoratedPotCracked(DecoratedPotCracked::True)),
				"false" => x.properties.contains(&Property::DecoratedPotCracked(DecoratedPotCracked::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::DetectorRail => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DetectorRailWaterlogged(DetectorRailWaterlogged::True)),
				"false" => x.properties.contains(&Property::DetectorRailWaterlogged(DetectorRailWaterlogged::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DetectorRailPowered(DetectorRailPowered::True)),
				"false" => x.properties.contains(&Property::DetectorRailPowered(DetectorRailPowered::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "triggered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DispenserTriggered(DispenserTriggered::True)),
				"false" => x.properties.contains(&Property::DispenserTriggered(DispenserTriggered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::DispenserFacing(DispenserFacing::North)),
				"east" => x.properties.contains(&Property::DispenserFacing(DispenserFacing::East)),
				"south" => x.properties.contains(&Property::DispenserFacing(DispenserFacing::South)),
				"west" => x.properties.contains(&Property::DispenserFacing(DispenserFacing::West)),
				"up" => x.properties.contains(&Property::DispenserFacing(DispenserFacing::Up)),
				"down" => x.properties.contains(&Property::DispenserFacing(DispenserFacing::Down)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Door => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "open").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DoorOpen(DoorOpen::True)),
				"false" => x.properties.contains(&Property::DoorOpen(DoorOpen::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"upper" => x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)),
				"lower" => x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DoorPowered(DoorPowered::True)),
				"false" => x.properties.contains(&Property::DoorPowered(DoorPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::DoorFacing(DoorFacing::North)),
				"south" => x.properties.contains(&Property::DoorFacing(DoorFacing::South)),
				"west" => x.properties.contains(&Property::DoorFacing(DoorFacing::West)),
				"east" => x.properties.contains(&Property::DoorFacing(DoorFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hinge").unwrap().1.as_str() {
				"left" => x.properties.contains(&Property::DoorHinge(DoorHinge::Left)),
				"right" => x.properties.contains(&Property::DoorHinge(DoorHinge::Right)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::DriedGhastWaterlogged(DriedGhastWaterlogged::True)),
				"false" => x.properties.contains(&Property::DriedGhastWaterlogged(DriedGhastWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hydration").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::DriedGhastHydration(DriedGhastHydration::Num0)),
				"1" => x.properties.contains(&Property::DriedGhastHydration(DriedGhastHydration::Num1)),
				"2" => x.properties.contains(&Property::DriedGhastHydration(DriedGhastHydration::Num2)),
				"3" => x.properties.contains(&Property::DriedGhastHydration(DriedGhastHydration::Num3)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::EndPortalFrameFacing(EndPortalFrameFacing::North)),
				"south" => x.properties.contains(&Property::EndPortalFrameFacing(EndPortalFrameFacing::South)),
				"west" => x.properties.contains(&Property::EndPortalFrameFacing(EndPortalFrameFacing::West)),
				"east" => x.properties.contains(&Property::EndPortalFrameFacing(EndPortalFrameFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "eye").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::EndPortalFrameEye(EndPortalFrameEye::True)),
				"false" => x.properties.contains(&Property::EndPortalFrameEye(EndPortalFrameEye::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::EnderChestWaterlogged(EnderChestWaterlogged::True)),
				"false" => x.properties.contains(&Property::EnderChestWaterlogged(EnderChestWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::EnderChestFacing(EnderChestFacing::North)),
				"south" => x.properties.contains(&Property::EnderChestFacing(EnderChestFacing::South)),
				"west" => x.properties.contains(&Property::EnderChestFacing(EnderChestFacing::West)),
				"east" => x.properties.contains(&Property::EnderChestFacing(EnderChestFacing::East)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FenceWest(FenceWest::True)),
				"false" => x.properties.contains(&Property::FenceWest(FenceWest::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FenceEast(FenceEast::True)),
				"false" => x.properties.contains(&Property::FenceEast(FenceEast::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FenceNorth(FenceNorth::True)),
				"false" => x.properties.contains(&Property::FenceNorth(FenceNorth::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::True)),
				"false" => x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "open").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::True)),
				"false" => x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "in_wall").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::True)),
				"false" => x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Fire => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FireNorth(FireNorth::True)),
				"false" => x.properties.contains(&Property::FireNorth(FireNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FireEast(FireEast::True)),
				"false" => x.properties.contains(&Property::FireEast(FireEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FireSouth(FireSouth::True)),
				"false" => x.properties.contains(&Property::FireSouth(FireSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FireWest(FireWest::True)),
				"false" => x.properties.contains(&Property::FireWest(FireWest::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "up").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::FireUp(FireUp::True)),
				"false" => x.properties.contains(&Property::FireUp(FireUp::False)),
				_ => false,
				})
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::GlowLichenWaterlogged(GlowLichenWaterlogged::True)),
				"false" => x.properties.contains(&Property::GlowLichenWaterlogged(GlowLichenWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "up").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::GlowLichenUp(GlowLichenUp::True)),
				"false" => x.properties.contains(&Property::GlowLichenUp(GlowLichenUp::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::GlowLichenWest(GlowLichenWest::True)),
				"false" => x.properties.contains(&Property::GlowLichenWest(GlowLichenWest::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"down" => x.properties.contains(&Property::HopperFacing(HopperFacing::Down)),
				"north" => x.properties.contains(&Property::HopperFacing(HopperFacing::North)),
				"south" => x.properties.contains(&Property::HopperFacing(HopperFacing::South)),
				"west" => x.properties.contains(&Property::HopperFacing(HopperFacing::West)),
				"east" => x.properties.contains(&Property::HopperFacing(HopperFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "enabled").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HopperEnabled(HopperEnabled::True)),
				"false" => x.properties.contains(&Property::HopperEnabled(HopperEnabled::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::HugeMushroom => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HugeMushroomNorth(HugeMushroomNorth::True)),
				"false" => x.properties.contains(&Property::HugeMushroomNorth(HugeMushroomNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "up").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HugeMushroomUp(HugeMushroomUp::True)),
				"false" => x.properties.contains(&Property::HugeMushroomUp(HugeMushroomUp::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HugeMushroomSouth(HugeMushroomSouth::True)),
				"false" => x.properties.contains(&Property::HugeMushroomSouth(HugeMushroomSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "down").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HugeMushroomDown(HugeMushroomDown::True)),
				"false" => x.properties.contains(&Property::HugeMushroomDown(HugeMushroomDown::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HugeMushroomWest(HugeMushroomWest::True)),
				"false" => x.properties.contains(&Property::HugeMushroomWest(HugeMushroomWest::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::HugeMushroomEast(HugeMushroomEast::True)),
				"false" => x.properties.contains(&Property::HugeMushroomEast(HugeMushroomEast::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::IronBarsSouth(IronBarsSouth::True)),
				"false" => x.properties.contains(&Property::IronBarsSouth(IronBarsSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::IronBarsWest(IronBarsWest::True)),
				"false" => x.properties.contains(&Property::IronBarsWest(IronBarsWest::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::IronBarsEast(IronBarsEast::True)),
				"false" => x.properties.contains(&Property::IronBarsEast(IronBarsEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::IronBarsWaterlogged(IronBarsWaterlogged::True)),
				"false" => x.properties.contains(&Property::IronBarsWaterlogged(IronBarsWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::IronBarsNorth(IronBarsNorth::True)),
				"false" => x.properties.contains(&Property::IronBarsNorth(IronBarsNorth::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::LanternWaterlogged(LanternWaterlogged::True)),
				"false" => x.properties.contains(&Property::LanternWaterlogged(LanternWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hanging").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::LanternHanging(LanternHanging::True)),
				"false" => x.properties.contains(&Property::LanternHanging(LanternHanging::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::LecternFacing(LecternFacing::North)),
				"south" => x.properties.contains(&Property::LecternFacing(LecternFacing::South)),
				"west" => x.properties.contains(&Property::LecternFacing(LecternFacing::West)),
				"east" => x.properties.contains(&Property::LecternFacing(LecternFacing::East)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::LeverPowered(LeverPowered::True)),
				"false" => x.properties.contains(&Property::LeverPowered(LeverPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::LeverFacing(LeverFacing::North)),
				"south" => x.properties.contains(&Property::LeverFacing(LeverFacing::South)),
				"west" => x.properties.contains(&Property::LeverFacing(LeverFacing::West)),
				"east" => x.properties.contains(&Property::LeverFacing(LeverFacing::East)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::LightningRodPowered(LightningRodPowered::True)),
				"false" => x.properties.contains(&Property::LightningRodPowered(LightningRodPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::LightningRodFacing(LightningRodFacing::North)),
				"east" => x.properties.contains(&Property::LightningRodFacing(LightningRodFacing::East)),
				"south" => x.properties.contains(&Property::LightningRodFacing(LightningRodFacing::South)),
				"west" => x.properties.contains(&Property::LightningRodFacing(LightningRodFacing::West)),
				"up" => x.properties.contains(&Property::LightningRodFacing(LightningRodFacing::Up)),
				"down" => x.properties.contains(&Property::LightningRodFacing(LightningRodFacing::Down)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MangroveLeavesWaterlogged(MangroveLeavesWaterlogged::True)),
				"false" => x.properties.contains(&Property::MangroveLeavesWaterlogged(MangroveLeavesWaterlogged::False)),
				_ => false,
				})
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
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::MangrovePropagule => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MangrovePropaguleWaterlogged(MangrovePropaguleWaterlogged::True)),
				"false" => x.properties.contains(&Property::MangrovePropaguleWaterlogged(MangrovePropaguleWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "age").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::MangrovePropaguleAge(MangrovePropaguleAge::Num0)),
				"1" => x.properties.contains(&Property::MangrovePropaguleAge(MangrovePropaguleAge::Num1)),
				"2" => x.properties.contains(&Property::MangrovePropaguleAge(MangrovePropaguleAge::Num2)),
				"3" => x.properties.contains(&Property::MangrovePropaguleAge(MangrovePropaguleAge::Num3)),
				"4" => x.properties.contains(&Property::MangrovePropaguleAge(MangrovePropaguleAge::Num4)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "stage").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::MangrovePropaguleStage(MangrovePropaguleStage::Num0)),
				"1" => x.properties.contains(&Property::MangrovePropaguleStage(MangrovePropaguleStage::Num1)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hanging").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MangrovePropaguleHanging(MangrovePropaguleHanging::True)),
				"false" => x.properties.contains(&Property::MangrovePropaguleHanging(MangrovePropaguleHanging::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::MossyCarpetEast(MossyCarpetEast::None)),
				"low" => x.properties.contains(&Property::MossyCarpetEast(MossyCarpetEast::Low)),
				"tall" => x.properties.contains(&Property::MossyCarpetEast(MossyCarpetEast::Tall)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::MossyCarpetSouth(MossyCarpetSouth::None)),
				"low" => x.properties.contains(&Property::MossyCarpetSouth(MossyCarpetSouth::Low)),
				"tall" => x.properties.contains(&Property::MossyCarpetSouth(MossyCarpetSouth::Tall)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "bottom").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MossyCarpetBottom(MossyCarpetBottom::True)),
				"false" => x.properties.contains(&Property::MossyCarpetBottom(MossyCarpetBottom::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::MossyCarpetWest(MossyCarpetWest::None)),
				"low" => x.properties.contains(&Property::MossyCarpetWest(MossyCarpetWest::Low)),
				"tall" => x.properties.contains(&Property::MossyCarpetWest(MossyCarpetWest::Tall)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::MossyCarpetNorth(MossyCarpetNorth::None)),
				"low" => x.properties.contains(&Property::MossyCarpetNorth(MossyCarpetNorth::Low)),
				"tall" => x.properties.contains(&Property::MossyCarpetNorth(MossyCarpetNorth::Tall)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::MovingPiston => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "type").unwrap().1.as_str() {
				"normal" => x.properties.contains(&Property::MovingPistonType(MovingPistonType::Normal)),
				"sticky" => x.properties.contains(&Property::MovingPistonType(MovingPistonType::Sticky)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::MovingPistonFacing(MovingPistonFacing::North)),
				"east" => x.properties.contains(&Property::MovingPistonFacing(MovingPistonFacing::East)),
				"south" => x.properties.contains(&Property::MovingPistonFacing(MovingPistonFacing::South)),
				"west" => x.properties.contains(&Property::MovingPistonFacing(MovingPistonFacing::West)),
				"up" => x.properties.contains(&Property::MovingPistonFacing(MovingPistonFacing::Up)),
				"down" => x.properties.contains(&Property::MovingPistonFacing(MovingPistonFacing::Down)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MultifaceWaterlogged(MultifaceWaterlogged::True)),
				"false" => x.properties.contains(&Property::MultifaceWaterlogged(MultifaceWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MultifaceEast(MultifaceEast::True)),
				"false" => x.properties.contains(&Property::MultifaceEast(MultifaceEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "down").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MultifaceDown(MultifaceDown::True)),
				"false" => x.properties.contains(&Property::MultifaceDown(MultifaceDown::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "up").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MultifaceUp(MultifaceUp::True)),
				"false" => x.properties.contains(&Property::MultifaceUp(MultifaceUp::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MultifaceNorth(MultifaceNorth::True)),
				"false" => x.properties.contains(&Property::MultifaceNorth(MultifaceNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MultifaceWest(MultifaceWest::True)),
				"false" => x.properties.contains(&Property::MultifaceWest(MultifaceWest::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::MultifaceSouth(MultifaceSouth::True)),
				"false" => x.properties.contains(&Property::MultifaceSouth(MultifaceSouth::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::NotePowered(NotePowered::True)),
				"false" => x.properties.contains(&Property::NotePowered(NotePowered::False)),
				_ => false,
				})
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ObserverPowered(ObserverPowered::True)),
				"false" => x.properties.contains(&Property::ObserverPowered(ObserverPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::ObserverFacing(ObserverFacing::North)),
				"east" => x.properties.contains(&Property::ObserverFacing(ObserverFacing::East)),
				"south" => x.properties.contains(&Property::ObserverFacing(ObserverFacing::South)),
				"west" => x.properties.contains(&Property::ObserverFacing(ObserverFacing::West)),
				"up" => x.properties.contains(&Property::ObserverFacing(ObserverFacing::Up)),
				"down" => x.properties.contains(&Property::ObserverFacing(ObserverFacing::Down)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::PlayerHeadPowered(PlayerHeadPowered::True)),
				"false" => x.properties.contains(&Property::PlayerHeadPowered(PlayerHeadPowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::PlayerWallHead => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::PlayerWallHeadPowered(PlayerWallHeadPowered::True)),
				"false" => x.properties.contains(&Property::PlayerWallHeadPowered(PlayerWallHeadPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::PlayerWallHeadFacing(PlayerWallHeadFacing::North)),
				"south" => x.properties.contains(&Property::PlayerWallHeadFacing(PlayerWallHeadFacing::South)),
				"west" => x.properties.contains(&Property::PlayerWallHeadFacing(PlayerWallHeadFacing::West)),
				"east" => x.properties.contains(&Property::PlayerWallHeadFacing(PlayerWallHeadFacing::East)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "shape").unwrap().1.as_str() {
				"north_south" => x.properties.contains(&Property::PoweredRailShape(PoweredRailShape::NorthSouth)),
				"east_west" => x.properties.contains(&Property::PoweredRailShape(PoweredRailShape::EastWest)),
				"ascending_east" => x.properties.contains(&Property::PoweredRailShape(PoweredRailShape::AscendingEast)),
				"ascending_west" => x.properties.contains(&Property::PoweredRailShape(PoweredRailShape::AscendingWest)),
				"ascending_north" => x.properties.contains(&Property::PoweredRailShape(PoweredRailShape::AscendingNorth)),
				"ascending_south" => x.properties.contains(&Property::PoweredRailShape(PoweredRailShape::AscendingSouth)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::PoweredRailPowered(PoweredRailPowered::True)),
				"false" => x.properties.contains(&Property::PoweredRailPowered(PoweredRailPowered::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::RailWaterlogged(RailWaterlogged::True)),
				"false" => x.properties.contains(&Property::RailWaterlogged(RailWaterlogged::False)),
				_ => false,
				})
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::RedstoneWallTorchLit(RedstoneWallTorchLit::True)),
				"false" => x.properties.contains(&Property::RedstoneWallTorchLit(RedstoneWallTorchLit::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::North)),
				"south" => x.properties.contains(&Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::South)),
				"west" => x.properties.contains(&Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::West)),
				"east" => x.properties.contains(&Property::RedstoneWallTorchFacing(RedstoneWallTorchFacing::East)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::RedstoneWire => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"up" => x.properties.contains(&Property::RedstoneWireWest(RedstoneWireWest::Up)),
				"side" => x.properties.contains(&Property::RedstoneWireWest(RedstoneWireWest::Side)),
				"none" => x.properties.contains(&Property::RedstoneWireWest(RedstoneWireWest::None)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"up" => x.properties.contains(&Property::RedstoneWireNorth(RedstoneWireNorth::Up)),
				"side" => x.properties.contains(&Property::RedstoneWireNorth(RedstoneWireNorth::Side)),
				"none" => x.properties.contains(&Property::RedstoneWireNorth(RedstoneWireNorth::None)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"up" => x.properties.contains(&Property::RedstoneWireSouth(RedstoneWireSouth::Up)),
				"side" => x.properties.contains(&Property::RedstoneWireSouth(RedstoneWireSouth::Side)),
				"none" => x.properties.contains(&Property::RedstoneWireSouth(RedstoneWireSouth::None)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"up" => x.properties.contains(&Property::RedstoneWireEast(RedstoneWireEast::Up)),
				"side" => x.properties.contains(&Property::RedstoneWireEast(RedstoneWireEast::Side)),
				"none" => x.properties.contains(&Property::RedstoneWireEast(RedstoneWireEast::None)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Repeater => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::RepeaterPowered(RepeaterPowered::True)),
				"false" => x.properties.contains(&Property::RepeaterPowered(RepeaterPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "delay").unwrap().1.as_str() {
				"1" => x.properties.contains(&Property::RepeaterDelay(RepeaterDelay::Num1)),
				"2" => x.properties.contains(&Property::RepeaterDelay(RepeaterDelay::Num2)),
				"3" => x.properties.contains(&Property::RepeaterDelay(RepeaterDelay::Num3)),
				"4" => x.properties.contains(&Property::RepeaterDelay(RepeaterDelay::Num4)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "locked").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::RepeaterLocked(RepeaterLocked::True)),
				"false" => x.properties.contains(&Property::RepeaterLocked(RepeaterLocked::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::RepeaterFacing(RepeaterFacing::North)),
				"south" => x.properties.contains(&Property::RepeaterFacing(RepeaterFacing::South)),
				"west" => x.properties.contains(&Property::RepeaterFacing(RepeaterFacing::West)),
				"east" => x.properties.contains(&Property::RepeaterFacing(RepeaterFacing::East)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "bottom").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ScaffoldingBottom(ScaffoldingBottom::True)),
				"false" => x.properties.contains(&Property::ScaffoldingBottom(ScaffoldingBottom::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "sculk_sensor_phase").unwrap().1.as_str() {
				"inactive" => x.properties.contains(&Property::SculkSensorSculkSensorPhase(SculkSensorSculkSensorPhase::Inactive)),
				"active" => x.properties.contains(&Property::SculkSensorSculkSensorPhase(SculkSensorSculkSensorPhase::Active)),
				"cooldown" => x.properties.contains(&Property::SculkSensorSculkSensorPhase(SculkSensorSculkSensorPhase::Cooldown)),
				_ => false,
				})
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkShriekerWaterlogged(SculkShriekerWaterlogged::True)),
				"false" => x.properties.contains(&Property::SculkShriekerWaterlogged(SculkShriekerWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "shrieking").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkShriekerShrieking(SculkShriekerShrieking::True)),
				"false" => x.properties.contains(&Property::SculkShriekerShrieking(SculkShriekerShrieking::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::SculkVein => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkVeinWest(SculkVeinWest::True)),
				"false" => x.properties.contains(&Property::SculkVeinWest(SculkVeinWest::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "down").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkVeinDown(SculkVeinDown::True)),
				"false" => x.properties.contains(&Property::SculkVeinDown(SculkVeinDown::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "up").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkVeinUp(SculkVeinUp::True)),
				"false" => x.properties.contains(&Property::SculkVeinUp(SculkVeinUp::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkVeinNorth(SculkVeinNorth::True)),
				"false" => x.properties.contains(&Property::SculkVeinNorth(SculkVeinNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkVeinWaterlogged(SculkVeinWaterlogged::True)),
				"false" => x.properties.contains(&Property::SculkVeinWaterlogged(SculkVeinWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkVeinSouth(SculkVeinSouth::True)),
				"false" => x.properties.contains(&Property::SculkVeinSouth(SculkVeinSouth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SculkVeinEast(SculkVeinEast::True)),
				"false" => x.properties.contains(&Property::SculkVeinEast(SculkVeinEast::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::ShelfPowered(ShelfPowered::True)),
				"false" => x.properties.contains(&Property::ShelfPowered(ShelfPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::ShelfFacing(ShelfFacing::North)),
				"south" => x.properties.contains(&Property::ShelfFacing(ShelfFacing::South)),
				"west" => x.properties.contains(&Property::ShelfFacing(ShelfFacing::West)),
				"east" => x.properties.contains(&Property::ShelfFacing(ShelfFacing::East)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SkullPowered(SkullPowered::True)),
				"false" => x.properties.contains(&Property::SkullPowered(SkullPowered::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SmallDripleafWaterlogged(SmallDripleafWaterlogged::True)),
				"false" => x.properties.contains(&Property::SmallDripleafWaterlogged(SmallDripleafWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"upper" => x.properties.contains(&Property::SmallDripleafHalf(SmallDripleafHalf::Upper)),
				"lower" => x.properties.contains(&Property::SmallDripleafHalf(SmallDripleafHalf::Lower)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::SmokerLit(SmokerLit::True)),
				"false" => x.properties.contains(&Property::SmokerLit(SmokerLit::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::SmokerFacing(SmokerFacing::North)),
				"south" => x.properties.contains(&Property::SmokerFacing(SmokerFacing::South)),
				"west" => x.properties.contains(&Property::SmokerFacing(SmokerFacing::West)),
				"east" => x.properties.contains(&Property::SmokerFacing(SmokerFacing::East)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::StainedGlassPaneWest(StainedGlassPaneWest::True)),
				"false" => x.properties.contains(&Property::StainedGlassPaneWest(StainedGlassPaneWest::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::StainedGlassPaneSouth(StainedGlassPaneSouth::True)),
				"false" => x.properties.contains(&Property::StainedGlassPaneSouth(StainedGlassPaneSouth::False)),
				_ => false,
				})
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::StainedGlassPaneWaterlogged(StainedGlassPaneWaterlogged::True)),
				"false" => x.properties.contains(&Property::StainedGlassPaneWaterlogged(StainedGlassPaneWaterlogged::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Stair => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::True)),
				"false" => x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TrapdoorWaterlogged(TrapdoorWaterlogged::True)),
				"false" => x.properties.contains(&Property::TrapdoorWaterlogged(TrapdoorWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TrapdoorPowered(TrapdoorPowered::True)),
				"false" => x.properties.contains(&Property::TrapdoorPowered(TrapdoorPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"top" => x.properties.contains(&Property::TrapdoorHalf(TrapdoorHalf::Top)),
				"bottom" => x.properties.contains(&Property::TrapdoorHalf(TrapdoorHalf::Bottom)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::TrapdoorFacing(TrapdoorFacing::North)),
				"south" => x.properties.contains(&Property::TrapdoorFacing(TrapdoorFacing::South)),
				"west" => x.properties.contains(&Property::TrapdoorFacing(TrapdoorFacing::West)),
				"east" => x.properties.contains(&Property::TrapdoorFacing(TrapdoorFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "open").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TrapdoorOpen(TrapdoorOpen::True)),
				"false" => x.properties.contains(&Property::TrapdoorOpen(TrapdoorOpen::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TrappedChestWaterlogged(TrappedChestWaterlogged::True)),
				"false" => x.properties.contains(&Property::TrappedChestWaterlogged(TrappedChestWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "type").unwrap().1.as_str() {
				"single" => x.properties.contains(&Property::TrappedChestType(TrappedChestType::Single)),
				"left" => x.properties.contains(&Property::TrappedChestType(TrappedChestType::Left)),
				"right" => x.properties.contains(&Property::TrappedChestType(TrappedChestType::Right)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "attached").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripWireHookAttached(TripWireHookAttached::True)),
				"false" => x.properties.contains(&Property::TripWireHookAttached(TripWireHookAttached::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Tripwire => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripwireSouth(TripwireSouth::True)),
				"false" => x.properties.contains(&Property::TripwireSouth(TripwireSouth::False)),
				_ => false,
				})
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripwireNorth(TripwireNorth::True)),
				"false" => x.properties.contains(&Property::TripwireNorth(TripwireNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "west").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripwireWest(TripwireWest::True)),
				"false" => x.properties.contains(&Property::TripwireWest(TripwireWest::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripwireEast(TripwireEast::True)),
				"false" => x.properties.contains(&Property::TripwireEast(TripwireEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::TripwirePowered(TripwirePowered::True)),
				"false" => x.properties.contains(&Property::TripwirePowered(TripwirePowered::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::TurtleEgg => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hatch").unwrap().1.as_str() {
				"0" => x.properties.contains(&Property::TurtleEggHatch(TurtleEggHatch::Num0)),
				"1" => x.properties.contains(&Property::TurtleEggHatch(TurtleEggHatch::Num1)),
				"2" => x.properties.contains(&Property::TurtleEggHatch(TurtleEggHatch::Num2)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "eggs").unwrap().1.as_str() {
				"1" => x.properties.contains(&Property::TurtleEggEggs(TurtleEggEggs::Num1)),
				"2" => x.properties.contains(&Property::TurtleEggEggs(TurtleEggEggs::Num2)),
				"3" => x.properties.contains(&Property::TurtleEggEggs(TurtleEggEggs::Num3)),
				"4" => x.properties.contains(&Property::TurtleEggEggs(TurtleEggEggs::Num4)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "persistent").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::UntintedParticleLeavesPersistent(UntintedParticleLeavesPersistent::True)),
				"false" => x.properties.contains(&Property::UntintedParticleLeavesPersistent(UntintedParticleLeavesPersistent::False)),
				_ => false,
				})
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "vault_state").unwrap().1.as_str() {
				"inactive" => x.properties.contains(&Property::VaultVaultState(VaultVaultState::Inactive)),
				"active" => x.properties.contains(&Property::VaultVaultState(VaultVaultState::Active)),
				"unlocking" => x.properties.contains(&Property::VaultVaultState(VaultVaultState::Unlocking)),
				"ejecting" => x.properties.contains(&Property::VaultVaultState(VaultVaultState::Ejecting)),
				_ => false,
				})
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
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::Vine => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "north").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::VineNorth(VineNorth::True)),
				"false" => x.properties.contains(&Property::VineNorth(VineNorth::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "up").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::VineUp(VineUp::True)),
				"false" => x.properties.contains(&Property::VineUp(VineUp::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::VineEast(VineEast::True)),
				"false" => x.properties.contains(&Property::VineEast(VineEast::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::VineSouth(VineSouth::True)),
				"false" => x.properties.contains(&Property::VineSouth(VineSouth::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "south").unwrap().1.as_str() {
				"none" => x.properties.contains(&Property::WallSouth(WallSouth::None)),
				"low" => x.properties.contains(&Property::WallSouth(WallSouth::Low)),
				"tall" => x.properties.contains(&Property::WallSouth(WallSouth::Tall)),
				_ => false,
				})
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WallHangingSignWaterlogged(WallHangingSignWaterlogged::True)),
				"false" => x.properties.contains(&Property::WallHangingSignWaterlogged(WallHangingSignWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WallHangingSignFacing(WallHangingSignFacing::North)),
				"south" => x.properties.contains(&Property::WallHangingSignFacing(WallHangingSignFacing::South)),
				"west" => x.properties.contains(&Property::WallHangingSignFacing(WallHangingSignFacing::West)),
				"east" => x.properties.contains(&Property::WallHangingSignFacing(WallHangingSignFacing::East)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "east").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperBarEast(WeatheringCopperBarEast::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperBarEast(WeatheringCopperBarEast::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperBulbPowered(WeatheringCopperBulbPowered::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperBulbPowered(WeatheringCopperBulbPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "lit").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperBulbLit(WeatheringCopperBulbLit::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperBulbLit(WeatheringCopperBulbLit::False)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperChestWaterlogged(WeatheringCopperChestWaterlogged::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperChestWaterlogged(WeatheringCopperChestWaterlogged::False)),
				_ => false,
				})
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
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringCopperDoor => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "open").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperDoorOpen(WeatheringCopperDoorOpen::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperDoorOpen(WeatheringCopperDoorOpen::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hinge").unwrap().1.as_str() {
				"left" => x.properties.contains(&Property::WeatheringCopperDoorHinge(WeatheringCopperDoorHinge::Left)),
				"right" => x.properties.contains(&Property::WeatheringCopperDoorHinge(WeatheringCopperDoorHinge::Right)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"upper" => x.properties.contains(&Property::WeatheringCopperDoorHalf(WeatheringCopperDoorHalf::Upper)),
				"lower" => x.properties.contains(&Property::WeatheringCopperDoorHalf(WeatheringCopperDoorHalf::Lower)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::North)),
				"south" => x.properties.contains(&Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::South)),
				"west" => x.properties.contains(&Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::West)),
				"east" => x.properties.contains(&Property::WeatheringCopperDoorFacing(WeatheringCopperDoorFacing::East)),
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "copper_golem_pose").unwrap().1.as_str() {
				"standing" => x.properties.contains(&Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Standing)),
				"sitting" => x.properties.contains(&Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Sitting)),
				"running" => x.properties.contains(&Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Running)),
				"star" => x.properties.contains(&Property::WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose::Star)),
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
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringCopperTrapDoor => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "half").unwrap().1.as_str() {
				"top" => x.properties.contains(&Property::WeatheringCopperTrapDoorHalf(WeatheringCopperTrapDoorHalf::Top)),
				"bottom" => x.properties.contains(&Property::WeatheringCopperTrapDoorHalf(WeatheringCopperTrapDoorHalf::Bottom)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperTrapDoorPowered(WeatheringCopperTrapDoorPowered::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperTrapDoorPowered(WeatheringCopperTrapDoorPowered::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "facing").unwrap().1.as_str() {
				"north" => x.properties.contains(&Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::North)),
				"south" => x.properties.contains(&Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::South)),
				"west" => x.properties.contains(&Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::West)),
				"east" => x.properties.contains(&Property::WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing::East)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperTrapDoorWaterlogged(WeatheringCopperTrapDoorWaterlogged::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperTrapDoorWaterlogged(WeatheringCopperTrapDoorWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "open").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringCopperTrapDoorOpen(WeatheringCopperTrapDoorOpen::True)),
				"false" => x.properties.contains(&Property::WeatheringCopperTrapDoorOpen(WeatheringCopperTrapDoorOpen::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringLantern => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringLanternWaterlogged(WeatheringLanternWaterlogged::True)),
				"false" => x.properties.contains(&Property::WeatheringLanternWaterlogged(WeatheringLanternWaterlogged::False)),
				_ => false,
				})
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "hanging").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringLanternHanging(WeatheringLanternHanging::True)),
				"false" => x.properties.contains(&Property::WeatheringLanternHanging(WeatheringLanternHanging::False)),
				_ => false,
				})
				.collect();
			let block_state_id = block_states.first().unwrap().id;
			assert_eq!(block_states.len(), 1);
			block_state_id
		},
		Type::WeatheringLightningRod => {
			let block_states: Vec<&State> = block.states.iter()
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "waterlogged").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WeatheringLightningRodWaterlogged(WeatheringLightningRodWaterlogged::True)),
				"false" => x.properties.contains(&Property::WeatheringLightningRodWaterlogged(WeatheringLightningRodWaterlogged::False)),
				_ => false,
				})
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
			.filter(|x| match properties.iter().find(|y| y.0.as_str() == "powered").unwrap().1.as_str() {
				"true" => x.properties.contains(&Property::WitherSkullPowered(WitherSkullPowered::True)),
				"false" => x.properties.contains(&Property::WitherSkullPowered(WitherSkullPowered::False)),
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
