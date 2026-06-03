package ca.ocentra.parent.agent;

import android.os.Bundle;

public final class ChildAndroidScreenCaptureProof {
    public static final String SCHEMA_VERSION = "child-android-screen-capture-proof";
    public static final String PACKAGE_ID = "ca.ocentra.parent.agent";
    public static final String NATIVE_BRIDGE_CLASS =
        "ca.ocentra.parent.agent.ChildAndroidScreenCaptureProof";
    public static final String CAPTURE_ACTIVITY_CLASS =
        "ca.ocentra.parent.agent.AndroidMediaProjectionCaptureActivity";
    public static final String CAPTURE_SERVICE_CLASS =
        "ca.ocentra.parent.agent.AndroidMediaProjectionCaptureService";
    public static final String PROOF_STORE_CLASS = "ca.ocentra.parent.agent.ScreenCaptureProofStore";
    public static final String MEDIA_PROJECTION_PERMISSION =
        "android.permission.FOREGROUND_SERVICE_MEDIA_PROJECTION";
    public static final String FOREGROUND_SERVICE_TYPE = "mediaProjection";
    public static final String SCREEN_CAPTURE_STATE = "manual-consent-required";
    public static final String CONSENT_STATE = "create-screen-capture-intent-required";
    public static final String SILENT_BACKGROUND_CAPTURE_STATE = "not-supported";
    public static final String FIELD_SCREEN_CAPTURE_STATE = "screenCaptureState";
    public static final String COMMAND_SCREEN_CAPTURE_PROOF_GET =
        "child.android.screen-capture.proof.get";
    public static final String EVENT_SCREEN_CAPTURE_PROOF_REPORTED =
        "child.android.screen-capture.proof.reported";

    private ChildAndroidScreenCaptureProof() {}

    public static Bundle createScreenCaptureBundle() {
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString("packageId", PACKAGE_ID);
        status.putString("nativeBridgeClass", NATIVE_BRIDGE_CLASS);
        status.putString("captureActivityClass", CAPTURE_ACTIVITY_CLASS);
        status.putString("captureServiceClass", CAPTURE_SERVICE_CLASS);
        status.putString("proofStoreClass", PROOF_STORE_CLASS);
        status.putString(FIELD_SCREEN_CAPTURE_STATE, SCREEN_CAPTURE_STATE);
        status.putString("consentState", CONSENT_STATE);
        status.putString("silentBackgroundCaptureState", SILENT_BACKGROUND_CAPTURE_STATE);
        status.putString("foregroundServiceType", FOREGROUND_SERVICE_TYPE);
        status.putString("requiredManifestPermission", MEDIA_PROJECTION_PERMISSION);
        status.putStringArray(
            "commands",
            new String[] { COMMAND_SCREEN_CAPTURE_PROOF_GET }
        );
        status.putStringArray(
            "events",
            new String[] { EVENT_SCREEN_CAPTURE_PROOF_REPORTED }
        );
        status.putStringArray(
            "proofRequirements",
            new String[] {
                "user-consent-dialog-approved",
                "foreground-service-started-with-mediaProjection",
                "virtual-display-created",
                "image-reader-produced-frame",
                "capture-digest-recorded",
                "raw-temp-bytes-deleted"
            }
        );
        return status;
    }
}
