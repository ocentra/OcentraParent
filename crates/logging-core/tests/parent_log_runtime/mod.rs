use ocentra_parent_logging_core::{
    duckdb_log_query::{
        decode_tags, default_db_file_name, duckdb_log_query_typescript, encode_tags,
        search_like_query,
    },
    parent_log_runtime::{
        has_stale_run_info_conflict, is_level_at_or_above, matches_debug_selection,
        normalize_bridge_endpoint, normalize_debug_path, parent_log_runtime_typescript,
        parse_boolean, parse_bridge_mode, parse_level, parse_list, resolve_bridge_route,
        should_log_to_console, should_store_log, stale_run_info_warning,
        ParentLogRuntimeDecisionInput,
    },
};

#[test]
fn parent_log_runtime_rules_match_expected_defaults_and_debug_selection() {
    assert!(parse_boolean(Some("yes"), false));
    assert!(!parse_boolean(Some("0"), true));
    assert_eq!(parse_level(Some("WARN"), "info"), "warn".to_string());
    assert_eq!(parse_level(Some("bogus"), "info"), "info".to_string());
    assert_eq!(parse_bridge_mode(Some("tunnel")), "tunnel");
    assert_eq!(parse_bridge_mode(Some("bogus")), "local");
    assert_eq!(
        parse_list(Some("portal, apps/portal/src/dev-logger.ts , run-42")),
        vec![
            "portal".to_string(),
            "apps/portal/src/dev-logger.ts".to_string(),
            "run-42".to_string()
        ]
    );
    assert_eq!(
        normalize_debug_path(r"apps\portal\src\dev-logger.ts"),
        "apps/portal/src/dev-logger.ts".to_string()
    );
    assert!(is_level_at_or_above("error", "info"));
    assert!(!is_level_at_or_above("debug", "warn"));

    let debug_sources = vec!["portal".to_string()];
    let debug_files = vec!["apps/portal/src/dev-logger.ts".to_string()];
    let debug_runs = vec!["run-42".to_string()];
    assert!(matches_debug_selection(
        &debug_sources,
        &debug_files,
        &debug_runs,
        Some("portal"),
        None,
        None,
        &[]
    ));
    assert!(matches_debug_selection(
        &debug_sources,
        &debug_files,
        &debug_runs,
        Some("worker"),
        Some("apps/portal/src/dev-logger.ts"),
        None,
        &[]
    ));
    assert!(matches_debug_selection(
        &debug_sources,
        &debug_files,
        &debug_runs,
        Some("worker"),
        None,
        Some("run-42"),
        &[]
    ));
    assert!(!matches_debug_selection(
        &debug_sources,
        &debug_files,
        &debug_runs,
        Some("worker"),
        None,
        None,
        &[]
    ));
}

#[test]
fn parent_log_runtime_rules_keep_console_store_and_bridge_state_explicit() {
    assert!(should_store_log(&ParentLogRuntimeDecisionInput {
        enabled: false,
        sink_enabled: false,
        node_env: Some("production"),
        test_mode: false,
        vitest_mode: false,
        level: "warn",
        min_level: "error",
        debug_selected: false,
    }));
    assert!(should_log_to_console(&ParentLogRuntimeDecisionInput {
        enabled: false,
        sink_enabled: true,
        node_env: Some("production"),
        test_mode: false,
        vitest_mode: false,
        level: "error",
        min_level: "error",
        debug_selected: false,
    }));
    assert!(!should_log_to_console(&ParentLogRuntimeDecisionInput {
        enabled: true,
        sink_enabled: false,
        node_env: Some("test"),
        test_mode: true,
        vitest_mode: false,
        level: "info",
        min_level: "info",
        debug_selected: false,
    }));
    assert!(should_store_log(&ParentLogRuntimeDecisionInput {
        enabled: true,
        sink_enabled: true,
        node_env: Some("development"),
        test_mode: false,
        vitest_mode: false,
        level: "info",
        min_level: "info",
        debug_selected: false,
    }));
    assert!(!should_store_log(&ParentLogRuntimeDecisionInput {
        enabled: true,
        sink_enabled: true,
        node_env: Some("production"),
        test_mode: false,
        vitest_mode: false,
        level: "info",
        min_level: "info",
        debug_selected: false,
    }));
    assert_eq!(
        normalize_bridge_endpoint("http://127.0.0.1:4479/"),
        "http://127.0.0.1:4479".to_string()
    );
    assert_eq!(resolve_bridge_route("POST", "/__logs__"), "logs");
    assert_eq!(resolve_bridge_route("GET", "/__logs__"), "not-found");
    assert_eq!(
        stale_run_info_warning(Some("run-1"), Some(0), 301_000),
        Some("previous run info was stale and has been replaced".to_string())
    );
    assert!(has_stale_run_info_conflict(
        Some("run-1"),
        Some("parent-test"),
        &[(Some("run-2"), "parent-test")]
    ));
    assert!(!has_stale_run_info_conflict(
        Some("run-1"),
        Some("parent-test"),
        &[
            (Some("run-1"), "parent-test"),
            (Some("run-2"), "parent-codex")
        ]
    ));
}

#[test]
fn parent_log_runtime_generated_helper_stays_checked_in() {
    let checked_in = include_str!("../../../../packages/logging-domain/src/parent-log-runtime.ts");
    assert_eq!(checked_in, parent_log_runtime_typescript());
    assert_eq!(
        checked_in.lines().next(),
        Some("/* generated from crates/logging-core/src/parent_log_runtime.rs */")
    );
}

#[test]
fn duckdb_query_rules_keep_sql_and_shape_helpers_checked_in() {
    assert_eq!(
        default_db_file_name("parent-test"),
        "parent-test-test-log.duckdb"
    );
    assert_eq!(
        encode_tags(&["failure".to_string(), "slow".to_string()]),
        Some("failure,slow".to_string())
    );
    assert_eq!(
        decode_tags(Some("failure,slow")),
        vec!["failure".to_string(), "slow".to_string()]
    );
    assert_eq!(search_like_query("portal"), "%portal%".to_string());

    let checked_in = include_str!("../../../../packages/logging-domain/src/duckdb-log-query.ts");
    assert_eq!(checked_in, duckdb_log_query_typescript());
    assert_eq!(
        checked_in.lines().next(),
        Some("/* generated from crates/logging-core/src/duckdb_log_query.rs */")
    );
}
