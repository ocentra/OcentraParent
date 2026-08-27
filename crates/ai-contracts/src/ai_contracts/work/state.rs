use super::{AiRetryPolicy, AiWorkState};
use serde::Deserialize;

impl AiWorkState {
    pub fn can_transition_from(self, previous: Option<Self>) -> bool {
        matches!(
            (previous, self),
            (None, Self::Queued)
                | (
                    Some(Self::Queued),
                    Self::Claimed | Self::Cancelled | Self::ManualRequired
                )
                | (
                    Some(Self::Claimed),
                    Self::Running | Self::Cancelled | Self::Failed | Self::ManualRequired
                )
                | (
                    Some(Self::Running),
                    Self::Succeeded
                        | Self::Failed
                        | Self::Cancelled
                        | Self::TimedOut
                        | Self::ManualRequired
                )
        )
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Succeeded
                | Self::Failed
                | Self::Cancelled
                | Self::TimedOut
                | Self::ManualRequired
        )
    }
}

impl AiRetryPolicy {
    pub fn new(max_attempts: u16, retry_after_ms: Option<u64>) -> Result<Self, &'static str> {
        if max_attempts == 0 {
            return Err("AI retry policy requires at least one attempt");
        }
        Ok(Self {
            max_attempts,
            retry_after_ms,
        })
    }

    pub fn max_attempts(&self) -> u16 {
        self.max_attempts
    }

    pub fn retry_after_ms(&self) -> Option<u64> {
        self.retry_after_ms
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiRetryPolicyFields {
    max_attempts: u16,
    retry_after_ms: Option<u64>,
}

impl<'de> Deserialize<'de> for AiRetryPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiRetryPolicyFields::deserialize(deserializer)?;
        Self::new(fields.max_attempts, fields.retry_after_ms).map_err(serde::de::Error::custom)
    }
}
