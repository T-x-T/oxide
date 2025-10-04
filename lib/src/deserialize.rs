use std::error::Error;
use crate::types::*;
use crate::CustomError;

pub fn boolean(data: &mut Vec<u8>) -> Result<bool, Box<dyn Error>> {
  if data.is_empty() {
    return Err(Box::new(CustomError::InputEmpty));
  }

  let value = data.remove(0);

  return match value {
    0x00 => Ok(false),
    0x01 => Ok(true),
    _ => Err(Box::new(CustomError::DeserializeInvalidBoolean(value))),
  }
}

pub fn unsigned_short(data: &mut Vec<u8>) -> Result<u16, Box<dyn Error>> {
  if data.len() < 2 {
    return Err(Box::new(CustomError::InputEmpty));
  }

  let first_byte = data.remove(0);
  let second_byte = data.remove(0);

  let output: u16 = (first_byte as u16 * 256) + second_byte as u16;

  return Ok(output);
}

pub fn short(data: &mut Vec<u8>) -> Result<i16, Box<dyn Error>> {
  if data.len() < 2 {
    return Err(Box::new(CustomError::InputEmpty));
  }

  let drained_data = data.drain(0..2);
  let output: i16 = i16::from_be_bytes(drained_data.as_slice().try_into()?);
  return Ok(output);
}

pub fn short_le(data: &mut Vec<u8>) -> Result<i16, Box<dyn Error>> {
  if data.len() < 2 {
    return Err(Box::new(CustomError::InputEmpty));
  }

  let drained_data = data.drain(data.len()-2..);
  let output: i16 = i16::from_le_bytes(drained_data.as_slice().try_into()?);
  return Ok(output);
}

pub fn int(data: &mut Vec<u8>) -> Result<i32, Box<dyn Error>> {
  if data.len() < 4 {
    return Err(Box::new(CustomError::InputEmpty));
  }

	let drained_data = data.drain(0..4);
  let output: i32 = i32::from_be_bytes(drained_data.as_slice().try_into()?);
  return Ok(output);
}

pub fn int_le(data: &mut Vec<u8>) -> Result<i32, Box<dyn Error>> {
  if data.len() < 4 {
    return Err(Box::new(CustomError::InputEmpty));
  }

	let drained_data = data.drain(data.len()-4..);
  let output: i32 = i32::from_le_bytes(drained_data.as_slice().try_into()?);
  return Ok(output);
}

pub fn long(data: &mut Vec<u8>) -> Result<i64, Box<dyn Error>> {
  if data.len() < 8 {
    return Err(Box::new(CustomError::InputEmpty));
  }

	let drained_data = data.drain(0..8);
  let output: i64 = i64::from_be_bytes(drained_data.as_slice().try_into()?);
  return Ok(output);
}

pub fn long_le(data: &mut Vec<u8>) -> Result<i64, Box<dyn Error>> {
  if data.len() < 8 {
    return Err(Box::new(CustomError::InputEmpty));
  }

	let drained_data = data.drain(data.len()-8..);
  let output: i64 = i64::from_le_bytes(drained_data.as_slice().try_into()?);
  return Ok(output);
}

pub fn unsigned_long(data: &mut Vec<u8>) -> Result<u64, Box<dyn Error>> {
  if data.len() < 8 {
    return Err(Box::new(CustomError::InputEmpty));
  }

	let drained_data = data.drain(0..8);
  let output: u64 = u64::from_be_bytes(drained_data.as_slice().try_into()?);
  return Ok(output);
}

pub fn double(data: &mut Vec<u8>) -> Result<f64, Box<dyn Error>> {
  if data.len() < 8 {
    return Err(Box::new(CustomError::InputEmpty));
  }

	let drained_data = data.drain(0..8);
  let output: f64 = f64::from_be_bytes(drained_data.as_slice().try_into()?);
  return Ok(output);
}

pub fn double_le(data: &mut Vec<u8>) -> Result<f64, Box<dyn Error>> {
  if data.len() < 8 {
    return Err(Box::new(CustomError::InputEmpty));
  }

	let drained_data = data.drain(data.len()-8..);
  let output: f64 = f64::from_le_bytes(drained_data.as_slice().try_into()?);
  return Ok(output);
}

pub fn float(data: &mut Vec<u8>) -> Result<f32, Box<dyn Error>> {
  if data.len() < 4 {
    return Err(Box::new(CustomError::InputEmpty));
  }

	let drained_data = data.drain(0..4);
  let output: f32 = f32::from_be_bytes(drained_data.as_slice().try_into()?);
  return Ok(output);
}

pub fn float_le(data: &mut Vec<u8>) -> Result<f32, Box<dyn Error>> {
  if data.len() < 4 {
    return Err(Box::new(CustomError::InputEmpty));
  }

	let drained_data = data.drain(data.len()-4..);
  let output: f32 = f32::from_le_bytes(drained_data.as_slice().try_into()?);
  return Ok(output);
}

