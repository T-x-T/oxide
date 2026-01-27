use super::*;
use lib::packets::Packet;

pub fn process(entity_tick_outcomes: Vec<(i32, EntityTickOutcome)>, game: Arc<Game>, players_clone: &[Player], dimension: &mut Dimension) {
	for outcome in entity_tick_outcomes {
		match outcome.1 {
			EntityTickOutcome::SelfDied => {
				let entity_event_packet = lib::packets::clientbound::play::EntityEvent {
					entity_id: outcome.0,
					entity_status: 3,
				};

				for player in players_clone {
					game.send_packet(
						&player.peer_socket_address,
						lib::packets::clientbound::play::EntityEvent::PACKET_ID,
						entity_event_packet.clone().try_into().unwrap(),
					);
				}
			}
			EntityTickOutcome::RemoveSelf => {
				let remove_entities_packet = lib::packets::clientbound::play::RemoveEntities {
					entity_ids: vec![outcome.0],
				};

				for player in players_clone {
					game.send_packet(
						&player.peer_socket_address,
						lib::packets::clientbound::play::RemoveEntities::PACKET_ID,
						remove_entities_packet.clone().try_into().unwrap(),
					);
				}

				if let Some(chunk) = dimension.get_chunk_from_position_mut(
					dimension
						.entities
						.iter()
						.find(|x| x.get_common_entity_data().entity_id == outcome.0)
						.unwrap()
						.get_common_entity_data()
						.position
						.into(),
				) {
					chunk.modified = true;
				};
				dimension.entities.retain(|x| x.get_common_entity_data().entity_id != outcome.0);
			}
			EntityTickOutcome::Updated => {
				if let Some(chunk) = dimension.get_chunk_from_position_mut(
					dimension
						.entities
						.iter()
						.find(|x| x.get_common_entity_data().entity_id == outcome.0)
						.unwrap()
						.get_common_entity_data()
						.position
						.into(),
				) {
					chunk.modified = true;
				};
			}
			EntityTickOutcome::None => (),
			EntityTickOutcome::RemoveOthers(entity_ids) => {
				let remove_entities_packet = lib::packets::clientbound::play::RemoveEntities {
					entity_ids: entity_ids.clone(),
				};

				for player in players_clone {
					game.send_packet(
						&player.peer_socket_address,
						lib::packets::clientbound::play::RemoveEntities::PACKET_ID,
						remove_entities_packet.clone().try_into().unwrap(),
					);
				}

				dimension.entities.retain(|x| !entity_ids.contains(&x.get_common_entity_data().entity_id));
			}
		}
	}
}
