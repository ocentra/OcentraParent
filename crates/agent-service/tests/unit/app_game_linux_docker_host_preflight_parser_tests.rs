use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;

use super::app_game_linux_docker_host_preflight::{
    parse_context_count, parse_context_count_with_limit, parse_inventory_counts,
    probe_has_fixed_marker, MAX_DOCKER_INVENTORY_COUNT,
};
use super::app_game_linux_docker_host_preflight_process::DockerProbeOutput;
use super::app_game_linux_docker_host_preflight_state::{build_preflight, DockerPreflightState};

fn probe_output(success: bool, stdout: &[u8]) -> DockerProbeOutput {
    DockerProbeOutput {
        success,
        stdout: stdout.to_vec(),
    }
}

#[test]
fn fixed_marker_requires_success_exact_bytes_and_valid_utf8() {
    let exact_marker = format!("{}\n", proof::DOCKER_READY_MARKER);
    let extra_output = format!("{} extra\n", proof::DOCKER_READY_MARKER);

    assert_eq!(
        probe_has_fixed_marker(probe_output(true, exact_marker.as_bytes())),
        true
    );
    assert_eq!(
        probe_has_fixed_marker(probe_output(true, extra_output.as_bytes())),
        false
    );
    assert_eq!(
        probe_has_fixed_marker(probe_output(false, exact_marker.as_bytes())),
        false
    );
    assert_eq!(
        probe_has_fixed_marker(probe_output(true, &[0xff])),
        false
    );
}

#[test]
fn context_parser_covers_zero_malformed_success_failed_and_invalid_utf8() {
    let marker = proof::DOCKER_CONTEXT_COUNT_MARKER;
    let successful_output = format!("{}\n{}\n", marker, marker);

    assert_eq!(parse_context_count(probe_output(true, b"")), None);
    assert_eq!(
        parse_context_count(probe_output(true, b"unexpected-context-output\n")),
        None
    );
    assert_eq!(
        parse_context_count(probe_output(true, successful_output.as_bytes())),
        Some(2)
    );
    assert_eq!(
        parse_context_count(probe_output(false, successful_output.as_bytes())),
        None
    );
    assert_eq!(parse_context_count(probe_output(true, &[0xff])), None);
}

#[test]
fn context_parser_rejects_marker_streams_over_the_configured_bound() {
    let marker = proof::DOCKER_CONTEXT_COUNT_MARKER;
    let over_bound_output = format!("{}\n{}\n{}\n", marker, marker, marker);

    assert_eq!(
        parse_context_count_with_limit(
            probe_output(true, over_bound_output.as_bytes()),
            2,
        ),
        None
    );
}

#[test]
fn inventory_parser_requires_exactly_two_bounded_counts_from_successful_output() {
    assert_eq!(
        parse_inventory_counts(probe_output(true, b"3 4\n")),
        Some((3, 4))
    );
    assert_eq!(
        parse_inventory_counts(probe_output(true, b"3 4 5\n")),
        None
    );
    assert_eq!(
        parse_inventory_counts(probe_output(true, b"three 4\n")),
        None
    );
    assert_eq!(parse_inventory_counts(probe_output(true, b"3\n")), None);
    assert_eq!(
        parse_inventory_counts(probe_output(false, b"3 4\n")),
        None
    );
    assert_eq!(
        parse_inventory_counts(probe_output(
            true,
            format!("{} 0\n", MAX_DOCKER_INVENTORY_COUNT + 1).as_bytes(),
        )),
        None
    );
    assert_eq!(
        parse_inventory_counts(probe_output(
            true,
            format!("0 {}\n", MAX_DOCKER_INVENTORY_COUNT + 1).as_bytes(),
        )),
        None
    );
}

#[test]
fn built_preflight_keeps_counts_redacted_and_makes_no_claims() {
    let preflight = build_preflight(
        DockerPreflightState::READY,
        true,
        true,
        Some(2),
        Some((3, 1)),
    );

    assert_eq!(
        (
            preflight.context_inventory_visible,
            preflight.context_count,
            preflight.image_inventory_visible,
            preflight.image_count,
            preflight.container_inventory_visible,
            preflight.container_count,
        ),
        (true, 2, true, 3, true, 1)
    );
    assert_eq!(preflight.identifiers_redacted, true);
    assert_eq!(preflight.proof_refs, Vec::<String>::new());
    assert_eq!(preflight.open_gaps, Vec::<String>::new());
    assert_eq!(
        (
            preflight.adapter_dispatch_claimed,
            preflight.platform_enforcement_claimed,
            preflight.provider_delivery_claimed,
            preflight.child_device_delivery_claimed,
            preflight.private_diagnostics_claimed,
        ),
        (false, false, false, false, false)
    );
}
