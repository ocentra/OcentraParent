mod aggregation;
mod validation;

use serde::{Deserialize, Serialize};

use self::{
    aggregation::{aggregate_rows, regression_codes},
    validation::validate_input,
};

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
pub(super) struct NetworkPerformanceAggregate {
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
