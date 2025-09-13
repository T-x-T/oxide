use crate::types::*;

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

pub fn position(input: &crate::types::position::Position) -> Vec<u8> {
  return unsigned_long(((input.x as u64 & 0x3FFFFFF) << 38) | ((input.z as u64 & 0x3FFFFFF) << 12) | (input.y as u64 & 0xFFF));
}

pub fn hashed_slot(input: Option<&Slot>) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  let Some(input) = input else {
    output.append(&mut boolean(false));
    return output;
  };

  if input.item_count == 0 {
    output.append(&mut boolean(false));
    return output;
  }

  output.append(&mut varint(input.item_count));
  output.append(&mut varint(input.item_id));
  output.append(&mut varint(input.components_to_add.len() as i32));
  for component in &input.components_to_add {
    output.append(&mut varint(component.into()));
    output.append(&mut int(0)); //there should be a CRC32C calculation of some sorts here... https://minecraft.wiki/w/Java_Edition_protocol/Slot_data#Hashed_Format
  }
  output.append(&mut varint(input.components_to_remove.len() as i32));
  for component in &input.components_to_remove {
    output.append(&mut varint(*component));
  }

  return output;
}

pub fn slot(input: Option<&Slot>) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  let Some(input) = input else {
    output.append(&mut varint(0));
    return output;
  };

  if input.item_count == 0 {
    output.append(&mut varint(0));
    return output;
  }

  output.append(&mut varint(input.item_count));
  output.append(&mut varint(input.item_id));
  output.append(&mut varint(input.components_to_add.len() as i32));
  output.append(&mut varint(input.components_to_remove.len() as i32));
  for component_to_add in &input.components_to_add {
    output.append(&mut varint(component_to_add.into()));
    output.append(&mut match component_to_add.clone() {
      SlotComponent::CustomData(a) => nbt_network(a),
      SlotComponent::MaxStackSize(a) => varint(a),
      SlotComponent::MaxDamage(a) => varint(a),
      SlotComponent::Damage(a) => varint(a),
      SlotComponent::Unbreakable => vec![],
      SlotComponent::CustomName(a) => nbt_network(a),
      SlotComponent::ItemName(a) => nbt_network(a),
      SlotComponent::ItemModel(a) => string(&a),
      SlotComponent::Lore(a) => a.into_iter().flat_map(nbt_network).collect(),
      SlotComponent::Rarity(a) => vec![a],
      SlotComponent::Enchantments(a) => a.into_iter().flat_map(|(x, y)| vec![varint(x), varint(y)]).flatten().collect(),
      SlotComponent::CanPlaceOn => todo!(),
      SlotComponent::CanBreak => todo!(),
      SlotComponent::AttributeModifiers => todo!(),
      SlotComponent::CustomModelData(a, b, c, d) => vec![a.into_iter().flat_map(float).collect::<Vec<u8>>(), b.into_iter().flat_map(boolean).collect(), c.into_iter().flat_map(|x| string(&x)).collect(), d.into_iter().flat_map(int).collect()].into_iter().flatten().collect(),
      SlotComponent::TooltipDisplay(a, b) => vec![boolean(a), b.into_iter().flat_map(varint).collect()].into_iter().flatten().collect(),
      SlotComponent::RepairCost(a) => varint(a),
      SlotComponent::CreativeSlotLock => vec![],
      SlotComponent::EnchantmentGlintOverride(a) => boolean(a),
      SlotComponent::IntangibleProjectile(a) => nbt_network(a),
      SlotComponent::Food(a, b, c) => vec![varint(a), float(b), boolean(c)].into_iter().flatten().collect(),
      SlotComponent::Consumable => todo!(),
      SlotComponent::UseRemainder(a) => slot(a.as_ref()),
      SlotComponent::UseCooldown(a, b) => vec![float(a), if b.is_some() {vec![vec![1], string(&b.unwrap())].into_iter().flatten().collect()} else {vec![0]}].into_iter().flatten().collect(),
      SlotComponent::DamageResistant(a) => string(&a),
      SlotComponent::Tool => todo!(),
      SlotComponent::Weapon(a, b) => vec![varint(a), float(b)].into_iter().flatten().collect(),
      SlotComponent::Enchantable(a) => varint(a),
      SlotComponent::Equippable => todo!(),
      SlotComponent::Repairable => todo!(),
      SlotComponent::Glider => vec![],
      SlotComponent::TooltipStyle(a) => string(&a),
      SlotComponent::DeathProtection => todo!(),
      SlotComponent::BlockAttacks => todo!(),
      SlotComponent::StoredEnchantments(a) => a.into_iter().flat_map(|(x, y)| vec![varint(x), varint(y)]).flatten().collect(),
      SlotComponent::DyedColor(a) => int(a),
      SlotComponent::MapColor(a) => int(a),
      SlotComponent::MapId(a) => varint(a),
      SlotComponent::MapDecorations(a) => nbt_network(a),
      SlotComponent::MapPostProcessing(a) => vec![a],
      SlotComponent::ChargedProjectiles(a) => a.into_iter().flat_map(|x| slot(x.as_ref())).collect(),
      SlotComponent::BundleContents(a) => a.into_iter().flat_map(|x| slot(x.as_ref())).collect(),
      SlotComponent::PotionContents => todo!(),
      SlotComponent::PotionDurationScale(a) => float(a),
      SlotComponent::SuspiciousStewEffects(a) => a.into_iter().flat_map(|(x, y)| vec![varint(x), varint(y)]).flatten().collect(),
      SlotComponent::WritableBookContent(a) => a.into_iter().flat_map(|(x, y)| vec![string(&x), if y.is_some() {vec![vec![0x01], string(&y.unwrap())].into_iter().flatten().collect()} else {vec![0x00]}]).flatten().collect(),
      SlotComponent::WrittenBookContent(a) => a.into_iter().flat_map(|(x, y)| vec![string(&x), if y.is_some() {vec![vec![0x01], string(&y.unwrap())].into_iter().flatten().collect()} else {vec![0x00]}]).flatten().collect(),
      SlotComponent::Trim => todo!(),
      SlotComponent::DebugStickState(a) => nbt_network(a),
      SlotComponent::EntityData(a) => nbt_network(a),
      SlotComponent::BucketEntityData(a) => nbt_network(a),
      SlotComponent::BlockEntityData(a) => nbt_network(a),
      SlotComponent::Instrument => todo!(),
      SlotComponent::ProvidesTrimMaterial => todo!(),
      SlotComponent::OminousBottleAmplifier(a) => vec![a],
      SlotComponent::JukeboxPlayable => todo!(),
      SlotComponent::ProvidesBannerPatterns(a) => string(&a),
      SlotComponent::Recipes(a) => nbt_network(a),
      SlotComponent::LodestoneTracker(a, b, c, d) => vec![boolean(a), string(&b), position(&c), boolean(d)].into_iter().flatten().collect(),
      SlotComponent::FireworkExplosion => todo!(),
      SlotComponent::Fireworks => todo!(),
      SlotComponent::Profile(a, b, c) => vec![if a.is_some(){vec![vec![0x01], string(&a.unwrap())].into_iter().flatten().collect()} else {vec![0x00]}, if b.is_some() {vec![vec![0x01], uuid(&b.unwrap())].into_iter().flatten().collect()} else {vec![0x00]}, c.into_iter().flat_map(|(x, y, z)| vec![string(&x), string(&y), if z.is_some() {vec![vec![0x01], string(&z.unwrap())].into_iter().flatten().collect()} else {vec![0x00]}]).flatten().collect()].into_iter().flatten().collect(),
      SlotComponent::NoteblockSound(a) => string(&a),
      SlotComponent::BannerPatterns => todo!(),
      SlotComponent::BaseColor(a) => vec![a],
      SlotComponent::PotDecorations(a) => a.into_iter().flat_map(varint).collect(),
      SlotComponent::Container(a) => a.into_iter().flat_map(varint).collect(),
      SlotComponent::BlockState(a) => a.into_iter().flat_map(|(x, y)| vec![string(&x), string(&y)]).flatten().collect(),
      SlotComponent::Bees(a) => a.into_iter().flat_map(|(x, y, z)| vec![nbt_network(x), varint(y), varint(z)]).flatten().collect(),
      SlotComponent::Lock(a) => nbt_network(a),
      SlotComponent::ContainerLoot(a) => nbt_network(a),
      SlotComponent::BreakSound => todo!(),
      SlotComponent::VillagerVariant => todo!(),
      SlotComponent::WolfVariant => todo!(),
      SlotComponent::WolfSoundVariant => todo!(),
      SlotComponent::WolfCollar(a) => vec![a],
      SlotComponent::FoxVariant(a) => vec![a],
      SlotComponent::SalmonSize(a) => vec![a],
      SlotComponent::ParrotVariant(a) => varint(a),
      SlotComponent::TropicalFishPattern(a) => vec![a],
      SlotComponent::TropicalFishBaseColor(a) => vec![a],
      SlotComponent::TropicalFishPatternColor(a) => vec![a],
      SlotComponent::MooshroomVariant(a) => vec![a],
      SlotComponent::RabbitVariant(a) => vec![a],
      SlotComponent::PigVariant(a) => vec![a],
      SlotComponent::CowVariant(a) => vec![a],
      SlotComponent::ChickenVariant => todo!(),
      SlotComponent::FrogVariant(a) => varint(a),
      SlotComponent::HorseVariant(a) => vec![a],
      SlotComponent::PaintingVariant => todo!(),
      SlotComponent::LlamaVariant(a) => vec![a],
      SlotComponent::AxolotlVariant(a) => vec![a],
      SlotComponent::CatVariant(a) => varint(a),
      SlotComponent::CatCollar(a) => vec![a],
      SlotComponent::SheepColor(a) => vec![a],
      SlotComponent::ShulkerColor(a) => vec![a],
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

pub fn nbt_network(input: NbtTag) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();
	match input {
		NbtTag::Root(p) => {
		  output.push(0x0a);

      p.into_iter().for_each(|x| {
        match x {
          NbtTag::Byte(d, p) => output.append(&mut nbt_byte(d, p)),
          NbtTag::Short(d, p) => output.append(&mut nbt_short(d, p)),
          NbtTag::Int(d, p) => output.append(&mut nbt_int(d, p)),
          NbtTag::Long(d, p) => output.append(&mut nbt_long(d, p)),
          NbtTag::Float(d, p) => output.append(&mut nbt_float(d, p)),
          NbtTag::Double(d, p) => output.append(&mut nbt_double(d, p)),
          NbtTag::ByteArray(d, p) => output.append(&mut nbt_byte_array(d, p)),
          NbtTag::String(d, p) => output.append(&mut nbt_string(d, p)),
          NbtTag::List(d, p) => output.append(&mut nbt_list(d, p)),
          NbtTag::TagCompound(d, p) => output.append(&mut nbt_tag_compound(d, p)),
          NbtTag::Root(_) => panic!("Root tag cannot be in a TagCompound!!"),
          NbtTag::IntArray(d, p) => output.append(&mut nbt_int_array(d, p)),
          NbtTag::LongArray(d, p) => output.append(&mut nbt_long_array(d, p)),
        };
      });

      output.push(0x00);
		},
		_ => panic!("root node must be a root tag"),
	}

	return output;
}

pub fn nbt_disk(input: NbtTag) -> Vec<u8> {
	match input {
		NbtTag::Root(p) => {
			return nbt_tag_compound("".to_string(), p);
		},
		_ => panic!("root node must be a tag compound"),
	}
}

fn nbt_byte(description: String, payload: u8) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();
  output.push(0x01);
  output.append(&mut nbt_string_list(description));

  output.push(payload);

  return output;
}

fn nbt_byte_list(payload: u8) -> Vec<u8> {
  return vec![payload];
}

fn nbt_short(description: String, payload: i16) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();
  output.push(0x02);
  output.append(&mut nbt_string_list(description));

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_short_list(payload: i16) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_int(description: String, payload: i32) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();
  output.push(0x03);
  output.append(&mut nbt_string_list(description));

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_int_list(payload: i32) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_long(description: String, payload: i64) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();
  output.push(0x04);
  output.append(&mut nbt_string_list(description));

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_long_list(payload: i64) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_float(description: String, payload: f32) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();
  output.push(0x05);
  output.append(&mut nbt_string_list(description));

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_float_list(payload: f32) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_double(description: String, payload: f64) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();
  output.push(0x06);
  output.append(&mut nbt_string_list(description));

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_double_list(payload: f64) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_byte_array(description: String, payload: Vec<u8>) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();
  output.push(0x07);
  output.append(&mut nbt_string_list(description));

  output.append(&mut (payload.len() as i32).to_be_bytes().to_vec());
  output.append(&mut payload.to_vec());

  return output;
}

