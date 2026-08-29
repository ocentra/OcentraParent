package ca.ocentra.parent.agent;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

import android.app.Application;
import android.content.ComponentName;
import android.content.Context;
import android.content.Intent;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.provider.Settings;
import android.view.accessibility.AccessibilityEvent;

import java.lang.reflect.Field;
import java.lang.reflect.Method;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;

import org.robolectric.Robolectric;
import org.robolectric.RuntimeEnvironment;
import org.robolectric.RobolectricTestRunner;

@RunWith(RobolectricTestRunner.class)
public final class AppGameAndroidAccessibilityRuntimeServiceTest {
    private static final String STATE_PREFERENCES = "app_game_android_accessibility_state";
    private static final String PREF_HAS_STATE = "hasState";
    private static final String PREF_EVENT_COUNT = "eventCount";
    private static final String PREF_LAST_OBSERVED_AT = "lastObservedAt";
    private static final String PREF_ENABLED_SERVICE_COUNT = "enabledServiceCount";
    private static final String PREF_SERVICE_ENABLED = "serviceEnabled";

    @Before
    public void resetRuntimeState() throws Exception {
        resetRuntimeState(true);
        setAccessibilitySettings(false, false);
    }

    @After
    public void stopPersistenceWorker() throws Exception {
        shutdownPersistence();
    }

