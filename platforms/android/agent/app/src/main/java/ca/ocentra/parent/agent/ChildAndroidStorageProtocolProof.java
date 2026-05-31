package ca.ocentra.parent.agent;

import android.os.Bundle;

public final class ChildAndroidStorageProtocolProof {
    public static final String SCHEMA_VERSION = "child-android-storage-protocol-capability-proof";
    public static final String PACKAGE_ID = "ca.ocentra.parent.agent";
    public static final String NATIVE_BRIDGE_CLASS =
        "ca.ocentra.parent.agent.ChildAndroidStorageProtocolProof";
    public static final String COMMAND_STORAGE_SNAPSHOT_GET = "child.android.storage.snapshot.get";
    public static final String COMMAND_STORAGE_CAPABILITY_PROOF_GET =
        "child.android.storage.capability.proof.get";
    public static final String COMMAND_STORAGE_PROTOCOL_PROOF_GET =
        "child.android.storage.protocol.proof.get";
    public static final String EVENT_STORAGE_SNAPSHOT_REPORTED =
        "child.android.storage.snapshot.reported";
    public static final String EVENT_STORAGE_CAPABILITY_PROOF_REPORTED =
        "child.android.storage.capability.proof.reported";
    public static final String EVENT_STORAGE_PROTOCOL_PROOF_REPORTED =
        "child.android.storage.protocol.proof.reported";
    public static final String BRIDGE_STATE = "package-local-scaffold";
    public static final String EXTERNAL_TRANSPORT_STATE = "not-implemented";
    public static final String APP_PRIVATE_FILES_STATE = "package-local-scaffold";
    public static final String ENCRYPTED_EVIDENCE_JOURNAL_STATE = "not-implemented";
    public static final String SQLITE_QUERY_STORE_STATE = "not-implemented";
    public static final String PARENT_OWNED_EXPORT_STATE = "planned";
    public static final String OCENTRA_HOSTED_CHILD_ACTIVITY_STORAGE_STATE = "not-default";
    public static final String FIELD_STORAGE_BRIDGE_STATE = "storageBridgeState";

    private ChildAndroidStorageProtocolProof() {}

    public static Bundle createStorageProtocolBundle() {
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString("packageId", PACKAGE_ID);
        status.putString("nativeBridgeClass", NATIVE_BRIDGE_CLASS);
        status.putString(FIELD_STORAGE_BRIDGE_STATE, BRIDGE_STATE);
        status.putString("externalTransportState", EXTERNAL_TRANSPORT_STATE);
        status.putStringArray(
            "commands",
            new String[] {
                COMMAND_STORAGE_SNAPSHOT_GET,
                COMMAND_STORAGE_CAPABILITY_PROOF_GET,
                COMMAND_STORAGE_PROTOCOL_PROOF_GET
            }
        );
        status.putStringArray(
            "events",
            new String[] {
                EVENT_STORAGE_SNAPSHOT_REPORTED,
                EVENT_STORAGE_CAPABILITY_PROOF_REPORTED,
                EVENT_STORAGE_PROTOCOL_PROOF_REPORTED
            }
        );
        status.putStringArray(
            "storageSurfaces",
            new String[] {
                "app-private-files",
                "encrypted-evidence-journal",
                "sqlite-query-store",
                "parent-owned-export",
                "ocentra-hosted-child-activity-storage",
                "protocol-storage-snapshot"
            }
        );
        status.putStringArray(
            "notDefaultStorageSurfaces",
            new String[] { "ocentra-hosted-child-activity-storage" }
        );
        status.putStringArray(
            "manualRequiredStorageSurfaces",
            new String[] {
                "encrypted-evidence-journal",
                "sqlite-query-store",
                "parent-owned-export"
            }
        );
        return status;
    }
}
