package ca.ocentra.parent.agent;

import android.Manifest;
import android.app.PendingIntent;
import android.content.Context;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.location.Location;
import android.location.LocationListener;
import android.location.LocationManager;
import android.os.Bundle;
import android.os.Looper;

public final class TrackingAndroidBackgroundLocationProof {
    public static final String SCHEMA_VERSION = "tracking-android-background-location-proof";
    public static final String FIELD_BACKGROUND_LOCATION_PERMISSION_STATE = "backgroundLocationPermissionState";
    public static final String FIELD_BACKGROUND_GEOFENCE_STATE = "backgroundGeofenceState";
    public static final String FIELD_BACKGROUND_GEOFENCE_TRANSITION_COUNT = "backgroundGeofenceTransitionCount";
    public static final String FIELD_BACKGROUND_GEOFENCE_ENTER_COUNT = "backgroundGeofenceEnterCount";
    public static final String FIELD_BACKGROUND_GEOFENCE_EXIT_COUNT = "backgroundGeofenceExitCount";
    public static final String FIELD_BACKGROUND_GEOFENCE_DWELL_COUNT = "backgroundGeofenceDwellCount";
    public static final String FIELD_BACKGROUND_GEOFENCE_DWELL_SOURCE = "backgroundGeofenceDwellSource";
    public static final String FIELD_BACKGROUND_GEOFENCE_LAST_TRANSITION = "backgroundGeofenceLastTransition";
    public static final String FIELD_BACKGROUND_GEOFENCE_SOURCE = "backgroundGeofenceSource";
    public static final String BACKGROUND_LOCATION_PERMISSION_GRANTED = "background-location-permission-granted";
    public static final String BACKGROUND_LOCATION_PERMISSION_REQUIRED = "background-location-permission-required";
    public static final String BACKGROUND_GEOFENCE_MANUAL_REQUIRED = "background-geofence-transition-manual-required";
    public static final String BACKGROUND_GEOFENCE_TRANSITION_OBSERVED =
        "background-geofence-transition-observed-emulator";
    public static final String BACKGROUND_LOCATION_PROOF_BOUNDARY =
        "emulator-background-permission-and-proximity-transition-proof-no-product-claim";
    private static final double EMULATOR_GEOFENCE_LATITUDE = 37.422;
    private static final double EMULATOR_GEOFENCE_LONGITUDE = -122.084;
    private static final float EMULATOR_GEOFENCE_RADIUS_METERS = 120.0f;
    private static final long EMULATOR_LOCAL_DWELL_THRESHOLD_MILLIS = 4_000L;
    private static final long EMULATOR_GEOFENCE_EXPIRATION_MILLIS = 60_000L;
    private static LocationListener emulatorProofListener;

    private TrackingAndroidBackgroundLocationProof() {}

    public static boolean hasBackgroundLocationPermission(Context context) {
        return (
            context.getPackageManager()
                .checkPermission(Manifest.permission.ACCESS_BACKGROUND_LOCATION, context.getPackageName()) ==
                PackageManager.PERMISSION_GRANTED
        );
    }

