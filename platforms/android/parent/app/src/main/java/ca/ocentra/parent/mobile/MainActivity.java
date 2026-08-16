package ca.ocentra.parent.mobile;

import android.app.Activity;
import android.graphics.Color;
import android.os.Bundle;
import android.view.Gravity;
import android.widget.TextView;

public final class MainActivity extends Activity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        TextView status = new TextView(this);
        String statusText = getString(R.string.parent_mobile_status_title) +
            "\n" +
            getString(R.string.parent_mobile_observer_state) +
            "\n" +
            getString(R.string.parent_mobile_controller_state) +
            "\n" +
            getString(R.string.parent_mobile_child_agent_boundary) +
            "\n" +
            getString(R.string.parent_mobile_store_boundary);
        status.setText(statusText);
        status.setBackgroundColor(Color.rgb(249, 250, 251));
        status.setTextColor(Color.rgb(17, 24, 39));
        status.setTextSize(18);
        status.setGravity(Gravity.CENTER);
        status.setPadding(32, 32, 32, 32);
        setContentView(status);
    }
}
