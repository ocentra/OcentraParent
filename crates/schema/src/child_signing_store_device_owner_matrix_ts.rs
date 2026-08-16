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

fn child_signing_store_device_owner_matrix_proof_typescript(proof_json: &str) -> String {
    let mut lines = Vec::new();

    for (index, line) in crate::typescript_literal::json_object_to_typescript_literal(proof_json)
        .lines()
        .enumerate()
    {
        let formatted_line = if index == 0 {
            line.to_owned()
        } else {
            format!("  {line}")
        };
        lines.extend(child_signing_store_device_owner_matrix_wrapped_line(
            &formatted_line,
        ));
    }

    lines.join("\n")
}

fn child_signing_store_device_owner_matrix_wrapped_line(line: &str) -> Vec<String> {
    if line.len() <= 120 {
        return vec![line.to_owned()];
    }

    let Some(separator_index) = line.find(": '") else {
        return vec![line.to_owned()];
    };
    let value = &line[separator_index + 2..];
    if !value.starts_with('\'') || !value.ends_with("',") {
        return vec![line.to_owned()];
    }

    let prefix = &line[..separator_index];
    let leading_indent = line.len() - line.trim_start().len();
    let indent = " ".repeat(leading_indent + 2);
    vec![format!("{prefix}:"), format!("{indent}{value}")]
}

pub fn child_signing_store_device_owner_matrix_contracts_typescript() -> String {
    let proof_json = crate::schema_result_or_unreachable(
        serde_json::to_string_pretty(&sample_child_signing_store_device_owner_matrix_proof()),
        "child signing/store/device-owner matrix proof json",
    );
    let proof_typescript = child_signing_store_device_owner_matrix_proof_typescript(&proof_json);

    CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_TEMPLATE
        .replace(
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_SCHEMA_VERSION_TOKEN,
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_SCHEMA_VERSION,
        )
        .replace(
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_PROOF_JSON_TOKEN,
            &proof_typescript,
        )
        .replace("{{", "{")
        .replace("}}", "}")
}
