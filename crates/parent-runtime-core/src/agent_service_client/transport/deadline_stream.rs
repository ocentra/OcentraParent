use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

pub(in crate::agent_service_client::transport) struct DeadlineTcpStream {
    stream: TcpStream,
    deadline: Instant,
}

impl DeadlineTcpStream {
    pub(in crate::agent_service_client::transport) fn new(
        stream: TcpStream,
        deadline: Instant,
    ) -> Self {
        Self { stream, deadline }
    }

    pub(in crate::agent_service_client::transport) fn set_read_timeout(
        &self,
        timeout: Option<Duration>,
    ) -> io::Result<()> {
        self.stream.set_read_timeout(timeout)
    }

    pub(in crate::agent_service_client::transport) fn set_write_timeout(
        &self,
        timeout: Option<Duration>,
    ) -> io::Result<()> {
        self.stream.set_write_timeout(timeout)
    }

    fn remaining(&self) -> io::Result<Duration> {
        match self.deadline.checked_duration_since(Instant::now()) {
            Some(remaining) if !remaining.is_zero() => Ok(remaining),
            _ => Err(Self::deadline_error()),
        }
    }

    fn deadline_error() -> io::Error {
        io::Error::new(
            io::ErrorKind::TimedOut,
            "agent-service WebSocket overall deadline exhausted",
        )
    }

    fn reject_if_expired(&self) -> io::Result<()> {
        if Instant::now() >= self.deadline {
            Err(Self::deadline_error())
        } else {
            Ok(())
        }
    }
}

impl Read for DeadlineTcpStream {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let remaining = self.remaining()?;
        self.stream.set_read_timeout(Some(remaining))?;
        let result = self.stream.read(buffer);
        self.reject_if_expired()?;
        result
    }
}

impl Write for DeadlineTcpStream {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let remaining = self.remaining()?;
        self.stream.set_write_timeout(Some(remaining))?;
        let result = self.stream.write(buffer);
        self.reject_if_expired()?;
        result
    }

    fn flush(&mut self) -> io::Result<()> {
        let remaining = self.remaining()?;
        self.stream.set_write_timeout(Some(remaining))?;
        let result = self.stream.flush();
        self.reject_if_expired()?;
        result
    }
}
