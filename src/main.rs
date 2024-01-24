mod packet;

use std::net::{IpAddr, SocketAddr, TcpStream};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Cursor, Read, Write};
use std::str::FromStr;
use byteorder::{BigEndian, WriteBytesExt};


struct MinecraftServer {
    ip: IpAddr,
}

struct TCPConnection {
    stream: TcpStream,
    socket: SocketAddr,
}

struct Packet {
    handshake: SocketAddr
}

impl Packet {
    fn send(&self) {

    }
}

fn parse_ip(line: String) -> Result<IpAddr, &'static str> {
    let slice = line.split_whitespace();
    for part in slice {
        let ip = match IpAddr::from_str(part) {
            Err(_) => { continue;
            },
            Ok(i) => {
                i
            }
        };
        return Ok(ip);
    };
    Err("Huge error reading IP's from the file, are there any?")
}

fn read_ips(file_path: &str)-> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
    println!("Reading from {}", file_path);
    let mut ips: Vec<IpAddr> = vec![];
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(ip) = parse_ip(line?) {
            ips.push(ip);
        }
    }
    Ok(ips)
}


fn handshake(socket: SocketAddr) -> io::Result<()> {
    println!("{}", socket);
    let mut tcp_stream = TcpStream::connect((socket.ip(), socket.port())).expect("Failed to connect to socket");

    let mut handshake_packet = Vec::new();



    Ok(())


}

fn main() {
    // let addr = read_ips("masscan.txt").expect("Failed to read IP's");
    // handshake(SocketAddr::from_str("188.40.22.42:25503").unwrap());
    // let (ping, response) = get_status("188.40.22.42:25503", None).unwrap();
    // println!("Ping {}, Response {:?}", ping, response.players.online);
}

