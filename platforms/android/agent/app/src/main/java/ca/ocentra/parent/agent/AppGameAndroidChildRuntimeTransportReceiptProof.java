package ca.ocentra.parent.agent;

import android.content.Context;
import android.os.Bundle;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;

public final class AppGameAndroidChildRuntimeTransportReceiptProof {
    public static final String SCHEMA_VERSION = "app-game-android-child-runtime-transport-receipt-proof";
    public static final String PACKAGE_ID = "ca.ocentra.parent.agent";
    public static final String NATIVE_BRIDGE_CLASS =
        "ca.ocentra.parent.agent.AppGameAndroidChildRuntimeTransportReceiptProof";
    public static final String FIELD_TRANSPORT_CHANNEL_STATE = "transportChannelState";
    public static final String FIELD_RECEIPT_STORE_STATE = "receiptStoreState";
    public static final String FIELD_RECEIPT_ACK_STATE = "receiptAckState";
    public static final String FIELD_RECEIPT_APPEND_STATE = "receiptAppendState";
    public static final String FIELD_RECEIPT_READBACK_STATE = "receiptReadbackState";
    public static final String FIELD_RECEIPT_LOCAL_ACK_STATE = "receiptLocalAckState";
    public static final String FIELD_RECEIPT_LOCAL_ACK_READBACK_STATE = "receiptLocalAckReadbackState";
    public static final String FIELD_RECEIPT_CHANNEL_STATE = "receiptChannelState";
    public static final String TRANSPORT_CHANNEL_ACTIVITY_VISIBLE = "activity-visible-transport-channel";
    public static final String TRANSPORT_CHANNEL_ACTIVITY_UNAVAILABLE = "activity-unavailable-transport-channel";
    public static final String RECEIPT_STORE_INTERNAL_AVAILABLE = "internal-receipt-store-available";
    public static final String RECEIPT_STORE_INTERNAL_UNAVAILABLE = "internal-receipt-store-unavailable";
    public static final String RECEIPT_ACK_WAITING_FOR_RUNTIME = "receipt-ack-waiting-for-runtime";
    public static final String RECEIPT_APPEND_LOCAL_RECORDED = "local-receipt-append-recorded";
    public static final String RECEIPT_APPEND_LOCAL_UNAVAILABLE = "local-receipt-append-unavailable";
    public static final String RECEIPT_READBACK_LOCAL_OBSERVED = "local-receipt-readback-observed";
    public static final String RECEIPT_READBACK_LOCAL_UNAVAILABLE = "local-receipt-readback-unavailable";
    public static final String RECEIPT_LOCAL_ACK_RECORDED = "local-receipt-ack-recorded";
    public static final String RECEIPT_LOCAL_ACK_UNAVAILABLE = "local-receipt-ack-unavailable";
    public static final String RECEIPT_LOCAL_ACK_READBACK_OBSERVED = "local-receipt-ack-readback-observed";
    public static final String RECEIPT_LOCAL_ACK_READBACK_UNAVAILABLE = "local-receipt-ack-readback-unavailable";
    public static final String RECEIPT_CHANNEL_PACKAGE_LOCAL_RECORDED = "package-local-receipt-channel-recorded";
    public static final String RECEIPT_CHANNEL_PACKAGE_LOCAL_UNAVAILABLE = "package-local-receipt-channel-unavailable";
    public static final String ACTION_LOCAL_RECEIPT_CHANNEL_PROOF =
        "ca.ocentra.parent.agent.APP_GAME_CHILD_RUNTIME_RECEIPT_CHANNEL_PROOF";
    public static final String COMMAND_CHILD_RUNTIME_RECEIPT_GET =
        "app-game.android.child-runtime-transport-receipt.get";
    public static final String EVENT_CHILD_RUNTIME_RECEIPT_REPORTED =
        "app-game.android.child-runtime-transport-receipt.reported";
    private static final String RECEIPT_DIR_NAME = "app-game-child-runtime-receipts";
    private static final String RECEIPT_FILE_NAME = "receipt-proof-state.txt";
    private static final String RECEIPT_ACK_FILE_NAME = "receipt-ack-proof-state.txt";
    private static final String RECEIPT_CHANNEL_FILE_NAME = "receipt-channel-proof-state.txt";
    private static final String RECEIPT_RECORD = "receiptId=android-child-runtime-local-receipt-ref\n";
    private static final String RECEIPT_ACK_RECORD = "receiptAckId=android-child-runtime-local-receipt-ack-ref\n";
    private static final String RECEIPT_CHANNEL_RECORD =
        "receiptChannelId=android-child-runtime-package-local-receipt-channel-ref\n";

