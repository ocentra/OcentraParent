package ca.ocentra.parent.agent;

import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;

public final class AppGameAndroidChildRuntimeReceiptReceiver extends BroadcastReceiver {
    @Override
    public void onReceive(Context context, Intent intent) {
        if (intent == null) {
            return;
        }
        if (!AppGameAndroidChildRuntimeTransportReceiptProof.ACTION_LOCAL_RECEIPT_CHANNEL_PROOF.equals(
            intent.getAction()
        )) {
            return;
        }
        AppGameAndroidChildRuntimeTransportReceiptProof.recordPackageLocalReceiptChannel(context);
    }
}
