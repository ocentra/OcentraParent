use std::io::{stderr, Write};
use std::time::Duration;
use std::{env, fs, path::PathBuf};

use rand_core::{OsRng, RngCore};
use semver::Version;
use tokio::time::sleep;

use crate::constants::{
    built_in_public_key_base64, INITIAL_DELAY_SECONDS_ENV, INTERVAL_SECONDS_ENV, PUBLIC_KEY_ENV,
};
use crate::error::UpdaterError;
use crate::hash::assert_sha256_file;
use crate::installer::{start_msi_upgrade, MsiUpgradeOutcome};
use crate::manifest::{parse_signed_manifest, verify_manifest};
use crate::network::{download_file, fetch_text};

pub enum UpdateOutcome {
    Current { version: String },
    WouldInstall { current: String, latest: String },
    InstallerCompleted { current: String, latest: String },
    InstallerCompletedRebootRequired { current: String, latest: String },
}

pub async fn run_once(
    manifest_url: &str,
    current_version: &str,
    dry_run: bool,
) -> Result<UpdateOutcome, UpdaterError> {
    let public_key = trusted_public_key()?;
    let manifest_text = fetch_text(manifest_url).await?;
    let manifest = parse_signed_manifest(&manifest_text)?;
    let payload = verify_manifest(manifest, &public_key)?;
    let current = Version::parse(current_version)?;
    let latest = Version::parse(&payload.version)?;

    if latest <= current {
        return Ok(UpdateOutcome::Current {
            version: current.to_string(),
        });
    }
    if dry_run {
        return Ok(UpdateOutcome::WouldInstall {
            current: current.to_string(),
            latest: latest.to_string(),
        });
    }

    let artifact = DownloadedArtifact::new(&payload.artifact.name)?;
    download_file(&payload.artifact.download_url, &artifact.path).await?;
    assert_sha256_file(&artifact.path, &payload.artifact.sha256)?;
    let installer_outcome = start_msi_upgrade(&artifact.path).await?;
    Ok(match installer_outcome {
        MsiUpgradeOutcome::Completed => UpdateOutcome::InstallerCompleted {
            current: current.to_string(),
            latest: latest.to_string(),
        },
        MsiUpgradeOutcome::CompletedRebootRequired => {
            UpdateOutcome::InstallerCompletedRebootRequired {
                current: current.to_string(),
                latest: latest.to_string(),
            }
        }
    })
}

pub async fn run_loop(manifest_url: &str, interval_seconds: u64) -> Result<(), UpdaterError> {
    sleep(Duration::from_secs(initial_delay_seconds())).await;
    loop {
        let result = run_once(manifest_url, env!("CARGO_PKG_VERSION"), false).await;
        if matches!(
            &result,
            Ok(UpdateOutcome::InstallerCompleted { .. })
                | Ok(UpdateOutcome::InstallerCompletedRebootRequired { .. })
        ) {
            return Ok(());
        }
        log_update_failure(&result)?;
        sleep(Duration::from_secs(effective_interval_seconds(
            interval_seconds,
        )))
        .await;
    }
}

fn log_update_failure(result: &Result<UpdateOutcome, UpdaterError>) -> Result<(), UpdaterError> {
    if let Err(error) = result {
        let mut output = stderr().lock();
        writeln!(output, "updater check failed: {error}")?;
    }
    Ok(())
}

fn trusted_public_key() -> Result<String, UpdaterError> {
    match env::var(PUBLIC_KEY_ENV) {
        Ok(value) if !value.trim().is_empty() => return Ok(value),
        _ => {}
    }
    let built_in = built_in_public_key_base64();
    if built_in.trim().is_empty() {
        return Err(UpdaterError::Policy(
            "updater has no trusted manifest public key".to_owned(),
        ));
    }
    Ok(built_in.to_owned())
}

struct DownloadedArtifact {
    root: PathBuf,
    path: PathBuf,
}

impl DownloadedArtifact {
    fn new(name: &str) -> Result<Self, UpdaterError> {
        let safe_name = name
            .rsplit(['/', '\\'])
            .next()
            .filter(|candidate| *candidate == name)
            .ok_or_else(|| {
                UpdaterError::Policy("artifact name is not a safe file name".to_owned())
            })?;
        let mut random = [0_u8; 16];
        for _ in 0..8 {
            OsRng.fill_bytes(&mut random);
            let suffix = random
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>();
            let root = env::temp_dir().join(format!("ocentra-child-agent-update-{suffix}"));
            match fs::create_dir(&root) {
                Ok(()) => {
                    return Ok(Self {
                        path: root.join(safe_name),
                        root,
                    })
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error.into()),
            }
        }
        Err(UpdaterError::Policy(
            "could not allocate a unique updater-owned temporary directory".to_owned(),
        ))
    }
}

impl Drop for DownloadedArtifact {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn initial_delay_seconds() -> u64 {
    env::var(INITIAL_DELAY_SECONDS_ENV)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(120)
}

fn effective_interval_seconds(interval_seconds: u64) -> u64 {
    env::var(INTERVAL_SECONDS_ENV)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(interval_seconds)
}
