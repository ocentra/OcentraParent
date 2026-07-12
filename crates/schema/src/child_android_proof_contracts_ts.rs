pub fn child_android_device_proof_artifact_gate_typescript() -> String {
    assemble_template_fragments(&[
        include_str!("child_android_device_proof_artifact_gate.contracts.template.txt"),
        include_str!("child_android_device_proof_artifact_gate.expectations.template.txt"),
        include_str!("child_android_device_proof_artifact_gate.readiness.template.txt"),
        include_str!("child_android_device_proof_artifact_gate.artifacts.template.txt"),
        include_str!("child_android_device_proof_artifact_gate.exports.template.txt"),
    ])
}

pub fn child_android_lifecycle_proof_typescript() -> String {
    assemble_template_fragments(&[
        include_str!("child_android_lifecycle_proof.contracts.template.txt"),
        include_str!("child_android_lifecycle_proof.readiness.template.txt"),
        include_str!("child_android_lifecycle_proof.exports.template.txt"),
    ])
}

pub fn child_android_permission_capability_proof_typescript() -> String {
    include_str!("child_android_permission_capability_proof.template.txt").to_string()
}

pub fn child_android_privileged_capability_proof_typescript() -> String {
    include_str!("child_android_privileged_capability_proof.template.txt").to_string()
}

pub fn child_android_service_protocol_proof_typescript() -> String {
    include_str!("child_android_service_protocol_proof.template.txt").to_string()
}

pub fn child_android_storage_protocol_proof_typescript() -> String {
    include_str!("child_android_storage_protocol_proof.template.txt").to_string()
}

fn assemble_template_fragments(fragments: &[&str]) -> String {
    fragments
        .iter()
        .map(|fragment| fragment.strip_suffix('\n').unwrap_or(fragment))
        .collect::<Vec<_>>()
        .join("\n\n")
        + "\n"
}
