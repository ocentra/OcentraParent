package ca.ocentra.parent.agent;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

import android.app.Application;
import android.content.Context;
import android.content.Intent;
import android.os.Bundle;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;

import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;

import org.robolectric.RobolectricTestRunner;
import org.robolectric.RuntimeEnvironment;

@RunWith(RobolectricTestRunner.class)
public final class AppGameAndroidChildRuntimeDeliveryProofTest {
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

    @Before
    public void clearPackageLocalDeliveryState() {
        deleteRecursively(deliveryDirectory(application()));
    }

    @Test
    public void localDeliveryQueueAndDrainAreFilesystemBackedIdempotentAndUnclaimed() throws IOException {
        Context context = application();
        AppGameAndroidChildRuntimeDeliveryProof.recordPackageLocalDeliveryIntake(context);
        AppGameAndroidChildRuntimeDeliveryProof.recordPackageLocalDeliveryIntake(context);

        Bundle status = AppGameAndroidChildRuntimeDeliveryProof
            .createChildRuntimeDeliveryBundle(context);

        assertEquals(
            AppGameAndroidChildRuntimeDeliveryProof.DELIVERY_INTAKE_PACKAGE_LOCAL_RECORDED,
            status.getString(AppGameAndroidChildRuntimeDeliveryProof.FIELD_DELIVERY_INTAKE_STATE)
        );
        assertEquals(
            AppGameAndroidChildRuntimeDeliveryProof.DELIVERY_READBACK_PACKAGE_LOCAL_OBSERVED,
            status.getString(AppGameAndroidChildRuntimeDeliveryProof.FIELD_DELIVERY_READBACK_STATE)
        );
        assertEquals(
            AppGameAndroidChildRuntimeDeliveryProof.DELIVERY_QUEUE_PACKAGE_LOCAL_RECORDED,
            status.getString(AppGameAndroidChildRuntimeDeliveryProof.FIELD_DELIVERY_QUEUE_STATE)
        );
        assertEquals(
            AppGameAndroidChildRuntimeDeliveryProof.DELIVERY_DRAIN_PACKAGE_LOCAL_RECORDED,
            status.getString(AppGameAndroidChildRuntimeDeliveryProof.FIELD_DELIVERY_DRAIN_STATE)
        );
        assertArrayEquals(
            new String[] {
                "android-child-runtime-package-local-delivery-intake-ref",
                "android-child-runtime-package-local-delivery-readback-ref",
                "android-child-runtime-package-local-delivery-queue-ref",
                "android-child-runtime-package-local-delivery-drain-ref",
                "android-child-runtime-package-local-delivery-receiver-ref",
                "android-child-runtime-package-local-delivery-activity-trigger-ref"
            },
            status.getStringArray("proofRefs")
        );
        assertFalse(status.getBoolean("serviceDeliveryIngested"));
        assertFalse(status.getBoolean("providerDeliveryExecuted"));
        assertFalse(status.getBoolean("adapterDispatchClaimed"));
        assertFalse(status.getBoolean("platformEnforcementClaimed"));
        assertFalse(status.getBoolean("rawPrivateSourceRowsIncluded"));

        assertEquals(DELIVERY_RECORD, readFile(new File(deliveryDirectory(context), DELIVERY_FILE_NAME)));
        assertEquals(
            DELIVERY_QUEUE_RECORD,
            readFile(new File(deliveryDirectory(context), DELIVERY_QUEUE_FILE_NAME))
        );
        assertEquals(
            DELIVERY_DRAIN_RECORD,
            readFile(new File(deliveryDirectory(context), DELIVERY_DRAIN_FILE_NAME))
        );
    }

    @Test
    public void receiverRequiresExactLocalDeliveryAction() throws IOException {
        Context context = application();
        AppGameAndroidChildRuntimeDeliveryReceiver receiver =
            new AppGameAndroidChildRuntimeDeliveryReceiver();

        receiver.onReceive(context, null);
        receiver.onReceive(context, new Intent("unrelated-action"));
        assertFalse(deliveryDirectory(context).exists());

        receiver.onReceive(
            context,
            new Intent(AppGameAndroidChildRuntimeDeliveryProof.ACTION_LOCAL_DELIVERY_INTAKE_PROOF)
        );

        Bundle status = AppGameAndroidChildRuntimeDeliveryProof
            .createChildRuntimeDeliveryBundle(context);
        assertEquals(
            AppGameAndroidChildRuntimeDeliveryProof.DELIVERY_QUEUE_PACKAGE_LOCAL_RECORDED,
            status.getString(AppGameAndroidChildRuntimeDeliveryProof.FIELD_DELIVERY_QUEUE_STATE)
        );
        assertTrue(new File(deliveryDirectory(context), DELIVERY_DRAIN_FILE_NAME).isFile());
    }

