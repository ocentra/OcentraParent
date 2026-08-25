use std::{
    io,
    process::ExitStatus,
    time::{Duration, Instant},
};

#[cfg(unix)]
use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;
use tokio::process::{Child as TokioChild, ChildStdout};

#[cfg(target_os = "linux")]
use rustix::{
    event::{poll, PollFd, PollFlags, Timespec},
    process::{pidfd_open, Pid, PidfdFlags},
};

use super::app_game_linux_docker_host_preflight_cleanup_owner::ReservedCleanupOwner;
use super::app_game_linux_docker_host_preflight_group::{
    direct_child_kill_succeeded_or_gone, DockerProcessGroup,
};

pub(super) const DOCKER_PROBE_POLL_INTERVAL: Duration = Duration::from_millis(10);
const CHILD_IDENTITY_MISSING: &str = "docker probe child has no process identity";
const CHILD_IDENTITY_INVALID: &str = "docker probe child has an invalid process identity";
const CHILD_OWNERSHIP_TRANSFERRED: &str = "docker probe child ownership was transferred";
#[cfg(unix)]
const LEADER_EXIT_OBSERVATION_UNAVAILABLE: &str =
    proof::DOCKER_PROBE_LEADER_EXIT_OBSERVATION_UNAVAILABLE;

/// Owns the probe child and, on Unix, the process group created for it.
///
/// The Linux pidfd observes direct-child exit without reaping it. The owner
/// sends the group signal while that original leader is still unreaped, then
/// reaps it and never signals or polls a numeric PGID after that point. If a
/// signal or reap cannot be proven safe before the absolute request deadline,
/// the child and group are handed to the already-reserved cleanup owner.
pub(super) struct DockerProcessSupervisor {
    pub(super) child: Option<TokioChild>,
    pub(super) group: DockerProcessGroup,
    pub(super) status: Option<ExitStatus>,
    pub(super) group_signal_sent: bool,
    pub(super) cleanup_complete: bool,
    pub(super) cleanup_failed: bool,
    pub(super) cleanup_deadline: Instant,
    pub(super) cleanup_owner: ReservedCleanupOwner,
    #[cfg(target_os = "linux")]
    leader_pidfd: Option<rustix::fd::OwnedFd>,
}

pub(super) type CleanupProgress = Result<Option<ExitStatus>, ()>;

impl DockerProcessSupervisor {
    pub(super) fn from_spawned_child(
        mut child: TokioChild,
        cleanup_owner: ReservedCleanupOwner,
        cleanup_deadline: Instant,
    ) -> io::Result<Self> {
        let Some(id) = child.id() else {
            let _ = child.start_kill();
            let _ = child.try_wait();
            return Err(io::Error::new(io::ErrorKind::Other, CHILD_IDENTITY_MISSING));
        };
        let Some(group) = DockerProcessGroup::from_pid(id) else {
            let _ = child.start_kill();
            let _ = child.try_wait();
            return Err(io::Error::new(io::ErrorKind::Other, CHILD_IDENTITY_INVALID));
        };
        #[cfg(target_os = "linux")]
        let leader_pidfd = Pid::from_raw(i32::try_from(id).unwrap_or_default())
            .and_then(|pid| pidfd_open(pid, PidfdFlags::NONBLOCK).ok());
        Ok(Self {
            child: Some(child),
            group,
            status: None,
            group_signal_sent: false,
            cleanup_complete: false,
            cleanup_deadline,
            cleanup_owner,
            cleanup_failed: false,
            #[cfg(target_os = "linux")]
            leader_pidfd,
        })
    }

    pub(super) fn take_stdout(&mut self) -> Option<ChildStdout> {
        self.child.as_mut()?.stdout.take()
    }

    pub(super) fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        if let Some(status) = self.status {
            return Ok(Some(status));
        }
        let child = self
            .child
            .as_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, CHILD_OWNERSHIP_TRANSFERRED))?;
        let status = child.try_wait()?;
        if let Some(status) = status {
            self.status = Some(status);
        }
        Ok(status)
    }

    pub(super) fn request_direct_kill(&mut self) -> bool {
        let Some(child) = self.child.as_mut() else {
            return false;
        };
        direct_child_kill_succeeded_or_gone(child)
    }

    pub(super) fn leader_exit_ready(&mut self) -> io::Result<bool> {
        #[cfg(target_os = "linux")]
        {
            let Some(pidfd) = self.leader_pidfd.as_ref() else {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    LEADER_EXIT_OBSERVATION_UNAVAILABLE,
                ));
            };
            let mut poll_fd = PollFd::new(pidfd, PollFlags::IN | PollFlags::ERR | PollFlags::HUP);
            let ready = poll(
                std::slice::from_mut(&mut poll_fd),
                Some(&Timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                }),
            )?;
            return Ok(ready > 0
                && poll_fd
                    .revents()
                    .intersects(PollFlags::IN | PollFlags::ERR | PollFlags::HUP));
        }
        #[cfg(windows)]
        {
            return self.try_wait().map(|status| status.is_some());
        }
        #[cfg(all(unix, not(target_os = "linux")))]
        {
            Err(io::Error::new(
                io::ErrorKind::Other,
                LEADER_EXIT_OBSERVATION_UNAVAILABLE,
            ))
        }
    }

    pub(super) fn cleanup_after_leader_exit(&mut self) -> CleanupProgress {
        if !self.group_signal_sent && !self.group.kill() {
            return Err(());
        }
        if !self.group_signal_sent {
            self.group_signal_sent = true;
        }
        self.try_wait().map_err(|_| ())
    }

    pub(super) fn mark_cleanup_complete(&mut self) {
        self.cleanup_complete = true;
    }

    pub(super) fn cleanup_step(&mut self) -> CleanupProgress {
        if self.status.is_some() {
            return self
                .group_signal_sent
                .then_some(self.status)
                .flatten()
                .map(Some)
                .ok_or(());
        }
        // Kill the exact unreaped leader first. Its retained child handle keeps
        // the PID/PGID from being reused while the group signal is sent.
        if !self.request_direct_kill() || (!self.group_signal_sent && !self.group.kill()) {
            return Err(());
        }
        self.group_signal_sent = true;
        self.try_wait().map_err(|_| ())
    }

    pub(super) fn cleanup_deadline(&self) -> Instant {
        self.cleanup_deadline
    }
}
