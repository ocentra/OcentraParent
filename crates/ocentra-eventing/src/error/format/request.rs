use std::fmt;

use super::super::EventingError;

pub(super) fn fmt_request_error(
    error: &EventingError,
    formatter: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    if let EventingError::InvalidRequestOptions { reason } = error {
        return write!(formatter, "invalid event request options: {reason}");
    }
    if let EventingError::DuplicateRequest { request_id } = error {
        return write!(formatter, "duplicate request id: {}", request_id.as_str());
    }
    if let EventingError::RequestTypeMismatch { request_id } = error {
        return write!(
            formatter,
            "event request type mismatch for request id: {}",
            request_id.as_str()
        );
    }
    if let EventingError::RequestTimedOut { request_id } = error {
        return write!(
            formatter,
            "event request timed out: {}",
            request_id.as_str()
        );
    }
    if let EventingError::RequestCancelled { request_id } = error {
        return write!(
            formatter,
            "event request cancelled: {}",
            request_id.as_str()
        );
    }
    if let EventingError::RequestResponseEncode { request_id, reason } = error {
        return write!(
            formatter,
            "event request response encode failed for {}: {reason}",
            request_id.as_str()
        );
    }
    if let EventingError::RequestResponseDecode { request_id, reason } = error {
        return write!(
            formatter,
            "event request response decode failed for {}: {reason}",
            request_id.as_str()
        );
    }
    if matches!(error, EventingError::BusShutdown) {
        return formatter.write_str("event bus is shut down");
    }
    debug_assert!(false, "request error formatter received non-request error");
    formatter.write_str("eventing request error")
}
