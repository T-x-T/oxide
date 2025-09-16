use std::net::{TcpStream, SocketAddr};
use std::io::Write;
use std::collections::HashMap;
use std::error::Error;
use lib::packets::Packet;
use lib::ConnectionState;
use lib::types::*;

pub enum Action {
	DisconnectPlayer,
}

pub fn handle_packet(mut packet: lib::Packet, stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>, game: &mut Game) -> Result<Option<Action>, Box<dyn Error>> {
	connections.entry(stream.peer_addr()?).or_insert(Connection { state: ConnectionState::Handshaking, player_name: None, player_uuid: None });
 	connection_streams.entry(stream.peer_addr()?).or_insert_with(|| stream.try_clone().unwrap());

	return match connections.get(&stream.peer_addr()?).unwrap().state {
    ConnectionState::Handshaking => match packet.id {
			lib::packets::serverbound::handshaking::Handshake::PACKET_ID => handshaking::handshake(&mut packet.data, stream, connections),
			_ => {Ok(None)},
		},
    ConnectionState::Status => match packet.id {
			0x00 => status::status_request(stream, game),
			0x01 => status::ping_request(&mut packet.data, stream),
			_ => {Ok(None)},
		},
    ConnectionState::Login => match packet.id {
			lib::packets::serverbound::login::LoginStart::PACKET_ID => login::login_start(&mut packet.data, stream, connections),
			lib::packets::serverbound::login::LoginAcknowledged::PACKET_ID => login::login_acknowledged(stream, connections),
			_ => {Ok(None)},
		},
    ConnectionState::Configuration => match packet.id {
      lib::packets::serverbound::configuration::ServerboundKnownPackets::PACKET_ID => configuration::serverbound_known_packets(stream),
      lib::packets::serverbound::configuration::AcknowledgeFinishConfiguration::PACKET_ID => configuration::acknowledge_finish_configuration(stream, connections, game, connection_streams),
      _ => {Ok(None)},
    },
    ConnectionState::Play => match packet.id {
      lib::packets::serverbound::play::ConfirmTeleportation::PACKET_ID => play::confirm_teleportation(&mut packet.data, game, stream, connections),
      lib::packets::serverbound::play::ChatCommand::PACKET_ID => play::chat_command(&mut packet.data, stream, game, connection_streams, connections),
      lib::packets::serverbound::play::ChatMessage::PACKET_ID => play::chat_message(&mut packet.data, connection_streams, game, stream, connections),
      lib::packets::serverbound::play::PlayerAction::PACKET_ID => play::player_action(&mut packet.data, stream, connection_streams, game, connections),
      lib::packets::serverbound::play::SetCreativeModeSlot::PACKET_ID => play::set_creative_mode_slot(&mut packet.data, stream, game, connections, connection_streams),
      lib::packets::serverbound::play::SetHandItem::PACKET_ID => play::set_held_item(&mut packet.data, stream, game, connections, connection_streams),
      lib::packets::serverbound::play::UseItemOn::PACKET_ID => play::use_item_on(&mut packet.data, stream, connection_streams, game, connections),
      lib::packets::serverbound::play::SetPlayerPosition::PACKET_ID => play::set_player_position(&mut packet.data, game, stream, connections, connection_streams),
      lib::packets::serverbound::play::SetPlayerPositionAndRotation::PACKET_ID => play::set_player_position_and_rotation(&mut packet.data, game, stream, connections, connection_streams),
      lib::packets::serverbound::play::SetPlayerRotation::PACKET_ID => play::set_player_rotation(&mut packet.data, game, stream, connections, connection_streams),
      lib::packets::serverbound::play::PickItemFromBlock::PACKET_ID => play::pick_item_from_block(&mut packet.data, stream, game, connections, connection_streams),
      lib::packets::serverbound::play::SwingArm::PACKET_ID => play::swing_arm(&mut packet.data, stream, game, connections, connection_streams),
      lib::packets::serverbound::play::ClickContainer::PACKET_ID => play::click_container(&mut packet.data, stream, game, connections, connection_streams),
      lib::packets::serverbound::play::CloseContainer::PACKET_ID => play::close_container(stream, &mut packet.data, game),
      _ => {Ok(None)},
		},
    ConnectionState::Transfer => todo!(),
  }
}

pub mod handshaking {
  use super::*;

  pub fn handshake(data: &mut [u8], stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>) -> Result<Option<Action>, Box<dyn Error>> {
    let parsed_packet = lib::packets::serverbound::handshaking::Handshake::try_from(data.to_vec())?;

    connections.entry(stream.peer_addr()?).and_modify(|x| x.state = parsed_packet.next_state.into());

    return Ok(None);
  }
}

pub mod status {
  use super::*;

  pub fn status_request(stream: &mut TcpStream, game: &mut Game) -> Result<Option<Action>, Box<dyn Error>> {
    lib::utils::send_packet(stream, lib::packets::clientbound::status::StatusResponse::PACKET_ID, lib::packets::clientbound::status::StatusResponse {
      status: format!("{{\"version\": {{\"name\": \"Oxide {}\",\"protocol\": {}}},\"players\": {{\"max\": -1,\"online\": {},\"sample\": []}},\"description\": {{\"text\": \"Hello oxide!\"}},\"enforcesSecureChat\": false}}", lib::packets::get_version_string(), lib::packets::get_protocol_version(), game.players.len()).to_string(),
    }.try_into()?)?;

    return Ok(None);
  }

  //implement actual packet struct https://git.thetxt.io/thetxt/oxide/issues/20
  pub fn ping_request(data: &mut Vec<u8>, stream: &mut TcpStream) -> Result<Option<Action>, Box<dyn Error>> {
    let mut output: Vec<u8> = Vec::new();
    output.push(9);
    output.push(1);
    output.append(data);
    stream.write_all(output.as_slice())?;

    return Ok(Some(Action::DisconnectPlayer));
  }
}

pub mod login {
  use super::*;

  pub fn login_start(data: &mut [u8], stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>) -> Result<Option<Action>, Box<dyn Error>> {
    let parsed_packet = lib::packets::serverbound::login::LoginStart::try_from(data.to_vec())?;

    connections.entry(stream.peer_addr()?).and_modify(|x| {
      x.player_name = Some(parsed_packet.name.clone());
      x.player_uuid = Some(parsed_packet.uuid);
    });

    lib::utils::send_packet(stream, lib::packets::clientbound::login::LoginSuccess::PACKET_ID, lib::packets::clientbound::login::LoginSuccess {
      uuid: parsed_packet.uuid,
      username: parsed_packet.name,
    }.try_into()?)?;

    return Ok(None);
  }

  pub fn login_acknowledged(stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>) -> Result<Option<Action>, Box<dyn Error>> {
    connections.entry(stream.peer_addr()?).and_modify(|x| x.state = ConnectionState::Configuration);

    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::ClientboundKnownPacks::PACKET_ID, lib::packets::clientbound::configuration::ClientboundKnownPacks {
      known_packs: vec![lib::Datapack { namespace: "minecraft".to_string(), id: "core".to_string(), version: lib::packets::get_version_string() }],
    }.try_into()?)?;

    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::ServerLinks::PACKET_ID, lib::packets::clientbound::configuration::ServerLinks {
   		links: vec![
     		(NbtTag::Root(vec![NbtTag::String("text".to_string(), "Git repository".to_string())]), "https://git.thetxt.io/thetxt/oxide".to_string()),
     		(NbtTag::Root(vec![NbtTag::String("text".to_string(), "Github mirror".to_string())]), "https://github.com/T-x-T/oxide".to_string()),
     		(NbtTag::Root(vec![NbtTag::String("text".to_string(), "Report bug".to_string())]), "https://git.thetxt.io/thetxt/oxide/issues/new".to_string()),
       	(NbtTag::Root(vec![NbtTag::String("text".to_string(), "Suggest feature".to_string())]), "https://git.thetxt.io/thetxt/oxide/issues/new".to_string()),
       	(NbtTag::Root(vec![NbtTag::String("text".to_string(), "Support development of Oxide ♥".to_string())]), "https://coff.ee/thetxt".to_string()),
     	]
    }.try_into()?)?;

    return Ok(None);
  }
}

pub mod configuration {

use super::*;
  use lib::{packets::{clientbound::configuration::{RegistryDataEntry, Tag}, Packet}};

