use std::path::Path;

use ocentra_parent_agent_protocol::constants;

use crate::local_ai_model_registry::{default_local_ai_model, known_model_for_id};
use crate::local_ai_runtime_config_values::LocalAiRuntimeText;

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
    let model = default_local_ai_model();
    let root = Path::new(constants::local_ai_runtime::OCENTRA_PARENT_CACHE_DIR).to_path_buf();
    let path = model.cache_path(root);
    let suffix = Path::new(constants::local_ai_runtime::LOCAL_AI_MODELS_CACHE_DIR)
        .join(constants::local_ai_runtime::DEFAULT_GEMMA_4_MODEL_FILE_NAME);

    assert!(AsRef::<Path>::as_ref(&path).ends_with(suffix));
}

#[test]
fn default_model_cache_path_can_be_built_from_installer_cache_root() {
    let model = default_local_ai_model();
    let root = Path::new(constants::local_ai_runtime::OCENTRA_PARENT_CACHE_DIR).to_path_buf();
    let suffix = Path::new(constants::local_ai_runtime::LOCAL_AI_MODELS_CACHE_DIR)
        .join(constants::local_ai_runtime::DEFAULT_GEMMA_4_MODEL_FILE_NAME);

    let path = model.cache_path(root);

    assert!(AsRef::<Path>::as_ref(&path).ends_with(suffix));
}

#[test]
fn unsupported_model_id_is_not_a_known_cache_artifact() {
    let known_model = known_model_for_id(&LocalAiRuntimeText(
        constants::local_ai_runtime::TEST_UNSUPPORTED_MODEL_ID.to_string(),
    ));

    assert_eq!(known_model, None);
}