    @Test
    public void nullContextReportsUnavailableWithoutRuntimeReadiness() {
        Bundle status = AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle(null);

        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.SETTINGS_READ_NO_CONTEXT,
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_SETTINGS_READ_STATE)
        );
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.RUNTIME_WAITING_FOR_ENABLEMENT,
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_RUNTIME_STATE)
        );
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.EVENT_SAMPLE_UNAVAILABLE,
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_STATE)
        );
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.PREFLIGHT_SETTINGS_UNAVAILABLE,
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_PREFLIGHT_STATE)
        );
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.DURABLE_STATE_NOT_AVAILABLE,
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_DURABLE_STATE)
        );
        assertEquals(0, status.getInt(AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_COUNT));
        assertEquals(0, status.getInt(AppGameAndroidAccessibilityRuntimeService.FIELD_ENABLED_SERVICE_COUNT));
        assertFalse(status.getBoolean(AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_ENABLED));
        assertFalse(status.getBoolean(AppGameAndroidAccessibilityRuntimeService.FIELD_DURABLE_RETRY_REQUIRED));
        assertEquals(0L, status.getLong(AppGameAndroidAccessibilityRuntimeService.FIELD_LAST_OBSERVED_AT));
        assertRedactedAndUnclaimed(status);
    }

    @Test
    public void globalAccessibilityOffBlocksListedService() {
        Application application = application();
        setAccessibilitySettings(false, true);

        Bundle status = AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle(application);

        assertEquals(1, status.getInt(AppGameAndroidAccessibilityRuntimeService.FIELD_ENABLED_SERVICE_COUNT));
        assertFalse(status.getBoolean(AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_ENABLED));
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.PREFLIGHT_SERVICE_DISABLED,
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_PREFLIGHT_STATE)
        );
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.EVENT_SAMPLE_WAITING_FOR_ENABLEMENT,
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_STATE)
        );
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.SETTINGS_READ_AVAILABLE,
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_SETTINGS_READ_STATE)
        );
        assertRedactedAndUnclaimed(status);
    }

    @Test
    public void enabledServiceListWithoutOwnComponentRemainsDisabled() {
        Application application = application();
        setAccessibilitySettings(true, false);

        Bundle status = AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle(application);

        assertEquals(1, status.getInt(AppGameAndroidAccessibilityRuntimeService.FIELD_ENABLED_SERVICE_COUNT));
        assertFalse(status.getBoolean(AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_ENABLED));
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.PREFLIGHT_SERVICE_DISABLED,
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_PREFLIGHT_STATE)
        );
        assertRedactedAndUnclaimed(status);
    }

    @Test
    public void enabledBoundServiceReportsRedactedWindowStateObservation() {
        Application application = application();
        setAccessibilitySettings(true, true);
        AppGameAndroidAccessibilityRuntimeService service = createService();
        service.onServiceConnected();

        Bundle beforeEvent = AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle(application);
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.RUNTIME_BOUND,
            beforeEvent.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_RUNTIME_STATE)
        );
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.PREFLIGHT_ENABLED_BOUND_NO_EVENTS,
            beforeEvent.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_PREFLIGHT_STATE)
        );
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.EVENT_SAMPLE_EMPTY,
            beforeEvent.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_STATE)
        );
        assertEquals(1, beforeEvent.getInt(AppGameAndroidAccessibilityRuntimeService.FIELD_ENABLED_SERVICE_COUNT));
        assertTrue(beforeEvent.getBoolean(AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_ENABLED));

        AccessibilityEvent event = AccessibilityEvent.obtain(AccessibilityEvent.TYPE_WINDOW_STATE_CHANGED);
        service.onAccessibilityEvent(event);
        event.recycle();

        Bundle afterEvent = AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle(application);
        assertEquals(1, afterEvent.getInt(AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_COUNT));
        assertEquals(1L, afterEvent.getLong(
            AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_COUNT_LONG
        ));
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.EVENT_SAMPLE_OBSERVED,
            afterEvent.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_STATE)
        );
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.PREFLIGHT_ENABLED_BOUND,
            afterEvent.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_PREFLIGHT_STATE)
        );
        assertTrue(afterEvent.getLong(AppGameAndroidAccessibilityRuntimeService.FIELD_LAST_OBSERVED_AT) > 0L);
        assertRedactedAndUnclaimed(afterEvent);
        service.onDestroy();
    }

    @Test
    public void onInterruptKeepsServiceBound() {
        Application application = application();
        setAccessibilitySettings(true, true);
        AppGameAndroidAccessibilityRuntimeService service = createService();
        service.onServiceConnected();

        service.onInterrupt();

        Bundle status = AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle(application);
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.RUNTIME_BOUND,
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_RUNTIME_STATE)
        );
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.PREFLIGHT_ENABLED_BOUND_NO_EVENTS,
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_PREFLIGHT_STATE)
        );
        assertTrue(status.getBoolean(AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_ENABLED));
        service.onDestroy();
    }

    @Test
    public void onUnbindAndDestroyClearConnectedState() {
        Application application = application();
        setAccessibilitySettings(true, true);
        AppGameAndroidAccessibilityRuntimeService service = createService();
        service.onServiceConnected();

        service.onUnbind(new Intent(application, AppGameAndroidAccessibilityRuntimeService.class));

        Bundle afterUnbind = AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle(application);
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.RUNTIME_WAITING_FOR_ENABLEMENT,
            afterUnbind.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_RUNTIME_STATE)
        );
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.PREFLIGHT_ENABLED_WAITING_FOR_BIND,
            afterUnbind.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_PREFLIGHT_STATE)
        );
        assertTrue(afterUnbind.getBoolean(AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_ENABLED));

        service.onDestroy();
        Bundle afterDestroy = AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle(application);
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.RUNTIME_WAITING_FOR_ENABLEMENT,
            afterDestroy.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_RUNTIME_STATE)
        );
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.PREFLIGHT_ENABLED_WAITING_FOR_BIND,
            afterDestroy.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_PREFLIGHT_STATE)
        );
    }

    @Test
    public void malformedDurableStateFailsClosedAndRequestsRepair() throws Exception {
        Application application = application();
        preferences(application).edit()
            .putBoolean(PREF_HAS_STATE, true)
            .putString(PREF_EVENT_COUNT, "not-a-long")
            .putLong(PREF_LAST_OBSERVED_AT, 100L)
            .putInt(PREF_ENABLED_SERVICE_COUNT, 1)
            .putBoolean(PREF_SERVICE_ENABLED, true)
            .commit();
        setAccessibilitySettings(true, true);
        holdStateVersionAtExhaustion();

        Bundle status = AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle(application);

        assertEquals(0L, status.getLong(AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_COUNT_LONG));
        assertEquals(0L, status.getLong(AppGameAndroidAccessibilityRuntimeService.FIELD_LAST_OBSERVED_AT));
        assertEquals(1, status.getInt(AppGameAndroidAccessibilityRuntimeService.FIELD_ENABLED_SERVICE_COUNT));
        assertTrue(status.getBoolean(AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_ENABLED));
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.DURABLE_STATE_WRITE_FAILED,
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_DURABLE_STATE)
        );
        assertTrue(status.getBoolean(AppGameAndroidAccessibilityRuntimeService.FIELD_DURABLE_RETRY_REQUIRED));
        assertRedactedAndUnclaimed(status);
    }

    @Test
    public void partialDurableStateIsNotAcceptedAsPersisted() throws Exception {
        Application application = application();
        preferences(application).edit()
            .putBoolean(PREF_HAS_STATE, true)
            .putLong(PREF_EVENT_COUNT, 9L)
            .putInt(PREF_ENABLED_SERVICE_COUNT, 1)
            .putBoolean(PREF_SERVICE_ENABLED, true)
            .commit();
        setAccessibilitySettings(true, true);
        holdStateVersionAtExhaustion();

        Bundle status = AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle(application);

        assertEquals(0L, status.getLong(AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_COUNT_LONG));
        assertEquals(0L, status.getLong(AppGameAndroidAccessibilityRuntimeService.FIELD_LAST_OBSERVED_AT));
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.DURABLE_STATE_WRITE_FAILED,
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_DURABLE_STATE)
        );
        assertTrue(status.getBoolean(AppGameAndroidAccessibilityRuntimeService.FIELD_DURABLE_RETRY_REQUIRED));
    }

    @Test
    public void futureTimestampDurableStateIsResetWithoutStaleObservation() throws Exception {
        Application application = application();
        preferences(application).edit()
            .putBoolean(PREF_HAS_STATE, true)
            .putLong(PREF_EVENT_COUNT, 2L)
            .putLong(PREF_LAST_OBSERVED_AT, System.currentTimeMillis() + 60_000L)
            .putInt(PREF_ENABLED_SERVICE_COUNT, 1)
            .putBoolean(PREF_SERVICE_ENABLED, true)
            .commit();
        setAccessibilitySettings(true, true);
        holdStateVersionAtExhaustion();

        Bundle status = AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle(application);

        assertEquals(0L, status.getLong(AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_COUNT_LONG));
        assertEquals(0L, status.getLong(AppGameAndroidAccessibilityRuntimeService.FIELD_LAST_OBSERVED_AT));
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.DURABLE_STATE_WRITE_FAILED,
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_DURABLE_STATE)
        );
        assertTrue(status.getBoolean(AppGameAndroidAccessibilityRuntimeService.FIELD_DURABLE_RETRY_REQUIRED));
    }

    @Test
    public void onDestroyFlushesDurableStateAndReadbackSurvivesStaticRestart() throws Exception {
        Application application = application();
        setAccessibilitySettings(true, true);
        AppGameAndroidAccessibilityRuntimeService service = createService();
        service.onServiceConnected();
        AccessibilityEvent event = AccessibilityEvent.obtain(AccessibilityEvent.TYPE_WINDOW_STATE_CHANGED);
        service.onAccessibilityEvent(event);
        event.recycle();

        service.onDestroy();

        SharedPreferences persisted = preferences(application);
        assertTrue(persisted.getBoolean(PREF_HAS_STATE, false));
        assertEquals(1L, persisted.getLong(PREF_EVENT_COUNT, 0L));
        assertTrue(persisted.getLong(PREF_LAST_OBSERVED_AT, 0L) > 0L);
        assertEquals(1, persisted.getInt(PREF_ENABLED_SERVICE_COUNT, 0));
        assertTrue(persisted.getBoolean(PREF_SERVICE_ENABLED, false));

        resetRuntimeState(false);
        setAccessibilitySettings(true, true);
        Bundle restored = AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle(application);
        assertEquals(1L, restored.getLong(AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_COUNT_LONG));
        assertTrue(restored.getLong(AppGameAndroidAccessibilityRuntimeService.FIELD_LAST_OBSERVED_AT) > 0L);
        assertEquals(
            AppGameAndroidAccessibilityRuntimeService.PREFLIGHT_ENABLED_WAITING_FOR_BIND,
            restored.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_PREFLIGHT_STATE)
        );
        assertTrue(restored.getBoolean(AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_ENABLED));
        assertRedactedAndUnclaimed(restored);
    }

    private static Application application() {
        return RuntimeEnvironment.getApplication();
    }

    private static SharedPreferences preferences(Application application) {
        return application.getSharedPreferences(STATE_PREFERENCES, Context.MODE_PRIVATE);
    }

    private static void setAccessibilitySettings(boolean globallyEnabled, boolean ownServiceListed) {
        Application application = application();
        Settings.Secure.putInt(
            application.getContentResolver(),
            Settings.Secure.ACCESSIBILITY_ENABLED,
            globallyEnabled ? 1 : 0
        );
        String enabledServices = ownServiceListed
            ? new ComponentName(application, AppGameAndroidAccessibilityRuntimeService.class).flattenToString()
            : new ComponentName("other.package", "other.Service").flattenToString();
        Settings.Secure.putString(
            application.getContentResolver(),
            Settings.Secure.ENABLED_ACCESSIBILITY_SERVICES,
            enabledServices
        );
    }

    private static AppGameAndroidAccessibilityRuntimeService createService() {
        return Robolectric.buildService(AppGameAndroidAccessibilityRuntimeService.class).create().get();
    }

    private static void assertRedactedAndUnclaimed(Bundle status) {
        assertEquals(
            "counts-only-no-service-names-or-event-content",
            status.getString(AppGameAndroidAccessibilityRuntimeService.FIELD_REDACTION_STATE)
        );
        assertFalse(status.getBoolean("rawAccessibilityServiceNamesStored"));
        assertFalse(status.getBoolean("rawAccessibilityEventsStored"));
        assertFalse(status.getBoolean("overlayExecutionClaimed"));
        assertFalse(status.getBoolean("adapterDispatchClaimed"));
        assertFalse(status.getBoolean("platformEnforcementClaimed"));
        assertFalse(status.getBoolean("childDeviceDeliveryClaimed"));
    }

    private static void holdStateVersionAtExhaustion() throws Exception {
        setStaticField("stateVersion", Long.MAX_VALUE);
    }

    private static void resetRuntimeState(boolean clearPreferences) throws Exception {
        shutdownPersistence();
        if (clearPreferences) {
            preferences(application()).edit().clear().commit();
        }
        setStaticField("processServiceConnected", false);
        setStaticField("observedWindowStateEventCount", 0L);
        setStaticField("lastObservedAt", 0L);
        setStaticField("enabledServiceCount", 0);
        setStaticField("serviceEnabled", false);
        setStaticField("settingsAvailable", false);
        setStaticField("stateLoaded", false);
        setStaticField(
            "settingsReadState",
            AppGameAndroidAccessibilityRuntimeService.SETTINGS_READ_NO_CONTEXT
        );
        setStaticField(
            "durableState",
            AppGameAndroidAccessibilityRuntimeService.DURABLE_STATE_NOT_AVAILABLE
        );
        setStaticField("stateVersion", 0L);
        setStaticField("persistedVersion", 0L);
        setStaticField("stateVersionExhausted", false);
        setStaticField("persistenceRetryRequired", false);
    }

    private static void setStaticField(String name, Object value) throws Exception {
        Field field = AppGameAndroidAccessibilityRuntimeService.class.getDeclaredField(name);
        field.setAccessible(true);
        field.set(null, value);
    }

    private static void shutdownPersistence() throws Exception {
        Field field = AppGameAndroidAccessibilityRuntimeService.class.getDeclaredField("PERSISTENCE");
        field.setAccessible(true);
        Object coordinator = field.get(null);
        Method shutdown = coordinator.getClass().getDeclaredMethod("shutdown");
        shutdown.setAccessible(true);
        shutdown.invoke(coordinator);
    }
}
