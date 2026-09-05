const DOMAIN_SEPARATOR: &[u8] = b"ocentra.account-authority-producer.signing.v1\0";

pub(crate) struct CanonicalAuthorityProducerEnvelope {
    pub(crate) key_id: String,
}

pub(crate) fn domain_separator() -> &'static [u8] {
    DOMAIN_SEPARATOR
}
