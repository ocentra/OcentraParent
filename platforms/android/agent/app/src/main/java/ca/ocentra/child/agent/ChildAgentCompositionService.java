package ca.ocentra.child.agent;

import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.Service;
import android.content.Intent;
import android.os.Binder;
import android.os.Build;
import android.os.Bundle;
import android.os.IBinder;

import java.util.List;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.RejectedExecutionException;
import java.util.concurrent.ThreadPoolExecutor;
import java.util.concurrent.ThreadFactory;
import java.util.concurrent.TimeUnit;

import ca.ocentra.parent.agent.AppGameAndroidAccessibilityRuntimeService;
import ca.ocentra.parent.agent.AppGameAndroidAuthorityPreflight;
import ca.ocentra.parent.agent.AppGameAndroidUsageEventsChildRuntimeReplay;
import ca.ocentra.parent.agent.AppGameAndroidUsageEventsRuntimePreflight;
import ca.ocentra.parent.agent.R;

public final class ChildAgentCompositionService extends Service {
    private static final String CHANNEL_ID = "ocentra_child_agent";
    private static final int NOTIFICATION_ID = 4477;

    private final LocalBinder binder = new LocalBinder();
    private final Object runtimeStateLock = new Object();
    private final ExecutorService runtimeWorker = new ThreadPoolExecutor(
        1,
        1,
        0L,
        TimeUnit.MILLISECONDS,
        new ArrayBlockingQueue<Runnable>(1),
        new RuntimeWorkerThreadFactory(),
        new ThreadPoolExecutor.AbortPolicy()
    );
    private ChildAgentComposition composition;
    private RuntimeStateSnapshot runtimeState = RuntimeStateSnapshot.empty();
    private boolean refreshQueued;
    private boolean refreshDirty;
    private boolean stopping;
    private CountDownLatch refreshCompletion = new CountDownLatch(0);
    private String shutdownState = "running";
    private String compositionCloseState = "open";
    private boolean compositionClosed;

