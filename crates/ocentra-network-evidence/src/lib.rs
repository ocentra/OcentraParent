pub mod bundle;
pub mod cascade;
pub mod category;
pub mod classifier;
pub mod dns;
pub mod domain;
pub mod encrypted_dns;
pub mod fixtures;
pub mod flow;
pub mod http;
pub mod local_ai_queue;
pub mod managed_browser;
pub mod packet;
pub mod pcap;
pub mod policy;
pub mod process;
pub mod tls;
pub mod transfer;
pub mod tunnel;

mod process_support;

#[cfg(test)]
mod tests;

pub use bundle::{
    build_network_cross_slice_evidence_bundle, NetworkCrossSliceEvidenceBundle,
    NetworkCrossSliceEvidenceBundleError, NetworkCrossSliceEvidenceBundleInput,
    NetworkCrossSliceEvidenceSource,
};
pub use cascade::{
    route_network_evidence_cascade, NetworkCascadeNextCheck, NetworkCascadeSignalStrength,
    NetworkCascadeSource, NetworkCascadeSourceKind, NetworkEvidenceCascadeDecision,
    NetworkEvidenceCascadeError, NetworkEvidenceCascadeInput,
};
pub use category::{
    evaluate_category_source_update, lookup_domain_category, CategoryFreshnessState,
    CategoryMatchKind, CategorySourceCustody, CategoryUpdateDecision, DomainCategoryDatabase,
    DomainCategoryError, DomainCategoryLookup, DomainCategoryRecord, DomainCategorySource,
    NetworkCategory,
};
pub use classifier::{
    classify_social_video_game_activity, BrowserClassifierConfirmation, CdnClassifierHint,
    NetworkActivityClassification, NetworkActivityClassifierInput, NetworkClassifierBasis,
    NetworkClassifierError, ProcessClassifierHint,
};
pub use dns::{
    parse_dns_message, replay_dns_observations, DnsMessage, DnsObservation, DnsQueryType,
    DnsQuestion, DnsRecordData, DnsResourceRecord, NetworkEvidenceGrade, NetworkReplayError,
    NetworkReplaySummary,
};
pub use domain::{
    normalize_domain_with_public_suffix, DomainNormalizationError, NormalizedDomainEvidence,
    PublicSuffixModel,
};
pub use encrypted_dns::{
    detect_encrypted_dns_candidate, detect_quic_http3_limitation, EncryptedDnsCandidate,
    EncryptedDnsProtocol, QuicVisibilityLimitation,
};
pub use fixtures::{
    dns_query_frame_fixture, dns_query_pcap_fixture, dns_query_replay_expected,
    dns_response_payload_fixture, http_host_request_fixture, icmp_echo_frame_fixture,
    quic_initial_payload_fixture, tcp_syn_frame_fixture, tls_client_hello_no_sni_fixture,
    tls_client_hello_sni_fixture,
};
pub use flow::{
    aggregate_network_flows, aggregate_pcap_flows, flow_packets_from_pcap, NetworkFlowError,
    NetworkFlowKey, NetworkFlowPacket, NetworkFlowProtocol, NetworkFlowSession, NetworkFlowSummary,
};
pub use http::{parse_http_host, HttpHostObservation, HttpHostParseError};
pub use local_ai_queue::{
    plan_network_local_ai_queue, NetworkLocalAiQueueError, NetworkLocalAiQueueInput,
    NetworkLocalAiQueueInputKind, NetworkLocalAiQueueJob, NetworkLocalAiQueuePlan,
    NetworkLocalAiQueueStatus,
};
pub use managed_browser::{
    correlate_managed_browser_activity, ManagedBrowserCorrelation, ManagedBrowserCorrelationBasis,
    ManagedBrowserCorrelationError, ManagedBrowserCorrelationInput, ManagedBrowserCorrelationState,
    ManagedBrowserPageEvidence, NetworkManagedBrowserFlowEvidence,
};
pub use packet::{
    parse_network_packet, udp_payload_from_ethernet_ipv4, EthernetFrameMetadata, IpProtocol,
    Ipv4PacketMetadata, PacketParseError, ParsedNetworkPacket, TransportPacketMetadata,
    UdpPayloadView,
};
pub use pcap::{parse_pcap_packets, PcapPacket, PcapReplayError};
pub use policy::{
    map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
    NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingError,
    NetworkEvidencePolicyMappingInput, NetworkEvidencePolicyMode,
};
pub use process::{
    correlate_process_app_activity, NetworkAppInventoryEntry, NetworkFlowProcessObservation,
    NetworkProcessAppCorrelation, NetworkProcessAppCorrelationInput,
    NetworkProcessCorrelationBasis, NetworkProcessCorrelationError, NetworkProcessCorrelationState,
    NetworkProcessCorrelationUncertainty, NetworkProcessSnapshot,
};
pub use tls::{parse_tls_client_hello_sni, TlsClientHelloError, TlsClientHelloVisibility};
pub use transfer::{
    classify_remote_torrent_download_activity, NetworkTransferActivityKind, NetworkTransferBasis,
    NetworkTransferClassification, NetworkTransferClassifierError, NetworkTransferClassifierInput,
    NetworkTransferIndicator, NetworkTransferIndicatorEvidence, NetworkTransferUncertainty,
};
pub use tunnel::{
    classify_vpn_proxy_tunnel_activity, NetworkTunnelBasis, NetworkTunnelClassification,
    NetworkTunnelClassifierError, NetworkTunnelClassifierInput, NetworkTunnelIndicator,
    NetworkTunnelIndicatorEvidence, NetworkTunnelKind,
};
