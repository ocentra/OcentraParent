package ca.ocentra.parent.agent;

import android.app.AppOpsManager;
import android.app.usage.UsageEvents;
import android.app.usage.UsageStatsManager;
import android.content.Context;
import android.os.Bundle;
import android.os.Process;

public final class AppGameAndroidUsageEventsRuntimePreflight {
    public static final String SCHEMA_VERSION = "app-game-android-usage-events-runtime-preflight";
    public static final String PACKAGE_ID = "ca.ocentra.parent.agent";
    public static final String NATIVE_BRIDGE_CLASS =
        "ca.ocentra.parent.agent.AppGameAndroidUsageEventsRuntimePreflight";
    public static final String FIELD_PERMISSION_CHECK_STATE = "permissionCheckState";
    public static final String FIELD_RUNTIME_COLLECTION_STATE = "runtimeCollectionState";
    public static final String FIELD_SAMPLE_STATE = "sampleState";
    public static final String FIELD_SAMPLE_EVENT_COUNT = "sampleEventCount";
    public static final String FIELD_FOREGROUND_EVENT_COUNT = "foregroundEventCount";
    public static final String PERMISSION_GRANTED = "usage-stats-granted";
    public static final String SETTINGS_GRANT_REQUIRED = "settings-grant-required";
    public static final String PERMISSION_CHECK_UNAVAILABLE = "permission-check-unavailable";
    public static final String COLLECTION_BLOCKED = "collection-blocked-before-runtime-proof";
    public static final String COLLECTION_READY_FOR_PROOF = "collection-ready-for-proof";
    public static final String SAMPLE_PERMISSION_REQUIRED = "sample-permission-required";
    public static final String SAMPLE_OBSERVED = "sample-observed";
    public static final String SAMPLE_EMPTY = "sample-empty";
    public static final String SAMPLE_UNAVAILABLE = "sample-unavailable";
    public static final long DEFAULT_SAMPLE_LOOKBACK_MILLIS = 900000L;
    public static final String COMMAND_RUNTIME_PREFLIGHT_GET =
        "app-game.android.usage-events.runtime-preflight.get";
    public static final String EVENT_RUNTIME_PREFLIGHT_REPORTED =
        "app-game.android.usage-events.runtime-preflight.reported";

    private AppGameAndroidUsageEventsRuntimePreflight() {}

    public static Bundle createRuntimePreflightBundle(Context context) {
        String permissionState = usageStatsPermissionState(context);
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString("packageId", PACKAGE_ID);
        status.putString("nativeBridgeClass", NATIVE_BRIDGE_CLASS);
        status.putString(FIELD_PERMISSION_CHECK_STATE, permissionState);
        status.putString(FIELD_RUNTIME_COLLECTION_STATE, runtimeCollectionState(permissionState));
        status.putString("usageStatsServiceState", usageStatsServiceState(context));
        putCountOnlySample(status, context, permissionState);
        status.putStringArray("commands", new String[] { COMMAND_RUNTIME_PREFLIGHT_GET });
        status.putStringArray("events", new String[] { EVENT_RUNTIME_PREFLIGHT_REPORTED });
        status.putStringArray(
            "proofRefs",
            new String[] {
                "android-usage-events-runtime-preflight-ref",
                "android-usage-stats-appops-preflight-ref"
            }
        );
        status.putStringArray(
            "openGaps",
            new String[] {
                "android-usage-events-runtime-sample-not-proved",
                "android-child-runtime-delivery-not-proved",
                "android-platform-enforcement-not-proved"
            }
        );
        status.putBoolean("rawUsageEventsStored", false);
        status.putBoolean("packageNamesStored", false);
        status.putBoolean("rawActivityRowsStored", false);
        status.putBoolean("runtimeCollectionClaimed", false);
        status.putBoolean("adapterDispatchClaimed", false);
        status.putBoolean("platformEnforcementClaimed", false);
        status.putBoolean("childDeviceDeliveryClaimed", false);
        return status;
    }

