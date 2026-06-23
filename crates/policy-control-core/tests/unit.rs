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

#[path = "unit/policy_control.rs"]
mod policy_control;

#[path = "unit/policy_authority.rs"]
mod policy_authority;

#[path = "unit/policy_conflict.rs"]
mod policy_conflict;

#[path = "unit/policy_compiler.rs"]
mod policy_compiler;

#[path = "unit/policy_delivery.rs"]
mod policy_delivery;

#[path = "unit/policy_event.rs"]
mod policy_event;

#[path = "unit/policy_preview.rs"]
mod policy_preview;

#[path = "unit/policy_request.rs"]
mod policy_request;

#[path = "unit/policy_source.rs"]
mod policy_source;
