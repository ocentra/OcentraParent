package ca.ocentra.parent.agent;

import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.Service;
import android.content.Intent;
import android.os.Build;
import android.os.Bundle;
import android.os.IBinder;

public final class OcentraParentAgentService extends Service {
    private static final String CHANNEL_ID = "ocentra_parent_agent";
    private static final int NOTIFICATION_ID = 4477;
    private Bundle lifecycleProof;
    private Bundle storageProof;
    private Bundle serviceProof;
    private Bundle permissionProof;
    private Bundle privilegedProof;
    private Bundle foregroundLocationProof;
    private Bundle backgroundLocationProof;
    private Bundle backgroundLocationSampleProof;

    @Override
    public void onCreate() {
        super.onCreate();
        lifecycleProof = ChildAndroidLifecycleProof.createStatusBundle();
        storageProof = ChildAndroidStorageProtocolProof.createStorageProtocolBundle();
        serviceProof = ChildAndroidServiceProtocolProof.createServiceProtocolBundle();
        permissionProof = ChildAndroidPermissionCapabilityProof.createPermissionCapabilityBundle();
        privilegedProof = ChildAndroidPrivilegedCapabilityProof.createPrivilegedCapabilityBundle();
        foregroundLocationProof = TrackingAndroidForegroundLocationProof.createForegroundLocationBundle(this);
        TrackingAndroidBackgroundLocationProof.registerEmulatorGeofenceProof(this);
        backgroundLocationProof = TrackingAndroidBackgroundLocationProof.createBackgroundLocationBundle(this);
        backgroundLocationSampleProof =
            TrackingAndroidBackgroundLocationSampleProof.createBackgroundSampleBundle(this);
        ensureNotificationChannel();
        startForeground(NOTIFICATION_ID, buildNotification());
        backgroundLocationSampleProof =
            TrackingAndroidBackgroundLocationSampleProof.startBackgroundSampleProof(this);
    }

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        lifecycleProof = ChildAndroidLifecycleProof.createStatusBundle();
        storageProof = ChildAndroidStorageProtocolProof.createStorageProtocolBundle();
        serviceProof = ChildAndroidServiceProtocolProof.createServiceProtocolBundle();
        permissionProof = ChildAndroidPermissionCapabilityProof.createPermissionCapabilityBundle();
        privilegedProof = ChildAndroidPrivilegedCapabilityProof.createPrivilegedCapabilityBundle();
        foregroundLocationProof = TrackingAndroidForegroundLocationProof.createForegroundLocationBundle(this);
        TrackingAndroidBackgroundLocationProof.registerEmulatorGeofenceProof(this);
        backgroundLocationProof = TrackingAndroidBackgroundLocationProof.createBackgroundLocationBundle(this);
        backgroundLocationSampleProof =
            TrackingAndroidBackgroundLocationSampleProof.startBackgroundSampleProof(this);
        return START_STICKY;
    }

    @Override
    public IBinder onBind(Intent intent) {
        return null;
    }

    private void ensureNotificationChannel() {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O) {
            return;
        }

        NotificationChannel channel = new NotificationChannel(
            CHANNEL_ID,
            getString(R.string.notification_channel_name),
            NotificationManager.IMPORTANCE_LOW
        );
        NotificationManager manager = getSystemService(NotificationManager.class);
        manager.createNotificationChannel(channel);
    }

    private Notification buildNotification() {
        Notification.Builder builder = new Notification.Builder(this, CHANNEL_ID);

        return builder
            .setContentTitle(getString(R.string.app_name))
            .setContentText(
                getString(R.string.notification_text) +
                " " +
                lifecycleProof.getString(ChildAndroidLifecycleProof.FIELD_BRIDGE_STATE) +
                " " +
                storageProof.getString(ChildAndroidStorageProtocolProof.FIELD_STORAGE_BRIDGE_STATE) +
                " " +
                serviceProof.getString(ChildAndroidServiceProtocolProof.FIELD_FOREGROUND_SERVICE_STATUS) +
                " " +
                permissionProof.getString(ChildAndroidPermissionCapabilityProof.FIELD_PERMISSION_BRIDGE_STATE) +
                " " +
                privilegedProof.getString(ChildAndroidPrivilegedCapabilityProof.FIELD_PRIVILEGED_BRIDGE_STATE) +
                " " +
                foregroundLocationProof.getString(
                    TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_PERMISSION_STATE
                ) +
                " " +
                foregroundLocationProof.getString(
                    TrackingAndroidForegroundLocationProof.FIELD_FOREGROUND_LOCATION_SAMPLE_STATE
                ) +
                " " +
                backgroundLocationProof.getString(
                    TrackingAndroidBackgroundLocationProof.FIELD_BACKGROUND_GEOFENCE_STATE
                ) +
                " " +
                backgroundLocationSampleProof.getString(
                    TrackingAndroidBackgroundLocationSampleProof.FIELD_BACKGROUND_SAMPLE_STATE
                )
            )
            .setSmallIcon(android.R.drawable.ic_menu_view)
            .setOngoing(true)
            .build();
    }
}
