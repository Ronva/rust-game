use std::net::UdpSocket;
use std::str;

use crate::constants::SERVER_PORT;
use crate::game;

pub fn connect_to_server() -> UdpSocket {
  let address = "127.0.0.1:0";
  let socket = UdpSocket::bind(address).expect("couldn't bind to address");
  
  socket.set_nonblocking(true).unwrap();
  socket
    .connect(format!("127.0.0.1:{}", SERVER_PORT))
    .expect("connect function failed");
  send_to_server(&socket, b"connect");
  socket
}

pub fn send_to_server(socket: &UdpSocket, msg: &[u8]) {
  socket.send(msg).expect("couldn't send message");
}

pub fn udp_listener(gs: &mut game::State) {
  let mut buf = vec![0_u8; 536870912];
  match gs.socket.recv(&mut buf) {
    Ok(received) => {
      let decoded = str::from_utf8(&buf[..received]).unwrap();
      game::process_server_data(gs, String::from(decoded));
    }
    _ => {}
  }
}