pub fn uuid(data: &mut Vec<u8>) -> Result<u128, Box<dyn Error>> {
  if data.len() < 16 {
    return Err(Box::new(CustomError::InputEmpty));
  }

	let drained_data = data.drain(0..16);
  let output: u128 = u128::from_be_bytes(drained_data.as_slice().try_into()?);
  return Ok(output);
}

pub fn string(data: &mut Vec<u8>) -> Result<String, Box<dyn Error>> {
  let length = varint(data)?;
  if data.len() < length as usize {
    return Err(Box::new(CustomError::InputEmpty));
  }
  let raw_string: &[u8] = &data.clone()[..length as usize];
  data.drain(..length as usize);

  return Ok(String::from_utf8(raw_string.to_vec())?);
}

pub fn bitset(data: &mut Vec<u8>) -> Result<Vec<u64>, Box<dyn Error>> {
  let length = varint(data)?;
  let mut output: Vec<u64> = Vec::new();
  for _ in 0..length {
    output.push(unsigned_long(data)?);
  }
  return Ok(output);
}

pub fn position(data: &mut Vec<u8>) -> Result<crate::types::position::Position, Box<dyn Error>> {
  let raw = long(data)?;
  return Ok(crate::types::position::Position {
    x: (raw >> 38) as i32,
    y: (raw << 52 >> 52) as i16,
    z: (raw << 26 >> 38) as i32,
  });
}

pub fn hashed_slot(data: &mut Vec<u8>) -> Result<Option<Slot>, Box<dyn Error>> {
  let has_item = boolean(data)?;

  if !has_item {
    return Ok(None);
  }

  let item_id = varint(data)?;
  let item_count = varint(data)?;

  let components_to_add_len = varint(data)?;
  let mut components_to_add: Vec<(i32, i32)> = Vec::new(); //(varint, int)
  for _ in 0..components_to_add_len {
    components_to_add.push((varint(data)?, int(data)?));
  }

  let components_to_remove_len = varint(data)?;
  let mut components_to_remove: Vec<i32> = Vec::new();
  for _ in 0..components_to_remove_len {
    components_to_remove.push(varint(data)?);
  }

  //might have to do something about the components_to_add but probably not(?)
  return Ok(Some(Slot { item_count, item_id, components_to_add: Vec::new(), components_to_remove }));
}

