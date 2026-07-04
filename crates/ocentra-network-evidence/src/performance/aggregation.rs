use super::{
    NetworkPerformanceAggregate, NetworkPerformanceBenchmarkRow,
    NetworkPerformanceBenchmarkThresholds, NetworkPerformancePathState,
    NetworkPerformanceRegressionCode, NetworkPerformanceScenarioType,
};

pub(super) fn aggregate_rows(
    rows: &[NetworkPerformanceBenchmarkRow],
) -> NetworkPerformanceAggregate {
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

pub(super) fn regression_codes(
    aggregate: &NetworkPerformanceAggregate,
    thresholds: &NetworkPerformanceBenchmarkThresholds,
) -> Vec<NetworkPerformanceRegressionCode> {
    [
        (
            aggregate.max_packet_to_detection_latency_ms
                > thresholds.max_packet_to_detection_latency_ms,
            NetworkPerformanceRegressionCode::PacketToDetectionLatencyExceeded,
        ),
        (
            aggregate.event_throughput_per_second < thresholds.min_event_throughput_per_second,
            NetworkPerformanceRegressionCode::EventThroughputBelowMinimum,
        ),
        (
            aggregate.max_queue_depth > thresholds.max_queue_depth,
            NetworkPerformanceRegressionCode::QueueDepthExceeded,
        ),
        (
            aggregate.dropped_event_count > thresholds.max_dropped_event_count,
            NetworkPerformanceRegressionCode::DroppedEventsObserved,
        ),
        (
            aggregate.max_cpu_millis > thresholds.max_cpu_millis,
            NetworkPerformanceRegressionCode::CpuBudgetExceeded,
        ),
        (
            aggregate.max_memory_peak_kib > thresholds.max_memory_peak_kib,
            NetworkPerformanceRegressionCode::MemoryBudgetExceeded,
        ),
        (
            aggregate.total_disk_written_bytes > thresholds.max_disk_written_bytes,
            NetworkPerformanceRegressionCode::DiskBudgetExceeded,
        ),
        (
            aggregate.high_concurrency_flow_count < thresholds.min_high_concurrency_flow_count,
            NetworkPerformanceRegressionCode::HighConcurrencyFlowCountBelowMinimum,
        ),
    ]
    .into_iter()
    .filter_map(|(exceeded, code)| exceeded.then_some(code))
    .collect()
}

fn push_path_state(
    path_states: &mut Vec<NetworkPerformancePathState>,
    path_state: NetworkPerformancePathState,
) {
    if !path_states.contains(&path_state) {
        path_states.push(path_state);
    }
}
