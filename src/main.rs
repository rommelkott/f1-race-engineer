use std::{net::UdpSocket, sync::{Arc, Mutex}};

mod packet;
use packet::*;

mod parsers;
use parsers::*;



fn main() {
    println!("Running");
    let socket = UdpSocket::bind("192.168.1.26:20777").unwrap();

    // create raw context to store data that will be passed to the handler function, it should be passed as a mutable reference
    let context = Arc::new(Mutex::new(Context::new()));

    // create seperate thread to handle the stream
    let passable_context = context.clone();
    let _handle = std::thread::spawn(move || {
        handle_stream(&socket, &passable_context);
    });

    // every 5 seconds print the lap data
    // let clone_context = context.clone();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
        let clone = context.lock().unwrap();
        let lap_data = &clone.lap_data;
        // clear the screen
        print!("\x1B[2J\x1B[1;1H");
        println!("{:?}", clone.lap_data);
    }
}

fn handle_stream (stream: &std::net::UdpSocket, context: &Arc<Mutex<Context>>) {
    let mut buf = [0u8; 1492];
    loop {
        // receive a single datagram message on the socket
        let (amt, src) = stream.recv_from(&mut buf).unwrap();

        // parse the header
        let mut context = context.lock().unwrap();
        let packet_type = parsers::get_packet_type(&buf);
        match packet_type {
            PacketType::LapData => {
              let lap_data = parse_lap_data(&buf);
              context.lap_data = lap_data;
            }
            _ => {
              println!("Unhandled Packet Type: {:?}", packet_type);
            }
        }
    }
}



