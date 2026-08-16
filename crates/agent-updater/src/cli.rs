use std::fs;
use std::io::{stderr, stdout, Write};

use crate::args::{self, CommandLine};
use crate::crypto;
use crate::error::UpdaterError;
use crate::manifest;
use crate::update::{self, UpdateOutcome};

pub async fn run_cli() -> Result<(), UpdaterError> {
    match args::parse_args()? {
        CommandLine::Keygen => keygen()?,
        CommandLine::DerivePublicKey { private_key_base64 } => {
            derive_public_key(&private_key_base64)?
        }
        CommandLine::SignManifest {
            payload_path,
            output_path,
            private_key_base64,
        } => sign_manifest(payload_path, output_path, &private_key_base64)?,
        CommandLine::VerifyManifest {
            manifest_path,
            public_key_base64,
        } => verify_manifest(manifest_path, &public_key_base64)?,
        CommandLine::RunOnce {
            manifest_url,
            dry_run,
            current_version,
        } => print_outcome(update::run_once(&manifest_url, &current_version, dry_run).await)?,
        CommandLine::RunLoop {
            manifest_url,
            interval_seconds,
        } => update::run_loop(&manifest_url, interval_seconds).await?,
    }
    Ok(())
}

fn keygen() -> Result<(), UpdaterError> {
    let keys = crypto::generate_key_pair();
    let mut output = stdout().lock();
    writeln!(output, "privateKeyBase64={}", keys.private_key_base64)?;
    writeln!(output, "publicKeyBase64={}", keys.public_key_base64)?;
    Ok(())
}

fn derive_public_key(private_key_base64: &str) -> Result<(), UpdaterError> {
    let mut output = stdout().lock();
    writeln!(output, "{}", crypto::derive_public_key(private_key_base64)?)?;
    Ok(())
}

fn sign_manifest(
    payload_path: std::path::PathBuf,
    output_path: std::path::PathBuf,
    private_key_base64: &str,
) -> Result<(), UpdaterError> {
    let payload_text = fs::read_to_string(payload_path)?;
    let payload = manifest::parse_payload(&payload_text)?;
    let signed = manifest::sign_payload(payload, private_key_base64)?;
    fs::write(output_path, serde_json::to_string_pretty(&signed)?)?;
    Ok(())
}

fn verify_manifest(
    manifest_path: std::path::PathBuf,
    public_key_base64: &str,
) -> Result<(), UpdaterError> {
    let text = fs::read_to_string(manifest_path)?;
    let manifest = manifest::parse_signed_manifest(&text)?;
    manifest::verify_manifest(manifest, public_key_base64)?;
    let mut output = stdout().lock();
    writeln!(output, "manifest-signature-ok")?;
    Ok(())
}

fn print_outcome(result: Result<UpdateOutcome, UpdaterError>) -> Result<(), UpdaterError> {
    match result {
        Ok(UpdateOutcome::Current { version }) => {
            write_stdout_line(&format!("updater-current:{version}"))
        }
        Ok(UpdateOutcome::WouldInstall { current, latest }) => {
            write_stdout_line(&format!("updater-would-install:{current}->{latest}"))
        }
        Ok(UpdateOutcome::InstallerCompleted { current, latest }) => {
            write_stdout_line(&format!("updater-installer-completed:{current}->{latest}"))
        }
        Ok(UpdateOutcome::InstallerCompletedRebootRequired { current, latest }) => {
            write_stdout_line(&format!(
                "updater-installer-completed-reboot-required:{current}->{latest}"
            ))
        }
        Err(error) => {
            write_stderr_line(&error.to_string())?;
            std::process::exit(1);
        }
    }
}

fn write_stdout_line(message: &str) -> Result<(), UpdaterError> {
    let mut output = stdout().lock();
    writeln!(output, "{message}")?;
    Ok(())
}

fn write_stderr_line(message: &str) -> Result<(), UpdaterError> {
    let mut output = stderr().lock();
    writeln!(output, "{message}")?;
    Ok(())
}