pub fn slot(data: &mut Vec<u8>) -> Result<Option<Slot>, Box<dyn Error>> {
  let item_count = varint(data)?;

  if item_count == 0 {
    return Ok(None);
  }

  let item_id = varint(data)?;
  let number_of_components_to_add = varint(data)?;
  let number_of_components_to_remove = varint(data)?;

  let mut components_to_add: Vec<SlotComponent> = Vec::new();
  for _ in 0..number_of_components_to_add {
    let component_id = varint(data)?;
    components_to_add.push(match component_id {
      0 => SlotComponent::CustomData(nbt_network(data)?),
      1 => SlotComponent::MaxStackSize(varint(data)?),
      2 => SlotComponent::MaxDamage(varint(data)?),
      3 => SlotComponent::Damage(varint(data)?),
      4 => SlotComponent::Unbreakable,
      5 => SlotComponent::CustomName(nbt_network(data)?),
      6 => SlotComponent::ItemName(nbt_network(data)?),
      7 => SlotComponent::ItemModel(string(data)?),
      8 => SlotComponent::Lore((0..varint(data)?).map(|_| nbt_network(data).unwrap()).collect()),
      9 => SlotComponent::Rarity(data.remove(0)),
      10 => SlotComponent::Enchantments((0..varint(data)?).map(|_| (varint(data).unwrap(), varint(data).unwrap())).collect()),
      //11 => todo!(), //SlotComponent::CanPlaceOn,
      //12 => todo!(), //SlotComponent::CanBreak,
      //13 => todo!(), //SlotComponent::AttributeModifiers,
      14 => SlotComponent::CustomModelData((0..varint(data)?).map(|_| float(data).unwrap()).collect(), (0..varint(data)?).map(|_| boolean(data).unwrap()).collect(), (0..varint(data)?).map(|_| string(data).unwrap()).collect(), (0..varint(data)?).map(|_| int(data).unwrap()).collect()),
      15 => SlotComponent::TooltipDisplay(boolean(data)?, (0..varint(data)?).map(|_| varint(data).unwrap()).collect()),
      16 => SlotComponent::RepairCost(varint(data)?),
      17 => SlotComponent::CreativeSlotLock,
      18 => SlotComponent::EnchantmentGlintOverride(boolean(data)?),
      19 => SlotComponent::IntangibleProjectile(nbt_network(data)?),
      20 => SlotComponent::Food(varint(data)?, float(data)?, boolean(data)?),
      //21 => todo!(), //SlotComponent::Consumable,
      22 => SlotComponent::UseRemainder(slot(data)?),
      23 => SlotComponent::UseCooldown(float(data)?, if boolean(data)? {Some(string(data)?)} else {None}),
      24 => SlotComponent::DamageResistant(string(data)?),
      //25 => todo!(), //SlotComponent::Tool,
      26 => SlotComponent::Weapon(varint(data)?, float(data)?),
      27 => SlotComponent::Enchantable(varint(data)?),
      //28 => todo!(), //SlotComponent::Equippable,
      //29 => todo!(), //SlotComponent::Repairable,
      30 => SlotComponent::Glider,
      31 => SlotComponent::TooltipStyle(string(data)?),
      //32 => todo!(), //SlotComponent::DeathProtection,
      //33 => todo!(), //SlotComponent::BlockAttacks,
      34 => SlotComponent::StoredEnchantments((0..varint(data)?).map(|_| (varint(data).unwrap(), varint(data).unwrap())).collect()),
      35 => SlotComponent::DyedColor(int(data)?),
      36 => SlotComponent::MapColor(int(data)?),
      37 => SlotComponent::MapId(varint(data)?),
      38 => SlotComponent::MapDecorations(nbt_network(data)?),
      39 => SlotComponent::MapPostProcessing(data.remove(0)),
      40 => SlotComponent::ChargedProjectiles((0..varint(data)?).map(|_| slot(data).unwrap()).collect()),
      41 => SlotComponent::BundleContents((0..varint(data)?).map(|_| slot(data).unwrap()).collect()),
      //42 => todo!(), //SlotComponent::PotionContents,
      43 => SlotComponent::PotionDurationScale(float(data)?),
      44 => SlotComponent::SuspiciousStewEffects((0..varint(data)?).map(|_| (varint(data).unwrap(), varint(data).unwrap())).collect()),
      45 => SlotComponent::WritableBookContent((0..varint(data)?).map(|_| (string(data).unwrap(), if boolean(data).unwrap() {Some(string(data).unwrap())} else {None})).collect()),
      46 => SlotComponent::WrittenBookContent((0..varint(data)?).map(|_| (string(data).unwrap(), if boolean(data).unwrap() {Some(string(data).unwrap())} else {None})).collect()),
      //47 => todo!(), //SlotComponent::Trim,
      48 => SlotComponent::DebugStickState(nbt_network(data)?),
      49 => SlotComponent::EntityData(nbt_network(data)?),
      50 => SlotComponent::BucketEntityData(nbt_network(data)?),
      51 => SlotComponent::BlockEntityData(nbt_network(data)?),
      //52 => todo!(), //SlotComponent::Instrument,
      //53 => todo!(), //SlotComponent::ProvidesTrimMaterial,
      54 => SlotComponent::OminousBottleAmplifier(data.remove(0)),
      //55 => todo!(), //SlotComponent::JukeboxPlayable,
      56 => SlotComponent::ProvidesBannerPatterns(string(data)?),
      57 => SlotComponent::Recipes(nbt_network(data)?),
      58 => SlotComponent::LodestoneTracker(boolean(data)?, string(data)?, position(data)?, boolean(data)?),
      //59 => todo!(), //SlotComponent::FireworkExplosion,
      //60 => todo!(), //SlotComponent::Fireworks,
      61 => SlotComponent::Profile(if boolean(data)? {Some(string(data)?)} else {None}, if boolean(data)? {Some(uuid(data)?)} else {None}, (0..varint(data)?).map(|_| (string(data).unwrap(), string(data).unwrap(), if boolean(data).unwrap() {Some(string(data).unwrap())} else {None})).collect()),
      62 => SlotComponent::NoteblockSound(string(data)?),
      //63 => todo!(), //SlotComponent::BannerPatterns,
      64 => SlotComponent::BaseColor(data.remove(0)),
      65 => SlotComponent::PotDecorations((0..varint(data)?).map(|_| varint(data).unwrap()).collect()),
      66 => SlotComponent::Container((0..varint(data)?).map(|_| varint(data).unwrap()).collect()),
      67 => SlotComponent::BlockState((0..varint(data)?).map(|_| (string(data).unwrap(), string(data).unwrap())).collect()),
      68 => SlotComponent::Bees((0..varint(data)?).map(|_| (nbt_network(data).unwrap(), varint(data).unwrap(), varint(data).unwrap())).collect()),
      69 => SlotComponent::Lock(nbt_network(data)?),
      70 => SlotComponent::ContainerLoot(nbt_network(data)?),
      //71 => todo!(), //SlotComponent::BreakSound,
      //72 => todo!(), //SlotComponent::VillagerVariant,
      //73 => todo!(), //SlotComponent::WolfVariant,
      //74 => todo!(), //SlotComponent::WolfSoundVariant,
      75 => SlotComponent::WolfCollar(data.remove(0)),
      76 => SlotComponent::FoxVariant(data.remove(0)),
      77 => SlotComponent::SalmonSize(data.remove(0)),
      78 => SlotComponent::ParrotVariant(varint(data)?),
      79 => SlotComponent::TropicalFishPattern(data.remove(0)),
      80 => SlotComponent::TropicalFishBaseColor(data.remove(0)),
      81 => SlotComponent::TropicalFishPatternColor(data.remove(0)),
      82 => SlotComponent::MooshroomVariant(data.remove(0)),
      83 => SlotComponent::RabbitVariant(data.remove(0)),
      84 => SlotComponent::PigVariant(data.remove(0)),
      85 => SlotComponent::CowVariant(data.remove(0)),
      //86 => todo!(), //SlotComponent::ChickenVariant,
      87 => SlotComponent::FrogVariant(varint(data)?),
      88 => SlotComponent::HorseVariant(data.remove(0)),
      //89 => todo!(), //SlotComponent::PaintingVariant,
      90 => SlotComponent::LlamaVariant(data.remove(0)),
      91 => SlotComponent::AxolotlVariant(data.remove(0)),
      92 => SlotComponent::CatVariant(varint(data)?),
      93 => SlotComponent::CatCollar(data.remove(0)),
      94 => SlotComponent::SheepColor(data.remove(0)),
      95 => SlotComponent::ShulkerColor(data.remove(0)),
      x => {
        println!("I cant deserialize the SlotComponent with id {x}, because I dont know it");
        return Ok(None);
      },
    });
  }
  let mut components_to_remove: Vec<i32> = Vec::new();
  for _ in 0..number_of_components_to_remove {
    components_to_remove.push(varint(data)?);
  }

  return Ok(Some(Slot {
    item_count,
    item_id,
    components_to_add,
    components_to_remove,
  }));
}

