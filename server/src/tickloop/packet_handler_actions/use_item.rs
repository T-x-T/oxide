use lib::entity::{BreedableMob, CommonEntity, CommonMob};
use rand::{Rng, rng};

use super::*;

pub fn process(peer_addr: SocketAddr, _parsed_packet: lib::packets::serverbound::play::UseItem, game: Arc<Game>) {
	let mut players = game.players.lock().unwrap();
	let player = players.iter_mut().find(|x| x.peer_socket_address == peer_addr).unwrap();

	let held_item = player.get_held_item(true);

	if held_item.is_some_and(|x| x.count > 0 && x.id == data::items::get_item_id_by_name("minecraft:egg").unwrap()) {
		let mut rng = rng();
		if rng.random_ratio(1, 8) {
			let mut world = game.world.lock().unwrap();
			let dimension = world.dimensions.get_mut(player.get_dimension()).unwrap();
			let chicken = lib::entity::Chicken {
				common: CommonEntity {
					position: player.get_position(),
					velocity: EntityPosition::default(),
					uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID,
					entity_id: game.entity_id_manager.get_new(),
					..Default::default()
				},
				mob: CommonMob {
					health: 20.0,
					..Default::default()
				},
				breedable_mob: BreedableMob {
					age: -lib::MOB_GROW_UP_TICKS,
					..Default::default()
				},
			};


			let summon_packet = chicken.to_spawn_entity_packet();

			for player in players.iter() {
				game.send_packet(
					&player.peer_socket_address,
					lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
					summon_packet.clone().try_into().unwrap(),
				);
			}

			chicken.resend_metadata_to_players(&players, game.clone());

			dimension.add_entity(Entity::Chicken(chicken));
		}
	} else {
		player.eat();
	}
}
