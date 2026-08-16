package ca.ocentra.child.agent;

import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.Service;
import android.content.Intent;
import android.os.Binder;
import android.os.Build;
import android.os.IBinder;

import ca.ocentra.parent.agent.R;

public final class ChildAgentCompositionService extends Service {
    private static final String CHANNEL_ID = "ocentra_child_agent";
    private static final int NOTIFICATION_ID = 4477;

    private final LocalBinder binder = new LocalBinder();
    private ChildAgentComposition composition;

    @Override
    public void onCreate() {
        super.onCreate();
        try {
            composition = ChildAgentComposition.open(this);
        } catch (Exception error) {
            composition = ChildAgentComposition.failed(this, error.getMessage());
        }
        ensureNotificationChannel();
        startForeground(NOTIFICATION_ID, buildNotification());
    }

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        return START_STICKY;
    }

    @Override
    public IBinder onBind(Intent intent) {
        return binder;
    }

    @Override
    public void onDestroy() {
        composition.close();
        super.onDestroy();
    }

    public final class LocalBinder extends Binder {
        public ChildAgentComposition.Health health() {
            return composition.health();
        }
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
        return new Notification.Builder(this, CHANNEL_ID)
            .setContentTitle(getString(R.string.app_name))
            .setContentText(getString(R.string.notification_text))
            .setSmallIcon(android.R.drawable.ic_menu_view)
            .setOngoing(true)
            .build();
    }
}
