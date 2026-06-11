package ca.ocentra.parent.agent;

import android.content.Context;
import android.os.Bundle;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;

public final class AppGameAndroidChildRuntimeDeliveryProof {
    public static final String SCHEMA_VERSION = "app-game-android-child-runtime-local-delivery-intake-proof";
    public static final String FIELD_DELIVERY_INTAKE_STATE = "deliveryIntakeState";
    public static final String FIELD_DELIVERY_READBACK_STATE = "deliveryReadbackState";
    public static final String FIELD_DELIVERY_QUEUE_STATE = "deliveryQueueState";
    public static final String FIELD_DELIVERY_DRAIN_STATE = "deliveryDrainState";
    public static final String DELIVERY_INTAKE_PACKAGE_LOCAL_RECORDED =
        "package-local-delivery-intake-recorded";
    public static final String DELIVERY_INTAKE_PACKAGE_LOCAL_UNAVAILABLE =
        "package-local-delivery-intake-unavailable";
    public static final String DELIVERY_READBACK_PACKAGE_LOCAL_OBSERVED =
        "package-local-delivery-readback-observed";
    public static final String DELIVERY_READBACK_PACKAGE_LOCAL_UNAVAILABLE =
        "package-local-delivery-readback-unavailable";
    public static final String DELIVERY_QUEUE_PACKAGE_LOCAL_RECORDED =
        "package-local-delivery-queue-recorded";
    public static final String DELIVERY_QUEUE_PACKAGE_LOCAL_UNAVAILABLE =
        "package-local-delivery-queue-unavailable";
    public static final String DELIVERY_DRAIN_PACKAGE_LOCAL_RECORDED =
        "package-local-delivery-drain-recorded";
    public static final String DELIVERY_DRAIN_PACKAGE_LOCAL_UNAVAILABLE =
        "package-local-delivery-drain-unavailable";
    public static final String ACTION_LOCAL_DELIVERY_INTAKE_PROOF =
        "ca.ocentra.parent.agent.APP_GAME_CHILD_RUNTIME_DELIVERY_INTAKE_PROOF";
    private static final String DELIVERY_DIR_NAME = "app-game-child-runtime-deliveries";
    private static final String DELIVERY_FILE_NAME = "delivery-intake-proof-state.txt";
    private static final String DELIVERY_QUEUE_FILE_NAME = "delivery-queue-proof-state.txt";
    private static final String DELIVERY_DRAIN_FILE_NAME = "delivery-drain-proof-state.txt";
    private static final String DELIVERY_RECORD =
        "deliveryId=android-child-runtime-package-local-delivery-intake-ref\n";
    private static final String DELIVERY_QUEUE_RECORD =
        "deliveryQueueId=android-child-runtime-package-local-delivery-queue-ref\n";
    private static final String DELIVERY_DRAIN_RECORD =
        "deliveryDrainId=android-child-runtime-package-local-delivery-drain-ref\n";

    private AppGameAndroidChildRuntimeDeliveryProof() {}

    public static Bundle createChildRuntimeDeliveryBundle(Context context) {
        Bundle status = new Bundle();
        LocalDeliveryProofState localDeliveryProofState = readPackageLocalDeliveryIntakeState(context);
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString(FIELD_DELIVERY_INTAKE_STATE, localDeliveryProofState.deliveryIntakeState);
        status.putString(FIELD_DELIVERY_READBACK_STATE, localDeliveryProofState.deliveryReadbackState);
        status.putString(FIELD_DELIVERY_QUEUE_STATE, localDeliveryProofState.deliveryQueueState);
        status.putString(FIELD_DELIVERY_DRAIN_STATE, localDeliveryProofState.deliveryDrainState);
        status.putStringArray(
            "proofRefs",
            new String[] {
                "android-child-runtime-package-local-delivery-intake-ref",
                "android-child-runtime-package-local-delivery-readback-ref",
                "android-child-runtime-package-local-delivery-queue-ref",
                "android-child-runtime-package-local-delivery-drain-ref",
                "android-child-runtime-package-local-delivery-receiver-ref",
                "android-child-runtime-package-local-delivery-activity-trigger-ref"
            }
        );
        status.putStringArray(
            "openGaps",
            new String[] {
                "android-child-runtime-service-delivery-ingestion-not-proved",
                "android-provider-delivery-not-executed",
                "android-platform-delivery-channel-not-proved-outside-package",
                "android-adapter-dispatch-not-proved",
                "android-platform-enforcement-not-proved",
                "android-raw-private-source-rows-not-included"
            }
        );
        status.putBoolean("providerDeliveryExecuted", false);
        status.putBoolean("serviceDeliveryIngested", false);
        status.putBoolean("adapterDispatchClaimed", false);
        status.putBoolean("platformEnforcementClaimed", false);
        status.putBoolean("rawPrivateSourceRowsIncluded", false);
        return status;
    }

