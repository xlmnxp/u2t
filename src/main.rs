use std::net::{TcpListener, UdpSocket};
use std::io::{Read, Write};

fn main() {
    let tcp_listener = TcpListener::bind("[::]:0").unwrap();
    let tcp_port = tcp_listener.local_addr().unwrap().port();
    let udp_socket = UdpSocket::bind("[::]:0").unwrap();
    let udp_port = udp_socket.local_addr().unwrap().port();
    println!("Listening on port {} for TCP and {} for UDP", tcp_port, udp_port);
    
    let mut tcp_stream = tcp_listener.accept().unwrap().0;
    let mut udp_buf = [0; 50];
    loop {
        let mut tcp_buf = [0; 50];
        let tcp_size = tcp_stream.read(&mut tcp_buf).unwrap();
        if tcp_size > 0 {
            udp_socket.send_to(&tcp_buf[0..tcp_size], "[::]:51820").unwrap();
        }
        let udp_size = udp_socket.recv(&mut udp_buf).unwrap();
        if udp_size > 0 {
            tcp_stream.write(&udp_buf[0..udp_size]).unwrap();
        }
    }
}