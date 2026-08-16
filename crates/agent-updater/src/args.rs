use std::env;
use std::path::PathBuf;

use crate::constants::{DEFAULT_INTERVAL_SECONDS, DEFAULT_MANIFEST_URL, MANIFEST_URL_ENV};
use crate::error::UpdaterError;

#[derive(Debug, Clone)]
pub enum CommandLine {
    Keygen,
    DerivePublicKey {
        private_key_base64: String,
    },
    SignManifest {
        payload_path: PathBuf,
        output_path: PathBuf,
        private_key_base64: String,
    },
    VerifyManifest {
        manifest_path: PathBuf,
        public_key_base64: String,
    },
    RunOnce {
        manifest_url: String,
        dry_run: bool,
        current_version: String,
    },
    RunLoop {
        manifest_url: String,
        interval_seconds: u64,
    },
}

pub fn parse_args() -> Result<CommandLine, UpdaterError> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    parse_args_from(&args)
}

pub fn parse_args_from(args: &[String]) -> Result<CommandLine, UpdaterError> {
    let Some(command) = args.first() else {
        return Err(UpdaterError::Usage("expected updater command".to_owned()));
    };
    let rest = &args[1..];
    match command.as_str() {
        "keygen" => Ok(CommandLine::Keygen),
        "derive-public-key" => derive_public_key(rest),
        "sign-manifest" => sign_manifest(rest),
        "verify-manifest" => verify_manifest(rest),
        "run-once" => run_once(rest),
        "run-loop" => run_loop(rest),
        value => Err(UpdaterError::Usage(format!(
            "unknown updater command: {value}"
        ))),
    }
}

fn derive_public_key(args: &[String]) -> Result<CommandLine, UpdaterError> {
    Ok(CommandLine::DerivePublicKey {
        private_key_base64: required_option(args, "--private-key-base64")?,
    })
}

fn sign_manifest(args: &[String]) -> Result<CommandLine, UpdaterError> {
    Ok(CommandLine::SignManifest {
        payload_path: PathBuf::from(required_option(args, "--payload")?),
        output_path: PathBuf::from(required_option(args, "--out")?),
        private_key_base64: required_option(args, "--private-key-base64")?,
    })
}

fn verify_manifest(args: &[String]) -> Result<CommandLine, UpdaterError> {
    Ok(CommandLine::VerifyManifest {
        manifest_path: PathBuf::from(required_option(args, "--manifest")?),
        public_key_base64: required_option(args, "--public-key-base64")?,
    })
}

fn run_once(args: &[String]) -> Result<CommandLine, UpdaterError> {
    Ok(CommandLine::RunOnce {
        manifest_url: option_or_default(args, "--manifest-url", default_manifest_url()),
        dry_run: flag_present(args, "--dry-run"),
        current_version: option_or_default(
            args,
            "--current-version",
            env!("CARGO_PKG_VERSION").to_owned(),
        ),
    })
}

fn run_loop(args: &[String]) -> Result<CommandLine, UpdaterError> {
    let interval_seconds = option_or_default(
        args,
        "--interval-seconds",
        DEFAULT_INTERVAL_SECONDS.to_string(),
    )
    .parse()
    .map_err(|error| {
        UpdaterError::Usage(format!("--interval-seconds must be an integer: {error}"))
    })?;

    Ok(CommandLine::RunLoop {
        manifest_url: option_or_default(args, "--manifest-url", default_manifest_url()),
        interval_seconds,
    })
}

fn required_option(args: &[String], name: &str) -> Result<String, UpdaterError> {
    option_value(args, name)
        .ok_or_else(|| UpdaterError::Usage(format!("missing required option: {name}")))
}

fn option_or_default(args: &[String], name: &str, default: String) -> String {
    option_value(args, name).unwrap_or(default)
}

fn option_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|value| value == name)
        .and_then(|index| args.get(index + 1))
        .cloned()
}

fn flag_present(args: &[String], name: &str) -> bool {
    args.iter().any(|value| value == name)
}

fn default_manifest_url() -> String {
    env::var(MANIFEST_URL_ENV).unwrap_or_else(|_| DEFAULT_MANIFEST_URL.to_owned())
}
