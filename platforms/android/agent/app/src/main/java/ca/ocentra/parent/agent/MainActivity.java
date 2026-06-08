package ca.ocentra.parent.agent;

import android.app.Activity;
import android.content.Intent;
import android.graphics.Color;
import android.os.Bundle;
import android.view.Gravity;
import android.widget.TextView;

public final class MainActivity extends Activity {
    public static final String EXTRA_START_SCREEN_CAPTURE_PROOF =
        "ca.ocentra.parent.agent.START_SCREEN_CAPTURE_PROOF";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        startForegroundService(new Intent(this, OcentraParentAgentService.class));
        if (getIntent().getBooleanExtra(EXTRA_START_SCREEN_CAPTURE_PROOF, false)) {
            startActivity(new Intent(this, AndroidMediaProjectionCaptureActivity.class));
        }
        Bundle lifecycleProof = ChildAndroidLifecycleProof.createStatusBundle();
        Bundle storageProof = ChildAndroidStorageProtocolProof.createStorageProtocolBundle();
        Bundle serviceProof = ChildAndroidServiceProtocolProof.createServiceProtocolBundle();
        Bundle permissionProof = ChildAndroidPermissionCapabilityProof.createPermissionCapabilityBundle();
        Bundle privilegedProof = ChildAndroidPrivilegedCapabilityProof.createPrivilegedCapabilityBundle();
        Bundle screenProof = ChildAndroidScreenCaptureProof.createScreenCaptureBundle();
        Bundle backgroundLocationProof = TrackingAndroidBackgroundLocationProof.createBackgroundLocationBundle(this);
        Bundle backgroundLocationSampleProof =
            TrackingAndroidBackgroundLocationSampleProof.createBackgroundSampleBundle(this);
        if (TrackingAndroidForegroundLocationProof.shouldRequestForegroundLocationPermission(this)) {
            requestPermissions(
                TrackingAndroidForegroundLocationProof.foregroundLocationPermissions(),
                TrackingAndroidForegroundLocationProof.REQUEST_FOREGROUND_LOCATION
            );
        }
        TrackingAndroidBackgroundLocationProof.registerEmulatorGeofenceProof(this);
        Bundle foregroundLocationProof = TrackingAndroidForegroundLocationProof.createForegroundLocationBundle(this);
        Bundle fusedForegroundLocationProof =
            TrackingAndroidForegroundLocationProof.createFusedForegroundLocationBundle(this);
        Bundle[] latestForegroundLocationProof = { foregroundLocationProof };
        Bundle[] latestFusedForegroundLocationProof = { fusedForegroundLocationProof };

