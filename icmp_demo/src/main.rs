use std::{net::Ipv4Addr, time::Duration};

use icmp_socket::{IcmpSocket4, Icmpv4Packet, packet::WithEchoRequest, IcmpSocket, Icmpv4Message};

fn main() {
    let identifier: u16 = 1337; // How the packet finds its way back
    let mut sequence: u16 = 1;

    loop {
        println!("<-------- ECHO REQUEST -------->");
        let payload: Vec<u8> = "Sending you some data".as_bytes().to_vec(); // Data we want to send
        let packet: Icmpv4Packet = Icmpv4Packet::with_echo_request(identifier, sequence, payload).unwrap().with_checksum();
        println!("[+] ICMP packet: {:?}", packet);

        let mut socket: IcmpSocket4 = IcmpSocket4::new().unwrap(); // create socket

        let addr: Ipv4Addr = "0.0.0.0".parse::<Ipv4Addr>().unwrap(); // Our address
        socket.bind(addr).unwrap();

        let dest_address: Ipv4Addr = "8.8.8.8".parse::<Ipv4Addr>().unwrap(); // Destination address
        socket.send_to(dest_address, packet).unwrap();

        loop {
            println!("<-------- ECHO REPLY -------->");
            let (packet_type, socket_addr) = socket.rcv_from().unwrap(); // Get the reply!

            println!("[+] Ping reply from {}", socket_addr.as_socket_ipv4().unwrap().ip().to_string());
            println!("ICMP Message: {:?}", packet_type);
            if let Icmpv4Message::EchoReply { identifier, sequence, payload } = packet_type.message {
                println!("[+] Identifier: {}", identifier);
                println!("[+] Sequence: {}", sequence);
                println!("[+] Payload: {}", String::from_utf8(payload).unwrap());
            }
            std::thread::sleep(Duration::from_secs(1));
            break;
        }
        sequence += 1;
    }
}