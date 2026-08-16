use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{Duration, Instant};

use super::{TargetedArpRefreshAttemptKey, TargetedArpRefreshTarget};

pub const TARGETED_ARP_REFRESH_THROTTLE_MS: u64 = 10_000;

pub static TARGETED_ARP_REFRESH_ATTEMPTS: OnceLock<
    Mutex<HashMap<TargetedArpRefreshAttemptKey, Instant>>,
> = OnceLock::new();

pub fn targeted_arp_refresh_throttled(target: &TargetedArpRefreshTarget) -> Option<Instant> {
    targeted_arp_refresh_throttled_at(target, Instant::now())
}

pub fn targeted_arp_refresh_throttled_at(
    target: &TargetedArpRefreshTarget,
    now: Instant,
) -> Option<Instant> {
    let key = targeted_arp_refresh_attempt_key(target);
    let mut attempts = targeted_arp_refresh_attempts_lock();
    if let Some(previous_attempt) = attempts.get(&key).copied() {
        if now.duration_since(previous_attempt)
            < Duration::from_millis(TARGETED_ARP_REFRESH_THROTTLE_MS)
        {
            return Some(previous_attempt);
        }
    }
    attempts.insert(key, now);
    None
}

pub fn targeted_arp_refresh_attempts_lock(
) -> MutexGuard<'static, HashMap<TargetedArpRefreshAttemptKey, Instant>> {
    let attempts = TARGETED_ARP_REFRESH_ATTEMPTS.get_or_init(|| Mutex::new(HashMap::new()));
    match attempts.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            attempts.clear_poison();
            poisoned.into_inner()
        }
    }
}

pub fn targeted_arp_refresh_attempt_key(
    target: &TargetedArpRefreshTarget,
) -> TargetedArpRefreshAttemptKey {
    TargetedArpRefreshAttemptKey {
        ip_address: target.ip_address,
        network_interface: target
            .network_interface
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_ascii_lowercase),
    }
}

pub fn clear_targeted_arp_refresh_attempts() {
    if TARGETED_ARP_REFRESH_ATTEMPTS.get().is_some() {
        targeted_arp_refresh_attempts_lock().clear();
    }
}
