use std::{
    io::{self, Read, Write},
    net::TcpStream,
    time::Duration,
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

    pub fn receive_exact(&mut self, buf: &mut [u8], timeout: Duration) -> io::Result<()> {
        self.stream.set_nonblocking(false)?;
        self.stream.set_read_timeout(Some(timeout))?;
        let result = self.stream.read_exact(buf);
        self.stream.set_read_timeout(None)?;
        self.stream.set_nonblocking(true)?;
        result
    }
}
