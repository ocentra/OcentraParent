fn network_remote_delivery_status_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkRemoteDeliveryStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "FixtureRequirementsRecordedButNotImplemented",
            value: NetworkRemoteDeliveryStatusState::FixtureRequirementsRecordedButNotImplemented,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkRemoteDeliveryStatusState::ManualRequired,
        },
    ]
}

fn network_remote_delivery_transport_dispatch_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkRemoteDeliveryTransportDispatchState>> {
    vec![ProtocolLiteralDescriptor {
        key: "ManualRequiredBlocked",
        value: NetworkRemoteDeliveryTransportDispatchState::ManualRequiredBlocked,
    }]
}

fn network_remote_delivery_provider_child_readiness_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkRemoteDeliveryProviderChildReadinessState>> {
    vec![ProtocolLiteralDescriptor {
        key: "ManualRequiredUnavailable",
        value: NetworkRemoteDeliveryProviderChildReadinessState::ManualRequiredUnavailable,
    }]
}

fn network_remote_delivery_cross_process_custody_readiness_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkRemoteDeliveryCrossProcessCustodyReadinessState>> {
    vec![ProtocolLiteralDescriptor {
        key: "ManualRequiredUnavailable",
        value: NetworkRemoteDeliveryCrossProcessCustodyReadinessState::ManualRequiredUnavailable,
    }]
}

fn network_remote_delivery_external_cross_process_transport_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkRemoteDeliveryExternalCrossProcessTransportState>> {
    vec![ProtocolLiteralDescriptor {
        key: "DeterministicEnvelopeAckRecorded",
        value: NetworkRemoteDeliveryExternalCrossProcessTransportState::DeterministicEnvelopeAckRecorded,
    }]
}

fn network_live_capture_platform_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkLiveCaptureStatusPlatform>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "WindowsNpcap",
            value: NetworkLiveCaptureStatusPlatform::WindowsNpcap,
        },
        ProtocolLiteralDescriptor {
            key: "LinuxLibpcap",
            value: NetworkLiveCaptureStatusPlatform::LinuxLibpcap,
        },
        ProtocolLiteralDescriptor {
            key: "MacosBpfLibpcap",
            value: NetworkLiveCaptureStatusPlatform::MacosBpfLibpcap,
        },
    ]
}

fn network_live_capture_proof_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkLiveCaptureProofStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ProofReady",
            value: NetworkLiveCaptureProofStatusState::ProofReady,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkLiveCaptureProofStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkLiveCaptureProofStatusState::Unavailable,
        },
        ProtocolLiteralDescriptor {
            key: "Degraded",
            value: NetworkLiveCaptureProofStatusState::Degraded,
        },
    ]
}

fn network_live_capture_storage_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkRawCaptureStorageStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "CustodyReady",
            value: NetworkRawCaptureStorageStatusState::CustodyReady,
        },
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkRawCaptureStorageStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkRawCaptureStorageStatusState::Unavailable,
        },
        ProtocolLiteralDescriptor {
            key: "Degraded",
            value: NetworkRawCaptureStorageStatusState::Degraded,
        },
    ]
}

fn network_live_capture_execution_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkLiveCaptureExecutionStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkLiveCaptureExecutionStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "BoundedExecuted",
            value: NetworkLiveCaptureExecutionStatusState::BoundedExecuted,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkLiveCaptureExecutionStatusState::Unavailable,
        },
        ProtocolLiteralDescriptor {
            key: "Degraded",
            value: NetworkLiveCaptureExecutionStatusState::Degraded,
        },
    ]
}

fn network_linux_nftables_status_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<NetworkLinuxNftablesLabStatusState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ManualRequired",
            value: NetworkLinuxNftablesLabStatusState::ManualRequired,
        },
        ProtocolLiteralDescriptor {
            key: "ExecutedAndRolledBack",
            value: NetworkLinuxNftablesLabStatusState::ExecutedAndRolledBack,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: NetworkLinuxNftablesLabStatusState::Unavailable,
        },
    ]
}
