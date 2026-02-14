#![allow(clippy::needless_return)]
#[derive(Debug, Clone)]
pub struct Block {
	pub block_type: Type,
	pub states: Vec<State>,
	pub default_state: usize,
	pub properties: Vec<Property>,
	pub block_name: &'static str,
}

#[derive(Debug, Clone)]
pub struct State {
	pub id: u16,
	pub properties: Vec<Property>,
}
impl Type {
	#[allow(clippy::match_like_matches_macro)]
	pub fn has_right_click_behavior(&self) -> bool {
		return match self {
			Type::Anvil => true,
			Type::Barrel => true,
			Type::Beacon => true,
			Type::Bed => true,
			Type::Beehive => true,
			Type::Bell => true,
			Type::BlastFurnace => true,
			Type::BrewingStand => true,
			Type::Button => true,
			Type::Cake => true,
			Type::CalibratedSculkSensor => true,
			Type::Campfire => true,
			Type::Candle => true,
			Type::CandleCake => true,
			Type::Carrot => true,
			Type::CartographyTable => true,
			Type::Cauldron => true,
			Type::CeilingHangingSign => true,
			Type::Chest => true,
			Type::Comparator => true,
			Type::Composter => true,
			Type::Crafter => true,
			Type::CraftingTable => true,
			Type::Dispenser => true,
			Type::Door => true,
			Type::DragonEgg => true,
			Type::Dropper => true,
			Type::EnchantmentTable => true,
			Type::EndGateway => true,
			Type::EndPortal => true,
			Type::EndPortalFrame => true,
			Type::EnderChest => true,
			Type::FenceGate => true,
			Type::FlowerPot => true,
			Type::Furnace => true,
			Type::Grindstone => true,
			Type::Hopper => true,
			Type::Jigsaw => true,
			Type::Jukebox => true,
			Type::LavaCauldron => true,
			Type::LayeredCauldron => true,
			Type::Lever => true,
			Type::Loom => true,
			Type::NetherPortal => true,
			Type::Repeater => true,
			Type::SmithingTable => true,
			Type::Smoker => true,
			Type::StandingSign => true,
			Type::Stonecutter => true,
			Type::Trapdoor => true,
			Type::TrappedChest => true,
			Type::WallSign => true,
			Type::Lectern => true,
			Type::ShulkerBox => true,
			_ => false,
		}
	}

