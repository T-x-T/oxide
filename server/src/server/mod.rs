use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
use lib::ConnectionState;

pub mod packet;

pub fn initialize_server() {
  let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
  
  let mut connection_states: Arc<Mutex<HashMap<SocketAddr, ConnectionState>>> = Arc::new(Mutex::new(HashMap::new()));

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    println!("New Connection from {}", stream.peer_addr().unwrap());
    let connection_states_clone = connection_states.clone();
    std::thread::spawn(move || {
      loop {
        let mut stream = stream.try_clone().unwrap();
        let packet = lib::utils::read_packet(&mut stream);
      
        if packet::handlers::handle_packet(packet, &mut stream, &mut connection_states_clone.lock().unwrap()) {
          connection_states_clone.lock().unwrap().remove(&stream.peer_addr().unwrap());
          break;
        }
      }
    });
  }
}