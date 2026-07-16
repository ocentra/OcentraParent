const DEFAULT_BRIDGE_MODE: &str = "local";
const DEFAULT_NODE_ENV: &str = "development";
const STALE_RUN_INFO_WARNING: &str = "previous run info was stale and has been replaced";

pub fn parse_boolean(value: Option<&str>, fallback: bool) -> bool {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .and_then(|value| {
            [
                ("true", true),
                ("1", true),
                ("yes", true),
                ("on", true),
                ("false", false),
                ("0", false),
                ("no", false),
                ("off", false),
            ]
            .into_iter()
            .find_map(|(needle, parsed)| value.eq_ignore_ascii_case(needle).then_some(parsed))
        })
        .unwrap_or(fallback)
}

pub fn parse_list(value: Option<&str>) -> Vec<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|entry| !entry.is_empty())
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

pub fn parse_level(value: Option<&str>, fallback: &str) -> String {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_lowercase)
        .filter(|value| ["trace", "debug", "info", "warn", "error"].contains(&value.as_str()))
        .unwrap_or_else(|| fallback.to_owned())
}

pub fn parse_bridge_mode(value: Option<&str>) -> &'static str {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .and_then(|value| {
            [("tunnel", "tunnel"), ("disabled", "disabled")]
                .into_iter()
                .find_map(|(needle, mode)| value.eq_ignore_ascii_case(needle).then_some(mode))
        })
        .unwrap_or(DEFAULT_BRIDGE_MODE)
}

pub fn normalize_debug_path(value: &str) -> String {
    value.replace('\\', "/").to_ascii_lowercase()
}

pub fn level_weight(level: &str) -> usize {
    ["trace", "debug", "info", "warn", "error"]
        .iter()
        .position(|candidate| candidate == &level)
        .unwrap_or(2)
}

pub fn is_level_at_or_above(level: &str, min_level: &str) -> bool {
    level_weight(level) >= level_weight(min_level)
}

pub fn is_dev_or_test_environment(
    node_env: Option<&str>,
    test_mode: bool,
    vitest_mode: bool,
) -> bool {
    let normalized_node_env = node_env
        .unwrap_or(DEFAULT_NODE_ENV)
        .trim()
        .to_ascii_lowercase();
    test_mode || vitest_mode || matches!(normalized_node_env.as_str(), "test" | "development")
}

fn normalize_source(source: Option<&str>) -> Option<String> {
    source
        .map(str::trim)
        .filter(|source| !source.is_empty())
        .map(str::to_ascii_lowercase)
}

pub fn matches_debug_selection(
    debug_sources: &[String],
    debug_files: &[String],
    debug_runs: &[String],
    source: Option<&str>,
    file_path: Option<&str>,
    run_id: Option<&str>,
    request_debug_sources: &[String],
) -> bool {
    let source_selected = normalize_source(source).is_some_and(|normalized_source| {
        debug_sources
            .iter()
            .any(|entry| entry == &normalized_source)
            || request_debug_sources
                .iter()
                .any(|entry| entry.trim().to_ascii_lowercase() == normalized_source)
    });
    let file_selected = file_path
        .map(str::trim)
        .filter(|file| !file.is_empty())
        .is_some_and(|file_path| {
            let normalized_file = normalize_debug_path(file_path);
            debug_files
                .iter()
                .any(|entry| normalized_file.contains(entry))
        });
    let run_selected = run_id
        .map(str::trim)
        .filter(|run_id| !run_id.is_empty())
        .is_some_and(|run_id| debug_runs.iter().any(|entry| entry == run_id));

    source_selected || file_selected || run_selected
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParentLogRuntimeDecisionInput<'a> {
    pub enabled: bool,
    pub sink_enabled: bool,
    pub node_env: Option<&'a str>,
    pub test_mode: bool,
    pub vitest_mode: bool,
    pub level: &'a str,
    pub min_level: &'a str,
    pub debug_selected: bool,
}

pub fn should_log_to_console(input: &ParentLogRuntimeDecisionInput<'_>) -> bool {
    input.sink_enabled
        && (input.level == "error"
            || input.level == "warn"
            || (input.enabled
                && (input.debug_selected
                    || is_dev_or_test_environment(
                        input.node_env,
                        input.test_mode,
                        input.vitest_mode,
                    ) && is_level_at_or_above(input.level, input.min_level))))
}

pub fn should_store_log(input: &ParentLogRuntimeDecisionInput<'_>) -> bool {
    input.level == "error"
        || input.level == "warn"
        || (input.enabled
            && input.sink_enabled
            && (input.debug_selected
                || is_dev_or_test_environment(input.node_env, input.test_mode, input.vitest_mode)
                    && is_level_at_or_above(input.level, input.min_level)))
}

pub fn normalize_bridge_endpoint(endpoint: &str) -> String {
    endpoint.trim_end_matches('/').to_owned()
}

pub fn resolve_bridge_route(method: &str, pathname: &str) -> &'static str {
    [
        ("GET", "/__health__", "health"),
        ("GET", "/__run_info__", "run-info"),
        ("POST", "/__run_started__", "run-started"),
        ("POST", "/__logs__", "logs"),
        ("GET", "/__flush__", "flush"),
        ("POST", "/__flush__", "flush"),
    ]
    .into_iter()
    .find_map(|(route_method, route_path, route)| {
        (route_method == method && route_path == pathname).then_some(route)
    })
    .unwrap_or("not-found")
}

pub fn stale_run_info_warning(
    run_id: Option<&str>,
    started_at: Option<u64>,
    now: u64,
) -> Option<String> {
    run_id.zip(started_at).and_then(|(_, started_at)| {
        (now.saturating_sub(started_at) > 5 * 60 * 1000).then(|| STALE_RUN_INFO_WARNING.to_owned())
    })
}

pub fn has_stale_run_info_conflict(
    active_run_id: Option<&str>,
    active_scope: Option<&str>,
    entries: &[(Option<&str>, &str)],
) -> bool {
    active_run_id
        .zip(active_scope)
        .is_some_and(|(active_run_id, active_scope)| {
            entries.iter().any(|(entry_run_id, entry_consumer)| {
                *entry_consumer == active_scope && entry_run_id.unwrap_or("") != active_run_id
            })
        })
}

pub fn parent_log_runtime_typescript() -> &'static str {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../logging-core-generated/parent_log_runtime.ts"
    ))
}
