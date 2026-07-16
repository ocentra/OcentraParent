use super::*;

pub(super) fn host_shape_for(hostname: &BrowserGameUrlText) -> BrowserGameShapeCode {
    let lowercase = hostname.0.to_ascii_lowercase();
    if lowercase == HOST_LOCALHOST {
        return HOST_LOCALHOST_LIKE;
    }
    if lowercase.split('.').all(|segment| {
        !segment.is_empty()
            && segment.len() <= 3
            && segment.chars().all(|character| character.is_ascii_digit())
    }) && lowercase.matches('.').count() == 3
    {
        return HOST_IP_LIKE;
    }
    if lowercase.contains('.') {
        return HOST_DOMAIN_LIKE;
    }
    HOST_UNKNOWN
}
