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
public final class AppGameAndroidChildRuntimeLocalReceiptAckProofTest {
    private static final String RECEIPT_DIR_NAME = "app-game-child-runtime-receipts";
    private static final String RECEIPT_ACK_FILE_NAME = "receipt-ack-proof-state.txt";
    private static final String RECEIPT_CHANNEL_FILE_NAME = "receipt-channel-proof-state.txt";
    private static final String RECEIPT_ACK_RECORD =
        "receiptAckId=android-child-runtime-local-receipt-ack-ref\n";

    @Before
    public void clearPackageLocalReceiptState() {
        deleteRecursively(receiptDirectory(application()));
    }

    @Test
    public void localReceiptAckWriteReadbackIsPackageLocalAndUnclaimed() throws IOException {
        Bundle status = AppGameAndroidChildRuntimeTransportReceiptProof
            .createChildRuntimeTransportReceiptBundle(application());

        assertEquals(
            AppGameAndroidChildRuntimeTransportReceiptProof.RECEIPT_ACK_WAITING_FOR_RUNTIME,
            status.getString(AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_ACK_STATE)
        );
        assertEquals(
            AppGameAndroidChildRuntimeTransportReceiptProof.RECEIPT_LOCAL_ACK_RECORDED,
            status.getString(AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_LOCAL_ACK_STATE)
        );
        assertEquals(
            AppGameAndroidChildRuntimeTransportReceiptProof.RECEIPT_LOCAL_ACK_READBACK_OBSERVED,
            status.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_LOCAL_ACK_READBACK_STATE
            )
        );
        assertArrayEquals(
            new String[] {
                "android-child-runtime-activity-transport-ref",
                "android-child-runtime-internal-receipt-store-ref",
                "android-child-runtime-local-receipt-write-ref",
                "android-child-runtime-local-receipt-readback-ref",
                "android-child-runtime-local-receipt-ack-write-ref",
                "android-child-runtime-local-receipt-ack-readback-ref",
                "android-child-runtime-package-local-receipt-channel-ref"
            },
            status.getStringArray("proofRefs")
        );
        assertArrayEquals(
            new String[] {
                "android-child-runtime-transport-not-executed",
                "android-child-runtime-receipt-not-ingested-by-service",
                "android-provider-delivery-not-executed",
                "android-platform-delivery-channel-not-proved"
            },
            status.getStringArray("openGaps")
        );
        assertFalse(status.getBoolean("runtimeTransportExecuted"));
        assertFalse(status.getBoolean("runtimeReceiptIngested"));
        assertFalse(status.getBoolean("providerDeliveryExecuted"));
        assertFalse(status.getBoolean("platformDeliveryChannelClaimed"));
        assertFalse(status.getBoolean("adapterDispatchClaimed"));
        assertFalse(status.getBoolean("platformEnforcementClaimed"));
        assertFalse(status.getBoolean("rawPrivateSourceRowsIncluded"));