fn nbt_byte_array_list(payload: Vec<u8>) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  output.append(&mut (payload.len() as i32).to_be_bytes().to_vec());
  output.append(&mut payload.to_vec());

  return output;
}

fn nbt_string(description: String, payload: String) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();
  output.push(0x08);
  output.append(&mut nbt_string_list(description));

  output.append(&mut nbt_short_list(payload.len() as i16));
  output.append(&mut payload.as_bytes().to_vec());

  return output;
}

fn nbt_string_list(payload: String) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  output.append(&mut nbt_short_list(payload.len() as i16));
  output.append(&mut payload.as_bytes().to_vec());

  return output;
}

fn nbt_list(description: String, payload: Vec<NbtListTag>) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();
  output.push(0x09);
  output.append(&mut nbt_string_list(description));

  output.append(&mut actual_nbt_list(payload));

  return output;
}

fn nbt_list_list(payload: Vec<NbtListTag>) -> Vec<u8> {
  return actual_nbt_list(payload);
}

fn actual_nbt_list(payload: Vec<NbtListTag>) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if payload.is_empty() {
  	output.append(&mut vec![0;5]);
    return output;
  }

  let length: i32 = payload.len() as i32;
  match payload[0] {
    NbtListTag::Byte(_) => {
      output.push(0x01);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_byte_list(match x {
        NbtListTag::Byte(x) => x,
        _ => panic!("impossible to reach"),
      })));
    },
    NbtListTag::Short(_) => {
      output.push(0x02);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_short_list(match x {
        NbtListTag::Short(x) => x,
        _ => panic!("impossible to reach"),
      })));
    },
    NbtListTag::Int(_) => {
      output.push(0x03);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_int_list(match x {
        NbtListTag::Int(x) => x,
        _ => panic!("impossible to reach"),
      })));
    },
    NbtListTag::Long(_) => {
      output.push(0x04);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_long_list(match x {
        NbtListTag::Long(x) => x,
        _ => panic!("impossible to reach"),
      })));
    },
    NbtListTag::Float(_) => {
      output.push(0x05);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_float_list(match x {
        NbtListTag::Float(x) => x,
        _ => panic!("impossible to reach"),
      })));
    },
    NbtListTag::Double(_) => {
      output.push(0x06);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_double_list(match x {
        NbtListTag::Double(x) => x,
        _ => panic!("impossible to reach"),
      })));
    },
    NbtListTag::ByteArray(_) => {
      output.push(0x07);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_byte_array_list(match x {
        NbtListTag::ByteArray(x) => x,
        _ => panic!("impossible to reach"),
      })));
    },
    NbtListTag::String(_) => {
      output.push(0x08);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_string_list(match x {
        NbtListTag::String(x) => x,
        _ => panic!("impossible to reach"),
      })));
    },
    NbtListTag::List(_) => {
      output.push(0x09);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_list_list(match x {
        NbtListTag::List(x) => x,
        _ => panic!("impossible to reach"),
      })));
    },
    NbtListTag::TagCompound(_) => {
      output.push(0x0a);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_tag_compound_list(match x {
        NbtListTag::TagCompound(x) => x,
        _ => panic!("impossible to reach"),
      })));
    },
    NbtListTag::IntArray(_) => {
      output.push(0x0b);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_int_array_list(match x {
        NbtListTag::IntArray(x) => x,
        _ => panic!("impossible to reach"),
      })));
    },
    NbtListTag::LongArray(_) => {
      output.push(0x0c);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_long_array_list(match x {
        NbtListTag::LongArray(x) => x,
        _ => panic!("impossible to reach"),
      })));
    },
  };

  return output;
}

