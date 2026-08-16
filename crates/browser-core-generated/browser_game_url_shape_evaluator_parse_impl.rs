use super::*;

pub(super) enum ParseBrowserGameUrlError {
    Invalid,
    UnsupportedProtocol(BrowserGameShapeCode),
}

pub(super) fn parse_browser_game_url(
    input: &BrowserGameUrlText,
) -> Result<BrowserGameParsedUrl<'_>, ParseBrowserGameUrlError> {
    let input = input.0.as_str();
    let colon_index = input.find(':').ok_or(ParseBrowserGameUrlError::Invalid)?;
    if colon_index == 0 {
        return Err(ParseBrowserGameUrlError::Invalid);
    }

    let protocol = &input[..=colon_index];
    if protocol != PROTOCOL_HTTP && protocol != PROTOCOL_HTTPS {
        let protocol_shape = if protocol.is_empty() {
            PROTOCOL_MISSING
        } else {
            PROTOCOL_NON_HTTP
        };
        return Err(ParseBrowserGameUrlError::UnsupportedProtocol(
            protocol_shape,
        ));
    }

    let after_protocol = &input[colon_index + 1..];
    if !after_protocol.starts_with(URL_AUTHORITY_PREFIX) {
        return Err(ParseBrowserGameUrlError::Invalid);
    }

    let remainder = &after_protocol[2..];
    let host_end = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
    let hostname = &remainder[..host_end];
    if hostname.trim().is_empty() {
        return Err(ParseBrowserGameUrlError::Invalid);
    }

    let path_and_more = &remainder[host_end..];
    let hash_index = path_and_more.find('#');
    let (before_hash, hash) = match hash_index {
        Some(index) => (&path_and_more[..index], &path_and_more[index..]),
        None => (path_and_more, EMPTY_TEXT),
    };
    let search_index = before_hash.find('?');
    let (pathname, search) = match search_index {
        Some(index) => (&before_hash[..index], &before_hash[index..]),
        None => (before_hash, EMPTY_TEXT),
    };

    Ok(BrowserGameParsedUrl {
        hostname,
        pathname: if pathname.is_empty() {
            PATH_ROOT_VALUE
        } else {
            pathname
        },
        search,
        hash,
    })
}