    @Test
    public void nullContextFailsClosedWithoutDeliveryClaims() {
        Bundle status = AppGameAndroidChildRuntimeDeliveryProof
            .createChildRuntimeDeliveryBundle(null);

        assertEquals(
            AppGameAndroidChildRuntimeDeliveryProof.DELIVERY_INTAKE_PACKAGE_LOCAL_UNAVAILABLE,
            status.getString(AppGameAndroidChildRuntimeDeliveryProof.FIELD_DELIVERY_INTAKE_STATE)
        );
        assertEquals(
            AppGameAndroidChildRuntimeDeliveryProof.DELIVERY_QUEUE_PACKAGE_LOCAL_UNAVAILABLE,
            status.getString(AppGameAndroidChildRuntimeDeliveryProof.FIELD_DELIVERY_QUEUE_STATE)
        );
        assertEquals(
            AppGameAndroidChildRuntimeDeliveryProof.DELIVERY_DRAIN_PACKAGE_LOCAL_UNAVAILABLE,
            status.getString(AppGameAndroidChildRuntimeDeliveryProof.FIELD_DELIVERY_DRAIN_STATE)
        );
        assertFalse(status.getBoolean("serviceDeliveryIngested"));
        assertFalse(status.getBoolean("providerDeliveryExecuted"));
        assertFalse(status.getBoolean("adapterDispatchClaimed"));
        assertFalse(status.getBoolean("platformEnforcementClaimed"));
        assertFalse(status.getBoolean("rawPrivateSourceRowsIncluded"));
    }

    @Test
    public void deliveryStorageFailureReportsUnavailableAndNoClaims() {
        Context context = application();
        assertTrue(deliveryDirectory(context).createNewFile());

        AppGameAndroidChildRuntimeDeliveryProof.recordPackageLocalDeliveryIntake(context);
        Bundle status = AppGameAndroidChildRuntimeDeliveryProof
            .createChildRuntimeDeliveryBundle(context);

        assertEquals(
            AppGameAndroidChildRuntimeDeliveryProof.DELIVERY_INTAKE_PACKAGE_LOCAL_UNAVAILABLE,
            status.getString(AppGameAndroidChildRuntimeDeliveryProof.FIELD_DELIVERY_INTAKE_STATE)
        );
        assertEquals(
            AppGameAndroidChildRuntimeDeliveryProof.DELIVERY_READBACK_PACKAGE_LOCAL_UNAVAILABLE,
            status.getString(AppGameAndroidChildRuntimeDeliveryProof.FIELD_DELIVERY_READBACK_STATE)
        );
        assertFalse(status.getBoolean("serviceDeliveryIngested"));
        assertFalse(status.getBoolean("providerDeliveryExecuted"));
        assertFalse(status.getBoolean("adapterDispatchClaimed"));
        assertFalse(status.getBoolean("platformEnforcementClaimed"));
        assertFalse(status.getBoolean("rawPrivateSourceRowsIncluded"));
    }

    private static Application application() {
        return RuntimeEnvironment.getApplication();
    }

    private static File deliveryDirectory(Context context) {
        return new File(context.getFilesDir(), DELIVERY_DIR_NAME);
    }

    private static String readFile(File file) throws IOException {
        try (FileInputStream input = new FileInputStream(file)) {
            byte[] bytes = new byte[(int) file.length()];
            int bytesRead = input.read(bytes);
            return new String(bytes, 0, Math.max(bytesRead, 0), StandardCharsets.UTF_8);
        }
    }

    private static void deleteRecursively(File path) {
        if (!path.exists()) {
            return;
        }
        File[] children = path.listFiles();
        if (children != null) {
            for (File child : children) {
                deleteRecursively(child);
            }
        }
        path.delete();
    }
}
