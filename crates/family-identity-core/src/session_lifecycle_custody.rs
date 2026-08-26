#![forbid(unsafe_code)]

//! Repository-owned browser-session custody.

use std::time::Duration;

#[path = "session_lifecycle/boundary/audit_delivery.rs"]
pub mod audit_delivery;
#[path = "session_lifecycle/boundary/authenticated_parent_local_bridge.rs"]
pub mod authenticated_parent_local_bridge;
#[path = "session_lifecycle/boundary/browser_credentials.rs"]
pub mod browser_credentials;
#[path = "session_lifecycle/boundary/parent_local_bridge.rs"]
pub mod parent_local_bridge;
#[path = "session_lifecycle/boundary/parent_local_bridge_audit.rs"]
pub mod parent_local_bridge_audit;
#[path = "session_lifecycle/session_record.rs"]
pub(crate) mod record;
#[path = "session_lifecycle/boundary/storage_values.rs"]
pub(crate) mod storage_values;

#[derive(Clone, Debug)]
pub struct SessionLifecyclePolicy {
    pub(crate) access_ttl_millis: i64,
    pub(crate) refresh_ttl_millis: i64,
    pub(crate) freshness_ttl_millis: i64,
    pub(crate) clock_skew_millis: i64,
    pub(crate) audit_delivery_lease_millis: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionLifecyclePolicyError {
    ZeroDuration,
    DurationTooLarge,
    InvalidOrdering,
}

impl SessionLifecyclePolicy {
    pub fn new(
        access_ttl: Duration,
        refresh_ttl: Duration,
        freshness_ttl: Duration,
        clock_skew: Duration,
    ) -> Result<Self, SessionLifecyclePolicyError> {
        let access_ttl_millis = duration_millis(access_ttl)?;
        let refresh_ttl_millis = duration_millis(refresh_ttl)?;
        let freshness_ttl_millis = duration_millis(freshness_ttl)?;
        let clock_skew_millis = duration_millis(clock_skew)?;
        if freshness_ttl_millis > access_ttl_millis
            || access_ttl_millis > refresh_ttl_millis
            || clock_skew_millis > access_ttl_millis
        {
            return Err(SessionLifecyclePolicyError::InvalidOrdering);
        }
        Ok(Self {
            access_ttl_millis,
            refresh_ttl_millis,
            freshness_ttl_millis,
            clock_skew_millis,
            audit_delivery_lease_millis: 5 * 60 * 1_000,
        })
    }

    pub fn with_audit_delivery_lease(
        mut self,
        delivery_lease: Duration,
    ) -> Result<Self, SessionLifecyclePolicyError> {
        self.audit_delivery_lease_millis = duration_millis(delivery_lease)?;
        Ok(self)
    }

    pub fn production_default() -> Self {
        Self::new(
            Duration::from_secs(15 * 60),
            Duration::from_secs(30 * 24 * 60 * 60),
            Duration::from_secs(5 * 60),
            Duration::from_secs(2 * 60),
        )
        .expect("production session policy is valid")
    }
}

fn duration_millis(value: Duration) -> Result<i64, SessionLifecyclePolicyError> {
    if value.is_zero() {
        return Err(SessionLifecyclePolicyError::ZeroDuration);
    }
    i64::try_from(value.as_millis()).map_err(|_| SessionLifecyclePolicyError::DurationTooLarge)
}
