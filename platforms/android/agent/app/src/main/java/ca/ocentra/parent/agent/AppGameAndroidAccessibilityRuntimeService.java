package ca.ocentra.parent.agent;

import android.accessibilityservice.AccessibilityService;
import android.content.ComponentName;
import android.content.Context;
import android.content.Intent;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.provider.Settings;
import android.view.accessibility.AccessibilityEvent;

public final class AppGameAndroidAccessibilityRuntimeService extends AccessibilityService {
    public static final String FIELD_SERVICE_DECLARATION_STATE = "accessibility-service-declared";
    public static final String FIELD_SERVICE_RUNTIME_STATE = "accessibility-runtime-state";
    public static final String FIELD_EVENT_SAMPLE_STATE = "accessibility-event-sample-state";
    public static final String FIELD_EVENT_SAMPLE_COUNT = "accessibility-event-sample-count";
    public static final String FIELD_EVENT_SAMPLE_COUNT_LONG = "accessibility-event-sample-count-long";
    public static final String FIELD_ENABLED_SERVICE_COUNT = "accessibility-enabled-service-count";
    public static final String FIELD_SERVICE_ENABLED = "accessibility-service-enabled";
    public static final String FIELD_PREFLIGHT_STATE = "accessibility-preflight-state";
    public static final String FIELD_DURABLE_STATE = "accessibility-durable-state";
    public static final String FIELD_SETTINGS_READ_STATE = "accessibility-settings-read-state";
    public static final String FIELD_LAST_OBSERVED_AT = "accessibility-last-observed-at-epoch-millis";
    public static final String FIELD_REDACTION_STATE = "accessibility-redaction-state";
    public static final String SERVICE_DECLARED = "accessibility-service-declared";
    public static final String RUNTIME_WAITING_FOR_ENABLEMENT = "accessibility-runtime-waiting-for-enablement";
    public static final String RUNTIME_BOUND = "accessibility-runtime-bound";
    public static final String EVENT_SAMPLE_WAITING_FOR_ENABLEMENT = "accessibility-event-sample-waiting-for-enablement";
    public static final String EVENT_SAMPLE_OBSERVED = "accessibility-event-sample-observed";
    public static final String EVENT_SAMPLE_EMPTY = "accessibility-event-sample-empty";
    public static final String EVENT_SAMPLE_UNAVAILABLE = "accessibility-event-sample-unavailable";
    public static final String PREFLIGHT_SETTINGS_UNAVAILABLE = "accessibility-settings-unavailable";
    public static final String PREFLIGHT_SERVICE_DISABLED = "accessibility-service-disabled";
    public static final String PREFLIGHT_ENABLED_WAITING_FOR_BIND = "accessibility-enabled-waiting-for-bind";
    public static final String PREFLIGHT_ENABLED_BOUND = "accessibility-enabled-and-bound";
    public static final String PREFLIGHT_ENABLED_BOUND_NO_EVENTS = "accessibility-enabled-bound-no-events";
    public static final String SETTINGS_READ_AVAILABLE = "accessibility-settings-read";
    public static final String SETTINGS_READ_FAILED = "accessibility-settings-read-failed";
    public static final String SETTINGS_READ_NO_CONTEXT = "accessibility-settings-read-no-context";
    public static final String DURABLE_STATE_PERSISTED = "accessibility-state-persisted";
    public static final String DURABLE_STATE_NOT_AVAILABLE = "accessibility-state-not-available";
    public static final String DURABLE_STATE_WRITE_FAILED = "accessibility-state-write-failed";

    private static final Object STATE_LOCK = new Object();
    private static final String STATE_PREFERENCES = "app_game_android_accessibility_state";
    private static final String PREF_HAS_STATE = "hasState";
    private static final String PREF_EVENT_COUNT = "eventCount";
    private static final String PREF_LAST_OBSERVED_AT = "lastObservedAt";
    private static final String PREF_ENABLED_SERVICE_COUNT = "enabledServiceCount";
    private static final String PREF_SERVICE_ENABLED = "serviceEnabled";
    private static boolean processServiceConnected;
    private static long observedWindowStateEventCount;
    private static long lastObservedAt;
    private static int enabledServiceCount;
    private static boolean serviceEnabled;
    private static boolean settingsAvailable;
    private static boolean stateLoaded;
    private static String settingsReadState = SETTINGS_READ_NO_CONTEXT;
    private static String durableState = DURABLE_STATE_NOT_AVAILABLE;

