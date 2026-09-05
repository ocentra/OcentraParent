package ca.ocentra.parent.agent;

import android.os.Bundle;

public final class AppGameAndroidUsageEventsCapabilityProof {
    public static final String SCHEMA_VERSION = "app-game-android-usage-events-capability-proof";
    public static final String PACKAGE_ID = "ca.ocentra.child.agent";
    public static final String NATIVE_BRIDGE_CLASS =
        "ca.ocentra.parent.agent.AppGameAndroidUsageEventsCapabilityProof";
    public static final String BRIDGE_STATE = "package-local-runtime";
    public static final String PERMISSION_STATE = "runtime-preflight-required";
    public static final String EVENT_COLLECTION_STATE = "count-only-runtime-preflight";
    public static final String REPLAY_CONSUMER_STATE = "child-runtime-service-consumer";
    public static final String FIELD_USAGE_EVENTS_BRIDGE_STATE = "usageEventsBridgeState";
    public static final String FIELD_CHILD_RUNTIME_SERVICE_CONSUMER_REACHABLE =
        "childRuntimeServiceConsumerReachable";
    public static final String COMMAND_USAGE_EVENTS_CAPABILITY_GET =
        "app-game.android.usage-events.capability.get";
    public static final String COMMAND_USAGE_EVENTS_REPLAY_BOUNDARY_GET =
        "app-game.android.usage-events.replay-boundary.get";
    public static final String EVENT_USAGE_EVENTS_CAPABILITY_REPORTED =
        "app-game.android.usage-events.capability.reported";
    public static final String EVENT_USAGE_EVENTS_REPLAY_BOUNDARY_REPORTED =
        "app-game.android.usage-events.replay-boundary.reported";

    private AppGameAndroidUsageEventsCapabilityProof() {}

    public static Bundle createUsageEventsCapabilityBundle() {
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString("packageId", PACKAGE_ID);
        status.putString("nativeBridgeClass", NATIVE_BRIDGE_CLASS);
        status.putString(FIELD_USAGE_EVENTS_BRIDGE_STATE, BRIDGE_STATE);
        status.putString("permissionState", PERMISSION_STATE);
        status.putString("eventCollectionState", EVENT_COLLECTION_STATE);
        status.putString("replayConsumerState", REPLAY_CONSUMER_STATE);
        status.putBoolean(FIELD_CHILD_RUNTIME_SERVICE_CONSUMER_REACHABLE, true);
        status.putStringArray(
            "commands",
            new String[] {
                COMMAND_USAGE_EVENTS_CAPABILITY_GET,
                COMMAND_USAGE_EVENTS_REPLAY_BOUNDARY_GET
            }
        );
        status.putStringArray(
            "events",
            new String[] {
                EVENT_USAGE_EVENTS_CAPABILITY_REPORTED,
                EVENT_USAGE_EVENTS_REPLAY_BOUNDARY_REPORTED
            }
        );
        status.putStringArray("proofRefs", new String[0]);
        status.putStringArray(
            "openGaps",
            new String[] {
                "android-usage-stats-settings-grant-not-proved",
                "android-usage-events-runtime-collection-not-proved",
                "android-usage-events-child-runtime-replay-proof-not-proved",
                "android-child-runtime-delivery-not-proved",
                "android-platform-enforcement-not-proved"
            }
        );
        status.putBoolean("rawUsageEventsStored", false);
        status.putBoolean("packageNamesStored", false);
        status.putBoolean("rawActivityRowsStored", false);
        status.putBoolean("adapterDispatchClaimed", false);
        status.putBoolean("platformEnforcementClaimed", false);
        status.putBoolean("childDeviceDeliveryClaimed", false);
        return status;
    }
}
