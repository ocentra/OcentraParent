use std::{
    collections::BTreeMap,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

pub type EventClockSleep<'a> = Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
pub type SharedEventClock = Arc<dyn EventClock>;

pub trait EventClock: Send + Sync + 'static {
    fn now(&self) -> EventClockInstant;
    fn sleep<'a>(&'a self, duration: Duration) -> EventClockSleep<'a>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EventClockInstant {
    elapsed: Duration,
}

impl EventClockInstant {
    pub fn duration_since(self, earlier: Self) -> Duration {
        self.elapsed.saturating_sub(earlier.elapsed)
    }

    pub fn checked_add(self, duration: Duration) -> Option<Self> {
        Some(Self {
            elapsed: self.elapsed.checked_add(duration)?,
        })
    }
}

impl From<Duration> for EventClockInstant {
    fn from(elapsed: Duration) -> Self {
        Self { elapsed }
    }
}

#[derive(Clone, Debug)]
pub struct SystemEventClock {
    started_at: Instant,
}

impl SystemEventClock {
    pub fn new() -> Self {
        Self {
            started_at: Instant::now(),
        }
    }

    pub fn shared() -> SharedEventClock {
        Arc::new(Self::new())
    }
}

impl Default for SystemEventClock {
    fn default() -> Self {
        Self::new()
    }
}

impl EventClock for SystemEventClock {
    fn now(&self) -> EventClockInstant {
        EventClockInstant::from(self.started_at.elapsed())
    }

    fn sleep<'a>(&'a self, duration: Duration) -> EventClockSleep<'a> {
        Box::pin(tokio::time::sleep(duration))
    }
}

#[derive(Clone, Debug, Default)]
pub struct ManualEventClock {
    state: Arc<Mutex<ManualEventClockState>>,
}

impl ManualEventClock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shared(&self) -> SharedEventClock {
        Arc::new(self.clone())
    }

    pub fn advance(&self, duration: Duration) {
        let ready_sleepers = {
            let mut state = self.state.lock().expect("manual event clock lock");
            state.now = state
                .now
                .checked_add(duration)
                .expect("manual event clock duration overflow");
            let ready_targets = state
                .sleepers
                .keys()
                .copied()
                .take_while(|target| *target <= state.now)
                .collect::<Vec<_>>();
            let mut ready_sleepers = Vec::new();
            for target in ready_targets {
                if let Some(mut sleepers) = state.sleepers.remove(&target) {
                    ready_sleepers.append(&mut sleepers);
                }
            }
            ready_sleepers
        };
        for sleeper in ready_sleepers {
            let _ = sleeper.send(());
        }
    }

    pub fn pending_sleep_count(&self) -> usize {
        self.state
            .lock()
            .expect("manual event clock lock")
            .sleepers
            .values()
            .map(Vec::len)
            .sum()
    }
}

impl EventClock for ManualEventClock {
    fn now(&self) -> EventClockInstant {
        EventClockInstant::from(self.state.lock().expect("manual event clock lock").now)
    }

    fn sleep<'a>(&'a self, duration: Duration) -> EventClockSleep<'a> {
        let receiver = {
            let mut state = self.state.lock().expect("manual event clock lock");
            let Some(target) = state.now.checked_add(duration) else {
                return Box::pin(async {});
            };
            if target <= state.now {
                return Box::pin(async {});
            }
            let (sender, receiver) = oneshot::channel();
            state.sleepers.entry(target).or_default().push(sender);
            receiver
        };
        Box::pin(async move {
            let _ = receiver.await;
        })
    }
}

#[derive(Default, Debug)]
struct ManualEventClockState {
    now: Duration,
    sleepers: BTreeMap<Duration, Vec<oneshot::Sender<()>>>,
}
