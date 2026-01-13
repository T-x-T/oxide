use lib::ConnectionState;
use lib::packets::Packet;
use lib::types::*;
use std::error::Error;
use std::net::TcpStream;
use std::sync::Arc;

pub fn handle_packet(
	mut packet: lib::Packet,
	stream: &mut TcpStream,
	game: Arc<Game>,
) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
	let state = game.connections.get(&stream.peer_addr()?).unwrap().state.clone();

	//println!("{}", packet.id);

	return match state {
		ConnectionState::Handshaking => match packet.id {
			lib::packets::serverbound::handshaking::Handshake::PACKET_ID => handshaking::handshake(&mut packet.data, stream, game),
			_ => Ok(None),
		},
		ConnectionState::Status => match packet.id {
			lib::packets::serverbound::status::StatusRequest::PACKET_ID => status::status_request(stream, game),
			lib::packets::serverbound::status::PingRequest::PACKET_ID => status::ping_request(&packet.data, stream, game),
			_ => Ok(None),
		},
		ConnectionState::Login => match packet.id {
			lib::packets::serverbound::login::LoginStart::PACKET_ID => login::login_start(&packet.data, stream, game),
			lib::packets::serverbound::login::LoginAcknowledged::PACKET_ID => login::login_acknowledged(stream, game),
			_ => Ok(None),
		},
		ConnectionState::Configuration => match packet.id {
			lib::packets::serverbound::configuration::ServerboundKnownPackets::PACKET_ID => {
				configuration::serverbound_known_packets(stream, game)
			}
			lib::packets::serverbound::configuration::AcknowledgeFinishConfiguration::PACKET_ID => {
				configuration::acknowledge_finish_configuration(stream)
			}
			_ => Ok(None),
		},
		ConnectionState::Play => match packet.id {
			lib::packets::serverbound::play::ConfirmTeleportation::PACKET_ID => play::confirm_teleportation(&mut packet.data, stream),
			lib::packets::serverbound::play::ChatCommand::PACKET_ID => play::chat_command(&mut packet.data, stream),
			lib::packets::serverbound::play::ChatMessage::PACKET_ID => play::chat_message(&mut packet.data, stream),
			lib::packets::serverbound::play::PlayerAction::PACKET_ID => play::player_action(&mut packet.data, stream),
			lib::packets::serverbound::play::SetCreativeModeSlot::PACKET_ID => play::set_creative_mode_slot(&mut packet.data, stream),
			lib::packets::serverbound::play::SetHandItem::PACKET_ID => play::set_held_item(&mut packet.data, stream),
			lib::packets::serverbound::play::UseItemOn::PACKET_ID => play::use_item_on(&mut packet.data, stream),
			lib::packets::serverbound::play::SetPlayerPosition::PACKET_ID => play::set_player_position(&mut packet.data, stream),
			lib::packets::serverbound::play::SetPlayerPositionAndRotation::PACKET_ID => {
				play::set_player_position_and_rotation(&mut packet.data, stream)
			}
			lib::packets::serverbound::play::SetPlayerRotation::PACKET_ID => play::set_player_rotation(&mut packet.data, stream),
			lib::packets::serverbound::play::PickItemFromBlock::PACKET_ID => play::pick_item_from_block(&mut packet.data, stream),
			lib::packets::serverbound::play::SwingArm::PACKET_ID => play::swing_arm(&mut packet.data, stream),
			lib::packets::serverbound::play::ClickContainer::PACKET_ID => play::click_container(&mut packet.data, stream),
			lib::packets::serverbound::play::CloseContainer::PACKET_ID => play::close_container(stream, &mut packet.data),
			lib::packets::serverbound::play::UpdateSign::PACKET_ID => play::update_sign(&mut packet.data),
			lib::packets::serverbound::play::PlayerInput::PACKET_ID => play::player_input(stream, &mut packet.data),
			lib::packets::serverbound::play::Interact::PACKET_ID => play::interact(stream, &mut packet.data),
			_ => Ok(None),
		},
		ConnectionState::Transfer => todo!(),
	};
}

pub mod handshaking {
	use super::*;

	pub fn handshake(data: &mut [u8], stream: &mut TcpStream, game: Arc<Game>) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::handshaking::Handshake::try_from(data.to_vec())?;

		game.connections.entry(stream.peer_addr()?).and_modify(|x| x.state = parsed_packet.next_state.into());

		return Ok(None);
	}
}

pub mod status {
	use super::*;

	pub fn status_request(stream: &mut TcpStream, game: Arc<Game>) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let version_string = lib::packets::get_version_string();
		let protocol_version = lib::packets::get_protocol_version();
		let player_count = game.players.lock().unwrap().len();
		let motd = &std::env::var("OXIDE_MOTD").unwrap_or("Hello oxide!".to_string());
		game.send_packet(
			&stream.peer_addr().unwrap(),
			lib::packets::clientbound::status::StatusResponse::PACKET_ID,
			lib::packets::clientbound::status::StatusResponse {
				status: format!(
					"{{
        \"version\": {{
          \"name\": \"Oxide {version_string}\",
          \"protocol\": {protocol_version}
        }},
        \"players\": {{
          \"max\": -1,
          \"online\": {player_count},
          \"sample\": []
        }},
        \"description\": {{
          \"text\": \"{motd}\"
        }},
        \"enforcesSecureChat\": false
      }}"
				)
				.to_string(),
			}
			.try_into()?,
		);

		return Ok(None);
	}

	//implement actual packet struct https://git.thetxt.io/thetxt/oxide/issues/20
	pub fn ping_request(data: &[u8], stream: &mut TcpStream, game: Arc<Game>) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::status::PingRequest::try_from(data.to_vec())?;

		game.send_packet(
			&stream.peer_addr().unwrap(),
			lib::packets::clientbound::status::PingResponse::PACKET_ID,
			lib::packets::clientbound::status::PingResponse {
				timestamp: parsed_packet.timestamp,
			}
			.try_into()?,
		);

		return Ok(Some(PacketHandlerAction::DisconnectPlayer(stream.peer_addr()?)));
	}
}

pub mod login {
	use super::*;

	pub fn login_start(data: &[u8], stream: &mut TcpStream, game: Arc<Game>) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::login::LoginStart::try_from(data.to_vec())?;

		game.connections.entry(stream.peer_addr()?).and_modify(|x| {
			x.player_name = Some(parsed_packet.name.clone());
			x.player_uuid = Some(parsed_packet.uuid);
		});

		game.send_packet(
			&stream.peer_addr()?,
			lib::packets::clientbound::login::LoginSuccess::PACKET_ID,
			lib::packets::clientbound::login::LoginSuccess {
				uuid: parsed_packet.uuid,
				username: parsed_packet.name,
			}
			.try_into()?,
		);

		return Ok(None);
	}

	pub fn login_acknowledged(stream: &mut TcpStream, game: Arc<Game>) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		game.connections.entry(stream.peer_addr()?).and_modify(|x| x.state = ConnectionState::Configuration);

		game.send_packet(
			&stream.peer_addr()?,
			lib::packets::clientbound::configuration::ClientboundKnownPacks::PACKET_ID,
			lib::packets::clientbound::configuration::ClientboundKnownPacks {
				known_packs: vec![lib::Datapack {
					namespace: "minecraft".to_string(),
					id: "core".to_string(),
					version: lib::packets::get_version_string(),
				}],
			}
			.try_into()?,
		);

		game.send_packet(
			&stream.peer_addr()?,
			lib::packets::clientbound::configuration::ServerLinks::PACKET_ID,
			lib::packets::clientbound::configuration::ServerLinks {
				links: vec![
					ServerLink::Custom(
						NbtTag::Root(vec![NbtTag::String("text".to_string(), "Git repository".to_string())]),
						"https://git.thetxt.io/thetxt/oxide".to_string(),
					),
					ServerLink::Custom(
						NbtTag::Root(vec![NbtTag::String("text".to_string(), "Github mirror".to_string())]),
						"https://github.com/T-x-T/oxide".to_string(),
					),
					ServerLink::BuiltIn(BuiltInServerLink::BugReport, "https://git.thetxt.io/thetxt/oxide/issues/new".to_string()),
					ServerLink::Custom(
						NbtTag::Root(vec![NbtTag::String("text".to_string(), "Suggest feature".to_string())]),
						"https://git.thetxt.io/thetxt/oxide/issues/new".to_string(),
					),
					ServerLink::Custom(
						NbtTag::Root(vec![NbtTag::String("text".to_string(), "Support development of Oxide ♥".to_string())]),
						"https://coff.ee/thetxt".to_string(),
					),
					ServerLink::BuiltIn(BuiltInServerLink::Community, "https://discord.gg/RW5Ug4epKS".to_string()),
				],
			}
			.try_into()?,
		);

		return Ok(None);
	}
}

pub mod configuration {
	use super::*;
	use lib::packets::Packet;
	use lib::packets::clientbound::configuration::{RegistryDataEntry, Tag};