    private AppGameAndroidChildRuntimeTransportReceiptProof() {}

    public static Bundle createChildRuntimeTransportReceiptBundle(Context context) {
        Bundle status = new Bundle();
        boolean internalStoreAvailable = internalReceiptStoreAvailable(context);
        LocalReceiptProofState localReceiptProofState = writeAndReadLocalReceiptProof(context);
        LocalReceiptAckProofState localReceiptAckProofState = writeAndReadLocalReceiptAckProof(context);
        String receiptChannelState = readPackageLocalReceiptChannelState(context);
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString("packageId", PACKAGE_ID);
        status.putString("nativeBridgeClass", NATIVE_BRIDGE_CLASS);
        status.putString(FIELD_TRANSPORT_CHANNEL_STATE, TRANSPORT_CHANNEL_ACTIVITY_VISIBLE);
        status.putString(
            FIELD_RECEIPT_STORE_STATE,
            internalStoreAvailable ? RECEIPT_STORE_INTERNAL_AVAILABLE : RECEIPT_STORE_INTERNAL_UNAVAILABLE
        );
        status.putString(FIELD_RECEIPT_ACK_STATE, RECEIPT_ACK_WAITING_FOR_RUNTIME);
        status.putString(FIELD_RECEIPT_APPEND_STATE, localReceiptProofState.receiptAppendState);
        status.putString(FIELD_RECEIPT_READBACK_STATE, localReceiptProofState.receiptReadbackState);
        status.putString(FIELD_RECEIPT_LOCAL_ACK_STATE, localReceiptAckProofState.receiptLocalAckState);
        status.putString(
            FIELD_RECEIPT_LOCAL_ACK_READBACK_STATE,
            localReceiptAckProofState.receiptLocalAckReadbackState
        );
        status.putString(FIELD_RECEIPT_CHANNEL_STATE, receiptChannelState);
        status.putStringArray("commands", new String[] { COMMAND_CHILD_RUNTIME_RECEIPT_GET });
        status.putStringArray("events", new String[] { EVENT_CHILD_RUNTIME_RECEIPT_REPORTED });
        status.putStringArray(
            "proofRefs",
            new String[] {
                "android-child-runtime-activity-transport-ref",
                "android-child-runtime-internal-receipt-store-ref",
                "android-child-runtime-local-receipt-write-ref",
                "android-child-runtime-local-receipt-readback-ref",
                "android-child-runtime-local-receipt-ack-write-ref",
                "android-child-runtime-local-receipt-ack-readback-ref",
                "android-child-runtime-package-local-receipt-channel-ref"
            }
        );
        status.putStringArray(
            "openGaps",
            new String[] {
                "android-child-runtime-transport-not-executed",
                "android-child-runtime-receipt-not-ingested-by-service",
                "android-provider-delivery-not-executed",
                "android-platform-delivery-channel-not-proved"
            }
        );
        status.putBoolean("runtimeTransportExecuted", false);
        status.putBoolean("runtimeReceiptIngested", false);
        status.putBoolean("providerDeliveryExecuted", false);
        status.putBoolean("platformDeliveryChannelClaimed", false);
        status.putBoolean("adapterDispatchClaimed", false);
        status.putBoolean("platformEnforcementClaimed", false);
        status.putBoolean("rawPrivateSourceRowsIncluded", false);
        return status;
    }

    public static void recordPackageLocalReceiptChannel(Context context) {
        File receiptDir = internalReceiptDirectory(context);
        if (receiptDir == null) {
            return;
        }
        writeAndReadReceiptFile(new File(receiptDir, RECEIPT_CHANNEL_FILE_NAME), RECEIPT_CHANNEL_RECORD);
        writeAndReadReceiptFile(new File(receiptDir, RECEIPT_FILE_NAME), RECEIPT_RECORD);
        writeAndReadReceiptFile(new File(receiptDir, RECEIPT_ACK_FILE_NAME), RECEIPT_ACK_RECORD);
    }

    private static boolean internalReceiptStoreAvailable(Context context) {
        File filesDir = context.getFilesDir();
        return filesDir != null && (filesDir.exists() || filesDir.mkdirs()) && filesDir.canWrite();
    }

    private static LocalReceiptProofState writeAndReadLocalReceiptProof(Context context) {
        File receiptDir = internalReceiptDirectory(context);
        if (receiptDir == null) {
            return LocalReceiptProofState.unavailable();
        }
        File receiptFile = new File(receiptDir, RECEIPT_FILE_NAME);
        if (writeAndReadReceiptFile(receiptFile, RECEIPT_RECORD)) {
            return LocalReceiptProofState.recorded();
        }
        return LocalReceiptProofState.unavailable();
    }

