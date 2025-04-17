use std::net::{TcpStream, SocketAddr};
use std::io::Write;
use std::collections::HashMap;
use lib::ConnectionState;
use crate::server::{Game, Player, Connection};

pub fn handle_packet(mut packet: lib::Packet, stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>, game: &mut Game) -> bool {
  //println!("received new packet from {}", stream.peer_addr().unwrap());
	
	if !connections.contains_key(&stream.peer_addr().unwrap()) {
		connections.insert(stream.peer_addr().unwrap(), Connection { state: ConnectionState::Handshaking, peer_address: stream.peer_addr().unwrap(), player_name: None, player_uuid: None });
	}
	
	if !connection_streams.contains_key(&stream.peer_addr().unwrap()) {
		connection_streams.insert(stream.peer_addr().unwrap(), stream.try_clone().unwrap());
	}
	
	//println!("client {} is in state {:?}", stream.peer_addr().unwrap(), connection_states.get(&stream.peer_addr().unwrap()).unwrap());
  
	return match connections.get(&stream.peer_addr().unwrap()).unwrap().state {
    ConnectionState::Handshaking => match packet.id {
			0x00 => handshaking::handshake(&mut packet.data, stream, connections),
			x => {
				//println!("got unrecognized packet with id {:2x?}", x);
				return true; 
			},
		},
    ConnectionState::Status => match packet.id {
			0x00 => status::status_request(stream, game),
			0x01 => status::ping_request(&mut packet.data, stream),
			x => {
				//println!("got unrecognized packet with id {:2x?}", x);
				return true; 
			},
		},
    ConnectionState::Login => match packet.id {
			0x00 => login::login_start(&mut packet.data, stream, connections),
			0x03 => login::login_acknowledged(stream, connections),
			x => {
				//println!("got unrecognized packet with id {:2x?}", x);
				return true; 
			},
		},
    ConnectionState::Configuration => match packet.id {
      0x07 => configuration::serverbound_known_packets(stream),
      0x03 => configuration::acknowledge_finish_configuration(stream, connections, game, connection_streams),
      x => {
				//println!("got unrecognized packet with id {:2x?}", x);
				return false; 
			},
    },
    ConnectionState::Play => match packet.id {
      0x00 => play::confirm_teleportation(&mut packet.data, game, stream, connections),
      0x1c => play::set_player_position(&mut packet.data, game, stream, connections, connection_streams),
      0x1d => play::set_player_position_and_rotation(&mut packet.data, game, stream, connections, connection_streams),
      0x1e => play::set_player_rotation(&mut packet.data, game, stream, connections, connection_streams),
			x => {
				//println!("got unrecognized packet with id {:2x?}", x);
				return false; 
			},
		},
    ConnectionState::Transfer => todo!(),
    }
}

pub mod handshaking {
  use super::*;

  pub fn handshake(data: &mut Vec<u8>, stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>) -> bool {
    let parsed_packet = lib::packets::serverbound::handshaking::Handshake::try_from(data.clone()).unwrap();

    connections.entry(stream.peer_addr().unwrap()).and_modify(|x| x.state = parsed_packet.next_state.into());
  
    return false;
  }
}

pub mod status {
  use super::*;

  pub fn status_request(stream: &mut TcpStream, game: &mut Game) -> bool {
    lib::utils::send_packet(stream, 0x00, lib::packets::clientbound::status::StatusResponse {
      status: format!("{{\"version\": {{\"name\": \"Oxide 1.21.4\",\"protocol\": 769}},\"players\": {{\"max\": -1,\"online\": {},\"sample\": []}},\"description\": {{\"text\": \"Hello oxide!\"}},\"enforcesSecureChat\": true}}", game.players.len()).to_string(),
    }.try_into().unwrap());
    return false;
  }
  
  pub fn ping_request(data: &mut Vec<u8>, stream: &mut TcpStream) -> bool {
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

  pub fn login_start(data: &mut Vec<u8>, stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>) -> bool {
    let parsed_packet = lib::packets::serverbound::login::LoginStart::try_from(data.clone()).unwrap();
    
    connections.entry(stream.peer_addr().unwrap()).and_modify(|x| {
      x.player_name = Some(parsed_packet.name.clone());
      x.player_uuid = Some(parsed_packet.uuid);
    });

    lib::utils::send_packet(stream, 0x02, lib::packets::clientbound::login::LoginSuccess {
      uuid: parsed_packet.uuid,
      username: parsed_packet.name,
    }.try_into().unwrap());

    return false;
  }

  pub fn login_acknowledged(stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>) -> bool {
    connections.entry(stream.peer_addr().unwrap()).and_modify(|x| x.state = ConnectionState::Configuration);

    lib::utils::send_packet(stream, 0x0e, lib::packets::clientbound::configuration::ClientboundKnownPacks {
      known_packs: vec![lib::Datapack { namespace: "minecraft".to_string(), id: "core".to_string(), version: "1.21.4".to_string() }],
    }.try_into().unwrap());

    return false;
  }

}

pub mod configuration {
  use super::*;

