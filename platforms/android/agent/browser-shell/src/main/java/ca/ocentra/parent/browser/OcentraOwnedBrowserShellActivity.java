package ca.ocentra.parent.browser;

import android.annotation.SuppressLint;
import android.app.Activity;
import android.app.admin.DevicePolicyManager;
import android.content.ComponentName;
import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;
import android.graphics.Color;
import android.net.Uri;
import android.os.Bundle;
import android.view.Gravity;
import android.view.ViewGroup;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import android.widget.LinearLayout;
import android.widget.TextView;

public final class OcentraOwnedBrowserShellActivity extends Activity {
    private static final String DEFAULT_PAGE =
            "data:text/html,<html><body><h1>Ocentra owned browser shell ready</h1></body></html>";
    private static final String PROOF_PATH_MARKER = "owned-browser-shell-proof";
    private static final String SCHEME_HTTP = "http";
    private static final String SCHEME_HTTPS = "https";

    private PolicyState policyState = PolicyState.NOT_DEVICE_OWNER;
    private TextView statusView;
    private TextView policyView;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setTitle(getString(R.string.owned_browser_shell_label));
        configureOwnedBrowserPolicy();
        setContentView(createContentView());
        loadRequestedPage();
    }

    @SuppressLint("SetJavaScriptEnabled")
    private LinearLayout createContentView() {
        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setBackgroundColor(Color.WHITE);
        root.setLayoutParams(
                new LinearLayout.LayoutParams(
                        ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.MATCH_PARENT));

        statusView = new TextView(this);
        statusView.setText(R.string.owned_browser_shell_ready);
        statusView.setTextColor(Color.rgb(16, 24, 40));
        statusView.setTextSize(18);
        statusView.setGravity(Gravity.CENTER_VERTICAL);
        statusView.setPadding(24, 20, 24, 20);
        root.addView(
                statusView,
                new LinearLayout.LayoutParams(
                        ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.WRAP_CONTENT));

        policyView = new TextView(this);
        policyView.setText(policyTextFor(policyState));
        policyView.setTextColor(Color.rgb(52, 64, 84));
        policyView.setTextSize(14);
        policyView.setGravity(Gravity.CENTER_VERTICAL);
        policyView.setPadding(24, 0, 24, 20);
        root.addView(
                policyView,
                new LinearLayout.LayoutParams(
                        ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.WRAP_CONTENT));

        WebView webView = new WebView(this);
        webView.getSettings().setJavaScriptEnabled(false);
        webView.getSettings().setDomStorageEnabled(false);
        webView.setWebViewClient(
                new WebViewClient() {
                    @Override
                    public void onPageFinished(WebView view, String url) {
                        if (url != null && url.contains(PROOF_PATH_MARKER)) {
                            statusView.setText(R.string.owned_browser_shell_proof_page_loaded);
                        } else {
                            statusView.setText(R.string.owned_browser_shell_page_loaded);
                        }
                    }
                });
        root.addView(
                webView,
                new LinearLayout.LayoutParams(
                        ViewGroup.LayoutParams.MATCH_PARENT, 0, 1));
        root.setTag(webView);
        return root;
    }

    private void loadRequestedPage() {
        WebView webView = (WebView) ((ViewGroup) statusView.getParent()).getTag();
        Uri data = getIntent().getData();
        webView.loadUrl(data == null ? DEFAULT_PAGE : data.toString());
    }

    private void configureOwnedBrowserPolicy() {
        DevicePolicyManager policyManager =
                (DevicePolicyManager) getSystemService(Context.DEVICE_POLICY_SERVICE);
        ComponentName admin =
                new ComponentName(this, OcentraOwnedBrowserDeviceAdminReceiver.class);
        if (policyManager == null || !policyManager.isDeviceOwnerApp(getPackageName())) {
            return;
        }

        ComponentName activity = new ComponentName(this, OcentraOwnedBrowserShellActivity.class);
        try {
            policyManager.addPersistentPreferredActivity(admin, browserIntentFilter(SCHEME_HTTP), activity);
            policyManager.addPersistentPreferredActivity(admin, browserIntentFilter(SCHEME_HTTPS), activity);
            policyState = PolicyState.PERSISTENT_BROWSER_ROUTING_CONFIGURED;
        } catch (RuntimeException exception) {
            policyState = PolicyState.PERSISTENT_BROWSER_ROUTING_UNAVAILABLE;
        }
    }

    private static IntentFilter browserIntentFilter(String scheme) {
        IntentFilter filter = new IntentFilter(Intent.ACTION_VIEW);
        filter.addCategory(Intent.CATEGORY_DEFAULT);
        filter.addCategory(Intent.CATEGORY_BROWSABLE);
        filter.addDataScheme(scheme);
        return filter;
    }

    private String policyTextFor(PolicyState state) {
        return switch (state) {
            case PERSISTENT_BROWSER_ROUTING_CONFIGURED ->
                    getString(R.string.owned_browser_policy_configured);
            case PERSISTENT_BROWSER_ROUTING_UNAVAILABLE ->
                    getString(R.string.owned_browser_policy_unavailable);
            case NOT_DEVICE_OWNER -> getString(R.string.owned_browser_policy_not_device_owner);
        };
    }

    private enum PolicyState {
        NOT_DEVICE_OWNER,
        PERSISTENT_BROWSER_ROUTING_CONFIGURED,
        PERSISTENT_BROWSER_ROUTING_UNAVAILABLE
    }
}
