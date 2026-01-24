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
pub enum SnowyDirtSnowy { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructureMode { 	Save, 	Load, 	Corner, 	Data, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MossyCarpetWest { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PiglinwallskullPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TintedParticleLeavesWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperStairWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkShriekerCanSummon { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperChestType { 	Single, 	Left, 	Right, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseCoralFanWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripwireEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LadderFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringLightningRodPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallHangingSignWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperChainWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShelfSideChain { 	Unconnected, 	Right, 	Center, 	Left, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndRodFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FireUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneTorchLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallEast { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanternHanging { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposterLevel { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlowLichenWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BigDripleafFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteInstrument { 	Harp, 	Basedrum, 	Snare, 	Hat, 	Bass, 	Flute, 	Bell, 	Guitar, 	Chime, 	Xylophone, 	IronXylophone, 	CowBell, 	Didgeridoo, 	Bit, 	Banjo, 	Pling, 	Zombie, 	Skeleton, 	Creeper, 	Dragon, 	WitherSkeleton, 	Piglin, 	CustomHead, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChorusPlantWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LightningRodPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneOreLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmokerLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CampfireFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DaylightDetectorPower { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlowLichenNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangroveLeavesDistance { 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StairWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChiseledBookShelfSlot2Occupied { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkVeinSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HugeMushroomDown { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalibratedSculkSensorWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FurnaceFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShelfFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HugeMushroomEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfestedRotatedPillarAxis { 	X, 	Y, 	Z, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperBulbLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrapdoorWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnifferEggHatch { 	Num0, 	Num1, 	Num2, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringLanternHanging { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitherWallSkullPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoralWallFanFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperChestWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndPortalFrameEye { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperGolemStatueWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeightedPressurePlatePower { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparatorPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UntintedParticleLeavesDistance { 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangrovePropaguleWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneLampLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StainedGlassPaneNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LightningRodFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestMode { 	Start, 	Log, 	Fail, 	Accept, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StainedGlassPaneWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlabWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanternWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripWireHookPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HugeMushroomUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BedPart { 	Head, 	Foot, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperChestType { 	Single, 	Left, 	Right, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperGolemStatueFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerWallHeadPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneWallTorchFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoorHalf { 	Upper, 	Lower, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepeaterLocked { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetherPortalAxis { 	X, 	Z, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkSensorSculkSensorPhase { 	Inactive, 	Active, 	Cooldown, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepeaterDelay { 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CeilingHangingSignWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceGateInWall { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperTrapDoorWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperChestWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JigsawOrientation { 	DownEast, 	DownNorth, 	DownSouth, 	DownWest, 	UpEast, 	UpNorth, 	UpSouth, 	UpWest, 	WestUp, 	EastUp, 	NorthUp, 	SouthUp, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrialSpawnerOminous { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BubbleColumnDrag { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UntintedParticleLeavesWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChestWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeavyCoreWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperChestFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperGrateWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperStairFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlowLichenDown { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripwireSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaterloggedTransparentWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PistonHeadShort { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoweredRailWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChestFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepeaterFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperChestFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoublePlantHalf { 	Upper, 	Lower, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseCoralPlantWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeverPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangrovePropaguleHanging { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RotatedPillarAxis { 	X, 	Y, 	Z, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HangingMossTip { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkVeinDown { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CropAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneWirePower { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepeaterPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StainedGlassPaneWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrapdoorHalf { 	Top, 	Bottom, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HugeMushroomNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringLightningRodWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallSignFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkShriekerShrieking { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlazedTerracottaFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitherSkullRotation { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrapdoorPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerWallHeadFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultVaultState { 	Inactive, 	Active, 	Unlocking, 	Ejecting, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperTrapDoorPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperBarNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BarrelOpen { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChorusFlowerAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayeredCauldronLevel { 	Num1, 	Num2, 	Num3, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TintedParticleLeavesDistance { 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaffoldingWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrewingStandHasBottle0 { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperDoorPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotePowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlowLichenWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointedDripstoneWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IronBarsEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoralFanWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperDoorHinge { 	Left, 	Right, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrappedChestWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriedGhastWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceGateOpen { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BigDripleafWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrewingStandHasBottle2 { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkullPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperDoorOpen { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StairHalf { 	Top, 	Bottom, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlowLichenEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JukeboxHasRecord { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteNote { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, 	Num16, 	Num17, 	Num18, 	Num19, 	Num20, 	Num21, 	Num22, 	Num23, 	Num24, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BigDripleafStemFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChiseledBookShelfSlot1Occupied { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperStairShape { 	Straight, 	InnerLeft, 	InnerRight, 	OuterLeft, 	OuterRight, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StemAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObserverPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointedDripstoneVerticalDirection { 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneWireSouth { 	Up, 	Side, 	None, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConduitWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneWireWest { 	Up, 	Side, 	None, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkVeinEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BambooStalkStage { 	Num0, 	Num1, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneWireEast { 	Up, 	Side, 	None, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DispenserFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoralPlantWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalibratedSculkSensorFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaffoldingDistance { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeaPickleWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripwirePowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CakeBites { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkullRotation { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoomFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreakingHeartAxis { 	X, 	Y, 	Z, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowerBedFlowerAmount { 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkVeinUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeehiveFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PistonHeadFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CampfireSignalFire { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeverFace { 	Floor, 	Wall, 	Ceiling, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeaPicklePickles { 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StainedGlassPaneSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DaylightDetectorInverted { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HayAxis { 	X, 	Y, 	Z, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KelpAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, 	Num16, 	Num17, 	Num18, 	Num19, 	Num20, 	Num21, 	Num22, 	Num23, 	Num24, 	Num25, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiquidLevel { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LightLevel { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultifaceWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripWireHookFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperBulbBlockLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovingPistonFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallHangingSignFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperGolemStatueFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IronBarsNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceGateFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChiseledBookShelfSlot4Occupied { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallBannerFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultifaceEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RespawnAnchorCharges { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetPower { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrassSnowy { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectorRailPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HugeMushroomSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkVeinWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripWireHookAttached { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChorusPlantNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperGolemStatueCopperGolemPose { 	Standing, 	Sitting, 	Running, 	Star, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlowLichenUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LightWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChorusPlantEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeepingVinesAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, 	Num16, 	Num17, 	Num18, 	Num19, 	Num20, 	Num21, 	Num22, 	Num23, 	Num24, 	Num25, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeafLitterSegmentAmount { 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmallDripleafHalf { 	Upper, 	Lower, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoratedPotWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultifaceUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlastFurnaceLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandleCakeLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparatorFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallWest { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FurnaceLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LecternFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangroveRootsWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnderChestFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HangingRootsWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmethystClusterFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperBarWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultifaceSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringLightningRodFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerHeadRotation { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StonecutterFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MossyCarpetSouth { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrushableDusted { 	Num0, 	Num1, 	Num2, 	Num3, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeetrootAge { 	Num0, 	Num1, 	Num2, 	Num3, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperBulbPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnderChestWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TallSeagrassHalf { 	Upper, 	Lower, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TntUnstable { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PressurePlatePowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DispenserTriggered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IronBarsWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChorusPlantSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BarrierWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkVeinWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FireNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrewingStandHasBottle1 { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StandingSignWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperGolemStatueWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CocoaAge { 	Num0, 	Num1, 	Num2, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoorPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultifaceDown { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperTrapDoorOpen { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FireEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BambooStalkLeaves { 	None, 	Small, 	Large, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrafterOrientation { 	DownEast, 	DownNorth, 	DownSouth, 	DownWest, 	UpEast, 	UpNorth, 	UpSouth, 	UpWest, 	WestUp, 	EastUp, 	NorthUp, 	SouthUp, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurtleEggEggs { 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoorFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObserverFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BannerRotation { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointedDripstoneThickness { 	TipMerge, 	Tip, 	Frustum, 	Middle, 	Base, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DropperFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RailShape { 	NorthSouth, 	EastWest, 	AscendingEast, 	AscendingWest, 	AscendingNorth, 	AscendingSouth, 	SouthEast, 	SouthWest, 	NorthWest, 	NorthEast, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneWallTorchLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalibratedSculkSensorSculkSensorPhase { 	Inactive, 	Active, 	Cooldown, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JackOLanternFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallSkullFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChiseledBookShelfSlot3Occupied { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TallFlowerHalf { 	Upper, 	Lower, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalibratedSculkSensorPower { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmallDripleafFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VineSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SugarCaneAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkSensorWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperSlabType { 	Top, 	Bottom, 	Double, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FireWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkSensorPower { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperDoorHalf { 	Upper, 	Lower, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperDoorFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringLanternWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangrovePropaguleAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VineNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CampfireLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StairFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BigDripleafTilt { 	None, 	Unstable, 	Partial, 	Full, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrapdoorFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandConditional { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TintedParticleLeavesPersistent { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkShriekerWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultOminous { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CeilingHangingSignAttached { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UntintedParticleLeavesPersistent { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrostedIceAge { 	Num0, 	Num1, 	Num2, 	Num3, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BedOccupied { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PitcherCropHalf { 	Upper, 	Lower, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripwireAttached { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperBulbBlockPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperBarSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LadderWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowerBedFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmallDripleafWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripwireWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VineWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlastFurnaceFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoweredRailShape { 	NorthSouth, 	EastWest, 	AscendingEast, 	AscendingWest, 	AscendingNorth, 	AscendingSouth, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrapdoorOpen { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmethystClusterWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MossyCarpetEast { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrindstoneFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitherSkullPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BarrelFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StandingSignRotation { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreakingHeartCreakingHeartState { 	Uprooted, 	Dormant, 	Awake, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttachedStemFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoralWallFanWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectorRailShape { 	NorthSouth, 	EastWest, 	AscendingEast, 	AscendingWest, 	AscendingNorth, 	AscendingSouth, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DropperTriggered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HugeMushroomWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoratedPotCracked { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnowLayerLayers { 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TorchflowerCropAge { 	Num0, 	Num1, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripwireNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoratedPotFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VineEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallSouth { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnvilFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriedGhastFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChiseledBookShelfSlot0Occupied { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShulkerBoxFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChorusPlantUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PistonBaseFacing { 	North, 	East, 	South, 	West, 	Up, 	Down, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaveVinesPlantBerries { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreakingHeartNatural { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitherWallSkullFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultifaceNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrindstoneFace { 	Floor, 	Wall, 	Ceiling, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LightningRodWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrappedChestFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BellFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrappedChestType { 	Single, 	Left, 	Right, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrialSpawnerTrialSpawnerState { 	Inactive, 	WaitingForPlayers, 	Active, 	WaitingForRewardEjection, 	EjectingReward, 	Cooldown, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperTrapDoorHalf { 	Top, 	Bottom, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CeilingHangingSignRotation { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaplingStage { 	Num0, 	Num1, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BellAttachment { 	Floor, 	Ceiling, 	SingleWall, 	DoubleWall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CocoaFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparatorMode { 	Compare, 	Subtract, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FireSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallNorth { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChiseledBookShelfFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangroveLeavesWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MossyCarpetBottom { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MossyCarpetNorth { 	None, 	Low, 	Tall, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperBarWest { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CampfireWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetherWartAge { 	Num0, 	Num1, 	Num2, 	Num3, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultifaceWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkCatalystBloom { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeafLitterFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FireAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangroveLeavesPersistent { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlowLichenSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MyceliumSnowy { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerHeadPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceGatePowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovingPistonType { 	Normal, 	Sticky, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PotatoAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BedFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BambooStalkAge { 	Num0, 	Num1, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StainedGlassPaneEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaveVinesAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, 	Num16, 	Num17, 	Num18, 	Num19, 	Num20, 	Num21, 	Num22, 	Num23, 	Num24, 	Num25, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperTrapDoorFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChorusPlantDown { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectorRailWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeverFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandleLit { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperChainAxis { 	X, 	Y, 	Z, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperBarEast { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoorHinge { 	Left, 	Right, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RailWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FarmMoisture { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandleWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarrotAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedstoneWireNorth { 	Up, 	Side, 	None, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaffoldingBottom { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SweetBerryBushAge { 	Num0, 	Num1, 	Num2, 	Num3, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurtleEggHatch { 	Num0, 	Num1, 	Num2, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PiglinwallskullFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SculkVeinNorth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BigDripleafStemWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandleCandles { 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IronBarsWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlabType { 	Top, 	Bottom, 	Double, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndPortalFrameFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonFace { 	Floor, 	Wall, 	Ceiling, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BellPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrafterCrafting { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IronBarsSouth { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainAxis { 	X, 	Y, 	Z, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MangrovePropaguleStage { 	Num0, 	Num1, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PitcherCropAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TripwireDisarmed { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoweredRailPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoorOpen { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeehiveHoneyLevel { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallSignWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrafterTriggered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallSkullPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperStairHalf { 	Top, 	Bottom, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriedGhastHydration { 	Num0, 	Num1, 	Num2, 	Num3, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HopperEnabled { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PistonBaseExtended { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmokerFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaveVinesBerries { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WallTorchFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseCoralWallFanWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseCoralWallFanFacing { 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PistonHeadType { 	Normal, 	Sticky, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChiseledBookShelfSlot5Occupied { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChestType { 	Single, 	Left, 	Right, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TwistingVinesAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, 	Num16, 	Num17, 	Num18, 	Num19, 	Num20, 	Num21, 	Num22, 	Num23, 	Num24, 	Num25, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HopperFacing { 	Down, 	North, 	South, 	West, 	East, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VineUp { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShelfWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShelfPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CactusAge { 	Num0, 	Num1, 	Num2, 	Num3, 	Num4, 	Num5, 	Num6, 	Num7, 	Num8, 	Num9, 	Num10, 	Num11, 	Num12, 	Num13, 	Num14, 	Num15, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperGolemStatueCopperGolemPose { 	Standing, 	Sitting, 	Running, 	Star, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StairShape { 	Straight, 	InnerLeft, 	InnerRight, 	OuterLeft, 	OuterRight, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LecternPowered { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LecternHasBook { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatheringCopperSlabWaterlogged { 	True, 	False, }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Property {
	SnowyDirtSnowy(SnowyDirtSnowy),
	VaultFacing(VaultFacing),
	StructureMode(StructureMode),
	MossyCarpetWest(MossyCarpetWest),
	PiglinwallskullPowered(PiglinwallskullPowered),
	TintedParticleLeavesWaterlogged(TintedParticleLeavesWaterlogged),
	WeatheringCopperStairWaterlogged(WeatheringCopperStairWaterlogged),
	SculkShriekerCanSummon(SculkShriekerCanSummon),
	CopperChestType(CopperChestType),
	BaseCoralFanWaterlogged(BaseCoralFanWaterlogged),
	TripwireEast(TripwireEast),
	LadderFacing(LadderFacing),
	WeatheringLightningRodPowered(WeatheringLightningRodPowered),
	WallHangingSignWaterlogged(WallHangingSignWaterlogged),
	WeatheringCopperChainWaterlogged(WeatheringCopperChainWaterlogged),
	ShelfSideChain(ShelfSideChain),
	EndRodFacing(EndRodFacing),
	FireUp(FireUp),
	RedstoneTorchLit(RedstoneTorchLit),
	WallEast(WallEast),
	LanternHanging(LanternHanging),
	ComposterLevel(ComposterLevel),
	GlowLichenWest(GlowLichenWest),
	BigDripleafFacing(BigDripleafFacing),
	NoteInstrument(NoteInstrument),
	ChorusPlantWest(ChorusPlantWest),
	LightningRodPowered(LightningRodPowered),
	RedstoneOreLit(RedstoneOreLit),
	SmokerLit(SmokerLit),
	CampfireFacing(CampfireFacing),
	DaylightDetectorPower(DaylightDetectorPower),
	GlowLichenNorth(GlowLichenNorth),
	MangroveLeavesDistance(MangroveLeavesDistance),
	StairWaterlogged(StairWaterlogged),
	ChiseledBookShelfSlot2Occupied(ChiseledBookShelfSlot2Occupied),
	SculkVeinSouth(SculkVeinSouth),
	HugeMushroomDown(HugeMushroomDown),
	CalibratedSculkSensorWaterlogged(CalibratedSculkSensorWaterlogged),
	FurnaceFacing(FurnaceFacing),
	ShelfFacing(ShelfFacing),
	HugeMushroomEast(HugeMushroomEast),
	InfestedRotatedPillarAxis(InfestedRotatedPillarAxis),
	WeatheringCopperBulbLit(WeatheringCopperBulbLit),
	TrapdoorWaterlogged(TrapdoorWaterlogged),
	SnifferEggHatch(SnifferEggHatch),
	WeatheringLanternHanging(WeatheringLanternHanging),
	WitherWallSkullPowered(WitherWallSkullPowered),
	CoralWallFanFacing(CoralWallFanFacing),
	WeatheringCopperChestWaterlogged(WeatheringCopperChestWaterlogged),
	EndPortalFrameEye(EndPortalFrameEye),
	WeatheringCopperGolemStatueWaterlogged(WeatheringCopperGolemStatueWaterlogged),
	WeightedPressurePlatePower(WeightedPressurePlatePower),
	ComparatorPowered(ComparatorPowered),
	UntintedParticleLeavesDistance(UntintedParticleLeavesDistance),
	MangrovePropaguleWaterlogged(MangrovePropaguleWaterlogged),
	RedstoneLampLit(RedstoneLampLit),
	StainedGlassPaneNorth(StainedGlassPaneNorth),
	LightningRodFacing(LightningRodFacing),
	TestMode(TestMode),
	FenceNorth(FenceNorth),
	StainedGlassPaneWest(StainedGlassPaneWest),
	SlabWaterlogged(SlabWaterlogged),
	FenceWaterlogged(FenceWaterlogged),
	LanternWaterlogged(LanternWaterlogged),
	TripWireHookPowered(TripWireHookPowered),
	HugeMushroomUp(HugeMushroomUp),
	BedPart(BedPart),
	WeatheringCopperChestType(WeatheringCopperChestType),
	WeatheringCopperGolemStatueFacing(WeatheringCopperGolemStatueFacing),
	PlayerWallHeadPowered(PlayerWallHeadPowered),
	RedstoneWallTorchFacing(RedstoneWallTorchFacing),
	DoorHalf(DoorHalf),
	RepeaterLocked(RepeaterLocked),
	NetherPortalAxis(NetherPortalAxis),
	SculkSensorSculkSensorPhase(SculkSensorSculkSensorPhase),
	RepeaterDelay(RepeaterDelay),
	CeilingHangingSignWaterlogged(CeilingHangingSignWaterlogged),
	FenceGateInWall(FenceGateInWall),
	WeatheringCopperTrapDoorWaterlogged(WeatheringCopperTrapDoorWaterlogged),
	CopperChestWaterlogged(CopperChestWaterlogged),
	JigsawOrientation(JigsawOrientation),
	TrialSpawnerOminous(TrialSpawnerOminous),
	BubbleColumnDrag(BubbleColumnDrag),
	UntintedParticleLeavesWaterlogged(UntintedParticleLeavesWaterlogged),
	ChestWaterlogged(ChestWaterlogged),
	HeavyCoreWaterlogged(HeavyCoreWaterlogged),
	WeatheringCopperChestFacing(WeatheringCopperChestFacing),
	WeatheringCopperGrateWaterlogged(WeatheringCopperGrateWaterlogged),
	WeatheringCopperStairFacing(WeatheringCopperStairFacing),
	GlowLichenDown(GlowLichenDown),
	TripwireSouth(TripwireSouth),
	WaterloggedTransparentWaterlogged(WaterloggedTransparentWaterlogged),
	PistonHeadShort(PistonHeadShort),
	PoweredRailWaterlogged(PoweredRailWaterlogged),
	ChestFacing(ChestFacing),
	RepeaterFacing(RepeaterFacing),
	CopperChestFacing(CopperChestFacing),
	DoublePlantHalf(DoublePlantHalf),
	BaseCoralPlantWaterlogged(BaseCoralPlantWaterlogged),
	LeverPowered(LeverPowered),
	MangrovePropaguleHanging(MangrovePropaguleHanging),
	RotatedPillarAxis(RotatedPillarAxis),
	HangingMossTip(HangingMossTip),
	SculkVeinDown(SculkVeinDown),
	CropAge(CropAge),
	RedstoneWirePower(RedstoneWirePower),
	RepeaterPowered(RepeaterPowered),
	StainedGlassPaneWaterlogged(StainedGlassPaneWaterlogged),
	FenceWest(FenceWest),
	TrapdoorHalf(TrapdoorHalf),
	ButtonPowered(ButtonPowered),
	HugeMushroomNorth(HugeMushroomNorth),
	FenceSouth(FenceSouth),
	WeatheringLightningRodWaterlogged(WeatheringLightningRodWaterlogged),
	WallSignFacing(WallSignFacing),
	SculkShriekerShrieking(SculkShriekerShrieking),
	GlazedTerracottaFacing(GlazedTerracottaFacing),
	WitherSkullRotation(WitherSkullRotation),
	TrapdoorPowered(TrapdoorPowered),
	PlayerWallHeadFacing(PlayerWallHeadFacing),
	VaultVaultState(VaultVaultState),
	WeatheringCopperTrapDoorPowered(WeatheringCopperTrapDoorPowered),
	WeatheringCopperBarNorth(WeatheringCopperBarNorth),
	BarrelOpen(BarrelOpen),
	ChorusFlowerAge(ChorusFlowerAge),
	LayeredCauldronLevel(LayeredCauldronLevel),
	TintedParticleLeavesDistance(TintedParticleLeavesDistance),
	ScaffoldingWaterlogged(ScaffoldingWaterlogged),
	BrewingStandHasBottle0(BrewingStandHasBottle0),
	WeatheringCopperDoorPowered(WeatheringCopperDoorPowered),
	NotePowered(NotePowered),
	GlowLichenWaterlogged(GlowLichenWaterlogged),
	PointedDripstoneWaterlogged(PointedDripstoneWaterlogged),
	IronBarsEast(IronBarsEast),
	CoralFanWaterlogged(CoralFanWaterlogged),
	WeatheringCopperDoorHinge(WeatheringCopperDoorHinge),
	TrappedChestWaterlogged(TrappedChestWaterlogged),
	DriedGhastWaterlogged(DriedGhastWaterlogged),
	FenceGateOpen(FenceGateOpen),
	BigDripleafWaterlogged(BigDripleafWaterlogged),
	BrewingStandHasBottle2(BrewingStandHasBottle2),
	SkullPowered(SkullPowered),
	WeatheringCopperDoorOpen(WeatheringCopperDoorOpen),
	StairHalf(StairHalf),
	GlowLichenEast(GlowLichenEast),
	JukeboxHasRecord(JukeboxHasRecord),
	NoteNote(NoteNote),
	WallUp(WallUp),
	BigDripleafStemFacing(BigDripleafStemFacing),
	ChiseledBookShelfSlot1Occupied(ChiseledBookShelfSlot1Occupied),
	WeatheringCopperStairShape(WeatheringCopperStairShape),
	StemAge(StemAge),
	ObserverPowered(ObserverPowered),
	PointedDripstoneVerticalDirection(PointedDripstoneVerticalDirection),
	RedstoneWireSouth(RedstoneWireSouth),
	ConduitWaterlogged(ConduitWaterlogged),
	RedstoneWireWest(RedstoneWireWest),
	SculkVeinEast(SculkVeinEast),
	BambooStalkStage(BambooStalkStage),
	RedstoneWireEast(RedstoneWireEast),
	DispenserFacing(DispenserFacing),
	CoralPlantWaterlogged(CoralPlantWaterlogged),
	CalibratedSculkSensorFacing(CalibratedSculkSensorFacing),
	ScaffoldingDistance(ScaffoldingDistance),
	SeaPickleWaterlogged(SeaPickleWaterlogged),
	TripwirePowered(TripwirePowered),
	CakeBites(CakeBites),
	SkullRotation(SkullRotation),
	LoomFacing(LoomFacing),
	CreakingHeartAxis(CreakingHeartAxis),
	FlowerBedFlowerAmount(FlowerBedFlowerAmount),
	SculkVeinUp(SculkVeinUp),
	BeehiveFacing(BeehiveFacing),
	PistonHeadFacing(PistonHeadFacing),
	CampfireSignalFire(CampfireSignalFire),
	LeverFace(LeverFace),
	SeaPicklePickles(SeaPicklePickles),
	StainedGlassPaneSouth(StainedGlassPaneSouth),
	DaylightDetectorInverted(DaylightDetectorInverted),
	HayAxis(HayAxis),
	KelpAge(KelpAge),
	LiquidLevel(LiquidLevel),
	LightLevel(LightLevel),
	MultifaceWest(MultifaceWest),
	TripWireHookFacing(TripWireHookFacing),
	CopperBulbBlockLit(CopperBulbBlockLit),
	MovingPistonFacing(MovingPistonFacing),
	CommandFacing(CommandFacing),
	WallHangingSignFacing(WallHangingSignFacing),
	CopperGolemStatueFacing(CopperGolemStatueFacing),
	IronBarsNorth(IronBarsNorth),
	FenceGateFacing(FenceGateFacing),
	ChiseledBookShelfSlot4Occupied(ChiseledBookShelfSlot4Occupied),
	WallBannerFacing(WallBannerFacing),
	MultifaceEast(MultifaceEast),
	RespawnAnchorCharges(RespawnAnchorCharges),
	TargetPower(TargetPower),
	GrassSnowy(GrassSnowy),
	DetectorRailPowered(DetectorRailPowered),
	HugeMushroomSouth(HugeMushroomSouth),
	SculkVeinWest(SculkVeinWest),
	TripWireHookAttached(TripWireHookAttached),
	ChorusPlantNorth(ChorusPlantNorth),
	FenceEast(FenceEast),
	WeatheringCopperGolemStatueCopperGolemPose(WeatheringCopperGolemStatueCopperGolemPose),
	GlowLichenUp(GlowLichenUp),
	LightWaterlogged(LightWaterlogged),
	ChorusPlantEast(ChorusPlantEast),
	WeepingVinesAge(WeepingVinesAge),
	LeafLitterSegmentAmount(LeafLitterSegmentAmount),
	SmallDripleafHalf(SmallDripleafHalf),
	DecoratedPotWaterlogged(DecoratedPotWaterlogged),
	MultifaceUp(MultifaceUp),
	BlastFurnaceLit(BlastFurnaceLit),
	CandleCakeLit(CandleCakeLit),
	ComparatorFacing(ComparatorFacing),
	WallWest(WallWest),
	FurnaceLit(FurnaceLit),
	LecternFacing(LecternFacing),
	MangroveRootsWaterlogged(MangroveRootsWaterlogged),
	EnderChestFacing(EnderChestFacing),
	HangingRootsWaterlogged(HangingRootsWaterlogged),
	AmethystClusterFacing(AmethystClusterFacing),
	WeatheringCopperBarWaterlogged(WeatheringCopperBarWaterlogged),
	MultifaceSouth(MultifaceSouth),
	WeatheringLightningRodFacing(WeatheringLightningRodFacing),
	PlayerHeadRotation(PlayerHeadRotation),
	StonecutterFacing(StonecutterFacing),
	MossyCarpetSouth(MossyCarpetSouth),
	BrushableDusted(BrushableDusted),
	BeetrootAge(BeetrootAge),
	WeatheringCopperBulbPowered(WeatheringCopperBulbPowered),
	EnderChestWaterlogged(EnderChestWaterlogged),
	TallSeagrassHalf(TallSeagrassHalf),
	TntUnstable(TntUnstable),
	PressurePlatePowered(PressurePlatePowered),
	DispenserTriggered(DispenserTriggered),
	IronBarsWest(IronBarsWest),
	ChorusPlantSouth(ChorusPlantSouth),
	BarrierWaterlogged(BarrierWaterlogged),
	SculkVeinWaterlogged(SculkVeinWaterlogged),
	FireNorth(FireNorth),
	BrewingStandHasBottle1(BrewingStandHasBottle1),
	StandingSignWaterlogged(StandingSignWaterlogged),
	CopperGolemStatueWaterlogged(CopperGolemStatueWaterlogged),
	CocoaAge(CocoaAge),
	DoorPowered(DoorPowered),
	MultifaceDown(MultifaceDown),
	WeatheringCopperTrapDoorOpen(WeatheringCopperTrapDoorOpen),
	FireEast(FireEast),
	BambooStalkLeaves(BambooStalkLeaves),
	CrafterOrientation(CrafterOrientation),
	TurtleEggEggs(TurtleEggEggs),
	DoorFacing(DoorFacing),
	ObserverFacing(ObserverFacing),
	BannerRotation(BannerRotation),
	PointedDripstoneThickness(PointedDripstoneThickness),
	DropperFacing(DropperFacing),
	RailShape(RailShape),
	RedstoneWallTorchLit(RedstoneWallTorchLit),
	CalibratedSculkSensorSculkSensorPhase(CalibratedSculkSensorSculkSensorPhase),
	JackOLanternFacing(JackOLanternFacing),
	WallSkullFacing(WallSkullFacing),
	ChiseledBookShelfSlot3Occupied(ChiseledBookShelfSlot3Occupied),
	TallFlowerHalf(TallFlowerHalf),
	CalibratedSculkSensorPower(CalibratedSculkSensorPower),
	SmallDripleafFacing(SmallDripleafFacing),
	VineSouth(VineSouth),
	SugarCaneAge(SugarCaneAge),
	SculkSensorWaterlogged(SculkSensorWaterlogged),
	WeatheringCopperSlabType(WeatheringCopperSlabType),
	FireWest(FireWest),
	SculkSensorPower(SculkSensorPower),
	WeatheringCopperDoorHalf(WeatheringCopperDoorHalf),
	WeatheringCopperDoorFacing(WeatheringCopperDoorFacing),
	WeatheringLanternWaterlogged(WeatheringLanternWaterlogged),
	MangrovePropaguleAge(MangrovePropaguleAge),
	VineNorth(VineNorth),
	CampfireLit(CampfireLit),
	StairFacing(StairFacing),
	BigDripleafTilt(BigDripleafTilt),
	TrapdoorFacing(TrapdoorFacing),
	CommandConditional(CommandConditional),
	TintedParticleLeavesPersistent(TintedParticleLeavesPersistent),
	SculkShriekerWaterlogged(SculkShriekerWaterlogged),
	VaultOminous(VaultOminous),
	CeilingHangingSignAttached(CeilingHangingSignAttached),
	UntintedParticleLeavesPersistent(UntintedParticleLeavesPersistent),
	FrostedIceAge(FrostedIceAge),
	BedOccupied(BedOccupied),
	PitcherCropHalf(PitcherCropHalf),
	TripwireAttached(TripwireAttached),
	CopperBulbBlockPowered(CopperBulbBlockPowered),
	WeatheringCopperBarSouth(WeatheringCopperBarSouth),
	LadderWaterlogged(LadderWaterlogged),
	FlowerBedFacing(FlowerBedFacing),
	SmallDripleafWaterlogged(SmallDripleafWaterlogged),
	TripwireWest(TripwireWest),
	VineWest(VineWest),
	BlastFurnaceFacing(BlastFurnaceFacing),
	PoweredRailShape(PoweredRailShape),
	TrapdoorOpen(TrapdoorOpen),
	AmethystClusterWaterlogged(AmethystClusterWaterlogged),
	MossyCarpetEast(MossyCarpetEast),
	GrindstoneFacing(GrindstoneFacing),
	WitherSkullPowered(WitherSkullPowered),
	BarrelFacing(BarrelFacing),
	StandingSignRotation(StandingSignRotation),
	CreakingHeartCreakingHeartState(CreakingHeartCreakingHeartState),
	AttachedStemFacing(AttachedStemFacing),
	CoralWallFanWaterlogged(CoralWallFanWaterlogged),
	DetectorRailShape(DetectorRailShape),
	DropperTriggered(DropperTriggered),
	HugeMushroomWest(HugeMushroomWest),
	DecoratedPotCracked(DecoratedPotCracked),
	SnowLayerLayers(SnowLayerLayers),
	TorchflowerCropAge(TorchflowerCropAge),
	TripwireNorth(TripwireNorth),
	DecoratedPotFacing(DecoratedPotFacing),
	VineEast(VineEast),
	WallSouth(WallSouth),
	AnvilFacing(AnvilFacing),
	DriedGhastFacing(DriedGhastFacing),
	ChiseledBookShelfSlot0Occupied(ChiseledBookShelfSlot0Occupied),
	ShulkerBoxFacing(ShulkerBoxFacing),
	ChorusPlantUp(ChorusPlantUp),
	PistonBaseFacing(PistonBaseFacing),
	CaveVinesPlantBerries(CaveVinesPlantBerries),
	CreakingHeartNatural(CreakingHeartNatural),
	WitherWallSkullFacing(WitherWallSkullFacing),
	MultifaceNorth(MultifaceNorth),
	GrindstoneFace(GrindstoneFace),
	LightningRodWaterlogged(LightningRodWaterlogged),
	TrappedChestFacing(TrappedChestFacing),
	BellFacing(BellFacing),
	TrappedChestType(TrappedChestType),
	TrialSpawnerTrialSpawnerState(TrialSpawnerTrialSpawnerState),
	WeatheringCopperTrapDoorHalf(WeatheringCopperTrapDoorHalf),
	CeilingHangingSignRotation(CeilingHangingSignRotation),
	SaplingStage(SaplingStage),
	BellAttachment(BellAttachment),
	CocoaFacing(CocoaFacing),
	ComparatorMode(ComparatorMode),
	FireSouth(FireSouth),
	ButtonFacing(ButtonFacing),
	WallNorth(WallNorth),
	ChiseledBookShelfFacing(ChiseledBookShelfFacing),
	MangroveLeavesWaterlogged(MangroveLeavesWaterlogged),
	MossyCarpetBottom(MossyCarpetBottom),
	MossyCarpetNorth(MossyCarpetNorth),
	WeatheringCopperBarWest(WeatheringCopperBarWest),
	CampfireWaterlogged(CampfireWaterlogged),
	NetherWartAge(NetherWartAge),
	MultifaceWaterlogged(MultifaceWaterlogged),
	SculkCatalystBloom(SculkCatalystBloom),
	LeafLitterFacing(LeafLitterFacing),
	FireAge(FireAge),
	MangroveLeavesPersistent(MangroveLeavesPersistent),
	GlowLichenSouth(GlowLichenSouth),
	MyceliumSnowy(MyceliumSnowy),
	PlayerHeadPowered(PlayerHeadPowered),
	FenceGatePowered(FenceGatePowered),
	MovingPistonType(MovingPistonType),
	PotatoAge(PotatoAge),
	BedFacing(BedFacing),
	BambooStalkAge(BambooStalkAge),
	StainedGlassPaneEast(StainedGlassPaneEast),
	CaveVinesAge(CaveVinesAge),
	WeatheringCopperTrapDoorFacing(WeatheringCopperTrapDoorFacing),
	ChorusPlantDown(ChorusPlantDown),
	DetectorRailWaterlogged(DetectorRailWaterlogged),
	LeverFacing(LeverFacing),
	CandleLit(CandleLit),
	WeatheringCopperChainAxis(WeatheringCopperChainAxis),
	WeatheringCopperBarEast(WeatheringCopperBarEast),
	DoorHinge(DoorHinge),
	RailWaterlogged(RailWaterlogged),
	FarmMoisture(FarmMoisture),
	CandleWaterlogged(CandleWaterlogged),
	CarrotAge(CarrotAge),
	RedstoneWireNorth(RedstoneWireNorth),
	ScaffoldingBottom(ScaffoldingBottom),
	WallWaterlogged(WallWaterlogged),
	SweetBerryBushAge(SweetBerryBushAge),
	TurtleEggHatch(TurtleEggHatch),
	PiglinwallskullFacing(PiglinwallskullFacing),
	SculkVeinNorth(SculkVeinNorth),
	BigDripleafStemWaterlogged(BigDripleafStemWaterlogged),
	CandleCandles(CandleCandles),
	IronBarsWaterlogged(IronBarsWaterlogged),
	SlabType(SlabType),
	EndPortalFrameFacing(EndPortalFrameFacing),
	ButtonFace(ButtonFace),
	BellPowered(BellPowered),
	CrafterCrafting(CrafterCrafting),
	IronBarsSouth(IronBarsSouth),
	ChainAxis(ChainAxis),
	MangrovePropaguleStage(MangrovePropaguleStage),
	PitcherCropAge(PitcherCropAge),
	TripwireDisarmed(TripwireDisarmed),
	PoweredRailPowered(PoweredRailPowered),
	DoorOpen(DoorOpen),
	BeehiveHoneyLevel(BeehiveHoneyLevel),
	WallSignWaterlogged(WallSignWaterlogged),
	CrafterTriggered(CrafterTriggered),
	WallSkullPowered(WallSkullPowered),
	WeatheringCopperStairHalf(WeatheringCopperStairHalf),
	DriedGhastHydration(DriedGhastHydration),
	HopperEnabled(HopperEnabled),
	ChainWaterlogged(ChainWaterlogged),
	PistonBaseExtended(PistonBaseExtended),
	SmokerFacing(SmokerFacing),
	CaveVinesBerries(CaveVinesBerries),
	WallTorchFacing(WallTorchFacing),
	BaseCoralWallFanWaterlogged(BaseCoralWallFanWaterlogged),
	BaseCoralWallFanFacing(BaseCoralWallFanFacing),
	PistonHeadType(PistonHeadType),
	ChiseledBookShelfSlot5Occupied(ChiseledBookShelfSlot5Occupied),
	ChestType(ChestType),
	TwistingVinesAge(TwistingVinesAge),
	HopperFacing(HopperFacing),
	VineUp(VineUp),
	ShelfWaterlogged(ShelfWaterlogged),
	ShelfPowered(ShelfPowered),
	CactusAge(CactusAge),
	CopperGolemStatueCopperGolemPose(CopperGolemStatueCopperGolemPose),
	StairShape(StairShape),
	LecternPowered(LecternPowered),
	LecternHasBook(LecternHasBook),
	WeatheringCopperSlabWaterlogged(WeatheringCopperSlabWaterlogged),
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
