package ca.ocentra.parent.agent;

import android.app.Activity;
import android.content.Context;
import android.content.Intent;
import android.graphics.Color;
import android.os.Bundle;
import android.view.Gravity;
import android.widget.TextView;

import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.RejectedExecutionException;
import java.util.concurrent.ThreadFactory;
import java.util.concurrent.ThreadPoolExecutor;
import java.util.concurrent.TimeUnit;

public final class MainActivity extends Activity {
    public static final String EXTRA_START_SCREEN_CAPTURE_PROOF =
        "ca.ocentra.parent.agent.START_SCREEN_CAPTURE_PROOF";
    public static final String EXTRA_RUN_APP_GAME_RECEIPT_CHANNEL_PROOF =
        "ca.ocentra.parent.agent.RUN_APP_GAME_RECEIPT_CHANNEL_PROOF";
    public static final String EXTRA_RUN_APP_GAME_DELIVERY_INTAKE_PROOF =
        "ca.ocentra.parent.agent.RUN_APP_GAME_DELIVERY_INTAKE_PROOF";
    public static final String EXTRA_RUN_APP_GAME_LOCAL_NOTIFICATION_PROOF =
        "ca.ocentra.parent.agent.RUN_APP_GAME_LOCAL_NOTIFICATION_PROOF";
    public static final String EXTRA_RUN_APP_GAME_LOCAL_NOTIFICATION_ACTION_PROOF =
        "ca.ocentra.parent.agent.RUN_APP_GAME_LOCAL_NOTIFICATION_ACTION_PROOF";

