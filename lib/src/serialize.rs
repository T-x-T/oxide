use crate::nbt::NbtTag;

const SEGMENT_BITS: u32 = 0b0111_1111;
const CONTINUE_BIT: u8 = 0b1000_0000;

pub fn varint(value: i32) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  let mut uvalue = value as u32;
  loop {
    let mut byte = (uvalue & SEGMENT_BITS) as u8;
    uvalue >>= 7;

    if uvalue != 0 {
      byte |= CONTINUE_BIT;
    }

    output.push(byte);

    if uvalue == 0 {
      break;
    }
  }

  return output;
}

pub fn boolean(input: bool) -> Vec<u8> {
  if input {
    return vec![0x01];
  } else {
    return vec![0x00];
  }
}

pub fn float(input: f32) -> Vec<u8> {
  return input.to_be_bytes().to_vec();
}

pub fn double(input: f64) -> Vec<u8> {
  return input.to_be_bytes().to_vec();
}

pub fn short(input: i16) -> Vec<u8> {
  return input.to_be_bytes().to_vec();
}

pub fn unsigned_short(input: u16) -> Vec<u8> {
  return input.to_be_bytes().to_vec();
}

pub fn int(input: i32) -> Vec<u8> {
  return input.to_be_bytes().to_vec();
}

pub fn long(input: i64) -> Vec<u8> {
  return input.to_be_bytes().to_vec();
}

pub fn unsigned_long(input: u64) -> Vec<u8> {
  return input.to_be_bytes().to_vec();
}

pub fn string(input: &str) -> Vec<u8> {
  let mut output: Vec<u8> = varint(input.len() as i32);

  output.append(&mut input.as_bytes().to_vec());

  return output;
}

pub fn bitset(input: &Vec<u64>) -> Vec<u8> {
  let mut output: Vec<u8> = varint(input.len() as i32);
  for x in input {
    output.append(&mut unsigned_long(*x));
  }
  return output;
}

pub fn position(input: &crate::packets::Position) -> Vec<u8> {
  return unsigned_long(((input.x as u64 & 0x3FFFFFF) << 38) | ((input.z as u64 & 0x3FFFFFF) << 12) | (input.y as u64 & 0xFFF));
}

