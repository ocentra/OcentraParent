use tokio::time::{sleep_until, Instant};

use super::super::LanPassiveDiscoveryRefreshSignalReceiver;

pub(super) enum ReconciliationWake {
    DeliberateSignal(bool),
    PassiveSignal(bool),
    AutomaticRefresh,
}

pub(super) async fn next(
    receiver: &mut LanPassiveDiscoveryRefreshSignalReceiver,
    automatic_refresh_at: Option<Instant>,
) -> ReconciliationWake {
    match automatic_refresh_at {
        Some(refresh_at) => next_with_timer(receiver, refresh_at).await,
        None => next_without_timer(receiver).await,
    }
}

async fn next_with_timer(
    receiver: &mut LanPassiveDiscoveryRefreshSignalReceiver,
    refresh_at: Instant,
) -> ReconciliationWake {
    tokio::select! {
        biased;
        changed = receiver.deliberate.changed() => ReconciliationWake::DeliberateSignal(changed.is_ok()),
        _ = sleep_until(refresh_at) => ReconciliationWake::AutomaticRefresh,
        changed = receiver.passive.changed() => ReconciliationWake::PassiveSignal(changed.is_ok()),
    }
}

async fn next_without_timer(
    receiver: &mut LanPassiveDiscoveryRefreshSignalReceiver,
) -> ReconciliationWake {
    tokio::select! {
        biased;
        changed = receiver.deliberate.changed() => ReconciliationWake::DeliberateSignal(changed.is_ok()),
        changed = receiver.passive.changed() => ReconciliationWake::PassiveSignal(changed.is_ok()),
    }
}
