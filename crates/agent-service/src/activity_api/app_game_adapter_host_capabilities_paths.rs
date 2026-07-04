use std::{env, path::PathBuf};

use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ExecutableName(pub(super) &'static str);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct EnvironmentName(pub(super) &'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ExecutableCandidateNames(pub(super) Vec<String>);

pub(super) fn android_sdk_adb_available(env_name: EnvironmentName) -> bool {
    match env::var_os(env_name.0) {
        Some(root) => {
            let platform_tools = PathBuf::from(root).join(proof::ANDROID_PLATFORM_TOOLS_DIR);
            executable_candidate_names(ExecutableName(proof::EXE_ADB))
                .0
                .into_iter()
                .any(|candidate| platform_tools.join(candidate).is_file())
        }
        None => false,
    }
}

pub(super) fn executable_available(executable: ExecutableName) -> bool {
    match env::var_os(proof::ENV_PATH) {
        Some(paths) => env::split_paths(&paths).any(|path| {
            executable_candidate_names(executable)
                .0
                .into_iter()
                .any(|candidate| path.join(candidate).is_file())
        }),
        None => false,
    }
}

fn executable_candidate_names(executable: ExecutableName) -> ExecutableCandidateNames {
    let mut names = vec![executable.0.to_string()];
    if cfg!(windows) {
        names.extend(windows_executable_candidate_names(executable).0);
    }
    ExecutableCandidateNames(names)
}

#[cfg(windows)]
fn windows_executable_candidate_names(executable: ExecutableName) -> ExecutableCandidateNames {
    if executable.0.ends_with(proof::WINDOWS_EXE_EXTENSION) {
        return ExecutableCandidateNames(Vec::new());
    }
    let mut executable_name = executable.0.to_string();
    executable_name.push_str(proof::WINDOWS_EXE_EXTENSION);
    let mut names = vec![executable_name];
    if let Some(path_ext) = env::var_os(proof::ENV_PATHEXT) {
        names.extend(env::split_paths(&path_ext).filter_map(|extension| {
            extension.to_str().map(|value| {
                let mut candidate = executable.0.to_string();
                candidate.push_str(value);
                candidate
            })
        }));
    }
    ExecutableCandidateNames(names)
}

#[cfg(not(windows))]
fn windows_executable_candidate_names(_executable: ExecutableName) -> ExecutableCandidateNames {
    ExecutableCandidateNames(Vec::new())
}
