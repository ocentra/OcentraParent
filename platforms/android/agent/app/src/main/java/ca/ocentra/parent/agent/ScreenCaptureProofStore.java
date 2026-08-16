package ca.ocentra.parent.agent;

import android.content.Context;
import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

public final class ScreenCaptureProofStore {
    public static final String PROOF_FILE_NAME = "screen-capture-mediaprojection-proof.json";
    private static final String RAW_TEMP_FILE_NAME = "screen-capture-frame.tmp";

    private ScreenCaptureProofStore() {}

    public static File proofFile(Context context) {
        return new File(context.getFilesDir(), PROOF_FILE_NAME);
    }

    public static void writePending(Context context) throws IOException {
        writeJson(
            proofFile(context),
            "{\n" +
            "  \"schemaVersion\": \"child-android-screen-capture-proof\",\n" +
            "  \"status\": \"pending-consent\",\n" +
            "  \"degradedIsCaptureProof\": false\n" +
            "}\n"
        );
    }

    public static void writeDenied(Context context) throws IOException {
        writeJson(
            proofFile(context),
            "{\n" +
            "  \"schemaVersion\": \"child-android-screen-capture-proof\",\n" +
            "  \"status\": \"permission-denied\",\n" +
            "  \"degradedIsCaptureProof\": false\n" +
            "}\n"
        );
    }

    public static void writeCaptured(Context context, int width, int height, byte[] frameBytes)
        throws IOException {
        File rawTemp = new File(context.getFilesDir(), RAW_TEMP_FILE_NAME);
        writeBytes(rawTemp, frameBytes);
        boolean existedBeforeDelete = rawTemp.exists();
        boolean deleted = rawTemp.delete();
        String digest = sha256Hex(frameBytes);
        String json =
            "{\n" +
            "  \"schemaVersion\": \"child-android-screen-capture-proof\",\n" +
            "  \"status\": \"captured\",\n" +
            "  \"platform\": \"android\",\n" +
            "  \"captureApi\": \"MediaProjection\",\n" +
            "  \"captureScope\": \"displayOrSelectedAppFromSystemConsent\",\n" +
            "  \"width\": " +
            width +
            ",\n" +
            "  \"height\": " +
            height +
            ",\n" +
            "  \"frameByteSize\": " +
            frameBytes.length +
            ",\n" +
            "  \"frameDigest\": \"" +
            digest +
            "\",\n" +
            "  \"rawTempExistedBeforeDelete\": " +
            existedBeforeDelete +
            ",\n" +
            "  \"rawTempExistsAfterDelete\": " +
            rawTemp.exists() +
            ",\n" +
            "  \"rawTempDeleted\": " +
            deleted +
            ",\n" +
            "  \"degradedIsCaptureProof\": false\n" +
            "}\n";
        writeJson(proofFile(context), json);
    }

    public static void writeError(Context context, String status) throws IOException {
        writeJson(
            proofFile(context),
            "{\n" +
            "  \"schemaVersion\": \"child-android-screen-capture-proof\",\n" +
            "  \"status\": \"" +
            escape(status) +
            "\",\n" +
            "  \"degradedIsCaptureProof\": false\n" +
            "}\n"
        );
    }

    private static void writeBytes(File file, byte[] bytes) throws IOException {
        try (FileOutputStream stream = new FileOutputStream(file, false)) {
            stream.write(bytes);
        }
    }

    private static void writeJson(File file, String json) throws IOException {
        writeBytes(file, json.getBytes(StandardCharsets.UTF_8));
    }

    private static String sha256Hex(byte[] bytes) {
        try {
            MessageDigest digest = MessageDigest.getInstance("SHA-256");
            byte[] hash = digest.digest(bytes);
            StringBuilder builder = new StringBuilder(hash.length * 2);
            for (byte value : hash) {
                builder.append(String.format("%02x", value));
            }
            return builder.toString();
        } catch (NoSuchAlgorithmException error) {
            throw new IllegalStateException(error);
        }
    }

    private static String escape(String value) {
        return value.replace("\\", "\\\\").replace("\"", "\\\"");
    }
}
