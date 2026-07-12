use ocentra_parent_agent_protocol::constants;

use crate::{
    local_ai_runtime_acceleration_config::LocalAiRuntimeAccelerationConfig,
    local_ai_runtime_config_environment::runtime_config_from_environment,
    local_ai_runtime_config_parts::{LocalAiRuntimeConfigParts, LocalAiRuntimeModelConfig},
    local_ai_runtime_config_path::ConfiguredLocalPath,
    local_ai_runtime_config_values::validation::{is_safe_local_ai_model_id, safe_ref_or_default},
    local_ai_runtime_config_values::{
        LocalAiRuntimePath, LocalAiRuntimeRefPrefix, LocalAiRuntimeText,
    },
};

#[derive(Clone, Debug)]
pub struct LocalAiRuntimeConfigSnapshot {
    runtime_binary: ConfiguredLocalPath,
    model_id: String,
    model_file: ConfiguredLocalPath,
    artifact_ref: String,
    manifest_ref: Option<String>,
    execution_enabled: bool,
    generation_timeout_ms: u64,
    generation_max_tokens: u32,
    acceleration: LocalAiRuntimeAccelerationConfig,
}

impl LocalAiRuntimeConfigSnapshot {
    pub fn from_environment() -> Self {
        runtime_config_from_environment()
    }

    pub fn from_parts(
        runtime_binary: Option<LocalAiRuntimePath>,
        model_file: Option<LocalAiRuntimePath>,
        artifact_ref: Option<LocalAiRuntimeText>,
        manifest_ref: Option<LocalAiRuntimeText>,
    ) -> Self {
        Self::from_parts_with_execution(
            runtime_binary,
            model_file,
            artifact_ref,
            manifest_ref,
            false,
            constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
            constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        )
    }

    pub fn from_parts_with_execution(
        runtime_binary: Option<LocalAiRuntimePath>,
        model_file: Option<LocalAiRuntimePath>,
        artifact_ref: Option<LocalAiRuntimeText>,
        manifest_ref: Option<LocalAiRuntimeText>,
        execution_enabled: bool,
        generation_timeout_ms: u64,
        generation_max_tokens: u32,
    ) -> Self {
        Self::from_config_parts(LocalAiRuntimeConfigParts {
            runtime_binary: runtime_binary.map(|path| path.0),
            model: LocalAiRuntimeModelConfig {
                model_id: constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4.to_string(),
                model_file: model_file.map(|path| path.0),
                artifact_ref: artifact_ref.map(|value| value.0),
                manifest_ref: manifest_ref.map(|value| value.0),
                default_artifact_ref: constants::local_ai_runtime::MODEL_REFERENCE_DEFAULT_GEMMA_4,
                default_manifest_ref:
                    constants::local_ai_runtime::MODEL_MANIFEST_REFERENCE_DEFAULT_GEMMA_4,
            },
            execution_enabled,
            generation_timeout_ms,
            generation_max_tokens,
        })
    }

    pub(crate) fn from_config_parts(parts: LocalAiRuntimeConfigParts) -> Self {
        let model_id_candidate = LocalAiRuntimeText(parts.model.model_id.clone());
        let model_id = if is_safe_local_ai_model_id(&model_id_candidate) {
            parts.model.model_id
        } else {
            constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4.to_string()
        };
        Self {
            runtime_binary: ConfiguredLocalPath::from_path(parts.runtime_binary.as_deref()),
            model_id,
            model_file: ConfiguredLocalPath::from_path(parts.model.model_file.as_deref()),
            artifact_ref: safe_ref_or_default(
                parts.model.artifact_ref.map(LocalAiRuntimeText),
                LocalAiRuntimeRefPrefix(constants::local_ai_runtime::MODEL_ARTIFACT_REF_PREFIX),
                LocalAiRuntimeText(parts.model.default_artifact_ref.to_string()),
            )
            .0,
            manifest_ref: Some(
                safe_ref_or_default(
                    parts.model.manifest_ref.map(LocalAiRuntimeText),
                    LocalAiRuntimeRefPrefix(constants::local_ai_runtime::MODEL_MANIFEST_REF_PREFIX),
                    LocalAiRuntimeText(parts.model.default_manifest_ref.to_string()),
                )
                .0,
            ),
            execution_enabled: parts.execution_enabled,
            generation_timeout_ms: parts.generation_timeout_ms,
            generation_max_tokens: parts.generation_max_tokens,
            acceleration: LocalAiRuntimeAccelerationConfig::default(),
        }
    }

    pub(crate) fn with_acceleration_config(
        mut self,
        acceleration: LocalAiRuntimeAccelerationConfig,
    ) -> Self {
        self.acceleration = acceleration;
        self
    }

    pub fn unconfigured() -> Self {
        Self::from_parts(None, None, None, None)
    }

    pub fn runtime_binary(&self) -> &ConfiguredLocalPath {
        &self.runtime_binary
    }

    pub fn model_id(&self) -> LocalAiRuntimeText {
        LocalAiRuntimeText(self.model_id.clone())
    }

    pub fn model_file(&self) -> &ConfiguredLocalPath {
        &self.model_file
    }

    pub fn artifact_ref(&self) -> LocalAiRuntimeText {
        LocalAiRuntimeText(self.artifact_ref.clone())
    }

    pub fn manifest_ref(&self) -> Option<LocalAiRuntimeText> {
        self.manifest_ref.clone().map(LocalAiRuntimeText)
    }

    pub fn execution_enabled(&self) -> bool {
        self.execution_enabled
    }

    pub fn generation_timeout_ms(&self) -> u64 {
        self.generation_timeout_ms
    }

    pub fn generation_max_tokens(&self) -> u32 {
        self.generation_max_tokens
    }

    pub(crate) fn acceleration(&self) -> &LocalAiRuntimeAccelerationConfig {
        &self.acceleration
    }
}
