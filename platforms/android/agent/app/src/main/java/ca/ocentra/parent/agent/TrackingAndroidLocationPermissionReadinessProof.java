package ca.ocentra.parent.agent;

import android.os.Bundle;

public final class TrackingAndroidLocationPermissionReadinessProof {
    public static final String SCHEMA_VERSION =
        "tracking-android-location-permission-readiness-proof";
    public static final String PACKAGE_ID = "ca.ocentra.parent.agent";
    public static final String NATIVE_BRIDGE_CLASS =
        "ca.ocentra.parent.agent.TrackingAndroidLocationPermissionReadinessProof";
    public static final String COMMAND_STATIC_PERMISSION_READINESS_GET =
        "tracking.android.static-permission.readiness.get";
    public static final String EVENT_STATIC_PERMISSION_READINESS_REPORTED =
        "tracking.android.static-permission.readiness.reported";
    public static final String STATIC_READINESS_STATE = "manifest-declared-build-proof";
    public static final String RUNTIME_LOCATION_STATE = "manual-runtime-required";
    public static final String GEOFENCE_RUNTIME_STATE = "manual-runtime-required";
    public static final String FIELD_STATIC_READINESS_STATE = "trackingStaticPermissionReadinessState";

    private TrackingAndroidLocationPermissionReadinessProof() {}

    public static Bundle createReadinessBundle() {
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString("packageId", PACKAGE_ID);
        status.putString("nativeBridgeClass", NATIVE_BRIDGE_CLASS);
        status.putString(FIELD_STATIC_READINESS_STATE, STATIC_READINESS_STATE);
        status.putString("runtimeLocationState", RUNTIME_LOCATION_STATE);
        status.putString("geofenceRuntimeState", GEOFENCE_RUNTIME_STATE);
        status.putStringArray("commands", new String[] { COMMAND_STATIC_PERMISSION_READINESS_GET });
        status.putStringArray("events", new String[] { EVENT_STATIC_PERMISSION_READINESS_REPORTED });
        status.putStringArray(
            "declaredManifestPermissions",
            new String[] {
                "android.permission.FOREGROUND_SERVICE_LOCATION",
                "android.permission.ACCESS_COARSE_LOCATION",
                "android.permission.ACCESS_FINE_LOCATION",
                "android.permission.ACCESS_BACKGROUND_LOCATION"
            }
        );
        status.putStringArray(
            "foregroundServiceTypes",
            new String[] {
                "dataSync",
                "location"
            }
        );
        status.putStringArray(
            "manualRuntimeClaims",
            new String[] {
                "foreground-permission-grant",
                "foreground-location-sample",
                "background-permission-grant",
                "geofence-transition-runtime",
                "physical-device-proof"
            }
        );
        return status;
    }
}
