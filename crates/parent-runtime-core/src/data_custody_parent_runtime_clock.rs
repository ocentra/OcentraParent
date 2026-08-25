use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, SecondsFormat, Utc};
use ocentra_eventing::error::EventingError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuntimeClockError {
    NotRecovered,
    InvalidDurableTimestamp,
    NonMonotonicTimestamp,
    ForwardSkew,
    Overflow,
}

#[derive(Debug, Default)]
pub(crate) struct DataCustodyRuntimeClock {
    last_committed: Option<DateTime<Utc>>,
    last_issued: Option<DateTime<Utc>>,
    recovered: bool,
}

pub(crate) type SharedDataCustodyRuntimeClock = Arc<Mutex<DataCustodyRuntimeClock>>;

const MAX_FORWARD_SKEW: Duration = Duration::seconds(86_400);

impl DataCustodyRuntimeClock {
    pub(crate) fn shared() -> SharedDataCustodyRuntimeClock {
        Arc::new(Mutex::new(Self::default()))
    }

    pub(crate) fn next_timestamp(
        clock: &SharedDataCustodyRuntimeClock,
    ) -> Result<String, RuntimeClockError> {
        let mut clock = clock
            .lock()
            .map_err(|_| RuntimeClockError::NonMonotonicTimestamp)?;
        if !clock.recovered {
            return Err(RuntimeClockError::NotRecovered);
        }
        let predecessor = clock.last_issued.or(clock.last_committed);
        let minimum = predecessor
            .map(|value| {
                value
                    .checked_add_signed(Duration::nanoseconds(1))
                    .ok_or(RuntimeClockError::Overflow)
            })
            .transpose()?;
        let observed = Utc::now();
        if let Some(predecessor) = predecessor {
            if observed < predecessor {
                return Err(RuntimeClockError::NonMonotonicTimestamp);
            }
            let maximum = predecessor
                .checked_add_signed(MAX_FORWARD_SKEW)
                .ok_or(RuntimeClockError::Overflow)?;
            if observed > maximum {
                return Err(RuntimeClockError::ForwardSkew);
            }
        }
        let timestamp = minimum.map_or(observed, |minimum| {
            if observed < minimum {
                minimum
            } else {
                observed
            }
        });
        clock.last_issued = Some(timestamp);
        Ok(timestamp.to_rfc3339_opts(SecondsFormat::Nanos, true))
    }

    pub(crate) fn commit_timestamp(
        clock: &SharedDataCustodyRuntimeClock,
        timestamp: &str,
    ) -> Result<(), RuntimeClockError> {
        let parsed = parse_timestamp(timestamp)?;
        let mut clock = clock
            .lock()
            .map_err(|_| RuntimeClockError::NonMonotonicTimestamp)?;
        let observed = Utc::now();
        if clock
            .last_committed
            .is_some_and(|previous| observed < previous)
        {
            return Err(RuntimeClockError::NonMonotonicTimestamp);
        }
        let durable_floor = clock.last_committed.max(clock.last_issued);
        if durable_floor.is_some_and(|previous| parsed < previous) {
            return Err(RuntimeClockError::NonMonotonicTimestamp);
        }
        if parsed
            > observed
                .checked_add_signed(MAX_FORWARD_SKEW)
                .ok_or(RuntimeClockError::Overflow)?
        {
            return Err(RuntimeClockError::ForwardSkew);
        }
        clock.last_committed = Some(parsed);
        clock.last_issued = Some(parsed);
        Ok(())
    }

    pub(crate) fn ensure_recovered(
        clock: &SharedDataCustodyRuntimeClock,
    ) -> Result<(), RuntimeClockError> {
        let clock = clock
            .lock()
            .map_err(|_| RuntimeClockError::NonMonotonicTimestamp)?;
        if clock.recovered {
            Ok(())
        } else {
            Err(RuntimeClockError::NotRecovered)
        }
    }

    pub(crate) fn mark_recovered(
        clock: &SharedDataCustodyRuntimeClock,
    ) -> Result<(), RuntimeClockError> {
        let mut clock = clock
            .lock()
            .map_err(|_| RuntimeClockError::NonMonotonicTimestamp)?;
        clock.recovered = true;
        Ok(())
    }

    pub(crate) fn begin_recovery(
        clock: &SharedDataCustodyRuntimeClock,
    ) -> Result<(), RuntimeClockError> {
        let mut clock = clock
            .lock()
            .map_err(|_| RuntimeClockError::NonMonotonicTimestamp)?;
        clock.last_committed = None;
        clock.last_issued = None;
        clock.recovered = false;
        Ok(())
    }
}

fn parse_timestamp(timestamp: &str) -> Result<DateTime<Utc>, RuntimeClockError> {
    DateTime::parse_from_rfc3339(timestamp)
        .map(|value| value.with_timezone(&Utc))
        .map_err(|_| RuntimeClockError::InvalidDurableTimestamp)
}

pub(crate) fn clock_error(error: RuntimeClockError) -> EventingError {
    EventingError::InvalidValue {
        field: "data_custody_runtime_clock",
        value: format!("{error:?}"),
    }
}
