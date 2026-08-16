package ca.ocentra.parent.agent;

import android.content.Context;
import android.os.Bundle;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;

public final class AppGameAndroidChildRuntimeNotificationRequestQueueProof {
    public static final String SCHEMA_VERSION =
        "app-game-android-child-runtime-local-notification-request-queue-proof";
    public static final String FIELD_REQUEST_QUEUE_STATE = "notificationRequestQueueState";
    public static final String FIELD_REQUEST_READBACK_STATE = "notificationRequestReadbackState";
    public static final String FIELD_REQUEST_DRAIN_STATE = "notificationRequestDrainState";
    public static final String REQUEST_QUEUE_RECORDED =
        "local-notification-request-queue-recorded";
    public static final String REQUEST_QUEUE_UNAVAILABLE =
        "local-notification-request-queue-unavailable";
    public static final String REQUEST_READBACK_OBSERVED =
        "local-notification-request-readback-observed";
    public static final String REQUEST_READBACK_UNAVAILABLE =
        "local-notification-request-readback-unavailable";
    public static final String REQUEST_DRAIN_RECORDED =
        "local-notification-request-drain-recorded";
    public static final String REQUEST_DRAIN_UNAVAILABLE =
        "local-notification-request-drain-unavailable";

    private static final String REQUEST_QUEUE_DIR_NAME =
        "app-game-local-notification-request-queue";
    private static final String REQUEST_QUEUE_FILE_NAME = "request-queue-proof-state.txt";
    private static final String REQUEST_DRAIN_FILE_NAME = "request-drain-proof-state.txt";
    private static final String REQUEST_QUEUE_RECORD =
        "requestQueueId=android-child-runtime-local-notification-request-queue-ref\n";
    private static final String REQUEST_DRAIN_RECORD =
        "requestDrainId=android-child-runtime-local-notification-request-drain-ref\n";

    private AppGameAndroidChildRuntimeNotificationRequestQueueProof() {}

    public static Bundle createRequestQueueBundle(Context context) {
        RequestQueueState requestQueueState = readRequestQueueState(context);
        Bundle bundle = new Bundle();
        bundle.putString("schemaVersion", SCHEMA_VERSION);
        bundle.putString(FIELD_REQUEST_QUEUE_STATE, requestQueueState.requestQueueState);
        bundle.putString(FIELD_REQUEST_READBACK_STATE, requestQueueState.requestReadbackState);
        bundle.putString(FIELD_REQUEST_DRAIN_STATE, requestQueueState.requestDrainState);
        bundle.putStringArray(
            "proofRefs",
            new String[] {
                "android-child-runtime-local-notification-request-action-ref",
                "android-child-runtime-local-notification-request-queue-ref",
                "android-child-runtime-local-notification-request-readback-ref",
                "android-child-runtime-local-notification-request-drain-ref"
            }
        );
        bundle.putStringArray(
            "openGaps",
            new String[] {
                "android-service-request-ingestion-not-proved",
                "android-parent-approval-round-trip-not-proved",
                "android-provider-delivery-not-proved",
                "android-platform-delivery-outside-package-not-proved",
                "android-adapter-dispatch-not-proved",
                "android-platform-enforcement-not-proved",
                "android-raw-private-source-rows-not-included"
            }
        );
        bundle.putBoolean("serviceRequestIngestionClaimed", false);
        bundle.putBoolean("parentApprovalRoundTripClaimed", false);
        bundle.putBoolean("providerDeliveryClaimed", false);
        bundle.putBoolean("platformDeliveryOutsidePackageClaimed", false);
        bundle.putBoolean("adapterDispatchClaimed", false);
        bundle.putBoolean("platformEnforcementClaimed", false);
        bundle.putBoolean("rawPrivateSourceRowsStored", false);
        return bundle;
    }

    public static void recordRequestQueue(Context context) {
        File requestQueueDir = requestQueueDirectory(context);
        if (requestQueueDir == null) {
            return;
        }
        writeAndReadRequestFile(new File(requestQueueDir, REQUEST_QUEUE_FILE_NAME), REQUEST_QUEUE_RECORD);
        writeAndReadRequestFile(new File(requestQueueDir, REQUEST_DRAIN_FILE_NAME), REQUEST_DRAIN_RECORD);
    }

    public static File requestQueueFile(Context context) {
        File requestQueueDir = requestQueueDirectory(context);
        return requestQueueDir == null ? new File(context.getFilesDir(), REQUEST_QUEUE_FILE_NAME) :
            new File(requestQueueDir, REQUEST_QUEUE_FILE_NAME);
    }

    public static File requestDrainFile(Context context) {
        File requestQueueDir = requestQueueDirectory(context);
        return requestQueueDir == null ? new File(context.getFilesDir(), REQUEST_DRAIN_FILE_NAME) :
            new File(requestQueueDir, REQUEST_DRAIN_FILE_NAME);
    }

    private static RequestQueueState readRequestQueueState(Context context) {
        File requestQueueFile = requestQueueFile(context);
        File requestDrainFile = requestDrainFile(context);
        if (
            requestQueueFile.isFile() &&
            readRequestFile(requestQueueFile, REQUEST_QUEUE_RECORD) &&
            requestDrainFile.isFile() &&
            readRequestFile(requestDrainFile, REQUEST_DRAIN_RECORD)
        ) {
            return RequestQueueState.recorded();
        }
        return RequestQueueState.unavailable();
    }

    private static File requestQueueDirectory(Context context) {
        File filesDir = context.getFilesDir();
        if (filesDir == null || (!filesDir.exists() && !filesDir.mkdirs())) {
            return null;
        }
        File requestQueueDir = new File(filesDir, REQUEST_QUEUE_DIR_NAME);
        if (!requestQueueDir.exists() && !requestQueueDir.mkdirs()) {
            return null;
        }
        return requestQueueDir;
    }

    private static boolean writeAndReadRequestFile(File requestFile, String requestRecord) {
        try (FileOutputStream outputStream = new FileOutputStream(requestFile, false)) {
            outputStream.write(requestRecord.getBytes(StandardCharsets.UTF_8));
        } catch (IOException ignored) {
            return false;
        }
        return readRequestFile(requestFile, requestRecord);
    }

    private static boolean readRequestFile(File requestFile, String requestRecord) {
        try (FileInputStream inputStream = new FileInputStream(requestFile)) {
            byte[] requestBytes = new byte[(int) requestFile.length()];
            int bytesRead = inputStream.read(requestBytes);
            String readback = new String(requestBytes, 0, Math.max(bytesRead, 0), StandardCharsets.UTF_8);
            return requestRecord.equals(readback);
        } catch (IOException ignored) {
            return false;
        }
    }

    private static final class RequestQueueState {
        final String requestQueueState;
        final String requestReadbackState;
        final String requestDrainState;

        private RequestQueueState(
            String requestQueueState,
            String requestReadbackState,
            String requestDrainState
        ) {
            this.requestQueueState = requestQueueState;
            this.requestReadbackState = requestReadbackState;
            this.requestDrainState = requestDrainState;
        }

        static RequestQueueState recorded() {
            return new RequestQueueState(
                REQUEST_QUEUE_RECORDED,
                REQUEST_READBACK_OBSERVED,
                REQUEST_DRAIN_RECORDED
            );
        }

        static RequestQueueState unavailable() {
            return new RequestQueueState(
                REQUEST_QUEUE_UNAVAILABLE,
                REQUEST_READBACK_UNAVAILABLE,
                REQUEST_DRAIN_UNAVAILABLE
            );
        }
    }
}
