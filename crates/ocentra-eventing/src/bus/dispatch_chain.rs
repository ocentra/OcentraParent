//! Fail-closed ordered-dispatch causality.
//!
//! A root publication owns its aggregate semaphore. Consecutive recursion for
//! the same `(bus, aggregate)` is serialized by a child gate instead of
//! reacquiring that semaphore. Re-entering an earlier non-current pair is a
//! cycle; distinct pairs must be acquired in strict key order to prevent
//! cross-chain lock inversion. Every nested frame is capped by the depth limit,
//! and a publisher whose owning dispatch has ended cannot revive an expired
//! lease.

use std::sync::{atomic::AtomicU64, atomic::Ordering, Arc, Weak};

use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use crate::{AggregateKey, EventingError, ExpectValue};

use super::EventBus;

const MAX_ORDERED_DISPATCH_CHAIN_DEPTH: usize = 32;

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

    fn value(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Default)]
pub(super) struct DispatchChain {
    frames: Vec<DispatchFrame>,
}

impl DispatchChain {
    pub(super) fn root() -> Self {
        Self::default()
    }

    pub(super) async fn admit_ordered(
        &self,
        bus: &EventBus,
        aggregate_key: AggregateKey,
    ) -> Result<OrderedDispatchAdmission, EventingError> {
        let parent_leases = self.live_leases(&aggregate_key)?;
        self.ensure_depth_available()?;
        let requested = OrderedDispatchKey {
            bus_identity: bus.identity,
            aggregate_key,
        };
        let admission_gate = self.select_gate(bus, &requested)?;
        let permit = admission_gate
            .semaphore
            .acquire_owned()
            .await
            .expect_value("ordered dispatch gate remains open");
        let lease = Arc::new(DispatchFrameLease {
            _parent_leases: parent_leases,
            permit: Some(permit),
            aggregate_cleanup: admission_gate.aggregate_cleanup,
        });
        let mut frames = self.frames.clone();
        frames.push(DispatchFrame {
            key: requested,
            lease: Arc::downgrade(&lease),
            child_gate: Arc::new(Semaphore::new(1)),
        });
        Ok(OrderedDispatchAdmission {
            chain: Self { frames },
            _lease: lease,
        })
    }

    fn ensure_depth_available(&self) -> Result<(), EventingError> {
        if self.frames.len() < MAX_ORDERED_DISPATCH_CHAIN_DEPTH {
            return Ok(());
        }
        Err(EventingError::OrderedDispatchDepthExceeded {
            max_depth: MAX_ORDERED_DISPATCH_CHAIN_DEPTH,
        })
    }

    fn select_gate(
        &self,
        bus: &EventBus,
        requested: &OrderedDispatchKey,
    ) -> Result<AdmissionGate, EventingError> {
        let Some(current) = self.frames.last() else {
            return Ok(AdmissionGate::for_aggregate(bus, requested));
        };
        if current.key == *requested {
            return Ok(AdmissionGate::for_child(current));
        }
        self.reject_cycle(requested)?;
        ensure_lock_order(current, requested)?;
        Ok(AdmissionGate::for_aggregate(bus, requested))
    }

    fn reject_cycle(&self, requested: &OrderedDispatchKey) -> Result<(), EventingError> {
        if !self.frames.iter().any(|frame| frame.key == *requested) {
            return Ok(());
        }
        Err(EventingError::OrderedDispatchCycle {
            bus_identity: requested.bus_identity.value(),
            aggregate_key: requested.aggregate_key.clone(),
        })
    }

    pub(super) fn retain_live(&self) -> Result<DispatchChainGuard, EventingError> {
        let Some(current) = self.frames.last() else {
            return Ok(DispatchChainGuard {
                _leases: Vec::new(),
            });
        };
        Ok(DispatchChainGuard {
            _leases: self.live_leases(&current.key.aggregate_key)?,
        })
    }

    fn live_leases(
        &self,
        aggregate_key: &AggregateKey,
    ) -> Result<Vec<Arc<DispatchFrameLease>>, EventingError> {
        let mut leases = Vec::with_capacity(self.frames.len());
        for frame in &self.frames {
            leases.push(upgrade_frame_lease(frame, aggregate_key)?);
        }
        Ok(leases)
    }
}

#[derive(Clone)]
struct DispatchFrame {
    key: OrderedDispatchKey,
    lease: Weak<DispatchFrameLease>,
    child_gate: Arc<Semaphore>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct OrderedDispatchKey {
    bus_identity: EventBusIdentity,
    aggregate_key: AggregateKey,
}

fn upgrade_frame_lease(
    frame: &DispatchFrame,
    aggregate_key: &AggregateKey,
) -> Result<Arc<DispatchFrameLease>, EventingError> {
    frame
        .lease
        .upgrade()
        .ok_or_else(|| EventingError::OrderedDispatchChainExpired {
            aggregate_key: aggregate_key.clone(),
        })
}

fn ensure_lock_order(
    current: &DispatchFrame,
    requested: &OrderedDispatchKey,
) -> Result<(), EventingError> {
    if *requested > current.key {
        return Ok(());
    }
    Err(EventingError::OrderedDispatchLockOrderViolation {
        held_bus_identity: current.key.bus_identity.value(),
        held_aggregate_key: current.key.aggregate_key.clone(),
        requested_bus_identity: requested.bus_identity.value(),
        requested_aggregate_key: requested.aggregate_key.clone(),
    })
}

struct AdmissionGate {
    semaphore: Arc<Semaphore>,
    aggregate_cleanup: Option<AggregateGateCleanup>,
}

impl AdmissionGate {
    fn for_child(current: &DispatchFrame) -> Self {
        Self {
            semaphore: Arc::clone(&current.child_gate),
            aggregate_cleanup: None,
        }
    }

    fn for_aggregate(bus: &EventBus, requested: &OrderedDispatchKey) -> Self {
        let aggregate_gate = bus.aggregate_gate(&requested.aggregate_key);
        Self {
            semaphore: Arc::clone(&aggregate_gate),
            aggregate_cleanup: Some(AggregateGateCleanup {
                bus: bus.clone(),
                aggregate_key: requested.aggregate_key.clone(),
                aggregate_gate,
            }),
        }
    }
}

pub(super) struct OrderedDispatchAdmission {
    chain: DispatchChain,
    _lease: Arc<DispatchFrameLease>,
}

impl OrderedDispatchAdmission {
    pub(super) fn chain(&self) -> &DispatchChain {
        &self.chain
    }
}

pub(super) struct DispatchChainGuard {
    _leases: Vec<Arc<DispatchFrameLease>>,
}

struct DispatchFrameLease {
    _parent_leases: Vec<Arc<DispatchFrameLease>>,
    permit: Option<OwnedSemaphorePermit>,
    aggregate_cleanup: Option<AggregateGateCleanup>,
}

impl Drop for DispatchFrameLease {
    fn drop(&mut self) {
        drop(self.permit.take());
        if let Some(cleanup) = self.aggregate_cleanup.take() {
            cleanup
                .bus
                .release_idle_aggregate_gate(&cleanup.aggregate_key, &cleanup.aggregate_gate);
        }
    }
}

struct AggregateGateCleanup {
    bus: EventBus,
    aggregate_key: AggregateKey,
    aggregate_gate: Arc<Semaphore>,
}
