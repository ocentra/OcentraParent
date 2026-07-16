package ca.ocentra.parent.agent;

import android.Manifest;
import android.content.Context;
import android.content.pm.PackageManager;
import android.location.Location;
import android.location.LocationListener;
import android.location.LocationManager;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;

import com.google.android.gms.location.FusedLocationProviderClient;
import com.google.android.gms.location.LocationServices;
import com.google.android.gms.location.Priority;
import com.google.android.gms.tasks.CancellationTokenSource;

public final class TrackingAndroidForegroundLocationProof {
    public static final String SCHEMA_VERSION = "tracking-android-foreground-location-proof";
    public static final String FIELD_FOREGROUND_LOCATION_STATE = "foregroundLocationState";
    public static final String FIELD_FOREGROUND_LOCATION_PERMISSION_STATE = "foregroundLocationPermissionState";
    public static final String FIELD_FOREGROUND_LOCATION_SAMPLE_STATE = "foregroundLocationSampleState";
    public static final String FIELD_FOREGROUND_LOCATION_PROVIDER = "foregroundLocationProvider";
    public static final String FIELD_FOREGROUND_LOCATION_OBSERVED_AT_EPOCH_MILLIS =
        "foregroundLocationObservedAtEpochMillis";
    public static final String FIELD_FOREGROUND_LOCATION_ACCURACY_METERS = "foregroundLocationAccuracyMeters";
    public static final String FIELD_FOREGROUND_LOCATION_LATITUDE = "foregroundLocationLatitude";
    public static final String FIELD_FOREGROUND_LOCATION_LONGITUDE = "foregroundLocationLongitude";
    public static final String FIELD_FOREGROUND_LOCATION_SAMPLE_SOURCE = "foregroundLocationSampleSource";
    public static final String FIELD_FUSED_FOREGROUND_LOCATION_SAMPLE_STATE = "fusedForegroundLocationSampleState";
    public static final String FIELD_FUSED_FOREGROUND_LOCATION_PROVIDER = "fusedForegroundLocationProvider";
    public static final String FIELD_FUSED_FOREGROUND_LOCATION_OBSERVED_AT_EPOCH_MILLIS =
        "fusedForegroundLocationObservedAtEpochMillis";
    public static final String FIELD_FUSED_FOREGROUND_LOCATION_ACCURACY_METERS =
        "fusedForegroundLocationAccuracyMeters";
    public static final String FIELD_FUSED_FOREGROUND_LOCATION_LATITUDE = "fusedForegroundLocationLatitude";
    public static final String FIELD_FUSED_FOREGROUND_LOCATION_LONGITUDE = "fusedForegroundLocationLongitude";
    public static final String FIELD_FUSED_FOREGROUND_LOCATION_SAMPLE_SOURCE = "fusedForegroundLocationSampleSource";
    public static final String FOREGROUND_LOCATION_PERMISSION_GRANTED = "foreground-location-permission-granted";
    public static final String FOREGROUND_LOCATION_PERMISSION_REQUIRED = "foreground-location-permission-required";
    public static final String FOREGROUND_LOCATION_SAMPLE_LAST_KNOWN = "last-known-location-sample-observed";
    public static final String FOREGROUND_LOCATION_SAMPLE_CURRENT =
        "current-location-sample-observed-emulator-location-manager";
    public static final String FOREGROUND_LOCATION_SAMPLE_MANUAL_REQUIRED = "foreground-location-sample-manual-required";
    public static final String FUSED_FOREGROUND_LOCATION_SAMPLE_CURRENT =
        "current-fused-foreground-location-sample-observed-emulator";
    public static final String FUSED_FOREGROUND_LOCATION_SAMPLE_LAST_KNOWN =
        "last-known-fused-foreground-location-sample-observed";
    public static final String FUSED_FOREGROUND_LOCATION_SAMPLE_MANUAL_REQUIRED =
        "fused-foreground-location-sample-manual-required";
    public static final String FOREGROUND_LOCATION_SAMPLE_SOURCE_LAST_KNOWN = "android-location-manager-last-known";
    public static final String FOREGROUND_LOCATION_SAMPLE_SOURCE_CURRENT =
        "android-location-manager-current-listener-emulator";
    public static final String FUSED_FOREGROUND_LOCATION_SAMPLE_SOURCE_CURRENT =
        "google-play-services-fused-current-emulator";
    public static final String FUSED_FOREGROUND_LOCATION_SAMPLE_SOURCE_LAST_KNOWN =
        "google-play-services-fused-last-known";
    public static final String FOREGROUND_LOCATION_PROOF_BOUNDARY =
        "permission-readiness-only-no-background-geofence-or-product-claim";
    public static final int REQUEST_FOREGROUND_LOCATION = 4478;
    private static final long FUSED_SAMPLE_TIMEOUT_MILLIS = 3_000L;

