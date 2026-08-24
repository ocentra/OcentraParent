mod frame;
mod handshake;
mod request;
mod response;

use crate::types::ProtocolError;

pub(crate) fn encode_request(request: &crate::request::Request) -> Result<Vec<u8>, ProtocolError> {
    request::encode(request)
}

pub(crate) fn decode_request(frame: &[u8]) -> Result<crate::request::Request, ProtocolError> {
    request::decode(frame)
}

pub(crate) fn encode_response(
    response: &crate::response::Response,
) -> Result<Vec<u8>, ProtocolError> {
    response::encode(response)
}

pub(crate) fn decode_response(frame: &[u8]) -> Result<crate::response::Response, ProtocolError> {
    response::decode(frame)
}

pub(crate) fn encode_client_hello(
    hello: &crate::handshake::ClientHello,
) -> Result<Vec<u8>, ProtocolError> {
    handshake::encode_client(hello)
}

pub(crate) fn decode_client_hello(
    frame: &[u8],
) -> Result<crate::handshake::ClientHello, ProtocolError> {
    handshake::decode_client(frame)
}

pub(crate) fn encode_broker_hello(
    hello: &crate::handshake::BrokerHello,
) -> Result<Vec<u8>, ProtocolError> {
    handshake::encode_broker(hello)
}

pub(crate) fn decode_broker_hello(
    frame: &[u8],
) -> Result<crate::handshake::BrokerHello, ProtocolError> {
    handshake::decode_broker(frame)
}
