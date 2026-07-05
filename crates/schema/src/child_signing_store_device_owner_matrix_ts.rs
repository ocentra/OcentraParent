use super::child_signing_store_device_owner_matrix::{
    sample_child_signing_store_device_owner_matrix_proof,
    CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_SCHEMA_VERSION,
};

const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_SCHEMA_VERSION_TOKEN: &str =
    "__CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_SCHEMA_VERSION__";
const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_PROOF_JSON_TOKEN: &str =
    "__CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_PROOF_JSON__";
const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_TEMPLATE: &str =
    include_str!("child_signing_store_device_owner_matrix.template.txt");

pub fn child_signing_store_device_owner_matrix_contracts_typescript() -> String {
    let proof_json = crate::schema_result_or_unreachable(
        serde_json::to_string_pretty(&sample_child_signing_store_device_owner_matrix_proof()),
        "child signing/store/device-owner matrix proof json",
    );

    CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_TEMPLATE
        .replace(
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_SCHEMA_VERSION_TOKEN,
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_SCHEMA_VERSION,
        )
        .replace(
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_PROOF_JSON_TOKEN,
            &proof_json,
        )
        .replace("{{", "{")
        .replace("}}", "}")
}
