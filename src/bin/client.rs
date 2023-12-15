use std::net::{TcpStream, UdpSocket};
use std::io::{Read, Write};

fn main() {
    let mut tcp_stream = TcpStream::connect("play.sy.sa:1422").unwrap();
    let tcp_port = tcp_stream.local_addr().unwrap().port();
    let udp_socket = UdpSocket::bind("[::]:51820").unwrap();
    let udp_port = udp_socket.local_addr().unwrap().port();
    println!("Listening on port {} for TCP and {} for UDP", tcp_port, udp_port);
    
    let mut udp_buf = [0; 50];

    println!("Got a connection!");
    
    loop {
        let mut tcp_buf = [0; 50];
        let udp_size = udp_socket.recv(&mut udp_buf).unwrap();
        if udp_size > 0 {
            println!("Got {} bytes from UDP", udp_size);
            tcp_stream.write(&udp_buf[0..udp_size]).unwrap();
        }
        let tcp_size = tcp_stream.read(&mut tcp_buf).unwrap();
        if tcp_size > 0 {
            println!("Got {} bytes from TCP", tcp_size);
            udp_socket.send_to(&tcp_buf[0..tcp_size], "[::]:51821").unwrap();
        }
    }
}