pub fn slot(input: &crate::packets::Slot) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  output.append(&mut varint(input.item_count));

  if input.item_count == 0 {
    return output;
  }

  output.append(&mut varint(input.item_id.unwrap_or(0)));
  output.append(&mut varint(input.components_to_add.len() as i32));
  output.append(&mut varint(input.components_to_remove.len() as i32));
  for component_to_add in &input.components_to_add {
    output.append(&mut varint(component_to_add.into()));
    output.append(&mut match component_to_add.clone() {
      crate::packets::SlotComponent::CustomData(a) => nbt(a),
      crate::packets::SlotComponent::MaxStackSize(a) => varint(a),
      crate::packets::SlotComponent::MaxDamage(a) => varint(a),
      crate::packets::SlotComponent::Damage(a) => varint(a),
      crate::packets::SlotComponent::Unbreakable => vec![],
      crate::packets::SlotComponent::CustomName(a) => nbt(a),
      crate::packets::SlotComponent::ItemName(a) => nbt(a),
      crate::packets::SlotComponent::ItemModel(a) => string(&a),
      crate::packets::SlotComponent::Lore(a) => a.into_iter().map(|x| nbt(x)).flatten().collect(),
      crate::packets::SlotComponent::Rarity(a) => vec![a],
      crate::packets::SlotComponent::Enchantments(a) => a.into_iter().map(|(x, y)| vec![varint(x), varint(y)]).flatten().flatten().collect(),
      crate::packets::SlotComponent::CanPlaceOn => todo!(),
      crate::packets::SlotComponent::CanBreak => todo!(),
      crate::packets::SlotComponent::AttributeModifiers => todo!(),
      crate::packets::SlotComponent::CustomModelData(a, b, c, d) => vec![a.into_iter().map(|x| float(x)).flatten().collect::<Vec<u8>>(), b.into_iter().map(|x| boolean(x)).flatten().collect(), c.into_iter().map(|x| string(&x)).flatten().collect(), d.into_iter().map(|x| int(x)).flatten().collect()].into_iter().flatten().collect(),
      crate::packets::SlotComponent::TooltipDisplay(a, b) => vec![boolean(a), b.into_iter().map(|x| varint(x)).flatten().collect()].into_iter().flatten().collect(),
      crate::packets::SlotComponent::RepairCost(a) => varint(a),
      crate::packets::SlotComponent::CreativeSlotLock => vec![],
      crate::packets::SlotComponent::EnchantmentGlintOverride(a) => boolean(a),
      crate::packets::SlotComponent::IntangibleProjectile(a) => nbt(a),
      crate::packets::SlotComponent::Food(a, b, c) => vec![varint(a), float(b), boolean(c)].into_iter().flatten().collect(),
      crate::packets::SlotComponent::Consumable => todo!(),
      crate::packets::SlotComponent::UseRemainder(a) => slot(&a),
      crate::packets::SlotComponent::UseCooldown(a, b) => vec![float(a), if b.is_some() {vec![vec![1], string(&b.unwrap())].into_iter().flatten().collect()} else {vec![0]}].into_iter().flatten().collect(),
      crate::packets::SlotComponent::DamageResistant(a) => string(&a),
      crate::packets::SlotComponent::Tool => todo!(),
      crate::packets::SlotComponent::Weapon(a, b) => vec![varint(a), float(b)].into_iter().flatten().collect(),
      crate::packets::SlotComponent::Enchantable(a) => varint(a),
      crate::packets::SlotComponent::Equippable => todo!(),
      crate::packets::SlotComponent::Repairable => todo!(),
      crate::packets::SlotComponent::Glider => vec![],
      crate::packets::SlotComponent::TooltipStyle(a) => string(&a),
      crate::packets::SlotComponent::DeathProtection => todo!(),
      crate::packets::SlotComponent::BlockAttacks => todo!(),
      crate::packets::SlotComponent::StoredEnchantments(a) => a.into_iter().map(|(x, y)| vec![varint(x), varint(y)]).flatten().flatten().collect(),
      crate::packets::SlotComponent::DyedColor(a) => int(a),
      crate::packets::SlotComponent::MapColor(a) => int(a),
      crate::packets::SlotComponent::MapId(a) => varint(a),
      crate::packets::SlotComponent::MapDecorations(a) => nbt(a),
      crate::packets::SlotComponent::MapPostProcessing(a) => vec![a],
      crate::packets::SlotComponent::ChargedProjectiles(a) => a.into_iter().map(|x| slot(&x)).flatten().collect(),
      crate::packets::SlotComponent::BundleContents(a) => a.into_iter().map(|x| slot(&x)).flatten().collect(),
      crate::packets::SlotComponent::PotionContents => todo!(),
      crate::packets::SlotComponent::PotionDurationScale(a) => float(a),
      crate::packets::SlotComponent::SuspiciousStewEffects(a) => a.into_iter().map(|(x, y)| vec![varint(x), varint(y)]).flatten().flatten().collect(),
      crate::packets::SlotComponent::WritableBookContent(a) => a.into_iter().map(|(x, y)| vec![string(&x), if y.is_some() {vec![vec![0x01], string(&y.unwrap())].into_iter().flatten().collect()} else {vec![0x00]}]).into_iter().flatten().flatten().collect(),
      crate::packets::SlotComponent::WrittenBookContent(a) => a.into_iter().map(|(x, y)| vec![string(&x), if y.is_some() {vec![vec![0x01], string(&y.unwrap())].into_iter().flatten().collect()} else {vec![0x00]}]).into_iter().flatten().flatten().collect(),
      crate::packets::SlotComponent::Trim => todo!(),
      crate::packets::SlotComponent::DebugStickState(a) => nbt(a),
      crate::packets::SlotComponent::EntityData(a) => nbt(a),
      crate::packets::SlotComponent::BucketEntityData(a) => nbt(a),
      crate::packets::SlotComponent::BlockEntityData(a) => nbt(a),
      crate::packets::SlotComponent::Instrument => todo!(),
      crate::packets::SlotComponent::ProvidesTrimMaterial => todo!(),
      crate::packets::SlotComponent::OminousBottleAmplifier(a) => vec![a],
      crate::packets::SlotComponent::JukeboxPlayable => todo!(),
      crate::packets::SlotComponent::ProvidesBannerPatterns(a) => string(&a),
      crate::packets::SlotComponent::Recipes(a) => nbt(a),
      crate::packets::SlotComponent::LodestoneTracker(a, b, c, d) => vec![boolean(a), string(&b), position(&c), boolean(d)].into_iter().flatten().collect(),
      crate::packets::SlotComponent::FireworkExplosion => todo!(),
      crate::packets::SlotComponent::Fireworks => todo!(),
      crate::packets::SlotComponent::Profile(a, b, c) => vec![if a.is_some(){vec![vec![0x01], string(&a.unwrap())].into_iter().flatten().collect()} else {vec![0x00]}, if b.is_some() {vec![vec![0x01], uuid(&b.unwrap())].into_iter().flatten().collect()} else {vec![0x00]}, c.into_iter().map(|(x, y, z)| vec![string(&x), string(&y), if z.is_some() {vec![vec![0x01], string(&z.unwrap())].into_iter().flatten().collect()} else {vec![0x00]}]).into_iter().flatten().flatten().collect()].into_iter().flatten().collect(),
      crate::packets::SlotComponent::NoteblockSound(a) => string(&a),
      crate::packets::SlotComponent::BannerPatterns => todo!(),
      crate::packets::SlotComponent::BaseColor(a) => vec![a],
      crate::packets::SlotComponent::PotDecorations(a) => a.into_iter().map(|x| varint(x)).flatten().collect(),
      crate::packets::SlotComponent::Container(a) => a.into_iter().map(|x| varint(x)).flatten().collect(),
      crate::packets::SlotComponent::BlockState(a) => a.into_iter().map(|(x, y)| vec![string(&x), string(&y)]).flatten().flatten().collect(),
      crate::packets::SlotComponent::Bees(a) => a.into_iter().map(|(x, y, z)| vec![nbt(x), varint(y), varint(z)]).flatten().flatten().collect(),
      crate::packets::SlotComponent::Lock(a) => nbt(a),
      crate::packets::SlotComponent::ContainerLoot(a) => nbt(a),
      crate::packets::SlotComponent::BreakSound => todo!(),
      crate::packets::SlotComponent::VillagerVariant => todo!(),
      crate::packets::SlotComponent::WolfVariant => todo!(),
      crate::packets::SlotComponent::WolfSoundVariant => todo!(),
      crate::packets::SlotComponent::WolfCollar(a) => vec![a],
      crate::packets::SlotComponent::FoxVariant(a) => vec![a],
      crate::packets::SlotComponent::SalmonSize(a) => vec![a],
      crate::packets::SlotComponent::ParrotVariant(a) => varint(a),
      crate::packets::SlotComponent::TropicalFishPattern(a) => vec![a],
      crate::packets::SlotComponent::TropicalFishBaseColor(a) => vec![a],
      crate::packets::SlotComponent::TropicalFishPatternColor(a) => vec![a],
      crate::packets::SlotComponent::MooshroomVariant(a) => vec![a],
      crate::packets::SlotComponent::RabbitVariant(a) => vec![a],
      crate::packets::SlotComponent::PigVariant(a) => vec![a],
      crate::packets::SlotComponent::CowVariant(a) => vec![a],
      crate::packets::SlotComponent::ChickenVariant => todo!(),
      crate::packets::SlotComponent::FrogVariant(a) => varint(a),
      crate::packets::SlotComponent::HorseVariant(a) => vec![a],
      crate::packets::SlotComponent::PaintingVariant => todo!(),
      crate::packets::SlotComponent::LlamaVariant(a) => vec![a],
      crate::packets::SlotComponent::AxolotlVariant(a) => vec![a],
      crate::packets::SlotComponent::CatVariant(a) => varint(a),
      crate::packets::SlotComponent::CatCollar(a) => vec![a],
      crate::packets::SlotComponent::SheepColor(a) => vec![a],
      crate::packets::SlotComponent::ShulkerColor(a) => vec![a],
    });
  }

  for component_to_remove in &input.components_to_remove {
    output.append(&mut varint(*component_to_remove));
  }

  return output;
}

