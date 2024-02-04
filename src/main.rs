mod packet;

use std::error::Error;
use std::thread;
use std::net::{IpAddr};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use mcping::{get_status, Response};


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

fn write_file(ip: &str) -> std::io::Result<()> {

    let mut file = File::create("active.txt")?;

    file.write_all(ip.as_bytes())?;

    println!("Wrote {} -> to file", ip);

    Ok(())
}





fn main()  {
    let ips = read_ips("ips.txt").unwrap();
    let active_ips: Arc<Mutex<Vec<Response>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for ip in ips {
        let active_ips_clone = Arc::clone(&active_ips);

        // Spawn many threads here
        let handle = thread::spawn(move || {
            println!("Contacting {}", ip);
            let status = get_status(&ip.to_string(), Duration::from_secs(2));
            match status {
                Ok((_, res)) => {
                    // Lock the mutex before modifying the shared vector
                    let mut active_ips = active_ips_clone.lock().unwrap();
                    active_ips.push(res);
                }
                Err(err) => {
                    eprintln!("Error contacting ip, {}", err);
                }
            }
        });
        handles.push(handle);

    }

    for handle in handles {
        handle.join().unwrap();
    }

// Do this after all threads have closed
    let active_ips = active_ips.lock().unwrap();
    for active_ip in active_ips.iter() {
        write_file(&active_ip.version.name).expect("TODO: panic message");
    }
}

