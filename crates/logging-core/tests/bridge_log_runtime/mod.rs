use ocentra_parent_logging_core::bridge_log_runtime::{
    bridge_entry_to_stored_log, bridge_log_runtime_typescript, bridge_payload_to_stored_log,
    create_bridge_entry_from_stored_log, stored_log_to_bridge_entry, stored_log_to_bridge_payload,
    BridgeEntry, BridgeEntryOverrides, BridgeLogPayload, BridgePayloadToStoredLogOptions,
    StoredTestLogLine, TestLogOrigin,
};

fn sample_payload() -> BridgeLogPayload {
    BridgeLogPayload {
        log_timestamp: 1_718_000_000_000,
        level: "info".to_string(),
        source: Some("portal".to_string()),
        context: Some("bridge".to_string()),
        message: "bridge write".to_string(),
        data: Some("{\"ok\":true}".to_string()),
        file: Some("bridge.ts".to_string()),
        file_path: Some("packages/logging-domain/src/transport/bridgeTransport.ts".to_string()),
        line: Some(10),
        column: Some(2),
        correlation_id: Some("cid-1".to_string()),
        tags: vec!["smoke".to_string()],
        stack: None,
        suite_type: Some("unit".to_string()),
        origin: Some(TestLogOrigin::Portal),
        environment: Some("test".to_string()),
    }
}

fn sample_stored_log() -> StoredTestLogLine {
    StoredTestLogLine {
        schema_version: 1,
        entry_type: "log".to_string(),
        scope: "parent-test".to_string(),
        run_id: "run-1".to_string(),
        run_type: "single".to_string(),
        suite_type: Some("unit".to_string()),
        test_name: "bridge test".to_string(),
        timestamp: 1_718_000_000_000,
        level: "info".to_string(),
        source: Some("portal".to_string()),
        context: Some("bridge".to_string()),
        message: "bridge write".to_string(),
        data: Some("{\"ok\":true}".to_string()),
        file: Some("bridge.ts".to_string()),
        file_path: Some("packages/logging-domain/src/transport/bridgeTransport.ts".to_string()),
        line: Some(10),
        column: Some(2),
        correlation_id: Some("cid-1".to_string()),
        tags: vec!["smoke".to_string()],
        stack: None,
        origin: Some(TestLogOrigin::Portal),
        environment: Some("test".to_string()),
    }
}

#[test]
fn bridge_runtime_maps_payload_to_stored_log_with_defaults() {
    let stored = bridge_payload_to_stored_log(
        &sample_payload(),
        &BridgePayloadToStoredLogOptions {
            test_name: "bridge test".to_string(),
            run_id: "run-1".to_string(),
            consumer: None,
            run_type: None,
        },
    );

    assert_eq!(stored.scope, "parent-test");
    assert_eq!(stored.run_type, "single");
    assert_eq!(stored.test_name, "bridge test");
    assert_eq!(stored.timestamp, 1_718_000_000_000);
    assert_eq!(
        stored.file_path.as_deref(),
        Some("packages/logging-domain/src/transport/bridgeTransport.ts")
    );
}

#[test]
fn bridge_runtime_maps_entries_and_round_trips_payloads() {
    let entry = BridgeEntry {
        test_name: "bridge test".to_string(),
        run_id: "run-1".to_string(),
        run_type: "single".to_string(),
        consumer: Some("parent-test".to_string()),
        log: sample_payload(),
    };

    let stored = bridge_entry_to_stored_log(&entry);
    assert_eq!(stored, sample_stored_log());
    assert_eq!(stored_log_to_bridge_payload(&stored), entry.log);
    assert_eq!(stored_log_to_bridge_entry(&stored), entry);
}

#[test]
fn bridge_runtime_applies_entry_overrides_without_mutating_payload() {
    let stored = sample_stored_log();
    let entry = create_bridge_entry_from_stored_log(
        &stored,
        &BridgeEntryOverrides {
            test_name: Some("override test".to_string()),
            run_id: Some("run-2".to_string()),
            run_type: Some("suite".to_string()),
            consumer: Some("worker-test".to_string()),
        },
    );

    assert_eq!(entry.test_name, "override test");
    assert_eq!(entry.run_id, "run-2");
    assert_eq!(entry.run_type, "suite");
    assert_eq!(entry.consumer.as_deref(), Some("worker-test"));
    assert_eq!(entry.log.message, "bridge write");
}

#[test]
fn generated_bridge_runtime_helper_stays_checked_in() {
    let checked_in = include_str!("../../../../packages/logging-domain/src/bridge-log-runtime.ts");

    assert_eq!(checked_in, bridge_log_runtime_typescript());
    assert_eq!(
        checked_in.lines().next(),
        Some("/* generated from crates/logging-core/src/bridge_log_runtime.rs */")
    );
}

#[test]
fn bridge_runtime_serializes_origin_as_contract_literal() {
    let payload = match serde_json::to_value(sample_payload()) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };
    let stored = match serde_json::to_value(sample_stored_log()) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };

    assert_eq!(
        payload.get("origin").and_then(serde_json::Value::as_str),
        Some("portal")
    );
    assert_eq!(
        stored.get("origin").and_then(serde_json::Value::as_str),
        Some("portal")
    );
}
