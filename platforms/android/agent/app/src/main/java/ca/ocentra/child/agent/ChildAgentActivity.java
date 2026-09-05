package ca.ocentra.child.agent;

import android.app.Activity;
import android.content.ComponentName;
import android.content.Context;
import android.content.Intent;
import android.content.ServiceConnection;
import android.graphics.Color;
import android.os.Bundle;
import android.os.Handler;
import android.os.IBinder;
import android.os.Looper;
import android.widget.TextView;

import ca.ocentra.parent.agent.R;

public final class ChildAgentActivity extends Activity {
    private final Handler statusHandler = new Handler(Looper.getMainLooper());
    private final Runnable statusRefresh = new Runnable() {
        @Override
        public void run() {
            if (!serviceBound) {
                return;
            }
            refreshRuntimeStatus();
            statusHandler.postDelayed(this, 500L);
        }
    };
    private final ServiceConnection serviceConnection = new ServiceConnection() {
        @Override
        public void onServiceConnected(ComponentName name, IBinder service) {
            if (!activityStarted) {
                return;
            }
            if (!(service instanceof ChildAgentCompositionService.LocalBinder)) {
                serviceBound = false;
                serviceBinder = null;
                renderUnavailable("child-runtime-binder-invalid");
                return;
            }
            serviceBinder = (ChildAgentCompositionService.LocalBinder) service;
            serviceBound = true;
            refreshRuntimeStatus();
        }

        @Override
        public void onServiceDisconnected(ComponentName name) {
            serviceBound = false;
            serviceBinder = null;
            renderUnavailable("child-runtime-service-disconnected");
        }
    };
    private ChildAgentCompositionService.LocalBinder serviceBinder;
    private TextView statusView;
    private boolean serviceBound;
    private boolean bindRequested;
    private boolean activityStarted;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        startForegroundService(new Intent(this, ChildAgentCompositionService.class));

        statusView = new TextView(this);
        statusView.setBackgroundColor(Color.rgb(249, 250, 251));
        statusView.setTextColor(Color.rgb(17, 24, 39));
        statusView.setTextSize(18);
        statusView.setText(getString(R.string.agent_status));
        statusView.setPadding(32, 32, 32, 32);
        setContentView(statusView);
    }

    @Override
    protected void onStart() {
        super.onStart();
        activityStarted = true;
        boolean bound = bindService(
            new Intent(this, ChildAgentCompositionService.class),
            serviceConnection,
            Context.BIND_AUTO_CREATE
        );
        bindRequested = bound;
        serviceBound = bound;
        if (!bound) {
            renderUnavailable("child-runtime-bind-failed");
            return;
        }
        statusHandler.post(statusRefresh);
    }

    @Override
    protected void onStop() {
        activityStarted = false;
        statusHandler.removeCallbacks(statusRefresh);
        if (bindRequested) {
            unbindService(serviceConnection);
            bindRequested = false;
            serviceBound = false;
            serviceBinder = null;
        }
        super.onStop();
    }

    private void refreshRuntimeStatus() {
        if (!serviceBound || serviceBinder == null || statusView == null) {
            return;
        }
        try {
            Bundle status = serviceBinder.runtimeStatus();
            Bundle usageEventsPreflight = status.getBundle("usageEventsPreflight");
            Bundle usageEventsReplay = status.getBundle("usageEventsReplay");
            Bundle authorityPreflight = status.getBundle("authorityPreflight");
            Bundle accessibilityPreflight = status.getBundle("accessibilityPreflight");
            statusView.setText(
                getString(R.string.agent_status) +
                "\n" +
                statusValue(status, "refreshState", "unknown") +
                "\n" +
                statusValue(status, "restartPolicy", "unknown") +
                "\n" +
                statusValue(status, "shutdownState", "unknown") +
                "\n" +
                statusValue(status, "compositionCloseState", "unknown") +
                "\n" +
                statusValue(
                    usageEventsPreflight,
                    ca.ocentra.parent.agent.AppGameAndroidUsageEventsRuntimePreflight.FIELD_PERMISSION_CHECK_STATE,
                    "unknown"
                ) +
                "\n" +
                statusValue(
                    usageEventsReplay,
                    ca.ocentra.parent.agent.AppGameAndroidUsageEventsChildRuntimeReplay.FIELD_CONSUMER_STATE,
                    "unknown"
                ) +
                "\n" +
                statusValue(
                    authorityPreflight,
                    ca.ocentra.parent.agent.AppGameAndroidAuthorityPreflight.FIELD_PREFLIGHT_STATE,
                    "unknown"
                ) +
                "\n" +
                statusValue(
                    accessibilityPreflight,
                    ca.ocentra.parent.agent.AppGameAndroidAccessibilityRuntimeService.FIELD_PREFLIGHT_STATE,
                    "unknown"
                )
            );
        } catch (RuntimeException error) {
            renderUnavailable("child-runtime-status-read-failed");
        }
    }

    private void renderUnavailable(String reason) {
        if (statusView != null) {
            statusView.setText(getString(R.string.agent_status) + "\n" + reason);
        }
    }

    private static String statusValue(Bundle bundle, String key, String fallback) {
        return bundle == null ? fallback : bundle.getString(key, fallback);
    }
}