fn nbt_tag_compound(description: String, payload: Vec<NbtTag>) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();
  output.push(0x0a);
  output.append(&mut nbt_string_list(description));

  if payload.is_empty() {
    output.push(0x00);
    return output;
  }

  payload.into_iter().for_each(|x| {
    match x {
      NbtTag::Byte(d, p) => output.append(&mut nbt_byte(d, p)),
      NbtTag::Short(d, p) => output.append(&mut nbt_short(d, p)),
      NbtTag::Int(d, p) => output.append(&mut nbt_int(d, p)),
      NbtTag::Long(d, p) => output.append(&mut nbt_long(d, p)),
      NbtTag::Float(d, p) => output.append(&mut nbt_float(d, p)),
      NbtTag::Double(d, p) => output.append(&mut nbt_double(d, p)),
      NbtTag::ByteArray(d, p) => output.append(&mut nbt_byte_array(d, p)),
      NbtTag::String(d, p) => output.append(&mut nbt_string(d, p)),
      NbtTag::List(d, p) => output.append(&mut nbt_list(d, p)),
      NbtTag::TagCompound(d, p) => output.append(&mut nbt_tag_compound(d, p)),
      NbtTag::Root(_) => panic!("Root tag cannot be in a TagCompound!!"),
      NbtTag::IntArray(d, p) => output.append(&mut nbt_int_array(d, p)),
      NbtTag::LongArray(d, p) => output.append(&mut nbt_long_array(d, p)),
    };
  });

  output.push(0x00);
  return output;
}

