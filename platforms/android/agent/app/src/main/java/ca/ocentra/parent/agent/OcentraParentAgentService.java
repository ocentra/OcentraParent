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

    @Override
    public void onCreate() {
        super.onCreate();
        lifecycleProof = ChildAndroidLifecycleProof.createStatusBundle();
        storageProof = ChildAndroidStorageProtocolProof.createStorageProtocolBundle();
        ensureNotificationChannel();
        startForeground(NOTIFICATION_ID, buildNotification());
    }

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        lifecycleProof = ChildAndroidLifecycleProof.createStatusBundle();
        storageProof = ChildAndroidStorageProtocolProof.createStorageProtocolBundle();
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
                storageProof.getString(ChildAndroidStorageProtocolProof.FIELD_STORAGE_BRIDGE_STATE)
            )
            .setSmallIcon(android.R.drawable.ic_menu_view)
            .setOngoing(true)
            .build();
    }
}
