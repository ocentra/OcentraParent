use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkPerformanceScenarioType {
    Safe,
    Suspicious,
    Edge,
    HighConcurrency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkPerformancePathState {
    DryRun,
    ManualRequired,
    Unsupported,
    Unavailable,
    Degraded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkPerformanceBenchmarkState {
    MeetsBenchmarkGate,
    BenchmarkGateExceeded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkPerformanceRegressionCode {
    PacketToDetectionLatencyExceeded,
    EventThroughputBelowMinimum,
    QueueDepthExceeded,
    DroppedEventsObserved,
    CpuBudgetExceeded,
    MemoryBudgetExceeded,
    DiskBudgetExceeded,
    HighConcurrencyFlowCountBelowMinimum,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkPerformanceBenchmarkThresholds {
    pub max_packet_to_detection_latency_ms: u32,
    pub min_event_throughput_per_second: u32,
    pub max_queue_depth: u32,
    pub max_dropped_event_count: u32,
    pub max_cpu_millis: u32,
    pub max_memory_peak_kib: u32,
    pub max_disk_written_bytes: u64,
    pub min_high_concurrency_flow_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkPerformanceBenchmarkRow {
    pub scenario_ref: String,
    pub scenario_type: NetworkPerformanceScenarioType,
    pub path_state: NetworkPerformancePathState,
    pub fixture_count: u32,
    pub packet_count: u32,
    pub flow_count: u32,
    pub event_count: u32,
    pub measurement_window_ms: u32,
    pub packet_to_summary_latency_ms: u32,
    pub packet_to_detection_latency_ms: u32,
    pub detection_to_cascade_latency_ms: u32,
    pub cascade_to_command_latency_ms: Option<u32>,
    pub cpu_millis: u32,
    pub memory_peak_kib: u32,
    pub disk_written_bytes: u64,
    pub queue_depth: u32,
    pub dropped_event_count: u32,
    pub precision_basis_points: Option<u16>,
    pub recall_basis_points: Option<u16>,
    pub false_positive_count: u32,
    pub false_negative_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkPerformanceBenchmarkInput {
    pub benchmark_run_ref: String,
    pub fixture_set_ref: String,
    pub event_history_ref: String,
    pub resource_snapshot_ref: String,
    pub thresholds: NetworkPerformanceBenchmarkThresholds,
    pub rows: Vec<NetworkPerformanceBenchmarkRow>,
    pub realtime_response_claimed: bool,
    pub production_slo_claimed: bool,
    pub raw_pcap_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub exact_url_claimed: bool,
    pub adapter_action_claimed: bool,
    pub host_filtering_claimed: bool,
    pub enforcement_command_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkPerformanceBenchmarkProof {
    pub benchmark_run_ref: String,
    pub fixture_set_ref: String,
    pub event_history_ref: String,
    pub resource_snapshot_ref: String,
    pub benchmark_state: NetworkPerformanceBenchmarkState,
    pub regression_codes: Vec<NetworkPerformanceRegressionCode>,
    pub scenario_count: usize,
    pub fixture_count: u32,
    pub packet_count: u32,
    pub flow_count: u32,
    pub event_count: u32,
    pub max_packet_to_summary_latency_ms: u32,
    pub max_packet_to_detection_latency_ms: u32,
    pub max_detection_to_cascade_latency_ms: u32,
    pub max_cascade_to_command_latency_ms: Option<u32>,
    pub event_throughput_per_second: u32,
    pub max_cpu_millis: u32,
    pub max_memory_peak_kib: u32,
    pub total_disk_written_bytes: u64,
    pub max_queue_depth: u32,
    pub dropped_event_count: u32,
    pub high_concurrency_flow_count: u32,
    pub path_states: Vec<NetworkPerformancePathState>,
    pub false_positive_count: u32,
    pub false_negative_count: u32,
    pub realtime_response_claimed: bool,
    pub production_slo_claimed: bool,
    pub adapter_action_executed: bool,
    pub host_filtering_executed: bool,
    pub enforcement_commands_published: usize,
    pub raw_pcap_available: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkPerformanceBenchmarkError {
    EmptyBenchmarkRunRef,
    EmptyFixtureSetRef,
    EmptyEventHistoryRef,
    EmptyResourceSnapshotRef,
    InvalidThresholds,
    EmptyRows,
    EmptyScenarioRef,
    EmptyMeasurementWindow,
    RealtimeResponseClaimRejected,
    ProductionSloClaimRejected,
    RawPcapClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    ExactUrlClaimRejected,
    AdapterActionClaimRejected,
    HostFilteringClaimRejected,
    EnforcementCommandClaimRejected,
}

pub fn evaluate_network_performance_benchmark(
    input: NetworkPerformanceBenchmarkInput,
) -> Result<NetworkPerformanceBenchmarkProof, NetworkPerformanceBenchmarkError> {
    validate_input(&input)?;

    let aggregate = aggregate_rows(&input.rows);
    let regression_codes = regression_codes(&aggregate, &input.thresholds);
    let benchmark_state = if regression_codes.is_empty() {
        NetworkPerformanceBenchmarkState::MeetsBenchmarkGate
    } else {
        NetworkPerformanceBenchmarkState::BenchmarkGateExceeded
    };

    Ok(NetworkPerformanceBenchmarkProof {
        benchmark_run_ref: input.benchmark_run_ref,
        fixture_set_ref: input.fixture_set_ref,
        event_history_ref: input.event_history_ref,
        resource_snapshot_ref: input.resource_snapshot_ref,
        benchmark_state,
        regression_codes,
        scenario_count: input.rows.len(),
        fixture_count: aggregate.fixture_count,
        packet_count: aggregate.packet_count,
        flow_count: aggregate.flow_count,
        event_count: aggregate.event_count,
        max_packet_to_summary_latency_ms: aggregate.max_packet_to_summary_latency_ms,
        max_packet_to_detection_latency_ms: aggregate.max_packet_to_detection_latency_ms,
        max_detection_to_cascade_latency_ms: aggregate.max_detection_to_cascade_latency_ms,
        max_cascade_to_command_latency_ms: aggregate.max_cascade_to_command_latency_ms,
        event_throughput_per_second: aggregate.event_throughput_per_second,
        max_cpu_millis: aggregate.max_cpu_millis,
        max_memory_peak_kib: aggregate.max_memory_peak_kib,
        total_disk_written_bytes: aggregate.total_disk_written_bytes,
        max_queue_depth: aggregate.max_queue_depth,
        dropped_event_count: aggregate.dropped_event_count,
        high_concurrency_flow_count: aggregate.high_concurrency_flow_count,
        path_states: aggregate.path_states,
        false_positive_count: aggregate.false_positive_count,
        false_negative_count: aggregate.false_negative_count,
        realtime_response_claimed: false,
        production_slo_claimed: false,
        adapter_action_executed: false,
        host_filtering_executed: false,
        enforcement_commands_published: 0,
        raw_pcap_available: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NetworkPerformanceAggregate {
    fixture_count: u32,
    packet_count: u32,
    flow_count: u32,
    event_count: u32,
    max_packet_to_summary_latency_ms: u32,
    max_packet_to_detection_latency_ms: u32,
    max_detection_to_cascade_latency_ms: u32,
    max_cascade_to_command_latency_ms: Option<u32>,
    event_throughput_per_second: u32,
    max_cpu_millis: u32,
    max_memory_peak_kib: u32,
    total_disk_written_bytes: u64,
    max_queue_depth: u32,
    dropped_event_count: u32,
    high_concurrency_flow_count: u32,
    path_states: Vec<NetworkPerformancePathState>,
    false_positive_count: u32,
    false_negative_count: u32,
}

fn validate_input(
    input: &NetworkPerformanceBenchmarkInput,
) -> Result<(), NetworkPerformanceBenchmarkError> {
    if input.benchmark_run_ref.trim().is_empty() {
        return Err(NetworkPerformanceBenchmarkError::EmptyBenchmarkRunRef);
    }
    if input.fixture_set_ref.trim().is_empty() {
        return Err(NetworkPerformanceBenchmarkError::EmptyFixtureSetRef);
    }
    if input.event_history_ref.trim().is_empty() {
        return Err(NetworkPerformanceBenchmarkError::EmptyEventHistoryRef);
    }
    if input.resource_snapshot_ref.trim().is_empty() {
        return Err(NetworkPerformanceBenchmarkError::EmptyResourceSnapshotRef);
    }
    validate_claims(input)?;
    validate_thresholds(&input.thresholds)?;
    if input.rows.is_empty() {
        return Err(NetworkPerformanceBenchmarkError::EmptyRows);
    }
    for row in &input.rows {
        if row.scenario_ref.trim().is_empty() {
            return Err(NetworkPerformanceBenchmarkError::EmptyScenarioRef);
        }
        if row.measurement_window_ms == 0 {
            return Err(NetworkPerformanceBenchmarkError::EmptyMeasurementWindow);
        }
    }
    Ok(())
}

fn validate_claims(
    input: &NetworkPerformanceBenchmarkInput,
) -> Result<(), NetworkPerformanceBenchmarkError> {
    if input.realtime_response_claimed {
        return Err(NetworkPerformanceBenchmarkError::RealtimeResponseClaimRejected);
    }
    if input.production_slo_claimed {
        return Err(NetworkPerformanceBenchmarkError::ProductionSloClaimRejected);
    }
    if input.raw_pcap_claimed {
        return Err(NetworkPerformanceBenchmarkError::RawPcapClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkPerformanceBenchmarkError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkPerformanceBenchmarkError::PageContentClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkPerformanceBenchmarkError::ExactUrlClaimRejected);
    }
    if input.adapter_action_claimed {
        return Err(NetworkPerformanceBenchmarkError::AdapterActionClaimRejected);
    }
    if input.host_filtering_claimed {
        return Err(NetworkPerformanceBenchmarkError::HostFilteringClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkPerformanceBenchmarkError::EnforcementCommandClaimRejected);
    }
    Ok(())
}

fn validate_thresholds(
    thresholds: &NetworkPerformanceBenchmarkThresholds,
) -> Result<(), NetworkPerformanceBenchmarkError> {
    let valid = thresholds.max_packet_to_detection_latency_ms > 0
        && thresholds.min_event_throughput_per_second > 0
        && thresholds.max_cpu_millis > 0
        && thresholds.max_memory_peak_kib > 0
        && thresholds.max_disk_written_bytes > 0
        && thresholds.min_high_concurrency_flow_count > 0;
    if valid {
        Ok(())
    } else {
        Err(NetworkPerformanceBenchmarkError::InvalidThresholds)
    }
}

fn aggregate_rows(rows: &[NetworkPerformanceBenchmarkRow]) -> NetworkPerformanceAggregate {
    let mut aggregate = NetworkPerformanceAggregate {
        fixture_count: 0,
        packet_count: 0,
        flow_count: 0,
        event_count: 0,
        max_packet_to_summary_latency_ms: 0,
        max_packet_to_detection_latency_ms: 0,
        max_detection_to_cascade_latency_ms: 0,
        max_cascade_to_command_latency_ms: None,
        event_throughput_per_second: 0,
        max_cpu_millis: 0,
        max_memory_peak_kib: 0,
        total_disk_written_bytes: 0,
        max_queue_depth: 0,
        dropped_event_count: 0,
        high_concurrency_flow_count: 0,
        path_states: Vec::new(),
        false_positive_count: 0,
        false_negative_count: 0,
    };

    let mut total_measurement_window_ms = 0_u64;
    for row in rows {
        aggregate.fixture_count += row.fixture_count;
        aggregate.packet_count += row.packet_count;
        aggregate.flow_count += row.flow_count;
        aggregate.event_count += row.event_count;
        aggregate.max_packet_to_summary_latency_ms = aggregate
            .max_packet_to_summary_latency_ms
            .max(row.packet_to_summary_latency_ms);
        aggregate.max_packet_to_detection_latency_ms = aggregate
            .max_packet_to_detection_latency_ms
            .max(row.packet_to_detection_latency_ms);
        aggregate.max_detection_to_cascade_latency_ms = aggregate
            .max_detection_to_cascade_latency_ms
            .max(row.detection_to_cascade_latency_ms);
        aggregate.max_cpu_millis = aggregate.max_cpu_millis.max(row.cpu_millis);
        aggregate.max_memory_peak_kib = aggregate.max_memory_peak_kib.max(row.memory_peak_kib);
        aggregate.total_disk_written_bytes += row.disk_written_bytes;
        aggregate.max_queue_depth = aggregate.max_queue_depth.max(row.queue_depth);
        aggregate.dropped_event_count += row.dropped_event_count;
        aggregate.false_positive_count += row.false_positive_count;
        aggregate.false_negative_count += row.false_negative_count;
        total_measurement_window_ms += u64::from(row.measurement_window_ms);
        push_path_state(&mut aggregate.path_states, row.path_state);

        if let Some(latency_ms) = row.cascade_to_command_latency_ms {
            aggregate.max_cascade_to_command_latency_ms = Some(
                aggregate
                    .max_cascade_to_command_latency_ms
                    .map_or(latency_ms, |existing| existing.max(latency_ms)),
            );
        }
        if row.scenario_type == NetworkPerformanceScenarioType::HighConcurrency {
            aggregate.high_concurrency_flow_count += row.flow_count;
        }
    }

    aggregate.event_throughput_per_second = if total_measurement_window_ms == 0 {
        0
    } else {
        ((u64::from(aggregate.event_count) * 1_000) / total_measurement_window_ms) as u32
    };
    aggregate
}

fn regression_codes(
    aggregate: &NetworkPerformanceAggregate,
    thresholds: &NetworkPerformanceBenchmarkThresholds,
) -> Vec<NetworkPerformanceRegressionCode> {
    let mut codes = Vec::new();
    if aggregate.max_packet_to_detection_latency_ms > thresholds.max_packet_to_detection_latency_ms
    {
        codes.push(NetworkPerformanceRegressionCode::PacketToDetectionLatencyExceeded);
    }
    if aggregate.event_throughput_per_second < thresholds.min_event_throughput_per_second {
        codes.push(NetworkPerformanceRegressionCode::EventThroughputBelowMinimum);
    }
    if aggregate.max_queue_depth > thresholds.max_queue_depth {
        codes.push(NetworkPerformanceRegressionCode::QueueDepthExceeded);
    }
    if aggregate.dropped_event_count > thresholds.max_dropped_event_count {
        codes.push(NetworkPerformanceRegressionCode::DroppedEventsObserved);
    }
    if aggregate.max_cpu_millis > thresholds.max_cpu_millis {
        codes.push(NetworkPerformanceRegressionCode::CpuBudgetExceeded);
    }
    if aggregate.max_memory_peak_kib > thresholds.max_memory_peak_kib {
        codes.push(NetworkPerformanceRegressionCode::MemoryBudgetExceeded);
    }
    if aggregate.total_disk_written_bytes > thresholds.max_disk_written_bytes {
        codes.push(NetworkPerformanceRegressionCode::DiskBudgetExceeded);
    }
    if aggregate.high_concurrency_flow_count < thresholds.min_high_concurrency_flow_count {
        codes.push(NetworkPerformanceRegressionCode::HighConcurrencyFlowCountBelowMinimum);
    }
    codes
}

fn push_path_state(
    path_states: &mut Vec<NetworkPerformancePathState>,
    path_state: NetworkPerformancePathState,
) {
    if !path_states.contains(&path_state) {
        path_states.push(path_state);
    }
}
