use ocentra_parent_agent_protocol::constants;
use std::path::Path;

use crate::local_ai_runtime_config_values::{LocalAiRuntimePath, LocalAiRuntimeText};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LocalAiKnownModel {
    pub(crate) model_id: &'static str,
    pub(crate) artifact_ref: &'static str,
    pub(crate) manifest_ref: &'static str,
    file_name: &'static str,
}

impl LocalAiKnownModel {
    pub(crate) fn cache_path(&self, cache_root: impl AsRef<Path>) -> LocalAiRuntimePath {
        let mut path = cache_root.as_ref().to_path_buf();
        path.push(constants::local_ai_runtime::LOCAL_AI_MODELS_CACHE_DIR);
        path.push(self.file_name);
        LocalAiRuntimePath(path)
    }
}

pub(crate) fn default_local_ai_model() -> LocalAiKnownModel {
    LocalAiKnownModel {
        model_id: constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4,
        artifact_ref: constants::local_ai_runtime::MODEL_REFERENCE_DEFAULT_GEMMA_4,
        manifest_ref: constants::local_ai_runtime::MODEL_MANIFEST_REFERENCE_DEFAULT_GEMMA_4,
        file_name: constants::local_ai_runtime::DEFAULT_GEMMA_4_MODEL_FILE_NAME,
    }
}

pub(crate) fn known_model_for_id(model_id: &LocalAiRuntimeText) -> Option<LocalAiKnownModel> {
    let default_model = default_local_ai_model();
    if model_id.0 == default_model.model_id {
        Some(default_model)
    } else {
        None
    }
}
