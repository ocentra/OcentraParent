#![forbid(unsafe_code)]

mod codec;

pub mod account_issuer;
pub mod account_issuer_contract;
pub mod account_issuer_session;
pub mod account_issuer_v2_codec;
pub mod bootstrap;
pub mod constants;
pub mod handshake;
pub mod request;
pub mod response;
pub mod target;
pub mod transport;
pub mod types;

pub fn encode_bootstrap(
    packet: &bootstrap::BootstrapPacket,
) -> Result<Vec<u8>, types::ProtocolError> {
    codec::encode_bootstrap(packet)
}

pub fn decode_bootstrap(frame: &[u8]) -> Result<bootstrap::BootstrapPacket, types::ProtocolError> {
    codec::decode_bootstrap(frame)
}

pub fn encode_request(
    request: &request::authenticated::AuthenticatedRequest,
) -> Result<Vec<u8>, types::ProtocolError> {
    codec::encode_request(request)
}

pub fn decode_request(frame: &[u8]) -> Result<request::UntrustedRequest, types::ProtocolError> {
    codec::decode_request(frame)
}

pub fn encode_response(
    response: &response::UntrustedResponse,
) -> Result<Vec<u8>, types::ProtocolError> {
    codec::encode_response(response)
}

pub fn decode_response(frame: &[u8]) -> Result<response::UntrustedResponse, types::ProtocolError> {
    codec::decode_response(frame)
}

pub fn encode_client_hello(
    hello: &handshake::UntrustedClientHello,
) -> Result<Vec<u8>, types::ProtocolError> {
    codec::encode_client_hello(hello)
}

pub fn decode_client_hello(
    frame: &[u8],
) -> Result<handshake::UntrustedClientHello, types::ProtocolError> {
    codec::decode_client_hello(frame)
}

pub fn encode_broker_hello(
    hello: &handshake::UntrustedBrokerHello,
) -> Result<Vec<u8>, types::ProtocolError> {
    codec::encode_broker_hello(hello)
}

pub fn decode_broker_hello(
    frame: &[u8],
) -> Result<handshake::UntrustedBrokerHello, types::ProtocolError> {
    codec::decode_broker_hello(frame)
}

pub fn read_frame(reader: &mut impl std::io::Read) -> Result<Vec<u8>, types::ProtocolError> {
    codec::read_frame(reader)
}

pub fn write_frame(
    writer: &mut impl std::io::Write,
    frame: &[u8],
) -> Result<(), types::ProtocolError> {
    codec::write_frame(writer, frame)
}

pub fn encode_account_issuer_request(
    request: &account_issuer::AccountIssuerRequest,
) -> Result<Vec<u8>, types::ProtocolError> {
    account_issuer_v2_codec::encode_request(request)
}

pub fn decode_account_issuer_request(
    frame: &[u8],
) -> Result<account_issuer::AccountIssuerRequest, types::ProtocolError> {
    account_issuer_v2_codec::decode_request(frame)
}

pub fn encode_account_issuer_receipt(
    receipt: &account_issuer::AccountIssuerReceipt,
) -> Result<Vec<u8>, types::ProtocolError> {
    account_issuer_v2_codec::encode_receipt(receipt)
}

pub fn decode_account_issuer_receipt(
    frame: &[u8],
) -> Result<account_issuer::AccountIssuerReceipt, types::ProtocolError> {
    account_issuer_v2_codec::decode_receipt(frame)
}
