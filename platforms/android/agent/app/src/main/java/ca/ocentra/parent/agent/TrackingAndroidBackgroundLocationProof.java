package ca.ocentra.parent.agent;

import android.Manifest;
import android.content.Context;
import android.content.pm.PackageManager;
import android.os.Bundle;

public final class TrackingAndroidBackgroundLocationProof {
    public static final String SCHEMA_VERSION = "tracking-android-background-location-proof";
    public static final String FIELD_BACKGROUND_LOCATION_PERMISSION_STATE = "backgroundLocationPermissionState";
    public static final String FIELD_BACKGROUND_GEOFENCE_STATE = "backgroundGeofenceState";
    public static final String BACKGROUND_LOCATION_PERMISSION_GRANTED = "background-location-permission-granted";
    public static final String BACKGROUND_LOCATION_PERMISSION_REQUIRED = "background-location-permission-required";
    public static final String BACKGROUND_GEOFENCE_MANUAL_REQUIRED = "background-geofence-transition-manual-required";
    public static final String BACKGROUND_LOCATION_PROOF_BOUNDARY =
        "background-permission-state-only-no-geofence-transition-or-product-claim";

    private TrackingAndroidBackgroundLocationProof() {}

    public static boolean hasBackgroundLocationPermission(Context context) {
        return (
            context.getPackageManager()
                .checkPermission(Manifest.permission.ACCESS_BACKGROUND_LOCATION, context.getPackageName()) ==
                PackageManager.PERMISSION_GRANTED
        );
    }

    public static Bundle createBackgroundLocationBundle(Context context) {
        boolean permissionGranted = hasBackgroundLocationPermission(context);
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString(
            FIELD_BACKGROUND_LOCATION_PERMISSION_STATE,
            permissionGranted ? BACKGROUND_LOCATION_PERMISSION_GRANTED : BACKGROUND_LOCATION_PERMISSION_REQUIRED
        );
        status.putString(FIELD_BACKGROUND_GEOFENCE_STATE, BACKGROUND_GEOFENCE_MANUAL_REQUIRED);
        status.putString("proofBoundary", BACKGROUND_LOCATION_PROOF_BOUNDARY);
        status.putBoolean("backgroundLocationPermissionGranted", permissionGranted);
        status.putBoolean("backgroundGeofenceTransitionCaptured", false);
        return status;
    }
}
