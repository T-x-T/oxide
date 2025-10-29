use super::*;

use std::error::Error;
use std::net::TcpStream;

type CommandExecFn = fn(String, Option<&mut TcpStream>, &mut Game) -> Result<(), Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct Command {
	pub name: String,
	pub execute: CommandExecFn,
	pub arguments: Vec<CommandArgument>,
}

#[derive(Debug, Clone)]
pub struct CommandArgument {
	pub name: String,
	pub properties: ParserProperty,
	pub next_arguments: Vec<CommandArgument>,
	pub optional: bool,
}


#[derive(Debug, Clone)]
pub struct CommandNode {
	pub flags: u8,
	pub children: Vec<i32>,
	pub redirect_node: Option<i32>,
	pub name: Option<String>,
	pub properties: Option<ParserProperty>,
	pub suggestions_type: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ParserProperty {
	Bool,
	Float(u8, Option<f32>, Option<f32>),
	Double(u8, Option<f64>, Option<f64>),
	Integer(u8, Option<i32>, Option<i32>),
	Long(u8, Option<i64>, Option<i64>),
	String(i32),
	Entity(u8),
	GameProfile,
	BlockPos,
	ColumnPos,
	Vec3,
	Vec2,
	BlockState,
	BlockPredicate,
	ItemStack,
	ItemPredicate,
	Color,
	Component,
	Style,
	Message,
	Nbt,
	NbtTag,
	NbtPath,
	Objective,
	ObjectiveCriteria,
	Operation,
	Particle,
	Angle,
	Rotation,
	ScoreboardSlot,
	ScoreHolder(u8),
	Swizzle,
	Team,
	ItemSlot,
	ItemSlots,
	ResourceLocation,
	Function,
	EntityAnchor,
	IntRange,
	FloatRange,
	Dimension,
	Gamemode,
	Time(i32),
	ResourceOrTag(String),
	ResourceOrTagKey(String),
	Resource(String),
	ResourceKey(String),
	ResourceSelector(String),
	TemplateMirror,
	TemplateRotation,
	Heightmap,
	Uuid,
	LootTable,
	LootPredicate,
	LootModifier,
}

impl From<ParserProperty> for i32 {
  fn from(value: ParserProperty) -> Self {
  	return match value {
	    ParserProperty::Bool => 0,
	    ParserProperty::Float(_, _, _) => 1,
	    ParserProperty::Double(_, _, _) => 2,
	    ParserProperty::Integer(_, _, _) => 3,
	    ParserProperty::Long(_, _, _) => 4,
	    ParserProperty::String(_) => 5,
	    ParserProperty::Entity(_) => 6,
	    ParserProperty::GameProfile => 7,
	    ParserProperty::BlockPos => 8,
	    ParserProperty::ColumnPos => 9,
	    ParserProperty::Vec3 => 10,
	    ParserProperty::Vec2 => 11,
	    ParserProperty::BlockState => 12,
	    ParserProperty::BlockPredicate => 13,
	    ParserProperty::ItemStack => 14,
	    ParserProperty::ItemPredicate => 15,
	    ParserProperty::Color => 16,
	    ParserProperty::Component => 17,
	    ParserProperty::Style => 18,
	    ParserProperty::Message => 19,
	    ParserProperty::Nbt => 20,
	    ParserProperty::NbtTag => 21,
	    ParserProperty::NbtPath => 22,
	    ParserProperty::Objective => 23,
	    ParserProperty::ObjectiveCriteria => 24,
	    ParserProperty::Operation => 25,
	    ParserProperty::Particle => 26,
	    ParserProperty::Angle => 27,
	    ParserProperty::Rotation => 28,
	    ParserProperty::ScoreboardSlot => 29,
	    ParserProperty::ScoreHolder(_) => 30,
	    ParserProperty::Swizzle => 31,
	    ParserProperty::Team => 32,
	    ParserProperty::ItemSlot => 33,
	    ParserProperty::ItemSlots => 34,
	    ParserProperty::ResourceLocation => 35,
	    ParserProperty::Function => 36,
	    ParserProperty::EntityAnchor => 37,
	    ParserProperty::IntRange => 38,
	    ParserProperty::FloatRange => 39,
	    ParserProperty::Dimension => 40,
	    ParserProperty::Gamemode => 41,
	    ParserProperty::Time(_) => 42,
	    ParserProperty::ResourceOrTag(_) => 43,
	    ParserProperty::ResourceOrTagKey(_) => 44,
	    ParserProperty::Resource(_) => 45,
	    ParserProperty::ResourceKey(_) => 46,
	    ParserProperty::ResourceSelector(_) => 47,
	    ParserProperty::TemplateMirror => 48,
	    ParserProperty::TemplateRotation => 49,
	    ParserProperty::Heightmap => 50,
	    ParserProperty::LootTable => 51,
	    ParserProperty::LootPredicate => 52,
	    ParserProperty::LootModifier => 53,
	    ParserProperty::Uuid => 54,
		};
  }
}

impl TryFrom<CommandNode> for Vec<u8> {
	type Error = Box<dyn Error>;

