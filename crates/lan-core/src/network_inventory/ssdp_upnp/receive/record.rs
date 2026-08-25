use std::collections::HashSet;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use super::super::description::fetch_ssdp_description_until;
use super::super::{parse_ssdp_response, SsdpDiscoveryError, SsdpDiscoveryRecord};

pub(super) fn add_ssdp_record(
    response_bytes: &[u8],
    aggregate_deadline: Instant,
    seen: &mut HashSet<String>,
    results: &mut Vec<SsdpDiscoveryRecord>,
    cancellation: Option<&AtomicBool>,
) -> Result<(), SsdpDiscoveryError> {
    if is_cancelled(cancellation) || results.len() >= super::super::SSDP_MAX_RECORDS {
        return Ok(());
    }
    let Some(response) = parse_response(response_bytes)? else {
        return Ok(());
    };
    if !seen.insert(response.dedup_key()) {
        return Ok(());
    }
    let description = response
        .description_fetch_allowed()
        .then(|| {
            fetch_ssdp_description_until(&response.location, aggregate_deadline, cancellation).ok()
        })
        .flatten();
    if is_cancelled(cancellation) {
        return Ok(());
    }
    results.push(SsdpDiscoveryRecord {
        response,
        description,
    });
    Ok(())
}

fn parse_response(
    response_bytes: &[u8],
) -> Result<Option<super::super::SsdpDiscoveryResponse>, SsdpDiscoveryError> {
    match parse_ssdp_response(response_bytes) {
        Ok(response) => Ok(Some(response)),
        Err(SsdpDiscoveryError::MalformedResponse)
        | Err(SsdpDiscoveryError::MissingLocation)
        | Err(SsdpDiscoveryError::MissingSearchTarget)
        | Err(SsdpDiscoveryError::MissingUsn) => Ok(None),
        Err(error) => Err(error),
    }
}

fn is_cancelled(cancellation: Option<&AtomicBool>) -> bool {
    cancellation.is_some_and(|value| value.load(std::sync::atomic::Ordering::Acquire))
}
