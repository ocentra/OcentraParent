package ca.ocentra.parent.agent;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

import android.os.Bundle;

import org.junit.Test;
import org.junit.runner.RunWith;

import org.robolectric.RobolectricTestRunner;

@RunWith(RobolectricTestRunner.class)
public final class AppGameAndroidUsageEventsCapabilityProofTest {
    @Test
    public void capabilityBundleReportsLiveChildRuntimeConsumerTopology() {
        Bundle status = AppGameAndroidUsageEventsCapabilityProof.createUsageEventsCapabilityBundle();

        assertEquals(
            AppGameAndroidUsageEventsCapabilityProof.SCHEMA_VERSION,
            status.getString("schemaVersion")
        );
        assertEquals(
            AppGameAndroidUsageEventsCapabilityProof.PACKAGE_ID,
            status.getString("packageId")
        );
        assertEquals(
            AppGameAndroidUsageEventsCapabilityProof.NATIVE_BRIDGE_CLASS,
            status.getString("nativeBridgeClass")
        );
        assertEquals(
            AppGameAndroidUsageEventsCapabilityProof.BRIDGE_STATE,
            status.getString(AppGameAndroidUsageEventsCapabilityProof.FIELD_USAGE_EVENTS_BRIDGE_STATE)
        );
        assertEquals(
            AppGameAndroidUsageEventsCapabilityProof.PERMISSION_STATE,
            status.getString("permissionState")
        );
        assertEquals(
            AppGameAndroidUsageEventsCapabilityProof.EVENT_COLLECTION_STATE,
            status.getString("eventCollectionState")
        );
        assertEquals(
            AppGameAndroidUsageEventsCapabilityProof.REPLAY_CONSUMER_STATE,
            status.getString("replayConsumerState")
        );
        assertTrue(status.getBoolean(
            AppGameAndroidUsageEventsCapabilityProof.FIELD_CHILD_RUNTIME_SERVICE_CONSUMER_REACHABLE
        ));
        assertArrayEquals(
            new String[] {
                AppGameAndroidUsageEventsCapabilityProof.COMMAND_USAGE_EVENTS_CAPABILITY_GET,
                AppGameAndroidUsageEventsCapabilityProof.COMMAND_USAGE_EVENTS_REPLAY_BOUNDARY_GET
            },
            status.getStringArray("commands")
        );
        assertArrayEquals(
            new String[] {
                AppGameAndroidUsageEventsCapabilityProof.EVENT_USAGE_EVENTS_CAPABILITY_REPORTED,
                AppGameAndroidUsageEventsCapabilityProof.EVENT_USAGE_EVENTS_REPLAY_BOUNDARY_REPORTED
            },
            status.getStringArray("events")
        );
        assertArrayEquals(new String[0], status.getStringArray("proofRefs"));
        assertArrayEquals(
            new String[] {
                "android-usage-stats-settings-grant-not-proved",
                "android-usage-events-runtime-collection-not-proved",
                "android-usage-events-child-runtime-replay-proof-not-proved",
                "android-child-runtime-delivery-not-proved",
                "android-platform-enforcement-not-proved"
            },
            status.getStringArray("openGaps")
        );
        assertFalse(status.getBoolean("rawUsageEventsStored"));
        assertFalse(status.getBoolean("packageNamesStored"));
        assertFalse(status.getBoolean("rawActivityRowsStored"));
        assertFalse(status.getBoolean("adapterDispatchClaimed"));
        assertFalse(status.getBoolean("platformEnforcementClaimed"));
        assertFalse(status.getBoolean("childDeviceDeliveryClaimed"));
    }
}
