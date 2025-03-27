use std::net::{TcpStream, SocketAddr};
use std::io::Write;
use std::collections::HashMap;
use lib::ConnectionState;

pub fn handle_packet(mut packet: lib::Packet, stream: &mut TcpStream, connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {
  println!("received new packet from {}", stream.peer_addr().unwrap());
	
	if !connection_states.contains_key(&stream.peer_addr().unwrap()) {
		connection_states.insert(stream.peer_addr().unwrap(), ConnectionState::Handshaking);
	}
	
	println!("client {} is in state {:?}", stream.peer_addr().unwrap(), connection_states.get(&stream.peer_addr().unwrap()).unwrap());
  
	return match connection_states.get(&stream.peer_addr().unwrap()).unwrap() {
    ConnectionState::Handshaking => match packet.id {
			0x00 => handshaking::handshake(&mut packet.data, stream, connection_states),
			x => {
				println!("got unrecognized packet with id {:2x?}", x);
				return true; 
			},
		},
    ConnectionState::Status => match packet.id {
			0x00 => status::status_request(&mut packet.data, stream, connection_states),
			0x01 => status::ping_request(&mut packet.data, stream, connection_states),
			x => {
				println!("got unrecognized packet with id {:2x?}", x);
				return true; 
			},
		},
    ConnectionState::Login => match packet.id {
			0x00 => login::login_start(&mut packet.data, stream),
			0x03 => login::login_acknowledged(&mut packet.data, stream, connection_states),
			x => {
				println!("got unrecognized packet with id {:2x?}", x);
				return true; 
			},
		},
    ConnectionState::Configuration => match packet.id {
      0x07 => configuration::serverbound_known_packets(&mut packet.data, stream),
      0x03 => configuration::acknowledge_finish_configuration(&mut packet.data, stream, connection_states),
      x => {
				println!("got unrecognized packet with id {:2x?}", x);
				return false; 
			},
    },
    ConnectionState::Play => match packet.id {

			x => {
				println!("got unrecognized packet with id {:2x?}", x);
				return false; 
			},
		},
    ConnectionState::Transfer => todo!(),
    }
}

pub mod handshaking {
  use super::*;

  pub fn handshake(data: &mut Vec<u8>, stream: &mut TcpStream, connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {
    let parsed_packet = lib::packets::serverbound::handshaking::Handshake::try_from(data.clone()).unwrap();

    connection_states.insert(stream.peer_addr().unwrap(), parsed_packet.next_state.into());
  
    return false;
  }
}

pub mod status {
  use super::*;

  pub fn status_request(_data: &mut Vec<u8>, stream: &mut TcpStream, _connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {
    lib::utils::send_packet(stream, 0x00, lib::packets::clientbound::status::StatusResponse {
      status: "{\"version\": {\"name\": \"Oxide 1.21.4\",\"protocol\": 769},\"players\": {\"max\": 9,\"online\": 6,\"sample\": []},\"description\": {\"text\": \"Hello oxide!\"},\"enforcesSecureChat\": true}".to_string(),
    }.try_into().unwrap());
    return false;
  }
  
  pub fn ping_request(data: &mut Vec<u8>, stream: &mut TcpStream, _connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {
    let mut output: Vec<u8> = Vec::new();
    output.push(9);
    output.push(1);
    output.append(data);
    stream.write(output.as_slice()).unwrap();
    return true;
  }
}

pub mod login {
  use super::*;

  pub fn login_start(data: &mut Vec<u8>, stream: &mut TcpStream) -> bool {
    lib::utils::send_packet(stream, 0x02, lib::packets::clientbound::login::LoginSuccess {
      uuid: 290780920670370370148908686767547353505,
      username: "The__TxT".to_string()
    }.try_into().unwrap());

    return false;
  }

  pub fn login_acknowledged(data: &mut Vec<u8>, stream: &mut TcpStream, connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {
    connection_states.insert(stream.peer_addr().unwrap(), ConnectionState::Configuration);

    lib::utils::send_packet(stream, 0x0e, lib::packets::clientbound::configuration::ClientboundKnownPacks {
      known_packs: vec![lib::Datapack { namespace: "minecraft".to_string(), id: "core".to_string(), version: "1.21.4".to_string() }],
    }.try_into().unwrap());

    return false;
  }

}

pub mod configuration {
  use super::*;

  pub fn serverbound_known_packets(data: &mut Vec<u8>, stream: &mut TcpStream) -> bool {
    lib::utils::send_packet(stream, 0x07, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:worldgen/biome".to_string(),
      entry_count: 65,
      entries: vec![
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:badlands".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:bamboo_jungle".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:basalt_deltas".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:beach".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:birch_forest".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:cherry_grove".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:cold_ocean".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:crimson_forest".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:dark_forest".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:deep_cold_ocean".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:deep_dark".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:deep_frozen_ocean".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:deep_lukewarm_ocean".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:deep_ocean".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:desert".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:dripstone_caves".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:end_barrens".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:end_highlands".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:end_midlands".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:eroded_badlands".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:flower_forest".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:forest".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:frozen_ocean".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:frozen_peaks".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:frozen_river".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:grove".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:ice_spikes".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:jagged_peaks".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:jungle".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:lukewarm_ocean".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:lush_caves".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:mangrove_swamp".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:meadow".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:mushroom_fields".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:nether_wastes".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:ocean".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:old_growth_birch_forest".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:old_growth_pine_taiga".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:old_growth_spruce_taiga".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:pale_garden".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:plains".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:river".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:savanna".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:savanna_plateau".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:small_end_islands".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:snowy_beach".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:snowy_plains".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:snowy_slopes".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:snowy_taiga".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:soul_sand_valley".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:sparse_jungle".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:stony_peaks".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:stony_shore".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:sunflower_plains".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:swamp".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:taiga".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:the_end".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:the_void".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:warm_ocean".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:warped_forest".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:windswept_forest".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:windswept_gravelly_hills".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:windswept_hills".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:windswept_savanna".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:wooded_badlands".to_string(),
          has_data: false,
          data: None,
        },
      ]
    }.try_into().unwrap());
    lib::utils::send_packet(stream, 0x07, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:dimension_type".to_string(),
      entry_count: 4,
      entries: vec![
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:overworld".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:overworld_caves".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:the_end".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:the_nether".to_string(),
          has_data: false,
          data: None,
        },
      ]
    }.try_into().unwrap());
    lib::utils::send_packet(stream, 0x07, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:painting_variant".to_string(),
      entry_count: 1,
      entries: vec![lib::packets::clientbound::configuration::RegistryDataEntry {
        entry_id: "minecraft:alban".to_string(),
        has_data: false,
        data: None,
      }]
    }.try_into().unwrap());
    lib::utils::send_packet(stream, 0x07, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:wolf_variant".to_string(),
      entry_count: 1,
      entries: vec![lib::packets::clientbound::configuration::RegistryDataEntry {
        entry_id: "minecraft:ashen".to_string(),
        has_data: false,
        data: None,
      }]
    }.try_into().unwrap());
    lib::utils::send_packet(stream, 0x07, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:damage_type".to_string(),
      entry_count: 49,
      entries: vec![
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:arrow".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:bad_respawn_point".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:cactus".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:campfire".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:cramming".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:dragon_breath".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:drown".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:dry_out".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:ender_pearl".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:explosion".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:fall".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:falling_anvil".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:falling_block".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:falling_stalactite".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:fireball".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:fireworks".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:fly_into_wall".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:freeze".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:generic".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:generic_kill".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:hot_floor".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:in_fire".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:in_wall".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:indirect_magic".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:lava".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:lightning_bolt".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:mace_smash".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:magic".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:mob_attack".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:mob_attack_no_aggro".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:mob_projectile".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:on_fire".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:out_of_world".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:outside_border".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:player_attack".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:player_explosion".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:sonic_boom".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:spit".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:stalagmite".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:starve".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:sting".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:sweet_berry_bush".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:thorns".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:thrown".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:trident".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:unattributed_fireball".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:wind_charge".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:wither".to_string(),
          has_data: false,
          data: None,
        },
        lib::packets::clientbound::configuration::RegistryDataEntry {
          entry_id: "minecraft:wither_skull".to_string(),
          has_data: false,
          data: None,
        },
      ]
    }.try_into().unwrap());

    lib::utils::send_packet(stream, 0x03, lib::packets::clientbound::configuration::FinishConfiguration {

    }.try_into().unwrap());

    return false;
  }

  pub fn acknowledge_finish_configuration(data: &mut Vec<u8>, stream: &mut TcpStream, connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {
    connection_states.insert(stream.peer_addr().unwrap(), ConnectionState::Play);

    lib::utils::send_packet(stream, 0x2c, lib::packets::clientbound::play::Login {
      entity_id: 123456,
      is_hardcore: false,
      dimension_names: vec!["minecraft:overworld".to_string()],
      max_players: 9,
      view_distance: 32,
      simulation_distance: 32,
      reduced_debug_info: false,
      enable_respawn_screen: true,
      do_limited_crafting: false,
      dimension_type: 0,
      dimension_name: "minecraft:overworld".to_string(),
      hashed_seed: 1,
      game_mode: 1,
      previous_game_mode: -1,
      is_debug: false,
      is_flat: false,
      has_death_location: false,
      death_dimension_name: None,
      death_location: None,
      portal_cooldown: 123,
      sea_level: 64,
      enforces_secure_chat: false,
    }.try_into().unwrap());

    lib::utils::send_packet(stream, 0x23, lib::packets::clientbound::play::GameEvent {
      event: 13,
      value: 0.0,
    }.try_into().unwrap());

    for x in -20..21 {
      for z in -20..21 {
        lib::utils::send_packet(stream, 0x28, lib::packets::clientbound::play::ChunkDataAndUpdateLight {
          chunk_x: x,
          chunk_z: z,
          heightmaps: lib::nbt::NbtTag::TagCompound(None, vec![
            lib::nbt::NbtTag::LongArray(Some("MOTION_BLOCKING".to_string()), vec![]),
            lib::nbt::NbtTag::LongArray(Some("MOTION_BLOCKING".to_string()), vec![]),
          ]),
          data: vec![lib::packets::clientbound::play::ChunkSection {
            block_count: 1,
            block_states: lib::packets::clientbound::play::PalettedContainer::SingleValued(lib::packets::clientbound::play::SingleValued {
              bits_per_entry: 1,
              value: 1,
              data_array: vec![1]
            }),
            biomes: lib::packets::clientbound::play::PalettedContainer::SingleValued(lib::packets::clientbound::play::SingleValued {
              bits_per_entry: 0,
              value: 0,
              data_array: vec![]
            }),
          }; 24],
          block_entities: vec![],
          sky_light_mask: vec![],
          block_light_mask: vec![],
          empty_sky_light_mask: vec![],
          empty_block_light_mask: vec![],
          sky_light_arrays: vec![],
          block_light_arrays: vec![],
        }.try_into().unwrap());
      }
    }

    return false;
  }
}

pub mod play {
  use super::*;


}