    public interface ForegroundLocationProofCallback {
        void onForegroundLocationProof(Bundle proof);
    }

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

        return createForegroundLocationBundle(context, lastKnown, FOREGROUND_LOCATION_SAMPLE_SOURCE_LAST_KNOWN);
    }

    public static void requestForegroundLocationSample(
        Context context,
        ForegroundLocationProofCallback callback
    ) {
        if (!hasForegroundLocationPermission(context)) {
            callback.onForegroundLocationProof(createForegroundLocationBundle(context));
            return;
        }
        LocationManager manager = (LocationManager) context.getSystemService(Context.LOCATION_SERVICE);
        if (manager == null) {
            callback.onForegroundLocationProof(createForegroundLocationBundle(context));
            return;
        }
        LocationListener listener = new LocationListener() {
            @Override
            public void onLocationChanged(Location location) {
                callback.onForegroundLocationProof(
                    createForegroundLocationBundle(context, location, FOREGROUND_LOCATION_SAMPLE_SOURCE_CURRENT)
                );
                manager.removeUpdates(this);
            }
        };
        try {
            manager.requestLocationUpdates(LocationManager.GPS_PROVIDER, 0L, 0.0f, listener, Looper.getMainLooper());
            manager.requestLocationUpdates(
                LocationManager.NETWORK_PROVIDER,
                0L,
                0.0f,
                listener,
                Looper.getMainLooper()
            );
        } catch (IllegalArgumentException | SecurityException error) {
            callback.onForegroundLocationProof(createForegroundLocationBundle(context));
        }
    }

    public static Bundle createFusedForegroundLocationBundle(Context context) {
        return createFusedForegroundLocationBundle(context, null, "none", false);
    }

    public static void requestFusedForegroundLocationSample(
        Context context,
        ForegroundLocationProofCallback callback
    ) {
        if (!hasForegroundLocationPermission(context)) {
            callback.onForegroundLocationProof(createFusedForegroundLocationBundle(context));
            return;
        }
        FusedLocationProviderClient client = LocationServices.getFusedLocationProviderClient(context);
        Handler handler = new Handler(Looper.getMainLooper());
        boolean[] delivered = { false };
        CancellationTokenSource tokenSource = new CancellationTokenSource();
        handler.postDelayed(
            () -> {
                if (!delivered[0]) {
                    tokenSource.cancel();
                    requestFusedLastKnownLocation(context, client, callback, delivered);
                }
            },
            FUSED_SAMPLE_TIMEOUT_MILLIS
        );
        try {
            client.getCurrentLocation(Priority.PRIORITY_HIGH_ACCURACY, tokenSource.getToken())
                .addOnSuccessListener(location -> {
                    if (delivered[0]) {
                        return;
                    }
                    if (location != null) {
                        delivered[0] = true;
                        callback.onForegroundLocationProof(
                            createFusedForegroundLocationBundle(
                                context,
                                location,
                                FUSED_FOREGROUND_LOCATION_SAMPLE_SOURCE_CURRENT,
                                true
                            )
                        );
                        return;
                    }
                    requestFusedLastKnownLocation(context, client, callback, delivered);
                })
                .addOnFailureListener(error -> requestFusedLastKnownLocation(context, client, callback, delivered));
        } catch (SecurityException error) {
            delivered[0] = true;
            callback.onForegroundLocationProof(createFusedForegroundLocationBundle(context));
        }
    }

    private static Bundle createForegroundLocationBundle(Context context, Location location, String sampleSource) {
        boolean permissionGranted = hasForegroundLocationPermission(context);
        boolean locationObserved = location != null;
        boolean currentSample = locationObserved && FOREGROUND_LOCATION_SAMPLE_SOURCE_CURRENT.equals(sampleSource);

        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString(
            FIELD_FOREGROUND_LOCATION_PERMISSION_STATE,
            permissionGranted ? FOREGROUND_LOCATION_PERMISSION_GRANTED : FOREGROUND_LOCATION_PERMISSION_REQUIRED
        );
        status.putString(
            FIELD_FOREGROUND_LOCATION_SAMPLE_STATE,
            locationObserved
                ? currentSample ? FOREGROUND_LOCATION_SAMPLE_CURRENT : FOREGROUND_LOCATION_SAMPLE_LAST_KNOWN
                : FOREGROUND_LOCATION_SAMPLE_MANUAL_REQUIRED
        );
        status.putString(
            FIELD_FOREGROUND_LOCATION_STATE,
            permissionGranted && locationObserved
                ? currentSample
                    ? "foreground-location-current-observed-emulator"
                    : "foreground-location-last-known-observed"
                : "foreground-location-sample-manual-required"
        );
        status.putString("proofBoundary", FOREGROUND_LOCATION_PROOF_BOUNDARY);
        status.putBoolean("foregroundLocationPermissionGranted", permissionGranted);
        status.putBoolean("foregroundLocationSampleCaptured", locationObserved);
        status.putString(FIELD_FOREGROUND_LOCATION_SAMPLE_SOURCE, locationObserved ? sampleSource : "none");
        if (locationObserved) {
            status.putString(FIELD_FOREGROUND_LOCATION_PROVIDER, location.getProvider());
            status.putLong(FIELD_FOREGROUND_LOCATION_OBSERVED_AT_EPOCH_MILLIS, location.getTime());
            status.putFloat(
                FIELD_FOREGROUND_LOCATION_ACCURACY_METERS,
                location.hasAccuracy() ? location.getAccuracy() : -1.0f
            );
            status.putDouble(FIELD_FOREGROUND_LOCATION_LATITUDE, location.getLatitude());
            status.putDouble(FIELD_FOREGROUND_LOCATION_LONGITUDE, location.getLongitude());
        }
        return status;
    }

    private static void requestFusedLastKnownLocation(
        Context context,
        FusedLocationProviderClient client,
        ForegroundLocationProofCallback callback,
        boolean[] delivered
    ) {
        try {
            client.getLastLocation()
                .addOnSuccessListener(location -> {
                    if (delivered[0]) {
                        return;
                    }
                    delivered[0] = true;
                    callback.onForegroundLocationProof(
                        createFusedForegroundLocationBundle(
                            context,
                            location,
                            FUSED_FOREGROUND_LOCATION_SAMPLE_SOURCE_LAST_KNOWN,
                            false
                        )
                    );
                })
                .addOnFailureListener(error -> {
                    if (!delivered[0]) {
                        delivered[0] = true;
                        callback.onForegroundLocationProof(createFusedForegroundLocationBundle(context));
                    }
                });
        } catch (SecurityException error) {
            if (!delivered[0]) {
                delivered[0] = true;
                callback.onForegroundLocationProof(createFusedForegroundLocationBundle(context));
            }
        }
    }

    private static Bundle createFusedForegroundLocationBundle(
        Context context,
        Location location,
        String sampleSource,
        boolean currentSample
    ) {
        boolean permissionGranted = hasForegroundLocationPermission(context);
        boolean locationObserved = permissionGranted && location != null;
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putBoolean("fusedForegroundLocationPermissionGranted", permissionGranted);
        status.putBoolean("fusedForegroundLocationSampleCaptured", locationObserved);
        status.putString(
            FIELD_FUSED_FOREGROUND_LOCATION_SAMPLE_STATE,
            locationObserved
                ? currentSample
                    ? FUSED_FOREGROUND_LOCATION_SAMPLE_CURRENT
                    : FUSED_FOREGROUND_LOCATION_SAMPLE_LAST_KNOWN
                : FUSED_FOREGROUND_LOCATION_SAMPLE_MANUAL_REQUIRED
        );
        status.putString(FIELD_FUSED_FOREGROUND_LOCATION_SAMPLE_SOURCE, locationObserved ? sampleSource : "none");
        if (locationObserved) {
            status.putString(FIELD_FUSED_FOREGROUND_LOCATION_PROVIDER, location.getProvider());
            status.putLong(FIELD_FUSED_FOREGROUND_LOCATION_OBSERVED_AT_EPOCH_MILLIS, location.getTime());
            status.putFloat(
                FIELD_FUSED_FOREGROUND_LOCATION_ACCURACY_METERS,
                location.hasAccuracy() ? location.getAccuracy() : -1.0f
            );
            status.putDouble(FIELD_FUSED_FOREGROUND_LOCATION_LATITUDE, location.getLatitude());
            status.putDouble(FIELD_FUSED_FOREGROUND_LOCATION_LONGITUDE, location.getLongitude());
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
