use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn io_result(result: std::io::Result<()>, action: &str) -> Result<(), String> {
    result.map_err(|error| format!("{action}: {error}"))
}

fn probe_source_path() -> Result<PathBuf, String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    io_result(
        fs::create_dir_all(root.join("examples")),
        "create workspace-owned probe directory",
    )?;
    Ok(root.join(format!(
        "examples/ceremony_mint_boundary_probe_{}.rs",
        std::process::id()
    )))
}

fn write_probe(path: &Path) -> Result<(), String> {
    io_result(
        fs::write(
            path,
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
        "write workspace-owned probe source",
    )
}

#[test]
fn caller_supplied_authority_flags_cannot_mint_a_parent_device_trust_ceremony() -> Result<(), String>
{
    let source = probe_source_path()?;
    let example = source
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "derive workspace probe example name".to_owned())?;
    write_probe(&source)?;
    let output = Command::new("cargo")
        .args([
            "check",
            "--quiet",
            "--locked",
            "--offline",
            "--manifest-path",
            concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"),
            "--example",
            example,
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .map_err(|error| format!("run external consumer probe: {error}"))?;
    io_result(
        fs::remove_file(&source),
        "remove workspace-owned probe source",
    )?;
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
