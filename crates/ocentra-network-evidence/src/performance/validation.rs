use super::{
    NetworkPerformanceBenchmarkError, NetworkPerformanceBenchmarkInput,
    NetworkPerformanceBenchmarkThresholds,
};

pub(super) fn validate_input(
    input: &NetworkPerformanceBenchmarkInput,
) -> Result<(), NetworkPerformanceBenchmarkError> {
    ensure_non_empty(
        &input.benchmark_run_ref,
        NetworkPerformanceBenchmarkError::EmptyBenchmarkRunRef,
    )?;
    ensure_non_empty(
        &input.fixture_set_ref,
        NetworkPerformanceBenchmarkError::EmptyFixtureSetRef,
    )?;
    ensure_non_empty(
        &input.event_history_ref,
        NetworkPerformanceBenchmarkError::EmptyEventHistoryRef,
    )?;
    ensure_non_empty(
        &input.resource_snapshot_ref,
        NetworkPerformanceBenchmarkError::EmptyResourceSnapshotRef,
    )?;
    validate_claims(input)?;
    validate_thresholds(&input.thresholds)?;
    if input.rows.is_empty() {
        return Err(NetworkPerformanceBenchmarkError::EmptyRows);
    }
    if input
        .rows
        .iter()
        .any(|row| row.scenario_ref.trim().is_empty())
    {
        return Err(NetworkPerformanceBenchmarkError::EmptyScenarioRef);
    }
    if input.rows.iter().any(|row| row.measurement_window_ms == 0) {
        return Err(NetworkPerformanceBenchmarkError::EmptyMeasurementWindow);
    }
    Ok(())
}

fn validate_claims(
    input: &NetworkPerformanceBenchmarkInput,
) -> Result<(), NetworkPerformanceBenchmarkError> {
    [
        (
            input.realtime_response_claimed,
            NetworkPerformanceBenchmarkError::RealtimeResponseClaimRejected,
        ),
        (
            input.production_slo_claimed,
            NetworkPerformanceBenchmarkError::ProductionSloClaimRejected,
        ),
        (
            input.raw_pcap_claimed,
            NetworkPerformanceBenchmarkError::RawPcapClaimRejected,
        ),
        (
            input.decrypted_payload_claimed,
            NetworkPerformanceBenchmarkError::DecryptedPayloadClaimRejected,
        ),
        (
            input.page_content_claimed,
            NetworkPerformanceBenchmarkError::PageContentClaimRejected,
        ),
        (
            input.exact_url_claimed,
            NetworkPerformanceBenchmarkError::ExactUrlClaimRejected,
        ),
        (
            input.adapter_action_claimed,
            NetworkPerformanceBenchmarkError::AdapterActionClaimRejected,
        ),
        (
            input.host_filtering_claimed,
            NetworkPerformanceBenchmarkError::HostFilteringClaimRejected,
        ),
        (
            input.enforcement_command_claimed,
            NetworkPerformanceBenchmarkError::EnforcementCommandClaimRejected,
        ),
    ]
    .into_iter()
    .find_map(|(claimed, error)| claimed.then_some(error))
    .map_or(Ok(()), Err)
}

fn validate_thresholds(
    thresholds: &NetworkPerformanceBenchmarkThresholds,
) -> Result<(), NetworkPerformanceBenchmarkError> {
    let valid = [
        thresholds.max_packet_to_detection_latency_ms > 0,
        thresholds.min_event_throughput_per_second > 0,
        thresholds.max_cpu_millis > 0,
        thresholds.max_memory_peak_kib > 0,
        thresholds.max_disk_written_bytes > 0,
        thresholds.min_high_concurrency_flow_count > 0,
    ]
    .into_iter()
    .all(|is_valid| is_valid);
    if valid {
        Ok(())
    } else {
        Err(NetworkPerformanceBenchmarkError::InvalidThresholds)
    }
}

fn ensure_non_empty(
    value: &str,
    error: NetworkPerformanceBenchmarkError,
) -> Result<(), NetworkPerformanceBenchmarkError> {
    if value.trim().is_empty() {
        Err(error)
    } else {
        Ok(())
    }
}
