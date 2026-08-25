use std::sync::Arc;

use super::app_game_linux_docker_host_preflight_cleanup_owner::{
    recover_lock, recover_wait, CleanupMailbox,
};
pub(super) fn cleanup_worker(mailbox: Arc<(std::sync::Mutex<CleanupMailbox>, std::sync::Condvar)>) {
    let (lock, wake) = &*mailbox;
    let mut mailbox = recover_lock(lock);
    while mailbox.owner.is_none() && !mailbox.stop {
        mailbox = recover_wait(wake, mailbox);
    }
    if mailbox.stop {
        return;
    }
    let Some(mut owner) = mailbox.owner.take() else {
        return;
    };
    drop(mailbox);
    owner.run();
}