  pub fn serverbound_known_packets(stream: &mut TcpStream) -> Result<Option<Action>, Box<dyn Error>> {
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:worldgen/biome".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:badlands".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:bamboo_jungle".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:basalt_deltas".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:beach".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:birch_forest".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:cherry_grove".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:cold_ocean".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:crimson_forest".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:dark_forest".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:deep_cold_ocean".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:deep_dark".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:deep_frozen_ocean".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:deep_lukewarm_ocean".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:deep_ocean".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:desert".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:dripstone_caves".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:end_barrens".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:end_highlands".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:end_midlands".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:eroded_badlands".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:flower_forest".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:forest".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:frozen_ocean".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:frozen_peaks".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:frozen_river".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:grove".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:ice_spikes".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:jagged_peaks".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:jungle".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:lukewarm_ocean".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:lush_caves".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:mangrove_swamp".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:meadow".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:mushroom_fields".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:nether_wastes".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:ocean".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:old_growth_birch_forest".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:old_growth_pine_taiga".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:old_growth_spruce_taiga".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:pale_garden".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:plains".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:river".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:savanna".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:savanna_plateau".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:small_end_islands".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:snowy_beach".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:snowy_plains".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:snowy_slopes".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:snowy_taiga".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:soul_sand_valley".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:sparse_jungle".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:stony_peaks".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:stony_shore".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:sunflower_plains".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:swamp".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:taiga".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:the_end".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:the_void".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:warm_ocean".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:warped_forest".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:windswept_forest".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:windswept_gravelly_hills".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:windswept_hills".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:windswept_savanna".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:wooded_badlands".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:chat_type".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:chat".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:emote_command".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:msg_command_incoming".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:msg_command_outgoing".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:say_command".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:team_msg_command_incoming".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:team_msg_command_outgoing".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:trim_pattern".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:bolt".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:coast".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:dune".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:eye".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:flow".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:host".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:raiser".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:rib".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:sentry".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:shaper".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:silence".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:snout".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:spire".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:tide".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:vex".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:ward".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:wayfinder".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:wild".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:trim_material".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:amethyst".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:copper".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:diamond".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:emerald".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:gold".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:iron".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:lapis".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:netherite".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:quartz".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:redstone".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:resin".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:wolf_variant".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:ashen".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:black".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:chestnut".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:pale".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:rusty".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:snowy".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:spotted".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:striped".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:woods".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:wolf_sound_variant".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:angry".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:big".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:classic".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:cute".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:grumpy".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:puglin".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:sad".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:pig_variant".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:cold".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:temperate".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:warm".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:frog_variant".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:cold".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:temperate".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:warm".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:cat_variant".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:all_black".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:black".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:british_shorthair".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:calico".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:jellie".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:persian".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:ragdoll".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:red".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:siamese".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:tabby".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:white".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:cow_variant".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:cold".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:temperate".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:warm".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:chicken_variant".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:cold".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:temperate".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:warm".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:painting_variant".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:alban".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:aztec".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:aztec2".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:backyard".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:baroque".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:bomb".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:bouquet".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:burning_skull".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:bust".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:cavebird".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:changing".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:cotan".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:courbet".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:creebet".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:donkey_kong".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:earth".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:endboss".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:fern".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:fighters".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:finding".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:fire".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:graham".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:humble".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:kebab".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:lowmist".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:match".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:meditative".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:orb".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:owlemons".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:passage".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:pigscene".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:plant".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:pointer".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:pond".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:pool".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:prairie_ride".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:sea".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:skeleton".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:skull_and_roses".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:stage".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:sunflowers".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:sunset".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:tides".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:unpacked".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:void".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:wanderer".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:wasteland".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:water".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:wind".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:wither".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:dimension_type".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:overworld".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:overworld_caves".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:the_end".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:the_nether".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:damage_type".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:arrow".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:bad_respawn_point".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:cactus".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:campfire".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:cramming".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:dragon_breath".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:drown".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:dry_out".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:ender_pearl".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:explosion".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:fall".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:falling_anvil".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:falling_block".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:falling_stalactite".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:fireball".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:fireworks".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:fly_into_wall".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:freeze".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:generic".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:generic_kill".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:hot_floor".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:in_fire".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:in_wall".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:indirect_magic".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:lava".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:lightning_bolt".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:mace_smash".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:magic".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:mob_attack".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:mob_attack_no_aggro".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:mob_projectile".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:on_fire".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:out_of_world".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:outside_border".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:player_attack".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:player_explosion".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:sonic_boom".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:spit".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:stalagmite".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:starve".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:sting".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:sweet_berry_bush".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:thorns".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:thrown".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:trident".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:unattributed_fireball".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:wind_charge".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:wither".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:wither_skull".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:banner_pattern".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:base".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:border".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:bricks".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:circle".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:creeper".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:cross".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:curly_border".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:diagonal_left".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:diagonal_right".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:diagonal_up_left".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:diagonal_up_right".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:flow".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:flower".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:globe".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:gradient".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:gradient_up".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:guster".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:half_horizontal".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:half_horizontal_bottom".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:half_vertical".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:half_vertical_right".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:mojang".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:piglin".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:rhombus".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:skull".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:small_stripes".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:square_bottom_left".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:square_bottom_right".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:square_top_left".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:square_top_right".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:straight_cross".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:stripe_bottom".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:stripe_center".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:stripe_downleft".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:stripe_downright".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:stripe_left".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:stripe_middle".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:stripe_right".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:stripe_top".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:triangle_bottom".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:triangle_top".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:triangles_bottom".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:triangles_top".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:enchantment".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:aqua_affinity".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:bane_of_arthropods".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:binding_curse".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:blast_protection".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:breach".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:channeling".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:density".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:depth_strider".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:efficiency".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:feather_falling".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:fire_aspect".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:fire_protection".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:flame".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:fortune".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:frost_walker".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:impaling".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:infinity".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:knockback".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:looting".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:loyalty".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:luck_of_the_sea".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:lure".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:mending".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:multishot".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:piercing".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:power".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:projectile_protection".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:protection".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:punch".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:quick_charge".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:respiration".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:riptide".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:sharpness".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:silk_touch".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:smite".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:soul_speed".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:sweeping_edge".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:swift_sneak".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:thorns".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:unbreaking".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:vanishing_curse".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:wind_burst".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:jukebox_song".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:11".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:13".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:5".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:blocks".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:cat".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:chirp".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:creator".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:creator_music_box".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:far".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:mall".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:mellohi".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:otherside".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:pigstep".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:precipice".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:relic".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:stal".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:strad".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:wait".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:ward".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:instrument".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:admire_goat_horn".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:call_goat_horn".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:dream_goat_horn".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:feel_goat_horn".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:ponder_goat_horn".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:seek_goat_horn".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:sing_goat_horn".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:yearn_goat_horn".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:dialog".to_string(),
      entries: vec![
	      RegistryDataEntry { entry_id: "minecraft:custom_options".to_string(), has_data: false, data: None },
	      RegistryDataEntry { entry_id: "minecraft:quick_actions".to_string(), has_data: false, data: None },
	      RegistryDataEntry { entry_id: "minecraft:server_links".to_string(), has_data: false, data: None }
      ]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:test_environment".to_string(),
      entries: vec![RegistryDataEntry {entry_id: "minecraft:default".to_string(), has_data: false, data: None}]
    }.try_into()?)?;
    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:test_instance".to_string(),
      entries: vec![RegistryDataEntry {entry_id: "minecraft:always_pass".to_string(), has_data: false, data: None}]
    }.try_into()?)?;

    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::UpdateTags::PACKET_ID, lib::packets::clientbound::configuration::UpdateTags {
        data: vec![
          ("minecraft:dialog".to_string(), vec![Tag { name: "minecraft:pause_screen_additions".to_string(), entries: vec![] }, Tag { name: "minecraft:quick_actions".to_string(), entries: vec![] }]), ("minecraft:point_of_interest_type".to_string(), vec![Tag { name: "minecraft:bee_home".to_string(), entries: vec![15, 16] }, Tag { name: "minecraft:acquirable_job_site".to_string(), entries: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] }, Tag { name: "minecraft:village".to_string(), entries: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14] }]), ("minecraft:block".to_string(), vec![Tag { name: "minecraft:mob_interactable_doors".to_string(), entries: vec![207, 614, 615, 616, 617, 619, 620, 858, 859, 621, 622, 618, 1008, 1009, 1011, 1010, 1012, 1013, 1015, 1014] }, Tag { name: "minecraft:campfires".to_string(), entries: vec![818, 819] }, Tag { name: "minecraft:soul_fire_base_blocks".to_string(), entries: vec![273, 274] }, Tag { name: "minecraft:infiniburn_nether".to_string(), entries: vec![272, 639] }, Tag { name: "minecraft:wooden_slabs".to_string(), entries: vec![567, 568, 569, 570, 571, 573, 574, 844, 845, 575, 576, 572] }, Tag { name: "minecraft:snaps_goat_horn".to_string(), entries: vec![53, 51, 49, 52, 50, 55, 56, 57, 54, 1, 524, 44, 46, 970, 367] }, Tag { name: "minecraft:coal_ores".to_string(), entries: vec![46, 47] }, Tag { name: "minecraft:occludes_vibration_signals".to_string(), entries: vec![140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155] }, Tag { name: "minecraft:pale_oak_logs".to_string(), entries: vec![56, 20, 67, 86] }, Tag { name: "minecraft:small_flowers".to_string(), entries: vec![157, 1099, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 170, 169, 158, 1100] }, Tag { name: "minecraft:azalea_root_replaceable".to_string(), entries: vec![1, 2, 4, 6, 943, 1059, 9, 8, 11, 10, 342, 1057, 1052, 1096, 1058, 59, 522, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466, 467, 39, 268, 40, 37, 265, 959] }, Tag { name: "minecraft:wooden_trapdoors".to_string(), entries: vec![305, 303, 307, 308, 304, 301, 302, 850, 851, 309, 310, 306] }, Tag { name: "minecraft:invalid_spawn_inside".to_string(), entries: vec![360, 635] }, Tag { name: "minecraft:dry_vegetation_may_place_on".to_string(), entries: vec![37, 39, 38, 522, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466, 467, 9, 8, 11, 10, 342, 1057, 1052, 1096, 1058, 59, 195] }, Tag { name: "minecraft:foxes_spawnable_on".to_string(), entries: vec![8, 263, 265, 11, 10] }, Tag { name: "minecraft:wolves_spawnable_on".to_string(), entries: vec![8, 263, 265, 10, 11] }, Tag { name: "minecraft:wool".to_string(), entries: vec![140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155] }, Tag { name: "minecraft:stairs".to_string(), entries: vec![187, 373, 374, 375, 484, 486, 487, 854, 855, 488, 489, 485, 490, 210, 366, 352, 340, 339, 628, 449, 566, 499, 498, 500, 764, 765, 766, 767, 768, 769, 770, 771, 772, 773, 774, 775, 776, 777, 884, 892, 895, 1061, 1065, 1069, 1073, 984, 985, 986, 987, 1001, 1002, 1003, 1000, 341, 945, 949, 954, 346] }, Tag { name: "minecraft:edible_for_sheep".to_string(), entries: vec![130, 134, 135, 131] }, Tag { name: "minecraft:logs".to_string(), entries: vec![55, 77, 66, 85, 56, 20, 67, 86, 49, 71, 68, 79, 53, 75, 64, 83, 51, 73, 62, 81, 52, 74, 63, 82, 50, 72, 61, 80, 57, 78, 69, 87, 54, 76, 65, 84, 830, 831, 832, 833, 821, 822, 823, 824] }, Tag { name: "minecraft:all_signs".to_string(), entries: vec![197, 198, 199, 200, 202, 203, 204, 860, 861, 205, 206, 201, 211, 212, 213, 214, 216, 217, 218, 862, 863, 219, 220, 215, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 242, 243, 241, 244] }, Tag { name: "minecraft:trail_ruins_replaceable".to_string(), entries: vec![40] }, Tag { name: "minecraft:beehives".to_string(), entries: vec![870, 871] }, Tag { name: "minecraft:ice".to_string(), entries: vec![264, 524, 756, 638] }, Tag { name: "minecraft:azalea_grows_on".to_string(), entries: vec![9, 8, 11, 10, 342, 1057, 1052, 1096, 1058, 59, 37, 39, 38, 522, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466, 467, 265, 959] }, Tag { name: "minecraft:enchantment_power_provider".to_string(), entries: vec![177] }, Tag { name: "minecraft:wool_carpets".to_string(), entries: vec![506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517, 518, 519, 520, 521] }, Tag { name: "minecraft:dragon_immune".to_string(), entries: vec![492, 34, 360, 361, 635, 376, 636, 637, 864, 865, 156, 180, 876, 362, 326, 877, 1090, 866, 867] }, Tag { name: "minecraft:crops".to_string(), entries: vec![633, 409, 410, 194, 334, 333, 630, 631] }, Tag { name: "minecraft:mangrove_roots_can_grow_through".to_string(), entries: vec![1058, 59, 58, 1048, 335, 33, 263] }, Tag { name: "minecraft:features_cannot_replace".to_string(), entries: vec![34, 185, 188, 361, 1090, 1093, 1094] }, Tag { name: "minecraft:valid_spawn".to_string(), entries: vec![8, 11] }, Tag { name: "minecraft:mushroom_grow_block".to_string(), entries: vec![342, 11, 834, 825] }, Tag { name: "minecraft:wooden_doors".to_string(), entries: vec![207, 614, 615, 616, 617, 619, 620, 858, 859, 621, 622, 618] }, Tag { name: "minecraft:crystal_sound_blocks".to_string(), entries: vec![937, 938] }, Tag { name: "minecraft:sniffer_egg_hatch_boost".to_string(), entries: vec![1052] }, Tag { name: "minecraft:standing_signs".to_string(), entries: vec![197, 198, 199, 200, 202, 203, 204, 860, 861, 205, 206, 201] }, Tag { name: "minecraft:warped_stems".to_string(), entries: vec![821, 822, 823, 824] }, Tag { name: "minecraft:infiniburn_end".to_string(), entries: vec![272, 639, 34] }, Tag { name: "minecraft:emerald_ores".to_string(), entries: vec![367, 368] }, Tag { name: "minecraft:bamboo_blocks".to_string(), entries: vec![60, 70] }, Tag { name: "minecraft:crimson_stems".to_string(), entries: vec![830, 831, 832, 833] }, Tag { name: "minecraft:replaceable_by_trees".to_string(), entries: vec![91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93, 157, 1099, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 170, 169, 158, 1100, 1097, 130, 131, 132, 335, 336, 525, 526, 527, 528, 529, 530, 1056, 632, 35, 136, 137, 133, 1103, 828, 829, 841, 1051, 134, 135] }, Tag { name: "minecraft:needs_stone_tool".to_string(), entries: vec![174, 1081, 44, 45, 104, 102, 103, 966, 1082, 970, 971, 991, 987, 975, 968, 989, 985, 973, 969, 988, 984, 972, 967, 990, 986, 974, 992, 1007, 1003, 999, 993, 1005, 1001, 997, 994, 1006, 1002, 998, 995, 1004, 1000, 996, 1040, 1092, 979, 978, 977, 976, 983, 982, 981, 980, 1024, 1025, 1026, 1027, 1028, 1029, 1030, 1031, 1032, 1033, 1034, 1035, 1036, 1037, 1038, 1039, 1016, 1017, 1019, 1018, 1020, 1021, 1023, 1022] }, Tag { name: "minecraft:concrete_powder".to_string(), entries: vec![694, 695, 696, 697, 698, 699, 700, 701, 702, 703, 704, 705, 706, 707, 708, 709] }, Tag { name: "minecraft:lava_pool_stone_cannot_replace".to_string(), entries: vec![34, 185, 188, 361, 1090, 1093, 1094, 91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93, 55, 77, 66, 85, 56, 20, 67, 86, 49, 71, 68, 79, 53, 75, 64, 83, 51, 73, 62, 81, 52, 74, 63, 82, 50, 72, 61, 80, 57, 78, 69, 87, 54, 76, 65, 84, 830, 831, 832, 833, 821, 822, 823, 824] }, Tag { name: "minecraft:inside_step_sound_blocks".to_string(), entries: vec![959, 963, 336, 343, 942, 1049, 1050, 1051] }, Tag { name: "minecraft:prevent_mob_spawning_inside".to_string(), entries: vec![209, 126, 127, 450] }, Tag { name: "minecraft:terracotta".to_string(), entries: vec![522, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466, 467] }, Tag { name: "minecraft:climbable".to_string(), entries: vec![208, 335, 804, 837, 838, 839, 840, 1043, 1044] }, Tag { name: "minecraft:wart_blocks".to_string(), entries: vec![640, 827] }, Tag { name: "minecraft:parrots_spawnable_on".to_string(), entries: vec![8, 0, 91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93, 55, 77, 66, 85, 56, 20, 67, 86, 49, 71, 68, 79, 53, 75, 64, 83, 51, 73, 62, 81, 52, 74, 63, 82, 50, 72, 61, 80, 57, 78, 69, 87, 54, 76, 65, 84, 830, 831, 832, 833, 821, 822, 823, 824] }, Tag { name: "minecraft:dark_oak_logs".to_string(), entries: vec![55, 77, 66, 85] }, Tag { name: "minecraft:frog_prefer_jump_to".to_string(), entries: vec![343, 1053] }, Tag { name: "minecraft:coral_plants".to_string(), entries: vec![730, 731, 732, 733, 734] }, Tag { name: "minecraft:goats_spawnable_on".to_string(), entries: vec![8, 1, 263, 265, 524, 40] }, Tag { name: "minecraft:ceiling_hanging_signs".to_string(), entries: vec![221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232] }, Tag { name: "minecraft:sculk_replaceable_world_gen".to_string(), entries: vec![1, 2, 4, 6, 943, 1059, 9, 8, 11, 10, 342, 1057, 1052, 1096, 1058, 59, 522, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466, 467, 834, 825, 272, 275, 883, 37, 39, 40, 273, 274, 957, 1080, 268, 1042, 362, 563, 106, 1072, 1068, 1060, 1077, 1078, 1064] }, Tag { name: "minecraft:beacon_base_blocks".to_string(), entries: vec![874, 372, 192, 173, 174] }, Tag { name: "minecraft:frogs_spawnable_on".to_string(), entries: vec![8, 1058, 58, 59] }, Tag { name: "minecraft:shulker_boxes".to_string(), entries: vec![645, 661, 657, 658, 655, 653, 659, 649, 654, 651, 648, 647, 652, 656, 660, 646, 650] }, Tag { name: "minecraft:blocks_wind_charge_explosions".to_string(), entries: vec![492, 34] }, Tag { name: "minecraft:anvil".to_string(), entries: vec![435, 436, 437] }, Tag { name: "minecraft:birch_logs".to_string(), entries: vec![51, 73, 62, 81] }, Tag { name: "minecraft:moss_replaceable".to_string(), entries: vec![1, 2, 4, 6, 943, 1059, 1044, 1043, 9, 8, 11, 10, 342, 1057, 1052, 1096, 1058, 59] }, Tag { name: "minecraft:lapis_ores".to_string(), entries: vec![102, 103] }, Tag { name: "minecraft:wall_corals".to_string(), entries: vec![750, 751, 752, 753, 754] }, Tag { name: "minecraft:maintains_farmland".to_string(), entries: vec![333, 331, 334, 332, 633, 409, 410, 630, 158, 631, 194] }, Tag { name: "minecraft:convertable_to_mud".to_string(), entries: vec![9, 10, 1057] }, Tag { name: "minecraft:air".to_string(), entries: vec![0, 761, 762] }, Tag { name: "minecraft:sniffer_diggable_block".to_string(), entries: vec![9, 8, 11, 10, 1057, 1052, 1096, 1058, 59] }, Tag { name: "minecraft:lush_ground_replaceable".to_string(), entries: vec![1, 2, 4, 6, 943, 1059, 1044, 1043, 9, 8, 11, 10, 342, 1057, 1052, 1096, 1058, 59, 268, 40, 37] }, Tag { name: "minecraft:sword_instantly_mines".to_string(), entries: vec![759, 758] }, Tag { name: "minecraft:bee_attractive".to_string(), entries: vec![157, 1099, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 170, 169, 158, 525, 526, 528, 527, 632, 98, 1047, 33, 93, 1049, 1050, 625, 1045, 267] }, Tag { name: "minecraft:fences".to_string(), entries: vec![271, 608, 610, 611, 605, 606, 607, 848, 849, 612, 613, 609, 351] }, Tag { name: "minecraft:saplings".to_string(), entries: vec![25, 26, 27, 28, 29, 31, 32, 1046, 1047, 33, 30] }, Tag { name: "minecraft:camels_spawnable_on".to_string(), entries: vec![37, 39, 38] }, Tag { name: "minecraft:mineable/pickaxe".to_string(), entries: vec![1, 2, 3, 4, 5, 6, 7, 12, 42, 43, 44, 45, 46, 47, 48, 102, 103, 104, 105, 106, 107, 108, 173, 174, 175, 179, 180, 185, 190, 191, 192, 196, 210, 246, 247, 258, 259, 272, 275, 276, 311, 312, 313, 314, 326, 327, 339, 340, 350, 351, 352, 354, 355, 362, 366, 367, 368, 369, 372, 439, 440, 443, 444, 445, 446, 447, 448, 449, 451, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466, 467, 494, 495, 496, 497, 498, 499, 500, 501, 502, 503, 522, 523, 563, 564, 565, 566, 578, 579, 580, 581, 582, 583, 584, 585, 587, 588, 589, 590, 591, 592, 593, 594, 595, 626, 627, 628, 629, 639, 641, 642, 644, 662, 663, 664, 665, 666, 667, 668, 669, 670, 671, 672, 673, 674, 675, 676, 677, 678, 679, 680, 681, 682, 683, 684, 685, 686, 687, 688, 689, 690, 691, 692, 693, 715, 716, 717, 718, 719, 720, 721, 722, 723, 724, 725, 726, 727, 728, 729, 735, 736, 737, 738, 739, 745, 746, 747, 748, 749, 764, 765, 766, 767, 768, 769, 770, 771, 772, 773, 774, 775, 776, 777, 778, 779, 780, 781, 782, 783, 784, 785, 786, 787, 788, 789, 790, 807, 808, 811, 814, 815, 816, 817, 825, 834, 874, 875, 876, 877, 882, 883, 884, 886, 887, 888, 889, 890, 891, 892, 894, 895, 896, 897, 900, 901, 902, 943, 957, 969, 968, 967, 966, 970, 971, 972, 973, 974, 975, 984, 985, 986, 987, 988, 989, 990, 991, 992, 993, 994, 995, 996, 997, 998, 999, 1000, 1001, 1002, 1003, 1004, 1005, 1006, 1007, 1040, 1041, 1042, 1059, 1060, 1061, 1062, 1064, 1065, 1066, 1068, 1069, 1070, 1072, 1073, 1074, 1076, 1077, 1078, 1080, 1081, 1082, 1083, 264, 524, 756, 138, 128, 139, 939, 942, 941, 940, 937, 938, 318, 322, 321, 1079, 317, 320, 319, 262, 898, 378, 379, 791, 792, 793, 794, 795, 796, 798, 799, 800, 801, 802, 803, 885, 893, 899, 1063, 1067, 1071, 1075, 797, 946, 950, 955, 348, 645, 661, 657, 658, 655, 653, 659, 649, 654, 651, 648, 647, 652, 656, 660, 646, 650, 435, 436, 437, 356, 357, 358, 359, 209, 126, 127, 450, 757, 316, 341, 586, 315, 1092, 944, 945, 951, 947, 948, 949, 952, 953, 954, 956, 979, 978, 977, 976, 983, 982, 981, 980, 1024, 1025, 1026, 1027, 1028, 1029, 1030, 1031, 1032, 1033, 1034, 1035, 1036, 1037, 1038, 1039, 1008, 1009, 1011, 1010, 1012, 1013, 1015, 1014, 1016, 1017, 1019, 1018, 1020, 1021, 1023, 1022, 1095, 345, 347, 346, 349] }, Tag { name: "minecraft:beds".to_string(), entries: vec![124, 125, 121, 122, 119, 117, 123, 113, 118, 115, 112, 111, 116, 120, 110, 114] }, Tag { name: "minecraft:iron_ores".to_string(), entries: vec![44, 45] }, Tag { name: "minecraft:unstable_bottom_center".to_string(), entries: vec![599, 597, 601, 602, 598, 338, 596, 852, 853, 603, 604, 600] }, Tag { name: "minecraft:oak_logs".to_string(), entries: vec![49, 71, 68, 79] }, Tag { name: "minecraft:doors".to_string(), entries: vec![207, 614, 615, 616, 617, 619, 620, 858, 859, 621, 622, 618, 1008, 1009, 1011, 1010, 1012, 1013, 1015, 1014, 247] }, Tag { name: "minecraft:enderman_holdable".to_string(), entries: vec![157, 1099, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 170, 169, 158, 1100, 9, 8, 11, 10, 342, 1057, 1052, 1096, 1058, 59, 37, 39, 40, 171, 172, 176, 266, 268, 329, 281, 330, 835, 834, 841, 826, 825, 828, 267] }, Tag { name: "minecraft:banners".to_string(), entries: vec![531, 532, 533, 534, 535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556, 557, 558, 559, 560, 561, 562] }, Tag { name: "minecraft:flower_pots".to_string(), entries: vec![380, 1101, 1102, 393, 394, 395, 396, 397, 398, 399, 400, 401, 392, 382, 383, 384, 385, 386, 388, 389, 405, 406, 407, 391, 408, 402, 403, 404, 760, 878, 879, 880, 881, 1084, 1085, 390, 387, 381] }, Tag { name: "minecraft:infiniburn_overworld".to_string(), entries: vec![272, 639] }, Tag { name: "minecraft:smelts_to_glass".to_string(), entries: vec![37, 39] }, Tag { name: "minecraft:wooden_fences".to_string(), entries: vec![271, 608, 610, 611, 605, 606, 607, 848, 849, 612, 613, 609] }, Tag { name: "minecraft:piglin_repellents".to_string(), entries: vec![184, 277, 817, 278, 819] }, Tag { name: "minecraft:incorrect_for_wooden_tool".to_string(), entries: vec![180, 876, 874, 877, 875, 192, 190, 191, 367, 368, 372, 173, 1083, 42, 43, 258, 259, 174, 1081, 44, 45, 104, 102, 103, 966, 1082, 970, 971, 991, 987, 975, 968, 989, 985, 973, 969, 988, 984, 972, 967, 990, 986, 974, 992, 1007, 1003, 999, 993, 1005, 1001, 997, 994, 1006, 1002, 998, 995, 1004, 1000, 996, 1040, 1092, 979, 978, 977, 976, 983, 982, 981, 980, 1024, 1025, 1026, 1027, 1028, 1029, 1030, 1031, 1032, 1033, 1034, 1035, 1036, 1037, 1038, 1039, 1016, 1017, 1019, 1018, 1020, 1021, 1023, 1022] }, Tag { name: "minecraft:incorrect_for_iron_tool".to_string(), entries: vec![180, 876, 874, 877, 875] }, Tag { name: "minecraft:mooshrooms_spawnable_on".to_string(), entries: vec![342] }, Tag { name: "minecraft:enchantment_power_transmitter".to_string(), entries: vec![0, 35, 36, 130, 131, 132, 133, 134, 135, 136, 137, 183, 184, 263, 335, 336, 337, 493, 529, 530, 643, 761, 762, 763, 828, 829, 841, 1051, 1056] }, Tag { name: "minecraft:wall_post_override".to_string(), entries: vec![181, 277, 260, 371, 197, 198, 199, 200, 202, 203, 204, 860, 861, 205, 206, 201, 211, 212, 213, 214, 216, 217, 218, 862, 863, 219, 220, 215, 531, 532, 533, 534, 535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556, 557, 558, 559, 560, 561, 562, 439, 440, 248, 249, 250, 251, 252, 254, 255, 846, 847, 256, 257, 253, 246, 897, 267] }, Tag { name: "minecraft:portals".to_string(), entries: vec![280, 360, 635] }, Tag { name: "minecraft:bamboo_plantable_on".to_string(), entries: vec![37, 39, 38, 9, 8, 11, 10, 342, 1057, 1052, 1096, 1058, 59, 759, 758, 40, 41] }, Tag { name: "minecraft:polar_bears_spawnable_on_alternate".to_string(), entries: vec![264] }, Tag { name: "minecraft:cauldrons".to_string(), entries: vec![356, 357, 358, 359] }, Tag { name: "minecraft:big_dripleaf_placeable".to_string(), entries: vec![268, 1052, 9, 8, 11, 10, 342, 1057, 1058, 59, 195] }, Tag { name: "minecraft:sword_efficient".to_string(), entries: vec![91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93, 335, 336, 329, 281, 282, 330, 365, 1053, 1054, 624, 625] }, Tag { name: "minecraft:pressure_plates".to_string(), entries: vec![439, 440, 248, 249, 250, 251, 252, 254, 255, 846, 847, 256, 257, 253, 246, 897] }, Tag { name: "minecraft:dampens_vibrations".to_string(), entries: vec![140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517, 518, 519, 520, 521] }, Tag { name: "minecraft:mangrove_logs".to_string(), entries: vec![57, 78, 69, 87] }, Tag { name: "minecraft:overworld_carver_replaceables".to_string(), entries: vec![1, 2, 4, 6, 943, 1059, 9, 8, 11, 10, 342, 1057, 1052, 1096, 1058, 59, 37, 39, 38, 522, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466, 467, 44, 45, 970, 971, 263, 265, 959, 35, 40, 41, 106, 563, 957, 524, 1081, 1082] }, Tag { name: "minecraft:snow_layer_can_survive_on".to_string(), entries: vec![872, 273, 1058] }, Tag { name: "minecraft:jungle_logs".to_string(), entries: vec![52, 74, 63, 82] }, Tag { name: "minecraft:vibration_resonators".to_string(), entries: vec![937] }, Tag { name: "minecraft:wall_hanging_signs".to_string(), entries: vec![233, 234, 235, 236, 237, 238, 239, 240, 242, 243, 241, 244] }, Tag { name: "minecraft:sculk_replaceable".to_string(), entries: vec![1, 2, 4, 6, 943, 1059, 9, 8, 11, 10, 342, 1057, 1052, 1096, 1058, 59, 522, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466, 467, 834, 825, 272, 275, 883, 37, 39, 40, 273, 274, 957, 1080, 268, 1042, 362, 563, 106] }, Tag { name: "minecraft:snow_layer_cannot_survive_on".to_string(), entries: vec![264, 524, 492] }, Tag { name: "minecraft:spruce_logs".to_string(), entries: vec![50, 72, 61, 80] }, Tag { name: "minecraft:wooden_stairs".to_string(), entries: vec![187, 373, 374, 375, 484, 486, 487, 854, 855, 488, 489, 485] }, Tag { name: "minecraft:ancient_city_replaceable".to_string(), entries: vec![1059, 1072, 1068, 1074, 1070, 1073, 1071, 1075, 1060, 1077, 1078, 147] }, Tag { name: "minecraft:signs".to_string(), entries: vec![197, 198, 199, 200, 202, 203, 204, 860, 861, 205, 206, 201, 211, 212, 213, 214, 216, 217, 218, 862, 863, 219, 220, 215] }, Tag { name: "minecraft:mangrove_logs_can_grow_through".to_string(), entries: vec![1058, 59, 58, 96, 57, 33, 1048, 335] }, Tag { name: "minecraft:base_stone_overworld".to_string(), entries: vec![1, 2, 4, 6, 943, 1059] }, Tag { name: "minecraft:wooden_buttons".to_string(), entries: vec![411, 412, 413, 414, 415, 417, 418, 856, 857, 419, 420, 416] }, Tag { name: "minecraft:axolotls_spawnable_on".to_string(), entries: vec![268] }, Tag { name: "minecraft:wither_summon_base_blocks".to_string(), entries: vec![273, 274] }, Tag { name: "minecraft:dripstone_replaceable_blocks".to_string(), entries: vec![1, 2, 4, 6, 943, 1059] }, Tag { name: "minecraft:hoglin_repellents".to_string(), entries: vec![826, 879, 280, 877] }, Tag { name: "minecraft:stone_bricks".to_string(), entries: vec![311, 312, 313, 314] }, Tag { name: "minecraft:fire".to_string(), entries: vec![183, 184] }, Tag { name: "minecraft:mineable/axe".to_string(), entries: vec![109, 759, 806, 870, 871, 1054, 1053, 177, 323, 818, 809, 281, 188, 625, 624, 365, 868, 193, 442, 810, 336, 282, 270, 208, 812, 805, 330, 325, 329, 324, 813, 819, 438, 335, 531, 532, 533, 534, 535, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556, 557, 558, 559, 560, 561, 562, 599, 597, 601, 602, 598, 338, 596, 852, 853, 603, 604, 600, 55, 77, 66, 85, 56, 20, 67, 86, 49, 71, 68, 79, 53, 75, 64, 83, 51, 73, 62, 81, 52, 74, 63, 82, 50, 72, 61, 80, 57, 78, 69, 87, 54, 76, 65, 84, 830, 831, 832, 833, 821, 822, 823, 824, 13, 14, 15, 16, 17, 19, 21, 842, 843, 22, 23, 18, 197, 198, 199, 200, 202, 203, 204, 860, 861, 205, 206, 201, 211, 212, 213, 214, 216, 217, 218, 862, 863, 219, 220, 215, 411, 412, 413, 414, 415, 417, 418, 856, 857, 419, 420, 416, 207, 614, 615, 616, 617, 619, 620, 858, 859, 621, 622, 618, 271, 608, 610, 611, 605, 606, 607, 848, 849, 612, 613, 609, 248, 249, 250, 251, 252, 254, 255, 846, 847, 256, 257, 253, 567, 568, 569, 570, 571, 573, 574, 844, 845, 575, 576, 572, 187, 373, 374, 375, 484, 486, 487, 854, 855, 488, 489, 485, 305, 303, 307, 308, 304, 301, 302, 850, 851, 309, 310, 306, 58, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 242, 243, 241, 244, 24, 577, 490, 60, 70, 178, 186] }, Tag { name: "minecraft:base_stone_nether".to_string(), entries: vec![272, 275, 883] }, Tag { name: "minecraft:needs_diamond_tool".to_string(), entries: vec![180, 876, 874, 877, 875] }, Tag { name: "minecraft:wall_signs".to_string(), entries: vec![211, 212, 213, 214, 216, 217, 218, 862, 863, 219, 220, 215] }, Tag { name: "minecraft:incorrect_for_stone_tool".to_string(), entries: vec![180, 876, 874, 877, 875, 192, 190, 191, 367, 368, 372, 173, 1083, 42, 43, 258, 259] }, Tag { name: "minecraft:slabs".to_string(), entries: vec![567, 568, 569, 570, 571, 573, 574, 844, 845, 575, 576, 572, 577, 578, 579, 585, 580, 591, 588, 589, 584, 583, 587, 582, 501, 502, 503, 778, 779, 780, 781, 782, 783, 784, 785, 786, 787, 788, 789, 790, 581, 590, 886, 891, 896, 1062, 1066, 1070, 1074, 1005, 1006, 1007, 988, 989, 990, 991, 1004, 586, 944, 948, 953, 347] }, Tag { name: "minecraft:animals_spawnable_on".to_string(), entries: vec![8] }, Tag { name: "minecraft:guarded_by_piglins".to_string(), entries: vec![173, 806, 188, 369, 894, 438, 1083, 645, 661, 657, 658, 655, 653, 659, 649, 654, 651, 648, 647, 652, 656, 660, 646, 650, 42, 48, 43] }, Tag { name: "minecraft:replaceable_by_mushrooms".to_string(), entries: vec![91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93, 157, 1099, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 170, 169, 158, 1100, 1097, 130, 131, 132, 335, 336, 525, 526, 527, 528, 529, 530, 1056, 632, 35, 136, 137, 171, 172, 323, 324, 828, 829, 841, 1051, 134, 135, 133, 1103] }, Tag { name: "minecraft:mineable/shovel".to_string(), entries: vec![268, 9, 10, 11, 195, 8, 40, 342, 37, 39, 265, 263, 273, 634, 274, 1057, 59, 1058, 38, 41, 694, 695, 696, 697, 698, 699, 700, 701, 702, 703, 704, 705, 706, 707, 708, 709] }, Tag { name: "minecraft:nether_carver_replaceables".to_string(), entries: vec![1, 2, 4, 6, 943, 1059, 272, 275, 883, 9, 8, 11, 10, 342, 1057, 1052, 1096, 1058, 59, 834, 825, 640, 827, 273, 274] }, Tag { name: "minecraft:plays_ambient_desert_block_sounds".to_string(), entries: vec![522, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466, 467, 37, 39] }, Tag { name: "minecraft:bats_spawnable_on".to_string(), entries: vec![1, 2, 4, 6, 943, 1059] }, Tag { name: "minecraft:stone_ore_replaceables".to_string(), entries: vec![1, 2, 4, 6] }, Tag { name: "minecraft:redstone_ores".to_string(), entries: vec![258, 259] }, Tag { name: "minecraft:trapdoors".to_string(), entries: vec![305, 303, 307, 308, 304, 301, 302, 850, 851, 309, 310, 306, 494, 1016, 1017, 1019, 1018, 1020, 1021, 1023, 1022] }, Tag { name: "minecraft:cherry_logs".to_string(), entries: vec![54, 76, 65, 84] }, Tag { name: "minecraft:fall_damage_resetting".to_string(), entries: vec![208, 335, 804, 837, 838, 839, 840, 1043, 1044, 820, 129] }, Tag { name: "minecraft:buttons".to_string(), entries: vec![411, 412, 413, 414, 415, 417, 418, 856, 857, 419, 420, 416, 262, 898] }, Tag { name: "minecraft:corals".to_string(), entries: vec![730, 731, 732, 733, 734, 740, 741, 742, 743, 744] }, Tag { name: "minecraft:flowers".to_string(), entries: vec![157, 1099, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 170, 169, 158, 1100, 525, 526, 528, 527, 632, 98, 1047, 33, 93, 1049, 1050, 625, 1045, 267] }, Tag { name: "minecraft:combination_step_sound_blocks".to_string(), entries: vec![506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517, 518, 519, 520, 521, 1048, 1097, 263, 829, 828, 841, 337] }, Tag { name: "minecraft:rabbits_spawnable_on".to_string(), entries: vec![8, 263, 265, 37] }, Tag { name: "minecraft:planks".to_string(), entries: vec![13, 14, 15, 16, 17, 19, 21, 842, 843, 22, 23, 18] }, Tag { name: "minecraft:stone_buttons".to_string(), entries: vec![262, 898] }, Tag { name: "minecraft:does_not_block_hoppers".to_string(), entries: vec![870, 871] }, Tag { name: "minecraft:soul_speed_blocks".to_string(), entries: vec![273, 274] }, Tag { name: "minecraft:rails".to_string(), entries: vec![209, 126, 127, 450] }, Tag { name: "minecraft:diamond_ores".to_string(), entries: vec![190, 191] }, Tag { name: "minecraft:geode_invalid_blocks".to_string(), entries: vec![34, 35, 36, 264, 524, 756] }, Tag { name: "minecraft:badlands_terracotta".to_string(), entries: vec![522, 452, 456, 453, 466, 464, 460] }, Tag { name: "minecraft:all_hanging_signs".to_string(), entries: vec![221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 242, 243, 241, 244] }, Tag { name: "minecraft:overworld_natural_logs".to_string(), entries: vec![53, 51, 49, 52, 50, 55, 56, 57, 54] }, Tag { name: "minecraft:leaves".to_string(), entries: vec![91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93] }, Tag { name: "minecraft:deepslate_ore_replaceables".to_string(), entries: vec![1059, 943] }, Tag { name: "minecraft:walls".to_string(), entries: vec![378, 379, 791, 792, 793, 794, 795, 796, 798, 799, 800, 801, 802, 803, 885, 893, 899, 1063, 1067, 1071, 1075, 797, 946, 950, 955, 348] }, Tag { name: "minecraft:cave_vines".to_string(), entries: vec![1044, 1043] }, Tag { name: "minecraft:coral_blocks".to_string(), entries: vec![720, 721, 722, 723, 724] }, Tag { name: "minecraft:strider_warm_blocks".to_string(), entries: vec![36] }, Tag { name: "minecraft:fence_gates".to_string(), entries: vec![599, 597, 601, 602, 598, 338, 596, 852, 853, 603, 604, 600] }, Tag { name: "minecraft:bee_growables".to_string(), entries: vec![633, 409, 410, 194, 334, 333, 630, 631, 820, 1043, 1044] }, Tag { name: "minecraft:incorrect_for_gold_tool".to_string(), entries: vec![180, 876, 874, 877, 875, 192, 190, 191, 367, 368, 372, 173, 1083, 42, 43, 258, 259, 174, 1081, 44, 45, 104, 102, 103, 966, 1082, 970, 971, 991, 987, 975, 968, 989, 985, 973, 969, 988, 984, 972, 967, 990, 986, 974, 992, 1007, 1003, 999, 993, 1005, 1001, 997, 994, 1006, 1002, 998, 995, 1004, 1000, 996, 1040, 1092, 979, 978, 977, 976, 983, 982, 981, 980, 1024, 1025, 1026, 1027, 1028, 1029, 1030, 1031, 1032, 1033, 1034, 1035, 1036, 1037, 1038, 1039, 1016, 1017, 1019, 1018, 1020, 1021, 1023, 1022] }, Tag { name: "minecraft:wither_immune".to_string(), entries: vec![492, 34, 360, 361, 635, 376, 636, 637, 864, 865, 156, 493, 1090, 866, 867] }, Tag { name: "minecraft:wooden_pressure_plates".to_string(), entries: vec![248, 249, 250, 251, 252, 254, 255, 846, 847, 256, 257, 253] }, Tag { name: "minecraft:acacia_logs".to_string(), entries: vec![53, 75, 64, 83] }, Tag { name: "minecraft:armadillo_spawnable_on".to_string(), entries: vec![8, 522, 452, 456, 453, 466, 464, 460, 39, 10] }, Tag { name: "minecraft:incorrect_for_netherite_tool".to_string(), entries: vec![] }, Tag { name: "minecraft:candles".to_string(), entries: vec![903, 904, 905, 906, 907, 908, 909, 910, 911, 912, 913, 914, 915, 916, 917, 918, 919] }, Tag { name: "minecraft:dragon_transparent".to_string(), entries: vec![493, 183, 184] }, Tag { name: "minecraft:underwater_bonemeals".to_string(), entries: vec![136, 730, 731, 732, 733, 734, 740, 741, 742, 743, 744, 750, 751, 752, 753, 754] }, Tag { name: "minecraft:stone_pressure_plates".to_string(), entries: vec![246, 897] }, Tag { name: "minecraft:impermeable".to_string(), entries: vec![101, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 958] }, Tag { name: "minecraft:copper_ores".to_string(), entries: vec![970, 971] }, Tag { name: "minecraft:snow".to_string(), entries: vec![263, 265, 959] }, Tag { name: "minecraft:sand".to_string(), entries: vec![37, 39, 38] }, Tag { name: "minecraft:nylium".to_string(), entries: vec![834, 825] }, Tag { name: "minecraft:gold_ores".to_string(), entries: vec![42, 48, 43] }, Tag { name: "minecraft:small_dripleaf_placeable".to_string(), entries: vec![268, 1052] }, Tag { name: "minecraft:incorrect_for_diamond_tool".to_string(), entries: vec![] }, Tag { name: "minecraft:logs_that_burn".to_string(), entries: vec![55, 77, 66, 85, 56, 20, 67, 86, 49, 71, 68, 79, 53, 75, 64, 83, 51, 73, 62, 81, 52, 74, 63, 82, 50, 72, 61, 80, 57, 78, 69, 87, 54, 76, 65, 84] }, Tag { name: "minecraft:camel_sand_step_sound_blocks".to_string(), entries: vec![37, 39, 38, 694, 695, 696, 697, 698, 699, 700, 701, 702, 703, 704, 705, 706, 707, 708, 709] }, Tag { name: "minecraft:completes_find_tree_tutorial".to_string(), entries: vec![55, 77, 66, 85, 56, 20, 67, 86, 49, 71, 68, 79, 53, 75, 64, 83, 51, 73, 62, 81, 52, 74, 63, 82, 50, 72, 61, 80, 57, 78, 69, 87, 54, 76, 65, 84, 830, 831, 832, 833, 821, 822, 823, 824, 91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93, 640, 827] }, Tag { name: "minecraft:mineable/hoe".to_string(), entries: vec![91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93, 640, 827, 505, 712, 869, 836, 99, 100, 960, 961, 1052, 1048, 1096, 1097, 962, 964, 963, 965] }, Tag { name: "minecraft:replaceable".to_string(), entries: vec![0, 35, 36, 130, 131, 132, 133, 134, 135, 136, 137, 183, 184, 263, 335, 336, 337, 493, 529, 530, 643, 761, 762, 763, 828, 829, 841, 1051, 1056] }, Tag { name: "minecraft:dirt".to_string(), entries: vec![9, 8, 11, 10, 342, 1057, 1052, 1096, 1058, 59] }, Tag { name: "minecraft:candle_cakes".to_string(), entries: vec![920, 921, 922, 923, 924, 925, 926, 927, 928, 929, 930, 931, 932, 933, 934, 935, 936] }, Tag { name: "minecraft:needs_iron_tool".to_string(), entries: vec![192, 190, 191, 367, 368, 372, 173, 1083, 42, 43, 258, 259] }]), ("minecraft:banner_pattern".to_string(), vec![Tag { name: "minecraft:no_item_required".to_string(), entries: vec![26, 27, 28, 29, 31, 38, 35, 37, 32, 36, 34, 33, 25, 5, 30, 39, 40, 41, 42, 7, 10, 9, 8, 3, 23, 19, 17, 20, 18, 1, 14, 15] }, Tag { name: "minecraft:pattern_item/piglin".to_string(), entries: vec![22] }, Tag { name: "minecraft:pattern_item/field_masoned".to_string(), entries: vec![2] }, Tag { name: "minecraft:pattern_item/flower".to_string(), entries: vec![12] }, Tag { name: "minecraft:pattern_item/flow".to_string(), entries: vec![11] }, Tag { name: "minecraft:pattern_item/creeper".to_string(), entries: vec![4] }, Tag { name: "minecraft:pattern_item/globe".to_string(), entries: vec![13] }, Tag { name: "minecraft:pattern_item/bordure_indented".to_string(), entries: vec![6] }, Tag { name: "minecraft:pattern_item/guster".to_string(), entries: vec![16] }, Tag { name: "minecraft:pattern_item/mojang".to_string(), entries: vec![21] }, Tag { name: "minecraft:pattern_item/skull".to_string(), entries: vec![24] }]), ("minecraft:enchantment".to_string(), vec![Tag { name: "minecraft:tooltip_order".to_string(), entries: vec![2, 40, 31, 5, 41, 14, 32, 34, 1, 15, 25, 6, 4, 24, 36, 23, 10, 12, 17, 28, 27, 3, 11, 26, 9, 13, 18, 33, 20, 8, 29, 21, 30, 0, 35, 37, 7, 38, 19, 39, 16, 22] }, Tag { name: "minecraft:double_trade_price".to_string(), entries: vec![2, 40, 37, 35, 14, 22, 41] }, Tag { name: "minecraft:treasure".to_string(), entries: vec![2, 40, 37, 35, 14, 22, 41] }, Tag { name: "minecraft:prevents_decorated_pot_shattering".to_string(), entries: vec![33] }, Tag { name: "minecraft:tradeable".to_string(), entries: vec![27, 11, 9, 3, 26, 30, 0, 38, 7, 32, 34, 1, 17, 10, 18, 36, 8, 33, 39, 13, 25, 28, 12, 16, 20, 21, 19, 15, 31, 5, 23, 29, 24, 6, 4, 2, 40, 14, 22] }, Tag { name: "minecraft:on_traded_equipment".to_string(), entries: vec![27, 11, 9, 3, 26, 30, 0, 38, 7, 32, 34, 1, 17, 10, 18, 36, 8, 33, 39, 13, 25, 28, 12, 16, 20, 21, 19, 15, 31, 5, 23, 29, 24, 6, 4] }, Tag { name: "minecraft:on_random_loot".to_string(), entries: vec![27, 11, 9, 3, 26, 30, 0, 38, 7, 32, 34, 1, 17, 10, 18, 36, 8, 33, 39, 13, 25, 28, 12, 16, 20, 21, 19, 15, 31, 5, 23, 29, 24, 6, 4, 2, 40, 14, 22] }, Tag { name: "minecraft:exclusive_set/bow".to_string(), entries: vec![16, 22] }, Tag { name: "minecraft:prevents_bee_spawns_when_mining".to_string(), entries: vec![33] }, Tag { name: "minecraft:prevents_infested_spawns".to_string(), entries: vec![33] }, Tag { name: "minecraft:non_treasure".to_string(), entries: vec![27, 11, 9, 3, 26, 30, 0, 38, 7, 32, 34, 1, 17, 10, 18, 36, 8, 33, 39, 13, 25, 28, 12, 16, 20, 21, 19, 15, 31, 5, 23, 29, 24, 6, 4] }, Tag { name: "minecraft:exclusive_set/riptide".to_string(), entries: vec![19, 5] }, Tag { name: "minecraft:exclusive_set/damage".to_string(), entries: vec![32, 34, 1, 15, 6, 4] }, Tag { name: "minecraft:on_mob_spawn_equipment".to_string(), entries: vec![27, 11, 9, 3, 26, 30, 0, 38, 7, 32, 34, 1, 17, 10, 18, 36, 8, 33, 39, 13, 25, 28, 12, 16, 20, 21, 19, 15, 31, 5, 23, 29, 24, 6, 4] }, Tag { name: "minecraft:smelts_loot".to_string(), entries: vec![10] }, Tag { name: "minecraft:exclusive_set/crossbow".to_string(), entries: vec![23, 24] }, Tag { name: "minecraft:exclusive_set/armor".to_string(), entries: vec![27, 3, 11, 26] }, Tag { name: "minecraft:exclusive_set/boots".to_string(), entries: vec![14, 7] }, Tag { name: "minecraft:in_enchanting_table".to_string(), entries: vec![27, 11, 9, 3, 26, 30, 0, 38, 7, 32, 34, 1, 17, 10, 18, 36, 8, 33, 39, 13, 25, 28, 12, 16, 20, 21, 19, 15, 31, 5, 23, 29, 24, 6, 4] }, Tag { name: "minecraft:prevents_ice_melting".to_string(), entries: vec![33] }, Tag { name: "minecraft:curse".to_string(), entries: vec![2, 40] }, Tag { name: "minecraft:exclusive_set/mining".to_string(), entries: vec![13, 33] }]), ("minecraft:damage_type".to_string(), vec![Tag { name: "minecraft:bypasses_effects".to_string(), entries: vec![39] }, Tag { name: "minecraft:is_fall".to_string(), entries: vec![10, 8, 38] }, Tag { name: "minecraft:ignites_armor_stands".to_string(), entries: vec![21, 3] }, Tag { name: "minecraft:witch_resistant_to".to_string(), entries: vec![27, 23, 36, 42] }, Tag { name: "minecraft:burns_armor_stands".to_string(), entries: vec![31] }, Tag { name: "minecraft:bypasses_resistance".to_string(), entries: vec![32, 19] }, Tag { name: "minecraft:avoids_guardian_thorns".to_string(), entries: vec![27, 42, 15, 9, 35, 1] }, Tag { name: "minecraft:panic_environmental_causes".to_string(), entries: vec![2, 17, 20, 21, 24, 25, 31] }, Tag { name: "minecraft:mace_smash".to_string(), entries: vec![26] }, Tag { name: "minecraft:bypasses_wolf_armor".to_string(), entries: vec![32, 19, 4, 6, 7, 17, 22, 23, 27, 33, 39, 42, 47] }, Tag { name: "minecraft:damages_helmet".to_string(), entries: vec![11, 12, 13] }, Tag { name: "minecraft:burn_from_stepping".to_string(), entries: vec![3, 20] }, Tag { name: "minecraft:bypasses_shield".to_string(), entries: vec![31, 22, 4, 6, 16, 18, 47, 5, 39, 10, 8, 17, 38, 27, 23, 32, 19, 36, 33, 2, 3, 7, 11, 13, 20, 21, 24, 25, 41] }, Tag { name: "minecraft:no_impact".to_string(), entries: vec![6] }, Tag { name: "minecraft:is_freezing".to_string(), entries: vec![17] }, Tag { name: "minecraft:is_projectile".to_string(), entries: vec![0, 44, 30, 45, 14, 48, 43, 46] }, Tag { name: "minecraft:bypasses_enchantments".to_string(), entries: vec![36] }, Tag { name: "minecraft:always_most_significant_fall".to_string(), entries: vec![32] }, Tag { name: "minecraft:can_break_armor_stand".to_string(), entries: vec![35, 34, 26] }, Tag { name: "minecraft:bypasses_armor".to_string(), entries: vec![31, 22, 4, 6, 16, 18, 47, 5, 39, 10, 8, 17, 38, 27, 23, 32, 19, 36, 33] }, Tag { name: "minecraft:bypasses_invulnerability".to_string(), entries: vec![32, 19] }, Tag { name: "minecraft:no_anger".to_string(), entries: vec![29] }, Tag { name: "minecraft:no_knockback".to_string(), entries: vec![9, 35, 1, 21, 25, 31, 24, 20, 22, 4, 6, 39, 2, 10, 8, 16, 32, 18, 27, 47, 5, 7, 41, 17, 38, 33, 19, 3] }, Tag { name: "minecraft:is_drowning".to_string(), entries: vec![6] }, Tag { name: "minecraft:panic_causes".to_string(), entries: vec![2, 17, 20, 21, 24, 25, 31, 0, 5, 9, 14, 15, 23, 27, 28, 30, 35, 36, 40, 43, 44, 45, 46, 47, 48, 34, 26] }, Tag { name: "minecraft:always_triggers_silverfish".to_string(), entries: vec![27] }, Tag { name: "minecraft:always_hurts_ender_dragons".to_string(), entries: vec![15, 9, 35, 1] }, Tag { name: "minecraft:is_fire".to_string(), entries: vec![21, 3, 31, 24, 20, 45, 14] }, Tag { name: "minecraft:wither_immune_to".to_string(), entries: vec![6] }, Tag { name: "minecraft:is_explosion".to_string(), entries: vec![15, 9, 35, 1] }, Tag { name: "minecraft:always_kills_armor_stands".to_string(), entries: vec![0, 44, 14, 48, 46] }, Tag { name: "minecraft:is_lightning".to_string(), entries: vec![25] }, Tag { name: "minecraft:is_player_attack".to_string(), entries: vec![34, 26] }]), ("minecraft:painting_variant".to_string(), vec![Tag { name: "minecraft:placeable".to_string(), entries: vec![23, 1, 0, 2, 5, 31, 46, 34, 12, 36, 41, 13, 45, 21, 25, 8, 39, 44, 38, 49, 18, 32, 30, 7, 37, 14, 4, 22, 26, 35, 43, 3, 6, 9, 10, 11, 16, 17, 19, 24, 27, 28, 29, 33, 40, 42] }]), ("minecraft:entity_type".to_string(), vec![Tag { name: "minecraft:axolotl_always_hostiles".to_string(), entries: vec![36, 60, 38] }, Tag { name: "minecraft:freeze_immune_entity_types".to_string(), entries: vec![122, 98, 115, 139] }, Tag { name: "minecraft:immune_to_oozing".to_string(), entries: vec![111] }, Tag { name: "minecraft:illager".to_string(), entries: vec![44, 65, 97, 134] }, Tag { name: "minecraft:undead".to_string(), entries: vec![109, 122, 140, 110, 16, 145, 144, 146, 147, 143, 36, 64, 139, 93] }, Tag { name: "minecraft:sensitive_to_bane_of_arthropods".to_string(), entries: vec![11, 40, 108, 118, 21] }, Tag { name: "minecraft:boat".to_string(), entries: vec![84, 119, 12, 71, 0, 22, 31, 89, 78, 9] }, Tag { name: "minecraft:frog_food".to_string(), entries: vec![111, 77] }, Tag { name: "minecraft:can_equip_saddle".to_string(), entries: vec![63, 110, 145, 34, 83, 94, 123, 19] }, Tag { name: "minecraft:redirectable_projectile".to_string(), entries: vec![50, 137, 18] }, Tag { name: "minecraft:fall_damage_immune".to_string(), entries: vec![67, 115, 106, 2, 10, 11, 14, 20, 25, 55, 93, 77, 86, 92, 139, 17] }, Tag { name: "minecraft:raiders".to_string(), entries: vec![44, 97, 103, 134, 65, 138] }, Tag { name: "minecraft:powder_snow_walkable_mobs".to_string(), entries: vec![102, 40, 108, 52] }, Tag { name: "minecraft:inverted_healing_and_harm".to_string(), entries: vec![109, 122, 140, 110, 16, 145, 144, 146, 147, 143, 36, 64, 139, 93] }, Tag { name: "minecraft:arthropod".to_string(), entries: vec![11, 40, 108, 118, 21] }, Tag { name: "minecraft:immune_to_infested".to_string(), entries: vec![108] }, Tag { name: "minecraft:axolotl_hunt_targets".to_string(), entries: vec![130, 101, 104, 26, 121, 58, 124] }, Tag { name: "minecraft:zombies".to_string(), entries: vec![145, 144, 146, 147, 143, 36, 64] }, Tag { name: "minecraft:sensitive_to_smite".to_string(), entries: vec![109, 122, 140, 110, 16, 145, 144, 146, 147, 143, 36, 64, 139, 93] }, Tag { name: "minecraft:dismounts_underwater".to_string(), entries: vec![19, 25, 34, 63, 75, 83, 94, 103, 118, 123, 128, 145] }, Tag { name: "minecraft:can_wear_horse_armor".to_string(), entries: vec![63] }, Tag { name: "minecraft:beehive_inhabitors".to_string(), entries: vec![11] }, Tag { name: "minecraft:impact_projectiles".to_string(), entries: vec![6, 117, 51, 114, 50, 112, 37, 129, 35, 141, 137, 18] }, Tag { name: "minecraft:not_scary_for_pufferfish".to_string(), entries: vec![131, 60, 38, 26, 101, 104, 130, 33, 121, 58, 124] }, Tag { name: "minecraft:non_controlling_rider".to_string(), entries: vec![111, 77] }, Tag { name: "minecraft:skeletons".to_string(), entries: vec![109, 122, 140, 110, 16] }, Tag { name: "minecraft:can_turn_in_boats".to_string(), entries: vec![17] }, Tag { name: "minecraft:can_breathe_under_water".to_string(), entries: vec![109, 122, 140, 110, 16, 145, 144, 146, 147, 143, 36, 64, 139, 93, 7, 53, 60, 38, 131, 58, 26, 101, 104, 121, 130, 124, 5] }, Tag { name: "minecraft:arrows".to_string(), entries: vec![6, 117] }, Tag { name: "minecraft:wither_friends".to_string(), entries: vec![109, 122, 140, 110, 16, 145, 144, 146, 147, 143, 36, 64, 139, 93] }, Tag { name: "minecraft:freeze_hurts_extra_types".to_string(), entries: vec![123, 14, 77] }, Tag { name: "minecraft:no_anger_from_wind_charge".to_string(), entries: vec![17, 109, 16, 122, 144, 64, 118, 21, 111] }, Tag { name: "minecraft:aquatic".to_string(), entries: vec![131, 7, 60, 38, 26, 101, 104, 130, 33, 121, 58, 124] }, Tag { name: "minecraft:illager_friends".to_string(), entries: vec![44, 65, 97, 134] }, Tag { name: "minecraft:ignores_poison_and_regen".to_string(), entries: vec![109, 122, 140, 110, 16, 145, 144, 146, 147, 143, 36, 64, 139, 93] }, Tag { name: "minecraft:deflects_projectiles".to_string(), entries: vec![17] }, Tag { name: "minecraft:sensitive_to_impaling".to_string(), entries: vec![131, 7, 60, 38, 26, 101, 104, 130, 33, 121, 58, 124] }]), ("minecraft:instrument".to_string(), vec![Tag { name: "minecraft:screaming_goat_horns".to_string(), entries: vec![0, 1, 7, 2] }, Tag { name: "minecraft:goat_horns".to_string(), entries: vec![4, 6, 5, 3, 0, 1, 7, 2] }, Tag { name: "minecraft:regular_goat_horns".to_string(), entries: vec![4, 6, 5, 3] }]), ("minecraft:worldgen/biome".to_string(), vec![Tag { name: "minecraft:has_structure/nether_fortress".to_string(), entries: vec![34, 49, 7, 59, 2] }, Tag { name: "minecraft:has_structure/ocean_ruin_warm".to_string(), entries: vec![29, 58, 12] }, Tag { name: "minecraft:is_river".to_string(), entries: vec![41, 24] }, Tag { name: "minecraft:water_on_map_outlines".to_string(), entries: vec![11, 9, 13, 12, 22, 35, 6, 29, 58, 41, 24, 54, 31] }, Tag { name: "minecraft:has_structure/mineshaft".to_string(), entries: vec![11, 9, 13, 12, 22, 35, 6, 29, 58, 41, 24, 3, 45, 32, 23, 27, 51, 47, 5, 62, 60, 61, 55, 48, 37, 38, 1, 28, 50, 21, 20, 4, 36, 8, 39, 25, 52, 33, 26, 63, 14, 42, 46, 40, 53, 54, 31, 43, 15, 30] }, Tag { name: "minecraft:has_structure/jungle_temple".to_string(), entries: vec![1, 28] }, Tag { name: "minecraft:has_structure/trial_chambers".to_string(), entries: vec![33, 11, 22, 9, 6, 13, 35, 12, 29, 58, 52, 54, 31, 47, 46, 45, 61, 25, 62, 48, 60, 55, 40, 32, 3, 21, 38, 20, 4, 8, 39, 43, 42, 28, 0, 14, 64, 27, 51, 24, 41, 26, 37, 53, 36, 50, 1, 19, 63, 5, 23, 15, 30] }, Tag { name: "minecraft:more_frequent_drowned_spawns".to_string(), entries: vec![41, 24] }, Tag { name: "minecraft:has_structure/village_desert".to_string(), entries: vec![14] }, Tag { name: "minecraft:is_jungle".to_string(), entries: vec![1, 28, 50] }, Tag { name: "minecraft:spawns_snow_foxes".to_string(), entries: vec![46, 26, 22, 48, 24, 45, 23, 27, 47, 25] }, Tag { name: "minecraft:is_ocean".to_string(), entries: vec![11, 9, 13, 12, 22, 35, 6, 29, 58] }, Tag { name: "minecraft:is_forest".to_string(), entries: vec![21, 20, 4, 36, 8, 39, 25] }, Tag { name: "minecraft:has_structure/village_savanna".to_string(), entries: vec![42] }, Tag { name: "minecraft:is_overworld".to_string(), entries: vec![33, 11, 22, 9, 6, 13, 35, 12, 29, 58, 52, 54, 31, 47, 46, 45, 61, 25, 62, 48, 60, 55, 40, 32, 3, 21, 38, 20, 4, 8, 39, 43, 42, 28, 0, 14, 64, 27, 51, 24, 41, 26, 37, 53, 36, 50, 1, 19, 63, 5, 23, 15, 30, 10] }, Tag { name: "minecraft:without_patrol_spawns".to_string(), entries: vec![33] }, Tag { name: "minecraft:has_structure/village_taiga".to_string(), entries: vec![55] }, Tag { name: "minecraft:allows_tropical_fish_spawns_at_any_height".to_string(), entries: vec![30] }, Tag { name: "minecraft:polar_bears_spawn_on_alternate_blocks".to_string(), entries: vec![22, 11] }, Tag { name: "minecraft:has_closer_water_fog".to_string(), entries: vec![54, 31] }, Tag { name: "minecraft:has_structure/village_plains".to_string(), entries: vec![40, 32] }, Tag { name: "minecraft:has_structure/buried_treasure".to_string(), entries: vec![3, 45] }, Tag { name: "minecraft:has_structure/ocean_ruin_cold".to_string(), entries: vec![22, 6, 35, 11, 9, 13] }, Tag { name: "minecraft:produces_corals_from_bonemeal".to_string(), entries: vec![58] }, Tag { name: "minecraft:snow_golem_melts".to_string(), entries: vec![0, 2, 7, 14, 19, 34, 42, 43, 49, 59, 63, 64] }, Tag { name: "minecraft:has_structure/stronghold".to_string(), entries: vec![33, 11, 22, 9, 6, 13, 35, 12, 29, 58, 52, 54, 31, 47, 46, 45, 61, 25, 62, 48, 60, 55, 40, 32, 3, 21, 38, 20, 4, 8, 39, 43, 42, 28, 0, 14, 64, 27, 51, 24, 41, 26, 37, 53, 36, 50, 1, 19, 63, 5, 23, 15, 30, 10] }, Tag { name: "minecraft:without_wandering_trader_spawns".to_string(), entries: vec![57] }, Tag { name: "minecraft:has_structure/ruined_portal_desert".to_string(), entries: vec![14] }, Tag { name: "minecraft:has_structure/mineshaft_mesa".to_string(), entries: vec![0, 19, 64] }, Tag { name: "minecraft:has_structure/ancient_city".to_string(), entries: vec![10] }, Tag { name: "minecraft:has_structure/ruined_portal_swamp".to_string(), entries: vec![54, 31] }, Tag { name: "minecraft:has_structure/shipwreck".to_string(), entries: vec![11, 9, 13, 12, 22, 35, 6, 29, 58] }, Tag { name: "minecraft:is_savanna".to_string(), entries: vec![42, 43, 63] }, Tag { name: "minecraft:has_structure/swamp_hut".to_string(), entries: vec![54] }, Tag { name: "minecraft:spawns_cold_variant_frogs".to_string(), entries: vec![46, 26, 23, 27, 47, 22, 11, 25, 10, 24, 48, 45, 56, 17, 18, 44, 16] }, Tag { name: "minecraft:has_structure/igloo".to_string(), entries: vec![48, 46, 47] }, Tag { name: "minecraft:has_structure/shipwreck_beached".to_string(), entries: vec![3, 45] }, Tag { name: "minecraft:has_structure/trail_ruins".to_string(), entries: vec![55, 48, 37, 38, 36, 28] }, Tag { name: "minecraft:is_hill".to_string(), entries: vec![62, 60, 61] }, Tag { name: "minecraft:has_structure/ruined_portal_nether".to_string(), entries: vec![34, 49, 7, 59, 2] }, Tag { name: "minecraft:is_end".to_string(), entries: vec![56, 17, 18, 44, 16] }, Tag { name: "minecraft:stronghold_biased_to".to_string(), entries: vec![40, 53, 46, 26, 14, 21, 20, 4, 8, 39, 36, 37, 38, 55, 48, 42, 43, 62, 61, 60, 63, 28, 50, 1, 0, 19, 64, 32, 25, 47, 23, 27, 51, 33, 15, 30] }, Tag { name: "minecraft:without_zombie_sieges".to_string(), entries: vec![33] }, Tag { name: "minecraft:is_beach".to_string(), entries: vec![3, 45] }, Tag { name: "minecraft:has_structure/ruined_portal_standard".to_string(), entries: vec![3, 45, 41, 24, 55, 48, 37, 38, 21, 20, 4, 36, 8, 39, 25, 33, 26, 15, 30, 42, 46, 40, 53] }, Tag { name: "minecraft:has_structure/pillager_outpost".to_string(), entries: vec![14, 40, 42, 46, 55, 32, 23, 27, 51, 47, 5, 25] }, Tag { name: "minecraft:allows_surface_slime_spawns".to_string(), entries: vec![54, 31] }, Tag { name: "minecraft:is_taiga".to_string(), entries: vec![55, 48, 37, 38] }, Tag { name: "minecraft:spawns_warm_variant_frogs".to_string(), entries: vec![14, 58, 1, 28, 50, 42, 43, 63, 34, 49, 7, 59, 2, 0, 19, 64, 31] }, Tag { name: "minecraft:mineshaft_blocking".to_string(), entries: vec![10] }, Tag { name: "minecraft:required_ocean_monument_surrounding".to_string(), entries: vec![11, 9, 13, 12, 22, 35, 6, 29, 58, 41, 24] }, Tag { name: "minecraft:is_mountain".to_string(), entries: vec![32, 23, 27, 51, 47, 5] }, Tag { name: "minecraft:has_structure/village_snowy".to_string(), entries: vec![46] }, Tag { name: "minecraft:has_structure/ocean_monument".to_string(), entries: vec![11, 9, 13, 12] }, Tag { name: "minecraft:is_badlands".to_string(), entries: vec![0, 19, 64] }, Tag { name: "minecraft:spawns_gold_rabbits".to_string(), entries: vec![14] }, Tag { name: "minecraft:spawns_white_rabbits".to_string(), entries: vec![46, 26, 22, 48, 24, 45, 23, 27, 47, 25] }, Tag { name: "minecraft:has_structure/ruined_portal_jungle".to_string(), entries: vec![1, 28, 50] }, Tag { name: "minecraft:plays_underwater_music".to_string(), entries: vec![11, 9, 13, 12, 22, 35, 6, 29, 58, 41, 24] }, Tag { name: "minecraft:has_structure/desert_pyramid".to_string(), entries: vec![14] }, Tag { name: "minecraft:has_structure/nether_fossil".to_string(), entries: vec![49] }, Tag { name: "minecraft:spawns_cold_variant_farm_animals".to_string(), entries: vec![46, 26, 23, 27, 47, 22, 11, 25, 10, 24, 48, 45, 56, 17, 18, 44, 16, 6, 9, 37, 38, 55, 60, 61, 62, 51] }, Tag { name: "minecraft:has_structure/ruined_portal_ocean".to_string(), entries: vec![11, 9, 13, 12, 22, 35, 6, 29, 58] }, Tag { name: "minecraft:has_structure/end_city".to_string(), entries: vec![17, 18] }, Tag { name: "minecraft:is_deep_ocean".to_string(), entries: vec![11, 9, 13, 12] }, Tag { name: "minecraft:is_nether".to_string(), entries: vec![34, 49, 7, 59, 2] }, Tag { name: "minecraft:reduce_water_ambient_spawns".to_string(), entries: vec![41, 24] }, Tag { name: "minecraft:has_structure/bastion_remnant".to_string(), entries: vec![7, 34, 49, 59] }, Tag { name: "minecraft:spawns_warm_variant_farm_animals".to_string(), entries: vec![14, 58, 1, 28, 50, 42, 43, 63, 34, 49, 7, 59, 2, 0, 19, 64, 31, 12, 29] }, Tag { name: "minecraft:increased_fire_burnout".to_string(), entries: vec![1, 33, 31, 47, 23, 27, 54, 28] }, Tag { name: "minecraft:has_structure/woodland_mansion".to_string(), entries: vec![8, 39] }, Tag { name: "minecraft:has_structure/ruined_portal_mountain".to_string(), entries: vec![0, 19, 64, 62, 60, 61, 43, 63, 52, 32, 23, 27, 51, 47, 5] }]), ("minecraft:game_event".to_string(), vec![Tag { name: "minecraft:vibrations".to_string(), entries: vec![1, 2, 3, 5, 6, 7, 8, 0, 4, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 24, 25, 26, 27, 28, 32, 33, 34, 35, 36, 38, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 23] }, Tag { name: "minecraft:ignore_vibrations_sneaking".to_string(), entries: vec![26, 36, 41, 42, 29, 28] }, Tag { name: "minecraft:warden_can_listen".to_string(), entries: vec![1, 2, 3, 5, 6, 7, 8, 0, 4, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 24, 25, 26, 27, 28, 32, 33, 34, 35, 36, 38, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 39, 37] }, Tag { name: "minecraft:allay_can_listen".to_string(), entries: vec![33] }, Tag { name: "minecraft:shrieker_can_listen".to_string(), entries: vec![37] }]), ("minecraft:fluid".to_string(), vec![Tag { name: "minecraft:lava".to_string(), entries: vec![4, 3] }, Tag { name: "minecraft:water".to_string(), entries: vec![2, 1] }]), ("minecraft:item".to_string(), vec![Tag { name: "minecraft:skulls".to_string(), entries: vec![1167, 1169, 1168, 1165, 1166, 1170, 1171] }, Tag { name: "minecraft:soul_fire_base_blocks".to_string(), entries: vec![348, 349] }, Tag { name: "minecraft:trim_materials".to_string(), entries: vec![849, 853, 845, 846, 855, 851, 847, 856, 848, 687, 1178] }, Tag { name: "minecraft:head_armor".to_string(), entries: vec![896, 900, 912, 904, 908, 916, 834] }, Tag { name: "minecraft:beacon_payment_items".to_string(), entries: vec![856, 846, 845, 855, 851] }, Tag { name: "minecraft:wooden_slabs".to_string(), entries: vec![270, 271, 272, 273, 274, 276, 277, 281, 282, 278, 279, 275] }, Tag { name: "minecraft:pale_oak_logs".to_string(), entries: vec![140, 177, 155, 166] }, Tag { name: "minecraft:chicken_food".to_string(), entries: vec![893, 1047, 1046, 1218, 1215, 1216] }, Tag { name: "minecraft:coal_ores".to_string(), entries: vec![64, 65] }, Tag { name: "minecraft:small_flowers".to_string(), entries: vec![229, 230, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 231] }, Tag { name: "minecraft:parrot_poisonous_food".to_string(), entries: vec![1040] }, Tag { name: "minecraft:wooden_trapdoors".to_string(), entries: vec![768, 766, 770, 771, 767, 764, 765, 774, 775, 772, 773, 769] }, Tag { name: "minecraft:pig_food".to_string(), entries: vec![1159, 1160, 1217] }, Tag { name: "minecraft:repairs_leather_armor".to_string(), entries: vec![955] }, Tag { name: "minecraft:repairs_diamond_armor".to_string(), entries: vec![845] }, Tag { name: "minecraft:iron_tool_materials".to_string(), entries: vec![851] }, Tag { name: "minecraft:trimmable_armor".to_string(), entries: vec![899, 903, 915, 907, 911, 919, 898, 902, 914, 906, 910, 918, 897, 901, 913, 905, 909, 917, 896, 900, 912, 904, 908, 916, 834] }, Tag { name: "minecraft:piglin_safe_armor".to_string(), entries: vec![912, 913, 914, 915] }, Tag { name: "minecraft:enchantable/mace".to_string(), entries: vec![1155] }, Tag { name: "minecraft:wool".to_string(), entries: vec![213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228] }, Tag { name: "minecraft:stairs".to_string(), entries: vec![412, 413, 414, 415, 416, 418, 419, 423, 424, 420, 421, 417, 422, 324, 409, 399, 391, 390, 316, 456, 543, 537, 536, 538, 651, 652, 653, 654, 655, 656, 657, 658, 659, 660, 661, 662, 663, 664, 1294, 1302, 1298, 665, 666, 668, 667, 109, 108, 107, 106, 128, 127, 126, 129, 392, 14, 19, 23, 386] }, Tag { name: "minecraft:logs".to_string(), entries: vec![141, 178, 154, 165, 140, 177, 155, 166, 134, 171, 148, 159, 138, 175, 152, 163, 136, 173, 150, 161, 137, 174, 151, 162, 135, 172, 149, 160, 142, 179, 156, 167, 139, 176, 153, 164, 145, 157, 180, 168, 146, 158, 181, 169] }, Tag { name: "minecraft:creeper_drop_music_discs".to_string(), entries: vec![1231, 1232, 1233, 1234, 1237, 1238, 1239, 1240, 1241, 1242, 1243, 1244] }, Tag { name: "minecraft:camel_food".to_string(), entries: vec![328] }, Tag { name: "minecraft:gold_tool_materials".to_string(), entries: vec![855] }, Tag { name: "minecraft:compasses".to_string(), entries: vec![972, 973] }, Tag { name: "minecraft:arrows".to_string(), entries: vec![842, 1223, 1222] }, Tag { name: "minecraft:repairs_wolf_armor".to_string(), entries: vec![836] }, Tag { name: "minecraft:wool_carpets".to_string(), entries: vec![476, 477, 478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491] }, Tag { name: "minecraft:furnace_minecart_fuel".to_string(), entries: vec![843, 844] }, Tag { name: "minecraft:cow_food".to_string(), entries: vec![894] }, Tag { name: "minecraft:bookshelf_books".to_string(), entries: vec![967, 1153, 1176, 1152, 1229] }, Tag { name: "minecraft:enchantable/fishing".to_string(), entries: vec![991] }, Tag { name: "minecraft:decorated_pot_ingredients".to_string(), entries: vec![963, 1352, 1353, 1354, 1355, 1356, 1357, 1358, 1359, 1361, 1363, 1364, 1365, 1366, 1367, 1368, 1369, 1371, 1372, 1373, 1374, 1360, 1362, 1370] }, Tag { name: "minecraft:wooden_doors".to_string(), entries: vec![743, 744, 745, 746, 747, 749, 750, 753, 754, 751, 752, 748] }, Tag { name: "minecraft:enchantable/sword".to_string(), entries: vec![878, 863, 868, 883, 858, 873] }, Tag { name: "minecraft:horse_food".to_string(), entries: vec![894, 1022, 475, 840, 1164, 924, 925] }, Tag { name: "minecraft:meat".to_string(), entries: vec![1048, 1050, 1049, 1051, 1195, 922, 1182, 1194, 921, 1181, 1052] }, Tag { name: "minecraft:warped_stems".to_string(), entries: vec![146, 158, 181, 169] }, Tag { name: "minecraft:emerald_ores".to_string(), entries: vec![74, 75] }, Tag { name: "minecraft:enchantable/fire_aspect".to_string(), entries: vec![878, 863, 868, 883, 858, 873, 1155] }, Tag { name: "minecraft:bamboo_blocks".to_string(), entries: vec![147, 170] }, Tag { name: "minecraft:crimson_stems".to_string(), entries: vec![145, 157, 180, 168] }, Tag { name: "minecraft:wolf_food".to_string(), entries: vec![1048, 1050, 1049, 1051, 1195, 922, 1182, 1194, 921, 1181, 1052, 995, 999, 996, 1000, 997, 998, 1183] }, Tag { name: "minecraft:horse_tempt_items".to_string(), entries: vec![1164, 924, 925] }, Tag { name: "minecraft:ignored_by_piglin_babies".to_string(), entries: vec![955] }, Tag { name: "minecraft:enchantable/bow".to_string(), entries: vec![841] }, Tag { name: "minecraft:swords".to_string(), entries: vec![878, 863, 868, 883, 858, 873] }, Tag { name: "minecraft:enchantable/leg_armor".to_string(), entries: vec![898, 902, 914, 906, 910, 918] }, Tag { name: "minecraft:stone_tool_materials".to_string(), entries: vec![35, 1292, 9] }, Tag { name: "minecraft:terracotta".to_string(), entries: vec![492, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472] }, Tag { name: "minecraft:wart_blocks".to_string(), entries: vec![547, 548] }, Tag { name: "minecraft:dark_oak_logs".to_string(), entries: vec![141, 178, 154, 165] }, Tag { name: "minecraft:enchantable/durability".to_string(), entries: vec![899, 903, 915, 907, 911, 919, 898, 902, 914, 906, 910, 918, 897, 901, 913, 905, 909, 917, 896, 900, 912, 904, 908, 916, 834, 809, 1225, 878, 863, 868, 883, 858, 873, 881, 866, 871, 886, 861, 876, 880, 865, 870, 885, 860, 875, 879, 864, 869, 884, 859, 874, 882, 867, 872, 887, 862, 877, 841, 1254, 1251, 838, 1043, 1332, 991, 806, 807, 1155] }, Tag { name: "minecraft:strider_food".to_string(), entries: vec![250] }, Tag { name: "minecraft:non_flammable_wood".to_string(), entries: vec![146, 158, 181, 169, 145, 157, 180, 168, 46, 47, 281, 282, 740, 741, 342, 343, 774, 775, 794, 795, 423, 424, 724, 725, 753, 754, 936, 937, 949, 948] }, Tag { name: "minecraft:villager_picks_up".to_string(), entries: vec![893, 1160, 1159, 1218, 1215, 1216, 895, 894, 1217] }, Tag { name: "minecraft:coals".to_string(), entries: vec![843, 844] }, Tag { name: "minecraft:hoglin_food".to_string(), entries: vec![249] }, Tag { name: "minecraft:piglin_food".to_string(), entries: vec![921, 922] }, Tag { name: "minecraft:repairs_iron_armor".to_string(), entries: vec![851] }, Tag { name: "minecraft:shulker_boxes".to_string(), entries: vec![552, 568, 564, 565, 562, 560, 566, 556, 561, 558, 555, 554, 559, 563, 567, 553, 557] }, Tag { name: "minecraft:piglin_preferred_weapons".to_string(), entries: vec![1254] }, Tag { name: "minecraft:breaks_decorated_pots".to_string(), entries: vec![878, 863, 868, 883, 858, 873, 881, 866, 871, 886, 861, 876, 880, 865, 870, 885, 860, 875, 879, 864, 869, 884, 859, 874, 882, 867, 872, 887, 862, 877, 1251, 1155] }, Tag { name: "minecraft:anvil".to_string(), entries: vec![449, 450, 451] }, Tag { name: "minecraft:panda_eats_from_ground".to_string(), entries: vec![269, 1023] }, Tag { name: "minecraft:birch_logs".to_string(), entries: vec![136, 173, 150, 161] }, Tag { name: "minecraft:lapis_ores".to_string(), entries: vec![76, 77] }, Tag { name: "minecraft:axes".to_string(), entries: vec![881, 866, 871, 886, 861, 876] }, Tag { name: "minecraft:enchantable/mining".to_string(), entries: vec![881, 866, 871, 886, 861, 876, 880, 865, 870, 885, 860, 875, 879, 864, 869, 884, 859, 874, 882, 867, 872, 887, 862, 877, 1043] }, Tag { name: "minecraft:hoes".to_string(), entries: vec![882, 867, 872, 887, 862, 877] }, Tag { name: "minecraft:repairs_turtle_helmet".to_string(), entries: vec![835] }, Tag { name: "minecraft:llama_food".to_string(), entries: vec![894, 475] }, Tag { name: "minecraft:sniffer_food".to_string(), entries: vec![1215] }, Tag { name: "minecraft:enchantable/head_armor".to_string(), entries: vec![896, 900, 912, 904, 908, 916, 834] }, Tag { name: "minecraft:fences".to_string(), entries: vec![332, 336, 338, 339, 333, 334, 335, 342, 343, 340, 341, 337, 398] }, Tag { name: "minecraft:saplings".to_string(), entries: vec![49, 50, 51, 52, 53, 55, 56, 205, 206, 57, 54] }, Tag { name: "minecraft:parrot_food".to_string(), entries: vec![893, 1047, 1046, 1218, 1215, 1216] }, Tag { name: "minecraft:beds".to_string(), entries: vec![1038, 1039, 1035, 1036, 1033, 1031, 1037, 1027, 1032, 1029, 1026, 1025, 1030, 1034, 1024, 1028] }, Tag { name: "minecraft:rabbit_food".to_string(), entries: vec![1159, 1164, 229] }, Tag { name: "minecraft:iron_ores".to_string(), entries: vec![66, 67] }, Tag { name: "minecraft:enchantable/armor".to_string(), entries: vec![899, 903, 915, 907, 911, 919, 898, 902, 914, 906, 910, 918, 897, 901, 913, 905, 909, 917, 896, 900, 912, 904, 908, 916, 834] }, Tag { name: "minecraft:oak_logs".to_string(), entries: vec![134, 171, 148, 159] }, Tag { name: "minecraft:bundles".to_string(), entries: vec![974, 990, 986, 987, 984, 982, 988, 978, 983, 980, 977, 976, 981, 985, 989, 979, 975] }, Tag { name: "minecraft:pillager_preferred_weapons".to_string(), entries: vec![1254] }, Tag { name: "minecraft:drowned_preferred_weapons".to_string(), entries: vec![1251] }, Tag { name: "minecraft:doors".to_string(), entries: vec![743, 744, 745, 746, 747, 749, 750, 753, 754, 751, 752, 748, 755, 756, 757, 758, 759, 760, 761, 762, 742] }, Tag { name: "minecraft:ocelot_food".to_string(), entries: vec![995, 996] }, Tag { name: "minecraft:banners".to_string(), entries: vec![1196, 1197, 1198, 1199, 1200, 1201, 1202, 1203, 1204, 1205, 1206, 1207, 1208, 1209, 1210, 1211] }, Tag { name: "minecraft:noteblock_top_instruments".to_string(), entries: vec![1168, 1165, 1169, 1170, 1166, 1171, 1167] }, Tag { name: "minecraft:eggs".to_string(), entries: vec![969, 970, 971] }, Tag { name: "minecraft:brewing_fuel".to_string(), entries: vec![1062] }, Tag { name: "minecraft:smelts_to_glass".to_string(), entries: vec![59, 62] }, Tag { name: "minecraft:stone_crafting_materials".to_string(), entries: vec![35, 1292, 9] }, Tag { name: "minecraft:wooden_fences".to_string(), entries: vec![332, 336, 338, 339, 333, 334, 335, 342, 343, 340, 341, 337] }, Tag { name: "minecraft:piglin_repellents".to_string(), entries: vec![353, 1279, 1283] }, Tag { name: "minecraft:axolotl_food".to_string(), entries: vec![960] }, Tag { name: "minecraft:villager_plantable_seeds".to_string(), entries: vec![893, 1160, 1159, 1218, 1215, 1216] }, Tag { name: "minecraft:leg_armor".to_string(), entries: vec![898, 902, 914, 906, 910, 918] }, Tag { name: "minecraft:wither_skeleton_disliked_weapons".to_string(), entries: vec![841, 1254] }, Tag { name: "minecraft:enchantable/foot_armor".to_string(), entries: vec![899, 903, 915, 907, 911, 919] }, Tag { name: "minecraft:panda_food".to_string(), entries: vec![269] }, Tag { name: "minecraft:dampens_vibrations".to_string(), entries: vec![213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 476, 477, 478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491] }, Tag { name: "minecraft:mangrove_logs".to_string(), entries: vec![142, 179, 156, 167] }, Tag { name: "minecraft:jungle_logs".to_string(), entries: vec![137, 174, 151, 162] }, Tag { name: "minecraft:lectern_books".to_string(), entries: vec![1153, 1152] }, Tag { name: "minecraft:enchantable/chest_armor".to_string(), entries: vec![897, 901, 913, 905, 909, 917] }, Tag { name: "minecraft:llama_tempt_items".to_string(), entries: vec![475] }, Tag { name: "minecraft:turtle_food".to_string(), entries: vec![211] }, Tag { name: "minecraft:spruce_logs".to_string(), entries: vec![135, 172, 149, 160] }, Tag { name: "minecraft:signs".to_string(), entries: vec![926, 927, 928, 930, 929, 932, 933, 936, 937, 934, 935, 931] }, Tag { name: "minecraft:wooden_stairs".to_string(), entries: vec![412, 413, 414, 415, 416, 418, 419, 423, 424, 420, 421, 417] }, Tag { name: "minecraft:enchantable/weapon".to_string(), entries: vec![878, 863, 868, 883, 858, 873, 881, 866, 871, 886, 861, 876, 1155] }, Tag { name: "minecraft:wooden_buttons".to_string(), entries: vec![714, 715, 716, 717, 718, 720, 721, 724, 725, 722, 723, 719] }, Tag { name: "minecraft:netherite_tool_materials".to_string(), entries: vec![856] }, Tag { name: "minecraft:book_cloning_target".to_string(), entries: vec![1152] }, Tag { name: "minecraft:fishes".to_string(), entries: vec![995, 999, 996, 1000, 998, 997] }, Tag { name: "minecraft:stone_bricks".to_string(), entries: vec![362, 363, 364, 365] }, Tag { name: "minecraft:shovels".to_string(), entries: vec![879, 864, 869, 884, 859, 874] }, Tag { name: "minecraft:chest_boats".to_string(), entries: vec![811, 813, 815, 817, 819, 823, 825, 827, 829, 821] }, Tag { name: "minecraft:creeper_igniters".to_string(), entries: vec![838, 1150] }, Tag { name: "minecraft:enchantable/equippable".to_string(), entries: vec![899, 903, 915, 907, 911, 919, 898, 902, 914, 906, 910, 918, 897, 901, 913, 905, 909, 917, 896, 900, 912, 904, 908, 916, 834, 809, 1167, 1169, 1168, 1165, 1166, 1170, 1171, 345] }, Tag { name: "minecraft:enchantable/trident".to_string(), entries: vec![1251] }, Tag { name: "minecraft:enchantable/mining_loot".to_string(), entries: vec![881, 866, 871, 886, 861, 876, 880, 865, 870, 885, 860, 875, 879, 864, 869, 884, 859, 874, 882, 867, 872, 887, 862, 877] }, Tag { name: "minecraft:enchantable/sharp_weapon".to_string(), entries: vec![878, 863, 868, 883, 858, 873, 881, 866, 871, 886, 861, 876] }, Tag { name: "minecraft:slabs".to_string(), entries: vec![270, 271, 272, 273, 274, 276, 277, 281, 282, 278, 279, 275, 280, 283, 284, 290, 285, 296, 293, 294, 289, 288, 292, 287, 297, 298, 299, 669, 670, 671, 672, 673, 674, 675, 676, 677, 678, 679, 680, 681, 286, 295, 1293, 1301, 1297, 682, 683, 685, 684, 132, 131, 130, 113, 112, 111, 110, 133, 291, 13, 18, 22, 387] }, Tag { name: "minecraft:repairs_netherite_armor".to_string(), entries: vec![856] }, Tag { name: "minecraft:enchantable/vanishing".to_string(), entries: vec![899, 903, 915, 907, 911, 919, 898, 902, 914, 906, 910, 918, 897, 901, 913, 905, 909, 917, 896, 900, 912, 904, 908, 916, 834, 809, 1225, 878, 863, 868, 883, 858, 873, 881, 866, 871, 886, 861, 876, 880, 865, 870, 885, 860, 875, 879, 864, 869, 884, 859, 874, 882, 867, 872, 887, 862, 877, 841, 1254, 1251, 838, 1043, 1332, 991, 806, 807, 1155, 972, 345, 1167, 1169, 1168, 1165, 1166, 1170, 1171] }, Tag { name: "minecraft:hanging_signs".to_string(), entries: vec![938, 939, 940, 942, 943, 941, 944, 945, 948, 949, 946, 947] }, Tag { name: "minecraft:wooden_tool_materials".to_string(), entries: vec![36, 37, 38, 39, 40, 42, 43, 46, 47, 44, 45, 41] }, Tag { name: "minecraft:trapdoors".to_string(), entries: vec![768, 766, 770, 771, 767, 764, 765, 774, 775, 772, 773, 769, 763, 776, 777, 778, 779, 780, 781, 782, 783] }, Tag { name: "minecraft:redstone_ores".to_string(), entries: vec![72, 73] }, Tag { name: "minecraft:duplicates_allays".to_string(), entries: vec![849] }, Tag { name: "minecraft:cherry_logs".to_string(), entries: vec![139, 176, 153, 164] }, Tag { name: "minecraft:flowers".to_string(), entries: vec![229, 230, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 231, 495, 496, 498, 497, 245, 192, 206, 57, 187, 258, 259, 313, 246, 329] }, Tag { name: "minecraft:buttons".to_string(), entries: vec![714, 715, 716, 717, 718, 720, 721, 724, 725, 722, 723, 719, 712, 713] }, Tag { name: "minecraft:dyeable".to_string(), entries: vec![896, 897, 898, 899, 1190, 837] }, Tag { name: "minecraft:planks".to_string(), entries: vec![36, 37, 38, 39, 40, 42, 43, 46, 47, 44, 45, 41] }, Tag { name: "minecraft:stone_buttons".to_string(), entries: vec![712, 713] }, Tag { name: "minecraft:boats".to_string(), entries: vec![810, 812, 814, 816, 818, 822, 824, 826, 828, 820, 811, 813, 815, 817, 819, 823, 825, 827, 829, 821] }, Tag { name: "minecraft:fox_food".to_string(), entries: vec![1280, 1281] }, Tag { name: "minecraft:enchantable/crossbow".to_string(), entries: vec![1254] }, Tag { name: "minecraft:chest_armor".to_string(), entries: vec![897, 901, 913, 905, 909, 917] }, Tag { name: "minecraft:skeleton_preferred_weapons".to_string(), entries: vec![841] }, Tag { name: "minecraft:frog_food".to_string(), entries: vec![968] }, Tag { name: "minecraft:rails".to_string(), entries: vec![798, 796, 797, 799] }, Tag { name: "minecraft:diamond_ores".to_string(), entries: vec![78, 79] }, Tag { name: "minecraft:strider_tempt_items".to_string(), entries: vec![250, 807] }, Tag { name: "minecraft:leaves".to_string(), entries: vec![185, 182, 183, 189, 188, 186, 184, 191, 192, 190, 187] }, Tag { name: "minecraft:walls".to_string(), entries: vec![427, 428, 429, 430, 431, 432, 433, 434, 436, 437, 438, 439, 440, 441, 442, 444, 443, 445, 446, 448, 447, 435, 15, 20, 24, 388] }, Tag { name: "minecraft:gaze_disguise_equipment".to_string(), entries: vec![345] }, Tag { name: "minecraft:map_invisibility_equipment".to_string(), entries: vec![345] }, Tag { name: "minecraft:fence_gates".to_string(), entries: vec![788, 786, 790, 791, 787, 784, 785, 794, 795, 792, 793, 789] }, Tag { name: "minecraft:armadillo_food".to_string(), entries: vec![1060] }, Tag { name: "minecraft:wooden_pressure_plates".to_string(), entries: vec![730, 731, 732, 733, 734, 736, 737, 740, 741, 738, 739, 735] }, Tag { name: "minecraft:repairs_gold_armor".to_string(), entries: vec![855] }, Tag { name: "minecraft:goat_food".to_string(), entries: vec![894] }, Tag { name: "minecraft:acacia_logs".to_string(), entries: vec![138, 175, 152, 163] }, Tag { name: "minecraft:piglin_loved".to_string(), entries: vec![70, 80, 71, 92, 1295, 728, 855, 1277, 992, 1164, 1067, 924, 925, 912, 913, 914, 915, 1188, 868, 870, 869, 871, 872, 854, 86] }, Tag { name: "minecraft:cat_food".to_string(), entries: vec![995, 996] }, Tag { name: "minecraft:sheep_food".to_string(), entries: vec![894] }, Tag { name: "minecraft:candles".to_string(), entries: vec![1305, 1306, 1307, 1308, 1309, 1310, 1311, 1312, 1313, 1314, 1315, 1316, 1317, 1318, 1319, 1320, 1321] }, Tag { name: "minecraft:bee_food".to_string(), entries: vec![229, 230, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 495, 496, 498, 497, 245, 192, 206, 57, 187, 258, 259, 313, 246, 329] }, Tag { name: "minecraft:copper_ores".to_string(), entries: vec![68, 69] }, Tag { name: "minecraft:sand".to_string(), entries: vec![59, 62, 60] }, Tag { name: "minecraft:gold_ores".to_string(), entries: vec![70, 80, 71] }, Tag { name: "minecraft:freeze_immune_wearables".to_string(), entries: vec![899, 898, 897, 896, 1190] }, Tag { name: "minecraft:logs_that_burn".to_string(), entries: vec![141, 178, 154, 165, 140, 177, 155, 166, 134, 171, 148, 159, 138, 175, 152, 163, 136, 173, 150, 161, 137, 174, 151, 162, 135, 172, 149, 160, 142, 179, 156, 167, 139, 176, 153, 164] }, Tag { name: "minecraft:completes_find_tree_tutorial".to_string(), entries: vec![141, 178, 154, 165, 140, 177, 155, 166, 134, 171, 148, 159, 138, 175, 152, 163, 136, 173, 150, 161, 137, 174, 151, 162, 135, 172, 149, 160, 142, 179, 156, 167, 139, 176, 153, 164, 145, 157, 180, 168, 146, 158, 181, 169, 185, 182, 183, 189, 188, 186, 184, 191, 192, 190, 187, 547, 548] }, Tag { name: "minecraft:diamond_tool_materials".to_string(), entries: vec![845] }, Tag { name: "minecraft:dirt".to_string(), entries: vec![28, 27, 30, 29, 393, 31, 262, 265, 32, 144] }, Tag { name: "minecraft:repairs_chain_armor".to_string(), entries: vec![851] }, Tag { name: "minecraft:pickaxes".to_string(), entries: vec![880, 865, 870, 885, 860, 875] }, Tag { name: "minecraft:decorated_pot_sherds".to_string(), entries: vec![1352, 1353, 1354, 1355, 1356, 1357, 1358, 1359, 1361, 1363, 1364, 1365, 1366, 1367, 1368, 1369, 1371, 1372, 1373, 1374, 1360, 1362, 1370] }, Tag { name: "minecraft:foot_armor".to_string(), entries: vec![899, 903, 915, 907, 911, 919] }, Tag { name: "minecraft:cluster_max_harvestables".to_string(), entries: vec![880, 870, 875, 885, 865, 860] }])
        ],
    }.try_into()?)?;

    lib::utils::send_packet(stream, lib::packets::clientbound::configuration::FinishConfiguration::PACKET_ID, lib::packets::clientbound::configuration::FinishConfiguration {

    }.try_into()?)?;

    return Ok(None);
  }

  pub fn acknowledge_finish_configuration(stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>, game: &mut Game, connection_streams: &mut HashMap<SocketAddr, TcpStream>) -> Result<Option<Action>, Box<dyn Error>> {
    connections.entry(stream.peer_addr()?).and_modify(|x| x.state = ConnectionState::Play);

    let connection_player = connections.get(&stream.peer_addr()?).unwrap();
    let mut new_player = Player::new(connection_player.player_name.clone().unwrap_or_default(), connection_player.player_uuid.unwrap_or_default(), stream.peer_addr()?, game, stream.try_clone()?);

    lib::utils::send_packet(stream, lib::packets::clientbound::play::Login::PACKET_ID, lib::packets::clientbound::play::Login {
      entity_id: new_player.entity_id,
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
    }.try_into()?)?;

    lib::utils::send_packet(stream, lib::packets::clientbound::play::SynchronizePlayerPosition::PACKET_ID, lib::packets::clientbound::play::SynchronizePlayerPosition {
      teleport_id: new_player.current_teleport_id,
      x: new_player.get_x(),
      y: new_player.get_y(),
      z: new_player.get_z(),
      velocity_x: 0.0,
      velocity_y: 0.0,
      velocity_z: 0.0,
      yaw: new_player.get_yaw(),
      pitch: new_player.get_pitch(),
      flags: 0,
    }.try_into()?)?;

    lib::utils::send_packet(stream, lib::packets::clientbound::play::GameEvent::PACKET_ID, lib::packets::clientbound::play::GameEvent {
      event: 13,
      value: 0.0,
    }.try_into()?)?;

    lib::utils::send_packet(stream, lib::packets::clientbound::play::SetContainerContent::PACKET_ID, lib::packets::clientbound::play::SetContainerContent {
	   	window_id: 0,
	    state_id: 1,
	    slot_data: new_player.get_inventory().clone(),
	    carried_item: None,
    }.try_into()?)?;

    let current_chunk_coords = new_player.get_position().convert_to_coordinates_of_chunk();

    for x in current_chunk_coords.x-lib::SPAWN_CHUNK_RADIUS as i32..=current_chunk_coords.x+lib::SPAWN_CHUNK_RADIUS as i32 {
      for z in current_chunk_coords.z-lib::SPAWN_CHUNK_RADIUS as i32..=current_chunk_coords.z+lib::SPAWN_CHUNK_RADIUS as i32 {
     		new_player.send_chunk(&mut game.world, x, z)?;
      }
    }

    lib::utils::send_packet(stream, lib::packets::clientbound::play::SetHeldItem::PACKET_ID, lib::packets::clientbound::play::SetHeldItem {
   		slot: new_player.get_selected_slot()
    }.try_into()?)?;

    let new_player_uuid = new_player.uuid;
    let new_player_entity_id = new_player.entity_id;
    let new_player_x = new_player.get_x();
    let new_player_y = new_player.get_y();
    let new_player_z = new_player.get_z();
    let new_player_inventory = new_player.get_inventory().clone();
    let new_player_selected_slot = new_player.get_selected_slot();
    game.players.push(new_player);

    connection_streams.iter()
      .filter(|x| connections.get(x.0).is_some_and(|x| x.state == ConnectionState::Play))
      .for_each(|x| {
     	//proper logic for updating players instead of removing and readding all https://git.thetxt.io/thetxt/oxide/issues/21
        let _ = lib::utils::send_packet(x.1, lib::packets::clientbound::play::PlayerInfoRemove::PACKET_ID, lib::packets::clientbound::play::PlayerInfoRemove {
          uuids: game.players.iter().map(|x| x.uuid).collect(),
        }.try_into().unwrap());

        let _ = lib::utils::send_packet(x.1, lib::packets::clientbound::play::PlayerInfoUpdate::PACKET_ID, lib::packets::clientbound::play::PlayerInfoUpdate {
          actions: 255,
          players: game.players.iter().map(|y| (y.uuid, vec![
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
      });

    //Spawn other already connected player entities for newly joined player
    for player in &game.players {
      if player.uuid == new_player_uuid {
        continue;
      }

      lib::utils::send_packet(stream, lib::packets::clientbound::play::SpawnEntity::PACKET_ID, lib::packets::clientbound::play::SpawnEntity {
        entity_id: player.entity_id,
        entity_uuid: player.uuid,
        entity_type: 149, //Player
        x: player.get_x(),
        y: player.get_y(),
        z: player.get_z(),
        pitch: 0,
        yaw: 0,
        head_yaw: 0,
        data: 0,
        velocity_x: 0,
        velocity_y: 0,
        velocity_z: 0,
      }.try_into()?)?;

      lib::utils::send_packet(stream, lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID, lib::packets::clientbound::play::SetEntityMetadata {
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
      }.try_into()?)?;

 	   	lib::utils::send_packet(stream, lib::packets::clientbound::play::SetEquipment::PACKET_ID, lib::packets::clientbound::play::SetEquipment {
	 			entity_id: player.entity_id,
	  		equipment: vec![
					(0, player.get_inventory()[(player.get_selected_slot() + 36) as usize].clone()),
					(1, player.get_inventory()[45].clone()),
					(2, player.get_inventory()[8].clone()),
					(3, player.get_inventory()[7].clone()),
					(4, player.get_inventory()[6].clone()),
					(5, player.get_inventory()[5].clone()),
				],
	   	}.try_into()?)?;

      let yaw: u8 = if player.get_yaw() < 0.0 {
     		(((player.get_yaw() / 90.0) * 64.0) + 256.0) as u8
      } else {
      	((player.get_yaw() / 90.0) * 64.0) as u8
      };
      let pitch: u8 = if player.get_pitch() < 0.0 {
     		(((player.get_pitch() / 90.0) * 64.0) + 256.0) as u8
      } else {
      	((player.get_pitch() / 90.0) * 64.0) as u8
      };
      lib::utils::send_packet(stream, lib::packets::clientbound::play::UpdateEntityRotation::PACKET_ID, lib::packets::clientbound::play::UpdateEntityRotation {
        entity_id: player.entity_id,
        on_ground: player.get_y() == -48.0, //add proper check https://git.thetxt.io/thetxt/oxide/issues/22
        yaw,
        pitch,
      }.try_into()?)?;
      lib::utils::send_packet(stream, lib::packets::clientbound::play::SetHeadRotation::PACKET_ID, lib::packets::clientbound::play::SetHeadRotation {
	        entity_id: player.entity_id,
					head_yaw: yaw,
	      }.try_into()?)?;
    }

    //Spawn player entity for other players that are already connected
    for player in &game.players {
      if stream.peer_addr().is_ok() && player.peer_socket_address == stream.peer_addr()? {
        continue;
      }
      let player_stream = connection_streams.get(&player.peer_socket_address).unwrap();

      lib::utils::send_packet(player_stream, lib::packets::clientbound::play::SpawnEntity::PACKET_ID, lib::packets::clientbound::play::SpawnEntity {
        entity_id: new_player_entity_id,
        entity_uuid: new_player_uuid,
        entity_type: 149, //Player
        x: new_player_x,
        y: new_player_y,
        z: new_player_z,
        pitch: 0,
        yaw: 0,
        head_yaw: 0,
        data: 0,
        velocity_x: 0,
        velocity_y: 0,
        velocity_z: 0,
      }.try_into()?)?;

      lib::utils::send_packet(player_stream, lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID, lib::packets::clientbound::play::SetEntityMetadata {
        entity_id: new_player_entity_id,
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
      }.try_into()?)?;

	   	lib::utils::send_packet(player_stream, lib::packets::clientbound::play::SetEquipment::PACKET_ID, lib::packets::clientbound::play::SetEquipment {
	 			entity_id: new_player_entity_id,
	  		equipment: vec![
					(0, new_player_inventory[(new_player_selected_slot + 36) as usize].clone()),
					(1, new_player_inventory[45].clone()),
					(2, new_player_inventory[8].clone()),
					(3, new_player_inventory[7].clone()),
					(4, new_player_inventory[6].clone()),
					(5, new_player_inventory[5].clone()),
				],
	   	}.try_into()?)?;

	    let yaw: u8 = if player.get_yaw() < 0.0 {
    		(((player.get_yaw() / 90.0) * 64.0) + 256.0) as u8
      } else {
       	((player.get_yaw() / 90.0) * 64.0) as u8
      };
	    let pitch: u8 = if player.get_pitch() < 0.0 {
    		(((player.get_pitch() / 90.0) * 64.0) + 256.0) as u8
      } else {
       	((player.get_pitch() / 90.0) * 64.0) as u8
      };
			lib::utils::send_packet(player_stream, lib::packets::clientbound::play::UpdateEntityRotation::PACKET_ID, lib::packets::clientbound::play::UpdateEntityRotation {
        entity_id: player.entity_id,
        on_ground: player.get_y() == -48.0, //add proper check https://git.thetxt.io/thetxt/oxide/issues/22
        yaw,
        pitch,
      }.try_into()?)?;
      lib::utils::send_packet(player_stream, lib::packets::clientbound::play::SetHeadRotation::PACKET_ID, lib::packets::clientbound::play::SetHeadRotation {
        entity_id: player.entity_id,
  			head_yaw: yaw,
      }.try_into()?)?;
    }

    lib::utils::send_packet(stream, lib::packets::clientbound::play::Commands::PACKET_ID, lib::packets::clientbound::play::Commands {
      nodes: crate::command::get_command_packet_data(game),
      root_index: 0,
    }.try_into()?)?;

    lib::utils::send_packet(stream, lib::packets::clientbound::play::SetTabListHeaderAndFooter::PACKET_ID, lib::packets::clientbound::play::SetTabListHeaderAndFooter {
  		header: NbtTag::Root(vec![NbtTag::String("text".to_string(), "".to_string())]),
  		footer: NbtTag::Root(vec![
    		NbtTag::String("type".to_string(), "text".to_string()),
    		NbtTag::String("text".to_string(), " powered by Oxide ".to_string()),
    		NbtTag::String("color".to_string(), "gray".to_string()),
    		NbtTag::Byte("italic".to_string(), 1),
    	]),
    }.try_into()?)?;

    //get rid of this once we have a real game loop https://git.thetxt.io/thetxt/oxide/issues/23
    let stream_clone = stream.try_clone()?;
    std::thread::spawn(move || {
      loop {
        let useless_buf_no_one_crates_about = &mut [0; 1];
        if stream_clone.peek(useless_buf_no_one_crates_about).is_err() {
          return;
        }
        if lib::utils::send_packet(&stream_clone, lib::packets::clientbound::play::ClientboundKeepAlive::PACKET_ID, lib::packets::clientbound::play::ClientboundKeepAlive {
          keep_alive_id: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
        }.try_into().unwrap()).is_err() {
          return;
        };

        std::thread::sleep(std::time::Duration::from_secs(5));
      }
    });

    return Ok(None);
  }
}

pub mod play {
  use lib::{nbt::NbtTag, packets::Packet, utils::send_packet};
  use super::*;

  pub fn set_player_position(data: &mut [u8], game: &mut Game, stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>) -> Result<Option<Action>, Box<dyn Error>> {
    let parsed_packet = lib::packets::serverbound::play::SetPlayerPosition::try_from(data.to_vec())?;

    let player_index = game.players.iter().enumerate().find_map(|x| {
      if x.1.uuid == connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.unwrap_or_default() {
        Some(x.0)}
        else {None}
      });
    let player = game.players.get_mut(player_index.unwrap()).unwrap();

    let old_x = player.get_x();
    let old_y = player.get_y();
    let old_z = player.get_z();

    player.new_position(parsed_packet.x, parsed_packet.y, parsed_packet.z, &mut game.world)?;

    let default_connection = Connection::default();
    for other_stream in connection_streams {
      if *other_stream.0 != stream.peer_addr()? && connections.get(other_stream.0).unwrap_or(&default_connection).state == ConnectionState::Play {
        lib::utils::send_packet(other_stream.1, lib::packets::clientbound::play::UpdateEntityPosition::PACKET_ID, lib::packets::clientbound::play::UpdateEntityPosition {
          entity_id: player.entity_id,
          delta_x: ((player.get_x() * 4096.0) - (old_x * 4096.0)) as i16,
          delta_y: ((player.get_y() * 4096.0) - (old_y * 4096.0)) as i16,
          delta_z: ((player.get_z() * 4096.0) - (old_z * 4096.0)) as i16,
          on_ground: player.get_y() == -48.0, //add proper check https://git.thetxt.io/thetxt/oxide/issues/22
        }.try_into()?)?;
      }
    }

    return Ok(None);
  }

  pub fn set_player_position_and_rotation(data: &mut [u8], game: &mut Game, stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>) -> Result<Option<Action>, Box<dyn Error>> {
    let parsed_packet = lib::packets::serverbound::play::SetPlayerPositionAndRotation::try_from(data.to_vec())?;

    let player_index = game.players.iter().enumerate().find_map(|x| {
      if x.1.uuid == connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.unwrap_or_default() {
        Some(x.0)}
        else {None}
      });
    let player = game.players.get_mut(player_index.unwrap()).unwrap();

    let old_x = player.get_x();
    let old_y = player.get_y();
    let old_z = player.get_z();

    player.new_position_and_rotation(parsed_packet.x, parsed_packet.y, parsed_packet.z, parsed_packet.yaw % 360.0, parsed_packet.pitch, &mut game.world)?;

    let pitch: u8 = if parsed_packet.pitch < 0.0 {
   		(((parsed_packet.pitch / 90.0) * 64.0) + 256.0) as u8
    } else {
    	((parsed_packet.pitch / 90.0) * 64.0) as u8
    };

    let yaw: u8 = if player.get_yaw() < 0.0 {
   		(((player.get_yaw() / 90.0) * 64.0) + 256.0) as u8
    } else {
    	((player.get_yaw() / 90.0) * 64.0) as u8
    };

    let default_connection = Connection::default();
    for other_stream in connection_streams {
      if *other_stream.0 != stream.peer_addr()? && connections.get(other_stream.0).unwrap_or(&default_connection).state == ConnectionState::Play {
	      lib::utils::send_packet(other_stream.1, lib::packets::clientbound::play::UpdateEntityPositionAndRotation::PACKET_ID, lib::packets::clientbound::play::UpdateEntityPositionAndRotation {
	        entity_id: player.entity_id,
	        delta_x: ((player.get_x() * 4096.0) - (old_x * 4096.0)) as i16,
	        delta_y: ((player.get_y() * 4096.0) - (old_y * 4096.0)) as i16,
	        delta_z: ((player.get_z() * 4096.0) - (old_z * 4096.0)) as i16,
	        on_ground: player.get_y() == -48.0, //add proper check https://git.thetxt.io/thetxt/oxide/issues/22
	        yaw,
	        pitch,
	      }.try_into()?)?;

	      lib::utils::send_packet(other_stream.1, lib::packets::clientbound::play::SetHeadRotation::PACKET_ID, lib::packets::clientbound::play::SetHeadRotation {
	        entity_id: player.entity_id,
					head_yaw: yaw,
	      }.try_into()?)?;
      }
    }

    return Ok(None);
  }

  pub fn set_player_rotation(data: &mut [u8], game: &mut Game, stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>) -> Result<Option<Action>, Box<dyn Error>> {
    let parsed_packet = lib::packets::serverbound::play::SetPlayerRotation::try_from(data.to_vec())?;

    let player_index = game.players.iter().enumerate().find_map(|x| {
      if x.1.uuid == connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.unwrap_or_default() {
        Some(x.0)}
        else {None}
      });
    let player = game.players.get_mut(player_index.unwrap()).unwrap();

    player.new_rotation(parsed_packet.yaw % 360.0, parsed_packet.pitch);

    let pitch: u8 = if parsed_packet.pitch < 0.0 {
   		(((parsed_packet.pitch / 90.0) * 64.0) + 256.0) as u8
    } else {
    	((parsed_packet.pitch / 90.0) * 64.0) as u8
    };

    let yaw: u8 = if player.get_yaw() < 0.0 {
   		(((player.get_yaw() / 90.0) * 64.0) + 256.0) as u8
    } else {
    	((player.get_yaw() / 90.0) * 64.0) as u8
    };

    let default_connection = Connection::default();
    for other_stream in connection_streams {
      if *other_stream.0 != stream.peer_addr()? && connections.get(other_stream.0).unwrap_or(&default_connection).state == ConnectionState::Play {
      	lib::utils::send_packet(other_stream.1, lib::packets::clientbound::play::UpdateEntityRotation::PACKET_ID, lib::packets::clientbound::play::UpdateEntityRotation {
	        entity_id: player.entity_id,
	        on_ground: player.get_y() == -48.0, //add proper check https://git.thetxt.io/thetxt/oxide/issues/22
	        yaw,
	        pitch,
	      }.try_into()?)?;

	      lib::utils::send_packet(other_stream.1, lib::packets::clientbound::play::SetHeadRotation::PACKET_ID, lib::packets::clientbound::play::SetHeadRotation {
	        entity_id: player.entity_id,
					head_yaw: yaw,
	      }.try_into()?)?;
      }
    }

    return Ok(None);
  }

  pub fn confirm_teleportation(data: &mut [u8], game: &mut Game, stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>) -> Result<Option<Action>, Box<dyn Error>> {
	  let player_index = game.players.iter().enumerate().find_map(|x| {
	    if x.1.uuid == connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.unwrap_or_default() {
	      Some(x.0)}
	      else {None}
	    });
	  let player = game.players.get_mut(player_index.unwrap()).unwrap();
    let parsed_packet = lib::packets::serverbound::play::ConfirmTeleportation::try_from(data.to_vec())?;
    if player.current_teleport_id == parsed_packet.teleport_id {
   		player.waiting_for_confirm_teleportation = false;
    }

    return Ok(None);
  }

  pub fn set_creative_mode_slot(data: &mut [u8], stream: &mut TcpStream, game: &mut Game, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>) -> Result<Option<Action>, Box<dyn Error>> {
    let parsed_packet = lib::packets::serverbound::play::SetCreativeModeSlot::try_from(data.to_vec())?;
    let player_index = game.players.iter().enumerate().find_map(|x| {
      if x.1.uuid == connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.unwrap_or_default() {
        Some(x.0)}
        else {None}
      });
    let Some(player) = game.players.get_mut(player_index.unwrap()) else {
      println!("got set_creative_mode_slot packet from invalid player");
      return Ok(None);
    };

    player.set_inventory_slot(parsed_packet.slot as u8, parsed_packet.item, connections, connection_streams);

    return Ok(None);
  }

  pub fn set_held_item(data: &mut [u8], stream: &mut TcpStream, game: &mut Game, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>) -> Result<Option<Action>, Box<dyn Error>> {
    let parsed_packet = lib::packets::serverbound::play::SetHandItem::try_from(data.to_vec())?;
    let player_index = game.players.iter().enumerate().find_map(|x| {
      if x.1.uuid == connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.unwrap_or_default() {
        Some(x.0)}
        else {None}
      });
    let Some(player) = game.players.get_mut(player_index.unwrap()) else {
      println!("got set_creative_mode_slot packet from invalid player");
      return Ok(None);
    };

    player.set_selected_slot(parsed_packet.slot as u8, connections, connection_streams);

    return Ok(None);
  }

  pub fn player_action(data: &mut [u8], stream: &mut TcpStream, connection_streams: &mut HashMap<SocketAddr, TcpStream>, game: &mut Game, connections: &mut HashMap<SocketAddr, Connection>) -> Result<Option<Action>, Box<dyn Error>> {
    let parsed_packet = lib::packets::serverbound::play::PlayerAction::try_from(data.to_vec())?;

    let old_block = game.world.dimensions.get("minecraft:overworld").unwrap().get_block(parsed_packet.location)?;
    let res = game.world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(parsed_packet.location, 0)?;
    if res.is_some() && matches!(res.unwrap(), BlockOverwriteOutcome::DestroyBlockentity) {
      game.world.dimensions.get_mut("minecraft:overworld").unwrap().get_chunk_from_position_mut(parsed_packet.location).unwrap().block_entities.retain(|x| x.position != parsed_packet.location);
      game.players.iter_mut()
        .filter(|x| x.opened_container_at.is_some_and(|y| y == parsed_packet.location))
        .for_each(|x| x.close_inventory().unwrap());
    }

    for stream in connection_streams.iter() {
      send_packet(stream.1, lib::packets::clientbound::play::BlockUpdate::PACKET_ID, lib::packets::clientbound::play::BlockUpdate {
        location: parsed_packet.location,
        block_id: 0,
      }.try_into()?)?;
    }

    send_packet(stream, lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID, lib::packets::clientbound::play::AcknowledgeBlockChange {
      sequence_id: parsed_packet.sequence,
    }.try_into()?)?;

   	connection_streams.iter()
     	.filter(|x| connections.get(x.0).unwrap().state == ConnectionState::Play)
     	.filter(|x| *x.0 != stream.peer_addr().unwrap())
     	.for_each(|x| {
	      send_packet(x.1, lib::packets::clientbound::play::WorldEvent::PACKET_ID, lib::packets::clientbound::play::WorldEvent {
	      	event: 2001,
	      	location: parsed_packet.location,
	       	data: old_block as i32,
	      }.try_into().unwrap()).unwrap();
      });

    return Ok(None);
  }

  pub fn use_item_on(data: &mut [u8], stream: &mut TcpStream, connection_streams: &mut HashMap<SocketAddr, TcpStream>, game: &mut Game, connections: &mut HashMap<SocketAddr, Connection>) -> Result<Option<Action>, Box<dyn Error>> {
    let parsed_packet = lib::packets::serverbound::play::UseItemOn::try_from(data.to_vec())?;

    let mut new_block_location = parsed_packet.location;
    match parsed_packet.face {
      0 => new_block_location.y -= 1,
      1 => new_block_location.y += 1,
      2 => new_block_location.z -= 1,
      3 => new_block_location.z += 1,
      4 => new_block_location.x -= 1,
      _ => new_block_location.x += 1,
    }

    let player_index = game.players.iter().enumerate().find_map(|x| {
      if x.1.uuid == connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.unwrap_or_default() {
        Some(x.0)}
        else {None}
      });
    let Some(player) = game.players.get_mut(player_index.unwrap()) else {
      println!("got use_item_on packet from invalid player");
      return Ok(None);
    };

    let dimension = game.world.dimensions.get("minecraft:overworld").unwrap();
    let block_id_at_location = dimension.get_block(parsed_packet.location).unwrap_or_default();
    let block_states = data::blocks::get_blocks();
    let block_type_at_location = data::blocks::get_type_from_block_state_id(block_id_at_location, &block_states);

    let blocks_to_place: Vec<(u16, Position)> = if block_type_at_location.has_right_click_behavior() {
      //Don't place block, because player right clicked something that does something when right clicked
      match lib::block::interact_with_block_at(parsed_packet.location, block_id_at_location, parsed_packet.face) {
        lib::block::BlockInteractionResult::OverwriteBlocks(blocks) => blocks,
        lib::block::BlockInteractionResult::OpenInventory(window_type) => {
          let Some(block_entity) = dimension.get_chunk_from_position(parsed_packet.location).unwrap().try_get_block_entity(parsed_packet.location) else {
            return Ok(None);
          };
          player.open_inventory(window_type, block_entity);
          Vec::new()
        },
        lib::block::BlockInteractionResult::Nothing => Vec::new(),
      }
    } else {
      //Let's go - we can place a block
      let used_item_id = player.get_held_item(true).unwrap_or(&Slot { item_count: 0, item_id: 0, components_to_add: Vec::new(), components_to_remove: Vec::new() }).item_id;
      let used_item_name = data::items::get_item_name_by_id(used_item_id);

      lib::block::get_block_state_id(parsed_packet.face, player.get_looking_cardinal_direction(), game.world.dimensions.get_mut("minecraft:overworld").unwrap(), new_block_location, used_item_name, parsed_packet.cursor_position_x, parsed_packet.cursor_position_y, parsed_packet.cursor_position_z)
    };

    for block_to_place in &blocks_to_place {
      match game.world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(block_to_place.1, block_to_place.0) {
        Ok(res) => {
          if res.is_some() && matches!(res.unwrap(), BlockOverwriteOutcome::DestroyBlockentity) {
            game.world.dimensions.get_mut("minecraft:overworld").unwrap().get_chunk_from_position_mut(parsed_packet.location).unwrap().block_entities.retain(|x| x.position != parsed_packet.location);
            game.players.iter_mut()
              .filter(|x| x.opened_container_at.is_some_and(|y| y == parsed_packet.location))
              .for_each(|x| x.close_inventory().unwrap());
          }
        },
        Err(err) => {
          println!("couldn't place block because {err}");
          return Ok(None);
        },
      };
    }

    for stream in connection_streams {
      for block_to_place in &blocks_to_place {
        send_packet(stream.1, lib::packets::clientbound::play::BlockUpdate::PACKET_ID, lib::packets::clientbound::play::BlockUpdate {
          location: block_to_place.1,
          block_id: block_to_place.0 as i32,
        }.try_into()?)?;
      }
    }

    send_packet(stream, lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID, lib::packets::clientbound::play::AcknowledgeBlockChange {
      sequence_id: parsed_packet.sequence,
    }.try_into()?)?;

    return Ok(None);
  }

  pub fn chat_message(data: &mut[u8], connection_streams: &mut HashMap<SocketAddr, TcpStream>, game: &mut Game, stream: &mut TcpStream, connections: &mut HashMap<SocketAddr, Connection>) -> Result<Option<Action>, Box<dyn Error>> {
    let parsed_packet = lib::packets::serverbound::play::ChatMessage::try_from(data.to_vec())?;

    let player_index = game.players.iter().enumerate().find_map(|x| {
      if x.1.uuid == connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.unwrap_or_default() {
        Some(x.0)}
        else {None}
      });
    let Some(player) = game.players.get(player_index.unwrap()) else {
      println!("got chat_message packet from invalid player");
      return Ok(None);
    };

    println!("<{}>: {}", player.display_name, parsed_packet.message);

    let packet_to_send = lib::packets::clientbound::play::PlayerChatMessage {
      global_index: game.chat_message_index,
      sender: player.uuid,
      index: 0,
      message_signature_bytes: Vec::new(),
      message: parsed_packet.message.clone(),
      timestamp: parsed_packet.timestamp,
      salt: parsed_packet.salt,
      signature_array: Vec::new(),
      unsigned_content: None,
      filter_type: 0,
      filter_type_bits: Vec::new(),
      chat_type: 1,
      sender_name: NbtTag::Root(vec![
    		NbtTag::TagCompound("click_event".to_string(), vec![
     			NbtTag::String("action".to_string(), "suggest_command".to_string()),
     			NbtTag::String("command".to_string(), format!("/tell {}", player.display_name).to_string()),
     	]),
     	NbtTag::String("insertion".to_string(), player.display_name.clone()),
     	NbtTag::String("text".to_string(), player.display_name.clone()),
      ]),
      target_name: None,
		};

    game.chat_message_index += 1;

    for connection in connection_streams {
	   	send_packet(connection.1, lib::packets::clientbound::play::PlayerChatMessage::PACKET_ID, packet_to_send.clone().try_into()?)?;
    }

    return Ok(None);
  }

  pub fn chat_command(data: &mut[u8], stream: &mut TcpStream, game: &mut Game, connection_streams: &mut HashMap<SocketAddr, TcpStream>, connections: &mut HashMap<SocketAddr, Connection>) -> Result<Option<Action>, Box<dyn Error>> {
  	let parsed_packet = lib::packets::serverbound::play::ChatCommand::try_from(data.to_vec())?;

    println!("<{}> invoked: {}", game.players.iter().find(|x| x.peer_socket_address == stream.peer_addr().unwrap()).unwrap().display_name, parsed_packet.command);

   	let Some(command) = game.commands.iter().find(|x| x.name == parsed_packet.command.split(" ").next().unwrap_or_default()) else {
  		lib::utils::send_packet(stream, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
				  content: NbtTag::Root(vec![
					NbtTag::String("type".to_string(), "text".to_string()),
					NbtTag::String("text".to_string(), "command not found".to_string()),
				]),
			  overlay: false,
    	}.try_into()?)?;

    	return Ok(None);
    };

    (command.execute)(parsed_packet.command, Some(stream), game, connection_streams, connections)?;

  	return Ok(None);
  }

  pub fn pick_item_from_block(data: &mut[u8], stream: &mut TcpStream, game: &mut Game, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>) -> Result<Option<Action>, Box<dyn Error>> {
  	let parsed_packet = lib::packets::serverbound::play::PickItemFromBlock::try_from(data.to_vec())?;
   	let picked_block = game.world.dimensions.get("minecraft:overworld").unwrap().get_block(parsed_packet.location)?;
    let picked_block_name = data::blocks::get_blocks().iter().find(|x| x.1.states.iter().any(|x| x.id == picked_block)).unwrap().0.clone();
    let item_id = data::items::get_items().get(&picked_block_name).unwrap_or(&data::items::Item {max_stack_size: 0, rarity: data::items::ItemRarity::Common, id:0, repair_cost:0}).id;

    let player_index = game.players.iter().enumerate().find_map(|x| {
      if x.1.uuid == connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.unwrap_or_default() {
        Some(x.0)}
        else {None}
      });
    let Some(player) = game.players.get_mut(player_index.unwrap()) else {
      println!("got pick_item_from_block packet from invalid player");
      return Ok(None);
    };

    let new_slot_data = Slot {
	   	item_count: 1,
	    item_id,
	    components_to_add: Vec::new(),
	    components_to_remove: Vec::new(),
    };

    player.set_selected_inventory_slot(Some(new_slot_data), connections, connection_streams);

  	return Ok(None);
  }

  pub fn swing_arm(data: &mut[u8], stream: &mut TcpStream, game: &mut Game, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>) -> Result<Option<Action>, Box<dyn Error>> {
  	let parsed_packet = lib::packets::serverbound::play::SwingArm::try_from(data.to_vec())?;

  	connection_streams.iter()
    	.filter(|x| connections.get(x.0).unwrap().state == ConnectionState::Play)
     	.filter(|x| *x.0 != stream.peer_addr().unwrap())
     	.for_each(|x| {
      	lib::utils::send_packet(x.1, lib::packets::clientbound::play::EntityAnimation::PACKET_ID, lib::packets::clientbound::play::EntityAnimation {
     			entity_id: game.players.iter().find(|x| x.peer_socket_address == stream.peer_addr().unwrap()).unwrap().entity_id,
       		animation: if parsed_packet.hand == 0 { 0 } else { 3 },
       	}.try_into().unwrap()).unwrap();
      });

  	return Ok(None);
  }

  pub fn click_container(data: &mut[u8], stream: &mut TcpStream, game: &mut Game, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>) -> Result<Option<Action>, Box<dyn Error>> {
    let parsed_packet = lib::packets::serverbound::play::ClickContainer::try_from(data.to_vec())?;

    let player_index = game.players.iter().enumerate().find_map(|x| {
      if x.1.uuid == connections.get(&stream.peer_addr().unwrap()).unwrap().player_uuid.unwrap_or_default() {
        Some(x.0)}
        else {None}
      });
    let Some(player) = game.players.get(player_index.unwrap()) else {
      println!("got click_container packet from invalid player");
      return Ok(None);
    };

    let Some(position) = player.opened_container_at else {
      println!("player doesn't seems to have a container opened at the moment");
      return Ok(None);
    };

    let streams_with_container_opened = game.players.iter()
      .filter(|x| x.opened_container_at.is_some_and(|x| x == position))
      .map(|x| x.connection_stream.try_clone().unwrap())
      .collect::<Vec<TcpStream>>();

    let block_entity = game.world.dimensions
      .get_mut("minecraft:overworld").unwrap()
      .get_chunk_from_position_mut(position).unwrap()
      .try_get_block_entity_mut(position).unwrap();

    let Some(player) = game.players.get_mut(player_index.unwrap()) else {
      println!("got click_container packet from invalid player");
      return Ok(None);
    };

    match &mut block_entity.data {
      BlockEntityData::Chest(items) => {
        assert!(items.len() == 27);
        assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
        handle_container_click(parsed_packet, items, player, connections, connection_streams, streams_with_container_opened);
      },
      BlockEntityData::Furnace(items) => {
        assert!(items.len() == 3);
        assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
        handle_container_click(parsed_packet, items, player, connections, connection_streams, streams_with_container_opened);
        block_entity.needs_ticking = true;
      },
      BlockEntityData::BrewingStand(items) => {
        assert!(items.len() == 5);
        assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
        handle_container_click(parsed_packet, items, player, connections, connection_streams, streams_with_container_opened);
      },
      BlockEntityData::Crafter(items) => {
        assert!(items.len() == 9);
        assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
        handle_container_click(parsed_packet, items, player, connections, connection_streams, streams_with_container_opened);
      },
      BlockEntityData::Dispenser(items) => {
        assert!(items.len() == 9);
        assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
        handle_container_click(parsed_packet, items, player, connections, connection_streams, streams_with_container_opened);
      },
      BlockEntityData::Hopper(items) => {
        assert!(items.len() == 5);
        assert!(parsed_packet.slot < 36 + items.len() as i16); //36 for the players inventory
        handle_container_click(parsed_packet, items, player, connections, connection_streams, streams_with_container_opened);
      },
      x => println!("can't handle click_container packet for entity {x:?}"),
    }

    return Ok(None);
  }

  pub fn close_container(stream: &mut TcpStream, data: &mut [u8], game: &mut Game) -> Result<Option<Action>, Box<dyn Error>> {
    let parsed_packet = lib::packets::serverbound::play::CloseContainer::try_from(data.to_vec())?;

    if parsed_packet.window_id != 0 {
      game.players.iter_mut()
        .filter(|x| x.connection_stream.peer_addr().unwrap() == stream.peer_addr().unwrap())
        .for_each(|x| x.close_inventory().unwrap());
    }

    return Ok(None);
  }
}


fn handle_container_click(parsed_packet: lib::packets::serverbound::play::ClickContainer, chest_items: &mut [Item], player: &mut Player, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>, streams_with_container_opened: Vec<TcpStream>) {
  const PLAYER_INVENTORY_STARTING_INDEX: i16 = 9;
  let player_inventory_index = parsed_packet.slot - chest_items.len() as i16 + PLAYER_INVENTORY_STARTING_INDEX;

  let outside_clicked = parsed_packet.slot < 0;
  let chest_inventory_clicked = parsed_packet.slot < chest_items.len() as i16;
  let orig_inventory_item: Option<Slot> = if outside_clicked {
    None
  } else if chest_inventory_clicked {
    if chest_items[parsed_packet.slot as usize].count > 0 {
      Some(chest_items[parsed_packet.slot as usize].clone().into())
    } else {
      None
    }
  } else {
    player.get_inventory()[player_inventory_index as usize].clone()
  };

  //println!("orig item: {orig_inventory_item:?}");
  let orig_cursor_item: Option<Slot> = player.cursor_item.clone();
  //println!("orig cursor: {orig_cursor_item:?}");

  if parsed_packet.mode == 0 {
    let (new_inventory_item, new_cursor_item) = lib::containerclick::handle_click(parsed_packet.button == 0, orig_inventory_item.clone(), orig_cursor_item.clone());

    //println!("new item: {new_inventory_item:?}");
    if new_inventory_item != orig_inventory_item {
      if outside_clicked {
        //nothing to do
      } else if chest_inventory_clicked {
        //Chest inventory got changed
        chest_items[parsed_packet.slot as usize] = new_inventory_item.clone().into();
        for stream in streams_with_container_opened {
          lib::utils::send_packet(&stream, lib::packets::clientbound::play::SetContainerSlot::PACKET_ID, lib::packets::clientbound::play::SetContainerSlot {
            window_id: 1,
            state_id: 1,
            slot: parsed_packet.slot,
            slot_data: new_inventory_item.clone(),
          }.try_into().unwrap()).unwrap();
        }
      } else {
        //Player inventory got changed
        player.set_inventory_slot(player_inventory_index as u8, new_inventory_item, connections, connection_streams);
      }
    }

    //println!("new cursor: {new_cursor_item:?}");
    if new_cursor_item != orig_cursor_item {
      player.cursor_item = new_cursor_item;
    }
  } else if parsed_packet.mode == 1 {
    let orig_chest_inventory: Vec<Option<Slot>> = chest_items.to_vec().clone().into_iter().map(|x| x.into()).collect();
    let orig_player_inventory: Vec<Option<Slot>> = player.get_inventory().clone();
    let (new_chest_inventory, new_player_inventory) = lib::containerclick::handle_shift_click(orig_chest_inventory.clone(), orig_player_inventory.clone(), parsed_packet.slot);

    if orig_chest_inventory != new_chest_inventory {
      let new_chest_items: Vec<Item> = new_chest_inventory.into_iter().map(|x| x.into()).collect();
      assert_eq!(chest_items.len(), new_chest_items.len());
      chest_items.clone_from_slice(&new_chest_items);

      for connection_stream in connection_streams.iter().clone() {
        let _ = lib::utils::send_packet(connection_stream.1, lib::packets::clientbound::play::SetContainerContent::PACKET_ID, lib::packets::clientbound::play::SetContainerContent {
          window_id: 1,
          state_id: 1,
          slot_data: chest_items.iter().cloned().map(|x| x.into()).collect(),
          carried_item: None,
        }.try_into().unwrap());
      }
    }

    if orig_player_inventory != new_player_inventory {
      player.set_inventory(new_player_inventory, connections, connection_streams);
    }
  }
}
