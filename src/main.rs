use std::{
    collections::{hash_map::Entry, HashMap},
    io::{self, Read},
    net::Ipv4Addr,
};

mod tcp;

use etherparse::IpNumber;

// ipv4 packet header itself, ethernet should be 0x0800
const IPV4: u16 = 0x4500;
const TCP: u8 = 0x06;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut config = tun::Configuration::default();
    let mut connections: HashMap<Quad, tcp::State> = Default::default();

    config.tun_name("utun4").up();

    let mut tun = tun::create(&config)?;

    let mut buf = [0u8; 1504];

    loop {
        match tun.read(&mut buf) {
            Ok(n) => {
                let nbytes = &buf[..n];
                let ip_version = u16::from_be_bytes([nbytes[0], nbytes[1]]);

                if ip_version != IPV4 {
                    continue;
                }

                match etherparse::Ipv4HeaderSlice::from_slice(&buf[..n]) {
                    Ok(iph) => {
                        let src = iph.source_addr();

                        let dst = iph.destination_addr();

                        let proto = iph.protocol();

                        if proto != IpNumber(TCP) {
                            continue;
                        }

                        match etherparse::TcpHeaderSlice::from_slice(&buf[iph.slice().len()..n]) {
                            Ok(tcph) => {
                                let datai = 4 + iph.slice().len() + tcph.slice().len();

                                match connections.entry(Quad {
                                    src: (src, tcph.source_port()),
                                    dst: (dst, tcph.destination_port()),
                                }) {
                                    Entry::Occupied(mut c) => {
                                        c.get_mut().on_packet(iph, tcph, &buf[datai..n]);
                                    }
                                    Entry::Vacant(e) => {
                                        if let Some(c) =
                                            tcp::Connection::on_accept(iph, tcph, &buf[datai..n])?
                                        {
                                            e.insert(c)
                                        }
                                    }
                                }

                                println!(
                                    "{} -> {}: TCP to port {}",
                                    src,
                                    dst,
                                    tcph.destination_port()
                                )
                            }
                            Err(e) => {
                                eprintln!("An error occured while parsing TCP packet: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("An error occured while parsing  TCP packet: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from TUN: {}", e);
                break;
            }
        }
    }

    Ok(())
}
