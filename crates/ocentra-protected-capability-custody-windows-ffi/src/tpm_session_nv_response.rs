//! Strict final NV response classification and uncertainty mapping.

use super::super::codec_types::handles::FixedNvOperation;
use super::super::response;
use super::super::response::auth::ResponseAuthorization;
use super::CounterOutcome;
use crate::{Error, Result, TpmCounterIncrementOutcome, TpmCounterIncrementUncertainty};

pub(super) enum DecodedCounterResponse {
    Accepted(CounterOutcome, ResponseAuthorization),
    Rejected(Error),
    Unverifiable(Error),
}

pub(super) fn decode(operation: FixedNvOperation, response_bytes: &[u8]) -> DecodedCounterResponse {
    let decoded = match operation {
        FixedNvOperation::Read => response::sessions::decode_nv_read(response_bytes)
            .map(|(value, response)| (CounterOutcome::Read(value), response)),
        FixedNvOperation::Increment => {
            response::sessions::decode_success_with_session(response_bytes).map(|response| {
                (
                    CounterOutcome::Increment(TpmCounterIncrementOutcome::Committed),
                    response,
                )
            })
        }
    };
    match decoded {
        Ok((outcome, response)) => {
            DecodedCounterResponse::Accepted(outcome, response.authorization)
        }
        Err(Error::Tpm(code)) => DecodedCounterResponse::Rejected(Error::Tpm(code)),
        Err(error) => DecodedCounterResponse::Unverifiable(error),
    }
}

pub(super) fn unverifiable(
    operation: FixedNvOperation,
    reason: TpmCounterIncrementUncertainty,
    read_error: Error,
) -> Result<CounterOutcome> {
    match operation {
        FixedNvOperation::Read => Err(read_error),
        FixedNvOperation::Increment => Ok(CounterOutcome::Increment(
            TpmCounterIncrementOutcome::Uncertain(reason),
        )),
    }
}
