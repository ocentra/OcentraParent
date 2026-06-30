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
        "derive-public-key" => Ok(CommandLine::DerivePublicKey {
            private_key_base64: option_value(rest, "--private-key-base64")?,
        }),
        "sign-manifest" => Ok(CommandLine::SignManifest {
            payload_path: PathBuf::from(option_value(rest, "--payload")?),
            output_path: PathBuf::from(option_value(rest, "--out")?),
            private_key_base64: option_value(rest, "--private-key-base64")?,
        }),
        "verify-manifest" => Ok(CommandLine::VerifyManifest {
            manifest_path: PathBuf::from(option_value(rest, "--manifest")?),
            public_key_base64: option_value(rest, "--public-key-base64")?,
        }),
        "run-once" => Ok(CommandLine::RunOnce {
            manifest_url: option_value_or(rest, "--manifest-url", default_manifest_url())?,
            dry_run: flag_present(rest, "--dry-run"),
            current_version: option_value_or(
                rest,
                "--current-version",
                env!("CARGO_PKG_VERSION").to_owned(),
            )?,
        }),
        "run-loop" => Ok(CommandLine::RunLoop {
            manifest_url: option_value_or(rest, "--manifest-url", default_manifest_url())?,
            interval_seconds: option_value_or(
                rest,
                "--interval-seconds",
                DEFAULT_INTERVAL_SECONDS.to_string(),
            )?
            .parse()
            .map_err(|error| {
                UpdaterError::Usage(format!("--interval-seconds must be an integer: {error}"))
            })?,
        }),
        value => Err(UpdaterError::Usage(format!(
            "unknown updater command: {value}"
        ))),
    }
}

fn option_value(args: &[String], name: &str) -> Result<String, UpdaterError> {
    option_value_or(args, name, String::new()).and_then(|value| {
        if value.is_empty() {
            Err(UpdaterError::Usage(format!(
                "missing required option: {name}"
            )))
        } else {
            Ok(value)
        }
    })
}

fn option_value_or(args: &[String], name: &str, default: String) -> Result<String, UpdaterError> {
    let Some(index) = args.iter().position(|value| value == name) else {
        return Ok(default);
    };
    args.get(index + 1)
        .cloned()
        .ok_or_else(|| UpdaterError::Usage(format!("missing value for option: {name}")))
}

fn flag_present(args: &[String], name: &str) -> bool {
    args.iter().any(|value| value == name)
}

fn default_manifest_url() -> String {
    env::var(MANIFEST_URL_ENV).unwrap_or_else(|_| DEFAULT_MANIFEST_URL.to_owned())
}
