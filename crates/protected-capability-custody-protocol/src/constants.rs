pub const PROTOCOL_VERSION: u16 = 1;
pub const PROTOCOL_DOMAIN: &[u8] = b"ocentra.protected-capability-custody.protocol.v1";
pub const FRAME_PREFIX_BYTES: usize = 4;
pub const NONCE_BYTES: usize = 32;
pub const CORRELATION_BYTES: usize = 16;
pub const OPAQUE_TOKEN_BYTES: usize = 96;
pub const SESSION_HANDLE_BYTES: usize = 32;
pub const ATTESTATION_DIGEST_BYTES: usize = 32;
pub const MAX_FRAME_BYTES: usize = 64 * 1024;
pub const MAX_FIELD_BYTES: usize = 1024;
pub const BROKER_PIPE_NAME: &str = r"\\.\pipe\ocentra-protected-capability-custody-v1";

pub(crate) const MESSAGE_REQUEST: u8 = 1;
pub(crate) const MESSAGE_RESPONSE: u8 = 2;
pub(crate) const MESSAGE_CLIENT_HELLO: u8 = 3;
pub(crate) const MESSAGE_BROKER_HELLO: u8 = 4;
