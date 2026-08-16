use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::performance::*;

#[derive(Clone, Copy)]
struct ScenarioRef(&'static str);

#[test]
fn performance_benchmark_records_latency_throughput_resource_and_high_concurrency_metrics() {
    let proof = evaluate_network_performance_benchmark(benchmark_input(vec![
        metric_row(
            ScenarioRef("network-perf-safe"),
            NetworkPerformanceScenarioType::Safe,
            NetworkPerformancePathState::DryRun,
            120,
            500,
            100,
        ),
        metric_row(
            ScenarioRef("network-perf-high-concurrency"),
            NetworkPerformanceScenarioType::HighConcurrency,
            NetworkPerformancePathState::DryRun,
            2_400,
            5_000,
            1_000,
        ),
    ]))
    .expect_value("benchmark proof should aggregate passing fixture rows");

    assert_eq!(
        proof.benchmark_state,
        NetworkPerformanceBenchmarkState::MeetsBenchmarkGate
    );
    assert!(proof.regression_codes.is_empty());
    assert_eq!(proof.scenario_count, 2);
    assert_eq!(proof.flow_count, 2_520);
    assert_eq!(proof.event_count, 5_500);
    assert_eq!(proof.high_concurrency_flow_count, 2_400);
    assert_eq!(proof.event_throughput_per_second, 5_000);
    assert_eq!(proof.max_packet_to_detection_latency_ms, 700);
    assert_eq!(proof.max_queue_depth, 4);
    assert_eq!(proof.dropped_event_count, 0);
    assert_eq!(proof.path_states, vec![NetworkPerformancePathState::DryRun]);
    assert!(!proof.realtime_response_claimed);
    assert!(!proof.adapter_action_executed);
    assert_eq!(proof.enforcement_commands_published, 0);
}

#[test]
fn performance_benchmark_flags_latency_queue_resource_and_throughput_regressions() {
    let proof = evaluate_network_performance_benchmark(benchmark_input(vec![
        NetworkPerformanceBenchmarkRow {
            packet_to_detection_latency_ms: 1_400,
            event_count: 600,
            measurement_window_ms: 1_000,
            cpu_millis: 500,
            memory_peak_kib: 90_000,
            disk_written_bytes: 700_000,
            queue_depth: 40,
            dropped_event_count: 3,
            ..metric_row(
                ScenarioRef("network-perf-regression"),
                NetworkPerformanceScenarioType::HighConcurrency,
                NetworkPerformancePathState::Degraded,
                1_500,
                600,
                1_000,
            )
        },
    ]))
    .expect_value("benchmark proof should preserve regression metrics");

    assert_eq!(
        proof.benchmark_state,
        NetworkPerformanceBenchmarkState::BenchmarkGateExceeded
    );
    assert_eq!(
        proof.regression_codes,
        vec![
            NetworkPerformanceRegressionCode::PacketToDetectionLatencyExceeded,
            NetworkPerformanceRegressionCode::EventThroughputBelowMinimum,
            NetworkPerformanceRegressionCode::QueueDepthExceeded,
            NetworkPerformanceRegressionCode::DroppedEventsObserved,
            NetworkPerformanceRegressionCode::CpuBudgetExceeded,
            NetworkPerformanceRegressionCode::MemoryBudgetExceeded,
            NetworkPerformanceRegressionCode::DiskBudgetExceeded,
            NetworkPerformanceRegressionCode::HighConcurrencyFlowCountBelowMinimum,
        ]
    );
    assert_eq!(
        proof.path_states,
        vec![NetworkPerformancePathState::Degraded]
    );
}

#[test]
fn performance_benchmark_preserves_manual_required_and_unavailable_paths() {
    let proof = evaluate_network_performance_benchmark(benchmark_input(vec![
        metric_row(
            ScenarioRef("network-perf-manual"),
            NetworkPerformanceScenarioType::Suspicious,
            NetworkPerformancePathState::ManualRequired,
            400,
            1_500,
            500,
        ),
        metric_row(
            ScenarioRef("network-perf-unavailable"),
            NetworkPerformanceScenarioType::HighConcurrency,
            NetworkPerformancePathState::Unavailable,
            2_100,
            3_000,
            600,
        ),
    ]))
    .expect_value("manual and unavailable benchmark paths should remain measurable");

    assert_eq!(
        proof.path_states,
        vec![
            NetworkPerformancePathState::ManualRequired,
            NetworkPerformancePathState::Unavailable,
        ]
    );
    assert_eq!(proof.max_cascade_to_command_latency_ms, None);
    assert_eq!(proof.high_concurrency_flow_count, 2_100);
    assert_eq!(proof.enforcement_commands_published, 0);
    assert!(!proof.host_filtering_executed);
}

#[test]
fn performance_benchmark_rejects_realtime_content_adapter_and_production_claims() {
    assert_eq!(
        evaluate_network_performance_benchmark(NetworkPerformanceBenchmarkInput {
            realtime_response_claimed: true,
            ..benchmark_input(vec![passing_high_concurrency_row()])
        }),
        Err(NetworkPerformanceBenchmarkError::RealtimeResponseClaimRejected)
    );
    assert_eq!(
        evaluate_network_performance_benchmark(NetworkPerformanceBenchmarkInput {
            exact_url_claimed: true,
            ..benchmark_input(vec![passing_high_concurrency_row()])
        }),
        Err(NetworkPerformanceBenchmarkError::ExactUrlClaimRejected)
    );
    assert_eq!(
        evaluate_network_performance_benchmark(NetworkPerformanceBenchmarkInput {
            adapter_action_claimed: true,
            ..benchmark_input(vec![passing_high_concurrency_row()])
        }),
        Err(NetworkPerformanceBenchmarkError::AdapterActionClaimRejected)
    );
    assert_eq!(
        evaluate_network_performance_benchmark(NetworkPerformanceBenchmarkInput {
            production_slo_claimed: true,
            ..benchmark_input(vec![passing_high_concurrency_row()])
        }),
        Err(NetworkPerformanceBenchmarkError::ProductionSloClaimRejected)
    );
}

fn benchmark_input(rows: Vec<NetworkPerformanceBenchmarkRow>) -> NetworkPerformanceBenchmarkInput {
    NetworkPerformanceBenchmarkInput {
        benchmark_run_ref: "network-performance-row49".to_owned(),
        fixture_set_ref: "network-performance-fixtures-row49".to_owned(),
        event_history_ref: "network-performance-event-history-row49".to_owned(),
        resource_snapshot_ref: "network-performance-resource-snapshot-row49".to_owned(),
        thresholds: NetworkPerformanceBenchmarkThresholds {
            max_packet_to_detection_latency_ms: 800,
            min_event_throughput_per_second: 3_000,
            max_queue_depth: 16,
            max_dropped_event_count: 0,
            max_cpu_millis: 250,
            max_memory_peak_kib: 65_536,
            max_disk_written_bytes: 500_000,
            min_high_concurrency_flow_count: 2_000,
        },
        rows,
        realtime_response_claimed: false,
        production_slo_claimed: false,
        raw_pcap_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
        adapter_action_claimed: false,
        host_filtering_claimed: false,
        enforcement_command_claimed: false,
    }
}

fn passing_high_concurrency_row() -> NetworkPerformanceBenchmarkRow {
    metric_row(
        ScenarioRef("network-perf-high-concurrency-passing"),
        NetworkPerformanceScenarioType::HighConcurrency,
        NetworkPerformancePathState::DryRun,
        2_100,
        3_200,
        800,
    )
}

fn metric_row(
    scenario_ref: ScenarioRef,
    scenario_type: NetworkPerformanceScenarioType,
    path_state: NetworkPerformancePathState,
    flow_count: u32,
    event_count: u32,
    measurement_window_ms: u32,
) -> NetworkPerformanceBenchmarkRow {
    NetworkPerformanceBenchmarkRow {
        scenario_ref: scenario_ref.0.to_owned(),
        scenario_type,
        path_state,
        fixture_count: 10,
        packet_count: flow_count * 3,
        flow_count,
        event_count,
        measurement_window_ms,
        packet_to_summary_latency_ms: 80,
        packet_to_detection_latency_ms: 700,
        detection_to_cascade_latency_ms: 90,
        cascade_to_command_latency_ms: None,
        cpu_millis: 120,
        memory_peak_kib: 40_000,
        disk_written_bytes: 20_000,
        queue_depth: 4,
        dropped_event_count: 0,
        precision_basis_points: Some(9_000),
        recall_basis_points: Some(8_500),
        false_positive_count: 0,
        false_negative_count: 0,
    }
}