	#[rustfmt::skip]
  pub fn serverbound_known_packets(stream: &mut TcpStream, game: Arc<Game>) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
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
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
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
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
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
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
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
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
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
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
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
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:pig_variant".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:cold".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:temperate".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:warm".to_string(), has_data: false, data: None }
      ]
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:frog_variant".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:cold".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:temperate".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:warm".to_string(), has_data: false, data: None }
      ]
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
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
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:cow_variant".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:cold".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:temperate".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:warm".to_string(), has_data: false, data: None }
      ]
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:chicken_variant".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:cold".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:temperate".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:warm".to_string(), has_data: false, data: None }
      ]
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
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
        RegistryDataEntry { entry_id: "minecraft:dennis".to_string(), has_data: false, data: None },
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
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:dimension_type".to_string(),
      entries: vec![
        RegistryDataEntry { entry_id: "minecraft:overworld".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:overworld_caves".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:the_end".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:the_nether".to_string(), has_data: false, data: None }
      ]
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
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
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
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
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
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
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
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
        RegistryDataEntry { entry_id: "minecraft:lava_chicken".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:mall".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:mellohi".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:otherside".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:pigstep".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:precipice".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:relic".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:stal".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:strad".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:tears".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:wait".to_string(), has_data: false, data: None },
        RegistryDataEntry { entry_id: "minecraft:ward".to_string(), has_data: false, data: None }
      ]
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
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
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:dialog".to_string(),
      entries: vec![
	      RegistryDataEntry { entry_id: "minecraft:custom_options".to_string(), has_data: false, data: None },
	      RegistryDataEntry { entry_id: "minecraft:quick_actions".to_string(), has_data: false, data: None },
	      RegistryDataEntry { entry_id: "minecraft:server_links".to_string(), has_data: false, data: None }
      ]
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:test_environment".to_string(),
      entries: vec![RegistryDataEntry {entry_id: "minecraft:default".to_string(), has_data: false, data: None}]
    }.try_into()?);
    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::RegistryData::PACKET_ID, lib::packets::clientbound::configuration::RegistryData {
      registry_id: "minecraft:test_instance".to_string(),
      entries: vec![RegistryDataEntry {entry_id: "minecraft:always_pass".to_string(), has_data: false, data: None}]
    }.try_into()?);

    game.send_packet(&stream.peer_addr()?, lib::packets::clientbound::configuration::UpdateTags::PACKET_ID, lib::packets::clientbound::configuration::UpdateTags {
      data: vec![("minecraft:item".to_string(), vec![Tag { name: "minecraft:skulls".to_string(), entries: vec![1231, 1233, 1232, 1229, 1230, 1234, 1235] }, Tag { name: "minecraft:soul_fire_base_blocks".to_string(), entries: vec![360, 361] }, Tag { name: "minecraft:trim_materials".to_string(), entries: vec![902, 906, 898, 899, 908, 904, 900, 909, 901, 717, 1242] }, Tag { name: "minecraft:head_armor".to_string(), entries: vec![954, 958, 962, 974, 966, 970, 978, 887] }, Tag { name: "minecraft:copper_golem_statues".to_string(), entries: vec![1475, 1476, 1477, 1478, 1479, 1480, 1481, 1482] }, Tag { name: "minecraft:beacon_payment_items".to_string(), entries: vec![909, 899, 898, 908, 904] }, Tag { name: "minecraft:wooden_slabs".to_string(), entries: vec![270, 271, 272, 273, 274, 276, 277, 281, 282, 278, 279, 275] }, Tag { name: "minecraft:copper_tool_materials".to_string(), entries: vec![906] }, Tag { name: "minecraft:pale_oak_logs".to_string(), entries: vec![140, 177, 155, 166] }, Tag { name: "minecraft:coal_ores".to_string(), entries: vec![64, 65] }, Tag { name: "minecraft:chicken_food".to_string(), entries: vec![951, 1109, 1108, 1283, 1280, 1281] }, Tag { name: "minecraft:small_flowers".to_string(), entries: vec![229, 230, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 231] }, Tag { name: "minecraft:parrot_poisonous_food".to_string(), entries: vec![1102] }, Tag { name: "minecraft:wooden_trapdoors".to_string(), entries: vec![805, 803, 807, 808, 804, 801, 802, 811, 812, 809, 810, 806] }, Tag { name: "minecraft:pig_food".to_string(), entries: vec![1223, 1224, 1282] }, Tag { name: "minecraft:repairs_leather_armor".to_string(), entries: vec![1017] }, Tag { name: "minecraft:happy_ghast_tempt_items".to_string(), entries: vec![1016, 838, 839, 840, 841, 842, 843, 844, 845, 846, 847, 848, 849, 850, 851, 852, 853] }, Tag { name: "minecraft:repairs_diamond_armor".to_string(), entries: vec![898] }, Tag { name: "minecraft:iron_tool_materials".to_string(), entries: vec![904] }, Tag { name: "minecraft:wooden_shelves".to_string(), entries: vec![305, 306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316] }, Tag { name: "minecraft:trimmable_armor".to_string(), entries: vec![957, 961, 965, 977, 969, 973, 981, 956, 960, 964, 976, 968, 972, 980, 955, 959, 963, 975, 967, 971, 979, 954, 958, 962, 974, 966, 970, 978, 887] }, Tag { name: "minecraft:piglin_safe_armor".to_string(), entries: vec![974, 975, 976, 977] }, Tag { name: "minecraft:enchantable/mace".to_string(), entries: vec![1219] }, Tag { name: "minecraft:wool".to_string(), entries: vec![213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228] }, Tag { name: "minecraft:stairs".to_string(), entries: vec![441, 442, 443, 444, 445, 447, 448, 452, 453, 449, 450, 446, 451, 336, 438, 428, 420, 419, 328, 485, 572, 566, 565, 567, 681, 682, 683, 684, 685, 686, 687, 688, 689, 690, 691, 692, 693, 694, 1370, 1378, 1374, 695, 696, 698, 697, 109, 108, 107, 106, 128, 127, 126, 129, 421, 14, 19, 23, 415] }, Tag { name: "minecraft:logs".to_string(), entries: vec![141, 178, 154, 165, 140, 177, 155, 166, 134, 171, 148, 159, 138, 175, 152, 163, 136, 173, 150, 161, 137, 174, 151, 162, 135, 172, 149, 160, 142, 179, 156, 167, 139, 176, 153, 164, 145, 157, 180, 168, 146, 158, 181, 169] }, Tag { name: "minecraft:creeper_drop_music_discs".to_string(), entries: vec![1297, 1298, 1299, 1300, 1303, 1305, 1306, 1307, 1308, 1309, 1310, 1311] }, Tag { name: "minecraft:camel_food".to_string(), entries: vec![340] }, Tag { name: "minecraft:repairs_copper_armor".to_string(), entries: vec![906] }, Tag { name: "minecraft:gold_tool_materials".to_string(), entries: vec![908] }, Tag { name: "minecraft:wool_carpets".to_string(), entries: vec![505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517, 518, 519, 520] }, Tag { name: "minecraft:repairs_wolf_armor".to_string(), entries: vec![889] }, Tag { name: "minecraft:compasses".to_string(), entries: vec![1034, 1035] }, Tag { name: "minecraft:arrows".to_string(), entries: vec![895, 1288, 1287] }, Tag { name: "minecraft:furnace_minecart_fuel".to_string(), entries: vec![896, 897] }, Tag { name: "minecraft:cow_food".to_string(), entries: vec![952] }, Tag { name: "minecraft:bookshelf_books".to_string(), entries: vec![1029, 1217, 1240, 1216, 1295] }, Tag { name: "minecraft:enchantable/fishing".to_string(), entries: vec![1053] }, Tag { name: "minecraft:decorated_pot_ingredients".to_string(), entries: vec![1025, 1428, 1429, 1430, 1431, 1432, 1433, 1434, 1435, 1437, 1439, 1440, 1441, 1442, 1443, 1444, 1445, 1447, 1448, 1449, 1450, 1436, 1438, 1446] }, Tag { name: "minecraft:wooden_doors".to_string(), entries: vec![780, 781, 782, 783, 784, 786, 787, 790, 791, 788, 789, 785] }, Tag { name: "minecraft:horse_food".to_string(), entries: vec![952, 1084, 504, 893, 1223, 1228, 986, 987] }, Tag { name: "minecraft:enchantable/sword".to_string(), entries: vec![936, 921, 926, 941, 911, 931, 916] }, Tag { name: "minecraft:meat".to_string(), entries: vec![1110, 1112, 1111, 1113, 1260, 984, 1246, 1259, 983, 1245, 1114] }, Tag { name: "minecraft:warped_stems".to_string(), entries: vec![146, 158, 181, 169] }, Tag { name: "minecraft:emerald_ores".to_string(), entries: vec![74, 75] }, Tag { name: "minecraft:enchantable/fire_aspect".to_string(), entries: vec![936, 921, 926, 941, 911, 931, 916, 1219] }, Tag { name: "minecraft:bamboo_blocks".to_string(), entries: vec![147, 170] }, Tag { name: "minecraft:crimson_stems".to_string(), entries: vec![145, 157, 180, 168] }, Tag { name: "minecraft:wolf_food".to_string(), entries: vec![1110, 1112, 1111, 1113, 1260, 984, 1246, 1259, 983, 1245, 1114, 1057, 1061, 1058, 1062, 1059, 1060, 1247] }, Tag { name: "minecraft:horse_tempt_items".to_string(), entries: vec![1228, 986, 987] }, Tag { name: "minecraft:enchantable/bow".to_string(), entries: vec![894] }, Tag { name: "minecraft:ignored_by_piglin_babies".to_string(), entries: vec![1017] }, Tag { name: "minecraft:swords".to_string(), entries: vec![936, 921, 926, 941, 911, 931, 916] }, Tag { name: "minecraft:enchantable/leg_armor".to_string(), entries: vec![956, 960, 964, 976, 968, 972, 980] }, Tag { name: "minecraft:stone_tool_materials".to_string(), entries: vec![35, 1368, 9] }, Tag { name: "minecraft:wart_blocks".to_string(), entries: vec![576, 577] }, Tag { name: "minecraft:terracotta".to_string(), entries: vec![521, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 498, 499, 500, 501] }, Tag { name: "minecraft:dark_oak_logs".to_string(), entries: vec![141, 178, 154, 165] }, Tag { name: "minecraft:enchantable/durability".to_string(), entries: vec![957, 961, 965, 977, 969, 973, 981, 956, 960, 964, 976, 968, 972, 980, 955, 959, 963, 975, 967, 971, 979, 954, 958, 962, 974, 966, 970, 978, 887, 862, 1290, 936, 921, 926, 941, 911, 931, 916, 939, 924, 929, 944, 914, 934, 919, 938, 923, 928, 943, 913, 933, 918, 937, 922, 927, 942, 912, 932, 917, 940, 925, 930, 945, 915, 935, 920, 894, 1322, 1319, 891, 1105, 1408, 1053, 859, 860, 1219] }, Tag { name: "minecraft:strider_food".to_string(), entries: vec![250] }, Tag { name: "minecraft:villager_picks_up".to_string(), entries: vec![951, 1224, 1223, 1283, 1280, 1281, 953, 952, 1282] }, Tag { name: "minecraft:non_flammable_wood".to_string(), entries: vec![146, 158, 181, 169, 145, 157, 180, 168, 46, 47, 281, 282, 777, 778, 354, 355, 811, 812, 831, 832, 452, 453, 761, 762, 790, 791, 998, 999, 1011, 1010, 316, 309] }, Tag { name: "minecraft:coals".to_string(), entries: vec![896, 897] }, Tag { name: "minecraft:hoglin_food".to_string(), entries: vec![249] }, Tag { name: "minecraft:piglin_food".to_string(), entries: vec![983, 984] }, Tag { name: "minecraft:repairs_iron_armor".to_string(), entries: vec![904] }, Tag { name: "minecraft:shulker_boxes".to_string(), entries: vec![581, 597, 593, 594, 591, 589, 595, 585, 590, 587, 584, 583, 588, 592, 596, 582, 586] }, Tag { name: "minecraft:piglin_preferred_weapons".to_string(), entries: vec![1322] }, Tag { name: "minecraft:harnesses".to_string(), entries: vec![838, 839, 840, 841, 842, 843, 844, 845, 846, 847, 848, 849, 850, 851, 852, 853] }, Tag { name: "minecraft:breaks_decorated_pots".to_string(), entries: vec![936, 921, 926, 941, 911, 931, 916, 939, 924, 929, 944, 914, 934, 919, 938, 923, 928, 943, 913, 933, 918, 937, 922, 927, 942, 912, 932, 917, 940, 925, 930, 945, 915, 935, 920, 1319, 1219] }, Tag { name: "minecraft:anvil".to_string(), entries: vec![478, 479, 480] }, Tag { name: "minecraft:panda_eats_from_ground".to_string(), entries: vec![269, 1085] }, Tag { name: "minecraft:birch_logs".to_string(), entries: vec![136, 173, 150, 161] }, Tag { name: "minecraft:axes".to_string(), entries: vec![939, 924, 929, 944, 914, 934, 919] }, Tag { name: "minecraft:lapis_ores".to_string(), entries: vec![76, 77] }, Tag { name: "minecraft:enchantable/mining".to_string(), entries: vec![939, 924, 929, 944, 914, 934, 919, 938, 923, 928, 943, 913, 933, 918, 937, 922, 927, 942, 912, 932, 917, 940, 925, 930, 945, 915, 935, 920, 1105] }, Tag { name: "minecraft:hoes".to_string(), entries: vec![940, 925, 930, 945, 915, 935, 920] }, Tag { name: "minecraft:repairs_turtle_helmet".to_string(), entries: vec![888] }, Tag { name: "minecraft:llama_food".to_string(), entries: vec![952, 504] }, Tag { name: "minecraft:sniffer_food".to_string(), entries: vec![1280] }, Tag { name: "minecraft:lanterns".to_string(), entries: vec![1346, 1347, 1348, 1352, 1349, 1353, 1350, 1354, 1351, 1355] }, Tag { name: "minecraft:enchantable/head_armor".to_string(), entries: vec![954, 958, 962, 974, 966, 970, 978, 887] }, Tag { name: "minecraft:fences".to_string(), entries: vec![344, 348, 350, 351, 345, 346, 347, 354, 355, 352, 353, 349, 427] }, Tag { name: "minecraft:saplings".to_string(), entries: vec![49, 50, 51, 52, 53, 55, 56, 205, 206, 57, 54] }, Tag { name: "minecraft:parrot_food".to_string(), entries: vec![951, 1109, 1108, 1283, 1280, 1281] }, Tag { name: "minecraft:beds".to_string(), entries: vec![1100, 1101, 1097, 1098, 1095, 1093, 1099, 1089, 1094, 1091, 1088, 1087, 1092, 1096, 1086, 1090] }, Tag { name: "minecraft:iron_ores".to_string(), entries: vec![66, 67] }, Tag { name: "minecraft:enchantable/armor".to_string(), entries: vec![957, 961, 965, 977, 969, 973, 981, 956, 960, 964, 976, 968, 972, 980, 955, 959, 963, 975, 967, 971, 979, 954, 958, 962, 974, 966, 970, 978, 887] }, Tag { name: "minecraft:rabbit_food".to_string(), entries: vec![1223, 1228, 229] }, Tag { name: "minecraft:bundles".to_string(), entries: vec![1036, 1052, 1048, 1049, 1046, 1044, 1050, 1040, 1045, 1042, 1039, 1038, 1043, 1047, 1051, 1041, 1037] }, Tag { name: "minecraft:pillager_preferred_weapons".to_string(), entries: vec![1322] }, Tag { name: "minecraft:oak_logs".to_string(), entries: vec![134, 171, 148, 159] }, Tag { name: "minecraft:drowned_preferred_weapons".to_string(), entries: vec![1319] }, Tag { name: "minecraft:doors".to_string(), entries: vec![780, 781, 782, 783, 784, 786, 787, 790, 791, 788, 789, 785, 792, 793, 794, 795, 796, 797, 798, 799, 779] }, Tag { name: "minecraft:ocelot_food".to_string(), entries: vec![1057, 1058] }, Tag { name: "minecraft:banners".to_string(), entries: vec![1261, 1262, 1263, 1264, 1265, 1266, 1267, 1268, 1269, 1270, 1271, 1272, 1273, 1274, 1275, 1276] }, Tag { name: "minecraft:noteblock_top_instruments".to_string(), entries: vec![1232, 1229, 1233, 1234, 1230, 1235, 1231] }, Tag { name: "minecraft:eggs".to_string(), entries: vec![1031, 1032, 1033] }, Tag { name: "minecraft:brewing_fuel".to_string(), entries: vec![1124] }, Tag { name: "minecraft:stone_crafting_materials".to_string(), entries: vec![35, 1368, 9] }, Tag { name: "minecraft:smelts_to_glass".to_string(), entries: vec![59, 62] }, Tag { name: "minecraft:wooden_fences".to_string(), entries: vec![344, 348, 350, 351, 345, 346, 347, 354, 355, 352, 353, 349] }, Tag { name: "minecraft:piglin_repellents".to_string(), entries: vec![365, 1347, 1359] }, Tag { name: "minecraft:axolotl_food".to_string(), entries: vec![1022] }, Tag { name: "minecraft:villager_plantable_seeds".to_string(), entries: vec![951, 1224, 1223, 1283, 1280, 1281] }, Tag { name: "minecraft:leg_armor".to_string(), entries: vec![956, 960, 964, 976, 968, 972, 980] }, Tag { name: "minecraft:wither_skeleton_disliked_weapons".to_string(), entries: vec![894, 1322] }, Tag { name: "minecraft:enchantable/foot_armor".to_string(), entries: vec![957, 961, 965, 977, 969, 973, 981] }, Tag { name: "minecraft:panda_food".to_string(), entries: vec![269] }, Tag { name: "minecraft:dampens_vibrations".to_string(), entries: vec![213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517, 518, 519, 520] }, Tag { name: "minecraft:mangrove_logs".to_string(), entries: vec![142, 179, 156, 167] }, Tag { name: "minecraft:jungle_logs".to_string(), entries: vec![137, 174, 151, 162] }, Tag { name: "minecraft:lectern_books".to_string(), entries: vec![1217, 1216] }, Tag { name: "minecraft:enchantable/chest_armor".to_string(), entries: vec![955, 959, 963, 975, 967, 971, 979] }, Tag { name: "minecraft:turtle_food".to_string(), entries: vec![211] }, Tag { name: "minecraft:copper".to_string(), entries: vec![91, 95, 96, 97, 114, 115, 116, 117] }, Tag { name: "minecraft:llama_tempt_items".to_string(), entries: vec![504] }, Tag { name: "minecraft:signs".to_string(), entries: vec![988, 989, 990, 992, 991, 994, 995, 998, 999, 996, 997, 993] }, Tag { name: "minecraft:spruce_logs".to_string(), entries: vec![135, 172, 149, 160] }, Tag { name: "minecraft:wooden_stairs".to_string(), entries: vec![441, 442, 443, 444, 445, 447, 448, 452, 453, 449, 450, 446] }, Tag { name: "minecraft:enchantable/weapon".to_string(), entries: vec![936, 921, 926, 941, 911, 931, 916, 939, 924, 929, 944, 914, 934, 919, 1219] }, Tag { name: "minecraft:wooden_buttons".to_string(), entries: vec![751, 752, 753, 754, 755, 757, 758, 761, 762, 759, 760, 756] }, Tag { name: "minecraft:book_cloning_target".to_string(), entries: vec![1216] }, Tag { name: "minecraft:netherite_tool_materials".to_string(), entries: vec![909] }, Tag { name: "minecraft:fishes".to_string(), entries: vec![1057, 1061, 1058, 1062, 1060, 1059] }, Tag { name: "minecraft:stone_bricks".to_string(), entries: vec![375, 376, 377, 378] }, Tag { name: "minecraft:copper_chests".to_string(), entries: vec![1467, 1468, 1469, 1470, 1471, 1472, 1473, 1474] }, Tag { name: "minecraft:shovels".to_string(), entries: vec![937, 922, 927, 942, 912, 932, 917] }, Tag { name: "minecraft:chest_boats".to_string(), entries: vec![864, 866, 868, 870, 872, 876, 878, 880, 882, 874] }, Tag { name: "minecraft:enchantable/equippable".to_string(), entries: vec![957, 961, 965, 977, 969, 973, 981, 956, 960, 964, 976, 968, 972, 980, 955, 959, 963, 975, 967, 971, 979, 954, 958, 962, 974, 966, 970, 978, 887, 862, 1231, 1233, 1232, 1229, 1230, 1234, 1235, 357] }, Tag { name: "minecraft:creeper_igniters".to_string(), entries: vec![891, 1214] }, Tag { name: "minecraft:enchantable/trident".to_string(), entries: vec![1319] }, Tag { name: "minecraft:slabs".to_string(), entries: vec![270, 271, 272, 273, 274, 276, 277, 281, 282, 278, 279, 275, 280, 283, 284, 290, 285, 296, 293, 294, 289, 288, 292, 287, 297, 298, 299, 699, 700, 701, 702, 703, 704, 705, 706, 707, 708, 709, 710, 711, 286, 295, 1369, 1377, 1373, 712, 713, 715, 714, 132, 131, 130, 113, 112, 111, 110, 133, 291, 13, 18, 22, 416] }, Tag { name: "minecraft:enchantable/mining_loot".to_string(), entries: vec![939, 924, 929, 944, 914, 934, 919, 938, 923, 928, 943, 913, 933, 918, 937, 922, 927, 942, 912, 932, 917, 940, 925, 930, 945, 915, 935, 920] }, Tag { name: "minecraft:enchantable/sharp_weapon".to_string(), entries: vec![936, 921, 926, 941, 911, 931, 916, 939, 924, 929, 944, 914, 934, 919] }, Tag { name: "minecraft:repairs_netherite_armor".to_string(), entries: vec![909] }, Tag { name: "minecraft:enchantable/vanishing".to_string(), entries: vec![957, 961, 965, 977, 969, 973, 981, 956, 960, 964, 976, 968, 972, 980, 955, 959, 963, 975, 967, 971, 979, 954, 958, 962, 974, 966, 970, 978, 887, 862, 1290, 936, 921, 926, 941, 911, 931, 916, 939, 924, 929, 944, 914, 934, 919, 938, 923, 928, 943, 913, 933, 918, 937, 922, 927, 942, 912, 932, 917, 940, 925, 930, 945, 915, 935, 920, 894, 1322, 1319, 891, 1105, 1408, 1053, 859, 860, 1219, 1034, 357, 1231, 1233, 1232, 1229, 1230, 1234, 1235] }, Tag { name: "minecraft:hanging_signs".to_string(), entries: vec![1000, 1001, 1002, 1004, 1005, 1003, 1006, 1007, 1010, 1011, 1008, 1009] }, Tag { name: "minecraft:wooden_tool_materials".to_string(), entries: vec![36, 37, 38, 39, 40, 42, 43, 46, 47, 44, 45, 41] }, Tag { name: "minecraft:redstone_ores".to_string(), entries: vec![72, 73] }, Tag { name: "minecraft:trapdoors".to_string(), entries: vec![805, 803, 807, 808, 804, 801, 802, 811, 812, 809, 810, 806, 800, 813, 814, 815, 816, 817, 818, 819, 820] }, Tag { name: "minecraft:duplicates_allays".to_string(), entries: vec![902] }, Tag { name: "minecraft:cherry_logs".to_string(), entries: vec![139, 176, 153, 164] }, Tag { name: "minecraft:flowers".to_string(), entries: vec![229, 230, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 231, 524, 525, 527, 526, 245, 192, 206, 57, 187, 258, 259, 325, 246, 341] }, Tag { name: "minecraft:buttons".to_string(), entries: vec![751, 752, 753, 754, 755, 757, 758, 761, 762, 759, 760, 756, 749, 750] }, Tag { name: "minecraft:dyeable".to_string(), entries: vec![954, 955, 956, 957, 1255, 890] }, Tag { name: "minecraft:planks".to_string(), entries: vec![36, 37, 38, 39, 40, 42, 43, 46, 47, 44, 45, 41] }, Tag { name: "minecraft:fox_food".to_string(), entries: vec![1356, 1357] }, Tag { name: "minecraft:bars".to_string(), entries: vec![390, 391, 395, 392, 396, 393, 397, 394, 398] }, Tag { name: "minecraft:stone_buttons".to_string(), entries: vec![749, 750] }, Tag { name: "minecraft:boats".to_string(), entries: vec![863, 865, 867, 869, 871, 875, 877, 879, 881, 873, 864, 866, 868, 870, 872, 876, 878, 880, 882, 874] }, Tag { name: "minecraft:enchantable/crossbow".to_string(), entries: vec![1322] }, Tag { name: "minecraft:chest_armor".to_string(), entries: vec![955, 959, 963, 975, 967, 971, 979] }, Tag { name: "minecraft:frog_food".to_string(), entries: vec![1030] }, Tag { name: "minecraft:skeleton_preferred_weapons".to_string(), entries: vec![894] }, Tag { name: "minecraft:rails".to_string(), entries: vec![835, 833, 834, 836] }, Tag { name: "minecraft:diamond_ores".to_string(), entries: vec![78, 79] }, Tag { name: "minecraft:happy_ghast_food".to_string(), entries: vec![1016] }, Tag { name: "minecraft:shearable_from_copper_golem".to_string(), entries: vec![232] }, Tag { name: "minecraft:leaves".to_string(), entries: vec![185, 182, 183, 189, 188, 186, 184, 191, 192, 190, 187] }, Tag { name: "minecraft:strider_tempt_items".to_string(), entries: vec![250, 860] }, Tag { name: "minecraft:walls".to_string(), entries: vec![456, 457, 458, 459, 460, 461, 462, 463, 465, 466, 467, 468, 469, 470, 471, 473, 472, 474, 475, 477, 476, 464, 15, 20, 24, 417] }, Tag { name: "minecraft:gaze_disguise_equipment".to_string(), entries: vec![357] }, Tag { name: "minecraft:map_invisibility_equipment".to_string(), entries: vec![357] }, Tag { name: "minecraft:fence_gates".to_string(), entries: vec![825, 823, 827, 828, 824, 821, 822, 831, 832, 829, 830, 826] }, Tag { name: "minecraft:armadillo_food".to_string(), entries: vec![1122] }, Tag { name: "minecraft:goat_food".to_string(), entries: vec![952] }, Tag { name: "minecraft:wooden_pressure_plates".to_string(), entries: vec![767, 768, 769, 770, 771, 773, 774, 777, 778, 775, 776, 772] }, Tag { name: "minecraft:repairs_gold_armor".to_string(), entries: vec![908] }, Tag { name: "minecraft:acacia_logs".to_string(), entries: vec![138, 175, 152, 163] }, Tag { name: "minecraft:piglin_loved".to_string(), entries: vec![70, 80, 71, 92, 1371, 765, 908, 1345, 1054, 1228, 1129, 986, 987, 974, 975, 976, 977, 1253, 926, 928, 927, 929, 930, 907, 86] }, Tag { name: "minecraft:cat_food".to_string(), entries: vec![1057, 1058] }, Tag { name: "minecraft:candles".to_string(), entries: vec![1381, 1382, 1383, 1384, 1385, 1386, 1387, 1388, 1389, 1390, 1391, 1392, 1393, 1394, 1395, 1396, 1397] }, Tag { name: "minecraft:sheep_food".to_string(), entries: vec![952] }, Tag { name: "minecraft:chains".to_string(), entries: vec![399, 400, 404, 401, 405, 402, 406, 403, 407] }, Tag { name: "minecraft:bee_food".to_string(), entries: vec![229, 230, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 524, 525, 527, 526, 245, 192, 206, 57, 187, 258, 259, 325, 246, 341] }, Tag { name: "minecraft:copper_ores".to_string(), entries: vec![68, 69] }, Tag { name: "minecraft:sand".to_string(), entries: vec![59, 62, 60] }, Tag { name: "minecraft:gold_ores".to_string(), entries: vec![70, 80, 71] }, Tag { name: "minecraft:freeze_immune_wearables".to_string(), entries: vec![957, 956, 955, 954, 1255] }, Tag { name: "minecraft:completes_find_tree_tutorial".to_string(), entries: vec![141, 178, 154, 165, 140, 177, 155, 166, 134, 171, 148, 159, 138, 175, 152, 163, 136, 173, 150, 161, 137, 174, 151, 162, 135, 172, 149, 160, 142, 179, 156, 167, 139, 176, 153, 164, 145, 157, 180, 168, 146, 158, 181, 169, 185, 182, 183, 189, 188, 186, 184, 191, 192, 190, 187, 576, 577] }, Tag { name: "minecraft:logs_that_burn".to_string(), entries: vec![141, 178, 154, 165, 140, 177, 155, 166, 134, 171, 148, 159, 138, 175, 152, 163, 136, 173, 150, 161, 137, 174, 151, 162, 135, 172, 149, 160, 142, 179, 156, 167, 139, 176, 153, 164] }, Tag { name: "minecraft:lightning_rods".to_string(), entries: vec![733, 734, 735, 736, 737, 738, 739, 740] }, Tag { name: "minecraft:dirt".to_string(), entries: vec![28, 27, 30, 29, 422, 31, 262, 265, 32, 144] }, Tag { name: "minecraft:diamond_tool_materials".to_string(), entries: vec![898] }, Tag { name: "minecraft:pickaxes".to_string(), entries: vec![938, 923, 928, 943, 913, 933, 918] }, Tag { name: "minecraft:repairs_chain_armor".to_string(), entries: vec![904] }, Tag { name: "minecraft:decorated_pot_sherds".to_string(), entries: vec![1428, 1429, 1430, 1431, 1432, 1433, 1434, 1435, 1437, 1439, 1440, 1441, 1442, 1443, 1444, 1445, 1447, 1448, 1449, 1450, 1436, 1438, 1446] }, Tag { name: "minecraft:cluster_max_harvestables".to_string(), entries: vec![938, 928, 933, 943, 923, 913, 918] }, Tag { name: "minecraft:foot_armor".to_string(), entries: vec![957, 961, 965, 977, 969, 973, 981] }]), ("minecraft:dialog".to_string(), vec![Tag { name: "minecraft:pause_screen_additions".to_string(), entries: vec![] }, Tag { name: "minecraft:quick_actions".to_string(), entries: vec![] }]), ("minecraft:block".to_string(), vec![Tag { name: "minecraft:mob_interactable_doors".to_string(), entries: vec![219, 644, 645, 646, 647, 649, 650, 897, 898, 651, 652, 648, 1047, 1048, 1050, 1049, 1051, 1052, 1054, 1053] }, Tag { name: "minecraft:soul_fire_base_blocks".to_string(), entries: vec![285, 286] }, Tag { name: "minecraft:copper_golem_statues".to_string(), entries: vec![1087, 1088, 1089, 1090, 1091, 1092, 1093, 1094] }, Tag { name: "minecraft:triggers_ambient_desert_sand_block_sounds".to_string(), entries: vec![37, 39] }, Tag { name: "minecraft:snaps_goat_horn".to_string(), entries: vec![53, 51, 49, 52, 50, 55, 56, 57, 54, 1, 554, 44, 46, 1009, 397] }, Tag { name: "minecraft:pale_oak_logs".to_string(), entries: vec![56, 20, 67, 86] }, Tag { name: "minecraft:occludes_vibration_signals".to_string(), entries: vec![140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155] }, Tag { name: "minecraft:invalid_spawn_inside".to_string(), entries: vec![390, 665] }, Tag { name: "minecraft:dry_vegetation_may_place_on".to_string(), entries: vec![37, 39, 38, 552, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 9, 8, 11, 10, 372, 1119, 1114, 1158, 1120, 59, 207] }, Tag { name: "minecraft:wool".to_string(), entries: vec![140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155] }, Tag { name: "minecraft:beehives".to_string(), entries: vec![909, 910] }, Tag { name: "minecraft:trail_ruins_replaceable".to_string(), entries: vec![40] }, Tag { name: "minecraft:ice".to_string(), entries: vec![276, 554, 787, 668] }, Tag { name: "minecraft:enchantment_power_provider".to_string(), entries: vec![177] }, Tag { name: "minecraft:wool_carpets".to_string(), entries: vec![536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551] }, Tag { name: "minecraft:dragon_immune".to_string(), entries: vec![522, 34, 390, 391, 665, 406, 666, 667, 903, 904, 156, 192, 915, 392, 340, 916, 1152, 905, 906] }, Tag { name: "minecraft:crops".to_string(), entries: vec![663, 439, 440, 206, 364, 363, 660, 661] }, Tag { name: "minecraft:mangrove_roots_can_grow_through".to_string(), entries: vec![1120, 59, 58, 1110, 365, 33, 275] }, Tag { name: "minecraft:features_cannot_replace".to_string(), entries: vec![34, 197, 200, 391, 1152, 1155, 1156] }, Tag { name: "minecraft:crystal_sound_blocks".to_string(), entries: vec![976, 977] }, Tag { name: "minecraft:warped_stems".to_string(), entries: vec![860, 861, 862, 863] }, Tag { name: "minecraft:bamboo_blocks".to_string(), entries: vec![60, 70] }, Tag { name: "minecraft:concrete_powder".to_string(), entries: vec![724, 725, 726, 727, 728, 729, 730, 731, 732, 733, 734, 735, 736, 737, 738, 739] }, Tag { name: "minecraft:lava_pool_stone_cannot_replace".to_string(), entries: vec![34, 197, 200, 391, 1152, 1155, 1156, 91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93, 55, 77, 66, 85, 56, 20, 67, 86, 49, 71, 68, 79, 53, 75, 64, 83, 51, 73, 62, 81, 52, 74, 63, 82, 50, 72, 61, 80, 57, 78, 69, 87, 54, 76, 65, 84, 869, 870, 871, 872, 860, 861, 862, 863] }, Tag { name: "minecraft:inside_step_sound_blocks".to_string(), entries: vec![998, 1002, 366, 373, 981, 1111, 1112, 1113] }, Tag { name: "minecraft:prevent_mob_spawning_inside".to_string(), entries: vec![221, 126, 127, 480] }, Tag { name: "minecraft:climbable".to_string(), entries: vec![220, 365, 835, 876, 877, 878, 879, 1105, 1106] }, Tag { name: "minecraft:parrots_spawnable_on".to_string(), entries: vec![8, 0, 91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93, 55, 77, 66, 85, 56, 20, 67, 86, 49, 71, 68, 79, 53, 75, 64, 83, 51, 73, 62, 81, 52, 74, 63, 82, 50, 72, 61, 80, 57, 78, 69, 87, 54, 76, 65, 84, 869, 870, 871, 872, 860, 861, 862, 863] }, Tag { name: "minecraft:dark_oak_logs".to_string(), entries: vec![55, 77, 66, 85] }, Tag { name: "minecraft:coral_plants".to_string(), entries: vec![761, 762, 763, 764, 765] }, Tag { name: "minecraft:goats_spawnable_on".to_string(), entries: vec![8, 1, 275, 277, 554, 40] }, Tag { name: "minecraft:ceiling_hanging_signs".to_string(), entries: vec![233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244] }, Tag { name: "minecraft:sculk_replaceable_world_gen".to_string(), entries: vec![1, 2, 4, 6, 982, 1121, 9, 8, 11, 10, 372, 1119, 1114, 1158, 1120, 59, 552, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 873, 864, 284, 287, 922, 37, 39, 40, 285, 286, 996, 1142, 280, 1104, 392, 593, 106, 1134, 1130, 1122, 1139, 1140, 1126] }, Tag { name: "minecraft:triggers_ambient_dried_ghast_block_sounds".to_string(), entries: vec![285, 286] }, Tag { name: "minecraft:lapis_ores".to_string(), entries: vec![102, 103] }, Tag { name: "minecraft:convertable_to_mud".to_string(), entries: vec![9, 10, 1119] }, Tag { name: "minecraft:sniffer_diggable_block".to_string(), entries: vec![9, 8, 11, 10, 1119, 1114, 1158, 1120, 59] }, Tag { name: "minecraft:lush_ground_replaceable".to_string(), entries: vec![1, 2, 4, 6, 982, 1121, 1106, 1105, 9, 8, 11, 10, 372, 1119, 1114, 1158, 1120, 59, 280, 40, 37] }, Tag { name: "minecraft:sword_instantly_mines".to_string(), entries: vec![790, 789] }, Tag { name: "minecraft:fences".to_string(), entries: vec![283, 638, 640, 641, 635, 636, 637, 887, 888, 642, 643, 639, 381] }, Tag { name: "minecraft:camels_spawnable_on".to_string(), entries: vec![37, 39, 38] }, Tag { name: "minecraft:saplings".to_string(), entries: vec![25, 26, 27, 28, 29, 31, 32, 1108, 1109, 33, 30] }, Tag { name: "minecraft:mineable/pickaxe".to_string(), entries: vec![1, 2, 3, 4, 5, 6, 7, 12, 42, 43, 44, 45, 46, 47, 48, 102, 103, 104, 105, 106, 107, 108, 173, 174, 175, 191, 192, 197, 202, 203, 204, 208, 222, 258, 259, 270, 271, 284, 287, 288, 325, 326, 327, 328, 369, 370, 380, 381, 382, 384, 385, 392, 396, 397, 398, 399, 402, 469, 470, 473, 474, 475, 476, 477, 478, 479, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 552, 553, 593, 594, 595, 596, 608, 609, 610, 611, 612, 613, 614, 615, 617, 618, 619, 620, 621, 622, 623, 624, 625, 656, 657, 658, 659, 669, 671, 672, 674, 692, 693, 694, 695, 696, 697, 698, 699, 700, 701, 702, 703, 704, 705, 706, 707, 708, 709, 710, 711, 712, 713, 714, 715, 716, 717, 718, 719, 720, 721, 722, 723, 746, 747, 748, 749, 750, 751, 752, 753, 754, 755, 756, 757, 758, 759, 760, 766, 767, 768, 769, 770, 776, 777, 778, 779, 780, 795, 796, 797, 798, 799, 800, 801, 802, 803, 804, 805, 806, 807, 808, 809, 810, 811, 812, 813, 814, 815, 816, 817, 818, 819, 820, 821, 838, 839, 842, 845, 846, 864, 873, 913, 914, 915, 916, 921, 922, 923, 925, 926, 927, 928, 929, 930, 931, 933, 934, 935, 936, 939, 940, 941, 982, 996, 1008, 1007, 1006, 1005, 1009, 1010, 1011, 1012, 1013, 1014, 1023, 1024, 1025, 1026, 1027, 1028, 1029, 1030, 1031, 1032, 1033, 1034, 1035, 1036, 1037, 1038, 1039, 1040, 1041, 1042, 1043, 1044, 1045, 1046, 1103, 1104, 1121, 1122, 1123, 1124, 1126, 1127, 1128, 1130, 1131, 1132, 1134, 1135, 1136, 1138, 1139, 1140, 1142, 1143, 1144, 1145, 276, 554, 787, 138, 128, 139, 978, 981, 980, 979, 976, 977, 332, 336, 335, 1141, 331, 334, 333, 274, 937, 408, 409, 822, 823, 824, 825, 826, 827, 829, 830, 831, 832, 833, 834, 924, 932, 938, 1125, 1129, 1133, 1137, 828, 985, 989, 994, 378, 675, 691, 687, 688, 685, 683, 689, 679, 684, 681, 678, 677, 682, 686, 690, 676, 680, 465, 466, 467, 386, 387, 388, 389, 221, 126, 127, 480, 788, 330, 371, 616, 329, 1154, 983, 984, 990, 986, 987, 988, 991, 992, 993, 995, 1018, 1017, 1016, 1015, 1022, 1021, 1020, 1019, 1063, 1064, 1065, 1066, 1067, 1068, 1069, 1070, 1071, 1072, 1073, 1074, 1075, 1076, 1077, 1078, 1047, 1048, 1050, 1049, 1051, 1052, 1054, 1053, 1055, 1056, 1058, 1057, 1059, 1060, 1062, 1061, 1157, 375, 377, 376, 379, 1079, 1080, 1081, 1082, 1083, 1084, 1085, 1086, 1087, 1088, 1089, 1090, 1091, 1092, 1093, 1094, 1095, 1096, 1097, 1098, 1099, 1100, 1101, 1102, 847, 848, 849, 853, 850, 854, 851, 855, 852, 856, 349, 350, 354, 351, 355, 352, 356, 353, 357, 340, 341, 345, 342, 346, 343, 347, 344, 348] }, Tag { name: "minecraft:beds".to_string(), entries: vec![124, 125, 121, 122, 119, 117, 123, 113, 118, 115, 112, 111, 116, 120, 110, 114] }, Tag { name: "minecraft:unstable_bottom_center".to_string(), entries: vec![629, 627, 631, 632, 628, 368, 626, 891, 892, 633, 634, 630] }, Tag { name: "minecraft:doors".to_string(), entries: vec![219, 644, 645, 646, 647, 649, 650, 897, 898, 651, 652, 648, 1047, 1048, 1050, 1049, 1051, 1052, 1054, 1053, 259] }, Tag { name: "minecraft:enderman_holdable".to_string(), entries: vec![157, 1161, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 170, 169, 158, 1162, 9, 8, 11, 10, 372, 1119, 1114, 1158, 1120, 59, 37, 39, 40, 171, 172, 176, 278, 280, 359, 295, 360, 874, 873, 880, 865, 864, 867, 279] }, Tag { name: "minecraft:banners".to_string(), entries: vec![561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572, 573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591, 592] }, Tag { name: "minecraft:smelts_to_glass".to_string(), entries: vec![37, 39] }, Tag { name: "minecraft:infiniburn_overworld".to_string(), entries: vec![284, 669] }, Tag { name: "minecraft:flower_pots".to_string(), entries: vec![410, 1163, 1164, 423, 424, 425, 426, 427, 428, 429, 430, 431, 422, 412, 413, 414, 415, 416, 418, 419, 435, 436, 437, 421, 438, 432, 433, 434, 791, 917, 918, 919, 920, 1146, 1147, 420, 417, 411] }, Tag { name: "minecraft:wooden_fences".to_string(), entries: vec![283, 638, 640, 641, 635, 636, 637, 887, 888, 642, 643, 639] }, Tag { name: "minecraft:incorrect_for_iron_tool".to_string(), entries: vec![192, 915, 913, 916, 914] }, Tag { name: "minecraft:enchantment_power_transmitter".to_string(), entries: vec![0, 35, 36, 130, 131, 132, 133, 134, 135, 136, 137, 195, 196, 275, 365, 366, 367, 523, 559, 560, 673, 792, 793, 794, 867, 868, 880, 1113, 1118] }, Tag { name: "minecraft:wall_post_override".to_string(), entries: vec![193, 289, 272, 291, 401, 209, 210, 211, 212, 214, 215, 216, 899, 900, 217, 218, 213, 223, 224, 225, 226, 228, 229, 230, 901, 902, 231, 232, 227, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572, 573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591, 592, 469, 470, 260, 261, 262, 263, 264, 266, 267, 885, 886, 268, 269, 265, 258, 936, 279] }, Tag { name: "minecraft:polar_bears_spawnable_on_alternate".to_string(), entries: vec![276] }, Tag { name: "minecraft:cauldrons".to_string(), entries: vec![386, 387, 388, 389] }, Tag { name: "minecraft:big_dripleaf_placeable".to_string(), entries: vec![280, 1114, 9, 8, 11, 10, 372, 1119, 1120, 59, 207] }, Tag { name: "minecraft:dampens_vibrations".to_string(), entries: vec![140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551] }, Tag { name: "minecraft:overworld_carver_replaceables".to_string(), entries: vec![1, 2, 4, 6, 982, 1121, 9, 8, 11, 10, 372, 1119, 1114, 1158, 1120, 59, 37, 39, 38, 552, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 44, 45, 1009, 1010, 275, 277, 998, 35, 40, 41, 106, 593, 996, 554, 1143, 1144] }, Tag { name: "minecraft:wall_hanging_signs".to_string(), entries: vec![245, 246, 247, 248, 249, 250, 251, 252, 254, 255, 253, 256] }, Tag { name: "minecraft:sculk_replaceable".to_string(), entries: vec![1, 2, 4, 6, 982, 1121, 9, 8, 11, 10, 372, 1119, 1114, 1158, 1120, 59, 552, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 873, 864, 284, 287, 922, 37, 39, 40, 285, 286, 996, 1142, 280, 1104, 392, 593, 106] }, Tag { name: "minecraft:copper".to_string(), entries: vec![1005, 1006, 1007, 1008, 1031, 1033, 1032, 1034] }, Tag { name: "minecraft:snow_layer_cannot_survive_on".to_string(), entries: vec![276, 554, 522] }, Tag { name: "minecraft:ancient_city_replaceable".to_string(), entries: vec![1121, 1134, 1130, 1136, 1132, 1135, 1133, 1137, 1122, 1139, 1140, 147] }, Tag { name: "minecraft:signs".to_string(), entries: vec![209, 210, 211, 212, 214, 215, 216, 899, 900, 217, 218, 213, 223, 224, 225, 226, 228, 229, 230, 901, 902, 231, 232, 227] }, Tag { name: "minecraft:mangrove_logs_can_grow_through".to_string(), entries: vec![1120, 59, 58, 96, 57, 33, 1110, 365] }, Tag { name: "minecraft:wither_summon_base_blocks".to_string(), entries: vec![285, 286] }, Tag { name: "minecraft:dripstone_replaceable_blocks".to_string(), entries: vec![1, 2, 4, 6, 982, 1121] }, Tag { name: "minecraft:hoglin_repellents".to_string(), entries: vec![865, 918, 294, 916] }, Tag { name: "minecraft:fire".to_string(), entries: vec![195, 196] }, Tag { name: "minecraft:needs_diamond_tool".to_string(), entries: vec![192, 915, 913, 916, 914] }, Tag { name: "minecraft:base_stone_nether".to_string(), entries: vec![284, 287, 922] }, Tag { name: "minecraft:wall_signs".to_string(), entries: vec![223, 224, 225, 226, 228, 229, 230, 901, 902, 231, 232, 227] }, Tag { name: "minecraft:incorrect_for_stone_tool".to_string(), entries: vec![192, 915, 913, 916, 914, 204, 202, 203, 397, 398, 402, 173, 1145, 42, 43, 270, 271] }, Tag { name: "minecraft:mineable/shovel".to_string(), entries: vec![280, 9, 10, 11, 207, 8, 40, 372, 37, 39, 277, 275, 285, 664, 286, 1119, 59, 1120, 38, 41, 724, 725, 726, 727, 728, 729, 730, 731, 732, 733, 734, 735, 736, 737, 738, 739] }, Tag { name: "minecraft:bats_spawnable_on".to_string(), entries: vec![1, 2, 4, 6, 982, 1121] }, Tag { name: "minecraft:trapdoors".to_string(), entries: vec![319, 317, 321, 322, 318, 315, 316, 889, 890, 323, 324, 320, 524, 1055, 1056, 1058, 1057, 1059, 1060, 1062, 1061] }, Tag { name: "minecraft:redstone_ores".to_string(), entries: vec![270, 271] }, Tag { name: "minecraft:cherry_logs".to_string(), entries: vec![54, 76, 65, 84] }, Tag { name: "minecraft:fall_damage_resetting".to_string(), entries: vec![220, 365, 835, 876, 877, 878, 879, 1105, 1106, 859, 129] }, Tag { name: "minecraft:bars".to_string(), entries: vec![340, 341, 345, 342, 346, 343, 347, 344, 348] }, Tag { name: "minecraft:does_not_block_hoppers".to_string(), entries: vec![909, 910] }, Tag { name: "minecraft:soul_speed_blocks".to_string(), entries: vec![285, 286] }, Tag { name: "minecraft:geode_invalid_blocks".to_string(), entries: vec![34, 35, 36, 276, 554, 787] }, Tag { name: "minecraft:all_hanging_signs".to_string(), entries: vec![233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 254, 255, 253, 256] }, Tag { name: "minecraft:leaves".to_string(), entries: vec![91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93] }, Tag { name: "minecraft:walls".to_string(), entries: vec![408, 409, 822, 823, 824, 825, 826, 827, 829, 830, 831, 832, 833, 834, 924, 932, 938, 1125, 1129, 1133, 1137, 828, 985, 989, 994, 378] }, Tag { name: "minecraft:coral_blocks".to_string(), entries: vec![751, 752, 753, 754, 755] }, Tag { name: "minecraft:cave_vines".to_string(), entries: vec![1106, 1105] }, Tag { name: "minecraft:wither_immune".to_string(), entries: vec![522, 34, 390, 391, 665, 406, 666, 667, 903, 904, 156, 523, 1152, 905, 906] }, Tag { name: "minecraft:acacia_logs".to_string(), entries: vec![53, 75, 64, 83] }, Tag { name: "minecraft:incorrect_for_netherite_tool".to_string(), entries: vec![] }, Tag { name: "minecraft:dragon_transparent".to_string(), entries: vec![523, 195, 196] }, Tag { name: "minecraft:underwater_bonemeals".to_string(), entries: vec![136, 761, 762, 763, 764, 765, 771, 772, 773, 774, 775, 781, 782, 783, 784, 785] }, Tag { name: "minecraft:chains".to_string(), entries: vec![349, 350, 354, 351, 355, 352, 356, 353, 357] }, Tag { name: "minecraft:impermeable".to_string(), entries: vec![101, 299, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 311, 312, 313, 314, 997, 522] }, Tag { name: "minecraft:sand".to_string(), entries: vec![37, 39, 38] }, Tag { name: "minecraft:copper_ores".to_string(), entries: vec![1009, 1010] }, Tag { name: "minecraft:small_dripleaf_placeable".to_string(), entries: vec![280, 1114] }, Tag { name: "minecraft:logs_that_burn".to_string(), entries: vec![55, 77, 66, 85, 56, 20, 67, 86, 49, 71, 68, 79, 53, 75, 64, 83, 51, 73, 62, 81, 52, 74, 63, 82, 50, 72, 61, 80, 57, 78, 69, 87, 54, 76, 65, 84] }, Tag { name: "minecraft:lightning_rods".to_string(), entries: vec![1095, 1096, 1097, 1098, 1099, 1100, 1101, 1102] }, Tag { name: "minecraft:mineable/hoe".to_string(), entries: vec![91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93, 670, 866, 535, 742, 908, 875, 99, 100, 999, 1000, 1114, 1110, 1158, 1159, 1001, 1003, 1002, 1004] }, Tag { name: "minecraft:needs_iron_tool".to_string(), entries: vec![204, 202, 203, 397, 398, 402, 173, 1145, 42, 43, 270, 271] }, Tag { name: "minecraft:campfires".to_string(), entries: vec![857, 858] }, Tag { name: "minecraft:infiniburn_nether".to_string(), entries: vec![284, 669] }, Tag { name: "minecraft:wooden_slabs".to_string(), entries: vec![597, 598, 599, 600, 601, 603, 604, 883, 884, 605, 606, 602] }, Tag { name: "minecraft:coal_ores".to_string(), entries: vec![46, 47] }, Tag { name: "minecraft:small_flowers".to_string(), entries: vec![157, 1161, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 170, 169, 158, 1162] }, Tag { name: "minecraft:azalea_root_replaceable".to_string(), entries: vec![1, 2, 4, 6, 982, 1121, 9, 8, 11, 10, 372, 1119, 1114, 1158, 1120, 59, 552, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 39, 280, 40, 37, 277, 998] }, Tag { name: "minecraft:wooden_trapdoors".to_string(), entries: vec![319, 317, 321, 322, 318, 315, 316, 889, 890, 323, 324, 320] }, Tag { name: "minecraft:wooden_shelves".to_string(), entries: vec![179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190] }, Tag { name: "minecraft:wolves_spawnable_on".to_string(), entries: vec![8, 275, 277, 10, 11] }, Tag { name: "minecraft:foxes_spawnable_on".to_string(), entries: vec![8, 275, 277, 11, 10] }, Tag { name: "minecraft:stairs".to_string(), entries: vec![199, 403, 404, 405, 514, 516, 517, 893, 894, 518, 519, 515, 520, 222, 396, 382, 370, 369, 658, 479, 596, 529, 528, 530, 795, 796, 797, 798, 799, 800, 801, 802, 803, 804, 805, 806, 807, 808, 923, 931, 934, 1123, 1127, 1131, 1135, 1023, 1024, 1025, 1026, 1040, 1041, 1042, 1039, 371, 984, 988, 993, 376] }, Tag { name: "minecraft:edible_for_sheep".to_string(), entries: vec![130, 134, 135, 131] }, Tag { name: "minecraft:logs".to_string(), entries: vec![55, 77, 66, 85, 56, 20, 67, 86, 49, 71, 68, 79, 53, 75, 64, 83, 51, 73, 62, 81, 52, 74, 63, 82, 50, 72, 61, 80, 57, 78, 69, 87, 54, 76, 65, 84, 869, 870, 871, 872, 860, 861, 862, 863] }, Tag { name: "minecraft:all_signs".to_string(), entries: vec![209, 210, 211, 212, 214, 215, 216, 899, 900, 217, 218, 213, 223, 224, 225, 226, 228, 229, 230, 901, 902, 231, 232, 227, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 254, 255, 253, 256] }, Tag { name: "minecraft:azalea_grows_on".to_string(), entries: vec![9, 8, 11, 10, 372, 1119, 1114, 1158, 1120, 59, 37, 39, 38, 552, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 277, 998] }, Tag { name: "minecraft:valid_spawn".to_string(), entries: vec![8, 11] }, Tag { name: "minecraft:mushroom_grow_block".to_string(), entries: vec![372, 11, 873, 864] }, Tag { name: "minecraft:wooden_doors".to_string(), entries: vec![219, 644, 645, 646, 647, 649, 650, 897, 898, 651, 652, 648] }, Tag { name: "minecraft:sniffer_egg_hatch_boost".to_string(), entries: vec![1114] }, Tag { name: "minecraft:infiniburn_end".to_string(), entries: vec![284, 669, 34] }, Tag { name: "minecraft:standing_signs".to_string(), entries: vec![209, 210, 211, 212, 214, 215, 216, 899, 900, 217, 218, 213] }, Tag { name: "minecraft:emerald_ores".to_string(), entries: vec![397, 398] }, Tag { name: "minecraft:crimson_stems".to_string(), entries: vec![869, 870, 871, 872] }, Tag { name: "minecraft:needs_stone_tool".to_string(), entries: vec![174, 1143, 44, 45, 104, 102, 103, 1005, 1144, 1009, 1010, 1030, 1026, 1014, 1007, 1028, 1024, 1012, 1008, 1027, 1023, 1011, 1006, 1029, 1025, 1013, 1031, 1046, 1042, 1038, 1032, 1044, 1040, 1036, 1033, 1045, 1041, 1037, 1034, 1043, 1039, 1035, 1154, 1018, 1017, 1016, 1015, 1022, 1021, 1020, 1019, 1063, 1064, 1065, 1066, 1067, 1068, 1069, 1070, 1071, 1072, 1073, 1074, 1075, 1076, 1077, 1078, 1055, 1056, 1058, 1057, 1059, 1060, 1062, 1061, 1079, 1080, 1081, 1082, 1083, 1084, 1085, 1086, 1095, 1096, 1097, 1098, 1099, 1100, 1101, 1102] }, Tag { name: "minecraft:replaceable_by_trees".to_string(), entries: vec![91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93, 157, 1161, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 170, 169, 158, 1162, 1159, 130, 131, 132, 365, 366, 555, 556, 557, 558, 559, 560, 1118, 662, 35, 136, 137, 133, 1165, 867, 868, 880, 1113, 134, 135] }, Tag { name: "minecraft:triggers_ambient_desert_dry_vegetation_block_sounds".to_string(), entries: vec![552, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 37, 39] }, Tag { name: "minecraft:terracotta".to_string(), entries: vec![552, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497] }, Tag { name: "minecraft:wart_blocks".to_string(), entries: vec![670, 866] }, Tag { name: "minecraft:frog_prefer_jump_to".to_string(), entries: vec![373, 1115] }, Tag { name: "minecraft:happy_ghast_avoids".to_string(), entries: vec![859, 278, 169, 669, 195, 1103] }, Tag { name: "minecraft:beacon_base_blocks".to_string(), entries: vec![913, 402, 204, 173, 174] }, Tag { name: "minecraft:frogs_spawnable_on".to_string(), entries: vec![8, 1120, 58, 59] }, Tag { name: "minecraft:shulker_boxes".to_string(), entries: vec![675, 691, 687, 688, 685, 683, 689, 679, 684, 681, 678, 677, 682, 686, 690, 676, 680] }, Tag { name: "minecraft:blocks_wind_charge_explosions".to_string(), entries: vec![522, 34] }, Tag { name: "minecraft:anvil".to_string(), entries: vec![465, 466, 467] }, Tag { name: "minecraft:birch_logs".to_string(), entries: vec![51, 73, 62, 81] }, Tag { name: "minecraft:moss_replaceable".to_string(), entries: vec![1, 2, 4, 6, 982, 1121, 1106, 1105, 9, 8, 11, 10, 372, 1119, 1114, 1158, 1120, 59] }, Tag { name: "minecraft:wall_corals".to_string(), entries: vec![781, 782, 783, 784, 785] }, Tag { name: "minecraft:maintains_farmland".to_string(), entries: vec![363, 361, 364, 362, 663, 439, 440, 660, 158, 661, 206] }, Tag { name: "minecraft:air".to_string(), entries: vec![0, 792, 793] }, Tag { name: "minecraft:lanterns".to_string(), entries: vec![847, 848, 849, 853, 850, 854, 851, 855, 852, 856] }, Tag { name: "minecraft:bee_attractive".to_string(), entries: vec![157, 1161, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 170, 169, 158, 555, 556, 558, 557, 662, 98, 1109, 33, 93, 1111, 1112, 655, 1107, 279] }, Tag { name: "minecraft:iron_ores".to_string(), entries: vec![44, 45] }, Tag { name: "minecraft:oak_logs".to_string(), entries: vec![49, 71, 68, 79] }, Tag { name: "minecraft:piglin_repellents".to_string(), entries: vec![196, 289, 848, 290, 858] }, Tag { name: "minecraft:incorrect_for_wooden_tool".to_string(), entries: vec![192, 915, 913, 916, 914, 204, 202, 203, 397, 398, 402, 173, 1145, 42, 43, 270, 271, 174, 1143, 44, 45, 104, 102, 103, 1005, 1144, 1009, 1010, 1030, 1026, 1014, 1007, 1028, 1024, 1012, 1008, 1027, 1023, 1011, 1006, 1029, 1025, 1013, 1031, 1046, 1042, 1038, 1032, 1044, 1040, 1036, 1033, 1045, 1041, 1037, 1034, 1043, 1039, 1035, 1154, 1018, 1017, 1016, 1015, 1022, 1021, 1020, 1019, 1063, 1064, 1065, 1066, 1067, 1068, 1069, 1070, 1071, 1072, 1073, 1074, 1075, 1076, 1077, 1078, 1055, 1056, 1058, 1057, 1059, 1060, 1062, 1061, 1079, 1080, 1081, 1082, 1083, 1084, 1085, 1086, 1095, 1096, 1097, 1098, 1099, 1100, 1101, 1102] }, Tag { name: "minecraft:mooshrooms_spawnable_on".to_string(), entries: vec![372] }, Tag { name: "minecraft:portals".to_string(), entries: vec![294, 390, 665] }, Tag { name: "minecraft:bamboo_plantable_on".to_string(), entries: vec![37, 39, 38, 9, 8, 11, 10, 372, 1119, 1114, 1158, 1120, 59, 790, 789, 40, 41] }, Tag { name: "minecraft:sword_efficient".to_string(), entries: vec![91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93, 365, 366, 359, 295, 296, 360, 395, 1115, 1116, 654, 655] }, Tag { name: "minecraft:pressure_plates".to_string(), entries: vec![469, 470, 260, 261, 262, 263, 264, 266, 267, 885, 886, 268, 269, 265, 258, 936] }, Tag { name: "minecraft:mangrove_logs".to_string(), entries: vec![57, 78, 69, 87] }, Tag { name: "minecraft:snow_layer_can_survive_on".to_string(), entries: vec![911, 285, 1120] }, Tag { name: "minecraft:jungle_logs".to_string(), entries: vec![52, 74, 63, 82] }, Tag { name: "minecraft:vibration_resonators".to_string(), entries: vec![976] }, Tag { name: "minecraft:wooden_stairs".to_string(), entries: vec![199, 403, 404, 405, 514, 516, 517, 893, 894, 518, 519, 515] }, Tag { name: "minecraft:spruce_logs".to_string(), entries: vec![50, 72, 61, 80] }, Tag { name: "minecraft:base_stone_overworld".to_string(), entries: vec![1, 2, 4, 6, 982, 1121] }, Tag { name: "minecraft:wooden_buttons".to_string(), entries: vec![441, 442, 443, 444, 445, 447, 448, 895, 896, 449, 450, 446] }, Tag { name: "minecraft:axolotls_spawnable_on".to_string(), entries: vec![280] }, Tag { name: "minecraft:stone_bricks".to_string(), entries: vec![325, 326, 327, 328] }, Tag { name: "minecraft:mineable/axe".to_string(), entries: vec![109, 790, 837, 909, 910, 1116, 1115, 177, 337, 857, 840, 295, 200, 655, 654, 395, 907, 205, 472, 841, 366, 296, 282, 220, 843, 836, 360, 339, 359, 338, 844, 858, 468, 365, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572, 573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591, 592, 629, 627, 631, 632, 628, 368, 626, 891, 892, 633, 634, 630, 55, 77, 66, 85, 56, 20, 67, 86, 49, 71, 68, 79, 53, 75, 64, 83, 51, 73, 62, 81, 52, 74, 63, 82, 50, 72, 61, 80, 57, 78, 69, 87, 54, 76, 65, 84, 869, 870, 871, 872, 860, 861, 862, 863, 13, 14, 15, 16, 17, 19, 21, 881, 882, 22, 23, 18, 209, 210, 211, 212, 214, 215, 216, 899, 900, 217, 218, 213, 223, 224, 225, 226, 228, 229, 230, 901, 902, 231, 232, 227, 441, 442, 443, 444, 445, 447, 448, 895, 896, 449, 450, 446, 219, 644, 645, 646, 647, 649, 650, 897, 898, 651, 652, 648, 283, 638, 640, 641, 635, 636, 637, 887, 888, 642, 643, 639, 260, 261, 262, 263, 264, 266, 267, 885, 886, 268, 269, 265, 597, 598, 599, 600, 601, 603, 604, 883, 884, 605, 606, 602, 199, 403, 404, 405, 514, 516, 517, 893, 894, 518, 519, 515, 319, 317, 321, 322, 318, 315, 316, 889, 890, 323, 324, 320, 58, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 254, 255, 253, 256, 24, 607, 520, 60, 70, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 198] }, Tag { name: "minecraft:copper_chests".to_string(), entries: vec![1079, 1080, 1081, 1082, 1083, 1084, 1085, 1086] }, Tag { name: "minecraft:incorrect_for_copper_tool".to_string(), entries: vec![192, 915, 913, 916, 914, 204, 202, 203, 397, 398, 402, 173, 1145, 42, 43, 270, 271] }, Tag { name: "minecraft:slabs".to_string(), entries: vec![597, 598, 599, 600, 601, 603, 604, 883, 884, 605, 606, 602, 607, 608, 609, 615, 610, 621, 618, 619, 614, 613, 617, 612, 531, 532, 533, 809, 810, 811, 812, 813, 814, 815, 816, 817, 818, 819, 820, 821, 611, 620, 925, 930, 935, 1124, 1128, 1132, 1136, 1044, 1045, 1046, 1027, 1028, 1029, 1030, 1043, 616, 983, 987, 992, 377] }, Tag { name: "minecraft:animals_spawnable_on".to_string(), entries: vec![8] }, Tag { name: "minecraft:guarded_by_piglins".to_string(), entries: vec![1079, 1080, 1081, 1082, 1083, 1084, 1085, 1086, 173, 837, 200, 399, 933, 468, 1145, 675, 691, 687, 688, 685, 683, 689, 679, 684, 681, 678, 677, 682, 686, 690, 676, 680, 42, 48, 43] }, Tag { name: "minecraft:replaceable_by_mushrooms".to_string(), entries: vec![91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93, 157, 1161, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 170, 169, 158, 1162, 1159, 130, 131, 132, 365, 366, 555, 556, 557, 558, 559, 560, 1118, 662, 35, 136, 137, 171, 172, 337, 338, 867, 868, 880, 1113, 134, 135, 133, 1165] }, Tag { name: "minecraft:nether_carver_replaceables".to_string(), entries: vec![1, 2, 4, 6, 982, 1121, 284, 287, 922, 9, 8, 11, 10, 372, 1119, 1114, 1158, 1120, 59, 873, 864, 670, 866, 285, 286] }, Tag { name: "minecraft:stone_ore_replaceables".to_string(), entries: vec![1, 2, 4, 6] }, Tag { name: "minecraft:buttons".to_string(), entries: vec![441, 442, 443, 444, 445, 447, 448, 895, 896, 449, 450, 446, 274, 937] }, Tag { name: "minecraft:corals".to_string(), entries: vec![761, 762, 763, 764, 765, 771, 772, 773, 774, 775] }, Tag { name: "minecraft:flowers".to_string(), entries: vec![157, 1161, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 170, 169, 158, 1162, 555, 556, 558, 557, 662, 98, 1109, 33, 93, 1111, 1112, 655, 1107, 279] }, Tag { name: "minecraft:combination_step_sound_blocks".to_string(), entries: vec![536, 537, 538, 539, 540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 1110, 1159, 275, 868, 867, 880, 367] }, Tag { name: "minecraft:rabbits_spawnable_on".to_string(), entries: vec![8, 275, 277, 37] }, Tag { name: "minecraft:planks".to_string(), entries: vec![13, 14, 15, 16, 17, 19, 21, 881, 882, 22, 23, 18] }, Tag { name: "minecraft:stone_buttons".to_string(), entries: vec![274, 937] }, Tag { name: "minecraft:rails".to_string(), entries: vec![221, 126, 127, 480] }, Tag { name: "minecraft:diamond_ores".to_string(), entries: vec![202, 203] }, Tag { name: "minecraft:badlands_terracotta".to_string(), entries: vec![552, 482, 486, 483, 496, 494, 490] }, Tag { name: "minecraft:overworld_natural_logs".to_string(), entries: vec![53, 51, 49, 52, 50, 55, 56, 57, 54] }, Tag { name: "minecraft:deepslate_ore_replaceables".to_string(), entries: vec![1121, 982] }, Tag { name: "minecraft:strider_warm_blocks".to_string(), entries: vec![36] }, Tag { name: "minecraft:fence_gates".to_string(), entries: vec![629, 627, 631, 632, 628, 368, 626, 891, 892, 633, 634, 630] }, Tag { name: "minecraft:incorrect_for_gold_tool".to_string(), entries: vec![192, 915, 913, 916, 914, 204, 202, 203, 397, 398, 402, 173, 1145, 42, 43, 270, 271, 174, 1143, 44, 45, 104, 102, 103, 1005, 1144, 1009, 1010, 1030, 1026, 1014, 1007, 1028, 1024, 1012, 1008, 1027, 1023, 1011, 1006, 1029, 1025, 1013, 1031, 1046, 1042, 1038, 1032, 1044, 1040, 1036, 1033, 1045, 1041, 1037, 1034, 1043, 1039, 1035, 1154, 1018, 1017, 1016, 1015, 1022, 1021, 1020, 1019, 1063, 1064, 1065, 1066, 1067, 1068, 1069, 1070, 1071, 1072, 1073, 1074, 1075, 1076, 1077, 1078, 1055, 1056, 1058, 1057, 1059, 1060, 1062, 1061, 1079, 1080, 1081, 1082, 1083, 1084, 1085, 1086, 1095, 1096, 1097, 1098, 1099, 1100, 1101, 1102] }, Tag { name: "minecraft:bee_growables".to_string(), entries: vec![663, 439, 440, 206, 364, 363, 660, 661, 859, 1105, 1106] }, Tag { name: "minecraft:wooden_pressure_plates".to_string(), entries: vec![260, 261, 262, 263, 264, 266, 267, 885, 886, 268, 269, 265] }, Tag { name: "minecraft:armadillo_spawnable_on".to_string(), entries: vec![8, 552, 482, 486, 483, 496, 494, 490, 39, 10] }, Tag { name: "minecraft:candles".to_string(), entries: vec![942, 943, 944, 945, 946, 947, 948, 949, 950, 951, 952, 953, 954, 955, 956, 957, 958] }, Tag { name: "minecraft:stone_pressure_plates".to_string(), entries: vec![258, 936] }, Tag { name: "minecraft:nylium".to_string(), entries: vec![873, 864] }, Tag { name: "minecraft:snow".to_string(), entries: vec![275, 277, 998] }, Tag { name: "minecraft:gold_ores".to_string(), entries: vec![42, 48, 43] }, Tag { name: "minecraft:incorrect_for_diamond_tool".to_string(), entries: vec![] }, Tag { name: "minecraft:completes_find_tree_tutorial".to_string(), entries: vec![55, 77, 66, 85, 56, 20, 67, 86, 49, 71, 68, 79, 53, 75, 64, 83, 51, 73, 62, 81, 52, 74, 63, 82, 50, 72, 61, 80, 57, 78, 69, 87, 54, 76, 65, 84, 869, 870, 871, 872, 860, 861, 862, 863, 91, 88, 89, 95, 94, 92, 90, 97, 98, 96, 93, 670, 866] }, Tag { name: "minecraft:camel_sand_step_sound_blocks".to_string(), entries: vec![37, 39, 38, 724, 725, 726, 727, 728, 729, 730, 731, 732, 733, 734, 735, 736, 737, 738, 739] }, Tag { name: "minecraft:replaceable".to_string(), entries: vec![0, 35, 36, 130, 131, 132, 133, 134, 135, 136, 137, 195, 196, 275, 365, 366, 367, 523, 559, 560, 673, 792, 793, 794, 867, 868, 880, 1113, 1118] }, Tag { name: "minecraft:dirt".to_string(), entries: vec![9, 8, 11, 10, 372, 1119, 1114, 1158, 1120, 59] }, Tag { name: "minecraft:candle_cakes".to_string(), entries: vec![959, 960, 961, 962, 963, 964, 965, 966, 967, 968, 969, 970, 971, 972, 973, 974, 975] }]), ("minecraft:entity_type".to_string(), vec![Tag { name: "minecraft:can_equip_harness".to_string(), entries: vec![57] }, Tag { name: "minecraft:axolotl_always_hostiles".to_string(), entries: vec![37, 62, 39] }, Tag { name: "minecraft:freeze_immune_entity_types".to_string(), entries: vec![125, 101, 118, 142] }, Tag { name: "minecraft:undead".to_string(), entries: vec![112, 125, 143, 113, 16, 148, 147, 149, 150, 146, 37, 66, 142, 96] }, Tag { name: "minecraft:illager".to_string(), entries: vec![45, 67, 100, 137] }, Tag { name: "minecraft:immune_to_oozing".to_string(), entries: vec![114] }, Tag { name: "minecraft:sensitive_to_bane_of_arthropods".to_string(), entries: vec![11, 41, 111, 121, 21] }, Tag { name: "minecraft:boat".to_string(), entries: vec![87, 122, 12, 73, 0, 22, 32, 92, 80, 9] }, Tag { name: "minecraft:cannot_be_pushed_onto_boats".to_string(), entries: vec![151, 39, 26, 104, 107, 133, 34, 124, 60, 127, 30] }, Tag { name: "minecraft:frog_food".to_string(), entries: vec![114, 79] }, Tag { name: "minecraft:can_equip_saddle".to_string(), entries: vec![65, 113, 148, 35, 86, 97, 126, 19] }, Tag { name: "minecraft:redirectable_projectile".to_string(), entries: vec![51, 140, 18] }, Tag { name: "minecraft:fall_damage_immune".to_string(), entries: vec![27, 69, 118, 109, 2, 10, 11, 14, 20, 25, 56, 57, 96, 79, 89, 95, 142, 17] }, Tag { name: "minecraft:raiders".to_string(), entries: vec![45, 100, 106, 137, 67, 141] }, Tag { name: "minecraft:arthropod".to_string(), entries: vec![11, 41, 111, 121, 21] }, Tag { name: "minecraft:inverted_healing_and_harm".to_string(), entries: vec![112, 125, 143, 113, 16, 148, 147, 149, 150, 146, 37, 66, 142, 96] }, Tag { name: "minecraft:powder_snow_walkable_mobs".to_string(), entries: vec![105, 41, 111, 53] }, Tag { name: "minecraft:axolotl_hunt_targets".to_string(), entries: vec![133, 104, 107, 26, 124, 60, 127] }, Tag { name: "minecraft:immune_to_infested".to_string(), entries: vec![111] }, Tag { name: "minecraft:zombies".to_string(), entries: vec![148, 147, 149, 150, 146, 37, 66] }, Tag { name: "minecraft:sensitive_to_smite".to_string(), entries: vec![112, 125, 143, 113, 16, 148, 147, 149, 150, 146, 37, 66, 142, 96] }, Tag { name: "minecraft:dismounts_underwater".to_string(), entries: vec![19, 25, 35, 57, 65, 77, 86, 97, 106, 121, 126, 131, 148] }, Tag { name: "minecraft:can_wear_horse_armor".to_string(), entries: vec![65] }, Tag { name: "minecraft:impact_projectiles".to_string(), entries: vec![6, 120, 52, 117, 51, 115, 38, 132, 36, 144, 140, 18] }, Tag { name: "minecraft:beehive_inhabitors".to_string(), entries: vec![11] }, Tag { name: "minecraft:not_scary_for_pufferfish".to_string(), entries: vec![134, 62, 39, 26, 104, 107, 133, 34, 124, 60, 127] }, Tag { name: "minecraft:non_controlling_rider".to_string(), entries: vec![114, 79] }, Tag { name: "minecraft:skeletons".to_string(), entries: vec![112, 125, 143, 113, 16] }, Tag { name: "minecraft:can_turn_in_boats".to_string(), entries: vec![17] }, Tag { name: "minecraft:arrows".to_string(), entries: vec![6, 120] }, Tag { name: "minecraft:can_breathe_under_water".to_string(), entries: vec![112, 125, 143, 113, 16, 148, 147, 149, 150, 146, 37, 66, 142, 96, 7, 54, 62, 39, 134, 60, 26, 104, 107, 124, 133, 127, 5, 27] }, Tag { name: "minecraft:followable_friendly_mobs".to_string(), entries: vec![4, 11, 19, 20, 25, 29, 35, 53, 61, 57, 65, 113, 77, 86, 89, 94, 95, 97, 101, 105, 108, 116, 126, 136, 145] }, Tag { name: "minecraft:freeze_hurts_extra_types".to_string(), entries: vec![126, 14, 79] }, Tag { name: "minecraft:wither_friends".to_string(), entries: vec![112, 125, 143, 113, 16, 148, 147, 149, 150, 146, 37, 66, 142, 96] }, Tag { name: "minecraft:no_anger_from_wind_charge".to_string(), entries: vec![17, 112, 16, 125, 147, 66, 121, 21, 114] }, Tag { name: "minecraft:candidate_for_iron_golem_gift".to_string(), entries: vec![136, 27] }, Tag { name: "minecraft:aquatic".to_string(), entries: vec![134, 7, 62, 39, 26, 104, 107, 133, 34, 124, 60, 127] }, Tag { name: "minecraft:illager_friends".to_string(), entries: vec![45, 67, 100, 137] }, Tag { name: "minecraft:ignores_poison_and_regen".to_string(), entries: vec![112, 125, 143, 113, 16, 148, 147, 149, 150, 146, 37, 66, 142, 96] }, Tag { name: "minecraft:deflects_projectiles".to_string(), entries: vec![17] }, Tag { name: "minecraft:sensitive_to_impaling".to_string(), entries: vec![134, 7, 62, 39, 26, 104, 107, 133, 34, 124, 60, 127] }, Tag { name: "minecraft:accepts_iron_golem_gift".to_string(), entries: vec![27] }]), ("minecraft:fluid".to_string(), vec![Tag { name: "minecraft:lava".to_string(), entries: vec![4, 3] }, Tag { name: "minecraft:water".to_string(), entries: vec![2, 1] }]), ("minecraft:point_of_interest_type".to_string(), vec![Tag { name: "minecraft:bee_home".to_string(), entries: vec![15, 16] }, Tag { name: "minecraft:acquirable_job_site".to_string(), entries: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] }, Tag { name: "minecraft:village".to_string(), entries: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14] }]), ("minecraft:enchantment".to_string(), vec![Tag { name: "minecraft:treasure".to_string(), entries: vec![2, 40, 37, 35, 14, 22, 41] }, Tag { name: "minecraft:double_trade_price".to_string(), entries: vec![2, 40, 37, 35, 14, 22, 41] }, Tag { name: "minecraft:tooltip_order".to_string(), entries: vec![2, 40, 31, 5, 41, 14, 32, 34, 1, 15, 25, 6, 4, 24, 36, 23, 10, 12, 17, 28, 27, 3, 11, 26, 9, 13, 18, 33, 20, 8, 29, 21, 30, 0, 35, 37, 7, 38, 19, 39, 16, 22] }, Tag { name: "minecraft:prevents_decorated_pot_shattering".to_string(), entries: vec![33] }, Tag { name: "minecraft:on_traded_equipment".to_string(), entries: vec![27, 11, 9, 3, 26, 30, 0, 38, 7, 32, 34, 1, 17, 10, 18, 36, 8, 33, 39, 13, 25, 28, 12, 16, 20, 21, 19, 15, 31, 5, 23, 29, 24, 6, 4] }, Tag { name: "minecraft:tradeable".to_string(), entries: vec![27, 11, 9, 3, 26, 30, 0, 38, 7, 32, 34, 1, 17, 10, 18, 36, 8, 33, 39, 13, 25, 28, 12, 16, 20, 21, 19, 15, 31, 5, 23, 29, 24, 6, 4, 2, 40, 14, 22] }, Tag { name: "minecraft:on_random_loot".to_string(), entries: vec![27, 11, 9, 3, 26, 30, 0, 38, 7, 32, 34, 1, 17, 10, 18, 36, 8, 33, 39, 13, 25, 28, 12, 16, 20, 21, 19, 15, 31, 5, 23, 29, 24, 6, 4, 2, 40, 14, 22] }, Tag { name: "minecraft:exclusive_set/bow".to_string(), entries: vec![16, 22] }, Tag { name: "minecraft:prevents_infested_spawns".to_string(), entries: vec![33] }, Tag { name: "minecraft:prevents_bee_spawns_when_mining".to_string(), entries: vec![33] }, Tag { name: "minecraft:non_treasure".to_string(), entries: vec![27, 11, 9, 3, 26, 30, 0, 38, 7, 32, 34, 1, 17, 10, 18, 36, 8, 33, 39, 13, 25, 28, 12, 16, 20, 21, 19, 15, 31, 5, 23, 29, 24, 6, 4] }, Tag { name: "minecraft:exclusive_set/riptide".to_string(), entries: vec![19, 5] }, Tag { name: "minecraft:exclusive_set/damage".to_string(), entries: vec![32, 34, 1, 15, 6, 4] }, Tag { name: "minecraft:on_mob_spawn_equipment".to_string(), entries: vec![27, 11, 9, 3, 26, 30, 0, 38, 7, 32, 34, 1, 17, 10, 18, 36, 8, 33, 39, 13, 25, 28, 12, 16, 20, 21, 19, 15, 31, 5, 23, 29, 24, 6, 4] }, Tag { name: "minecraft:smelts_loot".to_string(), entries: vec![10] }, Tag { name: "minecraft:exclusive_set/crossbow".to_string(), entries: vec![23, 24] }, Tag { name: "minecraft:exclusive_set/armor".to_string(), entries: vec![27, 3, 11, 26] }, Tag { name: "minecraft:exclusive_set/boots".to_string(), entries: vec![14, 7] }, Tag { name: "minecraft:in_enchanting_table".to_string(), entries: vec![27, 11, 9, 3, 26, 30, 0, 38, 7, 32, 34, 1, 17, 10, 18, 36, 8, 33, 39, 13, 25, 28, 12, 16, 20, 21, 19, 15, 31, 5, 23, 29, 24, 6, 4] }, Tag { name: "minecraft:prevents_ice_melting".to_string(), entries: vec![33] }, Tag { name: "minecraft:curse".to_string(), entries: vec![2, 40] }, Tag { name: "minecraft:exclusive_set/mining".to_string(), entries: vec![13, 33] }]), ("minecraft:painting_variant".to_string(), vec![Tag { name: "minecraft:placeable".to_string(), entries: vec![24, 1, 0, 2, 5, 32, 47, 35, 12, 37, 42, 13, 46, 22, 26, 8, 40, 45, 39, 50, 19, 33, 31, 7, 38, 15, 4, 23, 27, 36, 44, 3, 6, 9, 10, 11, 17, 18, 20, 25, 28, 29, 30, 34, 41, 43, 14] }]), ("minecraft:game_event".to_string(), vec![Tag { name: "minecraft:vibrations".to_string(), entries: vec![1, 2, 3, 5, 6, 7, 8, 0, 4, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 24, 25, 26, 27, 28, 32, 33, 34, 35, 36, 38, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 23] }, Tag { name: "minecraft:ignore_vibrations_sneaking".to_string(), entries: vec![26, 36, 41, 42, 29, 28] }, Tag { name: "minecraft:warden_can_listen".to_string(), entries: vec![1, 2, 3, 5, 6, 7, 8, 0, 4, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 24, 25, 26, 27, 28, 32, 33, 34, 35, 36, 38, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 39, 37] }, Tag { name: "minecraft:allay_can_listen".to_string(), entries: vec![33] }, Tag { name: "minecraft:shrieker_can_listen".to_string(), entries: vec![37] }]), ("minecraft:damage_type".to_string(), vec![Tag { name: "minecraft:bypasses_effects".to_string(), entries: vec![39] }, Tag { name: "minecraft:is_fall".to_string(), entries: vec![10, 8, 38] }, Tag { name: "minecraft:ignites_armor_stands".to_string(), entries: vec![21, 3] }, Tag { name: "minecraft:witch_resistant_to".to_string(), entries: vec![27, 23, 36, 42] }, Tag { name: "minecraft:avoids_guardian_thorns".to_string(), entries: vec![27, 42, 15, 9, 35, 1] }, Tag { name: "minecraft:burns_armor_stands".to_string(), entries: vec![31] }, Tag { name: "minecraft:bypasses_resistance".to_string(), entries: vec![32, 19] }, Tag { name: "minecraft:panic_environmental_causes".to_string(), entries: vec![2, 17, 20, 21, 24, 25, 31] }, Tag { name: "minecraft:mace_smash".to_string(), entries: vec![26] }, Tag { name: "minecraft:damages_helmet".to_string(), entries: vec![11, 12, 13] }, Tag { name: "minecraft:bypasses_wolf_armor".to_string(), entries: vec![32, 19, 4, 6, 7, 17, 22, 23, 27, 33, 39, 42, 47] }, Tag { name: "minecraft:bypasses_shield".to_string(), entries: vec![31, 22, 4, 6, 16, 18, 47, 5, 39, 10, 8, 17, 38, 27, 23, 32, 19, 36, 33, 2, 3, 7, 11, 13, 20, 21, 24, 25, 41] }, Tag { name: "minecraft:burn_from_stepping".to_string(), entries: vec![3, 20] }, Tag { name: "minecraft:no_impact".to_string(), entries: vec![6] }, Tag { name: "minecraft:is_freezing".to_string(), entries: vec![17] }, Tag { name: "minecraft:is_projectile".to_string(), entries: vec![0, 44, 30, 45, 14, 48, 43, 46] }, Tag { name: "minecraft:bypasses_enchantments".to_string(), entries: vec![36] }, Tag { name: "minecraft:always_most_significant_fall".to_string(), entries: vec![32] }, Tag { name: "minecraft:can_break_armor_stand".to_string(), entries: vec![35, 34, 26] }, Tag { name: "minecraft:bypasses_armor".to_string(), entries: vec![31, 22, 4, 6, 16, 18, 47, 5, 39, 10, 8, 17, 38, 27, 23, 32, 19, 36, 33] }, Tag { name: "minecraft:bypasses_invulnerability".to_string(), entries: vec![32, 19] }, Tag { name: "minecraft:no_knockback".to_string(), entries: vec![9, 35, 1, 21, 25, 31, 24, 20, 22, 4, 6, 39, 2, 10, 8, 16, 32, 18, 27, 47, 5, 7, 41, 17, 38, 33, 19, 3] }, Tag { name: "minecraft:no_anger".to_string(), entries: vec![29] }, Tag { name: "minecraft:is_drowning".to_string(), entries: vec![6] }, Tag { name: "minecraft:panic_causes".to_string(), entries: vec![2, 17, 20, 21, 24, 25, 31, 0, 5, 9, 14, 15, 23, 27, 28, 30, 35, 36, 40, 43, 44, 45, 46, 47, 48, 34, 26] }, Tag { name: "minecraft:always_triggers_silverfish".to_string(), entries: vec![27] }, Tag { name: "minecraft:always_hurts_ender_dragons".to_string(), entries: vec![15, 9, 35, 1] }, Tag { name: "minecraft:is_fire".to_string(), entries: vec![21, 3, 31, 24, 20, 45, 14] }, Tag { name: "minecraft:wither_immune_to".to_string(), entries: vec![6] }, Tag { name: "minecraft:is_explosion".to_string(), entries: vec![15, 9, 35, 1] }, Tag { name: "minecraft:always_kills_armor_stands".to_string(), entries: vec![0, 44, 14, 48, 46] }, Tag { name: "minecraft:is_lightning".to_string(), entries: vec![25] }, Tag { name: "minecraft:is_player_attack".to_string(), entries: vec![34, 26] }]), ("minecraft:worldgen/biome".to_string(), vec![Tag { name: "minecraft:has_structure/nether_fortress".to_string(), entries: vec![34, 49, 7, 59, 2] }, Tag { name: "minecraft:has_structure/ocean_ruin_warm".to_string(), entries: vec![29, 58, 12] }, Tag { name: "minecraft:water_on_map_outlines".to_string(), entries: vec![11, 9, 13, 12, 22, 35, 6, 29, 58, 41, 24, 54, 31] }, Tag { name: "minecraft:is_river".to_string(), entries: vec![41, 24] }, Tag { name: "minecraft:has_structure/mineshaft".to_string(), entries: vec![11, 9, 13, 12, 22, 35, 6, 29, 58, 41, 24, 3, 45, 32, 23, 27, 51, 47, 5, 62, 60, 61, 55, 48, 37, 38, 1, 28, 50, 21, 20, 4, 36, 8, 39, 25, 52, 33, 26, 63, 14, 42, 46, 40, 53, 54, 31, 43, 15, 30] }, Tag { name: "minecraft:has_structure/jungle_temple".to_string(), entries: vec![1, 28] }, Tag { name: "minecraft:has_structure/trial_chambers".to_string(), entries: vec![33, 11, 22, 9, 6, 13, 35, 12, 29, 58, 52, 54, 31, 47, 46, 45, 61, 25, 62, 48, 60, 55, 40, 32, 3, 21, 38, 20, 4, 8, 39, 43, 42, 28, 0, 14, 64, 27, 51, 24, 41, 26, 37, 53, 36, 50, 1, 19, 63, 5, 23, 15, 30] }, Tag { name: "minecraft:more_frequent_drowned_spawns".to_string(), entries: vec![41, 24] }, Tag { name: "minecraft:has_structure/village_desert".to_string(), entries: vec![14] }, Tag { name: "minecraft:is_jungle".to_string(), entries: vec![1, 28, 50] }, Tag { name: "minecraft:spawns_snow_foxes".to_string(), entries: vec![46, 26, 22, 48, 24, 45, 23, 27, 47, 25] }, Tag { name: "minecraft:is_ocean".to_string(), entries: vec![11, 9, 13, 12, 22, 35, 6, 29, 58] }, Tag { name: "minecraft:is_forest".to_string(), entries: vec![21, 20, 4, 36, 8, 39, 25] }, Tag { name: "minecraft:is_overworld".to_string(), entries: vec![33, 11, 22, 9, 6, 13, 35, 12, 29, 58, 52, 54, 31, 47, 46, 45, 61, 25, 62, 48, 60, 55, 40, 32, 3, 21, 38, 20, 4, 8, 39, 43, 42, 28, 0, 14, 64, 27, 51, 24, 41, 26, 37, 53, 36, 50, 1, 19, 63, 5, 23, 15, 30, 10] }, Tag { name: "minecraft:has_structure/village_savanna".to_string(), entries: vec![42] }, Tag { name: "minecraft:without_patrol_spawns".to_string(), entries: vec![33] }, Tag { name: "minecraft:has_structure/village_taiga".to_string(), entries: vec![55] }, Tag { name: "minecraft:allows_tropical_fish_spawns_at_any_height".to_string(), entries: vec![30] }, Tag { name: "minecraft:polar_bears_spawn_on_alternate_blocks".to_string(), entries: vec![22, 11] }, Tag { name: "minecraft:has_closer_water_fog".to_string(), entries: vec![54, 31] }, Tag { name: "minecraft:has_structure/village_plains".to_string(), entries: vec![40, 32] }, Tag { name: "minecraft:has_structure/buried_treasure".to_string(), entries: vec![3, 45] }, Tag { name: "minecraft:has_structure/ocean_ruin_cold".to_string(), entries: vec![22, 6, 35, 11, 9, 13] }, Tag { name: "minecraft:produces_corals_from_bonemeal".to_string(), entries: vec![58] }, Tag { name: "minecraft:snow_golem_melts".to_string(), entries: vec![0, 2, 7, 14, 19, 34, 42, 43, 49, 59, 63, 64] }, Tag { name: "minecraft:has_structure/stronghold".to_string(), entries: vec![33, 11, 22, 9, 6, 13, 35, 12, 29, 58, 52, 54, 31, 47, 46, 45, 61, 25, 62, 48, 60, 55, 40, 32, 3, 21, 38, 20, 4, 8, 39, 43, 42, 28, 0, 14, 64, 27, 51, 24, 41, 26, 37, 53, 36, 50, 1, 19, 63, 5, 23, 15, 30, 10] }, Tag { name: "minecraft:without_wandering_trader_spawns".to_string(), entries: vec![57] }, Tag { name: "minecraft:has_structure/ruined_portal_desert".to_string(), entries: vec![14] }, Tag { name: "minecraft:has_structure/mineshaft_mesa".to_string(), entries: vec![0, 19, 64] }, Tag { name: "minecraft:has_structure/ancient_city".to_string(), entries: vec![10] }, Tag { name: "minecraft:has_structure/ruined_portal_swamp".to_string(), entries: vec![54, 31] }, Tag { name: "minecraft:has_structure/shipwreck".to_string(), entries: vec![11, 9, 13, 12, 22, 35, 6, 29, 58] }, Tag { name: "minecraft:is_savanna".to_string(), entries: vec![42, 43, 63] }, Tag { name: "minecraft:has_structure/swamp_hut".to_string(), entries: vec![54] }, Tag { name: "minecraft:spawns_cold_variant_frogs".to_string(), entries: vec![46, 26, 23, 27, 47, 22, 11, 25, 10, 24, 48, 45, 56, 17, 18, 44, 16] }, Tag { name: "minecraft:has_structure/igloo".to_string(), entries: vec![48, 46, 47] }, Tag { name: "minecraft:has_structure/trail_ruins".to_string(), entries: vec![55, 48, 37, 38, 36, 28] }, Tag { name: "minecraft:has_structure/shipwreck_beached".to_string(), entries: vec![3, 45] }, Tag { name: "minecraft:is_hill".to_string(), entries: vec![62, 60, 61] }, Tag { name: "minecraft:has_structure/ruined_portal_nether".to_string(), entries: vec![34, 49, 7, 59, 2] }, Tag { name: "minecraft:is_end".to_string(), entries: vec![56, 17, 18, 44, 16] }, Tag { name: "minecraft:stronghold_biased_to".to_string(), entries: vec![40, 53, 46, 26, 14, 21, 20, 4, 8, 39, 36, 37, 38, 55, 48, 42, 43, 62, 61, 60, 63, 28, 50, 1, 0, 19, 64, 32, 5, 25, 47, 23, 27, 51, 33, 15, 30] }, Tag { name: "minecraft:without_zombie_sieges".to_string(), entries: vec![33] }, Tag { name: "minecraft:is_beach".to_string(), entries: vec![3, 45] }, Tag { name: "minecraft:has_structure/ruined_portal_standard".to_string(), entries: vec![3, 45, 41, 24, 55, 48, 37, 38, 21, 20, 4, 36, 8, 39, 25, 33, 26, 15, 30, 42, 46, 40, 53] }, Tag { name: "minecraft:has_structure/pillager_outpost".to_string(), entries: vec![14, 40, 42, 46, 55, 32, 23, 27, 51, 47, 5, 25] }, Tag { name: "minecraft:allows_surface_slime_spawns".to_string(), entries: vec![54, 31] }, Tag { name: "minecraft:is_taiga".to_string(), entries: vec![55, 48, 37, 38] }, Tag { name: "minecraft:spawns_warm_variant_frogs".to_string(), entries: vec![14, 58, 1, 28, 50, 42, 43, 63, 34, 49, 7, 59, 2, 0, 19, 64, 31] }, Tag { name: "minecraft:required_ocean_monument_surrounding".to_string(), entries: vec![11, 9, 13, 12, 22, 35, 6, 29, 58, 41, 24] }, Tag { name: "minecraft:mineshaft_blocking".to_string(), entries: vec![10] }, Tag { name: "minecraft:is_mountain".to_string(), entries: vec![32, 23, 27, 51, 47, 5] }, Tag { name: "minecraft:has_structure/village_snowy".to_string(), entries: vec![46] }, Tag { name: "minecraft:has_structure/ocean_monument".to_string(), entries: vec![11, 9, 13, 12] }, Tag { name: "minecraft:is_badlands".to_string(), entries: vec![0, 19, 64] }, Tag { name: "minecraft:spawns_gold_rabbits".to_string(), entries: vec![14] }, Tag { name: "minecraft:has_structure/ruined_portal_jungle".to_string(), entries: vec![1, 28, 50] }, Tag { name: "minecraft:spawns_white_rabbits".to_string(), entries: vec![46, 26, 22, 48, 24, 45, 23, 27, 47, 25] }, Tag { name: "minecraft:plays_underwater_music".to_string(), entries: vec![11, 9, 13, 12, 22, 35, 6, 29, 58, 41, 24] }, Tag { name: "minecraft:has_structure/nether_fossil".to_string(), entries: vec![49] }, Tag { name: "minecraft:has_structure/desert_pyramid".to_string(), entries: vec![14] }, Tag { name: "minecraft:spawns_cold_variant_farm_animals".to_string(), entries: vec![46, 26, 23, 27, 47, 22, 11, 25, 10, 24, 48, 45, 56, 17, 18, 44, 16, 6, 9, 37, 38, 55, 60, 61, 62, 51] }, Tag { name: "minecraft:has_structure/ruined_portal_ocean".to_string(), entries: vec![11, 9, 13, 12, 22, 35, 6, 29, 58] }, Tag { name: "minecraft:has_structure/end_city".to_string(), entries: vec![17, 18] }, Tag { name: "minecraft:is_deep_ocean".to_string(), entries: vec![11, 9, 13, 12] }, Tag { name: "minecraft:is_nether".to_string(), entries: vec![34, 49, 7, 59, 2] }, Tag { name: "minecraft:has_structure/bastion_remnant".to_string(), entries: vec![7, 34, 49, 59] }, Tag { name: "minecraft:reduce_water_ambient_spawns".to_string(), entries: vec![41, 24] }, Tag { name: "minecraft:spawns_warm_variant_farm_animals".to_string(), entries: vec![14, 58, 1, 28, 50, 42, 43, 63, 34, 49, 7, 59, 2, 0, 19, 64, 31, 12, 29] }, Tag { name: "minecraft:has_structure/woodland_mansion".to_string(), entries: vec![8, 39] }, Tag { name: "minecraft:increased_fire_burnout".to_string(), entries: vec![1, 33, 31, 47, 23, 27, 54, 28] }, Tag { name: "minecraft:has_structure/ruined_portal_mountain".to_string(), entries: vec![0, 19, 64, 62, 60, 61, 43, 63, 52, 32, 23, 27, 51, 47, 5] }]), ("minecraft:instrument".to_string(), vec![Tag { name: "minecraft:screaming_goat_horns".to_string(), entries: vec![0, 1, 7, 2] }, Tag { name: "minecraft:goat_horns".to_string(), entries: vec![4, 6, 5, 3, 0, 1, 7, 2] }, Tag { name: "minecraft:regular_goat_horns".to_string(), entries: vec![4, 6, 5, 3] }]), ("minecraft:banner_pattern".to_string(), vec![Tag { name: "minecraft:pattern_item/piglin".to_string(), entries: vec![22] }, Tag { name: "minecraft:no_item_required".to_string(), entries: vec![26, 27, 28, 29, 31, 38, 35, 37, 32, 36, 34, 33, 25, 5, 30, 39, 40, 41, 42, 7, 10, 9, 8, 3, 23, 19, 17, 20, 18, 1, 14, 15] }, Tag { name: "minecraft:pattern_item/field_masoned".to_string(), entries: vec![2] }, Tag { name: "minecraft:pattern_item/flower".to_string(), entries: vec![12] }, Tag { name: "minecraft:pattern_item/flow".to_string(), entries: vec![11] }, Tag { name: "minecraft:pattern_item/creeper".to_string(), entries: vec![4] }, Tag { name: "minecraft:pattern_item/globe".to_string(), entries: vec![13] }, Tag { name: "minecraft:pattern_item/bordure_indented".to_string(), entries: vec![6] }, Tag { name: "minecraft:pattern_item/guster".to_string(), entries: vec![16] }, Tag { name: "minecraft:pattern_item/mojang".to_string(), entries: vec![21] }, Tag { name: "minecraft:pattern_item/skull".to_string(), entries: vec![24] }])],
    }.try_into()?);

		game.send_packet(
			&stream.peer_addr()?,
			lib::packets::clientbound::configuration::FinishConfiguration::PACKET_ID,
			lib::packets::clientbound::configuration::FinishConfiguration {}.try_into()?,
		);

		return Ok(None);
	}

	pub fn acknowledge_finish_configuration(stream: &mut TcpStream) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		return Ok(Some(PacketHandlerAction::NewPlayer(stream.peer_addr()?, stream.try_clone()?)));
	}
}