        TextView status = new TextView(this);
        status.setText(
            buildStatusText(
                lifecycleProof,
                storageProof,
                serviceProof,
                permissionProof,
                privilegedProof,
                screenProof,
                foregroundLocationProof,
                fusedForegroundLocationProof,
                backgroundLocationProof,
                backgroundLocationSampleProof
            )
        );
        TrackingAndroidForegroundLocationProof.requestForegroundLocationSample(
            this,
            updatedForegroundLocationProof -> {
                latestForegroundLocationProof[0] = updatedForegroundLocationProof;
                status.setText(
                    buildStatusText(
                        lifecycleProof,
                        storageProof,
                        serviceProof,
                        permissionProof,
                        privilegedProof,
                        screenProof,
                        latestForegroundLocationProof[0],
                        latestFusedForegroundLocationProof[0],
                        TrackingAndroidBackgroundLocationProof.createBackgroundLocationBundle(this),
                        TrackingAndroidBackgroundLocationSampleProof.createBackgroundSampleBundle(this)
                    )
                );
            }
        );
        TrackingAndroidForegroundLocationProof.requestFusedForegroundLocationSample(
            this,
            updatedFusedForegroundLocationProof -> {
                latestFusedForegroundLocationProof[0] = updatedFusedForegroundLocationProof;
                status.setText(
                    buildStatusText(
                        lifecycleProof,
                        storageProof,
                        serviceProof,
                        permissionProof,
                        privilegedProof,
                        screenProof,
                        latestForegroundLocationProof[0],
                        latestFusedForegroundLocationProof[0],
                        TrackingAndroidBackgroundLocationProof.createBackgroundLocationBundle(this),
                        TrackingAndroidBackgroundLocationSampleProof.createBackgroundSampleBundle(this)
                    )
                );
            }
        );
        status.setBackgroundColor(Color.rgb(249, 250, 251));
        status.setTextColor(Color.rgb(17, 24, 39));
        status.setTextSize(18);
        status.setGravity(Gravity.CENTER);
        status.setPadding(32, 32, 32, 32);
        setContentView(status);
    }

    private String buildStatusText(
        Bundle lifecycleProof,
        Bundle storageProof,
        Bundle serviceProof,
        Bundle permissionProof,
        Bundle privilegedProof,
        Bundle screenProof,
        Bundle foregroundLocationProof,
        Bundle fusedForegroundLocationProof,
        Bundle backgroundLocationProof,
        Bundle backgroundLocationSampleProof
    ) {
        String foregroundLocationMetadata = foregroundLocationProof.getBoolean("foregroundLocationSampleCaptured")
            ? "\n" +
            TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_PROVIDER +
            ":" +
            foregroundLocationProof.getString(
                TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_PROVIDER
            ) +
            "\n" +
            TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_OBSERVED_AT_EPOCH_MILLIS +
            ":" +
            foregroundLocationProof.getLong(
                TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_OBSERVED_AT_EPOCH_MILLIS
            ) +
            "\n" +
            TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_ACCURACY_METERS +
            ":" +
            foregroundLocationProof.getFloat(
                TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_ACCURACY_METERS
            ) +
            "\n" +
            TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_SAMPLE_SOURCE +
            ":" +
            foregroundLocationProof.getString(
                TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_SAMPLE_SOURCE
            ) +
            "\n" +
            TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_LATITUDE +
            ":" +
            foregroundLocationProof.getDouble(
                TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_LATITUDE
            ) +
            "\n" +
            TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_LONGITUDE +
            ":" +
            foregroundLocationProof.getDouble(
                TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_LONGITUDE
            )
            : "";
        String fusedForegroundLocationMetadata = fusedForegroundLocationProof.getBoolean(
            "fusedForegroundLocationSampleCaptured"
        )
            ? "\n" +
            TrackingAndroidForegroundLocationProof.FIELD_FUSED_FOREGROUND_LOCATION_PROVIDER +
            ":" +
            fusedForegroundLocationProof.getString(
                TrackingAndroidForegroundLocationProof.FIELD_FUSED_FOREGROUND_LOCATION_PROVIDER
            ) +
            "\n" +
            TrackingAndroidForegroundLocationProof.FIELD_FUSED_FOREGROUND_LOCATION_OBSERVED_AT_EPOCH_MILLIS +
            ":" +
            fusedForegroundLocationProof.getLong(
                TrackingAndroidForegroundLocationProof.FIELD_FUSED_FOREGROUND_LOCATION_OBSERVED_AT_EPOCH_MILLIS
            ) +
            "\n" +
            TrackingAndroidForegroundLocationProof.FIELD_FUSED_FOREGROUND_LOCATION_ACCURACY_METERS +
            ":" +
            fusedForegroundLocationProof.getFloat(
                TrackingAndroidForegroundLocationProof.FIELD_FUSED_FOREGROUND_LOCATION_ACCURACY_METERS
            ) +
            "\n" +
            TrackingAndroidForegroundLocationProof.FIELD_FUSED_FOREGROUND_LOCATION_SAMPLE_SOURCE +
            ":" +
            fusedForegroundLocationProof.getString(
                TrackingAndroidForegroundLocationProof.FIELD_FUSED_FOREGROUND_LOCATION_SAMPLE_SOURCE
            ) +
            "\n" +
            TrackingAndroidForegroundLocationProof.FIELD_FUSED_FOREGROUND_LOCATION_LATITUDE +
            ":" +
            fusedForegroundLocationProof.getDouble(
                TrackingAndroidForegroundLocationProof.FIELD_FUSED_FOREGROUND_LOCATION_LATITUDE
            ) +
            "\n" +
            TrackingAndroidForegroundLocationProof.FIELD_FUSED_FOREGROUND_LOCATION_LONGITUDE +
            ":" +
            fusedForegroundLocationProof.getDouble(
                TrackingAndroidForegroundLocationProof.FIELD_FUSED_FOREGROUND_LOCATION_LONGITUDE
            )
            : "";
        return getString(R.string.agent_status) +
            "\n" +
            lifecycleProof.getString(ChildAndroidLifecycleProof.FIELD_BRIDGE_STATE) +
            "\n" +
            storageProof.getString(ChildAndroidStorageProtocolProof.FIELD_STORAGE_BRIDGE_STATE) +
            "\n" +
            serviceProof.getString(ChildAndroidServiceProtocolProof.FIELD_FOREGROUND_SERVICE_STATUS) +
            "\n" +
            permissionProof.getString(ChildAndroidPermissionCapabilityProof.FIELD_PERMISSION_BRIDGE_STATE) +
            "\n" +
            privilegedProof.getString(ChildAndroidPrivilegedCapabilityProof.FIELD_PRIVILEGED_BRIDGE_STATE) +
            "\n" +
            screenProof.getString(ChildAndroidScreenCaptureProof.FIELD_SCREEN_CAPTURE_STATE) +
            "\n" +
            foregroundLocationProof.getString(
                TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_PERMISSION_STATE
            ) +
            "\n" +
            foregroundLocationProof.getString(
                TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_SAMPLE_STATE
            ) +
            "\n" +
            fusedForegroundLocationProof.getString(
                TrackingAndroidForegroundLocationProof.FIELD_FUSED_FOREGROUND_LOCATION_SAMPLE_STATE
            ) +
            "\n" +
            backgroundLocationProof.getString(
                TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_LOCATION_PERMISSION_STATE
            ) +
            "\n" +
            backgroundLocationProof.getString(
                TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_STATE
            ) +
            "\n" +
            TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_TRANSITION_COUNT +
            ":" +
            backgroundLocationProof.getInt(
                TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_TRANSITION_COUNT
            ) +
            "\n" +
            TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_ENTER_COUNT +
            ":" +
            backgroundLocationProof.getInt(
                TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_ENTER_COUNT
            ) +
            "\n" +
            TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_EXIT_COUNT +
            ":" +
            backgroundLocationProof.getInt(
                TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_EXIT_COUNT
            ) +
            "\n" +
            TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_DWELL_COUNT +
            ":" +
            backgroundLocationProof.getInt(
                TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_DWELL_COUNT
            ) +
            "\n" +
            TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_DWELL_SOURCE +
            ":" +
            backgroundLocationProof.getString(
                TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_DWELL_SOURCE
            ) +
            "\n" +
            TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_LAST_TRANSITION +
            ":" +
            backgroundLocationProof.getString(
                TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_LAST_TRANSITION
            ) +
            "\n" +
            TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_SOURCE +
            ":" +
            backgroundLocationProof.getString(
                TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_SOURCE
            ) +
            "\n" +
            backgroundLocationSampleProof.getString(
                TrackingAndroidBackgroundLocationSampleProof.FIELD_BACKGROUND_SAMPLE_STATE
            ) +
            "\n" +
            TrackingAndroidBackgroundLocationSampleProof.FIELD_BACKGROUND_SAMPLE_COUNT +
            ":" +
            backgroundLocationSampleProof.getInt(
                TrackingAndroidBackgroundLocationSampleProof.FIELD_BACKGROUND_SAMPLE_COUNT
            ) +
            "\n" +
            TrackingAndroidBackgroundLocationSampleProof.FIELD_BACKGROUND_SAMPLE_PROVIDER +
            ":" +
            backgroundLocationSampleProof.getString(
                TrackingAndroidBackgroundLocationSampleProof.FIELD_BACKGROUND_SAMPLE_PROVIDER
            ) +
            "\n" +
            TrackingAndroidBackgroundLocationSampleProof.FIELD_BACKGROUND_SAMPLE_OBSERVED_AT_EPOCH_MILLIS +
            ":" +
            backgroundLocationSampleProof.getLong(
                TrackingAndroidBackgroundLocationSampleProof.FIELD_BACKGROUND_SAMPLE_OBSERVED_AT_EPOCH_MILLIS
            ) +
            "\n" +
            TrackingAndroidBackgroundLocationSampleProof.FIELD_BACKGROUND_SAMPLE_ACCURACY_METERS +
            ":" +
            backgroundLocationSampleProof.getFloat(
                TrackingAndroidBackgroundLocationSampleProof.FIELD_BACKGROUND_SAMPLE_ACCURACY_METERS
            ) +
            "\n" +
            TrackingAndroidBackgroundLocationSampleProof.FIELD_BACKGROUND_SAMPLE_SOURCE +
            ":" +
            backgroundLocationSampleProof.getString(
                TrackingAndroidBackgroundLocationSampleProof.FIELD_BACKGROUND_SAMPLE_SOURCE
            ) +
            "\n" +
            TrackingAndroidBackgroundLocationSampleProof.FIELD_BACKGROUND_SAMPLE_ACTIVITY_BACKGROUNDED +
            ":" +
            backgroundLocationSampleProof.getBoolean(
                TrackingAndroidBackgroundLocationSampleProof.FIELD_BACKGROUND_SAMPLE_ACTIVITY_BACKGROUNDED
            ) +
            foregroundLocationMetadata +
            fusedForegroundLocationMetadata;
    }
}
