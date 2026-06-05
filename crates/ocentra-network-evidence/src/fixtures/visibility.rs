pub fn tls_client_hello_sni_fixture() -> Vec<u8> {
    tls_client_hello(Some("video.example.test"))
}

pub fn tls_client_hello_no_sni_fixture() -> Vec<u8> {
    tls_client_hello(None)
}

pub fn http_host_request_fixture() -> Vec<u8> {
    b"GET /watch?v=secret HTTP/1.1\r\nHost: Video.Example.Test\r\nUser-Agent: fixture\r\n\r\n"
        .to_vec()
}

pub fn quic_initial_payload_fixture() -> Vec<u8> {
    vec![
        0xc3, 0x00, 0x00, 0x00, 0x01, 0x08, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11, 0x22,
        0x33,
    ]
}

fn tls_client_hello(sni: Option<&str>) -> Vec<u8> {
    let extensions = sni.map(sni_extension).unwrap_or_default();
    let mut body = Vec::new();
    body.extend_from_slice(&0x0303_u16.to_be_bytes());
    body.extend_from_slice(&[0; 32]);
    body.push(0);
    body.extend_from_slice(&2_u16.to_be_bytes());
    body.extend_from_slice(&0x1301_u16.to_be_bytes());
    body.push(1);
    body.push(0);
    body.extend_from_slice(&(extensions.len() as u16).to_be_bytes());
    body.extend_from_slice(&extensions);

    let mut handshake = Vec::new();
    handshake.push(1);
    push_u24(&mut handshake, body.len());
    handshake.extend_from_slice(&body);

    let mut record = Vec::new();
    record.push(22);
    record.extend_from_slice(&0x0303_u16.to_be_bytes());
    record.extend_from_slice(&(handshake.len() as u16).to_be_bytes());
    record.extend_from_slice(&handshake);
    record
}

fn sni_extension(host: &str) -> Vec<u8> {
    let host = host.as_bytes();
    let server_name_len = 1 + 2 + host.len();
    let extension_data_len = 2 + server_name_len;
    let mut extension = Vec::new();
    extension.extend_from_slice(&0_u16.to_be_bytes());
    extension.extend_from_slice(&(extension_data_len as u16).to_be_bytes());
    extension.extend_from_slice(&(server_name_len as u16).to_be_bytes());
    extension.push(0);
    extension.extend_from_slice(&(host.len() as u16).to_be_bytes());
    extension.extend_from_slice(host);
    extension
}

fn push_u24(bytes: &mut Vec<u8>, value: usize) {
    bytes.push(((value >> 16) & 0xff) as u8);
    bytes.push(((value >> 8) & 0xff) as u8);
    bytes.push((value & 0xff) as u8);
}
