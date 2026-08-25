mod bootstrap;
mod frame;
mod handshake;
mod request;
mod response;

use crate::types::ProtocolError;

pub(crate) fn encode_bootstrap(
    packet: &crate::bootstrap::BootstrapPacket,
) -> Result<Vec<u8>, ProtocolError> {
    bootstrap::encode(packet)
}

pub(crate) fn decode_bootstrap(
    frame: &[u8],
) -> Result<crate::bootstrap::BootstrapPacket, ProtocolError> {
    bootstrap::decode(frame)
}

pub(crate) fn encode_request(
    request: &crate::request::authenticated::AuthenticatedRequest,
) -> Result<Vec<u8>, ProtocolError> {
    request::encode(request)
}

pub(crate) fn decode_request(
    frame: &[u8],
) -> Result<crate::request::UntrustedRequest, ProtocolError> {
    request::decode(frame)
}

pub(crate) fn encode_response(
    response: &crate::response::UntrustedResponse,
) -> Result<Vec<u8>, ProtocolError> {
    response::encode(response)
}

pub(crate) fn decode_response(
    frame: &[u8],
) -> Result<crate::response::UntrustedResponse, ProtocolError> {
    response::decode(frame)
}

pub(crate) fn encode_client_hello(
    hello: &crate::handshake::UntrustedClientHello,
) -> Result<Vec<u8>, ProtocolError> {
    handshake::encode_client(hello)
}

pub(crate) fn decode_client_hello(
    frame: &[u8],
) -> Result<crate::handshake::UntrustedClientHello, ProtocolError> {
    handshake::decode_client(frame)
}

pub(crate) fn encode_broker_hello(
    hello: &crate::handshake::UntrustedBrokerHello,
) -> Result<Vec<u8>, ProtocolError> {
    handshake::encode_broker(hello)
}

pub(crate) fn decode_broker_hello(
    frame: &[u8],
) -> Result<crate::handshake::UntrustedBrokerHello, ProtocolError> {
    handshake::decode_broker(frame)
}

pub(crate) fn read_frame(reader: &mut impl std::io::Read) -> Result<Vec<u8>, ProtocolError> {
    frame::read_frame(reader)
}

pub(crate) fn write_frame(
    writer: &mut impl std::io::Write,
    encoded: &[u8],
) -> Result<(), ProtocolError> {
    frame::write_frame(writer, encoded)
}
