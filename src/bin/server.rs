use std::io::{Read, Write};
use std::net::{TcpListener, UdpSocket};
use std::thread;

fn main() {
    let tcp_listener = TcpListener::bind("[::]:1422").unwrap();
    let tcp_port = tcp_listener.local_addr().unwrap().port();
    let udp_socket: UdpSocket = UdpSocket::bind("[::]:0").unwrap();
    let udp_port = udp_socket.local_addr().unwrap().port();
    println!(
        "Listening on port {} for TCP and {} for UDP",
        tcp_port, udp_port
    );

    let tcp_stream = tcp_listener.accept().unwrap().0;
    let mut udp_buf = [0; 50];
    let mut tcp_buf = [0; 50];

    println!("Got a connection! from {}", tcp_stream.peer_addr().unwrap());
    // connect pipe of udp_socket to tcp_stream
    let _udp_socket = udp_socket.try_clone().unwrap();
    let mut _tcp_stream = tcp_stream.try_clone().unwrap();

    udp_socket.connect("[::1]:51820").unwrap();

    thread::spawn(move || {
        loop {
            let udp_size = _udp_socket
                .recv(&mut udp_buf)
                .expect("couldn't read from udp socket");
            if udp_size > 0 {
                // remove the first 4 bytes from the udp_buf
                let udp_buf = &udp_buf[4..udp_size];
                let udp_size = udp_size - 4;

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
    thread::spawn(move || {
        loop {
            let tcp_size = _tcp_stream
                .read(&mut tcp_buf)
                .expect("couldn't read from tcp stream");
            if tcp_size > 0 {
                // remove the first 4 bytes from the tcp_buf
                let tcp_buf = &tcp_buf[4..tcp_size];
                let tcp_size = tcp_size - 4;

                println!(
                    "Got {} bytes from TCP: {:?}",
                    tcp_size,
                    &tcp_buf[0..tcp_size]
                );
                udp_socket
                    .send(&tcp_buf[0..tcp_size])
                    .expect("couldn't send udp message");
            }
        }
    });

    loop {}
}
