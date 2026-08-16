package ca.ocentra.parent.agent;

import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;

public final class AppGameAndroidChildRuntimeNotificationActionReceiver extends BroadcastReceiver {
    public static final String ACTION_LOCAL_REQUEST_ACTION_PROOF =
        "ca.ocentra.parent.agent.APP_GAME_CHILD_RUNTIME_NOTIFICATION_REQUEST_ACTION_PROOF";

    private static final String REQUEST_ACTION_MARKER_FILE_NAME =
        "app-game-local-notification-request-action-proof-state.txt";
    private static final String REQUEST_ACTION_MARKER_RECORD =
        "requestActionId=android-child-runtime-local-notification-request-action-ref\n";

    @Override
    public void onReceive(Context context, Intent intent) {
        if (
            intent == null ||
            !ACTION_LOCAL_REQUEST_ACTION_PROOF.equals(intent.getAction()) ||
            context == null
        ) {
            return;
        }
        recordRequestAction(context);
    }

    public static boolean markerReadbackObserved(Context context) {
        File markerFile = requestActionMarkerFile(context);
        return markerFile.isFile() && markerFile.length() == REQUEST_ACTION_MARKER_RECORD.getBytes(
            StandardCharsets.UTF_8
        ).length;
    }

    public static File requestActionMarkerFile(Context context) {
        return new File(context.getFilesDir(), REQUEST_ACTION_MARKER_FILE_NAME);
    }

    public static void recordRequestAction(Context context) {
        try (FileOutputStream output = new FileOutputStream(requestActionMarkerFile(context), false)) {
            output.write(REQUEST_ACTION_MARKER_RECORD.getBytes(StandardCharsets.UTF_8));
        } catch (IOException ignored) {}
        AppGameAndroidChildRuntimeNotificationRequestQueueProof.recordRequestQueue(context);
    }
}
