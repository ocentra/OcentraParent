use std::net::UdpSocket;

pub(super) struct ReadTimeoutRestoreGuard<'a> {
    socket: &'a UdpSocket,
    previous_timeout: Option<std::time::Duration>,
    armed: bool,
}

impl<'a> ReadTimeoutRestoreGuard<'a> {
    pub(super) fn new(
        socket: &'a UdpSocket,
        previous_timeout: Option<std::time::Duration>,
    ) -> Self {
        Self {
            socket,
            previous_timeout,
            armed: true,
        }
    }

    pub(super) fn restore(&mut self) -> std::io::Result<()> {
        if !self.armed {
            return Ok(());
        }
        let result = self.socket.set_read_timeout(self.previous_timeout);
        if result.is_ok() {
            self.armed = false;
        }
        result
    }
}

impl Drop for ReadTimeoutRestoreGuard<'_> {
    fn drop(&mut self) {
        if self.armed {
            let _ = self.socket.set_read_timeout(self.previous_timeout);
        }
    }
}
