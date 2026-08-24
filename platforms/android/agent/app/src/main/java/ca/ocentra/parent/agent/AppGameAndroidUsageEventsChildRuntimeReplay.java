package ca.ocentra.parent.agent;

import android.content.Context;
import android.content.SharedPreferences;
import android.os.Bundle;

public final class AppGameAndroidUsageEventsChildRuntimeReplay {
    public static final String SCHEMA_VERSION =
        "app-game-android-usage-events-child-runtime-replay";
    public static final String FIELD_CONSUMER_STATE = "consumerState";
    public static final String FIELD_CONSUMED_AT_EPOCH_MILLIS = "consumedAtEpochMillis";
    public static final String FIELD_SOURCE_GENERATION = "sourceGeneration";
    public static final String FIELD_SOURCE_SAMPLE_STATE = "sourceSampleState";
    public static final String CONSUMED = "child-runtime-replay-consumed";
    public static final String BLOCKED = "child-runtime-replay-blocked";
    public static final String UNAVAILABLE = "child-runtime-replay-unavailable";
    public static final String WRITE_FAILED = "child-runtime-replay-write-failed";
    public static final String REPLAY_CUSTODY_SCOPE = "app-private-same-process";

    private static final Object REPLAY_LOCK = new Object();
    private static final String PREFERENCES = "app_game_android_child_runtime_replay";
    private static final String PREF_CONSUMED = "consumed";
    private static final String PREF_SOURCE_STATE = "sourceState";
    private static final String PREF_EVENT_COUNT = "eventCount";
    private static final String PREF_FOREGROUND_EVENT_COUNT = "foregroundEventCount";
    private static final String PREF_OBSERVED_AT = "observedAtEpochMillis";
    private static final String PREF_SOURCE_GENERATION = "sourceGeneration";
    private static final String PREF_CONSUMED_AT = "consumedAtEpochMillis";

    private AppGameAndroidUsageEventsChildRuntimeReplay() {}

    public static Bundle consume(
        Context context,
        AppGameAndroidUsageEventsRuntimePreflight.ChildRuntimeSource source
    ) {
        Bundle result = baseResult();
        if (!isAcceptableSource(source)) {
            result.putString(FIELD_CONSUMER_STATE, BLOCKED);
            result.putString("blockReason", "owner-produced-count-only-source-not-ready");
            return result;
        }
        if (context == null) {
            result.putString(FIELD_CONSUMER_STATE, UNAVAILABLE);
            result.putString("blockReason", "child-runtime-replay-context-unavailable");
            return result;
        }

        long eventCount = source.eventCount();
        long foregroundEventCount = source.foregroundEventCount();
        long observedAt = source.observedAtEpochMillis();
        long generation = source.generation();
        long consumedAt = System.currentTimeMillis();
        synchronized (REPLAY_LOCK) {
            SharedPreferences preferences = context.getSharedPreferences(PREFERENCES, Context.MODE_PRIVATE);
            long consumedGeneration = preferences.getLong(PREF_SOURCE_GENERATION, 0L);
            if (preferences.getBoolean(PREF_CONSUMED, false) && generation <= consumedGeneration) {
                result.putString(FIELD_CONSUMER_STATE, BLOCKED);
                result.putString("blockReason", "child-runtime-replay-generation-not-newer");
                result.putLong(FIELD_SOURCE_GENERATION, consumedGeneration);
                return result;
            }
            boolean written = preferences.edit()
                .putBoolean(PREF_CONSUMED, true)
                .putString(PREF_SOURCE_STATE, source.sampleState())
                .putLong(PREF_EVENT_COUNT, eventCount)
                .putLong(PREF_FOREGROUND_EVENT_COUNT, foregroundEventCount)
                .putLong(PREF_OBSERVED_AT, observedAt)
                .putLong(PREF_SOURCE_GENERATION, generation)
                .putLong(PREF_CONSUMED_AT, consumedAt)
                .commit();
            if (!written) {
                result.putString(FIELD_CONSUMER_STATE, WRITE_FAILED);
                result.putString("blockReason", "child-runtime-replay-durable-write-failed");
                return result;
            }
        }
        result.putString(FIELD_CONSUMER_STATE, CONSUMED);
        result.putLong(FIELD_CONSUMED_AT_EPOCH_MILLIS, consumedAt);
        result.putLong(FIELD_SOURCE_GENERATION, generation);
        result.putString(FIELD_SOURCE_SAMPLE_STATE, source.sampleState());
        result.putLong("eventCount", eventCount);
        result.putLong("foregroundEventCount", foregroundEventCount);
        result.putLong(
            AppGameAndroidUsageEventsRuntimePreflight.FIELD_REPLAY_OBSERVED_AT_EPOCH_MILLIS,
            observedAt
        );
        return result;
    }

