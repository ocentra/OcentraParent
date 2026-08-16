use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanServiceIdentityProbeEvidence, LanServiceIdentityProbeEvidenceKind,
};

use super::LanServiceIdentityProbeObservation;

impl LanServiceIdentityProbeObservation {
    pub(super) fn is_meaningful(&self) -> bool {
        self.status_code.is_some()
            || self.snmp_sys_descr.is_some()
            || self.snmp_sys_name.is_some()
            || self.title.is_some()
            || self.server_header.is_some()
            || self.banner.is_some()
            || self.redirect_location.is_some()
            || self.certificate_subject.is_some()
            || !self.descriptor_links.is_empty()
            || self.wsd_endpoint_address.is_some()
            || self.wsd_types.is_some()
    }

    pub fn observed_allowed_snmp_response(&self) -> bool {
        self.snmp_sys_descr.is_some() || self.snmp_sys_name.is_some()
    }

    pub fn into_evidence(self) -> Vec<LanServiceIdentityProbeEvidence> {
        self.into_evidence_with_selected_interface(None)
    }

    pub fn into_evidence_with_selected_interface(
        self,
        selected_interface: Option<String>,
    ) -> Vec<LanServiceIdentityProbeEvidence> {
        let mut evidence = Vec::new();
        if let Some(status_code) = self.status_code {
            evidence.push(LanServiceIdentityProbeEvidence {
                evidence_kind: LanServiceIdentityProbeEvidenceKind::HttpStatus,
                value: status_code.to_string(),
                selected_interface: selected_interface.clone(),
            });
        }
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::HtmlTitle,
            self.title,
            selected_interface.clone(),
        );
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::ServerHeader,
            self.server_header,
            selected_interface.clone(),
        );
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::Banner,
            self.banner,
            selected_interface.clone(),
        );
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::RedirectLocation,
            self.redirect_location,
            selected_interface.clone(),
        );
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::CertificateSubject,
            self.certificate_subject,
            selected_interface.clone(),
        );
        for descriptor_link in self.descriptor_links {
            push_probe_evidence(
                &mut evidence,
                LanServiceIdentityProbeEvidenceKind::DescriptorLink,
                Some(descriptor_link),
                selected_interface.clone(),
            );
        }
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::WsdEndpointAddress,
            self.wsd_endpoint_address,
            selected_interface.clone(),
        );
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::WsdTypes,
            self.wsd_types,
            selected_interface.clone(),
        );
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::SnmpSysDescr,
            self.snmp_sys_descr,
            selected_interface.clone(),
        );
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::SnmpSysName,
            self.snmp_sys_name,
            selected_interface,
        );
        evidence
    }
}

pub(super) fn push_probe_evidence(
    evidence: &mut Vec<LanServiceIdentityProbeEvidence>,
    evidence_kind: LanServiceIdentityProbeEvidenceKind,
    value: Option<String>,
    selected_interface: Option<String>,
) {
    let Some(value) = value else {
        return;
    };
    if evidence
        .iter()
        .any(|item| item.evidence_kind == evidence_kind && item.value == value)
    {
        return;
    }
    evidence.push(LanServiceIdentityProbeEvidence {
        evidence_kind,
        value,
        selected_interface,
    });
}

pub(super) fn merge_service_identity_probe_evidence(
    existing: Vec<LanServiceIdentityProbeEvidence>,
    incoming: Vec<LanServiceIdentityProbeEvidence>,
) -> Vec<LanServiceIdentityProbeEvidence> {
    let mut merged = Vec::with_capacity(existing.len().saturating_add(incoming.len()));
    for evidence in existing.into_iter().chain(incoming) {
        merge_or_append_probe_evidence(&mut merged, evidence);
    }
    merged
}

fn merge_or_append_probe_evidence(
    merged: &mut Vec<LanServiceIdentityProbeEvidence>,
    evidence: LanServiceIdentityProbeEvidence,
) {
    let Some(existing) = merged
        .iter_mut()
        .find(|item| item.evidence_kind == evidence.evidence_kind && item.value == evidence.value)
    else {
        merged.push(evidence);
        return;
    };
    if existing.selected_interface.is_none() {
        existing.selected_interface = evidence.selected_interface;
    }
}
