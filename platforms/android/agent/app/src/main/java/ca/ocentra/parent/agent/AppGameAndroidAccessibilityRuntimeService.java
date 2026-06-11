package ca.ocentra.parent.agent;

import android.accessibilityservice.AccessibilityService;
import android.os.Bundle;
import android.view.accessibility.AccessibilityEvent;

public final class AppGameAndroidAccessibilityRuntimeService extends AccessibilityService {
    public static final String FIELD_SERVICE_DECLARATION_STATE = "accessibility-service-declared";
    public static final String FIELD_SERVICE_RUNTIME_STATE = "accessibility-runtime-state";
    public static final String FIELD_EVENT_SAMPLE_STATE = "accessibility-event-sample-state";
    public static final String FIELD_EVENT_SAMPLE_COUNT = "accessibility-event-sample-count";
    public static final String SERVICE_DECLARED = "accessibility-service-declared";
    public static final String RUNTIME_WAITING_FOR_ENABLEMENT = "accessibility-runtime-waiting-for-enablement";
    public static final String RUNTIME_BOUND = "accessibility-runtime-bound";
    public static final String EVENT_SAMPLE_WAITING_FOR_ENABLEMENT = "accessibility-event-sample-waiting-for-enablement";
    public static final String EVENT_SAMPLE_OBSERVED = "accessibility-event-sample-observed";
    public static final String EVENT_SAMPLE_EMPTY = "accessibility-event-sample-empty";
    private static int observedWindowStateEventCount = 0;
    private static boolean serviceConnected = false;

    public static Bundle createAccessibilityRuntimeBundle() {
        Bundle bundle = new Bundle();
        bundle.putString(FIELD_SERVICE_DECLARATION_STATE, SERVICE_DECLARED);
        bundle.putString(
            FIELD_SERVICE_RUNTIME_STATE,
            serviceConnected ? RUNTIME_BOUND : RUNTIME_WAITING_FOR_ENABLEMENT
        );
        bundle.putString(FIELD_EVENT_SAMPLE_STATE, eventSampleState());
        bundle.putInt(FIELD_EVENT_SAMPLE_COUNT, observedWindowStateEventCount);
        return bundle;
    }

    @Override
    protected void onServiceConnected() {
        super.onServiceConnected();
        serviceConnected = true;
    }

    @Override
    public void onAccessibilityEvent(AccessibilityEvent event) {
        if (event != null && event.getEventType() == AccessibilityEvent.TYPE_WINDOW_STATE_CHANGED) {
            observedWindowStateEventCount += 1;
        }
    }

    @Override
    public void onInterrupt() {
        serviceConnected = false;
    }

    private static String eventSampleState() {
        if (!serviceConnected) {
            return EVENT_SAMPLE_WAITING_FOR_ENABLEMENT;
        }
        return observedWindowStateEventCount > 0 ? EVENT_SAMPLE_OBSERVED : EVENT_SAMPLE_EMPTY;
    }
}
