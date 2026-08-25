use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use tokio::sync::RwLock;

use crate::{
    EventQueue, EventQueuePolicy, HandlerExecutionPolicy, JournalPolicy, RequestRegistry,
    SharedEventClock, SharedEventJournal,
};

use super::{
    active_dispatch::ActiveDispatchTracker, identity::EventBusIdentity,
    publisher::RootEventPublisher, EventBus, EventBusLifecycleState,
};

impl EventBus {
    /// Constructs an event bus together with its nonforgeable root publication
    /// capability. The returned wrapper dereferences to `EventBus` for
    /// subscription and lifecycle operations.
    pub fn new() -> RootEventPublisher {
        RootEventPublisher::for_bus(Self::build())
    }

    fn build() -> Self {
        Self {
            identity: EventBusIdentity::generated(),
            registry: Arc::new(Mutex::new(BTreeMap::new())),
            stored_journal: Arc::new(RwLock::new(Vec::new())),
            dead_letters: Arc::new(RwLock::new(Vec::new())),
            aggregate_gates: Arc::new(Mutex::new(BTreeMap::new())),
            handler_policy: HandlerExecutionPolicy::default(),
            queue: EventQueue::new(EventQueuePolicy::default()),
            requests: RequestRegistry::default(),
            journal_policy: JournalPolicy::default(),
            event_journal: None,
            clock: crate::SystemEventClock::shared(),
            shutdown: Arc::new(Mutex::new(EventBusLifecycleState::Active)),
            active_dispatches: ActiveDispatchTracker::default(),
        }
    }

    pub fn with_clock(clock: SharedEventClock) -> RootEventPublisher {
        RootEventPublisher::for_bus(Self {
            clock,
            ..Self::build()
        })
    }

    pub fn with_handler_policy(policy: HandlerExecutionPolicy) -> RootEventPublisher {
        RootEventPublisher::for_bus(Self {
            handler_policy: policy,
            ..Self::build()
        })
    }

    pub fn with_handler_policy_and_clock(
        policy: HandlerExecutionPolicy,
        clock: SharedEventClock,
    ) -> RootEventPublisher {
        RootEventPublisher::for_bus(Self {
            handler_policy: policy,
            clock,
            ..Self::build()
        })
    }

    pub fn with_queue_policy(policy: EventQueuePolicy) -> RootEventPublisher {
        RootEventPublisher::for_bus(Self {
            queue: EventQueue::new(policy),
            ..Self::build()
        })
    }

    pub fn with_queue_policy_and_clock(
        policy: EventQueuePolicy,
        clock: SharedEventClock,
    ) -> RootEventPublisher {
        RootEventPublisher::for_bus(Self {
            queue: EventQueue::new(policy),
            clock,
            ..Self::build()
        })
    }

    pub fn with_policies(
        handler_policy: HandlerExecutionPolicy,
        queue_policy: EventQueuePolicy,
    ) -> RootEventPublisher {
        RootEventPublisher::for_bus(Self {
            handler_policy,
            queue: EventQueue::new(queue_policy),
            ..Self::build()
        })
    }

    pub fn with_policies_and_clock(
        handler_policy: HandlerExecutionPolicy,
        queue_policy: EventQueuePolicy,
        clock: SharedEventClock,
    ) -> RootEventPublisher {
        RootEventPublisher::for_bus(Self {
            handler_policy,
            queue: EventQueue::new(queue_policy),
            clock,
            ..Self::build()
        })
    }

    pub fn with_journal(policy: JournalPolicy, journal: SharedEventJournal) -> RootEventPublisher {
        RootEventPublisher::for_bus(Self {
            journal_policy: policy,
            event_journal: Some(journal),
            ..Self::build()
        })
    }

    pub fn with_journal_and_queue_policy(
        journal_policy: JournalPolicy,
        journal: SharedEventJournal,
        queue_policy: EventQueuePolicy,
    ) -> RootEventPublisher {
        RootEventPublisher::for_bus(Self {
            journal_policy,
            event_journal: Some(journal),
            queue: EventQueue::new(queue_policy),
            ..Self::build()
        })
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::build()
    }
}
