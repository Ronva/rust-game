use std::net::UdpSocket;

pub fn connect_to_server() -> UdpSocket {
  let socket = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to address");
  socket.set_nonblocking(true).unwrap();
  socket
    .connect("127.0.0.1:2052")
    .expect("connect function failed");
  send_to_server(&socket, b"connect");
  socket
}

pub fn send_to_server(socket: &UdpSocket, msg: &[u8]) {
  socket.send(msg).expect("couldn't send message");
}
