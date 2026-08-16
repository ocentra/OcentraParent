package ca.ocentra.child.agent;

import android.content.Context;
import android.os.Bundle;

import java.io.File;
import java.io.IOException;

import ca.ocentra.parent.agent.ChildAndroidServiceProtocolProof;
import ca.ocentra.parent.agent.ChildAndroidStorageProtocolProof;

public final class ChildAgentComposition implements AutoCloseable {
    public enum Readiness {
        RUST_RUNTIME_READY,
        RUST_RUNTIME_RECOVERY_PENDING,
        RUST_RUNTIME_REVOKED,
        RUST_RUNTIME_MANUAL_REQUIRED,
        STOPPED
    }

    public enum RustReadiness {
        UNAVAILABLE(0),
        READY(1),
        RECOVERY_PENDING(2),
        REVOKED(3);

        private final int nativeCode;

        RustReadiness(int nativeCode) {
            this.nativeCode = nativeCode;
        }

        static RustReadiness fromNativeCode(int nativeCode) {
            for (RustReadiness readiness : values()) {
                if (readiness.nativeCode == nativeCode) {
                    return readiness;
                }
            }
            return UNAVAILABLE;
        }
    }

    public enum TransportState {
        NOT_IMPLEMENTED
    }

    public static final class Health {
        private final Readiness readiness;
        private final TransportState transportState;
        private final String durableRoot;
        private final String legacyServiceState;
        private final String legacyStorageState;
        private final RustReadiness rustReadiness;
        private final int nativeDomainFlowCount;
        private final String failureReason;

        private Health(
            Readiness readiness,
            TransportState transportState,
            String durableRoot,
            String legacyServiceState,
            String legacyStorageState,
            RustReadiness rustReadiness,
            int nativeDomainFlowCount,
            String failureReason
        ) {
            this.readiness = readiness;
            this.transportState = transportState;
            this.durableRoot = durableRoot;
            this.legacyServiceState = legacyServiceState;
            this.legacyStorageState = legacyStorageState;
            this.rustReadiness = rustReadiness;
            this.nativeDomainFlowCount = nativeDomainFlowCount;
            this.failureReason = failureReason;
        }

        public Readiness readiness() {
            return readiness;
        }

        public TransportState transportState() {
            return transportState;
        }

        public String durableRoot() {
            return durableRoot;
        }

        public String legacyServiceState() {
            return legacyServiceState;
        }

        public String legacyStorageState() {
            return legacyStorageState;
        }

        public RustReadiness rustReadiness() {
            return rustReadiness;
        }

        public int nativeDomainFlowCount() {
            return nativeDomainFlowCount;
        }

        public String failureReason() {
            return failureReason;
        }
    }

    private final File durableRoot;
    private final String legacyServiceState;
    private final String legacyStorageState;
    private long nativeHandle;
    private Readiness readiness;
    private RustReadiness rustReadiness;
    private int nativeDomainFlowCount;
    private String failureReason;

    private ChildAgentComposition(
        File durableRoot,
        String legacyServiceState,
        String legacyStorageState
    ) {
        this.durableRoot = durableRoot;
        this.legacyServiceState = legacyServiceState;
        this.legacyStorageState = legacyStorageState;
        this.rustReadiness = RustReadiness.UNAVAILABLE;
        this.readiness = Readiness.RUST_RUNTIME_MANUAL_REQUIRED;
    }

    public static ChildAgentComposition open(Context context) throws IOException {
        File durableRoot = new File(context.getFilesDir(), "child-runtime");
        ensureDirectory(durableRoot);
        Bundle serviceStatus = ChildAndroidServiceProtocolProof.createServiceProtocolBundle();
        Bundle storageStatus = ChildAndroidStorageProtocolProof.createStorageProtocolBundle();
        ChildAgentComposition composition = new ChildAgentComposition(
            durableRoot,
            serviceStatus.getString(ChildAndroidServiceProtocolProof.FIELD_FOREGROUND_SERVICE_STATUS),
            storageStatus.getString(ChildAndroidStorageProtocolProof.FIELD_STORAGE_BRIDGE_STATE)
        );
        if (!NATIVE_BRIDGE_AVAILABLE) {
            composition.failureReason = "Rust child-runtime Android bridge is unavailable";
            return composition;
        }
        composition.nativeHandle = nativeStart(durableRoot.getAbsolutePath());
        if (composition.nativeHandle == 0L) {
            composition.failureReason = nativeFailureReason();
            return composition;
        }
        composition.refreshNativeHealth();
        return composition;
    }

