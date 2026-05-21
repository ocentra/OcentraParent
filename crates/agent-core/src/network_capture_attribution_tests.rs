use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus,
    ActivityProcessAttributionStatus,
};

use super::NetworkObservation;

#[test]
fn domain_attribution_status_is_domain_observed_when_domain_present() {
    let obs = NetworkObservation {
        destination_domain: Some(constants::test_network::TEST_DOMAIN.to_string()),
        destination_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        ..NetworkObservation::degraded(ActivityCaptureCapabilityStatus::Available)
    };
    assert_eq!(
        obs.domain_attribution_status(),
        ActivityDomainAttributionStatus::DomainObserved
    );
}

#[test]
fn domain_attribution_status_is_ip_only_when_no_domain_but_ip_present() {
    let obs = NetworkObservation {
        destination_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        destination_port: Some(443),
        ..NetworkObservation::degraded(ActivityCaptureCapabilityStatus::Available)
    };
    assert_eq!(
        obs.domain_attribution_status(),
        ActivityDomainAttributionStatus::IpOnly
    );
}

#[test]
fn domain_attribution_status_is_unavailable_when_nothing_present() {
    let obs = NetworkObservation::degraded(ActivityCaptureCapabilityStatus::NoNetworkObservations);
    assert_eq!(
        obs.domain_attribution_status(),
        ActivityDomainAttributionStatus::Unavailable
    );
}

#[test]
fn process_attribution_is_attributed_when_pid_present() {
    let obs = NetworkObservation {
        pid: Some(1234),
        ..NetworkObservation::degraded(ActivityCaptureCapabilityStatus::Available)
    };
    assert_eq!(
        obs.process_attribution_status(),
        ActivityProcessAttributionStatus::ProcessAttributed
    );
}

#[test]
fn process_attribution_is_unknown_when_pid_absent() {
    let obs = NetworkObservation::degraded(ActivityCaptureCapabilityStatus::NoNetworkObservations);
    assert_eq!(
        obs.process_attribution_status(),
        ActivityProcessAttributionStatus::ProcessUnknown
    );
}