    @Override
    public void onCreate() {
        super.onCreate();
        try {
            composition = ChildAgentComposition.open(this);
        } catch (Exception error) {
            composition = ChildAgentComposition.failed(this, error.getMessage());
        }
        requestRuntimeRefresh();
        ensureNotificationChannel();
        startForeground(NOTIFICATION_ID, buildNotification());
    }

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        requestRuntimeRefresh();
        return START_STICKY;
    }

    @Override
    public IBinder onBind(Intent intent) {
        return binder;
    }

    @Override
    public void onDestroy() {
        final CountDownLatch completion;
        synchronized (runtimeStateLock) {
            stopping = true;
            shutdownState = "stop-requested";
            completion = refreshCompletion;
        }
        runtimeWorker.shutdown();
        boolean workerTerminated = await(runtimeWorker, 250L);
        if (!workerTerminated) {
            List<Runnable> cancelledTasks = runtimeWorker.shutdownNow();
            if (!cancelledTasks.isEmpty()) {
                completion.countDown();
            }
            workerTerminated = await(runtimeWorker, 100L);
        }
        boolean refreshCompleted = completion.getCount() == 0L;
        boolean safeToClose = workerTerminated && refreshCompleted;
        synchronized (runtimeStateLock) {
            shutdownState = safeToClose ? "worker-stopped" : "worker-stop-timeout-fail-closed";
            if (!safeToClose) {
                compositionCloseState = "deferred-worker-timeout";
            }
            if (!safeToClose) {
                runtimeState = RuntimeStateSnapshot.failed("worker-shutdown-timeout");
            }
        }
        if (safeToClose) {
            closeCompositionAfterWorkerStop();
        }
        super.onDestroy();
    }

    public final class LocalBinder extends Binder {
        public ChildAgentComposition.Health health() {
            return composition.health();
        }

        public Bundle runtimeStatus() {
            synchronized (runtimeStateLock) {
                Bundle status = runtimeState.toBundle();
                status.putString("shutdownState", shutdownState);
                status.putString("compositionCloseState", compositionCloseState);
                return status;
            }
        }
    }

    private void requestRuntimeRefresh() {
        final CountDownLatch completion = new CountDownLatch(1);
        synchronized (runtimeStateLock) {
            if (stopping) {
                return;
            }
            if (refreshQueued) {
                refreshDirty = true;
                return;
            }
            refreshQueued = true;
            refreshDirty = false;
            refreshCompletion = completion;
        }
        enqueueRuntimeRefresh(completion);
    }

    private void enqueueRuntimeRefresh(final CountDownLatch completion) {
        try {
            runtimeWorker.execute(new Runnable() {
                @Override
                public void run() {
                    try {
                        refreshRuntimeStateOnWorker();
                    } finally {
                        completion.countDown();
                        closeCompositionAfterWorkerStop();
                    }
                }
            });
        } catch (RejectedExecutionException error) {
            synchronized (runtimeStateLock) {
                refreshQueued = false;
                refreshDirty = false;
                completion.countDown();
                runtimeState = RuntimeStateSnapshot.failed("worker-rejected-refresh");
            }
        }
    }

    private void closeCompositionAfterWorkerStop() {
        ChildAgentComposition compositionToClose;
        synchronized (runtimeStateLock) {
            if (!stopping || compositionClosed) {
                return;
            }
            compositionClosed = true;
            compositionCloseState = "closing";
            compositionToClose = composition;
        }
        if (compositionToClose == null) {
            synchronized (runtimeStateLock) {
                compositionCloseState = "closed-after-worker-stop";
            }
            return;
        }
        try {
            compositionToClose.close();
            synchronized (runtimeStateLock) {
                compositionCloseState = "closed-after-worker-stop";
            }
        } catch (RuntimeException error) {
            synchronized (runtimeStateLock) {
                compositionCloseState = "close-failed";
                runtimeState = RuntimeStateSnapshot.failed("composition-close-failed");
            }
        }
    }

    private void refreshRuntimeStateOnWorker() {
        RuntimeStateSnapshot nextState;
        try {
            AppGameAndroidUsageEventsRuntimePreflight.ChildRuntimeSource source =
                AppGameAndroidUsageEventsRuntimePreflight.createChildRuntimeSource(this);
            Bundle usageEventsPreflight = source.diagnostics();
            Bundle usageEventsReplay = AppGameAndroidUsageEventsChildRuntimeReplay.consume(this, source);
            usageEventsReplay.putBundle(
                "durableReadback",
                AppGameAndroidUsageEventsChildRuntimeReplay.read(this)
            );
            nextState = RuntimeStateSnapshot.of(
                usageEventsPreflight,
                usageEventsReplay,
                AppGameAndroidAuthorityPreflight.createAuthorityPreflightBundle(this),
                AppGameAndroidAccessibilityRuntimeService.createAccessibilityRuntimeBundle(this)
            );
        } catch (RuntimeException error) {
            nextState = RuntimeStateSnapshot.failed(error.getClass().getSimpleName());
        }
        CountDownLatch rerunCompletion = null;
        synchronized (runtimeStateLock) {
            if (!stopping) {
                runtimeState = nextState;
            }
            if (stopping) {
                refreshQueued = false;
                refreshDirty = false;
            } else if (refreshDirty) {
                refreshDirty = false;
                rerunCompletion = new CountDownLatch(1);
                refreshCompletion = rerunCompletion;
            } else {
                refreshQueued = false;
            }
        }
        if (rerunCompletion != null) {
            enqueueRuntimeRefresh(rerunCompletion);
        }
    }

    private void ensureNotificationChannel() {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O) {
            return;
        }
        NotificationChannel channel = new NotificationChannel(
            CHANNEL_ID,
            getString(R.string.notification_channel_name),
            NotificationManager.IMPORTANCE_LOW
        );
        NotificationManager manager = getSystemService(NotificationManager.class);
        manager.createNotificationChannel(channel);
    }

    private Notification buildNotification() {
        return new Notification.Builder(this, CHANNEL_ID)
            .setContentTitle(getString(R.string.app_name))
            .setContentText(getString(R.string.notification_text))
            .setSmallIcon(android.R.drawable.ic_menu_view)
            .setOngoing(true)
            .build();
    }

    private static final class RuntimeStateSnapshot {
        private final Bundle usageEventsPreflight;
        private final Bundle usageEventsReplay;
        private final Bundle authorityPreflight;
        private final Bundle accessibilityPreflight;
        private final String refreshState;
        private final String refreshFailure;

        private RuntimeStateSnapshot(
            Bundle usageEventsPreflight,
            Bundle usageEventsReplay,
            Bundle authorityPreflight,
            Bundle accessibilityPreflight,
            String refreshState,
            String refreshFailure
        ) {
            this.usageEventsPreflight = copyBundle(usageEventsPreflight);
            this.usageEventsReplay = copyBundle(usageEventsReplay);
            this.authorityPreflight = copyBundle(authorityPreflight);
            this.accessibilityPreflight = copyBundle(accessibilityPreflight);
            this.refreshState = refreshState;
            this.refreshFailure = refreshFailure;
        }

        static RuntimeStateSnapshot empty() {
            return new RuntimeStateSnapshot(
                new Bundle(),
                new Bundle(),
                new Bundle(),
                new Bundle(),
                "not-yet-refreshed",
                ""
            );
        }

        static RuntimeStateSnapshot of(
            Bundle usageEventsPreflight,
            Bundle usageEventsReplay,
            Bundle authorityPreflight,
            Bundle accessibilityPreflight
        ) {
            return new RuntimeStateSnapshot(
                usageEventsPreflight,
                usageEventsReplay,
                authorityPreflight,
                accessibilityPreflight,
                "refreshed",
                ""
            );
        }

        static RuntimeStateSnapshot failed(String failure) {
            return new RuntimeStateSnapshot(
                new Bundle(),
                new Bundle(),
                new Bundle(),
                new Bundle(),
                "refresh-failed-closed",
                failure
            );
        }

        Bundle toBundle() {
            Bundle status = new Bundle();
            status.putString("schemaVersion", "app-game-android-child-runtime-status");
            status.putBundle("usageEventsPreflight", copyBundle(usageEventsPreflight));
            status.putBundle("usageEventsReplay", copyBundle(usageEventsReplay));
            status.putBundle("authorityPreflight", copyBundle(authorityPreflight));
            status.putBundle("accessibilityPreflight", copyBundle(accessibilityPreflight));
            status.putString("refreshState", refreshState);
            status.putString("refreshFailure", refreshFailure);
            status.putString("restartPolicy", "START_STICKY");
            status.putString("refreshExecution", "single-worker-bounded-queue");
            status.putString("durableReadbackSemantics", "read-after-worker-commit");
            status.putBoolean("rawUsageEventsStored", false);
            status.putBoolean("rawPackageNamesStored", false);
            status.putBoolean("rawAccessibilityServiceNamesStored", false);
            status.putBoolean("rawAccessibilityEventsStored", false);
            status.putBoolean("adapterDispatchClaimed", false);
            status.putBoolean("platformEnforcementClaimed", false);
            status.putBoolean("childDeviceDeliveryClaimed", false);
            return status;
        }

        private static Bundle copyBundle(Bundle source) {
            if (source == null) {
                return new Bundle();
            }
            Bundle copy = new Bundle();
            for (String key : source.keySet()) {
                Object value = source.get(key);
                if (value instanceof Bundle) {
                    copy.putBundle(key, copyBundle((Bundle) value));
                } else if (value instanceof String[]) {
                    copy.putStringArray(key, ((String[]) value).clone());
                } else if (value instanceof String) {
                    copy.putString(key, (String) value);
                } else if (value instanceof Boolean) {
                    copy.putBoolean(key, (Boolean) value);
                } else if (value instanceof Integer) {
                    copy.putInt(key, (Integer) value);
                } else if (value instanceof Long) {
                    copy.putLong(key, (Long) value);
                }
            }
            return copy;
        }
    }

    private static final class RuntimeWorkerThreadFactory implements ThreadFactory {
        @Override
        public Thread newThread(Runnable runnable) {
            return new Thread(runnable, "ocentra-child-runtime-preflight");
        }
    }

    private static boolean await(ExecutorService executor, long timeoutMillis) {
        try {
            return executor.awaitTermination(timeoutMillis, TimeUnit.MILLISECONDS);
        } catch (InterruptedException error) {
            Thread.currentThread().interrupt();
            return false;
        }
    }
}
