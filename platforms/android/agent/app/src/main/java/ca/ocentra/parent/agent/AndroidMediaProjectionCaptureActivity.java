package ca.ocentra.parent.agent;

import android.app.Activity;
import android.content.Context;
import android.content.Intent;
import android.media.projection.MediaProjectionManager;
import android.os.Bundle;

public final class AndroidMediaProjectionCaptureActivity extends Activity {
    public static final String EXTRA_RESULT_CODE = "ca.ocentra.parent.agent.MEDIA_PROJECTION_RESULT_CODE";
    public static final String EXTRA_RESULT_DATA = "ca.ocentra.parent.agent.MEDIA_PROJECTION_RESULT_DATA";
    private static final int REQUEST_MEDIA_PROJECTION = 8035;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        try {
            ScreenCaptureProofStore.writePending(this);
        } catch (Exception ignored) {
        }
        MediaProjectionManager manager =
            (MediaProjectionManager) getSystemService(Context.MEDIA_PROJECTION_SERVICE);
        startActivityForResult(manager.createScreenCaptureIntent(), REQUEST_MEDIA_PROJECTION);
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        if (requestCode != REQUEST_MEDIA_PROJECTION) {
            finish();
            return;
        }
        if (resultCode != RESULT_OK || data == null) {
            try {
                ScreenCaptureProofStore.writeDenied(this);
            } catch (Exception ignored) {
            }
            finish();
            return;
        }

        Intent serviceIntent = new Intent(this, AndroidMediaProjectionCaptureService.class);
        serviceIntent.putExtra(EXTRA_RESULT_CODE, resultCode);
        serviceIntent.putExtra(EXTRA_RESULT_DATA, data);
        startForegroundService(serviceIntent);
        finish();
    }
}
