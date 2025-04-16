use std::collections::HashMap;
use std::net::{TcpListener, SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use lib::ConnectionState;

pub mod packet;

pub fn initialize_server() {
  let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
  
  let connections: Arc<Mutex<HashMap<SocketAddr, Connection>>> = Arc::new(Mutex::new(HashMap::new()));
  let game: Arc<Mutex<Game>> = Arc::new(Mutex::new(Game {
    players: Vec::new(),
  }));

  let connection_streams: Arc<Mutex<HashMap<SocketAddr, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

  for stream in listener.incoming() {
    let stream = stream.unwrap();


    println!("New Connection from {}", stream.peer_addr().unwrap());
    let connection_states_clone = connections.clone();
    let connection_streams_clone = connection_streams.clone();
    let game_clone = game.clone();
    std::thread::spawn(move || {
      loop {
        let mut stream = stream.try_clone().unwrap();
        let packet = lib::utils::read_packet(&mut stream);
      
        if packet::handlers::handle_packet(packet, &mut stream, &mut connection_states_clone.lock().unwrap(), &mut connection_streams_clone.lock().unwrap(), &mut game_clone.lock().unwrap()) {
          connection_states_clone.lock().unwrap().remove(&stream.peer_addr().unwrap());
          break;
        }
      }
    });
  }
}

#[derive(Debug, Clone)]
pub struct Game {
  pub players: Vec<Player>,
}

#[derive(Debug, Clone)]
pub struct Player {
  pub x: f64,
  pub y_feet: f64,
  pub z: f64,
  pub display_name: String,
  pub uuid: u128,
  pub peer_socket_address: SocketAddr,
}

#[derive(Debug, Clone)]
pub struct Connection {
  pub state: ConnectionState,
  pub peer_address: SocketAddr,
  pub player_name: Option<String>,
  pub player_uuid: Option<u128>,
}