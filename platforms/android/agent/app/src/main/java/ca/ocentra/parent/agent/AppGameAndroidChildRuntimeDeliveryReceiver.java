package ca.ocentra.parent.agent;

import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;

public final class AppGameAndroidChildRuntimeDeliveryReceiver extends BroadcastReceiver {
    @Override
    public void onReceive(Context context, Intent intent) {
        if (
            intent != null &&
            AppGameAndroidChildRuntimeDeliveryProof.ACTION_LOCAL_DELIVERY_INTAKE_PROOF.equals(intent.getAction())
        ) {
            AppGameAndroidChildRuntimeDeliveryProof.recordPackageLocalDeliveryIntake(context);
        }
    }
}
