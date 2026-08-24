//! Account issuer lifecycle and restart-safe delivery driver.
//!
//! The runtime owns no authority, key, or delivery cache.  Startup always
//! revalidates the protected durable store, and each delivery attempt enters
//! the existing Account transaction path that re-checks currentness, binding,
//! registry key, outer wire, acknowledgement, and reconciliation.

use std::path::Path;

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

use super::cloudflare_delivery::AccountIdentityIssuerCloudflareDelivery;
use super::key_custody::AccountIdentityIssuerSignerAdapter;
use super::service_binding::AccountIdentityIssuerService;
use super::startup::AccountIdentityIssuerStartupState;
use super::{AccountIdentityIssuer, AccountIdentityIssuerError};

const MAX_DELIVERY_ATTEMPTS: u8 = 3;

enum AccountIdentityIssuerRuntimeStep {
    Delivered,
    Empty,
    Retry,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AccountIdentityIssuerRuntimeCycle {
    attempts: u8,
    acknowledged: u8,
    exhausted: bool,
    startup_state: AccountIdentityIssuerStartupState,
}

impl AccountIdentityIssuerRuntimeCycle {
    fn new(
        attempts: u8,
        acknowledged: u8,
        exhausted: bool,
        startup_state: AccountIdentityIssuerStartupState,
    ) -> Self {
        Self {
            attempts,
            acknowledged,
            exhausted,
            startup_state,
        }
    }

    pub(crate) fn attempts(&self) -> u8 {
        self.attempts
    }

    pub(crate) fn acknowledged(&self) -> u8 {
        self.acknowledged
    }

    pub(crate) fn exhausted(&self) -> bool {
        self.exhausted
    }

    pub(crate) fn startup_state(&self) -> AccountIdentityIssuerStartupState {
        self.startup_state
    }
}

pub(crate) struct AccountIdentityIssuerRuntime {
    issuer: AccountIdentityIssuer,
    startup_state: AccountIdentityIssuerStartupState,
}

impl AccountIdentityIssuerRuntime {
    pub(crate) fn open(path: impl AsRef<Path>) -> Result<Self, AccountIdentityIssuerError> {
        let issuer = AccountIdentityIssuer::open(path)?;
        let startup_state = issuer.startup_state();
        Ok(Self {
            issuer,
            startup_state,
        })
    }

    pub(crate) fn from_issuer(
        mut issuer: AccountIdentityIssuer,
    ) -> Result<Self, AccountIdentityIssuerError> {
        let startup_state = issuer.recover_startup()?;
        Ok(Self {
            issuer,
            startup_state,
        })
    }

    pub(crate) fn startup_state(&self) -> AccountIdentityIssuerStartupState {
        self.startup_state
    }

    /// Revalidate every durable issuer row after a process or service restart.
    /// No in-memory signer, binding, or outbox value is restored as authority.
    pub(crate) fn recover_startup(
        &mut self,
    ) -> Result<AccountIdentityIssuerStartupState, AccountIdentityIssuerError> {
        self.startup_state = self.issuer.recover_startup()?;
        Ok(self.startup_state)
    }

    /// Install only owner-provided protected signer custody.  The wrapped
    /// adapter must keep private bytes outside this process; this method cannot
    /// provision a key or create a fallback signer.
    pub(crate) fn install_signer(&mut self, signer: Box<dyn AccountIdentityIssuerSignerAdapter>) {
        self.issuer.install_signer(signer);
    }

    /// Install one paired authenticated Cloudflare owner for both binding and
    /// delivery.  The owner port has no default implementation in this crate.
    pub(crate) fn install_cloudflare_delivery(
        &mut self,
        delivery: AccountIdentityIssuerCloudflareDelivery,
    ) {
        delivery.install_into(&mut self.issuer);
    }

    /// Drive a bounded delivery cycle.  A successful delivery is already
    /// acknowledged and reconciled by `deliver_next_pending`; retryable owner
    /// failures are released by the issuer delivery path before the next
    /// attempt.  Exhaustion is explicit and never loops indefinitely.
    pub(crate) fn drive_delivery_cycle(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        service: AccountIdentityIssuerService,
    ) -> Result<AccountIdentityIssuerRuntimeCycle, AccountIdentityIssuerError> {
        self.recover_startup()?;
        let mut attempts: u8 = 0;
        let mut acknowledged: u8 = 0;
        while attempts < MAX_DELIVERY_ATTEMPTS {
            attempts += 1;
            let step = self.delivery_step(authority, service, attempts < MAX_DELIVERY_ATTEMPTS)?;
            if matches!(step, AccountIdentityIssuerRuntimeStep::Delivered) {
                acknowledged = acknowledged.saturating_add(1);
                continue;
            }
            if matches!(step, AccountIdentityIssuerRuntimeStep::Empty) {
                return Ok(AccountIdentityIssuerRuntimeCycle::new(
                    attempts,
                    acknowledged,
                    false,
                    self.startup_state,
                ));
            }
        }
        Ok(AccountIdentityIssuerRuntimeCycle::new(
            attempts,
            acknowledged,
            true,
            self.startup_state,
        ))
    }

    fn delivery_step(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        service: AccountIdentityIssuerService,
        retry_allowed: bool,
    ) -> Result<AccountIdentityIssuerRuntimeStep, AccountIdentityIssuerError> {
        match self.issuer.deliver_next_pending(authority, service) {
            Ok(true) => Ok(AccountIdentityIssuerRuntimeStep::Delivered),
            Ok(false) => Ok(AccountIdentityIssuerRuntimeStep::Empty),
            Err(_error) if retry_allowed => {
                self.recover_startup()?;
                Ok(AccountIdentityIssuerRuntimeStep::Retry)
            }
            Err(error) => Err(error),
        }
    }
}
