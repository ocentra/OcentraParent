//! Length-delimited AccountIssuer v2 transport codec.

#[path = "account_issuer_v2_codec_decode.rs"]
mod decode;
#[path = "account_issuer_v2_codec_encode.rs"]
mod encode;

use crate::account_issuer::{AccountIssuerReceipt, AccountIssuerRequest};
use crate::types::ProtocolError;

pub fn encode_request(request: &AccountIssuerRequest) -> Result<Vec<u8>, ProtocolError> {
    encode::request(request)
}

pub fn decode_request(frame: &[u8]) -> Result<AccountIssuerRequest, ProtocolError> {
    decode::request(frame)
}

pub fn encode_receipt(receipt: &AccountIssuerReceipt) -> Result<Vec<u8>, ProtocolError> {
    encode::receipt(receipt)
}

pub fn decode_receipt(frame: &[u8]) -> Result<AccountIssuerReceipt, ProtocolError> {
    decode::receipt(frame)
}
