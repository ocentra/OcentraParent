//! Fixed policy and NV-public digest mechanics.

use super::auth::Sha256Digest;
use super::handles::FixedNvOperation;
use super::signer::TpmPolicySignerPublic;
use crate::tpm::{
    TPM_CC_POLICY_COMMAND_CODE, TPM_CC_POLICY_OR, TPM_CC_POLICY_SIGNED, TPM_MAX_POLICY_OR_DIGESTS,
};

const POLICY_REF: [u8; 25] = [
    111, 99, 101, 110, 116, 114, 97, 46, 112, 99, 99, 46, 110, 118, 45, 99, 111, 117, 110, 116,
    101, 114, 46, 118, 49,
];
const POLICY_EXPIRATION: i32 = 0;

pub(crate) struct FixedPolicyProfile {
    read_branch: Sha256Digest,
    increment_branch: Sha256Digest,
    counter_policy: Sha256Digest,
}

impl FixedPolicyProfile {
    pub(crate) fn for_signer(signer: &TpmPolicySignerPublic) -> Self {
        let signed = signed_policy_digest(signer.name());
        let read_branch = command_policy_digest(signed, FixedNvOperation::Read);
        let increment_branch = command_policy_digest(signed, FixedNvOperation::Increment);
        let counter_policy = or_policy_digest(&[read_branch, increment_branch]);
        Self {
            read_branch,
            increment_branch,
            counter_policy,
        }
    }

    pub(crate) fn policy_ref(&self) -> &'static [u8] {
        &POLICY_REF
    }

    pub(crate) fn expiration(&self) -> i32 {
        POLICY_EXPIRATION
    }

    pub(crate) fn counter_policy(&self) -> &Sha256Digest {
        &self.counter_policy
    }

    pub(crate) fn branches(&self) -> [&Sha256Digest; 2] {
        [&self.read_branch, &self.increment_branch]
    }
}

impl TpmPolicySignerPublic {
    /// The exact policy digest provisioned on the fixed NV counter.
    pub fn fixed_counter_policy_digest(&self) -> [u8; 32] {
        *FixedPolicyProfile::for_signer(self)
            .counter_policy()
            .as_bytes()
    }
}

fn signed_policy_digest(signer_name: &[u8]) -> Sha256Digest {
    let zero = [0u8; 32];
    let name_update =
        Sha256Digest::hash(&[&zero, &TPM_CC_POLICY_SIGNED.to_be_bytes(), signer_name]);
    Sha256Digest::hash(&[name_update.as_bytes(), &POLICY_REF])
}

fn command_policy_digest(previous: Sha256Digest, operation: FixedNvOperation) -> Sha256Digest {
    Sha256Digest::hash(&[
        previous.as_bytes(),
        &TPM_CC_POLICY_COMMAND_CODE.to_be_bytes(),
        &operation.command_code().to_be_bytes(),
    ])
}

fn or_policy_digest(branches: &[Sha256Digest]) -> Sha256Digest {
    debug_assert!(branches.len() <= TPM_MAX_POLICY_OR_DIGESTS);
    let zero = [0u8; 32];
    let mut input = Vec::with_capacity(36 + branches.len() * 32);
    input.extend_from_slice(&zero);
    input.extend_from_slice(&TPM_CC_POLICY_OR.to_be_bytes());
    for digest in branches {
        input.extend_from_slice(digest.as_bytes());
    }
    Sha256Digest::hash(&[&input])
}