pub fn uuid(input: &u128) -> Vec<u8> {
  return input.to_be_bytes().to_vec();
}

pub fn prefixed_array(mut data: Vec<u8>, len: i32) -> Vec<u8> {
  let mut output: Vec<u8> = varint(len);
  output.append(&mut data);

  return output;
}

pub fn nbt(input: NbtTag) -> Vec<u8> {
  let mut nbt = nbt_tag_compound(None, vec![input], false);
  nbt.pop(); //Otherwise we have one 0x00 byte too much at the end
  return nbt;
}

fn nbt_byte(description: Option<String>, payload: u8, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x01);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

    output.push(payload);

  return output;
}

fn nbt_short(description: Option<String>, payload: i16, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x02);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_int(description: Option<String>, payload: i32, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x03);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_long(description: Option<String>, payload: i64, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x04);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_float(description: Option<String>, payload: f32, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x05);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_double(description: Option<String>, payload: f64, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x06);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_byte_array(description: Option<String>, payload: Vec<u8>, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x07);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  output.append(&mut (payload.len() as i32).to_be_bytes().to_vec());

  output.append(&mut payload.to_vec());

  return output;
}

fn nbt_string(description: Option<String>, payload: String, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x08);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  output.append(&mut nbt_short(None, payload.len() as i16, false));
  output.append(&mut payload.as_bytes().to_vec());

  return output;
}

