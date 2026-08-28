use super::app_game_adapter_host_capabilities_paths::ExecutableName;
use super::app_game_linux_docker_host_preflight_path_security::trusted_docker_candidate;
use super::app_game_linux_docker_host_preflight_paths::TrustedDockerRoot;

#[cfg(target_os = "linux")]
#[test]
fn linux_rejects_parent_traversal_in_candidate_root() {
    assert_eq!(
        trusted_docker_candidate(
            TrustedDockerRoot("/tmp/../usr/bin"),
            ExecutableName("docker"),
        )
        .is_none(),
        true
    );
}

#[cfg(target_os = "linux")]
#[test]
fn linux_rejects_world_writable_probe_root() {
    assert_eq!(
        trusted_docker_candidate(TrustedDockerRoot("/tmp"), ExecutableName("docker"))
            .is_none(),
        true
    );
}

#[cfg(target_os = "linux")]
#[test]
fn linux_rejects_nonexistent_probe_root() {
    assert_eq!(
        trusted_docker_candidate(
            TrustedDockerRoot("/ocentra-parent/nonexistent-docker-root"),
            ExecutableName("docker"),
        )
        .is_none(),
        true
    );
}

#[cfg(windows)]
#[test]
fn windows_rejects_parent_traversal_from_protected_ancestor() {
    assert_eq!(
        trusted_docker_candidate(
            TrustedDockerRoot(r"C:\Program Files\..\Windows"),
            ExecutableName("docker.exe"),
        )
        .is_none(),
        true
    );
}

#[cfg(windows)]
#[test]
fn windows_rejects_unprotected_probe_root() {
    assert_eq!(
        trusted_docker_candidate(
            TrustedDockerRoot(r"C:\Users\Public"),
            ExecutableName("docker.exe"),
        )
        .is_none(),
        true
    );
}

#[cfg(all(unix, not(target_os = "linux")))]
#[test]
fn unsupported_unix_platform_rejects_docker_candidate() {
    assert_eq!(
        trusted_docker_candidate(TrustedDockerRoot("/usr/bin"), ExecutableName("docker"))
            .is_none(),
        true
    );
}
