use std::fs;
use std::io::{stderr, stdout, Write};

use crate::args::{self, CommandLine};
use crate::crypto;
use crate::error::UpdaterError;
use crate::manifest;
use crate::update::{self, UpdateOutcome};

pub async fn run_cli() -> Result<(), UpdaterError> {
    match args::parse_args()? {
        CommandLine::Keygen => {
            let keys = crypto::generate_key_pair();
            let mut output = stdout().lock();
            writeln!(output, "privateKeyBase64={}", keys.private_key_base64)?;
            writeln!(output, "publicKeyBase64={}", keys.public_key_base64)?;
        }
        CommandLine::DerivePublicKey { private_key_base64 } => {
            let mut output = stdout().lock();
            writeln!(
                output,
                "{}",
                crypto::derive_public_key(&private_key_base64)?
            )?;
        }
        CommandLine::SignManifest {
            payload_path,
            output_path,
            private_key_base64,
        } => {
            let payload_text = fs::read_to_string(payload_path)?;
            let payload = manifest::parse_payload(&payload_text)?;
            let signed = manifest::sign_payload(payload, &private_key_base64)?;
            fs::write(output_path, serde_json::to_string_pretty(&signed)?)?;
        }
        CommandLine::VerifyManifest {
            manifest_path,
            public_key_base64,
        } => {
            let text = fs::read_to_string(manifest_path)?;
            let manifest = manifest::parse_signed_manifest(&text)?;
            manifest::verify_manifest(manifest, &public_key_base64)?;
            let mut output = stdout().lock();
            writeln!(output, "manifest-signature-ok")?;
        }
        CommandLine::RunOnce {
            manifest_url,
            dry_run,
            current_version,
        } => {
            print_outcome(update::run_once(&manifest_url, &current_version, dry_run).await);
        }
        CommandLine::RunLoop {
            manifest_url,
            interval_seconds,
        } => update::run_loop(&manifest_url, interval_seconds).await?,
    }
    Ok(())
}

fn print_outcome(result: Result<UpdateOutcome, UpdaterError>) {
    match result {
        Ok(UpdateOutcome::Current { version }) => {
            let mut output = stdout().lock();
            if writeln!(output, "updater-current:{version}").is_err() {
                std::process::exit(1);
            }
        }
        Ok(UpdateOutcome::WouldInstall { current, latest }) => {
            let mut output = stdout().lock();
            if writeln!(output, "updater-would-install:{current}->{latest}").is_err() {
                std::process::exit(1);
            }
        }
        Ok(UpdateOutcome::InstallerStarted { current, latest }) => {
            let mut output = stdout().lock();
            if writeln!(output, "updater-installer-started:{current}->{latest}").is_err() {
                std::process::exit(1);
            }
        }
        Err(error) => {
            let mut output = stderr().lock();
            drop(writeln!(output, "{error}"));
            std::process::exit(1);
        }
    }
}
