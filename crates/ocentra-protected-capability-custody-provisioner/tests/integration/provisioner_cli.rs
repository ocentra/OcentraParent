const PROVISIONER_BINARY_ENV: &str =
    "CARGO_BIN_EXE_ocentra-protected-capability-custody-provisioner";

fn provisioner_binary() -> Option<std::path::PathBuf> {
    if let Some(binary) = std::env::var_os(PROVISIONER_BINARY_ENV) {
        return Some(binary.into());
    }

    let mut binary = std::env::current_exe().ok()?;
    binary.pop();
    binary.pop();
    binary.push("ocentra-protected-capability-custody-provisioner");
    #[cfg(windows)]
    binary.set_extension("exe");
    Some(binary)
}

fn provisioner_exit_code(arguments: &[&str]) -> Option<i32> {
    std::process::Command::new(provisioner_binary()?)
        .args(arguments)
        .output()
        .ok()?
        .status
        .code()
}

#[test]
fn rejects_unexpected_arguments_before_platform_probe() {
    assert_eq!(provisioner_exit_code(&["unexpected"]), Some(10));
}

#[cfg(not(windows))]
#[test]
fn reports_unsupported_platform_without_arguments() {
    assert_eq!(provisioner_exit_code(&[]), Some(2));
}