    private static String usageStatsPermissionState(Context context) {
        Object appOpsService = context.getSystemService(Context.APP_OPS_SERVICE);
        if (!(appOpsService instanceof AppOpsManager)) {
            return PERMISSION_CHECK_UNAVAILABLE;
        }
        AppOpsManager appOpsManager = (AppOpsManager) appOpsService;
        int mode = appOpsManager.checkOpNoThrow(
            AppOpsManager.OPSTR_GET_USAGE_STATS,
            Process.myUid(),
            context.getPackageName()
        );
        return mode == AppOpsManager.MODE_ALLOWED ? PERMISSION_GRANTED : SETTINGS_GRANT_REQUIRED;
    }

    private static String runtimeCollectionState(String permissionState) {
        return PERMISSION_GRANTED.equals(permissionState) ? COLLECTION_READY_FOR_PROOF : COLLECTION_BLOCKED;
    }

    private static String usageStatsServiceState(Context context) {
        Object service = context.getSystemService(Context.USAGE_STATS_SERVICE);
        return service instanceof UsageStatsManager ? "service-visible" : "service-unavailable";
    }

    private static void putCountOnlySample(Bundle status, Context context, String permissionState) {
        if (!PERMISSION_GRANTED.equals(permissionState)) {
            putSample(status, SAMPLE_PERMISSION_REQUIRED, 0, 0);
            return;
        }
        Object service = context.getSystemService(Context.USAGE_STATS_SERVICE);
        if (!(service instanceof UsageStatsManager)) {
            putSample(status, SAMPLE_UNAVAILABLE, 0, 0);
            return;
        }
        CountOnlySample sample = countUsageEvents((UsageStatsManager) service);
        putSample(status, sample.state, sample.totalEventCount, sample.foregroundEventCount);
    }

    private static CountOnlySample countUsageEvents(UsageStatsManager usageStatsManager) {
        long endTime = System.currentTimeMillis();
        long startTime = endTime - DEFAULT_SAMPLE_LOOKBACK_MILLIS;
        UsageEvents usageEvents = usageStatsManager.queryEvents(startTime, endTime);
        if (usageEvents == null) {
            return new CountOnlySample(SAMPLE_UNAVAILABLE, 0, 0);
        }
        UsageEvents.Event event = new UsageEvents.Event();
        int totalEventCount = 0;
        int foregroundEventCount = 0;
        while (usageEvents.hasNextEvent()) {
            usageEvents.getNextEvent(event);
            totalEventCount += 1;
            if (isForegroundEvent(event)) {
                foregroundEventCount += 1;
            }
        }
        String sampleState = totalEventCount > 0 ? SAMPLE_OBSERVED : SAMPLE_EMPTY;
        return new CountOnlySample(sampleState, totalEventCount, foregroundEventCount);
    }

    private static boolean isForegroundEvent(UsageEvents.Event event) {
        int eventType = event.getEventType();
        return eventType == UsageEvents.Event.MOVE_TO_FOREGROUND ||
            eventType == UsageEvents.Event.ACTIVITY_RESUMED;
    }

    private static void putSample(Bundle status, String sampleState, int totalEventCount, int foregroundEventCount) {
        status.putString(FIELD_SAMPLE_STATE, sampleState);
        status.putLong("sampleLookbackMillis", DEFAULT_SAMPLE_LOOKBACK_MILLIS);
        status.putInt(FIELD_SAMPLE_EVENT_COUNT, totalEventCount);
        status.putInt(FIELD_FOREGROUND_EVENT_COUNT, foregroundEventCount);
    }

    private static final class CountOnlySample {
        final String state;
        final int totalEventCount;
        final int foregroundEventCount;

        CountOnlySample(String state, int totalEventCount, int foregroundEventCount) {
            this.state = state;
            this.totalEventCount = totalEventCount;
            this.foregroundEventCount = foregroundEventCount;
        }
    }
}
