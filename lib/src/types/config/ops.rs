use std::{fs::{self, File}, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};

use crate::Permission;

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

pub fn add_permission_in_file(permission: OpsItem) {
    let mut permission = permission;
    if let Some(mut deserialized) = get_data() {
        if deserialized.contains(&permission) {
        let mut index = deserialized.iter().position(|p| *p == permission).unwrap();
        std::mem::swap(&mut deserialized[*&mut index], &mut permission);
    }
    else {
        deserialized.push(permission);
    }
    let serialized = serde_json::to_string(&deserialized).unwrap();
    fs::write(get_ops_file_path(), serialized).unwrap();
    }
}

pub fn get_permission_from_file(uuid: u128) -> Permission {
    if let Some(deserialized) = get_data() {
        for player in deserialized {
            if player.uuid == uuid {
                let player_permission = player.level.into();
                return player_permission;
            }
        }
    }
    
    Permission::Everyone
}

pub fn remove_permission_from_file(uuid: u128) {
    if let Some(mut deserialized) = get_data() {
        if let Some(index) = deserialized.iter().position(|p| p.uuid == uuid) {
            deserialized.remove(index);
            let serialized = serde_json::to_string(&deserialized).unwrap();
            fs::write(get_ops_file_path(), serialized).unwrap();
        }
    }
}

fn get_ops_file_path() -> PathBuf {
    let world_path = Path::new(&std::env::var("OXIDE_WORLD_PATH").unwrap_or("./world".to_string())).to_owned();
    world_path.parent().unwrap().with_file_name("ops.json")
}

fn get_data() -> Option<Vec<OpsItem>> {
    let data = fs::read_to_string(get_ops_file_path()).unwrap_or(String::new());
    if data.is_empty() {
        File::create(get_ops_file_path()).unwrap();
        return None;
    }
    let deserialized: Vec<OpsItem> = serde_json::from_str(&data).unwrap_or(vec![]);
    Some(deserialized)
}
