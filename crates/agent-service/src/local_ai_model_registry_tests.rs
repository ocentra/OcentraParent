use std::path::Path;

use ocentra_parent_agent_protocol::constants;

use crate::local_ai_model_registry::{
    default_local_ai_model, known_model_for_id, selected_cached_local_ai_model_path,
};

#[test]
fn default_model_is_gemma_four_runtime_artifact() {
    let model = default_local_ai_model();

    assert_eq!(
        model.model_id,
        constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4
    );
    assert_eq!(
        model.artifact_ref,
        constants::local_ai_runtime::MODEL_REFERENCE_DEFAULT_GEMMA_4
    );
    assert_eq!(
        model.manifest_ref,
        constants::local_ai_runtime::MODEL_MANIFEST_REFERENCE_DEFAULT_GEMMA_4
    );
}

#[test]
fn default_model_cache_path_uses_local_model_cache_directory() {
    let path =
        selected_cached_local_ai_model_path(constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4)
            .expect(constants::error::LOCAL_AI_CACHE_ROOT_EXISTS);
    let suffix = Path::new(constants::local_ai_runtime::LOCAL_AI_MODELS_CACHE_DIR)
        .join(constants::local_ai_runtime::DEFAULT_GEMMA_4_MODEL_FILE_NAME);

    assert!(path.ends_with(suffix));
}

#[test]
fn unsupported_model_id_is_not_a_known_cache_artifact() {
    let known_model = known_model_for_id(constants::local_ai_runtime::TEST_UNSUPPORTED_MODEL_ID);
    let path =
        selected_cached_local_ai_model_path(constants::local_ai_runtime::TEST_UNSUPPORTED_MODEL_ID);

    assert_eq!(known_model, None);
    assert_eq!(path, None);
}
