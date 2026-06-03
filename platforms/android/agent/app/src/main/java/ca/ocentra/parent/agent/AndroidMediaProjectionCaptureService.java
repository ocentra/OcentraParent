package ca.ocentra.parent.agent;

import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.Service;
import android.content.Context;
import android.content.Intent;
import android.content.pm.ServiceInfo;
import android.hardware.display.DisplayManager;
import android.hardware.display.VirtualDisplay;
import android.media.Image;
import android.media.ImageReader;
import android.media.projection.MediaProjection;
import android.media.projection.MediaProjectionManager;
import android.os.Build;
import android.os.Handler;
import android.os.HandlerThread;
import android.os.IBinder;
import android.util.DisplayMetrics;
import android.view.Surface;
import java.nio.ByteBuffer;

public final class AndroidMediaProjectionCaptureService extends Service {
    private static final String CHANNEL_ID = "ocentra_parent_agent_screen_capture";
    private static final int NOTIFICATION_ID = 4488;
    private HandlerThread captureThread;
    private Handler captureHandler;
    private ImageReader imageReader;
    private MediaProjection mediaProjection;
    private VirtualDisplay virtualDisplay;

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        ensureNotificationChannel();
        startMediaProjectionForeground();
        if (intent == null || !intent.hasExtra(AndroidMediaProjectionCaptureActivity.EXTRA_RESULT_DATA)) {
            writeErrorAndStop("missing-media-projection-consent");
            return START_NOT_STICKY;
        }

        Intent resultData;
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            resultData = intent.getParcelableExtra(
                AndroidMediaProjectionCaptureActivity.EXTRA_RESULT_DATA,
                Intent.class
            );
        } else {
            resultData =
                intent.getParcelableExtra(AndroidMediaProjectionCaptureActivity.EXTRA_RESULT_DATA);
        }
        int resultCode = intent.getIntExtra(
            AndroidMediaProjectionCaptureActivity.EXTRA_RESULT_CODE,
            0
        );
        MediaProjectionManager manager =
            (MediaProjectionManager) getSystemService(Context.MEDIA_PROJECTION_SERVICE);
        mediaProjection = manager.getMediaProjection(resultCode, resultData);
        if (mediaProjection == null) {
            writeErrorAndStop("media-projection-unavailable");
            return START_NOT_STICKY;
        }

        captureThread = new HandlerThread("ocentra-screen-capture-proof");
        captureThread.start();
        captureHandler = new Handler(captureThread.getLooper());
        mediaProjection.registerCallback(new MediaProjection.Callback() {
            @Override
            public void onStop() {
                cleanupCapture();
            }
        }, captureHandler);

        DisplayMetrics metrics = getResources().getDisplayMetrics();
        int width = Math.max(metrics.widthPixels, 1);
        int height = Math.max(metrics.heightPixels, 1);
        int density = Math.max(metrics.densityDpi, DisplayMetrics.DENSITY_DEFAULT);
        imageReader = ImageReader.newInstance(width, height, android.graphics.PixelFormat.RGBA_8888, 2);
        imageReader.setOnImageAvailableListener(reader -> captureFrame(reader), captureHandler);
        Surface surface = imageReader.getSurface();
        virtualDisplay = mediaProjection.createVirtualDisplay(
            "OcentraParentScreenCaptureProof",
            width,
            height,
            density,
            DisplayManager.VIRTUAL_DISPLAY_FLAG_AUTO_MIRROR,
            surface,
            null,
            captureHandler
        );
        return START_NOT_STICKY;
    }

    @Override
    public IBinder onBind(Intent intent) {
        return null;
    }

    @Override
    public void onDestroy() {
        cleanupCapture();
        super.onDestroy();
    }

    private void captureFrame(ImageReader reader) {
        Image image = reader.acquireLatestImage();
        if (image == null) {
            return;
        }
        try {
            Image.Plane plane = image.getPlanes()[0];
            ByteBuffer buffer = plane.getBuffer();
            byte[] bytes = new byte[buffer.remaining()];
            buffer.get(bytes);
            ScreenCaptureProofStore.writeCaptured(this, image.getWidth(), image.getHeight(), bytes);
        } catch (Exception error) {
            writeErrorAndStop("capture-frame-error");
            return;
        } finally {
            image.close();
        }
        cleanupCapture();
        stopSelf();
    }

    private void writeErrorAndStop(String status) {
        try {
            ScreenCaptureProofStore.writeError(this, status);
        } catch (Exception ignored) {
        }
        cleanupCapture();
        stopSelf();
    }

    private void cleanupCapture() {
        if (virtualDisplay != null) {
            virtualDisplay.release();
            virtualDisplay = null;
        }
        if (imageReader != null) {
            imageReader.close();
            imageReader = null;
        }
        if (mediaProjection != null) {
            MediaProjection projection = mediaProjection;
            mediaProjection = null;
            projection.stop();
        }
        if (captureThread != null) {
            captureThread.quitSafely();
            captureThread = null;
            captureHandler = null;
        }
    }

    private void startMediaProjectionForeground() {
        Notification notification = buildNotification();
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            startForeground(
                NOTIFICATION_ID,
                notification,
                ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PROJECTION
            );
            return;
        }
        startForeground(NOTIFICATION_ID, notification);
    }

    private void ensureNotificationChannel() {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O) {
            return;
        }
        NotificationChannel channel = new NotificationChannel(
            CHANNEL_ID,
            getString(R.string.notification_channel_screen_capture_name),
            NotificationManager.IMPORTANCE_LOW
        );
        NotificationManager manager = getSystemService(NotificationManager.class);
        manager.createNotificationChannel(channel);
    }

    private Notification buildNotification() {
        return new Notification.Builder(this, CHANNEL_ID)
            .setContentTitle(getString(R.string.app_name))
            .setContentText(getString(R.string.notification_screen_capture_text))
            .setSmallIcon(android.R.drawable.ic_menu_camera)
            .setOngoing(true)
            .build();
    }
}
