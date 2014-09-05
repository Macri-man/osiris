use std::io::net::udp;
use std::io::net::ip;

fn main() {
    let addr = ip::SocketAddr {ip: ip::Ipv4Addr(127, 0, 0, 1), port: 34000};
    let mut socket = match udp::UdpSocket::bind(addr) {
	Ok(s) => s,
	Err(e) => fail!("couldn't bind socket: {}", e),
    };

    let mut buf = [0, ..10];
    match socket.recv_from(buf) {
	Ok((amt, src)) => {
	    let buf = buf.mut_slice_to(amt);
	    buf.reverse();
	    socket.send_to(b"blargh", src);
	}
	Err(e) => println!("couldn't recieve a datagram: {}", e)
    }
    drop(socket);
}