pub mod play {
	use super::*;

	pub fn set_player_position(data: &mut [u8], stream: &TcpStream) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::SetPlayerPosition::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::MovePlayer(stream.peer_addr()?, Some((parsed_packet.x, parsed_packet.y, parsed_packet.z)), None)));
	}

	pub fn set_player_position_and_rotation(data: &mut [u8], stream: &TcpStream) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::SetPlayerPositionAndRotation::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::MovePlayer(
			stream.peer_addr()?,
			Some((parsed_packet.x, parsed_packet.y, parsed_packet.z)),
			Some((parsed_packet.yaw % 360.0, parsed_packet.pitch)),
		)));
	}

	pub fn set_player_rotation(data: &mut [u8], stream: &TcpStream) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::SetPlayerRotation::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::MovePlayer(stream.peer_addr()?, None, Some((parsed_packet.yaw % 360.0, parsed_packet.pitch)))));
	}

	pub fn confirm_teleportation(data: &mut [u8], stream: &mut TcpStream) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::ConfirmTeleportation::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::ConfirmTeleportation(stream.peer_addr()?, parsed_packet.teleport_id)));
	}

	pub fn set_creative_mode_slot(data: &mut [u8], stream: &mut TcpStream) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::SetCreativeModeSlot::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::SetCreativeModeSlot(stream.peer_addr()?, parsed_packet.slot as u8, parsed_packet.item)));
	}

	pub fn set_held_item(data: &mut [u8], stream: &mut TcpStream) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::SetHandItem::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::SetSelectedSlot(stream.peer_addr()?, parsed_packet.slot as u8)));
	}

	pub fn player_action(data: &mut [u8], stream: &mut TcpStream) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::PlayerAction::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::BreakBlock(
			stream.peer_addr()?,
			parsed_packet.status as u8,
			parsed_packet.location,
			parsed_packet.face,
			parsed_packet.sequence,
		)));
	}

	pub fn use_item_on(data: &mut [u8], stream: &mut TcpStream) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::UseItemOn::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::UseItemOn(
			stream.peer_addr()?,
			parsed_packet.hand as u8,
			parsed_packet.location,
			parsed_packet.face,
			parsed_packet.cursor_position_x,
			parsed_packet.cursor_position_y,
			parsed_packet.cursor_position_z,
			parsed_packet.inside_block,
			parsed_packet.world_border_hit,
			parsed_packet.sequence,
		)));
	}

	pub fn chat_message(data: &mut [u8], stream: &mut TcpStream) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::ChatMessage::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::SendChatMessage(
			stream.peer_addr()?,
			parsed_packet.message,
			parsed_packet.timestamp,
			parsed_packet.salt,
		)));
	}

	pub fn chat_command(data: &mut [u8], stream: &mut TcpStream) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::ChatCommand::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::SendCommand(stream.peer_addr()?, parsed_packet.command)));
	}

	pub fn pick_item_from_block(data: &mut [u8], stream: &mut TcpStream) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::PickItemFromBlock::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::PickItemFromBlock(stream.peer_addr()?, parsed_packet.location, parsed_packet.include_data)));
	}

	pub fn swing_arm(data: &mut [u8], stream: &mut TcpStream) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::SwingArm::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::SwingArm(stream.peer_addr()?, parsed_packet.hand as u8)));
	}

	pub fn click_container(data: &mut [u8], stream: &mut TcpStream) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::ClickContainer::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::ClickContainer(stream.peer_addr()?, parsed_packet)));
	}

	pub fn close_container(stream: &mut TcpStream, data: &mut [u8]) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::CloseContainer::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::CloseContainer(stream.peer_addr()?, parsed_packet.window_id)));
	}

	pub fn update_sign(data: &mut [u8]) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::UpdateSign::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::UpdateSign(
			parsed_packet.location,
			parsed_packet.is_front_text,
			[parsed_packet.line_1, parsed_packet.line_2, parsed_packet.line_3, parsed_packet.line_4],
		)));
	}

	pub fn player_input(stream: &mut TcpStream, data: &mut [u8]) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::PlayerInput::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::PlayerInput(stream.peer_addr()?, parsed_packet)));
	}

	pub fn interact(stream: &mut TcpStream, data: &mut [u8]) -> Result<Option<PacketHandlerAction>, Box<dyn Error>> {
		let parsed_packet = lib::packets::serverbound::play::Interact::try_from(data.to_vec())?;
		return Ok(Some(PacketHandlerAction::Interact(stream.peer_addr()?, parsed_packet)));
	}
}
