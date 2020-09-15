use std::net::UdpSocket;
use std::str;

use crate::constants::SERVER_PORT;
use crate::player::*;
use crate::structs::State;

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

pub fn udp_listener(gs: &mut State) {
  let mut buf = vec![0_u8; 536870912];
  match gs.socket.recv(&mut buf) {
    Ok(received) => {
      let decoded = str::from_utf8(&buf[..received]).unwrap();
      process_server_data(gs, String::from(decoded));
    }
    _ => {}
  }
}

pub fn process_server_data(gs: &mut State, data: String) {
  let strings: Vec<&str> = data.split(":").collect();
  let (op, info) = (strings[0], strings[1]);
  match op {
    // c: current player has connected
    "c" => {
      let players: Vec<&str> = info.split(";").collect();
      for p in players.iter() {
        if let Some((player, pos)) = get_player_info(p) {
          create_player(gs, player, pos);
        }
      }
    }
    // u: a player's position has been updated
    "u" => {
      if let Some((player, pos)) = get_player_info(info) {
        let id = player.id.clone();
        if let Some(_entry) = get_player_entry(gs, id) {
          move_player(gs, player.id, pos.x, pos.y)
        } else {
          create_player(gs, player, pos)
        }
      }
    }
    _ => {}
  }
}
