use std::net::{IpAddr, UdpSocket};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;


struct MinecraftServer {
    ip: IpAddr,
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


fn init_udp(addr: IpAddr) {
    let mut socket = addr.to_string();
    socket.push_str(":25565");
    println!("{}", &socket);
    UdpSocket::bind(socket).expect("Couldn't bind");

}

fn main() {
    let addr = read_ips("masscan.txt").expect("Failed to read IP's");
    for ip in addr {
        init_udp(ip).expect("TODO: panic message");
    }
}