const SEGMENT_BITS: u8 = 0b0111_1111;
const CONTINUE_BIT: u8 = 0b1000_0000;

pub fn varint(data: &mut Vec<u8>) -> Result<i32, Box<dyn Error>> {
  if data.is_empty() {
    return Err(Box::new(crate::CustomError::InputEmpty));
  }

  let mut value: i32 = 0;
  let mut position = 0;
  let mut current_byte: u8;

  loop {
    current_byte = data.remove(0);
    value |= (current_byte as i32 & SEGMENT_BITS as i32) << position;

    if (current_byte & CONTINUE_BIT) == 0 {
      break;
    }

    position += 7;

    if position >= 32 {
      return Err(Box::new(CustomError::ParseVarIntTooBig));
    }
  }
  return Ok(value);
}


pub fn nbt_network(data: &mut Vec<u8>) -> Result<NbtTag, Box<dyn Error>> {
	data.reverse();
	let root_id = data.pop().unwrap();
	if root_id != 0x0a {
	  return Err(Box::new(crate::CustomError::InvalidInput(format!("nbt_disk got a wrong id for the root tag: {root_id}"))));
	}

  let mut tags: Vec<NbtTag> = Vec::new();

  loop {
    let id = data.pop().unwrap();
    match id {
      0x00 => break,
      0x01 => {
        tags.push(NbtTag::Byte(nbt_string_value(data)?, data.pop().unwrap()));
      },
      0x02 => {
        tags.push(NbtTag::Short(nbt_string_value(data)?, short_le(data)?));
      },
      0x03 => {
        tags.push(NbtTag::Int(nbt_string_value(data)?, int_le(data)?));
      },
      0x04 => {
        tags.push(NbtTag::Long(nbt_string_value(data)?, long_le(data)?));
      },
      0x05 => {
        tags.push(NbtTag::Float(nbt_string_value(data)?, float_le(data)?));
      },
      0x06 => {
        tags.push(NbtTag::Double(nbt_string_value(data)?, double_le(data)?));
      },
      0x07 => {
        tags.push(NbtTag::ByteArray(nbt_string_value(data)?, nbt_byte_array_value(data)?));
      },
      0x08 => {
        tags.push(NbtTag::String(nbt_string_value(data)?, nbt_string_value(data)?));
      },
      0x09 => {
        tags.push(nbt_list(data)?);
      },
      0x0a => {
        tags.push(nbt_tag_compound(data)?);
      },
      0x0b => {
        tags.push(NbtTag::IntArray(nbt_string_value(data)?, nbt_int_array_value(data)?));
      },
      0x0c => {
        tags.push(NbtTag::LongArray(nbt_string_value(data)?, nbt_long_array_value(data)?));
      },
      x => {
        return Err(Box::new(CustomError::InvalidNbtTag(x)));
      }
    };
  }

  let output = NbtTag::Root(tags);
  data.reverse();
	return Ok(output);
}

