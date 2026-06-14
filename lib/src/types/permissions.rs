use std::{
	fs::{self, File},
	path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub enum Permission {
	Everyone,
	Operator,
	Moderator,
	Gamemaster,
	Admin,
	Owner,
}

impl From<i16> for Permission {
	fn from(value: i16) -> Self {
		match value {
			0 => Permission::Operator,
			1 => Permission::Moderator,
			2 => Permission::Gamemaster,
			3 => Permission::Admin,
			4 => Permission::Owner,
			_ => Permission::Owner,
		}
	}
}

impl From<Permission> for i16 {
    fn from(value: Permission) -> Self {
        match value {
            Permission::Operator => 0,
            Permission::Moderator => 1,
            Permission::Gamemaster=> 2,
            Permission::Admin => 3,
            Permission::Owner => 4,
            _ => 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpsItem {
	pub uuid: u128,
	pub name: String,
	pub level: i16,
	#[serde(rename = "bypassesPlayerLimit")]
	pub bypasses_player_limit: bool,
}

impl PartialEq for OpsItem {
	fn eq(&self, other: &Self) -> bool {
		self.uuid == other.uuid
	}
}

pub fn calculate_level_for_protocol(permission: Permission) -> u8 {
	let permission_level = i16::from(permission) as u8;
	permission_level + 24 
}

pub fn add_permission_in_file(permission: OpsItem) {
	let mut permission = permission;
	let mut deserialized = get_data(); 
	if deserialized.contains(&permission) {
		let mut index = deserialized.iter().position(|p| *p == permission).unwrap();
		std::mem::swap(&mut deserialized[*&mut index], &mut permission);
	} else {
    	deserialized.push(permission);
	}
	let serialized = serde_json::to_string(&deserialized).unwrap();
	fs::write(get_ops_file_path(), serialized).unwrap();
	
}

pub fn get_permission_from_file(uuid: u128) -> Permission {
	let deserialized = get_data();
	for player in deserialized {
		if player.uuid == uuid {
			let player_permission = player.level.into();
			return player_permission;
		}
	}

	Permission::Everyone
}

pub fn remove_permission_from_file(uuid: u128) {
	let mut deserialized = get_data(); 
	if let Some(index) = deserialized.iter().position(|p| p.uuid == uuid) {
		deserialized.remove(index);
		let serialized = serde_json::to_string(&deserialized).unwrap();
		fs::write(get_ops_file_path(), serialized).unwrap();
	}
	
}

fn get_ops_file_path() -> PathBuf {
	let world_path = Path::new(&std::env::var("OXIDE_WORLD_PATH").unwrap_or("./world".to_string())).to_owned();
	world_path.parent().unwrap().with_file_name("ops.json")
}

fn get_data() -> Vec<OpsItem> {
	let data = fs::read_to_string(get_ops_file_path()).unwrap_or(String::new());
	if data.is_empty() {
		File::create(get_ops_file_path()).unwrap();
		return vec![];
	}
	let deserialized: Vec<OpsItem> = serde_json::from_str(&data).unwrap_or(vec![]);
	deserialized
}
