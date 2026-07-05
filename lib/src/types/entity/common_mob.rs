use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CommonMob {
	pub absorption_amount: f32,
	pub active_effects: Vec<NbtListTag>,
	pub attributes: Vec<NbtListTag>,
	pub brain: NbtTag,
	pub can_pick_up_loot: bool,
	pub death_loot_table: Option<String>,
	pub death_loot_table_seed: Option<i64>,
	pub death_time: i16,
	pub drop_chances: NbtTag,
	pub equipment: HashMap<String, Slot>,
	pub fall_flying: u8,
	pub health: f32,
	pub home_location: (i32, i32, i32),
	pub home_radius: i32,
	pub hurt_by_timestamp: i32,
	pub hurt_time: i16,
	pub leashed_block: Option<(i32, i32, i32)>,
	pub leashed_entity: Option<u128>,
	pub is_left_handed: bool,
	pub locator_bar_icon_color: Option<i32>,
	pub locator_bar_icon_style: Option<String>,
	pub has_no_ai: bool,
	pub is_persistance_required: bool,
	pub sleeping_location: Option<(i32, i32, i32)>,
	pub team: Option<String>,
	pub alive_for_ticks: i32,
	pub wander_to: Option<BlockPosition>,
	pub wandered_for: u16,
	pub drop_items_upon_death: bool,
}

impl Default for CommonMob {
	fn default() -> Self {
		Self {
			absorption_amount: Default::default(),
			active_effects: Default::default(),
			attributes: Default::default(),
			brain: Default::default(),
			can_pick_up_loot: Default::default(),
			death_loot_table: Default::default(),
			death_loot_table_seed: Default::default(),
			death_time: Default::default(),
			drop_chances: Default::default(),
			equipment: Default::default(),
			fall_flying: Default::default(),
			health: Default::default(),
			home_location: Default::default(),
			home_radius: Default::default(),
			hurt_by_timestamp: Default::default(),
			hurt_time: Default::default(),
			leashed_block: Default::default(),
			leashed_entity: Default::default(),
			is_left_handed: Default::default(),
			locator_bar_icon_color: Default::default(),
			locator_bar_icon_style: Default::default(),
			has_no_ai: Default::default(),
			is_persistance_required: Default::default(),
			sleeping_location: Default::default(),
			team: Default::default(),
			alive_for_ticks: Default::default(),
			wander_to: Default::default(),
			wandered_for: Default::default(),
			drop_items_upon_death: true,
		}
	}
}


impl CommonMob {
	pub fn from_nbt(data: NbtListTag) -> CommonMob {
		let mut output = CommonMob::default();

		if let Some(value) = data.get_child("AbsorptionAmount") {
			output.absorption_amount = value.as_float();
		}
		if let Some(value) = data.get_child("active_effects") {
			output.active_effects = value.as_list();
		}
		if let Some(value) = data.get_child("attributes") {
			output.attributes = value.as_list();
		}
		if let Some(value) = data.get_child("brain") {
			output.brain = value.clone();
		}
		if let Some(value) = data.get_child("CanPickUpLoot") {
			output.can_pick_up_loot = value.as_byte() == 1;
		}
		if let Some(value) = data.get_child("DeathLootTable") {
			output.death_loot_table = Some(value.as_string().to_string());
		}
		if let Some(value) = data.get_child("DeathLootTableSeed") {
			output.death_loot_table_seed = Some(value.as_long());
		}
		if let Some(value) = data.get_child("DeathTime") {
			output.death_time = value.as_short();
		}
		if let Some(value) = data.get_child("drop_chances") {
			output.drop_chances = value.clone();
		}
		if let Some(value) = data.get_child("equipment") {
			let mut equipment: HashMap<String, Slot> = HashMap::new();
			if let Some(x) = value.get_child("head") {
				equipment.insert("head".to_string(), x.clone().into());
			}
			if let Some(x) = value.get_child("chest") {
				equipment.insert("chest".to_string(), x.clone().into());
			}
			if let Some(x) = value.get_child("legs") {
				equipment.insert("legs".to_string(), x.clone().into());
			}
			if let Some(x) = value.get_child("feet") {
				equipment.insert("feet".to_string(), x.clone().into());
			}
			if let Some(x) = value.get_child("mainhand") {
				equipment.insert("mainhand".to_string(), x.clone().into());
			}
			if let Some(x) = value.get_child("offhand") {
				equipment.insert("offhand".to_string(), x.clone().into());
			}
			if let Some(x) = value.get_child("body") {
				equipment.insert("body".to_string(), x.clone().into());
			}
			if let Some(x) = value.get_child("saddle") {
				equipment.insert("saddle".to_string(), x.clone().into());
			}
		}
		if let Some(value) = data.get_child("fall_flying") {
			output.fall_flying = value.as_byte();
		}
		if let Some(value) = data.get_child("health") {
			output.health = value.as_float();
		} else {
			output.health = 20.0
		} //TODO: find a proper way to assign default values like health
		if let Some(value) = data.get_child("home_pos") {
			output.home_location = (value.as_int_array()[0], value.as_int_array()[1], value.as_int_array()[2]);
		}
		if let Some(value) = data.get_child("home_radius") {
			output.home_radius = value.as_int();
		}
		if let Some(value) = data.get_child("HurtByTimestamp") {
			output.hurt_by_timestamp = value.as_int();
		}
		if let Some(value) = data.get_child("HurtTime") {
			output.hurt_time = value.as_short();
		}
		if let Some(value) = data.get_child("leash") {
			match value {
				NbtTag::TagCompound(_, _) => {
					output.leashed_block = Some((value.as_int_array()[0], value.as_int_array()[1], value.as_int_array()[2]))
				}
				NbtTag::IntArray(_, _) => {
					output.leashed_entity = Some(
						value
							.get_child("UUID")
							.unwrap()
							.as_int_array()
							.into_iter()
							.enumerate()
							.map(|x| (x.1 as u128) << (32 * (3 - x.0)))
							.reduce(|a, b| a | b)
							.unwrap(),
					)
				}
				_ => (),
			}
		}
		if let Some(value) = data.get_child("LeftHanded") {
			output.is_left_handed = value.as_byte() == 1;
		}
		if let Some(value) = data.get_child("locator_bar_icon") {
			output.locator_bar_icon_color = Some(value.get_child("color").unwrap().as_int());
			output.locator_bar_icon_style = Some(value.get_child("color").unwrap().as_string().to_string());
		}
		if let Some(value) = data.get_child("NoAI") {
			output.has_no_ai = value.as_byte() == 1;
		}
		if let Some(value) = data.get_child("PersistanceRequired") {
			output.is_persistance_required = value.as_byte() == 1;
		}
		if let Some(value) = data.get_child("sleeping_pos") {
			output.sleeping_location = Some((value.as_int_array()[0], value.as_int_array()[1], value.as_int_array()[2]));
		}

		return output;
	}