pub fn nbt_disk(data: &mut Vec<u8>) -> Result<NbtTag, Box<dyn Error>> {
	data.reverse();
	let root_id = data.pop().unwrap();
	if root_id != 0x0a {
	  return Err(Box::new(crate::CustomError::InvalidInput(format!("nbt_disk got a wrong id for the root tag: {root_id}"))));
	}
  let _: String = nbt_string_value(data)?;

  let mut tags: Vec<NbtTag> = Vec::new();

  loop {
    let id = data.pop().unwrap();
    match id {
      0x00 => break,
      0x01 => {
        tags.push(NbtTag::Byte(nbt_string_value(data)?, data.pop().unwrap()));
      },
      0x02 => {
        tags.push(NbtTag::Short(nbt_string_value(data)?, short_le(data)?));
      },
      0x03 => {
        tags.push(NbtTag::Int(nbt_string_value(data)?, int_le(data)?));
      },
      0x04 => {
        tags.push(NbtTag::Long(nbt_string_value(data)?, long_le(data)?));
      },
      0x05 => {
        tags.push(NbtTag::Float(nbt_string_value(data)?, float_le(data)?));
      },
      0x06 => {
        tags.push(NbtTag::Double(nbt_string_value(data)?, double_le(data)?));
      },
      0x07 => {
        tags.push(NbtTag::ByteArray(nbt_string_value(data)?, nbt_byte_array_value(data)?));
      },
      0x08 => {
        tags.push(NbtTag::String(nbt_string_value(data)?, nbt_string_value(data)?));
      },
      0x09 => {
        tags.push(nbt_list(data)?);
      },
      0x0a => {
        tags.push(nbt_tag_compound(data)?);
      },
      0x0b => {
        tags.push(NbtTag::IntArray(nbt_string_value(data)?, nbt_int_array_value(data)?));
      },
      0x0c => {
        tags.push(NbtTag::LongArray(nbt_string_value(data)?, nbt_long_array_value(data)?));
      },
      x => {
        return Err(Box::new(CustomError::InvalidNbtTag(x)));
      }
    };
  }

  let output = NbtTag::Root(tags);
	data.reverse();
  return Ok(output);
}

fn nbt_byte_array_value(data: &mut Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
  let len = int_le(data)?;
  let mut bytes: Vec<u8> = Vec::new();
  for _ in 0..len {
    bytes.push(data.pop().unwrap());
  }
  return Ok(bytes);
}

pub fn nbt_string_value(data: &mut Vec<u8>) -> Result<String, Box<dyn Error>> {
  let len = short_le(data)?;

  let drained_data = data.drain(data.len()-len as usize..);
  let string = String::from_utf8(drained_data.rev().collect())?;

  return Ok(string);
}

fn nbt_list(data: &mut Vec<u8>) -> Result<NbtTag, Box<dyn Error>> {
  let description: String = nbt_string_value(data)?;

  let id = data.pop().unwrap();
  let len = int_le(data)?;

  if len == 0 {
  	return Ok(NbtTag::List(description, Vec::new()));
  }

  let output: NbtTag = match id {
    0x01 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::Byte(data.pop().unwrap()));
      }

      NbtTag::List(description, list)
    },
    0x02 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::Short(short_le(data)?));
      }

      NbtTag::List(description, list)
    },
    0x03 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::Int(int_le(data)?));
      }

      NbtTag::List(description, list)
    },
    0x04 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::Long(long_le(data)?));
      }

      NbtTag::List(description, list)
    },
    0x05 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::Float(float_le(data)?));
      }

      NbtTag::List(description, list)
    },
    0x06 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::Double(double_le(data)?));
      }

      NbtTag::List(description, list)
    },
    0x07 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::ByteArray(nbt_byte_array_value(data)?));
      }

      NbtTag::List(description, list)
    },
    0x08 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::String(nbt_string_value(data)?));
      }

      NbtTag::List(description, list)
    },
    0x09 => {
    	let mut list: Vec<NbtListTag> = Vec::new();
			for _ in 0..len {
				list.push(nbt_list_list(data).unwrap());
			}
			NbtTag::List(description, list)
    },
    0x0a => {
	    let mut list: Vec<NbtListTag> = Vec::new();
			for _ in 0..len {
				list.push(nbt_tag_compound_list(data).unwrap());
			}
			NbtTag::List(description, list)
    },
    0x0b => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::IntArray(nbt_int_array_value(data)?));
      }

      NbtTag::List(description, list)
    },
    0x0c => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::LongArray(nbt_long_array_value(data)?));
      }

      NbtTag::List(description, list)
    },
    x => {
      return Err(Box::new(CustomError::InvalidNbtTag(x)));
    }
  };

  return Ok(output);
}

fn nbt_list_list(data: &mut Vec<u8>) -> Result<NbtListTag, Box<dyn Error>> {
  let id = data.pop().unwrap();
  let len = int_le(data)?;

  if len == 0 {
  	return Ok(NbtListTag::List(Vec::new()));
  }

  let output: NbtListTag = match id {
    0x01 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::Byte(data.pop().unwrap()));
      }

      NbtListTag::List(list)
    },
    0x02 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::Short(short_le(data)?));
      }

      NbtListTag::List(list)
    },
    0x03 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::Int(int_le(data)?));
      }

      NbtListTag::List(list)
    },
    0x04 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::Long(long_le(data)?));
      }

      NbtListTag::List(list)
    },
    0x05 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::Float(float_le(data)?));
      }

      NbtListTag::List(list)
    },
    0x06 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::Double(double_le(data)?));
      }

      NbtListTag::List(list)
    },
    0x07 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::ByteArray(nbt_byte_array_value(data)?));
      }

      NbtListTag::List(list)
    },
    0x08 => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::String(nbt_string_value(data)?));
      }

      NbtListTag::List(list)
    },
    0x09 => {
    	let mut list: Vec<NbtListTag> = Vec::new();
			for _ in 0..len {
				list.push(nbt_list_list(data).unwrap());
			}
			NbtListTag::List(list)
    },
    0x0a => {
	    let mut list: Vec<NbtListTag> = Vec::new();
			for _ in 0..len {
				list.push(nbt_tag_compound_list(data).unwrap());
			}
			NbtListTag::List(list)
    },
    0x0b => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::IntArray(nbt_int_array_value(data)?));
      }

      NbtListTag::List(list)
    },
    0x0c => {
      let mut list: Vec<NbtListTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtListTag::LongArray(nbt_long_array_value(data)?));
      }

      NbtListTag::List(list)
    },
    x => {
      return Err(Box::new(CustomError::InvalidNbtTag(x)));
    }
  };

  return Ok(output);
}

