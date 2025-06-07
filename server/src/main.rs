#![allow(clippy::needless_return)]

use std::collections::HashMap;
use std::net::{TcpListener, SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use lib::types::world::World;
use types::*;

mod packet_handlers;
mod types;

fn main() {
  println!("Starting the oxide server");
  initialize_server();
}

fn initialize_server() {
  let listener = TcpListener::bind("0.0.0.0:25565").unwrap();

  let connections: Arc<Mutex<HashMap<SocketAddr, Connection>>> = Arc::new(Mutex::new(HashMap::new()));
  let mut game = Game {
    players: Vec::new(),
    world: World::new(),
    last_created_entity_id: 0,
    chat_message_index: 0,
    commands: Vec::new(),
  };
  init(&mut game);

  let game: Arc<Mutex<Game>> = Arc::new(Mutex::new(game));

  let connection_streams: Arc<Mutex<HashMap<SocketAddr, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

  for stream in listener.incoming() {
    let stream = stream.unwrap();


    println!("New Connection from {}", stream.peer_addr().unwrap());
    let connections_clone = connections.clone();
    let connection_streams_clone = connection_streams.clone();
    let game_clone = game.clone();
    std::thread::spawn(move || {
      let mut stream = stream.try_clone().unwrap();
      let peer_addr = stream.peer_addr().unwrap();
      loop {

        let mut peek_buf = [0; 1];

        match stream.peek(&mut peek_buf) {
          Ok(0) => {
            println!("client disconnected.");
            disconnect_player(&peer_addr, &mut connections_clone.lock().unwrap(), &mut connection_streams_clone.lock().unwrap(), &mut game_clone.lock().unwrap().players);
            break;
          }
          Err(e) => {
            eprintln!("error reading from client: {e}");
            disconnect_player(&peer_addr, &mut connections_clone.lock().unwrap(), &mut connection_streams_clone.lock().unwrap(), &mut game_clone.lock().unwrap().players);
            break;
          }
          _ => {}
        }

        let packet = lib::utils::read_packet(&stream);

        if stream.peer_addr().is_err() {
          disconnect_player(&peer_addr, &mut connections_clone.lock().unwrap(), &mut connection_streams_clone.lock().unwrap(), &mut game_clone.lock().unwrap().players);
          break;
        }

        if packet_handlers::handle_packet(packet, &mut stream, &mut connections_clone.lock().unwrap(), &mut connection_streams_clone.lock().unwrap(), &mut game_clone.lock().unwrap()).is_err() {
          disconnect_player(&peer_addr, &mut connections_clone.lock().unwrap(), &mut connection_streams_clone.lock().unwrap(), &mut game_clone.lock().unwrap().players);
          break;
        }
      }
    });
  }
}

fn disconnect_player(peer_addr: &SocketAddr, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>, players: &mut Vec<Player>) {
  let players_clone = players.clone();
  let player_to_remove = players_clone.iter().find(|x| x.peer_socket_address == *peer_addr);
  packet_handlers::update_players(connection_streams, connections, players.clone(), player_to_remove);
  connections.remove(peer_addr);
  connection_streams.remove(peer_addr);
  players.retain(|x| x.peer_socket_address != *peer_addr);
}
