pub fn child_linux_service_package_proof_typescript() -> String {
    include_str!("child_linux_service_package_proof.template.txt").to_string()
}

pub fn child_macos_service_package_proof_typescript() -> String {
    include_str!("child_macos_service_package_proof.template.txt").to_string()
}