fn nbt_tag_compound(data: &mut Vec<u8>) -> Result<NbtTag, Box<dyn Error>> {
  let description: String = nbt_string_value(data)?;

  let mut tags: Vec<NbtTag> = Vec::new();

  loop {
    let id = data.pop().unwrap();
    match id {
      0x00 => break,
      0x01 => {
        tags.push(NbtTag::Byte(nbt_string_value(data)?, data.pop().unwrap()));
      },
      0x02 => {
        tags.push(NbtTag::Short(nbt_string_value(data)?, short_le(data)?));
      },
      0x03 => {
        tags.push(NbtTag::Int(nbt_string_value(data)?, int_le(data)?));
      },
      0x04 => {
        tags.push(NbtTag::Long(nbt_string_value(data)?, long_le(data)?));
      },
      0x05 => {
        tags.push(NbtTag::Float(nbt_string_value(data)?, float_le(data)?));
      },
      0x06 => {
        tags.push(NbtTag::Double(nbt_string_value(data)?, double_le(data)?));
      },
      0x07 => {
        tags.push(NbtTag::ByteArray(nbt_string_value(data)?, nbt_byte_array_value(data)?));
      },
      0x08 => {
        tags.push(NbtTag::String(nbt_string_value(data)?, nbt_string_value(data)?));
      },
      0x09 => {
        tags.push(nbt_list(data)?);
      },
      0x0a => {
        tags.push(nbt_tag_compound(data)?);
      },
      0x0b => {
        tags.push(NbtTag::IntArray(nbt_string_value(data)?, nbt_int_array_value(data)?));
      },
      0x0c => {
        tags.push(NbtTag::LongArray(nbt_string_value(data)?, nbt_long_array_value(data)?));
      },
      x => {
        return Err(Box::new(CustomError::InvalidNbtTag(x)));
      }
    };
  }

  let output = NbtTag::TagCompound(description, tags);

  return Ok(output);
}
fn nbt_tag_compound_list(data: &mut Vec<u8>) -> Result<NbtListTag, Box<dyn Error>> {
  let mut tags: Vec<NbtTag> = Vec::new();

  loop {
    let id = data.pop().unwrap();
    match id {
      0x00 => break,
      0x01 => {
        tags.push(NbtTag::Byte(nbt_string_value(data)?, data.pop().unwrap()));
      },
      0x02 => {
        tags.push(NbtTag::Short(nbt_string_value(data)?, short_le(data)?));
      },
      0x03 => {
        tags.push(NbtTag::Int(nbt_string_value(data)?, int_le(data)?));
      },
      0x04 => {
        tags.push(NbtTag::Long(nbt_string_value(data)?, long_le(data)?));
      },
      0x05 => {
        tags.push(NbtTag::Float(nbt_string_value(data)?, float_le(data)?));
      },
      0x06 => {
        tags.push(NbtTag::Double(nbt_string_value(data)?, double_le(data)?));
      },
      0x07 => {
        tags.push(NbtTag::ByteArray(nbt_string_value(data)?, nbt_byte_array_value(data)?));
      },
      0x08 => {
        tags.push(NbtTag::String(nbt_string_value(data)?, nbt_string_value(data)?));
      },
      0x09 => {
        tags.push(nbt_list(data)?);
      },
      0x0a => {
        tags.push(nbt_tag_compound(data)?);
      },
      0x0b => {
        tags.push(NbtTag::IntArray(nbt_string_value(data)?, nbt_int_array_value(data)?));
      },
      0x0c => {
        tags.push(NbtTag::LongArray(nbt_string_value(data)?, nbt_long_array_value(data)?));
      },
      x => {
        return Err(Box::new(CustomError::InvalidNbtTag(x)));
      }
    };
  }

  let output = NbtListTag::TagCompound(tags);

  return Ok(output);
}

fn nbt_int_array_value(data: &mut Vec<u8>) -> Result<Vec<i32>, Box<dyn Error>> {
  let length = int_le(data)?;

  let mut arr: Vec<i32> = Vec::new();
  for _ in 0..length {
    arr.push(int_le(data)?);
  }

  return Ok(arr);
}

