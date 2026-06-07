package ca.ocentra.parent.browser;

import android.annotation.SuppressLint;
import android.app.Activity;
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

    private TextView statusView;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setTitle(getString(R.string.owned_browser_shell_label));
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
}
