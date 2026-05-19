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

        TextView status = new TextView(this);
        status.setText(R.string.agent_status);
        status.setTextSize(18);
        status.setPadding(32, 32, 32, 32);
        setContentView(status);
    }
}
