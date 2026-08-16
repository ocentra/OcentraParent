package ca.ocentra.child.agent;

import android.content.Context;
import android.os.Bundle;

import java.io.File;
import java.io.IOException;

import ca.ocentra.parent.agent.ChildAndroidServiceProtocolProof;
import ca.ocentra.parent.agent.ChildAndroidStorageProtocolProof;

public final class ChildAgentComposition implements AutoCloseable {
    public enum Readiness {
        RUST_RUNTIME_MANUAL_REQUIRED,
        FAILED,
        STOPPED
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
        private final String failureReason;

        private Health(
            Readiness readiness,
            TransportState transportState,
            String durableRoot,
            String legacyServiceState,
            String legacyStorageState,
            String failureReason
        ) {
            this.readiness = readiness;
            this.transportState = transportState;
            this.durableRoot = durableRoot;
            this.legacyServiceState = legacyServiceState;
            this.legacyStorageState = legacyStorageState;
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

        public String failureReason() {
            return failureReason;
        }
    }

    private final File durableRoot;
    private final String legacyServiceState;
    private final String legacyStorageState;
    private Readiness readiness;
    private String failureReason;

    private ChildAgentComposition(
        File durableRoot,
        String legacyServiceState,
        String legacyStorageState
    ) {
        this.durableRoot = durableRoot;
        this.legacyServiceState = legacyServiceState;
        this.legacyStorageState = legacyStorageState;
        this.readiness = Readiness.RUST_RUNTIME_MANUAL_REQUIRED;
    }

    public static ChildAgentComposition open(Context context) throws IOException {
        File durableRoot = new File(context.getFilesDir(), "child-runtime");
        ensureDirectory(durableRoot);
        Bundle serviceStatus = ChildAndroidServiceProtocolProof.createServiceProtocolBundle();
        Bundle storageStatus = ChildAndroidStorageProtocolProof.createStorageProtocolBundle();
        return new ChildAgentComposition(
            durableRoot,
            serviceStatus.getString(ChildAndroidServiceProtocolProof.FIELD_FOREGROUND_SERVICE_STATUS),
            storageStatus.getString(ChildAndroidStorageProtocolProof.FIELD_STORAGE_BRIDGE_STATE)
        );
    }

    public static ChildAgentComposition failed(Context context, String reason) {
        File durableRoot = new File(context.getFilesDir(), "child-runtime");
        ChildAgentComposition composition = new ChildAgentComposition(
            durableRoot,
            "unavailable",
            "unavailable"
        );
        composition.failureReason = reason;
        composition.readiness = Readiness.FAILED;
        return composition;
    }

    public Health health() {
        return new Health(
            readiness,
            TransportState.NOT_IMPLEMENTED,
            durableRoot.getAbsolutePath(),
            legacyServiceState,
            legacyStorageState,
            failureReason
        );
    }

    @Override
    public void close() {
        readiness = Readiness.STOPPED;
    }

    private static void ensureDirectory(File directory) throws IOException {
        if (!directory.exists() && !directory.mkdirs()) {
            throw new IOException("unable to create child composition directory: " + directory);
        }
        if (!directory.isDirectory()) {
            throw new IOException("child composition path is not a directory: " + directory);
        }
    }
}
