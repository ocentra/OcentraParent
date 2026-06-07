package ca.ocentra.parent.agent;

import android.Manifest;
import android.content.Context;
import android.content.SharedPreferences;
import android.content.pm.PackageManager;
import android.location.Location;
import android.location.LocationListener;
import android.location.LocationManager;
import android.os.Bundle;
import android.os.Looper;

public final class TrackingAndroidBackgroundLocationSampleProof {
    public static final String PREFS_NAME = "tracking_background_location_sample_proof";
    public static final String FIELD_BACKGROUND_SAMPLE_STATE = "backgroundLocationSampleState";
    public static final String FIELD_BACKGROUND_SAMPLE_COUNT = "backgroundLocationSampleCount";
    public static final String FIELD_BACKGROUND_SAMPLE_PROVIDER = "backgroundLocationSampleProvider";
    public static final String FIELD_BACKGROUND_SAMPLE_OBSERVED_AT_EPOCH_MILLIS =
        "backgroundLocationSampleObservedAtEpochMillis";
    public static final String FIELD_BACKGROUND_SAMPLE_ACCURACY_METERS = "backgroundLocationSampleAccuracyMeters";
    public static final String FIELD_BACKGROUND_SAMPLE_SOURCE = "backgroundLocationSampleSource";
    public static final String FIELD_BACKGROUND_SAMPLE_ACTIVITY_BACKGROUNDED =
        "backgroundLocationSampleActivityBackgrounded";
    public static final String BACKGROUND_SAMPLE_OBSERVED =
        "background-location-sample-observed-emulator-foreground-service";
    public static final String BACKGROUND_SAMPLE_MANUAL_REQUIRED = "background-location-sample-manual-required";
    public static final String BACKGROUND_SAMPLE_SOURCE_FOREGROUND_SERVICE =
        "android-location-manager-gps-listener-foreground-service";
    public static final String BACKGROUND_SAMPLE_BOUNDARY =
        "emulator-foreground-service-background-activity-location-sample-no-product-claim";
    private static LocationListener backgroundSampleListener;

    private TrackingAndroidBackgroundLocationSampleProof() {}

    public static Bundle startBackgroundSampleProof(Context context) {
        Context appContext = context.getApplicationContext();
        if (!hasRequiredPermissions(appContext)) {
            return createBackgroundSampleBundle(appContext);
        }
        LocationManager locationManager = (LocationManager) appContext.getSystemService(Context.LOCATION_SERVICE);
        if (locationManager == null || backgroundSampleListener != null) {
            return createBackgroundSampleBundle(appContext);
        }
        backgroundSampleListener = location -> recordBackgroundSample(appContext, location);
        try {
            locationManager.requestLocationUpdates(
                LocationManager.GPS_PROVIDER,
                0L,
                0.0f,
                backgroundSampleListener,
                Looper.getMainLooper()
            );
        } catch (IllegalArgumentException | SecurityException ignored) {
            backgroundSampleListener = null;
        }
        return createBackgroundSampleBundle(appContext);
    }

    public static Bundle createBackgroundSampleBundle(Context context) {
        SharedPreferences prefs = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE);
        int sampleCount = prefs.getInt(FIELD_BACKGROUND_SAMPLE_COUNT, 0);
        Bundle status = new Bundle();
        status.putString(
            FIELD_BACKGROUND_SAMPLE_STATE,
            sampleCount > 0 ? BACKGROUND_SAMPLE_OBSERVED : BACKGROUND_SAMPLE_MANUAL_REQUIRED
        );
        status.putString("proofBoundary", BACKGROUND_SAMPLE_BOUNDARY);
        status.putBoolean("backgroundLocationSampleCaptured", sampleCount > 0);
        status.putInt(FIELD_BACKGROUND_SAMPLE_COUNT, sampleCount);
        status.putBoolean(
            FIELD_BACKGROUND_SAMPLE_ACTIVITY_BACKGROUNDED,
            prefs.getBoolean(FIELD_BACKGROUND_SAMPLE_ACTIVITY_BACKGROUNDED, false)
        );
        status.putString(FIELD_BACKGROUND_SAMPLE_SOURCE, prefs.getString(FIELD_BACKGROUND_SAMPLE_SOURCE, "none"));
        status.putString(FIELD_BACKGROUND_SAMPLE_PROVIDER, prefs.getString(FIELD_BACKGROUND_SAMPLE_PROVIDER, "none"));
        status.putLong(
            FIELD_BACKGROUND_SAMPLE_OBSERVED_AT_EPOCH_MILLIS,
            prefs.getLong(FIELD_BACKGROUND_SAMPLE_OBSERVED_AT_EPOCH_MILLIS, 0L)
        );
        status.putFloat(
            FIELD_BACKGROUND_SAMPLE_ACCURACY_METERS,
            prefs.getFloat(FIELD_BACKGROUND_SAMPLE_ACCURACY_METERS, -1.0f)
        );
        return status;
    }

    private static boolean hasRequiredPermissions(Context context) {
        return (
            context.getPackageManager()
                .checkPermission(Manifest.permission.ACCESS_BACKGROUND_LOCATION, context.getPackageName()) ==
                PackageManager.PERMISSION_GRANTED &&
            (
                context.getPackageManager()
                    .checkPermission(Manifest.permission.ACCESS_FINE_LOCATION, context.getPackageName()) ==
                    PackageManager.PERMISSION_GRANTED ||
                context.getPackageManager()
                    .checkPermission(Manifest.permission.ACCESS_COARSE_LOCATION, context.getPackageName()) ==
                    PackageManager.PERMISSION_GRANTED
            )
        );
    }

    private static void recordBackgroundSample(Context context, Location location) {
        SharedPreferences prefs = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE);
        prefs.edit()
            .putInt(FIELD_BACKGROUND_SAMPLE_COUNT, prefs.getInt(FIELD_BACKGROUND_SAMPLE_COUNT, 0) + 1)
            .putString(FIELD_BACKGROUND_SAMPLE_PROVIDER, location.getProvider())
            .putLong(FIELD_BACKGROUND_SAMPLE_OBSERVED_AT_EPOCH_MILLIS, location.getTime())
            .putFloat(
                FIELD_BACKGROUND_SAMPLE_ACCURACY_METERS,
                location.hasAccuracy() ? location.getAccuracy() : -1.0f
            )
            .putString(FIELD_BACKGROUND_SAMPLE_SOURCE, BACKGROUND_SAMPLE_SOURCE_FOREGROUND_SERVICE)
            .putBoolean(FIELD_BACKGROUND_SAMPLE_ACTIVITY_BACKGROUNDED, true)
            .apply();
    }
}
