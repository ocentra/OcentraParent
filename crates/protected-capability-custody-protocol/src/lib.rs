#![forbid(unsafe_code)]

mod codec;

pub mod constants;
pub mod handshake;
pub mod request;
pub mod response;
pub mod target;
pub mod types;

pub fn encode_request(request: &request::Request) -> Result<Vec<u8>, types::ProtocolError> {
    codec::encode_request(request)
}

pub fn decode_request(frame: &[u8]) -> Result<request::Request, types::ProtocolError> {
    codec::decode_request(frame)
}

pub fn encode_response(response: &response::Response) -> Result<Vec<u8>, types::ProtocolError> {
    codec::encode_response(response)
}

pub fn decode_response(frame: &[u8]) -> Result<response::Response, types::ProtocolError> {
    codec::decode_response(frame)
}

pub fn encode_client_hello(
    hello: &handshake::ClientHello,
) -> Result<Vec<u8>, types::ProtocolError> {
    codec::encode_client_hello(hello)
}

pub fn decode_client_hello(frame: &[u8]) -> Result<handshake::ClientHello, types::ProtocolError> {
    codec::decode_client_hello(frame)
}

pub fn encode_broker_hello(
    hello: &handshake::BrokerHello,
) -> Result<Vec<u8>, types::ProtocolError> {
    codec::encode_broker_hello(hello)
}

pub fn decode_broker_hello(frame: &[u8]) -> Result<handshake::BrokerHello, types::ProtocolError> {
    codec::decode_broker_hello(frame)
}
