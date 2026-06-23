type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

macro_rules! test_ok {
    ($expr:expr, $context:expr) => {{
        match $expr {
            Ok(value) => value,
            Err(error) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{}: {error}", $context),
                )
                .into())
            }
        }
    }};
}

macro_rules! test_some {
    ($expr:expr, $context:expr) => {{
        match $expr {
            Some(value) => value,
            None => return Err(std::io::Error::new(std::io::ErrorKind::Other, $context).into()),
        }
    }};
}

macro_rules! test_err {
    ($expr:expr, $context:expr) => {{
        match $expr {
            Ok(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, $context).into()),
            Err(error) => error,
        }
    }};
}

#[path = "version-skew/policy_source.rs"]
mod policy_source;

#[path = "version-skew/policy_source_migration.rs"]
mod policy_source_migration;

#[path = "version-skew/policy_compiler.rs"]
mod policy_compiler;

#[path = "version-skew/policy_delivery.rs"]
mod policy_delivery;

#[path = "version-skew/policy_event.rs"]
mod policy_event;

#[path = "version-skew/policy_preview.rs"]
mod policy_preview;

#[path = "version-skew/policy_request.rs"]
mod policy_request;
