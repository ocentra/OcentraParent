package ca.ocentra.parent.agent;

import android.os.Bundle;

public final class ChildAndroidServiceProtocolProof {
    public static final String SCHEMA_VERSION = "child-android-service-protocol-capability-proof";
    public static final String PACKAGE_ID = "ca.ocentra.parent.agent";
    public static final String NATIVE_BRIDGE_CLASS =
        "ca.ocentra.parent.agent.ChildAndroidServiceProtocolProof";
    public static final String STORAGE_BRIDGE_CLASS =
        "ca.ocentra.parent.agent.ChildAndroidStorageProtocolProof";
    public static final String FOREGROUND_SERVICE =
        "ca.ocentra.parent.agent/.OcentraParentAgentService";
    public static final String NOTIFICATION_CHANNEL_ID = "ocentra_parent_agent";
    public static final int NOTIFICATION_ID = 4477;
    public static final String FOREGROUND_SERVICE_TYPE = "dataSync";
    public static final String FOREGROUND_SERVICE_STATUS = "declared-started-by-package";
    public static final String BRIDGE_STATE = "package-local-scaffold";
    public static final String EXTERNAL_TRANSPORT_STATE = "not-implemented";
    public static final String STATUS_EXPORT_STATE = "package-local-bundle";
    public static final String FIELD_FOREGROUND_SERVICE_STATUS = "foregroundServiceStatus";
    public static final String FIELD_STORAGE_BRIDGE_STATE = "storageBridgeState";
    public static final String COMMAND_SERVICE_STATUS_GET = "child.android.service.status.get";
    public static final String COMMAND_SERVICE_CAPABILITY_LABELS_GET =
        "child.android.service.capability.labels.get";
    public static final String COMMAND_SERVICE_STATUS_EXPORT_GET =
        "child.android.service.status.export.get";
    public static final String COMMAND_SERVICE_PROTOCOL_PROOF_GET =
        "child.android.service.protocol.proof.get";
    public static final String EVENT_SERVICE_STATUS_REPORTED =
        "child.android.service.status.reported";
    public static final String EVENT_SERVICE_CAPABILITY_LABELS_REPORTED =
        "child.android.service.capability.labels.reported";
    public static final String EVENT_SERVICE_STATUS_EXPORT_REPORTED =
        "child.android.service.status.export.reported";
    public static final String EVENT_SERVICE_PROTOCOL_PROOF_REPORTED =
        "child.android.service.protocol.proof.reported";

    private ChildAndroidServiceProtocolProof() {}

    public static Bundle createServiceProtocolBundle() {
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString("packageId", PACKAGE_ID);
        status.putString("nativeBridgeClass", NATIVE_BRIDGE_CLASS);
        status.putString("storageBridgeClass", STORAGE_BRIDGE_CLASS);
        status.putString("foregroundService", FOREGROUND_SERVICE);
        status.putString("notificationChannelId", NOTIFICATION_CHANNEL_ID);
        status.putInt("notificationId", NOTIFICATION_ID);
        status.putString("foregroundServiceType", FOREGROUND_SERVICE_TYPE);
        status.putString(FIELD_FOREGROUND_SERVICE_STATUS, FOREGROUND_SERVICE_STATUS);
        status.putString("bridgeState", BRIDGE_STATE);
        status.putString(FIELD_STORAGE_BRIDGE_STATE, ChildAndroidStorageProtocolProof.BRIDGE_STATE);
        status.putString("externalTransportState", EXTERNAL_TRANSPORT_STATE);
        status.putString("statusExportState", STATUS_EXPORT_STATE);
        status.putStringArray(
            "commands",
            new String[] {
                COMMAND_SERVICE_STATUS_GET,
                COMMAND_SERVICE_CAPABILITY_LABELS_GET,
                COMMAND_SERVICE_STATUS_EXPORT_GET,
                COMMAND_SERVICE_PROTOCOL_PROOF_GET
            }
        );
        status.putStringArray(
            "events",
            new String[] {
                EVENT_SERVICE_STATUS_REPORTED,
                EVENT_SERVICE_CAPABILITY_LABELS_REPORTED,
                EVENT_SERVICE_STATUS_EXPORT_REPORTED,
                EVENT_SERVICE_PROTOCOL_PROOF_REPORTED
            }
        );
        status.putStringArray(
            "serviceSurfaces",
            new String[] {
                "foreground-service-status",
                "storage-protocol-bridge",
                "status-export-surface",
                "usage-stats-capability-label",
                "accessibility-capability-label",
                "vpn-dns-capability-label",
                "device-owner-capability-label",
                "managed-profile-capability-label"
            }
        );
        status.putStringArray(
            "capabilityLabels",
            new String[] {
                "foreground-service-status=scaffold-only",
                "storage-protocol-bridge=scaffold-only",
                "status-export-surface=scaffold-only",
                "usage-stats-capability-label=permission-required",
                "accessibility-capability-label=unavailable",
                "vpn-dns-capability-label=unavailable",
                "device-owner-capability-label=blocked",
                "managed-profile-capability-label=blocked"
            }
        );
        status.putStringArray(
            "statusExportFields",
            new String[] {
                "schemaVersion",
                "packageId",
                "nativeBridgeClass",
                "foregroundServiceStatus",
                "storageBridgeState",
                "capabilityLabels",
                "commands",
                "events"
            }
        );
        status.putStringArray(
            "permissionRequiredCapabilityLabels",
            new String[] { "usage-stats-capability-label" }
        );
        status.putStringArray(
            "unavailableCapabilityLabels",
            new String[] {
                "accessibility-capability-label",
                "vpn-dns-capability-label"
            }
        );
        status.putStringArray(
            "blockedCapabilityLabels",
            new String[] {
                "device-owner-capability-label",
                "managed-profile-capability-label"
            }
        );
        return status;
    }
}
