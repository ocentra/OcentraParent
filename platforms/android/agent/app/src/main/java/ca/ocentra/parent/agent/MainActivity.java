package ca.ocentra.parent.agent;

import android.app.Activity;
import android.content.Intent;
import android.graphics.Color;
import android.os.Bundle;
import android.view.Gravity;
import android.widget.TextView;

public final class MainActivity extends Activity {
    public static final String EXTRA_START_SCREEN_CAPTURE_PROOF =
        "ca.ocentra.parent.agent.START_SCREEN_CAPTURE_PROOF";
    public static final String EXTRA_RUN_APP_GAME_RECEIPT_CHANNEL_PROOF =
        "ca.ocentra.parent.agent.RUN_APP_GAME_RECEIPT_CHANNEL_PROOF";
    public static final String EXTRA_RUN_APP_GAME_DELIVERY_INTAKE_PROOF =
        "ca.ocentra.parent.agent.RUN_APP_GAME_DELIVERY_INTAKE_PROOF";
    public static final String EXTRA_RUN_APP_GAME_LOCAL_NOTIFICATION_PROOF =
        "ca.ocentra.parent.agent.RUN_APP_GAME_LOCAL_NOTIFICATION_PROOF";
    public static final String EXTRA_RUN_APP_GAME_LOCAL_NOTIFICATION_ACTION_PROOF =
        "ca.ocentra.parent.agent.RUN_APP_GAME_LOCAL_NOTIFICATION_ACTION_PROOF";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        startForegroundService(new Intent(this, OcentraParentAgentService.class));
        if (getIntent().getBooleanExtra(EXTRA_START_SCREEN_CAPTURE_PROOF, false)) {
            startActivity(new Intent(this, AndroidMediaProjectionCaptureActivity.class));
        }
        if (getIntent().getBooleanExtra(EXTRA_RUN_APP_GAME_RECEIPT_CHANNEL_PROOF, false)) {
            Intent receiptChannelIntent = new Intent(
                AppGameAndroidChildRuntimeTransportReceiptProof.ACTION_LOCAL_RECEIPT_CHANNEL_PROOF
            );
            receiptChannelIntent.setPackage(getPackageName());
            sendBroadcast(receiptChannelIntent);
        }
        if (getIntent().getBooleanExtra(EXTRA_RUN_APP_GAME_DELIVERY_INTAKE_PROOF, false)) {
            Intent deliveryIntakeIntent = new Intent(
                AppGameAndroidChildRuntimeDeliveryProof.ACTION_LOCAL_DELIVERY_INTAKE_PROOF
            );
            deliveryIntakeIntent.setPackage(getPackageName());
            sendBroadcast(deliveryIntakeIntent);
        }
        Bundle appGameChildRuntimeLocalNotification =
            getIntent().getBooleanExtra(EXTRA_RUN_APP_GAME_LOCAL_NOTIFICATION_PROOF, false) ||
                getIntent().getBooleanExtra(EXTRA_RUN_APP_GAME_LOCAL_NOTIFICATION_ACTION_PROOF, false)
                ? AppGameAndroidChildRuntimeLocalNotificationProof.postLocalAppGameNotification(this)
                : AppGameAndroidChildRuntimeLocalNotificationProof.createLocalNotificationBundle(this);
        if (getIntent().getBooleanExtra(EXTRA_RUN_APP_GAME_LOCAL_NOTIFICATION_ACTION_PROOF, false)) {
            AppGameAndroidChildRuntimeLocalNotificationProof.triggerLocalRequestAction(this);
            appGameChildRuntimeLocalNotification =
                AppGameAndroidChildRuntimeLocalNotificationProof.createLocalNotificationBundle(this);
        }
        Bundle lifecycleProof = ChildAndroidLifecycleProof.createStatusBundle();
        Bundle storageProof = ChildAndroidStorageProtocolProof.createStorageProtocolBundle();
        Bundle serviceProof = ChildAndroidServiceProtocolProof.createServiceProtocolBundle();
        Bundle permissionProof = ChildAndroidPermissionCapabilityProof.createPermissionCapabilityBundle();
        Bundle privilegedProof = ChildAndroidPrivilegedCapabilityProof.createPrivilegedCapabilityBundle();
        Bundle screenProof = ChildAndroidScreenCaptureProof.createScreenCaptureBundle();
        Bundle appGameUsageEventsProof =
            AppGameAndroidUsageEventsCapabilityProof.createUsageEventsCapabilityBundle();
        Bundle appGameUsageEventsPreflight =
            AppGameAndroidUsageEventsRuntimePreflight.createRuntimePreflightBundle(this);
        Bundle appGameAccessibilityRuntime =
            AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle();
        Bundle appGameChildRuntimeTransportReceipt =
            AppGameAndroidChildRuntimeTransportReceiptProof.createChildRuntimeTransportReceiptBundle(this);
        Bundle appGameChildRuntimeDelivery =
            AppGameAndroidChildRuntimeDeliveryProof.createChildRuntimeDeliveryBundle(this);
        Bundle appGameChildRuntimeNotificationRequestQueue =
            AppGameAndroidChildRuntimeNotificationRequestQueueProof.createRequestQueueBundle(this);

