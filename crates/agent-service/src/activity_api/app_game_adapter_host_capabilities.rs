use std::{env, path::PathBuf};

use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;

pub(super) struct HostCapabilitySignals {
    pub(super) android_adb: bool,
    pub(super) android_adb_path: bool,
    pub(super) android_adb_sdk: bool,
    pub(super) linux_wsl: bool,
    pub(super) linux_docker: bool,
}

impl HostCapabilitySignals {
    pub(super) fn detect() -> Self {
        let android_adb_path = executable_available(proof::EXE_ADB);
        let android_adb_sdk = android_sdk_adb_available(proof::ENV_ANDROID_HOME)
            || android_sdk_adb_available(proof::ENV_ANDROID_SDK_ROOT);
        Self {
            android_adb: android_adb_path || android_adb_sdk,
            android_adb_path,
            android_adb_sdk,
            linux_wsl: executable_available(proof::EXE_WSL),
            linux_docker: executable_available(proof::EXE_DOCKER),
        }
    }

    pub(super) fn android_evidence_refs(&self) -> Vec<&'static str> {
        if self.android_adb {
            return vec![proof::REF_ANDROID_ADB_HOST_TOOLCHAIN];
        }
        Vec::new()
    }

    pub(super) fn android_probe_refs(&self) -> Vec<&'static str> {
        let mut refs = Vec::new();
        if self.android_adb_path {
            refs.push(proof::REF_ANDROID_ADB_PATH_PROBE);
        }
        if self.android_adb_sdk {
            refs.push(proof::REF_ANDROID_ADB_SDK_PROBE);
        }
        refs
    }

    pub(super) fn android_state(&self) -> &'static str {
        if self.android_adb {
            return ocentra_parent_agent_protocol::APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE;
        }
        ocentra_parent_agent_protocol::APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED
    }

    pub(super) fn linux_evidence_refs(&self) -> Vec<&'static str> {
        let mut refs = Vec::new();
        if self.linux_wsl {
            refs.push(proof::REF_LINUX_WSL_HOST_TOOLCHAIN);
        }
        if self.linux_docker {
            refs.push(proof::REF_LINUX_DOCKER_HOST_TOOLCHAIN);
        }
        refs
    }

    pub(super) fn linux_probe_refs(&self) -> Vec<&'static str> {
        let mut refs = Vec::new();
        if self.linux_wsl {
            refs.push(proof::REF_LINUX_WSL_PATH_PROBE);
        }
        if self.linux_docker {
            refs.push(proof::REF_LINUX_DOCKER_PATH_PROBE);
        }
        refs
    }

    pub(super) fn linux_state(&self) -> &'static str {
        if self.linux_wsl || self.linux_docker {
            return ocentra_parent_agent_protocol::APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE;
        }
        ocentra_parent_agent_protocol::APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED
    }
}

fn android_sdk_adb_available(env_name: &str) -> bool {
    match env::var_os(env_name) {
        Some(root) => {
            let platform_tools = PathBuf::from(root).join(proof::ANDROID_PLATFORM_TOOLS_DIR);
            executable_candidate_names(proof::EXE_ADB)
                .into_iter()
                .any(|candidate| platform_tools.join(candidate).is_file())
        }
        None => false,
    }
}

fn executable_available(executable: &str) -> bool {
    match env::var_os(proof::ENV_PATH) {
        Some(paths) => env::split_paths(&paths).any(|path| {
            executable_candidate_names(executable)
                .into_iter()
                .any(|candidate| path.join(candidate).is_file())
        }),
        None => false,
    }
}

fn executable_candidate_names(executable: &str) -> Vec<String> {
    let mut names = vec![executable.to_string()];
    if cfg!(windows) {
        names.extend(windows_executable_candidate_names(executable));
    }
    names
}

#[cfg(windows)]
fn windows_executable_candidate_names(executable: &str) -> Vec<String> {
    if executable.ends_with(proof::WINDOWS_EXE_EXTENSION) {
        return Vec::new();
    }
    let mut names = vec![format!("{executable}{}", proof::WINDOWS_EXE_EXTENSION)];
    if let Some(path_ext) = env::var_os(proof::ENV_PATHEXT) {
        names.extend(env::split_paths(&path_ext).filter_map(|extension| {
            extension
                .to_str()
                .map(|value| format!("{executable}{value}"))
        }));
    }
    names
}

#[cfg(not(windows))]
fn windows_executable_candidate_names(_executable: &str) -> Vec<String> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE;
    use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED;

    use super::*;

    #[test]
    fn android_probe_refs_distinguish_path_and_sdk_visibility() {
        let signals = HostCapabilitySignals {
            android_adb: true,
            android_adb_path: true,
            android_adb_sdk: true,
            linux_wsl: false,
            linux_docker: false,
        };

        assert_eq!(
            signals.android_state(),
            APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE
        );
        assert_eq!(
            signals.android_evidence_refs(),
            vec![proof::REF_ANDROID_ADB_HOST_TOOLCHAIN]
        );
        assert_eq!(
            signals.android_probe_refs(),
            vec![
                proof::REF_ANDROID_ADB_PATH_PROBE,
                proof::REF_ANDROID_ADB_SDK_PROBE,
            ]
        );
    }

    #[test]
    fn linux_probe_refs_keep_wsl_and_docker_separate() {
        let signals = HostCapabilitySignals {
            android_adb: false,
            android_adb_path: false,
            android_adb_sdk: false,
            linux_wsl: true,
            linux_docker: true,
        };

        assert_eq!(
            signals.linux_state(),
            APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE
        );
        assert_eq!(
            signals.linux_evidence_refs(),
            vec![
                proof::REF_LINUX_WSL_HOST_TOOLCHAIN,
                proof::REF_LINUX_DOCKER_HOST_TOOLCHAIN,
            ]
        );
        assert_eq!(
            signals.linux_probe_refs(),
            vec![
                proof::REF_LINUX_WSL_PATH_PROBE,
                proof::REF_LINUX_DOCKER_PATH_PROBE,
            ]
        );
    }

    #[test]
    fn missing_host_tools_report_not_detected_without_probe_refs() {
        let signals = HostCapabilitySignals {
            android_adb: false,
            android_adb_path: false,
            android_adb_sdk: false,
            linux_wsl: false,
            linux_docker: false,
        };

        assert_eq!(
            signals.android_state(),
            APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED
        );
        assert_eq!(signals.android_evidence_refs(), Vec::<&'static str>::new());
        assert_eq!(signals.android_probe_refs(), Vec::<&'static str>::new());
        assert_eq!(
            signals.linux_state(),
            APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED
        );
        assert_eq!(signals.linux_evidence_refs(), Vec::<&'static str>::new());
        assert_eq!(signals.linux_probe_refs(), Vec::<&'static str>::new());
    }
}
