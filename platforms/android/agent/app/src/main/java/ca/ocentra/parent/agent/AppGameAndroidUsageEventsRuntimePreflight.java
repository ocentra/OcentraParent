package ca.ocentra.parent.agent;

import android.app.AppOpsManager;
import android.app.usage.UsageEvents;
import android.app.usage.UsageStatsManager;
import android.content.Context;
import android.os.Bundle;
import android.os.Process;

public final class AppGameAndroidUsageEventsRuntimePreflight {
    public static final String SCHEMA_VERSION = "app-game-android-usage-events-runtime-preflight";
    public static final String PACKAGE_ID = "ca.ocentra.child.agent";
    public static final String NATIVE_BRIDGE_CLASS =
        "ca.ocentra.parent.agent.AppGameAndroidUsageEventsRuntimePreflight";
    public static final String FIELD_PERMISSION_CHECK_STATE = "permissionCheckState";
    public static final String FIELD_RUNTIME_COLLECTION_STATE = "runtimeCollectionState";
    public static final String FIELD_SAMPLE_STATE = "sampleState";
    public static final String FIELD_SAMPLE_EVENT_COUNT = "sampleEventCount";
    public static final String FIELD_FOREGROUND_EVENT_COUNT = "foregroundEventCount";
    public static final String FIELD_SAMPLE_EVENT_COUNT_LONG = "sampleEventCountLong";
    public static final String FIELD_FOREGROUND_EVENT_COUNT_LONG = "foregroundEventCountLong";
    public static final String FIELD_DURABLE_REPLAY_STATE = "durableReplayState";
    public static final String FIELD_REPLAY_CURRENT = "replayCurrent";
    public static final String FIELD_REPLAY_SAMPLE_STATE = "replaySampleState";
    public static final String FIELD_REPLAY_EVENT_COUNT = "replayEventCount";
    public static final String FIELD_REPLAY_FOREGROUND_EVENT_COUNT = "replayForegroundEventCount";
    public static final String FIELD_REPLAY_OBSERVED_AT_EPOCH_MILLIS =
        "replayObservedAtEpochMillis";
    public static final String FIELD_REPLAY_GENERATION = "replayGeneration";
    public static final String PERMISSION_GRANTED = "usage-stats-granted";
    public static final String SETTINGS_GRANT_REQUIRED = "settings-grant-required";
    public static final String PERMISSION_CHECK_UNAVAILABLE = "permission-check-unavailable";
    public static final String COLLECTION_BLOCKED = "collection-blocked-before-runtime-proof";
    public static final String COLLECTION_READY_FOR_PROOF = "collection-ready-for-proof";
    public static final String SAMPLE_PERMISSION_REQUIRED = "sample-permission-required";
    public static final String SAMPLE_OBSERVED = "sample-observed";
    public static final String SAMPLE_EMPTY = "sample-empty";
    public static final String SAMPLE_UNAVAILABLE = "sample-unavailable";
    public static final String DURABLE_REPLAY_PERSISTED = "durable-replay-persisted";
    public static final String DURABLE_REPLAY_NOT_AVAILABLE = "durable-replay-not-available";
    public static final String DURABLE_REPLAY_WRITE_FAILED = "durable-replay-write-failed";
    public static final long DEFAULT_SAMPLE_LOOKBACK_MILLIS = 900000L;
    public static final String COMMAND_RUNTIME_PREFLIGHT_GET =
        "app-game.android.usage-events.runtime-preflight.get";
    public static final String EVENT_RUNTIME_PREFLIGHT_REPORTED =
        "app-game.android.usage-events.runtime-preflight.reported";

    private AppGameAndroidUsageEventsRuntimePreflight() {}