    private final ExecutorService runtimePreflightWorker = new ThreadPoolExecutor(
        1,
        1,
        0L,
        TimeUnit.MILLISECONDS,
        new ArrayBlockingQueue<Runnable>(1),
        new RuntimePreflightThreadFactory(),
        new ThreadPoolExecutor.AbortPolicy()
    );
    private volatile boolean activityDestroyed;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        startForegroundService(new Intent(this, OcentraParentAgentService.class));
        if (getIntent().getBooleanExtra(EXTRA_START_SCREEN_CAPTURE_PROOF, false)) {
            startActivity(new Intent(this, AndroidMediaProjectionCaptureActivity.class));
        }
        if (getIntent().getBooleanExtra(EXTRA_RUN_APP_GAME_RECEIPT_CHANNEL_PROOF, false)) {
            Intent receiptChannelIntent = new Intent(
                AppGameAndroidChildRuntimeTransportReceiptProof.ACTION_LOCAL_RECEIPT_CHANNEL_PROOF
            );
            receiptChannelIntent.setPackage(getPackageName());
            sendBroadcast(receiptChannelIntent);
        }
        if (getIntent().getBooleanExtra(EXTRA_RUN_APP_GAME_DELIVERY_INTAKE_PROOF, false)) {
            Intent deliveryIntakeIntent = new Intent(
                AppGameAndroidChildRuntimeDeliveryProof.ACTION_LOCAL_DELIVERY_INTAKE_PROOF
            );
            deliveryIntakeIntent.setPackage(getPackageName());
            sendBroadcast(deliveryIntakeIntent);
        }
        Bundle appGameChildRuntimeLocalNotification =
            getIntent().getBooleanExtra(EXTRA_RUN_APP_GAME_LOCAL_NOTIFICATION_PROOF, false) ||
                getIntent().getBooleanExtra(EXTRA_RUN_APP_GAME_LOCAL_NOTIFICATION_ACTION_PROOF, false)
                ? AppGameAndroidChildRuntimeLocalNotificationProof.postLocalAppGameNotification(this)
                : AppGameAndroidChildRuntimeLocalNotificationProof.createLocalNotificationBundle(this);
        if (getIntent().getBooleanExtra(EXTRA_RUN_APP_GAME_LOCAL_NOTIFICATION_ACTION_PROOF, false)) {
            AppGameAndroidChildRuntimeLocalNotificationProof.triggerLocalRequestAction(this);
            appGameChildRuntimeLocalNotification =
                AppGameAndroidChildRuntimeLocalNotificationProof.createLocalNotificationBundle(this);
        }
        final Bundle appGameChildRuntimeLocalNotificationProof = appGameChildRuntimeLocalNotification;
        Bundle lifecycleProof = ChildAndroidLifecycleProof.createStatusBundle();
        Bundle storageProof = ChildAndroidStorageProtocolProof.createStorageProtocolBundle();
        Bundle serviceProof = ChildAndroidServiceProtocolProof.createServiceProtocolBundle();
        Bundle permissionProof = ChildAndroidPermissionCapabilityProof.createPermissionCapabilityBundle();
        Bundle privilegedProof = ChildAndroidPrivilegedCapabilityProof.createPrivilegedCapabilityBundle();
        Bundle screenProof = ChildAndroidScreenCaptureProof.createScreenCaptureBundle();
        Bundle appGameUsageEventsProof =
            AppGameAndroidUsageEventsCapabilityProof.createUsageEventsCapabilityBundle();
        final Bundle[] latestAppGameUsageEventsPreflight = {
            AppGameAndroidUsageEventsRuntimePreflight.createUnavailableRuntimePreflightBundle()
        };
        Bundle appGameAccessibilityRuntime =
            AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle();
        Bundle appGameChildRuntimeTransportReceipt =
            AppGameAndroidChildRuntimeTransportReceiptProof.createChildRuntimeTransportReceiptBundle(this);
        Bundle appGameChildRuntimeDelivery =
            AppGameAndroidChildRuntimeDeliveryProof.createChildRuntimeDeliveryBundle(this);
        Bundle appGameChildRuntimeNotificationRequestQueue =
            AppGameAndroidChildRuntimeNotificationRequestQueueProof.createRequestQueueBundle(this);
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
                appGameUsageEventsProof,
                latestAppGameUsageEventsPreflight[0],
                appGameAccessibilityRuntime,
                appGameChildRuntimeTransportReceipt,
                appGameChildRuntimeDelivery,
                appGameChildRuntimeLocalNotificationProof,
                appGameChildRuntimeNotificationRequestQueue,
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
                        appGameUsageEventsProof,
                        latestAppGameUsageEventsPreflight[0],
                        appGameAccessibilityRuntime,
                        appGameChildRuntimeTransportReceipt,
                        appGameChildRuntimeDelivery,
                        appGameChildRuntimeLocalNotificationProof,
                        appGameChildRuntimeNotificationRequestQueue,
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
                        appGameUsageEventsProof,
                        latestAppGameUsageEventsPreflight[0],
                        appGameAccessibilityRuntime,
                        appGameChildRuntimeTransportReceipt,
                        appGameChildRuntimeDelivery,
                        appGameChildRuntimeLocalNotificationProof,
                        appGameChildRuntimeNotificationRequestQueue,
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
        final Context applicationContext = getApplicationContext();
        try {
            runtimePreflightWorker.execute(new Runnable() {
                @Override
                public void run() {
                    Bundle updatedPreflight;
                    try {
                        updatedPreflight =
                            AppGameAndroidUsageEventsRuntimePreflight.createRuntimePreflightBundle(
                                applicationContext
                            );
                    } catch (RuntimeException error) {
                        updatedPreflight =
                            AppGameAndroidUsageEventsRuntimePreflight.createUnavailableRuntimePreflightBundle();
                        updatedPreflight.putString(
                            "workerFailure",
                            error.getClass().getSimpleName()
                        );
                    }
                    final Bundle completedPreflight = updatedPreflight;
                    runOnUiThread(new Runnable() {
                        @Override
                        public void run() {
                            if (activityDestroyed) {
                                return;
                            }
                            latestAppGameUsageEventsPreflight[0] = completedPreflight;
                            status.setText(
                                buildStatusText(
                                    lifecycleProof,
                                    storageProof,
                                    serviceProof,
                                    permissionProof,
                                    privilegedProof,
                                    screenProof,
                                    appGameUsageEventsProof,
                                    latestAppGameUsageEventsPreflight[0],
                                    appGameAccessibilityRuntime,
                                    appGameChildRuntimeTransportReceipt,
                                    appGameChildRuntimeDelivery,
                                    appGameChildRuntimeLocalNotificationProof,
                                    appGameChildRuntimeNotificationRequestQueue,
                                    latestForegroundLocationProof[0],
                                    latestFusedForegroundLocationProof[0],
                                    TrackingAndroidBackgroundLocationProof.createBackgroundLocationBundle(
                                        MainActivity.this
                                    ),
                                    TrackingAndroidBackgroundLocationSampleProof.createBackgroundSampleBundle(
                                        MainActivity.this
                                    )
                                )
                            );
                        }
                    });
                }
            });
        } catch (RejectedExecutionException error) {
            // The unavailable initial bundle remains authoritative when the bounded worker is full.
        }
    }

    @Override
    protected void onDestroy() {
        activityDestroyed = true;
        runtimePreflightWorker.shutdownNow();
        try {
            runtimePreflightWorker.awaitTermination(250L, TimeUnit.MILLISECONDS);
        } catch (InterruptedException error) {
            Thread.currentThread().interrupt();
        }
        super.onDestroy();
    }

    private String buildStatusText(
        Bundle lifecycleProof,
        Bundle storageProof,
        Bundle serviceProof,
        Bundle permissionProof,
        Bundle privilegedProof,
        Bundle screenProof,
        Bundle appGameUsageEventsProof,
        Bundle appGameUsageEventsPreflight,
        Bundle appGameAccessibilityRuntime,
        Bundle appGameChildRuntimeTransportReceipt,
        Bundle appGameChildRuntimeDelivery,
        Bundle appGameChildRuntimeLocalNotification,
        Bundle appGameChildRuntimeNotificationRequestQueue,
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
            appGameUsageEventsProof.getString(
                AppGameAndroidUsageEventsCapabilityProof.FIELD_USAGE_EVENTS_BRIDGE_STATE
            ) +
            "\n" +
            appGameUsageEventsPreflight.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_PERMISSION_CHECK_STATE
            ) +
            "\n" +
            appGameUsageEventsPreflight.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_STATE
            ) +
            "\n" +
            AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_EVENT_COUNT +
            "=" +
            appGameUsageEventsPreflight.getInt(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_EVENT_COUNT
            ) +
            "\n" +
            AppGameAndroidUsageEventsRuntimePreflight.FIELD_FOREGROUND_EVENT_COUNT +
            "=" +
            appGameUsageEventsPreflight.getInt(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_FOREGROUND_EVENT_COUNT
            ) +
            "\n" +
            appGameAccessibilityRuntime.getString(
                AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_DECLARATION_STATE
            ) +
            "\n" +
            appGameAccessibilityRuntime.getString(
                AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_RUNTIME_STATE
            ) +
            "\n" +
            appGameAccessibilityRuntime.getString(
                AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_STATE
            ) +
            "\n" +
            AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_COUNT +
            "=" +
            appGameAccessibilityRuntime.getInt(
                AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_COUNT
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_TRANSPORT_CHANNEL_STATE
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_STORE_STATE
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_ACK_STATE
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_APPEND_STATE
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_READBACK_STATE
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_LOCAL_ACK_STATE
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_LOCAL_ACK_READBACK_STATE
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_CHANNEL_STATE
            ) +
            "\n" +
            appGameChildRuntimeDelivery.getString(
                AppGameAndroidChildRuntimeDeliveryProof.FIELD_DELIVERY_INTAKE_STATE
            ) +
            "\n" +
            appGameChildRuntimeDelivery.getString(
                AppGameAndroidChildRuntimeDeliveryProof.FIELD_DELIVERY_READBACK_STATE
            ) +
            "\n" +
            appGameChildRuntimeLocalNotification.getString(
                AppGameAndroidChildRuntimeLocalNotificationProof.FIELD_NOTIFICATION_CHANNEL_STATE
            ) +
            "\n" +
            appGameChildRuntimeLocalNotification.getString(
                AppGameAndroidChildRuntimeLocalNotificationProof.FIELD_NOTIFICATION_POST_STATE
            ) +
            "\n" +
            appGameChildRuntimeLocalNotification.getString(
                AppGameAndroidChildRuntimeLocalNotificationProof.FIELD_NOTIFICATION_MARKER_STATE
            ) +
            "\n" +
            appGameChildRuntimeLocalNotification.getString(
                AppGameAndroidChildRuntimeLocalNotificationProof.FIELD_NOTIFICATION_REQUEST_ACTION_STATE
            ) +
            "\n" +
            appGameChildRuntimeNotificationRequestQueue.getString(
                AppGameAndroidChildRuntimeNotificationRequestQueueProof.FIELD_REQUEST_QUEUE_STATE
            ) +
            "\n" +
            appGameChildRuntimeNotificationRequestQueue.getString(
                AppGameAndroidChildRuntimeNotificationRequestQueueProof.FIELD_REQUEST_READBACK_STATE
            ) +
            "\n" +
            appGameChildRuntimeNotificationRequestQueue.getString(
                AppGameAndroidChildRuntimeNotificationRequestQueueProof.FIELD_REQUEST_DRAIN_STATE
            ) +
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

    private static final class RuntimePreflightThreadFactory implements ThreadFactory {
        @Override
        public Thread newThread(Runnable runnable) {
            return new Thread(runnable, "ocentra-parent-runtime-preflight");
        }
    }
}
