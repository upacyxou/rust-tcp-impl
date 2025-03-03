use std::io;
use std::io::prelude::*;

use tun::Device;

pub enum State {
    Closed,
    Listen,
    SyncRcvd,
    Estab,
}

pub struct SendSequenceSpace {
    una: u32,
    nxt: u32,
    wnd: u16,
    up: bool,
    wl1: usize,
    wl2: usize,
    iss: u32,
}

pub struct RecvSequenceSpace {
    next: u32,
    wnd: u16,
    up: bool,
    irs: u32,
}

pub struct Connection {
    state: State,
    send: SendSequenceSpace,
    recv: RecvSequenceSpace,
    ip: etherparse::Ipv4Header,
    tcp: etherparse::TcpHeader,
}

impl Default for State {
    fn default() -> Self {
        State::Listen
    }
}

impl Connection {
    pub fn accept<'a>(
        nic: &mut Device,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<Option<Self>> {
        let mut buf = [0u8, 1500];
        if !tcph.syn() {
            return Ok(None);
        }

        let iss = 0;
        let wnd = 10;
        let mut c = Connection {
            state: State::SyncRcvd,
            send: SendSequenceSpace {
                iss,
                una: iss,
                nxt: 1,
                wnd,
                up: false,
                wl1: 0,
                wl2: 0,
            },
            recv: RecvSequenceSpace {

                irs: tcph.sequence_number(),
                next: tcph.sequence_number() + 1,
                wnd: tcph.window_size(),
                up: false,
            },
            tcp: etherparse::TcpHeader::new(tcph.source_port(), tcph.destination_port(), tcph.sequence_number(), tcph.window_size())б
            ip: etherparse::Ipv4Header::new(syn_ack.header_len(), 64 as u8, etherparse::IpNumber::TCP.0, source, destination)
        }

        return Ok(None);
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
        eprintln!(
            "{}:{} -> {}:{} {}b of TCP",
            iph.source_addr(),
            tcph.source_port(),
            iph.destination_addr(),
            tcph.destination_port(),
            data.len()
        );
    }
}
