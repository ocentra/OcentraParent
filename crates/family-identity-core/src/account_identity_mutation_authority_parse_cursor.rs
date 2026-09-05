use crate::account_identity_mutation_authority::envelope::MAX_CANONICAL_FIELD_BYTES;
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

pub(super) struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub(super) fn read_strings<const N: usize>(
        &mut self,
    ) -> Result<[String; N], AccountIdentityMutationAuthorityError> {
        let mut values = Vec::with_capacity(N);
        for _ in 0..N {
            values.push(self.read_string()?);
        }
        values
            .try_into()
            .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidEnvelope)
    }

    pub(super) fn read_u64s<const N: usize>(
        &mut self,
    ) -> Result<[u64; N], AccountIdentityMutationAuthorityError> {
        self.read_numbers(u64::from_be_bytes)
    }

    pub(super) fn read_i64s<const N: usize>(
        &mut self,
    ) -> Result<[i64; N], AccountIdentityMutationAuthorityError> {
        self.read_numbers(i64::from_be_bytes)
    }

    fn read_numbers<T, const N: usize>(
        &mut self,
        decode: impl Fn([u8; 8]) -> T,
    ) -> Result<[T; N], AccountIdentityMutationAuthorityError> {
        let mut values = Vec::with_capacity(N);
        for _ in 0..N {
            values.push(decode(self.read_array::<8>()?));
        }
        values
            .try_into()
            .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidEnvelope)
    }

    fn read_string(&mut self) -> Result<String, AccountIdentityMutationAuthorityError> {
        let length = usize::try_from(u32::from_be_bytes(self.read_array::<4>()?))
            .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
        if length > MAX_CANONICAL_FIELD_BYTES {
            return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
        }
        let end = self
            .offset
            .checked_add(length)
            .ok_or(AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
        self.offset = end;
        std::str::from_utf8(value)
            .map(str::to_owned)
            .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidEnvelope)
    }

    fn read_array<const N: usize>(
        &mut self,
    ) -> Result<[u8; N], AccountIdentityMutationAuthorityError> {
        let end = self
            .offset
            .checked_add(N)
            .ok_or(AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .and_then(|value| <[u8; N]>::try_from(value).ok())
            .ok_or(AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
        self.offset = end;
        Ok(value)
    }

    pub(super) fn finish(self) -> Result<(), AccountIdentityMutationAuthorityError> {
        (self.offset == self.bytes.len())
            .then_some(())
            .ok_or(AccountIdentityMutationAuthorityError::InvalidEnvelope)
    }
}
