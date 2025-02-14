use std::net::{TcpStream, SocketAddr};
use std::io::Write;
use std::collections::HashMap;

use super::deserialize;
use super::senders;
use super::super::ConnectionState;

pub fn handle_packet(mut packet: Vec<u8>, stream: &mut TcpStream, connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {
  println!("received new packet from {}", stream.peer_addr().unwrap());
	
	if !connection_states.contains_key(&stream.peer_addr().unwrap()) {
		connection_states.insert(stream.peer_addr().unwrap(), ConnectionState::Handshaking);
	}
	
	println!("client {} is in state {:?}", stream.peer_addr().unwrap(), connection_states.get(&stream.peer_addr().unwrap()).unwrap());
	
	super::print_binary(&packet);
  
  let length = deserialize::varint(&mut packet).unwrap(); //TODO: actually check if length is correct
  println!("length {}", length);

  let packet_id = deserialize::varint(&mut packet).unwrap();
  println!("packet id {:2x?}", packet_id);
  
	return match connection_states.get(&stream.peer_addr().unwrap()).unwrap() {
    ConnectionState::Handshaking => match packet_id {
			0x00 => handshaking::handshake(&mut packet, stream, connection_states),
			x => {
				println!("got unrecognized packet with id {:2x?}", x);
				return true; 
			},
		},
    ConnectionState::Status => match packet_id {
			0x00 => status::status_request(&mut packet, stream, connection_states),
			0x01 => status::ping_request(&mut packet, stream, connection_states),
			x => {
				println!("got unrecognized packet with id {:2x?}", x);
				return true; 
			},
		},
    ConnectionState::Login => match packet_id {
			0x00 => login::login_start(&mut packet, stream, connection_states),
			x => {
				println!("got unrecognized packet with id {:2x?}", x);
				return true; 
			},
		},
    ConnectionState::Play => match packet_id {
      0x08 => play::client_information(&mut packet, stream, connection_states),
      0x0d => play::plugin_message(&mut packet, stream, connection_states),
      0x14 => play::set_player_position(&mut packet, stream, connection_states),
      0x15 => play::set_player_position_and_rotation(&mut packet, stream, connection_states),
      0x00 => play::confirm_teleportation(&mut packet, stream, connection_states),
			x => {
				println!("got unrecognized packet with id {:2x?}", x);
				return false; 
			},
		},
	}
}

pub mod handshaking {
  use super::*;

  pub fn handshake(data: &mut Vec<u8>, stream: &mut TcpStream, connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {
    let protocol_version = deserialize::varint(data).unwrap();
    println!("protocol_version {}", protocol_version);
  
    let server_address = deserialize::string(data).unwrap();
    println!("server_address  {}", server_address);
  
    let server_port = deserialize::unsigned_short(data).unwrap();
    println!("server_port  {}", server_port);
  
    let next_state = deserialize::varint(data).unwrap();
    println!("next_state {}", next_state);

    if next_state == 1 {
      connection_states.insert(stream.peer_addr().unwrap(), ConnectionState::Status);
    } else {
      connection_states.insert(stream.peer_addr().unwrap(), ConnectionState::Login);
    }
  
    return false;
  }
}

pub mod status {
  use super::*;

  pub fn status_request(_data: &mut Vec<u8>, stream: &mut TcpStream, _connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {
    senders::status::status_response(stream);
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

  pub fn login_start(data: &mut Vec<u8>, stream: &mut TcpStream, connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {
    let name = deserialize::string(data).unwrap();
    println!("name {}", name);

/*     let has_sig_data = deserialize::boolean(data).unwrap();
    println!("has sig data {}", has_sig_data);
    if has_sig_data {
      let timestamp = deserialize::long(data).unwrap();
      println!("timestamp {}", timestamp);
      
      let pubkey_length = deserialize::varint(data).unwrap();
      println!("pubkey length {}", pubkey_length);
      let pubkey_length = 781; //Manual override because idfk what the length is supposed to mean

      let pubkey: Vec<u8> = data.drain(0..pubkey_length as usize).collect();
      println!("pubkey {:?}", pubkey);

      let signature_length = deserialize::varint(data).unwrap();
      println!("signature length {}", signature_length);
      let signature_length = 26; //Manual override because idfk what the length is supposed to mean

      let signature: Vec<u8> = data.drain(0..signature_length as usize).collect();
      println!("signature {:?}", signature);
    }

    let has_player_uuid = deserialize::boolean(data).unwrap();
    println!("has player uuid {}", has_player_uuid);
    if has_player_uuid {
      let player_uuid = deserialize::uuid(data).unwrap();
      println!("player uuid {}", player_uuid);
    } */

    connection_states.insert(stream.peer_addr().unwrap(), ConnectionState::Play);

    senders::login::login_success(stream);
    senders::play::login(stream);
    senders::play::plugin_message(stream);
    senders::play::change_difficulty(stream, senders::play::Difficulty::Peaceful);
    senders::play::player_abilities(stream);
    senders::play::set_held_item(stream, 0);
    senders::play::update_recipes(stream);
    senders::play::update_tags(stream);
    senders::play::entity_event(stream, 58, 24);
    senders::play::update_recipe_book(stream);
    senders::play::sync_player_position(stream, 0.0, 0.0, 0.0, 0.0, 0.0, 1, false);
    senders::play::player_info(stream, senders::play::PlayerInfoAction::AddPlayer);
    senders::play::set_render_distance(stream, 16);
    senders::play::set_simulation_distance(stream, 16);
    senders::play::set_center_chunk(stream, 0, 0);
    senders::play::initialize_world_border(stream);
    senders::play::update_time(stream);
    senders::play::set_default_spawn_position(stream);
    senders::play::server_data(stream);
    senders::play::set_container_content(stream);
    senders::play::spawn_entity(stream);
    senders::play::chunk_data_and_update_light(stream, 0, 0);
    senders::play::update_advancements(stream);
    senders::play::set_health(stream);
    senders::play::set_experience(stream);
    senders::play::chunk_data_and_update_light(stream, -1, 0);
    senders::play::chunk_data_and_update_light(stream, 0, -1);
    senders::play::chunk_data_and_update_light(stream, 0, 1);
    senders::play::chunk_data_and_update_light(stream, 1, 0);
    senders::play::chunk_data_and_update_light(stream, -2, 0);
    senders::play::chunk_data_and_update_light(stream, -1, -1);
    senders::play::chunk_data_and_update_light(stream, -1, 1);
    senders::play::chunk_data_and_update_light(stream, 0, -2);
    senders::play::chunk_data_and_update_light(stream, 0, 2);
    senders::play::chunk_data_and_update_light(stream, 1, -1);
    senders::play::chunk_data_and_update_light(stream, 1, 1);
    senders::play::chunk_data_and_update_light(stream, 2, 0);
    senders::play::chunk_data_and_update_light(stream, -3, 0);
    senders::play::chunk_data_and_update_light(stream, -2, -1);
    senders::play::chunk_data_and_update_light(stream, -2, 1);
    senders::play::chunk_data_and_update_light(stream, -1, -2);
    senders::play::chunk_data_and_update_light(stream, -1, 2);
    senders::play::chunk_data_and_update_light(stream, 0, -2);
    senders::play::chunk_data_and_update_light(stream, 0, 3);
    senders::play::chunk_data_and_update_light(stream, 1, -1);
    senders::play::chunk_data_and_update_light(stream, 1, 2);
    senders::play::chunk_data_and_update_light(stream, 2, -1);
    senders::play::chunk_data_and_update_light(stream, 2, 1);
    senders::play::chunk_data_and_update_light(stream, 3, 0);
    
    senders::play::commands(stream);
    senders::play::player_info(stream, senders::play::PlayerInfoAction::UpdateLatency);
    senders::play::sync_player_position(stream, 0.0, 0.0, 0.0, 0.0, 0.0, 2, false);

    return false;
  }

}

pub mod play {
  use super::*;

  pub fn client_information(_data: &mut Vec<u8>, _stream: &mut TcpStream, _connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {

    return false;
  }
  
  pub fn plugin_message(_data: &mut Vec<u8>, _stream: &mut TcpStream, _connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {

    return false;
  }
  
  pub fn set_player_position(data: &mut Vec<u8>, _stream: &mut TcpStream, _connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {
    let x: f64 = deserialize::double(data).unwrap();
    let y: f64 = deserialize::double(data).unwrap();
    let z: f64 = deserialize::double(data).unwrap();
    let on_ground: bool = deserialize::boolean(data).unwrap();

    println!("player sent position packet: x {} y {} z {} on ground? {}", x, y, z, on_ground);
    
    return false;
  }
  
  pub fn set_player_position_and_rotation(data: &mut Vec<u8>, _stream: &mut TcpStream, _connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {
    let x: f64 = deserialize::double(data).unwrap();
    let y: f64 = deserialize::double(data).unwrap();
    let z: f64 = deserialize::double(data).unwrap();
    let yaw: f32 = deserialize::float(data).unwrap();
    let pitch: f32 = deserialize::float(data).unwrap();
    let on_ground: bool = deserialize::boolean(data).unwrap();

    println!("player sent position packet: x {} y {} z {} yaw {} pitch {} on ground? {}", x, y, z, yaw, pitch, on_ground);
    
    return false;
  }

  pub fn confirm_teleportation(data: &mut Vec<u8>, _stream: &mut TcpStream, _connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {
    let teleport_id = deserialize::varint(data).unwrap();
    println!("client confirmed teleport id {}", teleport_id);
    
    return false;
  }
}