	pub fn to_nbt(&self) -> Vec<NbtTag> {
		let mut output: Vec<NbtTag> = vec![
			NbtTag::Float("AbsorptionAmount".to_string(), self.absorption_amount),
			NbtTag::List("active_effects".to_string(), self.active_effects.clone()),
			NbtTag::List("attributes".to_string(), self.attributes.clone()),
			self.brain.clone(),
			NbtTag::Byte("CanPickUpLoot".to_string(), if self.can_pick_up_loot { 1 } else { 0 }),
			NbtTag::Short("DeathTime".to_string(), self.death_time),
			self.drop_chances.clone(),
			NbtTag::Byte("FallFlying".to_string(), self.fall_flying),
			NbtTag::Float("Health".to_string(), self.health),
			NbtTag::IntArray("home_pos".to_string(), vec![self.home_location.0, self.home_location.1, self.home_location.2]),
			NbtTag::Int("home_radius".to_string(), self.home_radius),
			NbtTag::Int("HurtByTimestamp".to_string(), self.hurt_by_timestamp),
			NbtTag::Short("HurtTime".to_string(), self.hurt_time),
			NbtTag::Byte("LeftHanded".to_string(), if self.is_left_handed { 1 } else { 0 }),
			NbtTag::Byte("NoAI".to_string(), if self.has_no_ai { 1 } else { 0 }),
			NbtTag::Byte("PersitanceRequired".to_string(), if self.is_persistance_required { 1 } else { 0 }),
		];

		if let Some(value) = self.death_loot_table.clone() {
			output.push(NbtTag::String("DeathLootTable".to_string(), value));
		}
		if let Some(value) = self.death_loot_table_seed {
			output.push(NbtTag::Long("DeathLootTableSeed".to_string(), value));
		}
		if !self.equipment.is_empty() {
			let mut entries: Vec<NbtTag> = Vec::new();
			for x in &self.equipment {
				entries.push(NbtTag::TagCompound(x.0.clone(), x.1.clone().into()));
			}
		}
		if let Some(value) = self.leashed_block {
			output.push(NbtTag::IntArray("leash".to_string(), vec![value.0, value.1, value.2]));
		}
		if let Some(value) = self.leashed_entity {
			output.push(NbtTag::TagCompound(
				"leash".to_string(),
				vec![NbtTag::IntArray(
					"UUID".to_string(),
					vec![(value >> 96) as i32, (value << 32 >> 96) as i32, (value << 64 >> 96) as i32, (value << 96 >> 96) as i32],
				)],
			));
		}
		if let Some(locator_bar_icon_color) = self.locator_bar_icon_color {
			output.push(NbtTag::TagCompound(
				"locator_bar_icon".to_string(),
				vec![
					NbtTag::Int("color".to_string(), locator_bar_icon_color),
					NbtTag::String("style".to_string(), self.locator_bar_icon_style.clone().unwrap()),
				],
			));
		}
		if let Some(value) = self.sleeping_location {
			output.push(NbtTag::IntArray("sleeping_pos".to_string(), vec![value.0, value.1, value.2]));
		}

		return output;
	}
}