    public static void recordPackageLocalDeliveryIntake(Context context) {
        File deliveryDir = internalDeliveryDirectory(context);
        if (deliveryDir == null) {
            return;
        }
        writeAndReadDeliveryFile(new File(deliveryDir, DELIVERY_FILE_NAME), DELIVERY_RECORD);
        writeAndReadDeliveryFile(new File(deliveryDir, DELIVERY_QUEUE_FILE_NAME), DELIVERY_QUEUE_RECORD);
        writeAndReadDeliveryFile(new File(deliveryDir, DELIVERY_DRAIN_FILE_NAME), DELIVERY_DRAIN_RECORD);
        AppGameAndroidChildRuntimeTransportReceiptProof.recordPackageLocalReceiptChannel(context);
    }

    private static LocalDeliveryProofState readPackageLocalDeliveryIntakeState(Context context) {
        File deliveryDir = internalDeliveryDirectory(context);
        if (deliveryDir == null) {
            return LocalDeliveryProofState.unavailable();
        }
        File deliveryFile = new File(deliveryDir, DELIVERY_FILE_NAME);
        File deliveryQueueFile = new File(deliveryDir, DELIVERY_QUEUE_FILE_NAME);
        File deliveryDrainFile = new File(deliveryDir, DELIVERY_DRAIN_FILE_NAME);
        if (
            deliveryFile.exists() &&
            readDeliveryFile(deliveryFile, DELIVERY_RECORD) &&
            deliveryQueueFile.exists() &&
            readDeliveryFile(deliveryQueueFile, DELIVERY_QUEUE_RECORD) &&
            deliveryDrainFile.exists() &&
            readDeliveryFile(deliveryDrainFile, DELIVERY_DRAIN_RECORD)
        ) {
            return LocalDeliveryProofState.recorded();
        }
        return LocalDeliveryProofState.unavailable();
    }

    private static File internalDeliveryDirectory(Context context) {
        File filesDir = context.getFilesDir();
        if (filesDir == null || (!filesDir.exists() && !filesDir.mkdirs())) {
            return null;
        }
        File deliveryDir = new File(filesDir, DELIVERY_DIR_NAME);
        if (!deliveryDir.exists() && !deliveryDir.mkdirs()) {
            return null;
        }
        return deliveryDir;
    }

    private static boolean writeAndReadDeliveryFile(File deliveryFile, String deliveryRecord) {
        try (FileOutputStream outputStream = new FileOutputStream(deliveryFile, false)) {
            outputStream.write(deliveryRecord.getBytes(StandardCharsets.UTF_8));
        } catch (IOException ignored) {
            return false;
        }
        return readDeliveryFile(deliveryFile, deliveryRecord);
    }

    private static boolean readDeliveryFile(File deliveryFile, String deliveryRecord) {
        try (FileInputStream inputStream = new FileInputStream(deliveryFile)) {
            byte[] deliveryBytes = new byte[(int) deliveryFile.length()];
            int bytesRead = inputStream.read(deliveryBytes);
            String readback = new String(deliveryBytes, 0, Math.max(bytesRead, 0), StandardCharsets.UTF_8);
            return deliveryRecord.equals(readback);
        } catch (IOException ignored) {
            return false;
        }
    }

    private static final class LocalDeliveryProofState {
        final String deliveryIntakeState;
        final String deliveryReadbackState;
        final String deliveryQueueState;
        final String deliveryDrainState;

        private LocalDeliveryProofState(
            String deliveryIntakeState,
            String deliveryReadbackState,
            String deliveryQueueState,
            String deliveryDrainState
        ) {
            this.deliveryIntakeState = deliveryIntakeState;
            this.deliveryReadbackState = deliveryReadbackState;
            this.deliveryQueueState = deliveryQueueState;
            this.deliveryDrainState = deliveryDrainState;
        }

        static LocalDeliveryProofState recorded() {
            return new LocalDeliveryProofState(
                DELIVERY_INTAKE_PACKAGE_LOCAL_RECORDED,
                DELIVERY_READBACK_PACKAGE_LOCAL_OBSERVED,
                DELIVERY_QUEUE_PACKAGE_LOCAL_RECORDED,
                DELIVERY_DRAIN_PACKAGE_LOCAL_RECORDED
            );
        }

        static LocalDeliveryProofState unavailable() {
            return new LocalDeliveryProofState(
                DELIVERY_INTAKE_PACKAGE_LOCAL_UNAVAILABLE,
                DELIVERY_READBACK_PACKAGE_LOCAL_UNAVAILABLE,
                DELIVERY_QUEUE_PACKAGE_LOCAL_UNAVAILABLE,
                DELIVERY_DRAIN_PACKAGE_LOCAL_UNAVAILABLE
            );
        }
    }
}
