use super::RuntimeClockError;

impl std::fmt::Display for RuntimeClockError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotRecovered => formatter.write_str("clock recovery is incomplete"),
            Self::InvalidDurableTimestamp(error) => {
                write!(formatter, "durable timestamp is invalid: {error}")
            }
            Self::LockPoisoned(error) => write!(formatter, "clock lock is poisoned: {error}"),
            Self::NonMonotonicTimestamp => formatter.write_str("timestamp is non-monotonic"),
            Self::ForwardSkew => formatter.write_str("timestamp exceeds the forward-skew bound"),
            Self::Overflow => formatter.write_str("timestamp arithmetic overflowed"),
        }
    }
}
