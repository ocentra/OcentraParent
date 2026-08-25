//! One-use safe facade for fixed TPM counter ceremonies.

use super::super::codec_types::signer::{TpmPolicySignature, TpmPolicySignerPublic};
use super::prepared::{self, PreparedCounterOperation};
use super::CounterOutcome;
use crate::{Error, OwnedTbsContext, Result, TpmCounterIncrementOutcome};

/// A one-use, nonce-bound TPM counter read awaiting a PCP signature.
pub struct PreparedTpmCounterRead<'a> {
    inner: Option<PreparedCounterOperation<'a>>,
}

/// A one-use, nonce-bound TPM counter increment awaiting a PCP signature.
pub struct PreparedTpmCounterIncrement<'a> {
    inner: Option<PreparedCounterOperation<'a>>,
}

impl PreparedTpmCounterRead<'_> {
    pub fn signing_digest(&self) -> Result<[u8; 32]> {
        prepared_digest(&self.inner)
    }

    pub fn execute(mut self, signature: TpmPolicySignature) -> Result<u64> {
        match take_inner(&mut self.inner)?.execute(&signature)? {
            CounterOutcome::Read(value) => Ok(value),
            CounterOutcome::Increment(_) => Err(Error::MalformedTpm),
        }
    }

    pub fn close(mut self) -> Result<()> {
        take_inner(&mut self.inner)?.close()
    }
}

impl PreparedTpmCounterIncrement<'_> {
    pub fn signing_digest(&self) -> Result<[u8; 32]> {
        prepared_digest(&self.inner)
    }

    pub fn execute(mut self, signature: TpmPolicySignature) -> Result<TpmCounterIncrementOutcome> {
        match take_inner(&mut self.inner)?.execute(&signature)? {
            CounterOutcome::Increment(outcome) => Ok(outcome),
            CounterOutcome::Read(_) => Err(Error::MalformedTpm),
        }
    }

    pub fn close(mut self) -> Result<()> {
        take_inner(&mut self.inner)?.close()
    }
}

impl OwnedTbsContext {
    pub fn prepare_fixed_counter_read(
        &self,
        signer: &TpmPolicySignerPublic,
    ) -> Result<PreparedTpmCounterRead<'_>> {
        Ok(PreparedTpmCounterRead {
            inner: Some(prepared::prepare_read(self, signer)?),
        })
    }

    pub fn prepare_fixed_counter_increment(
        &self,
        signer: &TpmPolicySignerPublic,
    ) -> Result<PreparedTpmCounterIncrement<'_>> {
        Ok(PreparedTpmCounterIncrement {
            inner: Some(prepared::prepare_increment(self, signer)?),
        })
    }
}

fn prepared_digest(inner: &Option<PreparedCounterOperation<'_>>) -> Result<[u8; 32]> {
    inner
        .as_ref()
        .map(PreparedCounterOperation::signing_digest)
        .ok_or(Error::MalformedTpm)
}

fn take_inner<'a>(
    inner: &mut Option<PreparedCounterOperation<'a>>,
) -> Result<PreparedCounterOperation<'a>> {
    inner.take().ok_or(Error::MalformedTpm)
}
