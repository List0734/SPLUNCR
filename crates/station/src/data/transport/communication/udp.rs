use std::{io, net::UdpSocket};

pub struct Udp {
    socket: UdpSocket,
}

impl Udp {
    pub fn new(bind_addr: &str) -> io::Result<Self> {
        let socket = UdpSocket::bind(bind_addr)?;
        socket.set_nonblocking(true)?;

        Ok(Self { socket })
    }

    pub fn try_receive(&self, buf: &mut [u8]) -> io::Result<Option<(usize, std::net::SocketAddr)>> {
        match self.socket.recv_from(buf) {
            Ok(r) => Ok(Some(r)),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(e),
        }
    }
}
