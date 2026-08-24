use std::{
    os::fd::{AsFd, AsRawFd},
    path::Path,
    time::Duration,
};

use nix::{
    errno::Errno,
    poll::{poll, PollFd, PollFlags, PollTimeout},
    sys::socket::{
        connect, getsockopt, socket, sockopt::SocketError, AddressFamily, SockFlag, SockType,
        UnixAddr,
    },
};

use super::{linux_socket_security::validated_socket, LinuxProbeDeadline};

const SOCKET_CONNECT_LIMIT: Duration = Duration::from_millis(100);

pub(super) fn socket_ready(path: &Path, deadline: &LinuxProbeDeadline) -> Option<()> {
    let Some((canonical_path, root)) = validated_socket(path) else {
        return None;
    };
    let remaining = deadline.remaining();
    if remaining.is_zero() {
        return None;
    }
    let Some((revalidated_path, revalidated_root)) = validated_socket(path) else {
        return None;
    };
    if revalidated_path != canonical_path || revalidated_root != root {
        return None;
    }
    let Ok(address) = UnixAddr::new(&canonical_path) else {
        return None;
    };
    let Ok(socket) = socket(
        AddressFamily::Unix,
        SockType::Stream,
        SockFlag::SOCK_NONBLOCK | SockFlag::SOCK_CLOEXEC,
        None,
    ) else {
        return None;
    };
    match connect(socket.as_raw_fd(), &address) {
        Ok(()) => Some(()),
        Err(Errno::EINPROGRESS | Errno::EALREADY | Errno::EAGAIN) => {
            wait_for_socket(socket.as_fd(), deadline)
        }
        Err(_) => None,
    }
}

fn wait_for_socket(fd: impl AsFd, deadline: &LinuxProbeDeadline) -> Option<()> {
    let remaining = deadline.remaining();
    if remaining.is_zero() {
        return None;
    }
    let timeout =
        PollTimeout::try_from(SOCKET_CONNECT_LIMIT.min(remaining)).unwrap_or(PollTimeout::ZERO);
    let mut poll_fds = [PollFd::new(
        fd.as_fd(),
        PollFlags::POLLOUT | PollFlags::POLLERR | PollFlags::POLLHUP,
    )];
    if poll(&mut poll_fds, timeout).is_err() {
        return None;
    }
    let Some(revents) = poll_fds.first().and_then(|poll_fd| poll_fd.revents()) else {
        return None;
    };
    if !revents.intersects(PollFlags::POLLOUT | PollFlags::POLLERR | PollFlags::POLLHUP) {
        return None;
    }
    let Ok(error) = getsockopt(&fd, SocketError) else {
        return None;
    };
    (error == 0).then_some(())
}
