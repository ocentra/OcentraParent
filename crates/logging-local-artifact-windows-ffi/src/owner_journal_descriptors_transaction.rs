use sha2::{Digest, Sha256};

use crate::owner_types::Mutation;

use super::super::{DescriptorDigest, TRANSACTION_DESCRIPTOR_DOMAIN};

pub(crate) trait TransactionDescriptorInput {
    fn update_descriptor(&self, hasher: &mut Sha256);
}

pub(crate) trait TransactionDescriptorOutputInput<D> {}

impl TransactionDescriptorInput for [Mutation] {
    fn update_descriptor(&self, hasher: &mut Sha256) {
        for mutation in self {
            hasher.update(mutation.operation_name().as_bytes());
            hasher.update([0]);
            hasher.update(mutation.relative_path().as_bytes());
            hasher.update([0]);
            if let Some(payload) = mutation.payload() {
                hasher.update(payload);
            }
            hasher.update([0xff]);
        }
    }
}

impl TransactionDescriptorOutputInput<String> for [Mutation] {}

pub(in crate::owner_journal) fn transaction_descriptor<M>(mutations: &M) -> DescriptorDigest
where
    M: TransactionDescriptorInput + ?Sized,
{
    let mut hasher = Sha256::new();
    hasher.update(TRANSACTION_DESCRIPTOR_DOMAIN.as_bytes());
    mutations.update_descriptor(&mut hasher);
    let digest: [u8; 32] = hasher.finalize().into();
    DescriptorDigest(digest)
}