	#[allow(clippy::match_like_matches_macro)]
	pub fn is_solid(&self) -> bool {
		return match self {
			Type::Air => false,
			Type::SugarCane => false,
			Type::Liquid => false,
			Type::BubbleColumn => false,
			Type::KelpPlant => false,
			Type::CoralPlant => false,
			Type::DoublePlant => false,
			Type::BaseCoralPlant => false,
			Type::CaveVinesPlant => false,
			Type::WeepingVines => false,
			Type::WeepingVinesPlant => false,
			Type::TwistingVinesPlant => false,
			Type::Sapling => false,
			Type::BambooSapling => false,
			Type::Mushroom => false,
			Type::TallGrass => false,
			Type::TallDryGrass => false,
			Type::ShortDryGrass => false,
			Type::DryVegetation => false,
			Type::Fire => false,
			Type::SoulFire => false,
			Type::WallBanner => false,
			Type::WallSign => false,
			Type::StandingSign => false,
			Type::Torch => false,
			Type::TorchflowerCrop => false,
			Type::WallTorch => false,
			Type::RedstoneTorch => false,
			Type::RedstoneWallTorch => false,
			Type::PressurePlate => false,
			Type::WeightedPressurePlate => false,
			Type::Light => false,
			Type::Lever => false,
			_ => true,
		}
	}
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChiseledBookShelfSlot4Occupied { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfestedRotatedPillarAxis { 	X, 	Y, 	Z, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JukeboxHasRecord { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FireNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChiseledBookShelfSlot0Occupied { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IronBarsWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrappedChestWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperChestFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChorusPlantEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BellPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperTrapDoorFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PistonHeadType { 	Normal, 	Sticky, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RotatedPillarAxis { 	X, 	Y, 	Z, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BarrierWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoralWallFanWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlowLichenDown { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkShriekerShrieking { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultVaultState { 	Inactive, 	Active, 	Unlocking, 	Ejecting, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkSensorSculkSensorPhase { 	Inactive, 	Active, 	Cooldown, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IronBarsWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlastFurnaceLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaffoldingDistance { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BarrelOpen { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaterloggedTransparentWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TintedParticleLeavesPersistent { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StairWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChiseledBookShelfSlot5Occupied { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangrovePropaguleWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BigDripleafWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChorusPlantNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CocoaFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperBarWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseCoralFanWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MossyCarpetSouth { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PistonHeadShort { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepeaterDelay { 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VineNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangroveLeavesWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UntintedParticleLeavesWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CocoaAge { 	Num0, 	Num1, 	Num2, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnderChestWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerHeadPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaffoldingWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HugeMushroomNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectorRailWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StainedGlassPaneWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultifaceUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoweredRailPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperTrapDoorHalf { 	Top, 	Bottom, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperChestFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoratedPotFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FireEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PistonHeadFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripwireDisarmed { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripwireEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointedDripstoneWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperGolemStatueFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetherWartAge { 	Num0, 	Num1, 	Num2, 	Num3, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BarrelFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LightningRodWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CakeBites { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparatorPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperDoorHalf { 	Upper, 	Lower, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BigDripleafStemFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndRodFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaffoldingBottom { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TwistingVinesAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, 	Num16, 	Num17, 	Num18, 	Num19, 	Num20, 	Num21, 	Num22, 	Num23, 	Num24, 	Num25, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JigsawOrientation { 	DownEast, 	DownNorth, 	DownSouth, 	DownWest, 	UpEast, 	UpNorth, 	UpSouth, 	UpWest, 	WestUp, 	EastUp, 	NorthUp, 	SouthUp, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitherSkullPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandConditional { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LecternHasBook { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultOminous { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaplingStage { 	Num0, 	Num1, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StainedGlassPaneSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringLanternWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BambooStalkStage { 	Num0, 	Num1, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RailWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmallDripleafFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShelfFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallEast { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseCoralWallFanFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceGateFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlabWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndPortalFrameEye { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrostedIceAge { 	Num0, 	Num1, 	Num2, 	Num3, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FurnaceFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LadderWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrushableDusted { 	Num0, 	Num1, 	Num2, 	Num3, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandleLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperStairFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlastFurnaceFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripWireHookAttached { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoweredRailWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeaPicklePickles { 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripWireHookPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallWest { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChestFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperSlabWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IronBarsEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangroveLeavesPersistent { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SweetBerryBushAge { 	Num0, 	Num1, 	Num2, 	Num3, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitherWallSkullFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitherWallSkullPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneWirePower { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangrovePropaguleHanging { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrewingStandHasBottle0 { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrialSpawnerTrialSpawnerState { 	Inactive, 	WaitingForPlayers, 	Active, 	WaitingForRewardEjection, 	EjectingReward, 	Cooldown, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparatorMode { 	Compare, 	Subtract, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringLightningRodPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreakingHeartNatural { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperBarEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonFace { 	Floor, 	Wall, 	Ceiling, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TintedParticleLeavesDistance { 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DispenserTriggered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoorPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BedFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowerBedFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerHeadRotation { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepeaterFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetherPortalAxis { 	X, 	Z, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PistonBaseFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttachedStemFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrappedChestFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneWireNorth { 	Up, 	Side, 	None, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoorHalf { 	Upper, 	Lower, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneWallTorchFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkShriekerCanSummon { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeafLitterFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalibratedSculkSensorSculkSensorPhase { 	Inactive, 	Active, 	Cooldown, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnifferEggHatch { 	Num0, 	Num1, 	Num2, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HugeMushroomSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CampfireLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreakingHeartAxis { 	X, 	Y, 	Z, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObserverPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrapdoorPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperDoorOpen { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoweredRailShape { 	NorthSouth, 	EastWest, 	AscendingEast, 	AscendingWest, 	AscendingNorth, 	AscendingSouth, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperSlabType { 	Top, 	Bottom, 	Double, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkSensorPower { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepeaterPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DropperTriggered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeetrootAge { 	Num0, 	Num1, 	Num2, 	Num3, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DaylightDetectorInverted { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseCoralWallFanWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StairShape { 	Straight, 	InnerLeft, 	InnerRight, 	OuterLeft, 	OuterRight, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkSensorWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallHangingSignWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneWireEast { 	Up, 	Side, 	None, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneWireWest { 	Up, 	Side, 	None, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripwirePowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StairFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoorOpen { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoublePlantHalf { 	Upper, 	Lower, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UntintedParticleLeavesPersistent { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StainedGlassPaneNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrapdoorOpen { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlazedTerracottaFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HugeMushroomDown { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChorusPlantWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceGatePowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperChainWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangroveRootsWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StandingSignRotation { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PressurePlatePowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovingPistonFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowerBedFlowerAmount { 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriedGhastFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TallFlowerHalf { 	Upper, 	Lower, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoorFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultifaceWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UntintedParticleLeavesDistance { 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BigDripleafStemWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChorusFlowerAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectorRailShape { 	NorthSouth, 	EastWest, 	AscendingEast, 	AscendingWest, 	AscendingNorth, 	AscendingSouth, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnvilFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HopperEnabled { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkVeinUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShelfWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectorRailPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DropperFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StandingSignWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkVeinWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalibratedSculkSensorFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallSouth { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VineWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeepingVinesAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, 	Num16, 	Num17, 	Num18, 	Num19, 	Num20, 	Num21, 	Num22, 	Num23, 	Num24, 	Num25, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanternHanging { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LadderFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperBulbBlockLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PotatoAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperChestType { 	Single, 	Left, 	Right, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StemAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HangingMossTip { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JackOLanternFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RespawnAnchorCharges { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TorchflowerCropAge { 	Num0, 	Num1, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandleCakeLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShulkerBoxFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultifaceWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrafterTriggered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShelfSideChain { 	Unconnected, 	Right, 	Center, 	Left, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IronBarsNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperBulbLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PistonBaseExtended { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperTrapDoorOpen { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoomFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkVeinDown { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeaPickleWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlowLichenEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoralPlantWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrafterCrafting { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallTorchFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MyceliumSnowy { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BubbleColumnDrag { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteInstrument { 	Harp, 	Basedrum, 	Snare, 	Hat, 	Bass, 	Flute, 	Bell, 	Guitar, 	Chime, 	Xylophone, 	IronXylophone, 	CowBell, 	Didgeridoo, 	Bit, 	Banjo, 	Pling, 	Zombie, 	Skeleton, 	Creeper, 	Dragon, 	WitherSkeleton, 	Piglin, 	CustomHead, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChorusPlantSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovingPistonType { 	Normal, 	Sticky, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotePowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MossyCarpetWest { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnowyDirtSnowy { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoralFanWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneWireSouth { 	Up, 	Side, 	None, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PitcherCropHalf { 	Upper, 	Lower, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperChainAxis { 	X, 	Y, 	Z, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperGolemStatueCopperGolemPose { 	Standing, 	Sitting, 	Running, 	Star, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConduitWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChiseledBookShelfSlot2Occupied { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HugeMushroomUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlowLichenUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkVeinSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FurnaceLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultifaceSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriedGhastWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperChestType { 	Single, 	Left, 	Right, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreakingHeartCreakingHeartState { 	Uprooted, 	Dormant, 	Awake, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointedDripstoneVerticalDirection { 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KelpAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, 	Num16, 	Num17, 	Num18, 	Num19, 	Num20, 	Num21, 	Num22, 	Num23, 	Num24, 	Num25, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperChestWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CampfireSignalFire { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperBulbPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrapdoorWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CampfireWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaveVinesBerries { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChiseledBookShelfFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperDoorFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperTrapDoorWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LecternFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeafLitterSegmentAmount { 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkShriekerWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangroveLeavesDistance { 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TallSeagrassHalf { 	Upper, 	Lower, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HopperFacing { 	Down, 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TntUnstable { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrialSpawnerOminous { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperDoorHinge { 	Left, 	Right, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TintedParticleLeavesWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChestType { 	Single, 	Left, 	Right, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BedOccupied { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceGateInWall { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CactusAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrindstoneFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HangingRootsWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoralWallFanFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HugeMushroomEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangrovePropaguleAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PiglinwallskullFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteNote { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, 	Num16, 	Num17, 	Num18, 	Num19, 	Num20, 	Num21, 	Num22, 	Num23, 	Num24, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointedDripstoneThickness { 	TipMerge, 	Tip, 	Frustum, 	Middle, 	Base, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChorusPlantUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperChestWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeverFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseCoralPlantWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalibratedSculkSensorWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlabType { 	Top, 	Bottom, 	Double, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlowLichenWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MossyCarpetBottom { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultifaceEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructureMode { 	Save, 	Load, 	Corner, 	Data, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripwireNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChiseledBookShelfSlot3Occupied { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FireSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestMode { 	Start, 	Log, 	Fail, 	Accept, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmallDripleafHalf { 	Upper, 	Lower, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperGolemStatueFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperStairWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PiglinwallskullPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChestWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmethystClusterWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChiseledBookShelfSlot1Occupied { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriedGhastHydration { 	Num0, 	Num1, 	Num2, 	Num3, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeehiveHoneyLevel { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BedPart { 	Head, 	Foot, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalibratedSculkSensorPower { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkVeinNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmokerLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperGolemStatueCopperGolemPose { 	Standing, 	Sitting, 	Running, 	Star, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DispenserFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeverFace { 	Floor, 	Wall, 	Ceiling, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperBarNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallNorth { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BellAttachment { 	Floor, 	Ceiling, 	SingleWall, 	DoubleWall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StairHalf { 	Top, 	Bottom, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperStairShape { 	Straight, 	InnerLeft, 	InnerRight, 	OuterLeft, 	OuterRight, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BigDripleafFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneWallTorchLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FarmMoisture { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringLanternHanging { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallBannerFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanternWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeverPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmallDripleafWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandleWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VineSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeightedPressurePlatePower { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StainedGlassPaneWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObserverFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayeredCauldronLevel { 	Num1, 	Num2, 	Num3, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultifaceNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlowLichenNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BambooStalkAge { 	Num0, 	Num1, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VineUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaveVinesPlantBerries { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurtleEggHatch { 	Num0, 	Num1, 	Num2, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeehiveFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MossyCarpetNorth { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitherSkullRotation { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneLampLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceGateOpen { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoorHinge { 	Left, 	Right, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperGrateWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangrovePropaguleStage { 	Num0, 	Num1, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LightningRodFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarrotAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnderChestFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoratedPotWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IronBarsSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperGolemStatueWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposterLevel { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LightLevel { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparatorFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrewingStandHasBottle1 { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallSkullPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripwireSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnowLayerLayers { 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrewingStandHasBottle2 { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainAxis { 	X, 	Y, 	Z, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LecternPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiquidLevel { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerWallHeadFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallHangingSignFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrafterOrientation { 	DownEast, 	DownNorth, 	DownSouth, 	DownWest, 	UpEast, 	UpNorth, 	UpSouth, 	UpWest, 	WestUp, 	EastUp, 	NorthUp, 	SouthUp, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkullPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FireUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShelfPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmethystClusterFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CeilingHangingSignRotation { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BellFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StainedGlassPaneEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallSkullFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CampfireFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkullRotation { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlowLichenSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrassSnowy { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkCatalystBloom { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StonecutterFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SugarCaneAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RailShape { 	NorthSouth, 	EastWest, 	AscendingEast, 	AscendingWest, 	AscendingNorth, 	AscendingSouth, 	SouthEast, 	SouthWest, 	NorthWest, 	NorthEast, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrappedChestType { 	Single, 	Left, 	Right, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperBarWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperStairHalf { 	Top, 	Bottom, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FireAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultifaceDown { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CeilingHangingSignAttached { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneTorchLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmokerFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrapdoorHalf { 	Top, 	Bottom, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChorusPlantDown { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperDoorPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DaylightDetectorPower { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaveVinesAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, 	Num16, 	Num17, 	Num18, 	Num19, 	Num20, 	Num21, 	Num22, 	Num23, 	Num24, 	Num25, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrapdoorFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HugeMushroomWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkVeinWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetPower { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CeilingHangingSignWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BannerRotation { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoratedPotCracked { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LightWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringLightningRodFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MossyCarpetEast { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PitcherCropAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlowLichenWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerWallHeadPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkVeinEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripWireHookFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurtleEggEggs { 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CropAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HayAxis { 	X, 	Y, 	Z, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneOreLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringLightningRodWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BambooStalkLeaves { 	None, 	Small, 	Large, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FireWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepeaterLocked { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperTrapDoorPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripwireWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripwireAttached { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BigDripleafTilt { 	None, 	Unstable, 	Partial, 	Full, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeavyCoreWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperBulbBlockPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VineEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperGolemStatueWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperBarSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandleCandles { 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LightningRodPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrindstoneFace { 	Floor, 	Wall, 	Ceiling, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallSignWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndPortalFrameFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallSignFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Property {
	ChiseledBookShelfSlot4Occupied(ChiseledBookShelfSlot4Occupied),
	InfestedRotatedPillarAxis(InfestedRotatedPillarAxis),
	JukeboxHasRecord(JukeboxHasRecord),
	FireNorth(FireNorth),
	ChiseledBookShelfSlot0Occupied(ChiseledBookShelfSlot0Occupied),
	IronBarsWest(IronBarsWest),
	TrappedChestWaterlogged(TrappedChestWaterlogged),
	CopperChestFacing(CopperChestFacing),
	ChorusPlantEast(ChorusPlantEast),
	BellPowered(BellPowered),
	WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing),
	PistonHeadType(PistonHeadType),
	WallWaterlogged(WallWaterlogged),
	RotatedPillarAxis(RotatedPillarAxis),
	BarrierWaterlogged(BarrierWaterlogged),
	CoralWallFanWaterlogged(CoralWallFanWaterlogged),
	GlowLichenDown(GlowLichenDown),
	SculkShriekerShrieking(SculkShriekerShrieking),
	VaultVaultState(VaultVaultState),
	WallUp(WallUp),
	SculkSensorSculkSensorPhase(SculkSensorSculkSensorPhase),
	IronBarsWaterlogged(IronBarsWaterlogged),
	BlastFurnaceLit(BlastFurnaceLit),
	ScaffoldingDistance(ScaffoldingDistance),
	BarrelOpen(BarrelOpen),
	WaterloggedTransparentWaterlogged(WaterloggedTransparentWaterlogged),
	TintedParticleLeavesPersistent(TintedParticleLeavesPersistent),
	StairWaterlogged(StairWaterlogged),
	ChiseledBookShelfSlot5Occupied(ChiseledBookShelfSlot5Occupied),
	MangrovePropaguleWaterlogged(MangrovePropaguleWaterlogged),
	BigDripleafWaterlogged(BigDripleafWaterlogged),
	ChorusPlantNorth(ChorusPlantNorth),
	CocoaFacing(CocoaFacing),
	WeatheringCopperBarWest(WeatheringCopperBarWest),
	BaseCoralFanWaterlogged(BaseCoralFanWaterlogged),
	MossyCarpetSouth(MossyCarpetSouth),
	PistonHeadShort(PistonHeadShort),
	RepeaterDelay(RepeaterDelay),
	VineNorth(VineNorth),
	MangroveLeavesWaterlogged(MangroveLeavesWaterlogged),
	UntintedParticleLeavesWaterlogged(UntintedParticleLeavesWaterlogged),
	CocoaAge(CocoaAge),
	EnderChestWaterlogged(EnderChestWaterlogged),
	PlayerHeadPowered(PlayerHeadPowered),
	ScaffoldingWaterlogged(ScaffoldingWaterlogged),
	HugeMushroomNorth(HugeMushroomNorth),
	DetectorRailWaterlogged(DetectorRailWaterlogged),
	StainedGlassPaneWaterlogged(StainedGlassPaneWaterlogged),
	MultifaceUp(MultifaceUp),
	ButtonFacing(ButtonFacing),
	PoweredRailPowered(PoweredRailPowered),
	WeatheringCopperTrapDoorHalf(WeatheringCopperTrapDoorHalf),
	WeatheringCopperChestFacing(WeatheringCopperChestFacing),
	DecoratedPotFacing(DecoratedPotFacing),
	FireEast(FireEast),
	PistonHeadFacing(PistonHeadFacing),
	TripwireDisarmed(TripwireDisarmed),
	TripwireEast(TripwireEast),
	PointedDripstoneWaterlogged(PointedDripstoneWaterlogged),
	WeatheringCopperGolemStatueFacing(WeatheringCopperGolemStatueFacing),
	NetherWartAge(NetherWartAge),
	BarrelFacing(BarrelFacing),
	LightningRodWaterlogged(LightningRodWaterlogged),
	CakeBites(CakeBites),
	ComparatorPowered(ComparatorPowered),
	WeatheringCopperDoorHalf(WeatheringCopperDoorHalf),
	BigDripleafStemFacing(BigDripleafStemFacing),
	EndRodFacing(EndRodFacing),
	FenceWest(FenceWest),
	ScaffoldingBottom(ScaffoldingBottom),
	TwistingVinesAge(TwistingVinesAge),
	JigsawOrientation(JigsawOrientation),
	WitherSkullPowered(WitherSkullPowered),
	CommandConditional(CommandConditional),
	LecternHasBook(LecternHasBook),
	VaultOminous(VaultOminous),
	SaplingStage(SaplingStage),
	StainedGlassPaneSouth(StainedGlassPaneSouth),
	WeatheringLanternWaterlogged(WeatheringLanternWaterlogged),
	FenceNorth(FenceNorth),
	BambooStalkStage(BambooStalkStage),
	RailWaterlogged(RailWaterlogged),
	SmallDripleafFacing(SmallDripleafFacing),
	ShelfFacing(ShelfFacing),
	WallEast(WallEast),
	BaseCoralWallFanFacing(BaseCoralWallFanFacing),
	FenceGateFacing(FenceGateFacing),
	FenceEast(FenceEast),
	SlabWaterlogged(SlabWaterlogged),
	EndPortalFrameEye(EndPortalFrameEye),
	FrostedIceAge(FrostedIceAge),
	FurnaceFacing(FurnaceFacing),
	LadderWaterlogged(LadderWaterlogged),
	BrushableDusted(BrushableDusted),
	CandleLit(CandleLit),
	WeatheringCopperStairFacing(WeatheringCopperStairFacing),
	BlastFurnaceFacing(BlastFurnaceFacing),
	TripWireHookAttached(TripWireHookAttached),
	PoweredRailWaterlogged(PoweredRailWaterlogged),
	SeaPicklePickles(SeaPicklePickles),
	TripWireHookPowered(TripWireHookPowered),
	WallWest(WallWest),
	ChestFacing(ChestFacing),
	WeatheringCopperSlabWaterlogged(WeatheringCopperSlabWaterlogged),
	IronBarsEast(IronBarsEast),
	MangroveLeavesPersistent(MangroveLeavesPersistent),
	SweetBerryBushAge(SweetBerryBushAge),
	WitherWallSkullFacing(WitherWallSkullFacing),
	WitherWallSkullPowered(WitherWallSkullPowered),
	RedstoneWirePower(RedstoneWirePower),
	MangrovePropaguleHanging(MangrovePropaguleHanging),
	BrewingStandHasBottle0(BrewingStandHasBottle0),
	TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState),
	ComparatorMode(ComparatorMode),
	WeatheringLightningRodPowered(WeatheringLightningRodPowered),
	CreakingHeartNatural(CreakingHeartNatural),
	WeatheringCopperBarEast(WeatheringCopperBarEast),
	ButtonFace(ButtonFace),
	TintedParticleLeavesDistance(TintedParticleLeavesDistance),
	DispenserTriggered(DispenserTriggered),
	DoorPowered(DoorPowered),
	BedFacing(BedFacing),
	FlowerBedFacing(FlowerBedFacing),
	PlayerHeadRotation(PlayerHeadRotation),
	RepeaterFacing(RepeaterFacing),
	NetherPortalAxis(NetherPortalAxis),
	PistonBaseFacing(PistonBaseFacing),
	AttachedStemFacing(AttachedStemFacing),
	TrappedChestFacing(TrappedChestFacing),
	RedstoneWireNorth(RedstoneWireNorth),
	DoorHalf(DoorHalf),
	RedstoneWallTorchFacing(RedstoneWallTorchFacing),
	SculkShriekerCanSummon(SculkShriekerCanSummon),
	LeafLitterFacing(LeafLitterFacing),
	CalibratedSculkSensorSculkSensorPhase(CalibratedSculkSensorSculkSensorPhase),
	SnifferEggHatch(SnifferEggHatch),
	HugeMushroomSouth(HugeMushroomSouth),
	CampfireLit(CampfireLit),
	CreakingHeartAxis(CreakingHeartAxis),
	ObserverPowered(ObserverPowered),
	TrapdoorPowered(TrapdoorPowered),
	WeatheringCopperDoorOpen(WeatheringCopperDoorOpen),
	PoweredRailShape(PoweredRailShape),
	WeatheringCopperSlabType(WeatheringCopperSlabType),
	ChainWaterlogged(ChainWaterlogged),
	SculkSensorPower(SculkSensorPower),
	RepeaterPowered(RepeaterPowered),
	DropperTriggered(DropperTriggered),
	BeetrootAge(BeetrootAge),
	DaylightDetectorInverted(DaylightDetectorInverted),
	BaseCoralWallFanWaterlogged(BaseCoralWallFanWaterlogged),
	StairShape(StairShape),
	SculkSensorWaterlogged(SculkSensorWaterlogged),
	WallHangingSignWaterlogged(WallHangingSignWaterlogged),
	RedstoneWireEast(RedstoneWireEast),
	RedstoneWireWest(RedstoneWireWest),
	TripwirePowered(TripwirePowered),
	StairFacing(StairFacing),
	DoorOpen(DoorOpen),
	DoublePlantHalf(DoublePlantHalf),
	UntintedParticleLeavesPersistent(UntintedParticleLeavesPersistent),
	StainedGlassPaneNorth(StainedGlassPaneNorth),
	TrapdoorOpen(TrapdoorOpen),
	GlazedTerracottaFacing(GlazedTerracottaFacing),
	HugeMushroomDown(HugeMushroomDown),
	ChorusPlantWest(ChorusPlantWest),
	FenceGatePowered(FenceGatePowered),
	CommandFacing(CommandFacing),
	WeatheringCopperChainWaterlogged(WeatheringCopperChainWaterlogged),
	MangroveRootsWaterlogged(MangroveRootsWaterlogged),
	StandingSignRotation(StandingSignRotation),
	PressurePlatePowered(PressurePlatePowered),
	MovingPistonFacing(MovingPistonFacing),
	FlowerBedFlowerAmount(FlowerBedFlowerAmount),
	DriedGhastFacing(DriedGhastFacing),
	TallFlowerHalf(TallFlowerHalf),
	DoorFacing(DoorFacing),
	MultifaceWaterlogged(MultifaceWaterlogged),
	UntintedParticleLeavesDistance(UntintedParticleLeavesDistance),
	BigDripleafStemWaterlogged(BigDripleafStemWaterlogged),
	ChorusFlowerAge(ChorusFlowerAge),
	DetectorRailShape(DetectorRailShape),
	AnvilFacing(AnvilFacing),
	HopperEnabled(HopperEnabled),
	SculkVeinUp(SculkVeinUp),
	ShelfWaterlogged(ShelfWaterlogged),
	DetectorRailPowered(DetectorRailPowered),
	DropperFacing(DropperFacing),
	StandingSignWaterlogged(StandingSignWaterlogged),
	SculkVeinWaterlogged(SculkVeinWaterlogged),
	CalibratedSculkSensorFacing(CalibratedSculkSensorFacing),
	WallSouth(WallSouth),
	VineWest(VineWest),
	WeepingVinesAge(WeepingVinesAge),
	LanternHanging(LanternHanging),
	LadderFacing(LadderFacing),
	CopperBulbBlockLit(CopperBulbBlockLit),
	PotatoAge(PotatoAge),
	WeatheringCopperChestType(WeatheringCopperChestType),
	StemAge(StemAge),
	HangingMossTip(HangingMossTip),
	JackOLanternFacing(JackOLanternFacing),
	RespawnAnchorCharges(RespawnAnchorCharges),
	TorchflowerCropAge(TorchflowerCropAge),
	CandleCakeLit(CandleCakeLit),
	ShulkerBoxFacing(ShulkerBoxFacing),
	MultifaceWest(MultifaceWest),
	CrafterTriggered(CrafterTriggered),
	ShelfSideChain(ShelfSideChain),
	IronBarsNorth(IronBarsNorth),
	WeatheringCopperBulbLit(WeatheringCopperBulbLit),
	PistonBaseExtended(PistonBaseExtended),
	WeatheringCopperTrapDoorOpen(WeatheringCopperTrapDoorOpen),
	LoomFacing(LoomFacing),
	SculkVeinDown(SculkVeinDown),
	SeaPickleWaterlogged(SeaPickleWaterlogged),
	GlowLichenEast(GlowLichenEast),
	CoralPlantWaterlogged(CoralPlantWaterlogged),
	CrafterCrafting(CrafterCrafting),
	WallTorchFacing(WallTorchFacing),
	MyceliumSnowy(MyceliumSnowy),
	BubbleColumnDrag(BubbleColumnDrag),
	NoteInstrument(NoteInstrument),
	ChorusPlantSouth(ChorusPlantSouth),
	MovingPistonType(MovingPistonType),
	NotePowered(NotePowered),
	MossyCarpetWest(MossyCarpetWest),
	SnowyDirtSnowy(SnowyDirtSnowy),
	CoralFanWaterlogged(CoralFanWaterlogged),
	RedstoneWireSouth(RedstoneWireSouth),
	PitcherCropHalf(PitcherCropHalf),
	WeatheringCopperChainAxis(WeatheringCopperChainAxis),
	WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose),
	ConduitWaterlogged(ConduitWaterlogged),
	ChiseledBookShelfSlot2Occupied(ChiseledBookShelfSlot2Occupied),
	HugeMushroomUp(HugeMushroomUp),
	GlowLichenUp(GlowLichenUp),
	VaultFacing(VaultFacing),
	SculkVeinSouth(SculkVeinSouth),
	FurnaceLit(FurnaceLit),
	MultifaceSouth(MultifaceSouth),
	DriedGhastWaterlogged(DriedGhastWaterlogged),
	CopperChestType(CopperChestType),
	CreakingHeartCreakingHeartState(CreakingHeartCreakingHeartState),
	PointedDripstoneVerticalDirection(PointedDripstoneVerticalDirection),
	KelpAge(KelpAge),
	CopperChestWaterlogged(CopperChestWaterlogged),
	CampfireSignalFire(CampfireSignalFire),
	WeatheringCopperBulbPowered(WeatheringCopperBulbPowered),
	TrapdoorWaterlogged(TrapdoorWaterlogged),
	CampfireWaterlogged(CampfireWaterlogged),
	CaveVinesBerries(CaveVinesBerries),
	ChiseledBookShelfFacing(ChiseledBookShelfFacing),
	WeatheringCopperDoorFacing(WeatheringCopperDoorFacing),
	FenceWaterlogged(FenceWaterlogged),
	WeatheringCopperTrapDoorWaterlogged(WeatheringCopperTrapDoorWaterlogged),
	LecternFacing(LecternFacing),
	LeafLitterSegmentAmount(LeafLitterSegmentAmount),
	SculkShriekerWaterlogged(SculkShriekerWaterlogged),
	MangroveLeavesDistance(MangroveLeavesDistance),
	TallSeagrassHalf(TallSeagrassHalf),
	HopperFacing(HopperFacing),
	TntUnstable(TntUnstable),
	TrialSpawnerOminous(TrialSpawnerOminous),
	WeatheringCopperDoorHinge(WeatheringCopperDoorHinge),
	TintedParticleLeavesWaterlogged(TintedParticleLeavesWaterlogged),
	ChestType(ChestType),
	BedOccupied(BedOccupied),
	FenceGateInWall(FenceGateInWall),
	CactusAge(CactusAge),
	GrindstoneFacing(GrindstoneFacing),
	HangingRootsWaterlogged(HangingRootsWaterlogged),
	CoralWallFanFacing(CoralWallFanFacing),
	HugeMushroomEast(HugeMushroomEast),
	MangrovePropaguleAge(MangrovePropaguleAge),
	PiglinwallskullFacing(PiglinwallskullFacing),
	NoteNote(NoteNote),
	PointedDripstoneThickness(PointedDripstoneThickness),
	ChorusPlantUp(ChorusPlantUp),
	WeatheringCopperChestWaterlogged(WeatheringCopperChestWaterlogged),
	LeverFacing(LeverFacing),
	BaseCoralPlantWaterlogged(BaseCoralPlantWaterlogged),
	CalibratedSculkSensorWaterlogged(CalibratedSculkSensorWaterlogged),
	SlabType(SlabType),
	GlowLichenWaterlogged(GlowLichenWaterlogged),
	MossyCarpetBottom(MossyCarpetBottom),
	MultifaceEast(MultifaceEast),
	StructureMode(StructureMode),
	TripwireNorth(TripwireNorth),
	ChiseledBookShelfSlot3Occupied(ChiseledBookShelfSlot3Occupied),
	FireSouth(FireSouth),
	TestMode(TestMode),
	SmallDripleafHalf(SmallDripleafHalf),
	CopperGolemStatueFacing(CopperGolemStatueFacing),
	WeatheringCopperStairWaterlogged(WeatheringCopperStairWaterlogged),
	PiglinwallskullPowered(PiglinwallskullPowered),
	ChestWaterlogged(ChestWaterlogged),
	AmethystClusterWaterlogged(AmethystClusterWaterlogged),
	ChiseledBookShelfSlot1Occupied(ChiseledBookShelfSlot1Occupied),
	DriedGhastHydration(DriedGhastHydration),
	BeehiveHoneyLevel(BeehiveHoneyLevel),
	BedPart(BedPart),
	CalibratedSculkSensorPower(CalibratedSculkSensorPower),
	SculkVeinNorth(SculkVeinNorth),
	SmokerLit(SmokerLit),
	ButtonPowered(ButtonPowered),
	CopperGolemStatueCopperGolemPose(CopperGolemStatueCopperGolemPose),
	DispenserFacing(DispenserFacing),
	FenceSouth(FenceSouth),
	LeverFace(LeverFace),
	WeatheringCopperBarNorth(WeatheringCopperBarNorth),
	WallNorth(WallNorth),
	BellAttachment(BellAttachment),
	StairHalf(StairHalf),
	WeatheringCopperStairShape(WeatheringCopperStairShape),
	BigDripleafFacing(BigDripleafFacing),
	RedstoneWallTorchLit(RedstoneWallTorchLit),
	FarmMoisture(FarmMoisture),
	WeatheringLanternHanging(WeatheringLanternHanging),
	WallBannerFacing(WallBannerFacing),
	LanternWaterlogged(LanternWaterlogged),
	LeverPowered(LeverPowered),
	SmallDripleafWaterlogged(SmallDripleafWaterlogged),
	CandleWaterlogged(CandleWaterlogged),
	VineSouth(VineSouth),
	WeightedPressurePlatePower(WeightedPressurePlatePower),
	StainedGlassPaneWest(StainedGlassPaneWest),
	ObserverFacing(ObserverFacing),
	LayeredCauldronLevel(LayeredCauldronLevel),
	MultifaceNorth(MultifaceNorth),
	GlowLichenNorth(GlowLichenNorth),
	BambooStalkAge(BambooStalkAge),
	VineUp(VineUp),
	CaveVinesPlantBerries(CaveVinesPlantBerries),
	TurtleEggHatch(TurtleEggHatch),
	BeehiveFacing(BeehiveFacing),
	MossyCarpetNorth(MossyCarpetNorth),
	WitherSkullRotation(WitherSkullRotation),
	RedstoneLampLit(RedstoneLampLit),
	FenceGateOpen(FenceGateOpen),
	DoorHinge(DoorHinge),
	WeatheringCopperGrateWaterlogged(WeatheringCopperGrateWaterlogged),
	MangrovePropaguleStage(MangrovePropaguleStage),
	LightningRodFacing(LightningRodFacing),
	CarrotAge(CarrotAge),
	EnderChestFacing(EnderChestFacing),
	DecoratedPotWaterlogged(DecoratedPotWaterlogged),
	IronBarsSouth(IronBarsSouth),
	WeatheringCopperGolemStatueWaterlogged(WeatheringCopperGolemStatueWaterlogged),
	ComposterLevel(ComposterLevel),
	LightLevel(LightLevel),
	ComparatorFacing(ComparatorFacing),
	BrewingStandHasBottle1(BrewingStandHasBottle1),
	WallSkullPowered(WallSkullPowered),
	TripwireSouth(TripwireSouth),
	SnowLayerLayers(SnowLayerLayers),
	BrewingStandHasBottle2(BrewingStandHasBottle2),
	ChainAxis(ChainAxis),
	LecternPowered(LecternPowered),
	LiquidLevel(LiquidLevel),
	PlayerWallHeadFacing(PlayerWallHeadFacing),
	WallHangingSignFacing(WallHangingSignFacing),
	CrafterOrientation(CrafterOrientation),
	SkullPowered(SkullPowered),
	FireUp(FireUp),
	ShelfPowered(ShelfPowered),
	AmethystClusterFacing(AmethystClusterFacing),
	CeilingHangingSignRotation(CeilingHangingSignRotation),
	BellFacing(BellFacing),
	StainedGlassPaneEast(StainedGlassPaneEast),
	WallSkullFacing(WallSkullFacing),
	CampfireFacing(CampfireFacing),
	SkullRotation(SkullRotation),
	GlowLichenSouth(GlowLichenSouth),
	GrassSnowy(GrassSnowy),
	SculkCatalystBloom(SculkCatalystBloom),
	StonecutterFacing(StonecutterFacing),
	SugarCaneAge(SugarCaneAge),
	RailShape(RailShape),
	TrappedChestType(TrappedChestType),
	WeatheringCopperBarWaterlogged(WeatheringCopperBarWaterlogged),
	WeatheringCopperStairHalf(WeatheringCopperStairHalf),
	FireAge(FireAge),
	MultifaceDown(MultifaceDown),
	CeilingHangingSignAttached(CeilingHangingSignAttached),
	RedstoneTorchLit(RedstoneTorchLit),
	SmokerFacing(SmokerFacing),
	TrapdoorHalf(TrapdoorHalf),
	ChorusPlantDown(ChorusPlantDown),
	WeatheringCopperDoorPowered(WeatheringCopperDoorPowered),
	DaylightDetectorPower(DaylightDetectorPower),
	CaveVinesAge(CaveVinesAge),
	TrapdoorFacing(TrapdoorFacing),
	HugeMushroomWest(HugeMushroomWest),
	SculkVeinWest(SculkVeinWest),
	TargetPower(TargetPower),
	CeilingHangingSignWaterlogged(CeilingHangingSignWaterlogged),
	BannerRotation(BannerRotation),
	DecoratedPotCracked(DecoratedPotCracked),
	LightWaterlogged(LightWaterlogged),
	WeatheringLightningRodFacing(WeatheringLightningRodFacing),
	MossyCarpetEast(MossyCarpetEast),
	PitcherCropAge(PitcherCropAge),
	GlowLichenWest(GlowLichenWest),
	PlayerWallHeadPowered(PlayerWallHeadPowered),
	SculkVeinEast(SculkVeinEast),
	TripWireHookFacing(TripWireHookFacing),
	TurtleEggEggs(TurtleEggEggs),
	CropAge(CropAge),
	HayAxis(HayAxis),
	RedstoneOreLit(RedstoneOreLit),
	WeatheringLightningRodWaterlogged(WeatheringLightningRodWaterlogged),
	BambooStalkLeaves(BambooStalkLeaves),
	FireWest(FireWest),
	RepeaterLocked(RepeaterLocked),
	WeatheringCopperTrapDoorPowered(WeatheringCopperTrapDoorPowered),
	TripwireWest(TripwireWest),
	TripwireAttached(TripwireAttached),
	BigDripleafTilt(BigDripleafTilt),
	HeavyCoreWaterlogged(HeavyCoreWaterlogged),
	CopperBulbBlockPowered(CopperBulbBlockPowered),
	VineEast(VineEast),
	CopperGolemStatueWaterlogged(CopperGolemStatueWaterlogged),
	WeatheringCopperBarSouth(WeatheringCopperBarSouth),
	CandleCandles(CandleCandles),
	LightningRodPowered(LightningRodPowered),
	GrindstoneFace(GrindstoneFace),
	WallSignWaterlogged(WallSignWaterlogged),
	EndPortalFrameFacing(EndPortalFrameFacing),
	WallSignFacing(WallSignFacing),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
	Air,
	Amethyst,
	AmethystCluster,
	Anvil,
	AttachedStem,
	Azalea,
	BambooSapling,
	BambooStalk,
	Banner,
	Barrel,
	Barrier,
	BaseCoralFan,
	BaseCoralPlant,
	BaseCoralWallFan,
	Beacon,
	Bed,
	Beehive,
	Beetroot,
	Bell,
	BigDripleaf,
	BigDripleafStem,
	BlastFurnace,
	Block,
	BonemealableFeaturePlacer,
	Bottom,
	BrewingStand,
	Brushable,
	BubbleColumn,
	BuddingAmethyst,
	Bush,
	Button,
	Cactus,
	CactusFlower,
	Cake,
	CalibratedSculkSensor,
	Campfire,
	Candle,
	CandleCake,
	Carpet,
	Carrot,
	CartographyTable,
	Cauldron,
	CaveVines,
	CaveVinesPlant,
	CeilingHangingSign,
	Chain,
	CherryLeaves,
	Chest,
	ChiseledBookShelf,
	ChorusFlower,
	ChorusPlant,
	Cocoa,
	ColoredFalling,
	Command,
	Comparator,
	Composter,
	ConcretePowder,
	Conduit,
	CopperBulbBlock,
	CopperChest,
	CopperGolemStatue,
	Coral,
	CoralFan,
	CoralPlant,
	CoralWallFan,
	Crafter,
	CraftingTable,
	CreakingHeart,
	Crop,
	CryingObsidian,
	DaylightDetector,
	DecoratedPot,
	DetectorRail,
	DirtPath,
	Dispenser,
	Door,
	Double,
	DoublePlant,
	DragonEgg,
	DriedGhast,
	DropExperience,
	Dropper,
	DryVegetation,
	EnchantmentTable,
	EndGateway,
	EndPortal,
	EndPortalFrame,
	EndRod,
	EnderChest,
	Eyeblossom,
	Farm,
	Fence,
	FenceGate,
	Fire,
	FireflyBush,
	Flower,
	FlowerBed,
	FlowerPot,
	Frogspawn,
	FrostedIce,
	Fungus,
	Furnace,
	GlazedTerracotta,
	GlowLichen,
	Grass,
	Grindstone,
	HalfTransparent,
	HangingMoss,
	HangingRoots,
	Hay,
	HeavyCore,
	Honey,
	Hopper,
	HugeMushroom,
	Ice,
	Infested,
	InfestedRotatedPillar,
	IronBars,
	JackOLantern,
	Jigsaw,
	Jukebox,
	Kelp,
	KelpPlant,
	Ladder,
	Lantern,
	LavaCauldron,
	LayeredCauldron,
	LeafLitter,
	Lectern,
	Left,
	Lever,
	Light,
	LightningRod,
	Liquid,
	Loom,
	Magma,
	MangroveLeaves,
	MangrovePropagule,
	MangroveRoots,
	MossyCarpet,
	MovingPiston,
	Mud,
	Multiface,
	Mushroom,
	Mycelium,
	NetherPortal,
	NetherSprouts,
	NetherWart,
	Netherrack,
	Normal,
	Note,
	Nylium,
	Observer,
	PaleOakLeaves,
	Piglinwallskull,
	PistonBase,
	PistonHead,
	PitcherCrop,
	PlayerHead,
	PlayerWallHead,
	PointedDripstone,
	Potato,
	PowderSnow,
	Powered,
	PoweredRail,
	PressurePlate,
	Pumpkin,
	Rail,
	RedstoneLamp,
	RedstoneOre,
	RedstoneTorch,
	RedstoneWallTorch,
	RedstoneWire,
	Repeater,
	RespawnAnchor,
	Right,
	RootedDirt,
	Roots,
	RotatedPillar,
	Sand,
	Sapling,
	Scaffolding,
	Sculk,
	SculkCatalyst,
	SculkSensor,
	SculkShrieker,
	SculkVein,
	SeaPickle,
	Seagrass,
	Shelf,
	ShortDryGrass,
	ShulkerBox,
	Single,
	Skull,
	Slab,
	Slime,
	SmallDripleaf,
	SmithingTable,
	Smoker,
	SnifferEgg,
	SnowLayer,
	SnowyDirt,
	SoulFire,
	SoulSand,
	Spawner,
	Sponge,
	SporeBlossom,
	StainedGlass,
	StainedGlassPane,
	Stair,
	StandingSign,
	Stem,
	Sticky,
	Stonecutter,
	Structure,
	StructureVoid,
	SugarCane,
	SweetBerryBush,
	TallDryGrass,
	TallFlower,
	TallGrass,
	TallSeagrass,
	Target,
	Test,
	TestInstance,
	TintedGlass,
	TintedLeaves,
	TintedParticleLeaves,
	Tnt,
	Top,
	Torch,
	TorchflowerCrop,
	Transparent,
	Trapdoor,
	TrappedChest,
	TrialSpawner,
	TripWireHook,
	Tripwire,
	TurtleEgg,
	TwistingVines,
	TwistingVinesPlant,
	Uniform,
	UntintedParticleLeaves,
	Vault,
	Vine,
	Wall,
	WallBanner,
	WallHangingSign,
	WallSign,
	WallSkull,
	WallTorch,
	Waterlily,
	WaterloggedTransparent,
	WeatheringCopperBar,
	WeatheringCopperBulb,
	WeatheringCopperChain,
	WeatheringCopperChest,
	WeatheringCopperDoor,
	WeatheringCopperFull,
	WeatheringCopperGolemStatue,
	WeatheringCopperGrate,
	WeatheringCopperSlab,
	WeatheringCopperStair,
	WeatheringCopperTrapDoor,
	WeatheringLantern,
	WeatheringLightningRod,
	Web,
	WeepingVines,
	WeepingVinesPlant,
	WeightedPressurePlate,
	WetSponge,
	WitherRose,
	WitherSkull,
	WitherWallSkull,
	WoolCarpet,
}