        TextView status = new TextView(this);
        String statusText = getString(R.string.agent_status) +
            "\n" +
            lifecycleProof.getString(ChildAndroidLifecycleProof.FIELD_BRIDGE_STATE) +
            "\n" +
            storageProof.getString(ChildAndroidStorageProtocolProof.FIELD_STORAGE_BRIDGE_STATE) +
            "\n" +
            serviceProof.getString(ChildAndroidServiceProtocolProof.FIELD_FOREGROUND_SERVICE_STATUS) +
            "\n" +
            permissionProof.getString(ChildAndroidPermissionCapabilityProof.FIELD_PERMISSION_BRIDGE_STATE) +
            "\n" +
            privilegedProof.getString(ChildAndroidPrivilegedCapabilityProof.FIELD_PRIVILEGED_BRIDGE_STATE) +
            "\n" +
            screenProof.getString(ChildAndroidScreenCaptureProof.FIELD_SCREEN_CAPTURE_STATE) +
            "\n" +
            appGameUsageEventsProof.getString(
                AppGameAndroidUsageEventsCapabilityProof.FIELD_USAGE_EVENTS_BRIDGE_STATE
            ) +
            "\n" +
            appGameUsageEventsPreflight.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_PERMISSION_CHECK_STATE
            ) +
            "\n" +
            appGameUsageEventsPreflight.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_STATE
            ) +
            "\n" +
            AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_EVENT_COUNT +
            "=" +
            appGameUsageEventsPreflight.getInt(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_EVENT_COUNT
            ) +
            "\n" +
            AppGameAndroidUsageEventsRuntimePreflight.FIELD_FOREGROUND_EVENT_COUNT +
            "=" +
            appGameUsageEventsPreflight.getInt(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_FOREGROUND_EVENT_COUNT
            ) +
            "\n" +
            appGameAccessibilityRuntime.getString(
                AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_DECLARATION_STATE
            ) +
            "\n" +
            appGameAccessibilityRuntime.getString(
                AppGameAndroidAccessibilityRuntimeService.FIELD_SERVICE_RUNTIME_STATE
            ) +
            "\n" +
            appGameAccessibilityRuntime.getString(
                AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_STATE
            ) +
            "\n" +
            AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_COUNT +
            "=" +
            appGameAccessibilityRuntime.getInt(
                AppGameAndroidAccessibilityRuntimeService.FIELD_EVENT_SAMPLE_COUNT
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_TRANSPORT_CHANNEL_STATE
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_STORE_STATE
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_ACK_STATE
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_APPEND_STATE
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_READBACK_STATE
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_LOCAL_ACK_STATE
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_LOCAL_ACK_READBACK_STATE
            ) +
            "\n" +
            appGameChildRuntimeTransportReceipt.getString(
                AppGameAndroidChildRuntimeTransportReceiptProof.FIELD_RECEIPT_CHANNEL_STATE
            ) +
            "\n" +
            appGameChildRuntimeDelivery.getString(
                AppGameAndroidChildRuntimeDeliveryProof.FIELD_DELIVERY_INTAKE_STATE
            ) +
            "\n" +
            appGameChildRuntimeDelivery.getString(
                AppGameAndroidChildRuntimeDeliveryProof.FIELD_DELIVERY_READBACK_STATE
            ) +
            "\n" +
            appGameChildRuntimeLocalNotification.getString(
                AppGameAndroidChildRuntimeLocalNotificationProof.FIELD_NOTIFICATION_CHANNEL_STATE
            ) +
            "\n" +
            appGameChildRuntimeLocalNotification.getString(
                AppGameAndroidChildRuntimeLocalNotificationProof.FIELD_NOTIFICATION_POST_STATE
            ) +
            "\n" +
            appGameChildRuntimeLocalNotification.getString(
                AppGameAndroidChildRuntimeLocalNotificationProof.FIELD_NOTIFICATION_MARKER_STATE
            ) +
            "\n" +
            appGameChildRuntimeLocalNotification.getString(
                AppGameAndroidChildRuntimeLocalNotificationProof.FIELD_NOTIFICATION_REQUEST_ACTION_STATE
            ) +
            "\n" +
            appGameChildRuntimeNotificationRequestQueue.getString(
                AppGameAndroidChildRuntimeNotificationRequestQueueProof.FIELD_REQUEST_QUEUE_STATE
            ) +
            "\n" +
            appGameChildRuntimeNotificationRequestQueue.getString(
                AppGameAndroidChildRuntimeNotificationRequestQueueProof.FIELD_REQUEST_READBACK_STATE
            ) +
            "\n" +
            appGameChildRuntimeNotificationRequestQueue.getString(
                AppGameAndroidChildRuntimeNotificationRequestQueueProof.FIELD_REQUEST_DRAIN_STATE
            );
        status.setText(statusText);
        status.setBackgroundColor(Color.rgb(249, 250, 251));
        status.setTextColor(Color.rgb(17, 24, 39));
        status.setTextSize(18);
        status.setGravity(Gravity.CENTER);
        status.setPadding(32, 32, 32, 32);
        setContentView(status);
    }
}
