use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind;

pub(super) fn service_probe_kind_value(
    kind: &LanServiceIdentityProbeEvidenceKind,
) -> Option<&'static str> {
    match kind {
        LanServiceIdentityProbeEvidenceKind::HttpStatus => Some("http-status"),
        LanServiceIdentityProbeEvidenceKind::HtmlTitle => Some("html-title"),
        LanServiceIdentityProbeEvidenceKind::ServerHeader => Some("server-header"),
        LanServiceIdentityProbeEvidenceKind::Banner => Some("banner"),
        LanServiceIdentityProbeEvidenceKind::RedirectLocation => Some("redirect-location"),
        LanServiceIdentityProbeEvidenceKind::CertificateSubject => Some("certificate-subject"),
        LanServiceIdentityProbeEvidenceKind::DescriptorLink => Some("descriptor-link"),
        _ => None,
    }
}
