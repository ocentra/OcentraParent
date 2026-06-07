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
        if (TrackingAndroidForegroundLocationProof.shouldRequestForegroundLocationPermission(this)) {
            requestPermissions(
                TrackingAndroidForegroundLocationProof.foregroundLocationPermissions(),
                TrackingAndroidForegroundLocationProof.REQUEST_FOREGROUND_LOCATION
            );
        }
        Bundle foregroundLocationProof = TrackingAndroidForegroundLocationProof.createForegroundLocationBundle(this);

        TextView status = new TextView(this);
        status.setText(
            buildStatusText(
                lifecycleProof,
                storageProof,
                serviceProof,
                permissionProof,
                privilegedProof,
                screenProof,
                foregroundLocationProof
            )
        );
        TrackingAndroidForegroundLocationProof.requestForegroundLocationSample(
            this,
            updatedForegroundLocationProof -> status.setText(
                buildStatusText(
                    lifecycleProof,
                    storageProof,
                    serviceProof,
                    permissionProof,
                    privilegedProof,
                    screenProof,
                    updatedForegroundLocationProof
                )
            )
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
        Bundle foregroundLocationProof
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
            foregroundLocationMetadata;
    }
}