fn nbt_long_array_value(data: &mut Vec<u8>) -> Result<Vec<i64>, Box<dyn Error>> {
  let length: i32 = int_le(data)?;

  let mut arr: Vec<i64> = Vec::new();
  for _ in 0..length {
 		arr.push(long_le(data)?);
  }

  return Ok(arr);
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn varint_works_small_number() {
    let res = varint(&mut vec![0x01]);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 1);
  }

  #[test]
  fn varint_works_large_number() {
    let res = varint(&mut vec![0xff, 0xff, 0xff, 0xff, 0x07]);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 2147483647);
  }

  #[test]
  fn varint_works_zero() {
    let res = varint(&mut vec![0x00]);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 0);
  }

  #[test]
  fn varint_works_negative() {
    let res = varint(&mut vec![0xff, 0xff, 0xff, 0xff, 0x0f]);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), -1);
  }

  #[test]
  fn varint_works_medium_number() {
    let res = varint(&mut vec![0xf4, 0xd0, 0x01]);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 26740);
  }

  #[test]
  fn varint_works_256() {
    let res = varint(&mut vec![128, 2]);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 256);
  }

  #[test]
  fn nbt_mobspawner() {
    let nbt_parsed = NbtTag::Root(vec![
      NbtTag::Short("MaxNearbyEntities".to_string(), 6),
      NbtTag::Short("RequiredPlayerRange".to_string(), 16),
      NbtTag::Short("SpawnCount".to_string(), 4),
      NbtTag::TagCompound("SpawnData".to_string(), vec![
        NbtTag::TagCompound("entity".to_string(), vec![
          NbtTag::String("id".to_string(), "minecraft:spider".to_string()),
        ]),
      ]),
      NbtTag::Short("MaxSpawnDelay".to_string(), 800),
      NbtTag::Short("SpawnRange".to_string(), 4),
      NbtTag::Short("Delay".to_string(), 20),
      NbtTag::Short("MinSpawnDelay".to_string(), 200),
    ]);

    let mut nbt_bytes: Vec<u8> = vec![10,2,0,17,77,97,120,78,101,97,114,98,121,69,110,116,105,116,105,101,115,0,6,2,0,19,82,101,113,117,105,114,101,100,80,108,97,121,101,114,82,97,110,103,101,0,16,2,0,10,83,112,97,119,110,67,111,117,110,116,0,4,10,0,9,83,112,97,119,110,68,97,116,97,10,0,6,101,110,116,105,116,121,8,0,2,105,100,0,16,109,105,110,101,99,114,97,102,116,58,115,112,105,100,101,114,0,0,2,0,13,77,97,120,83,112,97,119,110,68,101,108,97,121,3,32,2,0,10,83,112,97,119,110,82,97,110,103,101,0,4,2,0,5,68,101,108,97,121,0,20,2,0,13,77,105,110,83,112,97,119,110,68,101,108,97,121,0,200,0];

    assert_eq!(nbt_network(&mut nbt_bytes).unwrap(), nbt_parsed);
  }

  #[test]
  fn nbt_nested_compound_network() {
    let nbt = NbtTag::Root(vec![
      NbtTag::TagCompound("a".to_string(), vec![
        NbtTag::TagCompound("b".to_string(), vec![
          NbtTag::String("c".to_string(), "hi!".to_string()),
        ]),
      ]),
    ]);

    let mut nbt_bytes = crate::serialize::nbt_network(nbt.clone());
    println!("nbt_bytes\n{nbt_bytes:?}");
    assert_eq!(nbt_network(&mut nbt_bytes).unwrap(), nbt);
  }

  #[test]
  fn nbt_basic_network() {
    let nbt = NbtTag::Root(vec![
      NbtTag::String("hi".to_string(), "hello".to_string()),
      NbtTag::Int("hi2".to_string(), 1234),
    ]);

    let mut nbt_bytes = crate::serialize::nbt_network(nbt.clone());
    println!("nbt_bytes\n{nbt_bytes:?}");
    assert_eq!(nbt_network(&mut nbt_bytes).unwrap(), nbt);
  }

  #[test]
  fn nbt_basic_disk() {
    let nbt = NbtTag::Root(vec![
      NbtTag::String("hi".to_string(), "hello".to_string()),
      NbtTag::Int("hi2".to_string(), 1234),
    ]);

    let mut nbt_bytes = crate::serialize::nbt_disk(nbt.clone());
    println!("nbt_bytes\n{nbt_bytes:?}");
    assert_eq!(super::nbt_disk(&mut nbt_bytes).unwrap(), nbt);
  }

  #[test]
  fn nbt_with_list_of_compounds_network() {
    let nbt = NbtTag::Root(vec![
	    NbtTag::List("this_is_a_list".to_string(), vec![
				NbtListTag::TagCompound(vec![
					NbtTag::String("a".to_string(), "b".to_string()),
				]),
				NbtListTag::TagCompound(vec![
					NbtTag::String("a".to_string(), "b".to_string()),
				]),
				NbtListTag::TagCompound(vec![
					NbtTag::String("a".to_string(), "b".to_string()),
				]),
	    ]),
    ]);

    let mut nbt_bytes = crate::serialize::nbt_network(nbt.clone());
    println!("nbt_bytes\n{nbt_bytes:?}");
    assert_eq!(nbt_network(&mut nbt_bytes).unwrap(), nbt);
  }

  #[test]
  fn nbt_with_list_of_compounds_disk() {
    let nbt = NbtTag::Root(vec![
	    NbtTag::List("this_is_a_list".to_string(), vec![
				NbtListTag::TagCompound(vec![
					NbtTag::String("a".to_string(), "b".to_string()),
				]),
				NbtListTag::TagCompound(vec![
					NbtTag::String("a".to_string(), "b".to_string()),
				]),
				NbtListTag::TagCompound(vec![
					NbtTag::String("a".to_string(), "b".to_string()),
				]),
	    ]),
    ]);

    let mut nbt_bytes = crate::serialize::nbt_disk(nbt.clone());
    println!("nbt_bytes\n{nbt_bytes:?}");
    assert_eq!(super::nbt_disk(&mut nbt_bytes).unwrap(), nbt);
  }

  #[test]
  fn nbt_with_empty_list_disk() {
    let nbt = NbtTag::Root(vec![
	    NbtTag::List("this_is_a_list".to_string(), vec![

	    ]),
    ]);

    let mut nbt_bytes = crate::serialize::nbt_disk(nbt.clone());
    println!("nbt_bytes\n{nbt_bytes:?}");
    assert_eq!(super::nbt_disk(&mut nbt_bytes).unwrap(), nbt);
  }

  #[test]
  fn nbt_block_entity() {
    let nbt = NbtTag::Root(vec![
      NbtTag::List("block_entities".to_string(), vec![
        NbtListTag::TagCompound(vec![
          NbtTag::List("Items".to_string(), vec![
            NbtListTag::TagCompound(vec![
              NbtTag::Byte("Slot".to_string(), 0),
              NbtTag::Int("count".to_string(), 1),
              NbtTag::String("id".to_string(), "minecraft:furnace".to_string()),
            ]),
            NbtListTag::TagCompound(vec![
              NbtTag::Byte("Slot".to_string(), 1),
              NbtTag::Int("count".to_string(), 12),
              NbtTag::String("id".to_string(), "minecraft:furnace".to_string()),
            ]),
            NbtListTag::TagCompound(vec![
              NbtTag::Byte("Slot".to_string(), 3),
              NbtTag::Int("count".to_string(), 1),
              NbtTag::String("id".to_string(), "minecraft:furnace".to_string()),
            ]),
          ]),
          NbtTag::TagCompound("components".to_string(), vec![

          ]),
          NbtTag::String("id".to_string(), "minecraft:chest".to_string()),
          NbtTag::Byte("keepPacked".to_string(), 0),
          NbtTag::Int("x".to_string(), 22),
          NbtTag::Int("y".to_string(), 64),
          NbtTag::Int("z".to_string(), 30),
        ]),
        NbtListTag::TagCompound(vec![
          NbtTag::List("Items".to_string(), vec![

          ]),
          NbtTag::TagCompound("components".to_string(), vec![

          ]),
          NbtTag::Short("cooking_time_spent".to_string(), 0),
          NbtTag::Short("cooking_total_time".to_string(), 200),
          NbtTag::String("id".to_string(), "minecraft:furnace".to_string()),
          NbtTag::Byte("keepPacked".to_string(), 0),
          NbtTag::Short("lit_time_remaining".to_string(), 200),
          NbtTag::Short("lit_total_time".to_string(), 300),
          NbtTag::Int("x".to_string(), 22),
          NbtTag::Int("y".to_string(), 64),
          NbtTag::Int("z".to_string(), 31),
        ]),
      ]),
    ]);

    let mut nbt_bytes = crate::serialize::nbt_disk(nbt.clone());
    println!("nbt_bytes\n{nbt_bytes:?}");
    assert_eq!(super::nbt_disk(&mut nbt_bytes).unwrap(), nbt);
  }

  #[test]
  fn nbt_empty_tag_compound() {
    let nbt = NbtTag::Root(vec![
      NbtTag::TagCompound("abc".to_string(), vec![

      ]),
    ]);

    let mut nbt_bytes = crate::serialize::nbt_disk(nbt.clone());
    println!("nbt_bytes\n{nbt_bytes:?}");
    assert_eq!(super::nbt_disk(&mut nbt_bytes).unwrap(), nbt);
  }

  #[test]
  fn nbt_empty_tag_compound_in_list_in_compound() {
    let nbt = NbtTag::Root(vec![
      NbtTag::List("abc".to_string(), vec![
        NbtListTag::TagCompound(vec![
          NbtTag::TagCompound("def".to_string(), vec![

          ]),
        ]),
      ])
    ]);

    let mut nbt_bytes = crate::serialize::nbt_disk(nbt.clone());
    println!("nbt_bytes\n{nbt_bytes:?}");
    assert_eq!(super::nbt_disk(&mut nbt_bytes).unwrap(), nbt);
  }
}