    public static void registerEmulatorGeofenceProof(Context context) {
        if (!hasBackgroundLocationPermission(context)) {
            return;
        }
        LocationManager locationManager = (LocationManager) context.getSystemService(Context.LOCATION_SERVICE);
        Intent intent = new Intent(context, TrackingAndroidGeofenceTransitionReceiver.class);
        intent.setAction(TrackingAndroidGeofenceTransitionReceiver.ACTION_TRACKING_GEOFENCE_TRANSITION);
        PendingIntent transitionIntent = PendingIntent.getBroadcast(
            context,
            0,
            intent,
            PendingIntent.FLAG_UPDATE_CURRENT | PendingIntent.FLAG_MUTABLE
        );
        try {
            locationManager.addProximityAlert(
                EMULATOR_GEOFENCE_LATITUDE,
                EMULATOR_GEOFENCE_LONGITUDE,
                EMULATOR_GEOFENCE_RADIUS_METERS,
                EMULATOR_GEOFENCE_EXPIRATION_MILLIS,
                transitionIntent
            );
            android.content.SharedPreferences prefs = context.getSharedPreferences(
                TrackingAndroidGeofenceTransitionReceiver.PREFS_NAME,
                Context.MODE_PRIVATE
            );
            long registrationEpochMillis = System.currentTimeMillis();
            android.content.SharedPreferences.Editor editor = prefs.edit()
                .putBoolean(TrackingAndroidGeofenceTransitionReceiver.FIELD_REGISTERED, true)
                .putBoolean(TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_REGISTERED, true)
                .putLong(
                    TrackingAndroidGeofenceTransitionReceiver.FIELD_REGISTRATION_EPOCH_MILLIS,
                    registrationEpochMillis
                )
                .putLong(
                    TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_REGISTRATION_EPOCH_MILLIS,
                    registrationEpochMillis
                )
                .putString(
                    TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_REGISTRATION_SOURCE,
                    TrackingAndroidGeofenceTransitionReceiver.SOURCE_ANDROID_PROXIMITY_ALERT
                );
            if (prefs.getInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_TRANSITION_COUNT, 0) == 0) {
                editor.putString(
                    TrackingAndroidGeofenceTransitionReceiver.FIELD_SOURCE,
                    TrackingAndroidGeofenceTransitionReceiver.SOURCE_ANDROID_PROXIMITY_ALERT
                );
            }
            editor.apply();
        } catch (SecurityException ignored) {
            // The proof bundle below keeps the geofence state manual-required when permission is unavailable.
        }
        registerEmulatorLocationListenerProof(context, locationManager);
    }

    private static void registerEmulatorLocationListenerProof(Context context, LocationManager locationManager) {
        if (emulatorProofListener != null) {
            return;
        }
        Context appContext = context.getApplicationContext();
        emulatorProofListener = location -> recordLocalGeofenceObservation(appContext, location);
        try {
            locationManager.requestLocationUpdates(
                LocationManager.GPS_PROVIDER,
                0L,
                0.0f,
                emulatorProofListener,
                Looper.getMainLooper()
            );
        } catch (SecurityException ignored) {
            emulatorProofListener = null;
        } catch (IllegalArgumentException ignored) {
            emulatorProofListener = null;
        }
    }

    private static void recordLocalGeofenceObservation(Context context, Location location) {
        float[] distanceMeters = new float[1];
        Location.distanceBetween(
            location.getLatitude(),
            location.getLongitude(),
            EMULATOR_GEOFENCE_LATITUDE,
            EMULATOR_GEOFENCE_LONGITUDE,
            distanceMeters
        );
        boolean inside = distanceMeters[0] <= EMULATOR_GEOFENCE_RADIUS_METERS;
        android.content.SharedPreferences prefs = context.getSharedPreferences(
            TrackingAndroidGeofenceTransitionReceiver.PREFS_NAME,
            Context.MODE_PRIVATE
        );
        boolean hasInsideState = prefs.getBoolean(TrackingAndroidGeofenceTransitionReceiver.FIELD_HAS_INSIDE_STATE, false);
        boolean previousInside = prefs.getBoolean(TrackingAndroidGeofenceTransitionReceiver.FIELD_INSIDE_STATE, inside);
        long observedAtEpochMillis = System.currentTimeMillis();
        long insideStartedEpochMillis = prefs.getLong(
            TrackingAndroidGeofenceTransitionReceiver.FIELD_DWELL_INSIDE_STARTED_EPOCH_MILLIS,
            0L
        );
        android.content.SharedPreferences.Editor editor = prefs.edit()
            .putBoolean(TrackingAndroidGeofenceTransitionReceiver.FIELD_REGISTERED, true)
            .putString(
                TrackingAndroidGeofenceTransitionReceiver.FIELD_SOURCE,
                TrackingAndroidGeofenceTransitionReceiver.SOURCE_ANDROID_LOCATION_LISTENER_LOCAL_GEOFENCE
            )
            .putBoolean(TrackingAndroidGeofenceTransitionReceiver.FIELD_HAS_INSIDE_STATE, true)
            .putBoolean(TrackingAndroidGeofenceTransitionReceiver.FIELD_INSIDE_STATE, inside);
        if (inside && (!hasInsideState || !previousInside || insideStartedEpochMillis == 0L)) {
            insideStartedEpochMillis = observedAtEpochMillis;
            editor.putLong(
                TrackingAndroidGeofenceTransitionReceiver.FIELD_DWELL_INSIDE_STARTED_EPOCH_MILLIS,
                insideStartedEpochMillis
            );
        }
        if (!inside) {
            insideStartedEpochMillis = 0L;
            editor.putLong(TrackingAndroidGeofenceTransitionReceiver.FIELD_DWELL_INSIDE_STARTED_EPOCH_MILLIS, 0L);
        }
        if (hasInsideState && previousInside != inside) {
            boolean entering = inside;
            String transition = entering
                ? TrackingAndroidGeofenceTransitionReceiver.TRANSITION_ENTER
                : TrackingAndroidGeofenceTransitionReceiver.TRANSITION_EXIT;
            int transitionCount = prefs.getInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_TRANSITION_COUNT, 0) + 1;
            int enterCount = prefs.getInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_ENTER_COUNT, 0) +
                (entering ? 1 : 0);
            int exitCount = prefs.getInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_EXIT_COUNT, 0) +
                (entering ? 0 : 1);
            editor
                .putInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_TRANSITION_COUNT, transitionCount)
                .putInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_ENTER_COUNT, enterCount)
                .putInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_EXIT_COUNT, exitCount)
                .putString(TrackingAndroidGeofenceTransitionReceiver.FIELD_LAST_TRANSITION, transition)
                .putLong(
                    TrackingAndroidGeofenceTransitionReceiver.FIELD_LAST_TRANSITION_EPOCH_MILLIS,
                    observedAtEpochMillis
                );
        }
        if (
            inside &&
            insideStartedEpochMillis > 0L &&
            observedAtEpochMillis - insideStartedEpochMillis >= EMULATOR_LOCAL_DWELL_THRESHOLD_MILLIS
        ) {
            int dwellCount = prefs.getInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_DWELL_COUNT, 0) + 1;
            editor
                .putInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_DWELL_COUNT, dwellCount)
                .putString(
                    TrackingAndroidGeofenceTransitionReceiver.FIELD_DWELL_SOURCE,
                    TrackingAndroidGeofenceTransitionReceiver.SOURCE_ANDROID_LOCATION_LISTENER_LOCAL_DWELL
                )
                .putString(
                    TrackingAndroidGeofenceTransitionReceiver.FIELD_LAST_TRANSITION,
                    TrackingAndroidGeofenceTransitionReceiver.TRANSITION_DWELL
                )
                .putLong(
                    TrackingAndroidGeofenceTransitionReceiver.FIELD_DWELL_LAST_OBSERVED_EPOCH_MILLIS,
                    observedAtEpochMillis
                );
        }
        editor.apply();
    }

    public static Bundle createBackgroundLocationBundle(Context context) {
        boolean permissionGranted = hasBackgroundLocationPermission(context);
        android.content.SharedPreferences prefs = context.getSharedPreferences(
            TrackingAndroidGeofenceTransitionReceiver.PREFS_NAME,
            Context.MODE_PRIVATE
        );
        int transitionCount = prefs.getInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_TRANSITION_COUNT, 0);
        int systemProximityTransitionCount = prefs.getInt(
            TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_TRANSITION_COUNT,
            0
        );
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString(
            FIELD_BACKGROUND_LOCATION_PERMISSION_STATE,
            permissionGranted ? BACKGROUND_LOCATION_PERMISSION_GRANTED : BACKGROUND_LOCATION_PERMISSION_REQUIRED
        );
        status.putString(
            FIELD_BACKGROUND_GEOFENCE_STATE,
            transitionCount > 0 || systemProximityTransitionCount > 0
                ? BACKGROUND_GEOFENCE_TRANSITION_OBSERVED
                : BACKGROUND_GEOFENCE_MANUAL_REQUIRED
        );
        status.putString("proofBoundary", BACKGROUND_LOCATION_PROOF_BOUNDARY);
        status.putBoolean("backgroundLocationPermissionGranted", permissionGranted);
        status.putBoolean("backgroundGeofenceTransitionCaptured", transitionCount > 0 || systemProximityTransitionCount > 0);
        status.putInt(FIELD_BACKGROUND_GEOFENCE_TRANSITION_COUNT, transitionCount);
        status.putInt(
            FIELD_BACKGROUND_GEOFENCE_ENTER_COUNT,
            prefs.getInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_ENTER_COUNT, 0)
        );
        status.putInt(
            FIELD_BACKGROUND_GEOFENCE_EXIT_COUNT,
            prefs.getInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_EXIT_COUNT, 0)
        );
        status.putInt(
            FIELD_BACKGROUND_GEOFENCE_DWELL_COUNT,
            prefs.getInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_DWELL_COUNT, 0)
        );
        status.putString(
            FIELD_BACKGROUND_GEOFENCE_DWELL_SOURCE,
            prefs.getString(TrackingAndroidGeofenceTransitionReceiver.FIELD_DWELL_SOURCE, "not-observed")
        );
        status.putString(
            FIELD_BACKGROUND_GEOFENCE_LAST_TRANSITION,
            prefs.getString(TrackingAndroidGeofenceTransitionReceiver.FIELD_LAST_TRANSITION, "none")
        );
        status.putString(
            FIELD_BACKGROUND_GEOFENCE_SOURCE,
            prefs.getString(TrackingAndroidGeofenceTransitionReceiver.FIELD_SOURCE, "not-registered")
        );
        status.putBoolean(
            TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_REGISTERED,
            prefs.getBoolean(TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_REGISTERED, false)
        );
        status.putLong(
            TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_REGISTRATION_EPOCH_MILLIS,
            prefs.getLong(
                TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_REGISTRATION_EPOCH_MILLIS,
                0L
            )
        );
        status.putString(
            TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_REGISTRATION_SOURCE,
            prefs.getString(
                TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_REGISTRATION_SOURCE,
                "not-registered"
            )
        );
        status.putInt(
            TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_TRANSITION_COUNT,
            systemProximityTransitionCount
        );
        status.putInt(
            TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_ENTER_COUNT,
            prefs.getInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_ENTER_COUNT, 0)
        );
        status.putInt(
            TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_EXIT_COUNT,
            prefs.getInt(TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_EXIT_COUNT, 0)
        );
        status.putString(
            TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_LAST_TRANSITION,
            prefs.getString(
                TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_LAST_TRANSITION,
                "none"
            )
        );
        status.putLong(
            TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_LAST_TRANSITION_EPOCH_MILLIS,
            prefs.getLong(
                TrackingAndroidGeofenceTransitionReceiver.FIELD_SYSTEM_PROXIMITY_LAST_TRANSITION_EPOCH_MILLIS,
                0L
            )
        );
        return status;
    }
}
