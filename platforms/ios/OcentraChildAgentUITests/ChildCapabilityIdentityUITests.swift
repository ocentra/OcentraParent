import XCTest

final class ChildCapabilityIdentityUITests: XCTestCase {
    private var app: XCUIApplication!

    override func setUpWithError() throws {
        continueAfterFailure = false
        app = XCUIApplication()
        app.launch()
        XCTAssertTrue(app.wait(for: .runningForeground, timeout: 10))
    }

    func testCapabilityStatusReportsObservedFailClosedRuntimeStates() {
        let statusView = app.textViews["child-ios-capability-status"]
        XCTAssertTrue(statusView.waitForExistence(timeout: 10))

        let rendered = (statusView.value as? String) ?? statusView.label
        XCTAssertFalse(rendered.isEmpty)
        XCTAssertTrue(rendered.contains("service-mode=capability-only"))
        XCTAssertTrue(rendered.contains("launch-availability=manual-required"))
        XCTAssertTrue(rendered.contains("recovery=not-implemented"))
        XCTAssertTrue(rendered.contains("daemon=not-claimed"))
        XCTAssertTrue(rendered.contains("external-transport=not-implemented"))

        assertObservation(in: rendered, key: "bundle-identity", allowed: ["available"])
        assertObservation(
            in: rendered,
            key: "family-controls",
            allowed: ["available", "unavailable", "manual-required"]
        )
        assertObservation(in: rendered, key: "device-activity", allowed: ["manual-required"])
        assertObservation(in: rendered, key: "screen-time", allowed: ["manual-required"])
        assertObservation(in: rendered, key: "network-extension", allowed: ["manual-required"])
        assertObservation(
            in: rendered,
            key: "notifications",
            allowed: ["available", "unavailable", "manual-required"]
        )
        assertObservation(in: rendered, key: "background-execution", allowed: ["manual-required"])
        assertObservation(in: rendered, key: "provisioning", allowed: ["manual-required"])
        assertObservation(in: rendered, key: "supervision", allowed: ["manual-required"])
        assertObservation(in: rendered, key: "signing", allowed: ["manual-required"])
        assertObservation(in: rendered, key: "testflight", allowed: ["manual-required"])
        assertObservation(in: rendered, key: "device-proof", allowed: ["manual-required"])
    }

    private func assertObservation(
        in rendered: String,
        key: String,
        allowed: Set<String>
    ) {
        let prefix = "\(key)="
        guard let line = rendered.split(separator: "\n").first(where: { $0.hasPrefix(prefix) }) else {
            XCTFail("Missing runtime observation for \(key)")
            return
        }

        guard let availability = line.split(separator: " ", maxSplits: 1).first else {
            XCTFail("Missing runtime availability for \(key)")
            return
        }

        XCTAssertTrue(
            allowed.contains(String(availability.dropFirst(prefix.count))),
            "Unexpected runtime availability for \(key): \(line)"
        )
    }
}
