use std::path::PathBuf;

use ocentra_parent_agent_protocol::constants;

use crate::local_ai_cache_root::local_ai_cache_root;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LocalAiKnownModel {
    pub(crate) model_id: &'static str,
    pub(crate) artifact_ref: &'static str,
    pub(crate) manifest_ref: &'static str,
    file_name: &'static str,
}

pub(crate) fn default_local_ai_model() -> LocalAiKnownModel {
    LocalAiKnownModel {
        model_id: constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4,
        artifact_ref: constants::local_ai_runtime::MODEL_REFERENCE_DEFAULT_GEMMA_4,
        manifest_ref: constants::local_ai_runtime::MODEL_MANIFEST_REFERENCE_DEFAULT_GEMMA_4,
        file_name: constants::local_ai_runtime::DEFAULT_GEMMA_4_MODEL_FILE_NAME,
    }
}

pub(crate) fn known_model_for_id(model_id: &str) -> Option<LocalAiKnownModel> {
    let default_model = default_local_ai_model();
    if model_id == default_model.model_id {
        Some(default_model)
    } else {
        None
    }
}

pub(crate) fn selected_cached_local_ai_model_path(model_id: &str) -> Option<PathBuf> {
    let model = known_model_for_id(model_id)?;
    let mut path = local_ai_cache_root()?;
    path.push(constants::local_ai_runtime::LOCAL_AI_MODELS_CACHE_DIR);
    path.push(model.file_name);
    Some(path)
}
