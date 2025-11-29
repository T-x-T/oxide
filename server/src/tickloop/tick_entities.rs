use super::*;

pub fn process(game: Arc<Game>, players_clone: &[Player]) {
  for dimension in &mut game.world.lock().unwrap().dimensions {
    let mut entities = std::mem::take(&mut dimension.1.entities);
    let mut entity_tick_outcomes: Vec<(i32, EntityTickOutcome)> = Vec::new();
    for entity in &mut entities {
      let outcome = entity.tick(dimension.1, players_clone, &game.block_state_data, game.clone());
      //println!("ticked entity in {:.2?}", std::time::Instant::now() - now);
      if outcome != EntityTickOutcome::None {
        entity_tick_outcomes.push((entity.get_common_entity_data().entity_id, outcome));
      }
    }
    dimension.1.entities = entities;
    for outcome in entity_tick_outcomes {
      match outcome.1 {
        EntityTickOutcome::SelfDied => {
          let entity_event_packet = lib::packets::clientbound::play::EntityEvent {
            entity_id: outcome.0,
            entity_status: 3,
          };

          for player in players_clone {
            game.send_packet(&player.peer_socket_address, lib::packets::clientbound::play::EntityEvent::PACKET_ID, entity_event_packet.clone().try_into().unwrap());
          }
        },
        EntityTickOutcome::RemoveSelf => {
          let remove_entities_packet = lib::packets::clientbound::play::RemoveEntities {
            entity_ids: vec![outcome.0],
          };

          for player in players_clone {
            game.send_packet(&player.peer_socket_address, lib::packets::clientbound::play::RemoveEntities::PACKET_ID, remove_entities_packet.clone().try_into().unwrap());
          }

          if let Some(chunk) = dimension.1.get_chunk_from_position_mut(
            dimension.1.entities
              .iter()
              .find(|x| x.get_common_entity_data().entity_id == outcome.0)
              .unwrap()
              .get_common_entity_data()
              .position.into()
          ) {
            chunk.modified = true;
          };
          dimension.1.entities.retain(|x| x.get_common_entity_data().entity_id != outcome.0);
        },
        EntityTickOutcome::Updated => {
          if let Some(chunk) = dimension.1.get_chunk_from_position_mut(
            dimension.1.entities
              .iter()
              .find(|x| x.get_common_entity_data().entity_id == outcome.0)
              .unwrap()
              .get_common_entity_data()
              .position.into()
          ) {
            chunk.modified = true;
          };
        },
        EntityTickOutcome::None => (),
      }
    }
  }
}