    public static ChildAgentComposition failed(Context context, String reason) {
        File durableRoot = new File(context.getFilesDir(), "child-runtime");
        ChildAgentComposition composition = new ChildAgentComposition(
            durableRoot,
            "unavailable",
            "unavailable"
        );
        composition.failureReason = reason;
        composition.readiness = Readiness.RUST_RUNTIME_MANUAL_REQUIRED;
        return composition;
    }

    public Health health() {
        refreshNativeHealth();
        return new Health(
            readiness,
            TransportState.NOT_IMPLEMENTED,
            durableRoot.getAbsolutePath(),
            legacyServiceState,
            legacyStorageState,
            rustReadiness,
            nativeDomainFlowCount,
            failureReason
        );
    }

    @Override
    public void close() {
        if (nativeHandle != 0L) {
            nativeStop(nativeHandle);
            nativeHandle = 0L;
        }
        readiness = Readiness.STOPPED;
    }

    private void refreshNativeHealth() {
        if (readiness == Readiness.STOPPED) {
            return;
        }
        if (nativeHandle == 0L) {
            rustReadiness = RustReadiness.UNAVAILABLE;
            readiness = Readiness.RUST_RUNTIME_MANUAL_REQUIRED;
            if (failureReason == null || failureReason.isEmpty()) {
                failureReason = nativeFailureReason();
            }
            return;
        }
        rustReadiness = RustReadiness.fromNativeCode(nativeReadiness(nativeHandle));
        nativeDomainFlowCount = nativeDomainFlowCount(nativeHandle);
        switch (rustReadiness) {
            case READY:
                readiness = Readiness.RUST_RUNTIME_READY;
                failureReason = null;
                break;
            case RECOVERY_PENDING:
                readiness = Readiness.RUST_RUNTIME_RECOVERY_PENDING;
                failureReason = "Rust child-runtime tombstone recovery remains pending";
                break;
            case REVOKED:
                readiness = Readiness.RUST_RUNTIME_REVOKED;
                failureReason = "Rust child-runtime trust is revoked";
                break;
            case UNAVAILABLE:
            default:
                readiness = Readiness.RUST_RUNTIME_MANUAL_REQUIRED;
                failureReason = nativeFailureReason();
                break;
        }
    }

    private static String nativeFailureReason() {
        if (!NATIVE_BRIDGE_AVAILABLE) {
            return "Rust child-runtime Android bridge is unavailable";
        }
        String reason = nativeLastError();
        if (reason == null || reason.isEmpty()) {
            return "Rust child-runtime bridge startup or health query failed";
        }
        return reason;
    }

    private static final boolean NATIVE_BRIDGE_AVAILABLE;

    static {
        boolean loaded;
        try {
            System.loadLibrary("ocentra_child_runtime_android");
            loaded = true;
        } catch (UnsatisfiedLinkError error) {
            loaded = false;
        }
        NATIVE_BRIDGE_AVAILABLE = loaded;
    }

    private static native long nativeStart(String durableRoot);

    private static native int nativeReadiness(long handle);

    private static native int nativeDomainFlowCount(long handle);

    private static native String nativeLastError();

    private static native boolean nativeStop(long handle);

    private static void ensureDirectory(File directory) throws IOException {
        if (!directory.exists() && !directory.mkdirs()) {
            throw new IOException("unable to create child composition directory: " + directory);
        }
        if (!directory.isDirectory()) {
            throw new IOException("child composition path is not a directory: " + directory);
        }
    }
}
