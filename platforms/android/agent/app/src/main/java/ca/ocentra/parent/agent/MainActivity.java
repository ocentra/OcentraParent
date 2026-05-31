package ca.ocentra.parent.agent;

import android.app.Activity;
import android.content.Intent;
import android.os.Bundle;
import android.widget.TextView;

public final class MainActivity extends Activity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        startForegroundService(new Intent(this, OcentraParentAgentService.class));
        Bundle lifecycleProof = ChildAndroidLifecycleProof.createStatusBundle();
        Bundle storageProof = ChildAndroidStorageProtocolProof.createStorageProtocolBundle();
        Bundle serviceProof = ChildAndroidServiceProtocolProof.createServiceProtocolBundle();
        Bundle permissionProof = ChildAndroidPermissionCapabilityProof.createPermissionCapabilityBundle();

        TextView status = new TextView(this);
        status.setText(
            getString(R.string.agent_status) +
            "\n" +
            lifecycleProof.getString(ChildAndroidLifecycleProof.FIELD_BRIDGE_STATE) +
            "\n" +
            storageProof.getString(ChildAndroidStorageProtocolProof.FIELD_STORAGE_BRIDGE_STATE) +
            "\n" +
            serviceProof.getString(ChildAndroidServiceProtocolProof.FIELD_FOREGROUND_SERVICE_STATUS) +
            "\n" +
            permissionProof.getString(ChildAndroidPermissionCapabilityProof.FIELD_PERMISSION_BRIDGE_STATE)
        );
        status.setTextSize(18);
        status.setPadding(32, 32, 32, 32);
        setContentView(status);
    }
}
