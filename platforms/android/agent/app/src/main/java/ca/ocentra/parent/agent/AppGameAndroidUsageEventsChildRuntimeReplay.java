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
            try {
                boolean consumed = preferences.getBoolean(PREF_CONSUMED, false);
                StoredSnapshot existing = consumed ? readStoredRecord(preferences) : null;
                if (consumed && existing == null) {
                    result.putString(FIELD_CONSUMER_STATE, BLOCKED);
                    result.putString("blockReason", "child-runtime-replay-corrupt-durable-record");
                    return result;
                }
                if (!consumed && hasRecordFields(preferences)) {
                    result.putString(FIELD_CONSUMER_STATE, BLOCKED);
                    result.putString("blockReason", "child-runtime-replay-corrupt-durable-record");
                    return result;
                }
                if (existing != null && generation <= existing.sourceGeneration) {
                    result.putString(FIELD_CONSUMER_STATE, BLOCKED);
                    result.putString(
                        "blockReason",
                        existing.isCurrent(consumedAt)
                            ? "child-runtime-replay-generation-not-newer"
                            : "child-runtime-replay-stale-durable-record"
                    );
                    result.putLong(FIELD_SOURCE_GENERATION, existing.sourceGeneration);
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
                StoredSnapshot persisted = readStoredRecord(preferences);
                if (persisted == null || !persisted.matches(
                    source.sampleState(),
                    eventCount,
                    foregroundEventCount,
                    observedAt,
                    generation,
                    consumedAt
                )) {
                    result.putString(FIELD_CONSUMER_STATE, BLOCKED);
                    result.putString("blockReason", "child-runtime-replay-corrupt-durable-record");
                    return result;
                }
                if (!persisted.isCurrent(consumedAt)) {
                    result.putString(FIELD_CONSUMER_STATE, BLOCKED);
                    result.putString("blockReason", "child-runtime-replay-stale-durable-record");
                    return result;
                }
                putConsumedResult(result, persisted);
                return result;
            } catch (RuntimeException error) {
                result.putString(FIELD_CONSUMER_STATE, BLOCKED);
                result.putString("blockReason", "child-runtime-replay-corrupt-durable-record");
                return result;
            }
        }
    }

    public static Bundle read(Context context) {
        synchronized (REPLAY_LOCK) {
            if (context == null) {
                Bundle result = baseResult();
                result.putString(FIELD_CONSUMER_STATE, UNAVAILABLE);
                result.putString("blockReason", "child-runtime-replay-context-unavailable");
                return result;
            }
            Bundle result = baseResult();
            try {
                SharedPreferences preferences = context.getSharedPreferences(PREFERENCES, Context.MODE_PRIVATE);
                boolean consumed = preferences.getBoolean(PREF_CONSUMED, false);
                if (!consumed) {
                    if (hasRecordFields(preferences)) {
                        result.putString(FIELD_CONSUMER_STATE, BLOCKED);
                        result.putString("blockReason", "child-runtime-replay-corrupt-durable-record");
                    } else {
                        result.putString(FIELD_CONSUMER_STATE, UNAVAILABLE);
                    }
                    return result;
                }
                StoredSnapshot persisted = readStoredRecord(preferences);
                if (persisted == null) {
                    result.putString(FIELD_CONSUMER_STATE, BLOCKED);
                    result.putString("blockReason", "child-runtime-replay-corrupt-durable-record");
                    return result;
                }
                if (!persisted.isCurrent(System.currentTimeMillis())) {
                    result.putString(FIELD_CONSUMER_STATE, BLOCKED);
                    result.putString("blockReason", "child-runtime-replay-stale-durable-record");
                    return result;
                }
                putConsumedResult(result, persisted);
                return result;
            } catch (RuntimeException error) {
                result.putString(FIELD_CONSUMER_STATE, BLOCKED);
                result.putString("blockReason", "child-runtime-replay-corrupt-durable-record");
                return result;
            }
        }
    }

    private static void putConsumedResult(Bundle result, StoredSnapshot persisted) {
        result.putString(FIELD_CONSUMER_STATE, CONSUMED);
        result.putLong(FIELD_CONSUMED_AT_EPOCH_MILLIS, persisted.consumedAt);
        result.putLong(FIELD_SOURCE_GENERATION, persisted.sourceGeneration);
        result.putString(FIELD_SOURCE_SAMPLE_STATE, persisted.sourceState);
        result.putLong("eventCount", persisted.eventCount);
        result.putLong("foregroundEventCount", persisted.foregroundEventCount);
        result.putLong(
            AppGameAndroidUsageEventsRuntimePreflight.FIELD_REPLAY_OBSERVED_AT_EPOCH_MILLIS,
            persisted.observedAt
        );
    }

    private static StoredSnapshot readStoredRecord(SharedPreferences preferences) {
        if (!hasRecordFields(preferences)) {
            return null;
        }
        String sourceState = preferences.getString(PREF_SOURCE_STATE, null);
        long eventCount = preferences.getLong(PREF_EVENT_COUNT, Long.MIN_VALUE);
        long foregroundEventCount = preferences.getLong(
            PREF_FOREGROUND_EVENT_COUNT,
            Long.MIN_VALUE
        );
        long observedAt = preferences.getLong(PREF_OBSERVED_AT, Long.MIN_VALUE);
        long sourceGeneration = preferences.getLong(PREF_SOURCE_GENERATION, Long.MIN_VALUE);
        long consumedAt = preferences.getLong(PREF_CONSUMED_AT, Long.MIN_VALUE);
        long now = System.currentTimeMillis();
        if (!isValidSampleState(sourceState) || eventCount < 0L || foregroundEventCount < 0L ||
            foregroundEventCount > eventCount || observedAt <= 0L ||
            observedAt > now + 5000L || sourceGeneration <= 0L || consumedAt <= 0L ||
            consumedAt > now + 5000L || consumedAt < observedAt) {
            return null;
        }
        return new StoredSnapshot(
            sourceState,
            eventCount,
            foregroundEventCount,
            observedAt,
            sourceGeneration,
            consumedAt
        );
    }

    private static boolean hasRecordFields(SharedPreferences preferences) {
        return preferences.contains(PREF_SOURCE_STATE) ||
            preferences.contains(PREF_EVENT_COUNT) ||
            preferences.contains(PREF_FOREGROUND_EVENT_COUNT) ||
            preferences.contains(PREF_OBSERVED_AT) ||
            preferences.contains(PREF_SOURCE_GENERATION) ||
            preferences.contains(PREF_CONSUMED_AT);
    }

    private static boolean isValidSampleState(String sampleState) {
        return AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_OBSERVED.equals(sampleState) ||
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_EMPTY.equals(sampleState);
    }

    private static final class StoredSnapshot {
        final String sourceState;
        final long eventCount;
        final long foregroundEventCount;
        final long observedAt;
        final long sourceGeneration;
        final long consumedAt;

        StoredSnapshot(
            String sourceState,
            long eventCount,
            long foregroundEventCount,
            long observedAt,
            long sourceGeneration,
            long consumedAt
        ) {
            this.sourceState = sourceState;
            this.eventCount = eventCount;
            this.foregroundEventCount = foregroundEventCount;
            this.observedAt = observedAt;
            this.sourceGeneration = sourceGeneration;
            this.consumedAt = consumedAt;
        }

        boolean matches(
            String expectedSourceState,
            long expectedEventCount,
            long expectedForegroundEventCount,
            long expectedObservedAt,
            long expectedGeneration,
            long expectedConsumedAt
        ) {
            return sourceState.equals(expectedSourceState) &&
                eventCount == expectedEventCount &&
                foregroundEventCount == expectedForegroundEventCount &&
                observedAt == expectedObservedAt &&
                sourceGeneration == expectedGeneration &&
                consumedAt == expectedConsumedAt;
        }

        boolean isCurrent(long now) {
            return observedAt <= now &&
                now - observedAt <=
                    AppGameAndroidUsageEventsRuntimePreflight.DEFAULT_SAMPLE_LOOKBACK_MILLIS &&
                consumedAt <= now &&
                now - consumedAt <=
                    AppGameAndroidUsageEventsRuntimePreflight.DEFAULT_SAMPLE_LOOKBACK_MILLIS;
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
