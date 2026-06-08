package ca.ocentra.parent.agent;

import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.content.SharedPreferences;
import android.location.LocationManager;
import android.util.Log;

public final class TrackingAndroidGeofenceTransitionReceiver extends BroadcastReceiver {
    public static final String ACTION_TRACKING_GEOFENCE_TRANSITION =
        "ca.ocentra.parent.agent.TRACKING_GEOFENCE_TRANSITION_PROOF";
    public static final String PREFS_NAME = "tracking_geofence_transition_proof";
    public static final String FIELD_TRANSITION_COUNT = "transitionCount";
    public static final String FIELD_ENTER_COUNT = "enterCount";
    public static final String FIELD_EXIT_COUNT = "exitCount";
    public static final String FIELD_LAST_TRANSITION = "lastTransition";
    public static final String FIELD_LAST_TRANSITION_EPOCH_MILLIS = "lastTransitionEpochMillis";
    public static final String FIELD_REGISTERED = "registered";
    public static final String FIELD_REGISTRATION_EPOCH_MILLIS = "registrationEpochMillis";
    public static final String FIELD_SOURCE = "source";
    public static final String FIELD_HAS_INSIDE_STATE = "hasInsideState";
    public static final String FIELD_INSIDE_STATE = "insideState";
    public static final String FIELD_SYSTEM_PROXIMITY_REGISTERED = "systemProximityRegistered";
    public static final String FIELD_SYSTEM_PROXIMITY_REGISTRATION_EPOCH_MILLIS =
        "systemProximityRegistrationEpochMillis";
    public static final String FIELD_SYSTEM_PROXIMITY_REGISTRATION_SOURCE = "systemProximityRegistrationSource";
    public static final String FIELD_SYSTEM_PROXIMITY_TRANSITION_COUNT = "systemProximityTransitionCount";
    public static final String FIELD_SYSTEM_PROXIMITY_ENTER_COUNT = "systemProximityEnterCount";
    public static final String FIELD_SYSTEM_PROXIMITY_EXIT_COUNT = "systemProximityExitCount";
    public static final String FIELD_SYSTEM_PROXIMITY_LAST_TRANSITION = "systemProximityLastTransition";
    public static final String FIELD_SYSTEM_PROXIMITY_LAST_TRANSITION_EPOCH_MILLIS =
        "systemProximityLastTransitionEpochMillis";
    public static final String FIELD_DWELL_COUNT = "dwellCount";
    public static final String FIELD_DWELL_LAST_OBSERVED_EPOCH_MILLIS = "dwellLastObservedEpochMillis";
    public static final String FIELD_DWELL_INSIDE_STARTED_EPOCH_MILLIS = "dwellInsideStartedEpochMillis";
    public static final String FIELD_DWELL_SOURCE = "dwellSource";
    public static final String TRANSITION_ENTER = "enter";
    public static final String TRANSITION_EXIT = "exit";
    public static final String TRANSITION_DWELL = "dwell";
    public static final String SOURCE_ANDROID_PROXIMITY_ALERT = "android-location-manager-proximity-alert";
    public static final String SOURCE_ANDROID_LOCATION_LISTENER_LOCAL_GEOFENCE =
        "android-location-manager-gps-listener-local-geofence";
    public static final String SOURCE_ANDROID_LOCATION_LISTENER_LOCAL_DWELL =
        "android-location-manager-gps-listener-local-dwell";
    private static final String LOG_TAG = "TrackingGeofenceProof";

    @Override
    public void onReceive(Context context, Intent intent) {
        if (!ACTION_TRACKING_GEOFENCE_TRANSITION.equals(intent.getAction())) {
            return;
        }
        boolean entering = intent.getBooleanExtra(LocationManager.KEY_PROXIMITY_ENTERING, false);
        String transition = entering ? TRANSITION_ENTER : TRANSITION_EXIT;
        long observedAtEpochMillis = System.currentTimeMillis();
        SharedPreferences prefs = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE);
        int transitionCount = prefs.getInt(FIELD_SYSTEM_PROXIMITY_TRANSITION_COUNT, 0) + 1;
        int enterCount = prefs.getInt(FIELD_SYSTEM_PROXIMITY_ENTER_COUNT, 0) + (entering ? 1 : 0);
        int exitCount = prefs.getInt(FIELD_SYSTEM_PROXIMITY_EXIT_COUNT, 0) + (entering ? 0 : 1);
        prefs.edit()
            .putBoolean(FIELD_REGISTERED, true)
            .putBoolean(FIELD_SYSTEM_PROXIMITY_REGISTERED, true)
            .putString(FIELD_SOURCE, SOURCE_ANDROID_PROXIMITY_ALERT)
            .putInt(FIELD_SYSTEM_PROXIMITY_TRANSITION_COUNT, transitionCount)
            .putInt(FIELD_SYSTEM_PROXIMITY_ENTER_COUNT, enterCount)
            .putInt(FIELD_SYSTEM_PROXIMITY_EXIT_COUNT, exitCount)
            .putString(FIELD_SYSTEM_PROXIMITY_LAST_TRANSITION, transition)
            .putLong(FIELD_SYSTEM_PROXIMITY_LAST_TRANSITION_EPOCH_MILLIS, observedAtEpochMillis)
            .apply();
        Log.i(
            LOG_TAG,
            "tracking-geofence-transition transition=" +
            transition +
            " transitionCount=" +
            transitionCount +
            " enterCount=" +
            enterCount +
            " exitCount=" +
            exitCount
        );
    }
}