        File ackFile = new File(receiptDirectory(application()), RECEIPT_ACK_FILE_NAME);
        assertTrue(ackFile.isFile());
        assertEquals(RECEIPT_ACK_RECORD, readFile(ackFile));
        assertFalse(new File(receiptDirectory(application()), RECEIPT_CHANNEL_FILE_NAME).exists());
    }

    @Test
    public void nullContextFailsClosedWithoutReceiptAckClaims() {
        Bundle status = AppGameAndroidChildRuntimeTransportReceiptProof
            .createChildRuntimeTransportReceiptBundle(null);

        assertEquals(
            AppGameAndroidChildRuntimeTransportReceiptProof.TRANSPORT_CHANNEL_ACTIVITY_UNAVAILABLE,
            status.getString(AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_TRANSPORT_CHANNEL_STATE)
        );
        assertEquals(
            AppGameAndroidChildRuntimeTransportReceiptProof.RECEIPT_STORE_INTERNAL_UNAVAILABLE,
            status.getString(AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_STORE_STATE)
        );
        assertEquals(
            AppGameAndroidChildRuntimeTransportReceiptProof.RECEIPT_APPEND_LOCAL_UNAVAILABLE,
            status.getString(AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_APPEND_STATE)
        );
        assertEquals(
            AppGameAndroidChildRuntimeTransportReceiptProof.RECEIPT_LOCAL_ACK_UNAVAILABLE,
            status.getString(AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_LOCAL_ACK_STATE)
        );
        assertEquals(
            AppGameAndroidChildRuntimeTransportReceiptProof.RECEIPT_LOCAL_ACK_READBACK_UNAVAILABLE,
            status.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_LOCAL_ACK_READBACK_STATE
            )
        );
        assertEquals(
            AppGameAndroidChildRuntimeTransportReceiptProof.RECEIPT_CHANNEL_PACKAGE_LOCAL_UNAVAILABLE,
            status.getString(AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_CHANNEL_STATE)
        );
        assertFalse(status.getBoolean("runtimeTransportExecuted"));
        assertFalse(status.getBoolean("runtimeReceiptIngested"));
        assertFalse(status.getBoolean("providerDeliveryExecuted"));
        assertFalse(status.getBoolean("platformDeliveryChannelClaimed"));
        assertFalse(status.getBoolean("adapterDispatchClaimed"));
        assertFalse(status.getBoolean("platformEnforcementClaimed"));
        assertFalse(status.getBoolean("rawPrivateSourceRowsIncluded"));
    }

    @Test
    public void matchingReceiverRecordsLocalAckWithoutServiceReceipt() throws IOException {
        Context context = application();
        AppGameAndroidChildRuntimeReceiptReceiver receiver =
            new AppGameAndroidChildRuntimeReceiptReceiver();

        receiver.onReceive(context, null);
        receiver.onReceive(context, new Intent("unrelated-action"));
        File ackFile = new File(receiptDirectory(context), RECEIPT_ACK_FILE_NAME);
        assertFalse(ackFile.exists());

        receiver.onReceive(
            context,
            new Intent(AppGameAndroidChildRuntimeTransportReceiptProof.ACTION_LOCAL_RECEIPT_CHANNEL_PROOF)
        );

        assertTrue(ackFile.isFile());
        assertEquals(RECEIPT_ACK_RECORD, readFile(ackFile));
        Bundle status = AppGameAndroidChildRuntimeTransportReceiptProof
            .createChildRuntimeTransportReceiptBundle(context);
        assertEquals(
            AppGameAndroidChildRuntimeTransportReceiptProof.RECEIPT_LOCAL_ACK_RECORDED,
            status.getString(AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_LOCAL_ACK_STATE)
        );
        assertEquals(
            AppGameAndroidChildRuntimeTransportReceiptProof.RECEIPT_LOCAL_ACK_READBACK_OBSERVED,
            status.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_LOCAL_ACK_READBACK_STATE
            )
        );
        assertFalse(status.getBoolean("runtimeReceiptIngested"));
        assertFalse(status.getBoolean("providerDeliveryExecuted"));
        assertFalse(status.getBoolean("platformDeliveryChannelClaimed"));
        assertFalse(status.getBoolean("adapterDispatchClaimed"));
        assertFalse(status.getBoolean("platformEnforcementClaimed"));
    }

    @Test
    public void localReceiptAckStorageFailureReportsUnavailableAndNoClaims() throws IOException {
        Context context = application();
        assertTrue(receiptDirectory(context).createNewFile());

        Bundle status = AppGameAndroidChildRuntimeTransportReceiptProof
            .createChildRuntimeTransportReceiptBundle(context);

        assertEquals(
            AppGameAndroidChildRuntimeTransportReceiptProof.RECEIPT_LOCAL_ACK_UNAVAILABLE,
            status.getString(AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_LOCAL_ACK_STATE)
        );
        assertEquals(
            AppGameAndroidChildRuntimeTransportReceiptProof.RECEIPT_LOCAL_ACK_READBACK_UNAVAILABLE,
            status.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_LOCAL_ACK_READBACK_STATE
            )
        );
        assertEquals(
            AppGameAndroidChildRuntimeTransportReceiptProof.RECEIPT_CHANNEL_PACKAGE_LOCAL_UNAVAILABLE,
            status.getString(AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_CHANNEL_STATE)
        );
        assertFalse(status.getBoolean("runtimeTransportExecuted"));
        assertFalse(status.getBoolean("runtimeReceiptIngested"));
        assertFalse(status.getBoolean("providerDeliveryExecuted"));
        assertFalse(status.getBoolean("platformDeliveryChannelClaimed"));
        assertFalse(status.getBoolean("adapterDispatchClaimed"));
        assertFalse(status.getBoolean("platformEnforcementClaimed"));
        assertFalse(status.getBoolean("rawPrivateSourceRowsIncluded"));
    }

    private static Application application() {
        return RuntimeEnvironment.getApplication();
    }

    private static File receiptDirectory(Context context) {
        return new File(context.getFilesDir(), RECEIPT_DIR_NAME);
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
