use super::PassiveSsdpNotifyHeaders;

pub(super) fn parse_notify_headers(payload: &[u8]) -> Option<PassiveSsdpNotifyHeaders> {
    let text = std::str::from_utf8(payload).ok()?;
    let mut lines = text.lines();
    let first_line = lines.next()?.trim_end_matches('\r').trim();
    if !first_line.eq_ignore_ascii_case("NOTIFY * HTTP/1.1") {
        return None;
    }
    let mut headers = PassiveSsdpNotifyHeaders::default();
    for line in lines {
        let line = line.trim_end_matches('\r');
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        headers.apply_header(name, value.trim());
    }
    Some(headers)
}

impl PassiveSsdpNotifyHeaders {
    fn apply_header(&mut self, name: &str, value: &str) {
        match name.trim().to_ascii_lowercase().as_str() {
            "nt" => self.notification_type = Some(value.to_string()),
            "nts" => self.notification_subtype = Some(value.to_string()),
            "usn" => self.usn = Some(value.to_string()),
            "location" => self.location = Some(value.to_string()),
            _ => {}
        }
    }

    pub(super) fn summary(self) -> String {
        let mut summary = String::from("SSDP notify");
        append_summary_part(&mut summary, "nt", self.notification_type);
        append_summary_part(&mut summary, "nts", self.notification_subtype);
        append_summary_part(&mut summary, "usn", self.usn);
        append_summary_part(&mut summary, "location", self.location);
        summary
    }
}

fn append_summary_part(summary: &mut String, name: &str, value: Option<String>) {
    let Some(value) = value else {
        return;
    };
    summary.push(if summary == "SSDP notify" { ':' } else { ';' });
    summary.push(' ');
    summary.push_str(name);
    summary.push('=');
    summary.push_str(&value);
}
