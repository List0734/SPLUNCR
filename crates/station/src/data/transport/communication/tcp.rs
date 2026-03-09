use std::{
    io::{self, Write},
    net::TcpStream,
};

pub struct Tcp {
    stream: TcpStream,
}

impl Tcp {
    pub fn connect(target_addr: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(target_addr)?;
        stream.set_nonblocking(true)?;

        Ok(Self { stream })
    }

    pub fn send(&mut self, data: &[u8]) -> io::Result<()> {
        let len = (data.len() as u32).to_be_bytes();
        self.stream.write_all(&len)?;
        self.stream.write_all(data)?;
        Ok(())
    }
}