	fn try_from(value: CommandNode) -> Result<Self, Box<dyn Error>> {
		let mut output: Vec<u8> = Vec::new();

		output.push(value.flags);
		output.append(&mut crate::serialize::varint(value.children.len() as i32));
		value.children.iter().for_each(|x| output.append(&mut crate::serialize::varint(*x)));
		if value.flags & 0x08 == 0x08 {
			output.append(&mut crate::serialize::varint(value.redirect_node.unwrap()));
		}
		if value.flags & 0x03 == 1 || value.flags & 0x03 == 2 {
			output.append(&mut crate::serialize::string(&value.name.unwrap()));
		}
		if value.flags & 0x03 == 2 {
			output.append(&mut crate::serialize::varint(value.properties.clone().unwrap().into()));
			match value.properties.unwrap() {
				ParserProperty::Float(flags, min, max) => {
					output.push(flags);
					if flags & 0x01 == 0x01 {
						output.append(&mut crate::serialize::float(min.unwrap()));
					}
					if flags & 0x02 == 0x02 {
						output.append(&mut crate::serialize::float(max.unwrap()));
					}
				},
				ParserProperty::Double(flags, min, max) => {
					output.push(flags);
					if flags & 0x01 == 0x01 {
						output.append(&mut crate::serialize::double(min.unwrap()));
					}
					if flags & 0x02 == 0x02 {
						output.append(&mut crate::serialize::double(max.unwrap()));
					}
				},
				ParserProperty::Integer(flags, min, max) => {
					output.push(flags);
					if flags & 0x01 == 0x01 {
						output.append(&mut crate::serialize::int(min.unwrap()));
					}
					if flags & 0x02 == 0x02 {
						output.append(&mut crate::serialize::int(max.unwrap()));
					}
				},
				ParserProperty::Long(flags, min, max) => {
					output.push(flags);
					if flags & 0x01 == 0x01 {
						output.append(&mut crate::serialize::long(min.unwrap()));
					}
					if flags & 0x02 == 0x02 {
						output.append(&mut crate::serialize::long(max.unwrap()));
					}
				},
				ParserProperty::String(behavior) => {
					output.append(&mut crate::serialize::varint(behavior));
				},
				ParserProperty::Entity(flags) => {
					output.push(flags);
				},
				ParserProperty::ScoreHolder(flags) => {
					output.push(flags);
				},
				ParserProperty::Time(min) => {
					output.append(&mut crate::serialize::int(min));
				},
				ParserProperty::ResourceOrTag(registry) => {
					output.append(&mut crate::serialize::string(&registry));
				},
				ParserProperty::ResourceOrTagKey(registry) => {
					output.append(&mut crate::serialize::string(&registry));
				},
				ParserProperty::Resource(registry) => {
					output.append(&mut crate::serialize::string(&registry));
				},
				ParserProperty::ResourceKey(registry) => {
					output.append(&mut crate::serialize::string(&registry));
				},
				ParserProperty::ResourceSelector(registry) => {
					output.append(&mut crate::serialize::string(&registry));
				},
				_ => (),
			}
		}
		if value.flags & 0x10 == 0x10 {
			output.append(&mut crate::serialize::string(&value.suggestions_type.unwrap()));
		}

		return Ok(output);
	}
}

impl TryFrom<&mut Vec<u8>> for CommandNode {
	type Error = Box<dyn Error>;

