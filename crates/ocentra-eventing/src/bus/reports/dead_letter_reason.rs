use super::DeadLetterReason;

impl DeadLetterReason {
    pub(crate) fn idempotency_label(self) -> &'static str {
        match self {
            Self::HandlerFailed => "handler-failed",
            Self::HandlerTimedOut => "handler-timed-out",
            Self::HandlerDeadlineExpired => "handler-deadline-expired",
            Self::HandlerPanicked => "handler-panicked",
            Self::NoSubscriber => "no-subscriber",
            Self::QueueOverflow => "queue-overflow",
            Self::QueueExpired => "queue-expired",
            Self::DeadlineExpired => "deadline-expired",
            Self::Shutdown => "shutdown",
        }
    }
}
