use std::fs;

use crate::args::{self, CommandLine};
use crate::crypto;
use crate::error::UpdaterError;
use crate::manifest;
use crate::update::{self, UpdateOutcome};

pub async fn run_cli() -> Result<(), UpdaterError> {
    match args::parse_args()? {
        CommandLine::Keygen => {
            let keys = crypto::generate_key_pair();
            println!("privateKeyBase64={}", keys.private_key_base64);
            println!("publicKeyBase64={}", keys.public_key_base64);
        }
        CommandLine::DerivePublicKey { private_key_base64 } => {
            println!("{}", crypto::derive_public_key(&private_key_base64)?);
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
            println!("manifest-signature-ok");
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
        Ok(UpdateOutcome::Current { version }) => println!("updater-current:{version}"),
        Ok(UpdateOutcome::WouldInstall { current, latest }) => {
            println!("updater-would-install:{current}->{latest}");
        }
        Ok(UpdateOutcome::InstallerStarted { current, latest }) => {
            println!("updater-installer-started:{current}->{latest}");
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