fn nbt_list(description: Option<String>, payload: Vec<NbtTag>, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x09);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  if payload.len() == 0 {
    return output;
  }

  let length: i32 = payload.len() as i32;

  match payload[0] {
    NbtTag::Byte(_, _) => {
      output.push(0x01);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_byte(None, match x {
        NbtTag::Byte(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::Short(_, _) => {
      output.push(0x02);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_short(None, match x {
        NbtTag::Short(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::Int(_, _) => {
      output.push(0x03);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_int(None, match x {
        NbtTag::Int(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::Long(_, _) => {
      output.push(0x04);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_long(None, match x {
        NbtTag::Long(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::Float(_, _) => {
      output.push(0x05);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_float(None, match x {
        NbtTag::Float(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::Double(_, _) => {
      output.push(0x06);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_double(None, match x {
        NbtTag::Double(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::ByteArray(_, _) => {
      output.push(0x07);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_byte_array(None, match x {
        NbtTag::ByteArray(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::String(_, _) => {
      output.push(0x08);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_string(None, match x {
        NbtTag::String(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::List(_, _) => {
      output.push(0x09);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_list(None, match x {
        NbtTag::List(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::TagCompound(_, _) => {
      output.push(0x0a);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_tag_compound(None, match x {
        NbtTag::TagCompound(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::IntArray(_, _) => {
      output.push(0x0b);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_int_array(None, match x {
        NbtTag::IntArray(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::LongArray(_, _) => {
      output.push(0x0c);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_long_array(None, match x {
        NbtTag::LongArray(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
  };

  return output;
}

fn nbt_tag_compound(description: Option<String>, payload: Vec<NbtTag>, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x0a);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  if payload.len() == 0 {
    return output;
  }

  payload.into_iter().for_each(|x| {
    match x {
      NbtTag::Byte(d, p) => output.append(&mut nbt_byte(d, p, true)),
      NbtTag::Short(d, p) => output.append(&mut nbt_short(d, p, true)),
      NbtTag::Int(d, p) => output.append(&mut nbt_int(d, p, true)),
      NbtTag::Long(d, p) => output.append(&mut nbt_long(d, p, true)),
      NbtTag::Float(d, p) => output.append(&mut nbt_float(d, p, true)),
      NbtTag::Double(d, p) => output.append(&mut nbt_double(d, p, true)),
      NbtTag::ByteArray(d, p) => output.append(&mut nbt_byte_array(d, p, true)),
      NbtTag::String(d, p) => output.append(&mut nbt_string(d, p, true)),
      NbtTag::List(d, p) => output.append(&mut nbt_list(d, p, true)),
      NbtTag::TagCompound(d, p) => output.append(&mut nbt_tag_compound(d, p, true)),
      NbtTag::IntArray(d, p) => output.append(&mut nbt_int_array(d, p, true)),
      NbtTag::LongArray(d, p) => output.append(&mut nbt_long_array(d, p, true)),
    };
  });

  output.push(0x00);
  return output;
}

fn nbt_int_array(description: Option<String>, payload: Vec<i32>, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x0b);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  let length: i32 = payload.len() as i32;
  output.append(&mut length.to_be_bytes().into());

  payload.into_iter().for_each(|x| output.append(&mut nbt_int(None, x, false)));

  return output;
}

fn nbt_long_array(description: Option<String>, payload: Vec<i64>, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x0c);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  let length: i32 = payload.len() as i32;
  output.append(&mut length.to_be_bytes().into());

  payload.into_iter().for_each(|x| output.append(&mut nbt_long(None, x, false)));

  return output;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn varint_works_small_number() {
    let res = varint(1);
    assert_eq!(res, vec![0x01]);
  }

  #[test]
  fn varint_works_large_number() {
    let res = varint(2147483647);
    assert_eq!(res, vec![0xff, 0xff, 0xff, 0xff, 0x07]);
  }

  #[test]
  fn varint_works_zero() {
    let res = varint(0);
    assert_eq!(res, vec![0x00]);
  }

  #[test]
  fn varint_works_negative() {
    let res = varint(-1);
    assert_eq!(res, vec![0xff, 0xff, 0xff, 0xff, 0x0f]);
  }

  #[test]
  fn varint_works_medium_number() {
    let res = varint(26740);
    assert_eq!(res, vec![0xf4, 0xd0, 0x01]);
  }

  #[test]
  #[ignore]
  fn test() {
    use std::fs::write;

    let test_nbt = NbtTag::TagCompound(Some("Level".to_string()), vec![
        NbtTag::Long(Some("longTest".to_string()), 9223372036854775807),
        NbtTag::Short(Some("shortTest".to_string()), 32767),
        NbtTag::String(Some("stringTest".to_string()), "HELLO WORLD THIS IS A TEST STRING ÅÄÖ!".to_string()),
        NbtTag::Float(Some("floatTest".to_string()), 0.4982314705848694),
        NbtTag::Int(Some("intTest".to_string()), 2147483647),
        NbtTag::TagCompound(Some("nested compound test".to_string()), vec![
          NbtTag::TagCompound(Some("ham".to_string()), vec![
            NbtTag::String(Some("name".to_string()), "Hampus".to_string()),
            NbtTag::Float(Some("value".to_string()), 0.75)
          ]),
          NbtTag::TagCompound(Some("egg".to_string()), vec![
            NbtTag::String(Some("name".to_string()), "Eggbert".to_string()),
            NbtTag::Float(Some("value".to_string()), 0.5)
          ])
        ]),
        NbtTag::List(Some("listTest (long)".to_string()), vec![
          NbtTag::Long(None, 11),
          NbtTag::Long(None, 12),
          NbtTag::Long(None, 13),
          NbtTag::Long(None, 14),
          NbtTag::Long(None, 15)
        ]),
        NbtTag::List(Some("listTest (compound)".to_string()), vec![
          NbtTag::TagCompound(None, vec![
            NbtTag::String(Some("name".to_string()), "Compound tag #0".to_string()),
            NbtTag::Long(Some("created-on".to_string()), 1264099775885)
          ]),
          NbtTag::TagCompound(None, vec![
            NbtTag::String(Some("name".to_string()), "Compound tag #1".to_string()),
            NbtTag::Long(Some("created-on".to_string()), 1264099775885)
          ])
        ]),
        NbtTag::Byte(Some("byteTest".to_string()), 127),
        NbtTag::ByteArray(Some("byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))".to_string()), vec![0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48]),
        NbtTag::Double(Some("doubleTest".to_string()), 0.4931287132182315),
      ]);

    write("/tmp/my_bigtest.nbt", nbt(test_nbt.clone())).unwrap();
  }
}
