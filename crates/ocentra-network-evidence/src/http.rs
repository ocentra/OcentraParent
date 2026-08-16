#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpHostObservation {
    pub host: String,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpHostParseError {
    InvalidUtf8,
}

const HOST_HEADER_PREFIX: &str = "host:";
const HTTP_METHODS: [&str; 9] = [
    "GET ", "POST ", "HEAD ", "PUT ", "PATCH ", "DELETE ", "OPTIONS ", "CONNECT ", "TRACE ",
];

pub fn parse_http_host(payload: &[u8]) -> Result<Option<HttpHostObservation>, HttpHostParseError> {
    let Ok(text) = std::str::from_utf8(payload) else {
        return Ok(None);
    };
    if !looks_like_http_request(text) {
        return Ok(None);
    }

    for line in text.lines().skip(1) {
        if line.is_empty() {
            break;
        }
        if let Some(host) = header_value(line, HOST_HEADER_PREFIX) {
            return Ok(Some(HttpHostObservation {
                host: host.to_ascii_lowercase(),
                exact_url_available: false,
                decrypted_payload_available: false,
            }));
        }
    }

    Ok(None)
}

fn looks_like_http_request(text: &str) -> bool {
    HTTP_METHODS.iter().any(|method| text.starts_with(method))
}

fn header_value<'a>(line: &'a str, prefix: &str) -> Option<&'a str> {
    if line.len() < prefix.len() || !line[..prefix.len()].eq_ignore_ascii_case(prefix) {
        return None;
    }

    let value = line[prefix.len()..].trim();
    if value.is_empty() {
        return None;
    }

    Some(value)
}