	fn try_from(value: &mut Vec<u8>) -> Result<Self, Self::Error> {
		let flags = value.remove(0);
		let children_len = crate::deserialize::varint(value)?;
		let children: Vec<i32> = (0..children_len).map(|_| crate::deserialize::varint(value).unwrap()).collect();
		let redirect_node = if flags & 0x08 == 0x08 {
			Some(crate::deserialize::varint(value)?)
		} else {
			None
		};
		let name = if flags & 0x03 == 1 || flags & 0x03 == 2 {
			Some(crate::deserialize::string(value)?)
		} else {
			None
		};
		let properties = if flags & 0x03 == 2 {
			let properties_id = crate::deserialize::varint(value)?;
			Some(match properties_id {
				0 => ParserProperty::Bool,
				1 => {
					let flags = value.remove(0);
					let min = if flags & 0x01 == 0x01 {	Some(crate::deserialize::float(value)?) } else { None };
					let max = if flags & 0x02 == 0x02 {	Some(crate::deserialize::float(value)?) } else { None };
					ParserProperty::Float(flags, min, max)
				},
				2 => {
					let flags = value.remove(0);
					let min = if flags & 0x01 == 0x01 {	Some(crate::deserialize::double(value)?) } else {	None };
					let max = if flags & 0x02 == 0x02 {	Some(crate::deserialize::double(value)?) } else { None };
					ParserProperty::Double(flags, min, max)
				},
				3 => {
					let flags = value.remove(0);
					let min = if flags & 0x01 == 0x01 {	Some(crate::deserialize::int(value)?) } else { None };
					let max = if flags & 0x02 == 0x02 {	Some(crate::deserialize::int(value)?) } else { None };
					ParserProperty::Integer(flags, min, max)
				},
				4 => {
					let flags = value.remove(0);
					let min = if flags & 0x01 == 0x01 {	Some(crate::deserialize::long(value)?) } else {	None };
					let max = if flags & 0x02 == 0x02 {	Some(crate::deserialize::long(value)?) } else { None };
					ParserProperty::Long(flags, min, max)
				},
				5 => ParserProperty::String(crate::deserialize::varint(value)?),
				6 => ParserProperty::Entity(value.remove(0)),
				7 => ParserProperty::GameProfile,
				8 => ParserProperty::BlockPos,
				9 => ParserProperty::ColumnPos,
				10 => ParserProperty::Vec3,
				11 => ParserProperty::Vec2,
				12 => ParserProperty::BlockState,
				13 => ParserProperty::BlockPredicate,
				14 => ParserProperty::ItemStack,
				15 => ParserProperty::ItemPredicate,
				16 => ParserProperty::Color,
				17 => ParserProperty::Component,
				18 => ParserProperty::Style,
				19 => ParserProperty::Message,
				20 => ParserProperty::Nbt,
				21 => ParserProperty::NbtTag,
				22 => ParserProperty::NbtPath,
				23 => ParserProperty::Objective,
				24 => ParserProperty::ObjectiveCriteria,
				25 => ParserProperty::Operation,
				26 => ParserProperty::Particle,
				27 => ParserProperty::Angle,
				28 => ParserProperty::Rotation,
				29 => ParserProperty::ScoreboardSlot,
				30 => ParserProperty::ScoreHolder(value.remove(0)),
				31 => ParserProperty::Swizzle,
				32 => ParserProperty::Team,
				33 => ParserProperty::ItemSlot,
				34 => ParserProperty::ItemSlots,
				35 => ParserProperty::ResourceLocation,
				36 => ParserProperty::Function,
				37 => ParserProperty::EntityAnchor,
				38 => ParserProperty::IntRange,
				39 => ParserProperty::FloatRange,
				40 => ParserProperty::Dimension,
				41 => ParserProperty::Gamemode,
				42 => ParserProperty::Time(crate::deserialize::int(value)?),
				43 => ParserProperty::ResourceOrTag(crate::deserialize::string(value)?),
				44 => ParserProperty::ResourceOrTagKey(crate::deserialize::string(value)?),
				45 => ParserProperty::Resource(crate::deserialize::string(value)?),
				46 => ParserProperty::ResourceKey(crate::deserialize::string(value)?),
				47 => ParserProperty::ResourceSelector(crate::deserialize::string(value)?),
				48 => ParserProperty::TemplateMirror,
				49 => ParserProperty::TemplateRotation,
				50 => ParserProperty::Heightmap,
				51 => ParserProperty::LootTable,
				52 => ParserProperty::LootPredicate,
				53 => ParserProperty::LootModifier,
				54 => ParserProperty::Uuid,
				_ => panic!("unknown properties_id {properties_id}"),
			})
		} else {
			None
		};
		let suggestions_type = if flags & 0x10 == 0x10 {
			Some(crate::deserialize::string(value)?)
		} else {
			None
		};

		return Ok(Self {
			flags,
			children,
			redirect_node,
			name,
			properties,
			suggestions_type,
		});
  }
}