fn nbt_tag_compound_list(payload: Vec<NbtTag>) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if payload.is_empty() {
    output.push(0x00);
    return output;
  }

  payload.into_iter().for_each(|x| {
    match x {
      NbtTag::Byte(d, p) => output.append(&mut nbt_byte(d, p)),
      NbtTag::Short(d, p) => output.append(&mut nbt_short(d, p)),
      NbtTag::Int(d, p) => output.append(&mut nbt_int(d, p)),
      NbtTag::Long(d, p) => output.append(&mut nbt_long(d, p)),
      NbtTag::Float(d, p) => output.append(&mut nbt_float(d, p)),
      NbtTag::Double(d, p) => output.append(&mut nbt_double(d, p)),
      NbtTag::ByteArray(d, p) => output.append(&mut nbt_byte_array(d, p)),
      NbtTag::String(d, p) => output.append(&mut nbt_string(d, p)),
      NbtTag::List(d, p) => output.append(&mut nbt_list(d, p)),
      NbtTag::TagCompound(d, p) => output.append(&mut nbt_tag_compound(d, p)),
      NbtTag::Root(_) => panic!("Root tag cannot be in a TagCompound!!"),
      NbtTag::IntArray(d, p) => output.append(&mut nbt_int_array(d, p)),
      NbtTag::LongArray(d, p) => output.append(&mut nbt_long_array(d, p)),
    };
  });

  output.push(0x00);
  return output;
}

fn nbt_int_array(description: String, payload: Vec<i32>) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();
  output.push(0x0b);
  output.append(&mut nbt_string_list(description));

  let length: i32 = payload.len() as i32;
  output.append(&mut length.to_be_bytes().into());

  payload.into_iter().for_each(|x| output.append(&mut nbt_int_list(x)));

  return output;
}

fn nbt_int_array_list(payload: Vec<i32>) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  let length: i32 = payload.len() as i32;
  output.append(&mut length.to_be_bytes().into());

  payload.into_iter().for_each(|x| output.append(&mut nbt_int_list(x)));

  return output;
}

fn nbt_long_array(description: String, payload: Vec<i64>) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();
  output.push(0x0c);
  output.append(&mut nbt_string_list(description));

  let length: i32 = payload.len() as i32;
  output.append(&mut length.to_be_bytes().into());

  payload.into_iter().for_each(|x| output.append(&mut nbt_long_list(x)));

  return output;
}

fn nbt_long_array_list(payload: Vec<i64>) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  let length: i32 = payload.len() as i32;
  output.append(&mut length.to_be_bytes().into());

  payload.into_iter().for_each(|x| output.append(&mut nbt_long_list(x)));

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
}
