use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

pub struct Tcp {
    listener: TcpListener,
    stream: Option<TcpStream>,
}

impl Tcp {
    pub fn new(bind_addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(bind_addr)?;
        listener.set_nonblocking(true)?;

        Ok(Self {
            listener,
            stream: None,
        })
    }

    pub fn try_receive(&mut self, buf: &mut [u8]) -> io::Result<Option<usize>> {
        if self.stream.is_none() {
            match self.listener.accept() {
                Ok((stream, _addr)) => {
                    stream.set_nonblocking(true)?;
                    self.stream = Some(stream);
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(None),
                Err(e) => return Err(e),
            }
        }

        let stream = self.stream.as_mut().unwrap();
        let mut len_buf = [0u8; 4];
        match stream.read_exact(&mut len_buf) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(None),
            Err(e) => {
                self.stream = None;
                return Err(e);
            }
        }

        let len = u32::from_be_bytes(len_buf) as usize;
        stream.set_nonblocking(false)?;
        let result = stream.read_exact(&mut buf[..len]);
        stream.set_nonblocking(true)?;
        match result {
            Ok(()) => Ok(Some(len)),
            Err(e) => {
                self.stream = None;
                Err(e)
            }
        }
    }

    pub fn send(&mut self, data: &[u8]) -> io::Result<()> {
        if let Some(stream) = &mut self.stream {
            stream.write_all(data)?;
            stream.flush()?;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "no client connected"))
        }
    }
}
