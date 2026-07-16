use crate::dns::types::NetworkEvidenceGrade;
use crate::flow::{NetworkFlowKey, NetworkFlowPacket, NetworkFlowSession};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FlowDirection {
    InitiatorToResponder,
    ResponderToInitiator,
}

pub(crate) fn aggregate_network_flows(
    packets: &[NetworkFlowPacket],
    idle_timeout_micros: u64,
) -> Vec<NetworkFlowSession> {
    let mut ordered_packets = packets.iter().collect::<Vec<_>>();
    ordered_packets.sort_by_key(|packet| packet.observed_at_micros);
    let mut sessions: Vec<NetworkFlowSession> = Vec::new();
    for packet in ordered_packets {
        let mut matched = false;
        for session in &mut sessions {
            if let Some(direction) = session.match_direction(packet, idle_timeout_micros) {
                session.record_packet(packet, direction);
                matched = true;
                break;
            }
        }
        if !matched {
            sessions.push(NetworkFlowSession::from_packet(packet));
        }
    }

    sessions
}

impl NetworkFlowSession {
    pub(super) fn from_packet(packet: &NetworkFlowPacket) -> Self {
        Self {
            key: NetworkFlowKey {
                initiator_ip: packet.source_ip.clone(),
                initiator_port: packet.source_port,
                responder_ip: packet.destination_ip.clone(),
                responder_port: packet.destination_port,
                protocol: packet.protocol,
            },
            first_seen_micros: packet.observed_at_micros,
            last_seen_micros: packet.observed_at_micros,
            duration_micros: 0,
            packet_count: 1,
            initiator_to_responder_packets: 1,
            responder_to_initiator_packets: 0,
            initiator_to_responder_bytes: packet.observed_bytes,
            responder_to_initiator_bytes: 0,
            evidence_grade: NetworkEvidenceGrade::C,
            exact_url_available: false,
            decrypted_payload_available: false,
        }
    }

    pub(super) fn match_direction(
        &self,
        packet: &NetworkFlowPacket,
        idle_timeout_micros: u64,
    ) -> Option<FlowDirection> {
        if packet.protocol != self.key.protocol {
            return None;
        }
        if packet.observed_at_micros > self.last_seen_micros.saturating_add(idle_timeout_micros) {
            return None;
        }
        if self.matches_initiator_to_responder(packet) {
            return Some(FlowDirection::InitiatorToResponder);
        }
        if self.matches_responder_to_initiator(packet) {
            return Some(FlowDirection::ResponderToInitiator);
        }

        None
    }

    fn matches_initiator_to_responder(&self, packet: &NetworkFlowPacket) -> bool {
        packet.source_ip == self.key.initiator_ip
            && packet.source_port == self.key.initiator_port
            && packet.destination_ip == self.key.responder_ip
            && packet.destination_port == self.key.responder_port
    }

    fn matches_responder_to_initiator(&self, packet: &NetworkFlowPacket) -> bool {
        packet.source_ip == self.key.responder_ip
            && packet.source_port == self.key.responder_port
            && packet.destination_ip == self.key.initiator_ip
            && packet.destination_port == self.key.initiator_port
    }

    fn record_packet(&mut self, packet: &NetworkFlowPacket, direction: FlowDirection) {
        self.last_seen_micros = self.last_seen_micros.max(packet.observed_at_micros);
        self.duration_micros = self.last_seen_micros - self.first_seen_micros;
        self.packet_count += 1;
        match direction {
            FlowDirection::InitiatorToResponder => {
                self.initiator_to_responder_packets += 1;
                self.initiator_to_responder_bytes += packet.observed_bytes;
            }
            FlowDirection::ResponderToInitiator => {
                self.responder_to_initiator_packets += 1;
                self.responder_to_initiator_bytes += packet.observed_bytes;
            }
        }
    }
}
