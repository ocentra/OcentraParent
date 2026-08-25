use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_EVENT_BUS_IDENTITY: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct EventBusIdentity(u64);

impl EventBusIdentity {
    pub(super) fn generated() -> Self {
        let identity = NEXT_EVENT_BUS_IDENTITY
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .unwrap_or_else(|_| std::process::abort());
        Self(identity)
    }

    pub(super) fn value(self) -> u64 {
        self.0
    }
}