    public static Bundle createAccessibilityRuntimeBundle() {
        synchronized (STATE_LOCK) {
            return createBundle(false);
        }
    }

    public static Bundle createAccessibilityRuntimeBundle(Context context) {
        synchronized (STATE_LOCK) {
            loadPersistedState(context);
            refreshEnabledState(context);
            return createBundle(true);
        }
    }

    @Override
    protected void onServiceConnected() {
        super.onServiceConnected();
        synchronized (STATE_LOCK) {
            processServiceConnected = true;
            loadPersistedState(getApplicationContext());
            refreshEnabledState(getApplicationContext());
            persistState(getApplicationContext());
        }
    }

    @Override
    public void onAccessibilityEvent(AccessibilityEvent event) {
        if (event == null || event.getEventType() != AccessibilityEvent.TYPE_WINDOW_STATE_CHANGED) {
            return;
        }
        synchronized (STATE_LOCK) {
            observedWindowStateEventCount = incrementCount(observedWindowStateEventCount);
            lastObservedAt = System.currentTimeMillis();
            persistState(getApplicationContext());
        }
    }

    @Override
    public void onInterrupt() {
        synchronized (STATE_LOCK) {
            processServiceConnected = false;
            persistState(getApplicationContext());
        }
    }

    @Override
    public boolean onUnbind(Intent intent) {
        synchronized (STATE_LOCK) {
            processServiceConnected = false;
            persistState(getApplicationContext());
        }
        return super.onUnbind(intent);
    }

    private static Bundle createBundle(boolean contextBound) {
        Bundle bundle = new Bundle();
        bundle.putString(FIELD_SERVICE_DECLARATION_STATE, SERVICE_DECLARED);
        bundle.putString(
            FIELD_SERVICE_RUNTIME_STATE,
            contextBound && processServiceConnected ? RUNTIME_BOUND : RUNTIME_WAITING_FOR_ENABLEMENT
        );
        bundle.putString(
            FIELD_EVENT_SAMPLE_STATE,
            contextBound ? eventSampleState() : EVENT_SAMPLE_UNAVAILABLE
        );
        bundle.putInt(
            FIELD_EVENT_SAMPLE_COUNT,
            contextBound ? toIntCount(observedWindowStateEventCount) : 0
        );
        bundle.putLong(
            FIELD_EVENT_SAMPLE_COUNT_LONG,
            contextBound ? observedWindowStateEventCount : 0L
        );
        bundle.putInt(FIELD_ENABLED_SERVICE_COUNT, contextBound ? enabledServiceCount : 0);
        bundle.putBoolean(FIELD_SERVICE_ENABLED, contextBound && serviceEnabled);
        bundle.putString(
            FIELD_PREFLIGHT_STATE,
            contextBound ? preflightState() : PREFLIGHT_SETTINGS_UNAVAILABLE
        );
        bundle.putString(
            FIELD_DURABLE_STATE,
            contextBound ? durableState : DURABLE_STATE_NOT_AVAILABLE
        );
        bundle.putString(
            FIELD_SETTINGS_READ_STATE,
            contextBound ? settingsReadState : SETTINGS_READ_NO_CONTEXT
        );
        bundle.putLong(FIELD_LAST_OBSERVED_AT, contextBound ? lastObservedAt : 0L);
        bundle.putString(FIELD_REDACTION_STATE, "counts-only-no-service-names-or-event-content");
        bundle.putBoolean("rawAccessibilityServiceNamesStored", false);
        bundle.putBoolean("rawAccessibilityEventsStored", false);
        bundle.putBoolean("overlayExecutionClaimed", false);
        bundle.putBoolean("adapterDispatchClaimed", false);
        bundle.putBoolean("platformEnforcementClaimed", false);
        bundle.putBoolean("childDeviceDeliveryClaimed", false);
        return bundle;
    }