  pub fn serverbound_known_packets(stream: &mut TcpStream) -> bool {
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

  pub fn acknowledge_finish_configuration(stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>, game: &mut Game, connection_streams: &mut HashMap<SocketAddr, TcpStream>) -> bool {
    connections.entry(stream.peer_addr().unwrap()).and_modify(|x| x.state = ConnectionState::Play);

    let current_player = Player { 
      x: 0.0,
      y_feet: -48.0,
      z: 0.0,
      yaw: 0.0,
      pitch: 0.0,
      display_name: connections.get(&stream.peer_addr().unwrap()).unwrap().player_name.clone().unwrap_or_default(),
      uuid: connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.clone().unwrap_or_default(),
      peer_socket_address: stream.peer_addr().unwrap(),
      entity_id: game.last_created_entity_id + 1,
      waiting_for_confirm_teleportation: true,
      current_teleport_id: Some((std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() / (game.last_created_entity_id + 1 + 12345) as u64) as i32), //TODO: use random number instead
    };
    game.last_created_entity_id += 1;

    lib::utils::send_packet(stream, 0x2c, lib::packets::clientbound::play::Login {
      entity_id: current_player.entity_id,
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

    lib::utils::send_packet(stream, 0x42, lib::packets::clientbound::play::SynchronizePlayerPosition {
      teleport_id: current_player.current_teleport_id.unwrap(),
      x: current_player.x,
      y: current_player.y_feet,
      z: current_player.z,
      velocity_x: 0.0,
      velocity_y: 0.0,
      velocity_z: 0.0,
      yaw: current_player.yaw,
      pitch: current_player.pitch,
      flags: 0,
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

    game.players.push(current_player.clone());

    update_players(connection_streams, connections, game.players.clone(), None);

    //Spawn other already connected player entities for newly joined player
    for player in &game.players {
      if player.uuid == current_player.uuid {
        continue;
      }

      lib::utils::send_packet(stream, 0x01, lib::packets::clientbound::play::SpawnEntity {
        entity_id: player.entity_id,
        entity_uuid: player.uuid,
        entity_type: 147, //Player
        x: player.x,
        y: player.y_feet,
        z: player.z,
        pitch: 0,
        yaw: 0,
        head_yaw: 0,
        data: 0,
        velocity_x: 0,
        velocity_y: 0,
        velocity_z: 0,
      }.try_into().unwrap());

      lib::utils::send_packet(stream, 0x5d, lib::packets::clientbound::play::SetEntityMetadata {
        entity_id: player.entity_id,
        metadata: vec![
          lib::packets::clientbound::play::EntityMetadata {
            index: 9,
            value: lib::packets::clientbound::play::EntityMetadataValue::Float(20.0),
          },
          lib::packets::clientbound::play::EntityMetadata {
            index: 17,
            value: lib::packets::clientbound::play::EntityMetadataValue::Byte(127),
          },
        ],
      }.try_into().unwrap());
    }

    //Spawn player entity for other players that are already connected
    for player in &game.players {
      if player.peer_socket_address == stream.peer_addr().unwrap() {
        continue;
      }
      let player_stream = connection_streams.get(&player.peer_socket_address).unwrap();
    
      lib::utils::send_packet(player_stream, 0x01, lib::packets::clientbound::play::SpawnEntity {
        entity_id: current_player.entity_id,
        entity_uuid: current_player.uuid,
        entity_type: 147, //Player
        x: current_player.x,
        y: current_player.y_feet,
        z: current_player.z,
        pitch: 0,
        yaw: 0,
        head_yaw: 0,
        data: 0,
        velocity_x: 0,
        velocity_y: 0,
        velocity_z: 0,
      }.try_into().unwrap());

      lib::utils::send_packet(player_stream, 0x5d, lib::packets::clientbound::play::SetEntityMetadata {
        entity_id: current_player.entity_id,
        metadata: vec![
          lib::packets::clientbound::play::EntityMetadata {
            index: 9,
            value: lib::packets::clientbound::play::EntityMetadataValue::Float(20.0),
          },
          lib::packets::clientbound::play::EntityMetadata {
            index: 17,
            value: lib::packets::clientbound::play::EntityMetadataValue::Byte(127),
          },
        ],
      }.try_into().unwrap());
    }

    //TODO: get rid of this once we have a real game loop
    let stream_clone = stream.try_clone().unwrap();
    std::thread::spawn(move || {
      loop {
        let useless_buf_no_one_crates_about = &mut [0; 1];
        if stream_clone.peek(useless_buf_no_one_crates_about).is_err() {
          return;
        }
        lib::utils::send_packet(&stream_clone, 0x27, lib::packets::clientbound::play::ClientboundKeepAlive {
          keep_alive_id: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
        }.try_into().unwrap());

        std::thread::sleep(std::time::Duration::from_secs(5));
      }
    });

    return false;
  }
}

pub mod play {
  use super::*;

  pub fn set_player_position(data: &mut Vec<u8>, game: &mut Game, stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>) -> bool {
    let parsed_packet = lib::packets::serverbound::play::SetPlayerPosition::try_from(data.clone()).unwrap();
    let uuid = connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.unwrap();

    let mut player = game.players.iter().find(|x| x.uuid == uuid).unwrap().clone();
    if player.waiting_for_confirm_teleportation {
      return false;
    }
    game.players = game.players.iter().filter(|x| x.uuid != uuid).cloned().collect();

    let old_x = player.x;
    let old_y_feet = player.y_feet;
    let old_z = player.z;

    player.x = parsed_packet.x;
    player.y_feet = parsed_packet.feet_y;
    player.z = parsed_packet.z;

    game.players.push(player.clone());

    let default_connection = Connection::default();
    for other_stream in connection_streams {
      if other_stream.0.clone() != stream.peer_addr().unwrap() && connections.get(&other_stream.0).unwrap_or(&default_connection).state == ConnectionState::Play {
        /* lib::utils::send_packet(other_stream.1, 0x2f, lib::packets::clientbound::play::UpdateEntityPosition {
          entity_id: player.entity_id,
          delta_x: ((player.x * 4096.0) - (old_x * 4096.0)) as i16,
          delta_y: ((player.y_feet * 4096.0) - (old_y_feet * 4096.0)) as i16,
          delta_z: ((player.z * 4096.0) - (old_z * 4096.0)) as i16,
          on_ground: player.y_feet == -48.0, //TODO: add proper check
        }.try_into().unwrap()); */

        lib::utils::send_packet(other_stream.1, 0x20, lib::packets::clientbound::play::TeleportEntity {
          entity_id: player.entity_id,
          x: player.x,
          y: player.y_feet,
          z: player.z,
          velocity_x: 0.0,
          velocity_y: 0.0,
          velocity_z: 0.0,
          yaw: player.yaw,
          pitch: player.pitch,
          on_ground: player.y_feet == -48.0,
        }.try_into().unwrap());
      }
    }

    return false;
  }

  pub fn set_player_position_and_rotation(data: &mut Vec<u8>, game: &mut Game, stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>) -> bool {
    let parsed_packet = lib::packets::serverbound::play::SetPlayerPositionAndRotation::try_from(data.clone()).unwrap();
    let uuid = connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.unwrap();

    let mut player = game.players.iter().find(|x| x.uuid == uuid).unwrap().clone();
    if player.waiting_for_confirm_teleportation {
      return false;
    }
    game.players = game.players.iter().filter(|x| x.uuid != uuid).cloned().collect();

    player.x = parsed_packet.x;
    player.y_feet = parsed_packet.feet_y;
    player.z = parsed_packet.z;
    player.yaw = parsed_packet.yaw % 360.0;
    player.pitch = parsed_packet.pitch % 360.0;

    game.players.push(player.clone());

    let default_connection = Connection::default();
    for other_stream in connection_streams {
      if other_stream.0.clone() != stream.peer_addr().unwrap() && connections.get(&other_stream.0).unwrap_or(&default_connection).state == ConnectionState::Play {
        lib::utils::send_packet(other_stream.1, 0x20, lib::packets::clientbound::play::TeleportEntity {
          entity_id: player.entity_id,
          x: player.x,
          y: player.y_feet,
          z: player.z,
          velocity_x: 0.0,
          velocity_y: 0.0,
          velocity_z: 0.0,
          yaw: player.yaw,
          pitch: player.pitch,
          on_ground: player.y_feet == -48.0,
        }.try_into().unwrap());
      }
    }

    return false;
  }

  pub fn set_player_rotation(data: &mut Vec<u8>, game: &mut Game, stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>) -> bool {
    let parsed_packet = lib::packets::serverbound::play::SetPlayerRotation::try_from(data.clone()).unwrap();
    let uuid = connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.unwrap();

    let mut player = game.players.iter().find(|x| x.uuid == uuid).unwrap().clone();
    if player.waiting_for_confirm_teleportation {
      return false;
    }
    game.players = game.players.iter().filter(|x| x.uuid != uuid).cloned().collect();

    //player.yaw = if parsed_packet.yaw % 360.0 > 180.0 {(parsed_packet.yaw % 360.0) - 360.0} else {parsed_packet.yaw % 360.0};
    player.yaw = parsed_packet.yaw % 360.0;
    player.pitch = parsed_packet.pitch % 360.0;

    game.players.push(player.clone());

    let default_connection = Connection::default();
    for other_stream in connection_streams {
      if other_stream.0.clone() != stream.peer_addr().unwrap() && connections.get(&other_stream.0).unwrap_or(&default_connection).state == ConnectionState::Play {
        lib::utils::send_packet(other_stream.1, 0x20, lib::packets::clientbound::play::TeleportEntity {
          entity_id: player.entity_id,
          x: player.x,
          y: player.y_feet,
          z: player.z,
          velocity_x: 0.0,
          velocity_y: 0.0,
          velocity_z: 0.0,
          yaw: player.yaw,
          pitch: player.pitch,
          on_ground: player.y_feet == -48.0,
        }.try_into().unwrap());
      }
    }

    return false;
  }

  pub fn confirm_teleportation(data: &mut Vec<u8>, game: &mut Game, stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>) -> bool {
    let player = game.players.iter().find(|x| x.uuid == connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.unwrap_or_default());
    if player.is_none() {
      println!("got confirm_teleportation packet from invalid player");
      return false;
    }
    let parsed_packet = lib::packets::serverbound::play::ConfirmTeleportation::try_from(data.clone()).unwrap();
    if player.unwrap().current_teleport_id.unwrap_or_default() == parsed_packet.teleport_id {
      game.players = game.players.clone().into_iter().map(|mut x| {
        if player.is_some() && player.unwrap().peer_socket_address == stream.peer_addr().unwrap() {
          x.waiting_for_confirm_teleportation = false;
          x.current_teleport_id = None;
          return x;
        } else {
          return x;
        }
      }).collect();
    }

    return false;
  }
}



pub fn update_players(connection_streams: &mut HashMap<SocketAddr, TcpStream>, connections: &mut HashMap<SocketAddr, Connection>, all_players: Vec<Player>, removed_player: Option<&Player>) {
  connection_streams.iter()
    .filter(|x| connections.get(x.0).is_some_and(|x| x.state == ConnectionState::Play))
    .for_each(|x| {
      let mut uuids_to_remove: Vec<u128> = all_players.iter().map(|x| x.uuid).collect();
      if removed_player.is_some() {
        uuids_to_remove.push(removed_player.clone().unwrap().uuid);
      }
      lib::utils::send_packet(x.1, 0x3f, lib::packets::clientbound::play::PlayerInfoRemove {
        uuids: uuids_to_remove,
      }.try_into().unwrap());

      lib::utils::send_packet(x.1, 0x40, lib::packets::clientbound::play::PlayerInfoUpdate {
        actions: 255,
        players: all_players.iter().map(|y| (y.uuid, vec![
          lib::packets::clientbound::play::PlayerAction::AddPlayer(y.display_name.clone(), vec![]),
          lib::packets::clientbound::play::PlayerAction::InitializeChat(None),
          lib::packets::clientbound::play::PlayerAction::UpdateGameMode(1),
          lib::packets::clientbound::play::PlayerAction::UpdateListed(true),
          lib::packets::clientbound::play::PlayerAction::UpdateLatency(0),
          lib::packets::clientbound::play::PlayerAction::UpdateDisplayName(None),
          lib::packets::clientbound::play::PlayerAction::UpdateListPriority(0),
          lib::packets::clientbound::play::PlayerAction::UpdateHat(true),
        ])).collect(),
      }.try_into().unwrap());

      if removed_player.is_some() {
        lib::utils::send_packet(x.1, 0x47, lib::packets::clientbound::play::RemoveEntities {
          entity_ids: vec![removed_player.clone().unwrap().entity_id]
        }.try_into().unwrap());
      }
    });
}