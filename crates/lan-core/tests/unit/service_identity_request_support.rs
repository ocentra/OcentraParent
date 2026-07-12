use super::*;

pub(super) struct RequestText(pub(super) String);

pub(super) fn read_request(stream: &mut impl Read) -> RequestText {
    let mut request = Vec::new();
    let mut chunk = [0_u8; 512];
    let mut expected_total_len = None;

    loop {
        let read = stream.read(&mut chunk).value_or_unreachable();
        if read == 0 {
            break;
        }
        request.extend_from_slice(&chunk[..read]);
        if let Some(expected_total_len) = expected_total_len {
            if request.len() >= expected_total_len {
                break;
            }
            continue;
        }
        if let Some(header_end) = request.windows(4).position(|window| window == b"\r\n\r\n") {
            let header_len = header_end + 4;
            let content_length = String::from_utf8_lossy(&request[..header_end])
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.trim()
                        .eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().ok())
                        .flatten()
                })
                .unwrap_or(0);
            let total_len = header_len + content_length;
            if request.len() >= total_len {
                break;
            }
            expected_total_len = Some(total_len);
        }
    }

    RequestText(String::from_utf8_lossy(&request).into_owned())
}