    public static Bundle read(Context context) {
        synchronized (REPLAY_LOCK) {
            if (context == null) {
                Bundle result = baseResult();
                result.putString(FIELD_CONSUMER_STATE, UNAVAILABLE);
                result.putString("blockReason", "child-runtime-replay-context-unavailable");
                return result;
            }
            SharedPreferences preferences = context.getSharedPreferences(PREFERENCES, Context.MODE_PRIVATE);
            if (!preferences.getBoolean(PREF_CONSUMED, false)) {
                Bundle result = baseResult();
                result.putString(FIELD_CONSUMER_STATE, UNAVAILABLE);
                return result;
            }
            Bundle result = baseResult();
            result.putString(FIELD_CONSUMER_STATE, CONSUMED);
            result.putString(FIELD_SOURCE_SAMPLE_STATE, preferences.getString(PREF_SOURCE_STATE, "unknown"));
            result.putLong(FIELD_CONSUMED_AT_EPOCH_MILLIS, preferences.getLong(PREF_CONSUMED_AT, 0L));
            result.putLong(FIELD_SOURCE_GENERATION, preferences.getLong(PREF_SOURCE_GENERATION, 0L));
            result.putLong("eventCount", preferences.getLong(PREF_EVENT_COUNT, 0L));
            result.putLong("foregroundEventCount", preferences.getLong(PREF_FOREGROUND_EVENT_COUNT, 0L));
            result.putLong(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_REPLAY_OBSERVED_AT_EPOCH_MILLIS,
                preferences.getLong(PREF_OBSERVED_AT, 0L)
            );
            return result;
        }
    }

    private static boolean isAcceptableSource(
        AppGameAndroidUsageEventsRuntimePreflight.ChildRuntimeSource source
    ) {
        if (source == null || !source.current() ||
            !AppGameAndroidUsageEventsRuntimePreflight.PERMISSION_GRANTED.equals(
                source.permissionState()
            ) ||
            !AppGameAndroidUsageEventsRuntimePreflight.DURABLE_REPLAY_PERSISTED.equals(
                source.durableState()
            )) {
            return false;
        }
        long now = System.currentTimeMillis();
        long observedAt = source.observedAtEpochMillis();
        return source.eventCount() >= 0L &&
            source.foregroundEventCount() >= 0L &&
            source.foregroundEventCount() <= source.eventCount() &&
            source.generation() > 0L &&
            observedAt > 0L &&
            observedAt <= now + 5000L &&
            now - observedAt <= AppGameAndroidUsageEventsRuntimePreflight.DEFAULT_SAMPLE_LOOKBACK_MILLIS;
    }

    private static Bundle baseResult() {
        Bundle result = new Bundle();
        result.putString("schemaVersion", SCHEMA_VERSION);
        result.putString("sourceKind", "android-usage-events-count-only");
        result.putString("replayCustodyScope", REPLAY_CUSTODY_SCOPE);
        result.putBoolean("rawUsageEventsStored", false);
        result.putBoolean("packageNamesStored", false);
        result.putBoolean("rawActivityRowsStored", false);
        result.putBoolean("childDeviceDeliveryClaimed", false);
        result.putBoolean("adapterDispatchClaimed", false);
        result.putBoolean("platformEnforcementClaimed", false);
        result.putStringArray(
            "openGaps",
            new String[] {
                "android-child-device-delivery-not-proved",
                "android-usage-events-replay-test-not-written",
                "android-platform-enforcement-not-proved",
                "android-cross-process-replay-custody-not-proved"
            }
        );
        return result;
    }
}
