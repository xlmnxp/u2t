use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::thread;

fn main() {
    let tcp_stream = TcpStream::connect("play.sy.sa:1422").unwrap();
    let tcp_port = tcp_stream.local_addr().unwrap().port();
    let udp_socket: UdpSocket = UdpSocket::bind("[::]:51820").unwrap();
    let udp_port = udp_socket.local_addr().unwrap().port();
    println!(
        "Listening on port {} for TCP and {} for UDP",
        tcp_port, udp_port
    );

    let mut udp_buf = [0; 50];
    let mut tcp_buf = [0; 50];

    // connect pipe of udp_socket to tcp_stream
    let _udp_socket = udp_socket.try_clone().unwrap();
    let mut _tcp_stream = tcp_stream.try_clone().unwrap();

    thread::spawn(move || {
        loop {
            let udp_size = _udp_socket
                .recv(&mut udp_buf)
                .expect("couldn't read from udp socket");
            if udp_size > 0 {
                println!(
                    "Got {} bytes from UDP: {:?}",
                    udp_size,
                    &udp_buf[0..udp_size]
                );

                _tcp_stream
                    .write(&udp_buf[0..udp_size])
                    .expect("couldn't write to tcp stream");
            }
        }
    });

    // connect pipe of tcp_stream to udp_socket
    let mut _tcp_stream: std::net::TcpStream = tcp_stream.try_clone().unwrap();
    thread::spawn(move || loop {
        let tcp_size = _tcp_stream
            .read(&mut tcp_buf)
            .expect("couldn't read from tcp stream");
        if tcp_size > 0 {
            println!(
                "Got {} bytes from TCP: {:?}",
                tcp_size,
                &tcp_buf[0..tcp_size]
            );
            udp_socket
                .send_to(&tcp_buf[0..tcp_size], "[::1]:51821")
                .expect("couldn't send udp message");
        }
    });

    loop {}
}
