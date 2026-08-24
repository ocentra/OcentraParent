use std::{
    os::fd::{AsFd, AsRawFd},
    path::Path,
    time::{Duration, Instant},
};

use nix::{
    errno::Errno,
    poll::{poll, PollFd, PollFlags, PollTimeout},
    sys::socket::{
        connect, getsockopt, socket, sockopt::SocketError, AddressFamily, SockFlag, SockType,
        UnixAddr,
    },
};

use super::linux_socket_security::validated_socket;

const SOCKET_CONNECT_LIMIT: Duration = Duration::from_millis(100);

pub(super) fn socket_ready(path: &Path, deadline: Instant) -> bool {
    let Some((canonical_path, root)) = validated_socket(path) else {
        return false;
    };
    let remaining = deadline.saturating_duration_since(Instant::now());
    if remaining.is_zero() {
        return false;
    }
    let Some((revalidated_path, revalidated_root)) = validated_socket(path) else {
        return false;
    };
    if revalidated_path != canonical_path || revalidated_root != root {
        return false;
    }
    let Ok(address) = UnixAddr::new(&canonical_path) else {
        return false;
    };
    let Ok(socket) = socket(
        AddressFamily::Unix,
        SockType::Stream,
        SockFlag::SOCK_NONBLOCK | SockFlag::SOCK_CLOEXEC,
        None,
    ) else {
        return false;
    };
    match connect(socket.as_raw_fd(), &address) {
        Ok(()) => true,
        Err(Errno::EINPROGRESS | Errno::EALREADY | Errno::EAGAIN) => {
            wait_for_socket(socket.as_fd(), deadline)
        }
        Err(_) => false,
    }
}

fn wait_for_socket(fd: impl AsFd, deadline: Instant) -> bool {
    let remaining = deadline.saturating_duration_since(Instant::now());
    if remaining.is_zero() {
        return false;
    }
    let timeout =
        PollTimeout::try_from(SOCKET_CONNECT_LIMIT.min(remaining)).unwrap_or(PollTimeout::ZERO);
    let mut poll_fds = [PollFd::new(
        fd.as_fd(),
        PollFlags::POLLOUT | PollFlags::POLLERR | PollFlags::POLLHUP,
    )];
    if poll(&mut poll_fds, timeout).is_err() {
        return false;
    }
    let Some(revents) = poll_fds[0].revents() else {
        return false;
    };
    if !revents.intersects(PollFlags::POLLOUT | PollFlags::POLLERR | PollFlags::POLLHUP) {
        return false;
    }
    getsockopt(&fd, SocketError)
        .map(|error| error == 0)
        .unwrap_or(false)
}
