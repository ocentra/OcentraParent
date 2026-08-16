package ca.ocentra.child.agent;

import android.app.Activity;
import android.content.Intent;
import android.graphics.Color;
import android.os.Bundle;
import android.widget.TextView;

import ca.ocentra.parent.agent.R;

public final class ChildAgentActivity extends Activity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        startForegroundService(new Intent(this, ChildAgentCompositionService.class));

        TextView status = new TextView(this);
        status.setBackgroundColor(Color.rgb(249, 250, 251));
        status.setTextColor(Color.rgb(17, 24, 39));
        status.setTextSize(18);
        status.setText(getString(R.string.agent_status));
        status.setPadding(32, 32, 32, 32);
        setContentView(status);
    }
}
