//! Fail-closed ordered-dispatch causality.
//!
//! A root publication owns its aggregate semaphore. Consecutive recursion for
//! the same `(bus, aggregate)` is serialized by a child gate instead of
//! reacquiring that semaphore. Re-entering an earlier non-current pair is a
//! cycle; distinct pairs must be acquired in strict key order to prevent
//! cross-chain lock inversion. Every nested frame is capped by the depth limit,
//! and handler-scope cancellation prevents descendant work from outliving a
//! completed, failed, or timed-out handler.

use std::sync::{Arc, Weak};

use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use crate::{AggregateKey, EventingError, ExpectValue};

use super::{
    handler_scope::{HandlerScopeChain, HandlerScopeGuard},
    identity::EventBusIdentity,
    EventBus,
};

const MAX_ORDERED_DISPATCH_CHAIN_DEPTH: usize = 32;

#[derive(Clone, Default)]
pub(super) struct DispatchChain {
    frames: Vec<DispatchFrame>,
    handler_scopes: HandlerScopeChain,
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
        self.ensure_live_for(&aggregate_key)?;
        self.ensure_depth_available()?;
        let requested = OrderedDispatchPair {
            bus_identity: bus.identity,
            aggregate_key,
        };
        let admission_gate = self.select_gate(bus, &requested)?;
        let permit = self.acquire_permit(admission_gate.semaphore).await?;
        let lease = Arc::new(DispatchFrameLease {
            permit: Some(permit),
            aggregate_cleanup: admission_gate.aggregate_cleanup,
        });
        // CLONE-JUSTIFICATION: a child chain owns a snapshot of immutable
        // causal ancestry while its parent continues dispatching.
        let mut frames = self.frames.clone();
        frames.push(DispatchFrame {
            key: requested,
            lease: Arc::downgrade(&lease),
            child_gate: Arc::new(Semaphore::new(1)),
        });
        Ok(OrderedDispatchAdmission {
            chain: Self {
                frames,
                // CLONE-JUSTIFICATION: every descendant must retain all
                // ancestor cancellation scopes across task boundaries.
                handler_scopes: self.handler_scopes.clone(),
            },
            _lease: lease,
        })
    }

    pub(super) fn scoped_to_handler(&self) -> HandlerScopedDispatchChain {
        let binding = self.handler_scopes.append();
        HandlerScopedDispatchChain {
            chain: Self {
                // CLONE-JUSTIFICATION: each handler attempt extends an
                // immutable snapshot without mutating its publisher's chain.
                frames: self.frames.clone(),
                handler_scopes: binding.chain,
            },
            guard: binding.guard,
        }
    }

    pub(super) fn ensure_live(&self) -> Result<(), EventingError> {
        self.handler_scopes.ensure_active()?;
        let Some(current) = self.frames.last() else {
            return Ok(());
        };
        self.ensure_frames_live(&current.key.aggregate_key)
    }

    pub(super) async fn cancelled(&self) {
        self.handler_scopes.cancelled().await;
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
        requested: &OrderedDispatchPair,
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

    fn reject_cycle(&self, requested: &OrderedDispatchPair) -> Result<(), EventingError> {
        if !self.frames.iter().any(|frame| frame.key == *requested) {
            return Ok(());
        }
        // CLONE-JUSTIFICATION: the typed error must own the rejected aggregate
        // after this borrowed admission request returns.
        Err(EventingError::OrderedDispatchCycle {
            bus_identity: requested.bus_identity.value(),
            aggregate_key: requested.aggregate_key.clone(),
        })
    }

    async fn acquire_permit(
        &self,
        semaphore: Arc<Semaphore>,
    ) -> Result<OwnedSemaphorePermit, EventingError> {
        let acquire = semaphore.acquire_owned();
        tokio::pin!(acquire);
        // CANCEL-SAFE: dropping an acquire future removes only its waiter; it
        // cannot leak a permit because ownership starts on successful return.
        tokio::select! {
            biased;
            _ = self.handler_scopes.cancelled() => Err(EventingError::CausalDispatchCancelled),
            permit = acquire => Ok(permit.expect_value("ordered dispatch gate remains open")),
        }
    }

    fn ensure_live_for(&self, aggregate_key: &AggregateKey) -> Result<(), EventingError> {
        self.handler_scopes.ensure_active()?;
        self.ensure_frames_live(aggregate_key)
    }

    fn ensure_frames_live(&self, aggregate_key: &AggregateKey) -> Result<(), EventingError> {
        for frame in &self.frames {
            ensure_frame_live(frame, aggregate_key)?;
        }
        Ok(())
    }
}

pub(super) struct HandlerScopedDispatchChain {
    pub(super) chain: DispatchChain,
    pub(super) guard: HandlerScopeGuard,
}

#[derive(Clone)]
struct DispatchFrame {
    key: OrderedDispatchPair,
    lease: Weak<DispatchFrameLease>,
    child_gate: Arc<Semaphore>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct OrderedDispatchPair {
    bus_identity: EventBusIdentity,
    aggregate_key: AggregateKey,
}

fn ensure_frame_live(
    frame: &DispatchFrame,
    aggregate_key: &AggregateKey,
) -> Result<(), EventingError> {
    if frame.lease.strong_count() > 0 {
        return Ok(());
    }
    // CLONE-JUSTIFICATION: the typed error crosses this borrowed validation
    // boundary and therefore owns the aggregate identity it reports.
    Err(EventingError::OrderedDispatchChainExpired {
        aggregate_key: aggregate_key.clone(),
    })
}

fn ensure_lock_order(
    current: &DispatchFrame,
    requested: &OrderedDispatchPair,
) -> Result<(), EventingError> {
    if *requested > current.key {
        return Ok(());
    }
    // CLONE-JUSTIFICATION: the typed error owns both aggregate identities so
    // callers can inspect the lock-order rejection after this borrow ends.
    Err(EventingError::OrderedDispatchLockOrderViolation {
        held_bus_identity: current.key.bus_identity.value(),
        held_aggregate_key: current.key.aggregate_key.clone(),
        requested_bus_identity: requested.bus_identity.value(),
        // CLONE-JUSTIFICATION: the rejection report independently owns the
        // requested aggregate after this borrowed comparison returns.
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

    fn for_aggregate(bus: &EventBus, requested: &OrderedDispatchPair) -> Self {
        let aggregate_gate = bus.aggregate_gate(&requested.aggregate_key);
        Self {
            semaphore: Arc::clone(&aggregate_gate),
            aggregate_cleanup: Some(AggregateGateCleanup {
                // CLONE-JUSTIFICATION: cleanup must retain the exact bus and
                // aggregate identity until the owned permit is dropped.
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

struct DispatchFrameLease {
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