    public static Bundle createUnavailableRuntimePreflightBundle() {
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString("packageId", PACKAGE_ID);
        status.putString("nativeBridgeClass", NATIVE_BRIDGE_CLASS);
        status.putString(FIELD_PERMISSION_CHECK_STATE, PERMISSION_CHECK_UNAVAILABLE);
        status.putString(FIELD_RUNTIME_COLLECTION_STATE, COLLECTION_BLOCKED);
        status.putString("usageStatsServiceState", "service-unavailable");
        status.putString(FIELD_SAMPLE_STATE, SAMPLE_UNAVAILABLE);
        status.putLong("sampleLookbackMillis", DEFAULT_SAMPLE_LOOKBACK_MILLIS);
        status.putInt(FIELD_SAMPLE_EVENT_COUNT, 0);
        status.putInt(FIELD_FOREGROUND_EVENT_COUNT, 0);
        status.putLong(FIELD_SAMPLE_EVENT_COUNT_LONG, 0L);
        status.putLong(FIELD_FOREGROUND_EVENT_COUNT_LONG, 0L);
        status.putString(FIELD_DURABLE_REPLAY_STATE, DURABLE_REPLAY_NOT_AVAILABLE);
        status.putBoolean(FIELD_REPLAY_CURRENT, false);
        status.putString(FIELD_REPLAY_SAMPLE_STATE, SAMPLE_UNAVAILABLE);
        putReplayCounts(status, 0L, 0L, 0L, 0L);
        status.putStringArray("commands", new String[] { COMMAND_RUNTIME_PREFLIGHT_GET });
        status.putStringArray("events", new String[] { EVENT_RUNTIME_PREFLIGHT_REPORTED });
        status.putStringArray("proofRefs", new String[0]);
        status.putStringArray(
            "openGaps",
            new String[] {
                "android-usage-events-runtime-preflight-context-unavailable",
                "android-usage-events-child-runtime-replay-proof-not-proved",
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

    public static ChildRuntimeSource createChildRuntimeSource(Context context) {
        return new ChildRuntimeSource(createRuntimePreflightBundle(context));
    }

    public static Bundle createRuntimePreflightBundle(Context context) {
        String permissionState = usageStatsPermissionState(context);
        Bundle status = createBaseBundle(context, permissionState);
        CountOnlySample sample = collectCountOnlySample(context, permissionState);
        if (Thread.currentThread().isInterrupted()) {
            return createUnavailableRuntimePreflightBundle();
        }
        putSample(status, sample);
        AppGameAndroidUsageEventsReplayStore.Snapshot persisted = null;
        String durableState = DURABLE_REPLAY_NOT_AVAILABLE;
        if (sample.current) {
            if (Thread.currentThread().isInterrupted()) {
                return createUnavailableRuntimePreflightBundle();
            }
            long observedAt = System.currentTimeMillis();
            durableState = AppGameAndroidUsageEventsReplayStore.persist(
                context,
                sample.state,
                sample.eventCount,
                sample.foregroundEventCount,
                observedAt
            ) ?
                DURABLE_REPLAY_PERSISTED : DURABLE_REPLAY_WRITE_FAILED;
            if (DURABLE_REPLAY_PERSISTED.equals(durableState)) {
                persisted = AppGameAndroidUsageEventsReplayStore.read(context);
            }
        } else {
            persisted = AppGameAndroidUsageEventsReplayStore.read(context);
            if (persisted != null) {
                durableState = DURABLE_REPLAY_PERSISTED;
            }
        }
        putReplay(status, sample, persisted, durableState, sample.current);
        return status;
    }

    private static Bundle createBaseBundle(Context context, String permissionState) {
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString("packageId", context.getPackageName());
        status.putString("nativeBridgeClass", NATIVE_BRIDGE_CLASS);
        status.putString(FIELD_PERMISSION_CHECK_STATE, permissionState);
        status.putString(FIELD_RUNTIME_COLLECTION_STATE, runtimeCollectionState(permissionState));
        status.putString("usageStatsServiceState", usageStatsServiceState(context));
        status.putStringArray("commands", new String[] { COMMAND_RUNTIME_PREFLIGHT_GET });
        status.putStringArray("events", new String[] { EVENT_RUNTIME_PREFLIGHT_REPORTED });
        status.putStringArray("proofRefs", new String[0]);
        status.putStringArray(
            "openGaps",
            new String[] {
                "android-usage-events-child-runtime-replay-proof-not-proved",
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
        try {
            AppOpsManager appOpsManager = (AppOpsManager) appOpsService;
            int mode = appOpsManager.checkOpNoThrow(
                AppOpsManager.OPSTR_GET_USAGE_STATS,
                Process.myUid(),
                context.getPackageName()
            );
            return mode == AppOpsManager.MODE_ALLOWED ? PERMISSION_GRANTED : SETTINGS_GRANT_REQUIRED;
        } catch (RuntimeException error) {
            return PERMISSION_CHECK_UNAVAILABLE;
        }
    }

    private static String runtimeCollectionState(String permissionState) {
        return PERMISSION_GRANTED.equals(permissionState) ? COLLECTION_READY_FOR_PROOF : COLLECTION_BLOCKED;
    }

    private static String usageStatsServiceState(Context context) {
        Object service = context.getSystemService(Context.USAGE_STATS_SERVICE);
        return service instanceof UsageStatsManager ? "service-visible" : "service-unavailable";
    }

    private static CountOnlySample collectCountOnlySample(Context context, String permissionState) {
        if (!PERMISSION_GRANTED.equals(permissionState)) {
            return new CountOnlySample(SAMPLE_PERMISSION_REQUIRED, 0L, 0L, false);
        }
        Object service = context.getSystemService(Context.USAGE_STATS_SERVICE);
        if (!(service instanceof UsageStatsManager)) {
            return new CountOnlySample(SAMPLE_UNAVAILABLE, 0L, 0L, false);
        }
        return countUsageEvents((UsageStatsManager) service);
    }

    private static CountOnlySample countUsageEvents(UsageStatsManager usageStatsManager) {
        if (Thread.currentThread().isInterrupted()) {
            return new CountOnlySample(SAMPLE_UNAVAILABLE, 0L, 0L, false);
        }
        long endTime = System.currentTimeMillis();
        long startTime = endTime - DEFAULT_SAMPLE_LOOKBACK_MILLIS;
        final UsageEvents usageEvents;
        try {
            usageEvents = usageStatsManager.queryEvents(startTime, endTime);
        } catch (RuntimeException error) {
            return new CountOnlySample(SAMPLE_UNAVAILABLE, 0L, 0L, false);
        }
        if (usageEvents == null || Thread.currentThread().isInterrupted()) {
            return new CountOnlySample(SAMPLE_UNAVAILABLE, 0L, 0L, false);
        }
        UsageEvents.Event event = new UsageEvents.Event();
        long totalEventCount = 0L;
        long foregroundEventCount = 0L;
        while (usageEvents.hasNextEvent()) {
            if (Thread.currentThread().isInterrupted()) {
                return new CountOnlySample(SAMPLE_UNAVAILABLE, 0L, 0L, false);
            }
            usageEvents.getNextEvent(event);
            totalEventCount = incrementCount(totalEventCount);
            if (isForegroundEvent(event)) {
                foregroundEventCount = incrementCount(foregroundEventCount);
            }
        }
        String sampleState = totalEventCount > 0 ? SAMPLE_OBSERVED : SAMPLE_EMPTY;
        return new CountOnlySample(sampleState, totalEventCount, foregroundEventCount, true);
    }

    private static boolean isForegroundEvent(UsageEvents.Event event) {
        int eventType = event.getEventType();
        return eventType == UsageEvents.Event.MOVE_TO_FOREGROUND ||
            eventType == UsageEvents.Event.ACTIVITY_RESUMED;
    }

    private static void putSample(Bundle status, CountOnlySample sample) {
        status.putString(FIELD_SAMPLE_STATE, sample.state);
        status.putLong("sampleLookbackMillis", DEFAULT_SAMPLE_LOOKBACK_MILLIS);
        status.putInt(FIELD_SAMPLE_EVENT_COUNT, toIntCount(sample.eventCount));
        status.putInt(FIELD_FOREGROUND_EVENT_COUNT, toIntCount(sample.foregroundEventCount));
        status.putLong(FIELD_SAMPLE_EVENT_COUNT_LONG, sample.eventCount);
        status.putLong(FIELD_FOREGROUND_EVENT_COUNT_LONG, sample.foregroundEventCount);
    }

    private static void putReplay(
        Bundle status,
        CountOnlySample current,
        AppGameAndroidUsageEventsReplayStore.Snapshot persisted,
        String durableState,
        boolean currentSample
    ) {
        status.putString(FIELD_DURABLE_REPLAY_STATE, durableState);
        status.putBoolean(FIELD_REPLAY_CURRENT, currentSample);
        if (currentSample && persisted != null) {
            status.putString(FIELD_REPLAY_SAMPLE_STATE, persisted.sampleState);
            putReplayCounts(
                status,
                persisted.eventCount,
                persisted.foregroundEventCount,
                persisted.observedAtEpochMillis,
                persisted.generation
            );
        } else if (currentSample) {
            status.putString(FIELD_REPLAY_SAMPLE_STATE, current.state);
            putReplayCounts(status, current.eventCount, current.foregroundEventCount, System.currentTimeMillis(), 0L);
        } else if (persisted == null) {
            status.putString(FIELD_REPLAY_SAMPLE_STATE, current.state);
            putReplayCounts(status, 0L, 0L, 0L, 0L);
        } else {
            status.putString(FIELD_REPLAY_SAMPLE_STATE, persisted.sampleState);
            putReplayCounts(
                status,
                persisted.eventCount,
                persisted.foregroundEventCount,
                persisted.observedAtEpochMillis,
                persisted.generation
            );
        }
    }

    private static void putReplayCounts(
        Bundle status,
        long eventCount,
        long foregroundEventCount,
        long observedAtEpochMillis,
        long generation
    ) {
        status.putLong(FIELD_REPLAY_EVENT_COUNT, eventCount);
        status.putLong(FIELD_REPLAY_FOREGROUND_EVENT_COUNT, foregroundEventCount);
        status.putLong(FIELD_REPLAY_OBSERVED_AT_EPOCH_MILLIS, observedAtEpochMillis);
        status.putLong(FIELD_REPLAY_GENERATION, generation);
    }

    private static int toIntCount(long count) {
        return count > Integer.MAX_VALUE ? Integer.MAX_VALUE : (int) count;
    }

    private static long incrementCount(long count) {
        return count == Long.MAX_VALUE ? Long.MAX_VALUE : count + 1L;
    }

    private static final class CountOnlySample {
        final String state;
        final long eventCount;
        final long foregroundEventCount;
        final boolean current;

        CountOnlySample(String state, long eventCount, long foregroundEventCount, boolean current) {
            this.state = state;
            this.eventCount = eventCount;
            this.foregroundEventCount = foregroundEventCount;
            this.current = current;
        }
    }

    public static final class ChildRuntimeSource {
        private final String permissionState;
        private final String durableState;
        private final String sampleState;
        private final boolean current;
        private final long eventCount;
        private final long foregroundEventCount;
        private final long observedAtEpochMillis;
        private final long generation;
        private final Bundle diagnostics;

        private ChildRuntimeSource(Bundle source) {
            diagnostics = new Bundle(source);
            permissionState = source.getString(FIELD_PERMISSION_CHECK_STATE);
            durableState = source.getString(FIELD_DURABLE_REPLAY_STATE);
            sampleState = source.getString(FIELD_REPLAY_SAMPLE_STATE, SAMPLE_UNAVAILABLE);
            current = source.getBoolean(FIELD_REPLAY_CURRENT, false);
            eventCount = source.getLong(FIELD_REPLAY_EVENT_COUNT, -1L);
            foregroundEventCount = source.getLong(FIELD_REPLAY_FOREGROUND_EVENT_COUNT, -1L);
            observedAtEpochMillis = source.getLong(FIELD_REPLAY_OBSERVED_AT_EPOCH_MILLIS, 0L);
            generation = source.getLong(FIELD_REPLAY_GENERATION, 0L);
        }

        public Bundle diagnostics() {
            return new Bundle(diagnostics);
        }

        public String permissionState() {
            return permissionState;
        }

        public String durableState() {
            return durableState;
        }

        public String sampleState() {
            return sampleState;
        }

        public boolean current() {
            return current;
        }

        public long eventCount() {
            return eventCount;
        }

        public long foregroundEventCount() {
            return foregroundEventCount;
        }

        public long observedAtEpochMillis() {
            return observedAtEpochMillis;
        }

        public long generation() {
            return generation;
        }
    }

}