    private static LocalReceiptAckProofState writeAndReadLocalReceiptAckProof(Context context) {
        File receiptDir = internalReceiptDirectory(context);
        if (receiptDir == null) {
            return LocalReceiptAckProofState.unavailable();
        }
        File receiptAckFile = new File(receiptDir, RECEIPT_ACK_FILE_NAME);
        if (writeAndReadReceiptFile(receiptAckFile, RECEIPT_ACK_RECORD)) {
            return LocalReceiptAckProofState.recorded();
        }
        return LocalReceiptAckProofState.unavailable();
    }

    private static String readPackageLocalReceiptChannelState(Context context) {
        File receiptDir = internalReceiptDirectory(context);
        if (receiptDir == null) {
            return RECEIPT_CHANNEL_PACKAGE_LOCAL_UNAVAILABLE;
        }
        File receiptChannelFile = new File(receiptDir, RECEIPT_CHANNEL_FILE_NAME);
        if (!receiptChannelFile.exists()) {
            return RECEIPT_CHANNEL_PACKAGE_LOCAL_UNAVAILABLE;
        }
        if (readReceiptFile(receiptChannelFile, RECEIPT_CHANNEL_RECORD)) {
            return RECEIPT_CHANNEL_PACKAGE_LOCAL_RECORDED;
        }
        return RECEIPT_CHANNEL_PACKAGE_LOCAL_UNAVAILABLE;
    }

    private static File internalReceiptDirectory(Context context) {
        File filesDir = context.getFilesDir();
        if (filesDir == null || (!filesDir.exists() && !filesDir.mkdirs())) {
            return null;
        }
        File receiptDir = new File(filesDir, RECEIPT_DIR_NAME);
        if (!receiptDir.exists() && !receiptDir.mkdirs()) {
            return null;
        }
        return receiptDir;
    }

    private static boolean writeAndReadReceiptFile(File receiptFile, String receiptRecord) {
        try (FileOutputStream outputStream = new FileOutputStream(receiptFile, false)) {
            outputStream.write(receiptRecord.getBytes(StandardCharsets.UTF_8));
        } catch (IOException ignored) {
            return false;
        }
        return readReceiptFile(receiptFile, receiptRecord);
    }

    private static boolean readReceiptFile(File receiptFile, String receiptRecord) {
        try (FileInputStream inputStream = new FileInputStream(receiptFile)) {
            byte[] receiptBytes = new byte[(int) receiptFile.length()];
            int bytesRead = inputStream.read(receiptBytes);
            String readback = new String(receiptBytes, 0, Math.max(bytesRead, 0), StandardCharsets.UTF_8);
            return receiptRecord.equals(readback);
        } catch (IOException ignored) {
            return false;
        }
    }

    private static final class LocalReceiptProofState {
        final String receiptAppendState;
        final String receiptReadbackState;

        private LocalReceiptProofState(String receiptAppendState, String receiptReadbackState) {
            this.receiptAppendState = receiptAppendState;
            this.receiptReadbackState = receiptReadbackState;
        }

        static LocalReceiptProofState recorded() {
            return new LocalReceiptProofState(RECEIPT_APPEND_LOCAL_RECORDED, RECEIPT_READBACK_LOCAL_OBSERVED);
        }

        static LocalReceiptProofState unavailable() {
            return new LocalReceiptProofState(RECEIPT_APPEND_LOCAL_UNAVAILABLE, RECEIPT_READBACK_LOCAL_UNAVAILABLE);
        }
    }

    private static final class LocalReceiptAckProofState {
        final String receiptLocalAckState;
        final String receiptLocalAckReadbackState;

        private LocalReceiptAckProofState(String receiptLocalAckState, String receiptLocalAckReadbackState) {
            this.receiptLocalAckState = receiptLocalAckState;
            this.receiptLocalAckReadbackState = receiptLocalAckReadbackState;
        }

        static LocalReceiptAckProofState recorded() {
            return new LocalReceiptAckProofState(RECEIPT_LOCAL_ACK_RECORDED, RECEIPT_LOCAL_ACK_READBACK_OBSERVED);
        }

        static LocalReceiptAckProofState unavailable() {
            return new LocalReceiptAckProofState(
                RECEIPT_LOCAL_ACK_UNAVAILABLE,
                RECEIPT_LOCAL_ACK_READBACK_UNAVAILABLE
            );
        }
    }
}
