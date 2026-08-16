use super::super::super::SsdpDiscoveryError;

pub(super) fn parse_authority(authority: &str) -> Result<(&str, u16), SsdpDiscoveryError> {
    if let Some(stripped) = authority.strip_prefix('[') {
        let (host, port) = stripped
            .split_once(']')
            .ok_or(SsdpDiscoveryError::MalformedResponse)?;
        let port = port
            .strip_prefix(':')
            .ok_or(SsdpDiscoveryError::MalformedResponse)?;
        return Ok((host, parse_port(port)?));
    }
    let (host, port) = authority
        .rsplit_once(':')
        .map(|(host, port)| (host, parse_port(port)))
        .unwrap_or((authority, Ok(80)));
    if host.trim().is_empty() {
        return Err(SsdpDiscoveryError::MalformedResponse);
    }
    Ok((host, port?))
}

pub(super) fn parse_port(value: &str) -> Result<u16, SsdpDiscoveryError> {
    value
        .parse::<u16>()
        .map_err(|_error| SsdpDiscoveryError::MalformedResponse)
}