    private static String eventSampleState() {
        if (!settingsAvailable) {
            return EVENT_SAMPLE_UNAVAILABLE;
        }
        if (!serviceEnabled || !processServiceConnected) {
            return EVENT_SAMPLE_WAITING_FOR_ENABLEMENT;
        }
        return observedWindowStateEventCount > 0 ? EVENT_SAMPLE_OBSERVED : EVENT_SAMPLE_EMPTY;
    }

    private static String preflightState() {
        if (!settingsAvailable) {
            return PREFLIGHT_SETTINGS_UNAVAILABLE;
        }
        if (!serviceEnabled) {
            return PREFLIGHT_SERVICE_DISABLED;
        }
        if (!processServiceConnected) {
            return PREFLIGHT_ENABLED_WAITING_FOR_BIND;
        }
        return observedWindowStateEventCount > 0 ? PREFLIGHT_ENABLED_BOUND : PREFLIGHT_ENABLED_BOUND_NO_EVENTS;
    }

    private static void loadPersistedState(Context context) {
        if (stateLoaded || context == null) {
            return;
        }
        SharedPreferences preferences = context.getSharedPreferences(STATE_PREFERENCES, Context.MODE_PRIVATE);
        if (!preferences.getBoolean(PREF_HAS_STATE, false)) {
            stateLoaded = true;
            durableState = DURABLE_STATE_NOT_AVAILABLE;
            return;
        }
        observedWindowStateEventCount = Math.max(0L, preferences.getLong(PREF_EVENT_COUNT, 0L));
        lastObservedAt = Math.max(0L, preferences.getLong(PREF_LAST_OBSERVED_AT, 0L));
        enabledServiceCount = Math.max(0, preferences.getInt(PREF_ENABLED_SERVICE_COUNT, 0));
        serviceEnabled = preferences.getBoolean(PREF_SERVICE_ENABLED, false);
        durableState = DURABLE_STATE_PERSISTED;
        stateLoaded = true;
    }

    private static void refreshEnabledState(Context context) {
        if (context == null) {
            settingsAvailable = false;
            settingsReadState = SETTINGS_READ_NO_CONTEXT;
            return;
        }
        try {
            String enabledServices = Settings.Secure.getString(
                context.getContentResolver(),
                Settings.Secure.ENABLED_ACCESSIBILITY_SERVICES
            );
            ComponentName ownService = new ComponentName(
                context,
                AppGameAndroidAccessibilityRuntimeService.class
            );
            enabledServiceCount = 0;
            serviceEnabled = false;
            if (enabledServices != null && !enabledServices.isEmpty()) {
                for (String component : enabledServices.split(":")) {
                    if (component == null || component.isEmpty()) {
                        continue;
                    }
                    ComponentName enabledComponent = ComponentName.unflattenFromString(component);
                    if (enabledComponent == null) {
                        continue;
                    }
                    enabledServiceCount += 1;
                    serviceEnabled |= ownService.equals(enabledComponent);
                }
            }
            settingsAvailable = true;
            settingsReadState = SETTINGS_READ_AVAILABLE;
            persistState(context);
        } catch (RuntimeException error) {
            settingsAvailable = false;
            settingsReadState = SETTINGS_READ_FAILED;
        }
    }

    private static void persistState(Context context) {
        if (context == null) {
            durableState = DURABLE_STATE_NOT_AVAILABLE;
            return;
        }
        boolean written = context.getSharedPreferences(STATE_PREFERENCES, Context.MODE_PRIVATE)
            .edit()
            .putBoolean(PREF_HAS_STATE, true)
            .putLong(PREF_EVENT_COUNT, observedWindowStateEventCount)
            .putLong(PREF_LAST_OBSERVED_AT, lastObservedAt)
            .putInt(PREF_ENABLED_SERVICE_COUNT, enabledServiceCount)
            .putBoolean(PREF_SERVICE_ENABLED, serviceEnabled)
            .commit();
        durableState = written ? DURABLE_STATE_PERSISTED : DURABLE_STATE_WRITE_FAILED;
    }

    private static long incrementCount(long count) {
        return count == Long.MAX_VALUE ? Long.MAX_VALUE : count + 1L;
    }

    private static int toIntCount(long count) {
        return count > Integer.MAX_VALUE ? Integer.MAX_VALUE : (int) count;
    }
}
