use std::path::PathBuf;

pub(crate) struct LocalAiRuntimeConfigParts {
    pub(crate) runtime_binary: Option<PathBuf>,
    pub(crate) model: LocalAiRuntimeModelConfig,
    pub(crate) execution_enabled: bool,
    pub(crate) generation_timeout_ms: u64,
    pub(crate) generation_max_tokens: u32,
}

pub(crate) struct LocalAiRuntimeModelConfig {
    pub(crate) model_id: String,
    pub(crate) model_file: Option<PathBuf>,
    pub(crate) artifact_ref: Option<String>,
    pub(crate) manifest_ref: Option<String>,
    pub(crate) default_artifact_ref: &'static str,
    pub(crate) default_manifest_ref: &'static str,
}
