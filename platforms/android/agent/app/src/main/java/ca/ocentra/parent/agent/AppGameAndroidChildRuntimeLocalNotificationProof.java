package ca.ocentra.parent.agent;

import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.PendingIntent;
import android.content.Context;
import android.content.Intent;
import android.os.Build;
import android.os.Bundle;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;

public final class AppGameAndroidChildRuntimeLocalNotificationProof {
    public static final String SCHEMA_VERSION = "app-game-android-child-runtime-local-notification-proof";
    public static final String CHANNEL_ID = "ocentra_parent_app_game_child";
    public static final int NOTIFICATION_ID = 4480;
    public static final String FIELD_NOTIFICATION_CHANNEL_STATE = "notificationChannelState";
    public static final String FIELD_NOTIFICATION_POST_STATE = "notificationPostState";
    public static final String FIELD_NOTIFICATION_MARKER_STATE = "notificationMarkerState";
    public static final String FIELD_NOTIFICATION_REQUEST_ACTION_STATE = "notificationRequestActionState";
    public static final String CHANNEL_DECLARED = "local-notification-channel-declared";
    public static final String CHANNEL_UNAVAILABLE = "local-notification-channel-unavailable";
    public static final String POST_RECORDED = "local-notification-post-recorded";
    public static final String POST_UNAVAILABLE = "local-notification-post-unavailable";
    public static final String MARKER_RECORDED = "local-notification-marker-recorded";
    public static final String MARKER_UNAVAILABLE = "local-notification-marker-unavailable";
    public static final String REQUEST_ACTION_RECORDED = "local-notification-request-action-recorded";
    public static final String REQUEST_ACTION_UNAVAILABLE = "local-notification-request-action-unavailable";

    private static final String MARKER_FILE_NAME = "app-game-local-notification-proof-state.txt";
    private static final String MARKER_RECORD = "localNotificationId=android-child-runtime-app-game-warning-ref\n";

    private AppGameAndroidChildRuntimeLocalNotificationProof() {}

    public static Bundle postLocalAppGameNotification(Context context) {
        ensureNotificationChannel(context);
        boolean markerRecorded = writeMarker(context);
        boolean notificationPosted = postNotification(context);
        return createLocalNotificationBundle(context, notificationPosted, markerRecorded);
    }

    public static Bundle createLocalNotificationBundle(Context context) {
        return createLocalNotificationBundle(context, false, markerFile(context).isFile());
    }

    private static Bundle createLocalNotificationBundle(
        Context context,
        boolean notificationPosted,
        boolean markerRecorded
    ) {
        Bundle bundle = new Bundle();
        bundle.putString("schemaVersion", SCHEMA_VERSION);
        bundle.putString(FIELD_NOTIFICATION_CHANNEL_STATE, notificationChannelState(context));
        bundle.putString(FIELD_NOTIFICATION_POST_STATE, notificationPosted ? POST_RECORDED : POST_UNAVAILABLE);
        bundle.putString(FIELD_NOTIFICATION_MARKER_STATE, markerRecorded ? MARKER_RECORDED : MARKER_UNAVAILABLE);
        bundle.putString(FIELD_NOTIFICATION_REQUEST_ACTION_STATE, requestActionState(context));
        bundle.putStringArray(
            "proofRefs",
            new String[] {
                "android-child-runtime-local-notification-channel-ref",
                "android-child-runtime-local-notification-post-ref",
                "android-child-runtime-local-notification-marker-ref",
                "android-child-runtime-local-notification-request-action-ref"
            }
        );
        bundle.putBoolean("providerDeliveryClaimed", false);
        bundle.putBoolean("platformDeliveryOutsidePackageClaimed", false);
        bundle.putBoolean("adapterDispatchClaimed", false);
        bundle.putBoolean("platformEnforcementClaimed", false);
        bundle.putBoolean("rawPrivateSourceRowsStored", false);
        return bundle;
    }

    private static void ensureNotificationChannel(Context context) {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O) {
            return;
        }
        NotificationChannel channel = new NotificationChannel(
            CHANNEL_ID,
            context.getString(R.string.app_game_child_notification_channel_name),
            NotificationManager.IMPORTANCE_LOW
        );
        NotificationManager manager = context.getSystemService(NotificationManager.class);
        if (manager != null) {
            manager.createNotificationChannel(channel);
        }
    }

    private static boolean postNotification(Context context) {
        NotificationManager manager = context.getSystemService(NotificationManager.class);
        if (manager == null) {
            return false;
        }
        Notification.Builder builder = Build.VERSION.SDK_INT >= Build.VERSION_CODES.O
            ? new Notification.Builder(context, CHANNEL_ID)
            : new Notification.Builder(context);
        Notification notification = builder
            .setContentTitle(context.getString(R.string.app_game_child_notification_title))
            .setContentText(context.getString(R.string.app_game_child_notification_text))
            .setSmallIcon(android.R.drawable.ic_dialog_info)
            .addAction(
                android.R.drawable.ic_menu_send,
                context.getString(R.string.app_game_child_notification_request_action),
                requestActionIntent(context)
            )
            .setOngoing(false)
            .build();
        manager.notify(NOTIFICATION_ID, notification);
        return true;
    }

    public static void triggerLocalRequestAction(Context context) {
        Intent intent = new Intent(AppGameAndroidChildRuntimeNotificationActionReceiver.ACTION_LOCAL_REQUEST_ACTION_PROOF);
        intent.setPackage(context.getPackageName());
        context.sendBroadcast(intent);
        AppGameAndroidChildRuntimeNotificationActionReceiver.recordRequestAction(context);
    }

    private static String notificationChannelState(Context context) {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O) {
            return CHANNEL_DECLARED;
        }
        NotificationManager manager = context.getSystemService(NotificationManager.class);
        if (manager == null || manager.getNotificationChannel(CHANNEL_ID) == null) {
            return CHANNEL_UNAVAILABLE;
        }
        return CHANNEL_DECLARED;
    }

    private static boolean writeMarker(Context context) {
        try (FileOutputStream output = new FileOutputStream(markerFile(context), false)) {
            output.write(MARKER_RECORD.getBytes(StandardCharsets.UTF_8));
            return true;
        } catch (IOException ignored) {
            return false;
        }
    }

    private static File markerFile(Context context) {
        return new File(context.getFilesDir(), MARKER_FILE_NAME);
    }

    private static PendingIntent requestActionIntent(Context context) {
        Intent intent = new Intent(AppGameAndroidChildRuntimeNotificationActionReceiver.ACTION_LOCAL_REQUEST_ACTION_PROOF);
        intent.setPackage(context.getPackageName());
        return PendingIntent.getBroadcast(
            context,
            NOTIFICATION_ID,
            intent,
            PendingIntent.FLAG_UPDATE_CURRENT | PendingIntent.FLAG_IMMUTABLE
        );
    }

    private static String requestActionState(Context context) {
        return AppGameAndroidChildRuntimeNotificationActionReceiver.requestActionMarkerFile(context).isFile()
            ? REQUEST_ACTION_RECORDED
            : REQUEST_ACTION_UNAVAILABLE;
    }
}
