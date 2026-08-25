//! Owned TPM policy-session and fixed prepared-operation modules.

#[path = "tpm_session_facade.rs"]
pub(crate) mod facade;
#[path = "tpm_session_lifetimes.rs"]
mod lifetimes;
#[path = "tpm_session_nv.rs"]
mod nv;
#[path = "tpm_session_nv_response.rs"]
mod nv_response;
#[path = "tpm_session_policy.rs"]
mod policy;
#[path = "tpm_session_prepared.rs"]
mod prepared;

pub(super) enum CounterOutcome {
    Read(u64),
    Increment(crate::TpmCounterIncrementOutcome),
}
