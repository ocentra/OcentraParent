package ca.ocentra.parent.agent;

import android.Manifest;
import android.content.Context;
import android.content.pm.PackageManager;
import android.location.Location;
import android.location.LocationManager;
import android.os.Bundle;

public final class TrackingAndroidForegroundLocationProof {
    public static final String SCHEMA_VERSION = "tracking-android-foreground-location-proof";
    public static final String FIELD_FOREGROUND_LOCATION_STATE = "foregroundLocationState";
    public static final String FIELD_FOREGROUND_LOCATION_PERMISSION_STATE = "foregroundLocationPermissionState";
    public static final String FIELD_FOREGROUND_LOCATION_SAMPLE_STATE = "foregroundLocationSampleState";
    public static final String FOREGROUND_LOCATION_PERMISSION_GRANTED = "foreground-location-permission-granted";
    public static final String FOREGROUND_LOCATION_PERMISSION_REQUIRED = "foreground-location-permission-required";
    public static final String FOREGROUND_LOCATION_SAMPLE_LAST_KNOWN = "last-known-location-sample-observed";
    public static final String FOREGROUND_LOCATION_SAMPLE_MANUAL_REQUIRED = "foreground-location-sample-manual-required";
    public static final String FOREGROUND_LOCATION_PROOF_BOUNDARY =
        "permission-readiness-only-no-background-geofence-or-product-claim";
    public static final int REQUEST_FOREGROUND_LOCATION = 4478;

    private TrackingAndroidForegroundLocationProof() {}

    public static String[] foregroundLocationPermissions() {
        return new String[] {
            Manifest.permission.ACCESS_FINE_LOCATION,
            Manifest.permission.ACCESS_COARSE_LOCATION
        };
    }

    public static boolean shouldRequestForegroundLocationPermission(Context context) {
        return !hasForegroundLocationPermission(context);
    }

    public static boolean hasForegroundLocationPermission(Context context) {
        return (
            context.getPackageManager()
                .checkPermission(Manifest.permission.ACCESS_FINE_LOCATION, context.getPackageName()) ==
                PackageManager.PERMISSION_GRANTED ||
            context.getPackageManager()
                .checkPermission(Manifest.permission.ACCESS_COARSE_LOCATION, context.getPackageName()) ==
                PackageManager.PERMISSION_GRANTED
        );
    }

    public static Bundle createForegroundLocationBundle(Context context) {
        boolean permissionGranted = hasForegroundLocationPermission(context);
        Location lastKnown = permissionGranted ? readLastKnownLocation(context) : null;

        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString(
            FIELD_FOREGROUND_LOCATION_PERMISSION_STATE,
            permissionGranted ? FOREGROUND_LOCATION_PERMISSION_GRANTED : FOREGROUND_LOCATION_PERMISSION_REQUIRED
        );
        status.putString(
            FIELD_FOREGROUND_LOCATION_SAMPLE_STATE,
            lastKnown == null ? FOREGROUND_LOCATION_SAMPLE_MANUAL_REQUIRED : FOREGROUND_LOCATION_SAMPLE_LAST_KNOWN
        );
        status.putString(
            FIELD_FOREGROUND_LOCATION_STATE,
            permissionGranted && lastKnown != null
                ? "foreground-location-last-known-observed"
                : "foreground-location-sample-manual-required"
        );
        status.putString("proofBoundary", FOREGROUND_LOCATION_PROOF_BOUNDARY);
        status.putBoolean("foregroundLocationPermissionGranted", permissionGranted);
        status.putBoolean("foregroundLocationSampleCaptured", lastKnown != null);
        if (lastKnown != null) {
            status.putString("provider", lastKnown.getProvider());
            status.putLong("observedAtEpochMillis", lastKnown.getTime());
            status.putFloat("accuracyMeters", lastKnown.hasAccuracy() ? lastKnown.getAccuracy() : -1.0f);
        }
        return status;
    }

    private static Location readLastKnownLocation(Context context) {
        LocationManager manager = (LocationManager) context.getSystemService(Context.LOCATION_SERVICE);
        if (manager == null) {
            return null;
        }
        try {
            Location gps = manager.getLastKnownLocation(LocationManager.GPS_PROVIDER);
            if (gps != null) {
                return gps;
            }
            return manager.getLastKnownLocation(LocationManager.NETWORK_PROVIDER);
        } catch (SecurityException error) {
            return null;
        }
    }
}
