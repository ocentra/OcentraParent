use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn io_result(result: std::io::Result<()>, action: &str) -> Result<(), String> {
    result.map_err(|error| format!("{action}: {error}"))
}

fn probe_dir() -> Result<PathBuf, String> {
    let root = std::env::temp_dir().join(format!(
        "ocentra-family-identity-core-ceremony-mint-probe-{}",
        std::process::id()
    ));
    io_result(
        fs::create_dir_all(root.join("src")),
        "create probe source directory",
    )?;
    Ok(root)
}

fn workspace_root() -> Result<PathBuf, String> {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| "locate workspace root from family-identity-core manifest".to_owned())
}

fn write_probe(root: &Path) -> Result<(), String> {
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = manifest_path.to_string_lossy().replace('\\', "/");
    let lockfile = workspace_root()?.join("Cargo.lock");
    io_result(
        fs::copy(lockfile, root.join("Cargo.lock")).map(|_bytes| ()),
        "copy workspace lockfile into external consumer probe",
    )?;
    io_result(
        fs::write(
            root.join("Cargo.toml"),
            format!(
                "[package]\nname = \"ocentra-family-identity-core-ceremony-mint-probe\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\nocentra-family-identity-core = {{ path = \"{manifest_path}\" }}\n"
            )
        ),
        "write probe manifest",
    )?;
    io_result(
        fs::write(
            root.join("src/main.rs"),
            r#"use ocentra_family_identity_core::family_identity::{
    ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
    HouseholdMembershipState, HouseholdRole, SessionFreshnessState,
};
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, HouseholdAuthorityInput,
};
use ocentra_family_identity_core::trust_bootstrap::{
    authorize_parent_device_trust_ceremony, ParentDeviceTrustAuthorityInput,
};

fn main() {
    let flags = ParentDeviceTrustAuthorityInput {
        household_authority: HouseholdAuthorityInput {
            actor_role: HouseholdRole::ParentOwner,
            same_family: true,
            actor_account_state: ActorAccountState::Active,
            membership_state: HouseholdMembershipState::Active,
            child_profile_binding_state: ChildProfileBindingState::Bound,
            device_ownership_scope: DeviceOwnershipScope::ParentControllerDevice,
            device_trust_state: DeviceTrustState::Pending,
            session_freshness_state: SessionFreshnessState::Fresh,
            capability_granted: true,
            controller_lease_state: None,
            action: HouseholdAuthorityAction::SealParentDeviceTrust,
        },
        family_id: "family".to_owned(),
        parent_account_id: "parent".to_owned(),
        device_ref: "device".to_owned(),
    };
    let _ = (flags, authorize_parent_device_trust_ceremony);
}
"#,
        ),
        "write probe source",
    )
}

#[test]
fn caller_supplied_authority_flags_cannot_mint_a_parent_device_trust_ceremony() -> Result<(), String>
{
    let root = probe_dir()?;
    write_probe(&root)?;
    let output = Command::new("cargo")
        .args(["check", "--quiet", "--offline"])
        .current_dir(&root)
        .output()
        .map_err(|error| format!("run external consumer probe: {error}"))?;
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unresolved import")
            && stderr.contains("ParentDeviceTrustAuthorityInput")
            && stderr.contains("authorize_parent_device_trust_ceremony"),
        "expected caller-supplied ceremony authority minting to be rejected, stderr: {stderr}"
    );
    Ok(())
}
