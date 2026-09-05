import UIKit
import UserNotifications

#if canImport(FamilyControls)
import FamilyControls
#endif

private enum ChildAgentIOSCapabilityAvailability: String {
    case available
    case unavailable
    case manualRequired = "manual-required"
    case notImplemented = "not-implemented"
    case notClaimed = "not-claimed"
}

private struct ChildAgentIOSCapabilityObservation {
    let key: String
    let availability: ChildAgentIOSCapabilityAvailability
    let detail: String

    var rendered: String {
        "\(key)=\(availability.rawValue) (\(detail))"
    }
}

private struct ChildAgentIOSCapabilitySnapshot {
    static let schemaVersion = "child-ios-entitlement-capability-proof"
    static let expectedBundleIdentifier = "ca.ocentra.child.agent"
    static let expectedProductName = "OcentraChildAgent"

    let bundleIdentity: ChildAgentIOSCapabilityObservation
    let applicationState: String
    let familyControls: ChildAgentIOSCapabilityObservation
    let deviceActivity: ChildAgentIOSCapabilityObservation
    let screenTime: ChildAgentIOSCapabilityObservation
    let networkExtension: ChildAgentIOSCapabilityObservation
    var notifications: ChildAgentIOSCapabilityObservation
    let backgroundExecution: ChildAgentIOSCapabilityObservation
    let provisioning: ChildAgentIOSCapabilityObservation
    let supervision: ChildAgentIOSCapabilityObservation
    let signing: ChildAgentIOSCapabilityObservation
    let testFlight: ChildAgentIOSCapabilityObservation
    let deviceProof: ChildAgentIOSCapabilityObservation

    static func capture() -> ChildAgentIOSCapabilitySnapshot {
        let observedBundleIdentifier = Bundle.main.bundleIdentifier
        let bundleIdentity = ChildAgentIOSCapabilityObservation(
            key: "bundle-identity",
            availability: observedBundleIdentifier == expectedBundleIdentifier
                ? .available
                : .unavailable,
            detail: observedBundleIdentifier == expectedBundleIdentifier
                ? "expected-child-bundle"
                : "unexpected-or-missing-bundle"
        )

        return ChildAgentIOSCapabilitySnapshot(
            bundleIdentity: bundleIdentity,
            applicationState: applicationState(),
            familyControls: familyControlsAuthorization(),
            deviceActivity: ownerRequired("device-activity", "Apple DeviceActivity"),
            screenTime: ownerRequired("screen-time", "Apple Screen Time"),
            networkExtension: ownerRequired("network-extension", "Apple NetworkExtension"),
            notifications: ChildAgentIOSCapabilityObservation(
                key: "notifications",
                availability: .manualRequired,
                detail: "notification-provider-state-pending"
            ),
            backgroundExecution: backgroundExecutionDeclaration(),
            provisioning: ownerRequired("provisioning", "Apple provisioning profile"),
            supervision: ownerRequired("supervision", "supervised-device enrollment"),
            signing: ownerRequired("signing", "Apple signing entitlements"),
            testFlight: ownerRequired("testflight", "App Store Connect distribution"),
            deviceProof: ownerRequired("device-proof", "physical-device evidence")
        )
    }

    func withNotifications(
        _ observation: ChildAgentIOSCapabilityObservation
    ) -> ChildAgentIOSCapabilitySnapshot {
        var snapshot = self
        snapshot.notifications = observation
        return snapshot
    }

    var rendered: String {
        [
            "product=\(Self.expectedProductName)",
            "schema-version=\(Self.schemaVersion)",
            "bundle-id=\(Self.expectedBundleIdentifier)",
            bundleIdentity.rendered,
            "application-state=\(applicationState)",
            "service-mode=capability-only",
            "launch-availability=manual-required",
            "recovery=not-implemented",
            familyControls.rendered,
            deviceActivity.rendered,
            screenTime.rendered,
            networkExtension.rendered,
            notifications.rendered,
            backgroundExecution.rendered,
            provisioning.rendered,
            supervision.rendered,
            signing.rendered,
            testFlight.rendered,
            deviceProof.rendered,
            "daemon=not-claimed (no iOS daemon is implemented)",
            "child-agent-parity=not-claimed (no background service parity is implemented)",
            "external-transport=not-implemented (no child-agent transport is implemented)",
        ].joined(separator: "\n")
    }

    private static func applicationState() -> String {
        switch UIApplication.shared.applicationState {
        case .active:
            return "active"
        case .inactive:
            return "inactive"
        case .background:
            return "background"
        @unknown default:
            return "unavailable"
        }
    }

    private static func familyControlsAuthorization() -> ChildAgentIOSCapabilityObservation {
#if canImport(FamilyControls)
        switch AuthorizationCenter.shared.authorizationStatus {
        case .approved:
            return ChildAgentIOSCapabilityObservation(
                key: "family-controls",
                availability: .available,
                detail: "OS-authorization-observed; provider-behavior-unclaimed"
            )
        case .denied:
            return ChildAgentIOSCapabilityObservation(
                key: "family-controls",
                availability: .unavailable,
                detail: "OS-authorization-denied"
            )
        case .notDetermined:
            return ChildAgentIOSCapabilityObservation(
                key: "family-controls",
                availability: .manualRequired,
                detail: "OS-authorization-not-determined"
            )
        @unknown default:
            return ChildAgentIOSCapabilityObservation(
                key: "family-controls",
                availability: .unavailable,
                detail: "unknown-OS-authorization-state"
            )
        }
#else
        return ownerRequired("family-controls", "FamilyControls framework")
#endif
    }

    private static func backgroundExecutionDeclaration() -> ChildAgentIOSCapabilityObservation {
        let modes = Bundle.main.object(forInfoDictionaryKey: "UIBackgroundModes") as? [String]
        return ChildAgentIOSCapabilityObservation(
            key: "background-execution",
            availability: .manualRequired,
            detail: modes?.isEmpty == false
                ? "Info.plist-declaration-observed; device-runtime-proof-required"
                : "Info.plist-declaration-missing"
        )
    }

    private static func ownerRequired(
        _ key: String,
        _ owner: String
    ) -> ChildAgentIOSCapabilityObservation {
        ChildAgentIOSCapabilityObservation(
            key: key,
            availability: .manualRequired,
            detail: "owner-required: \(owner)"
        )
    }

    fileprivate static func notificationAuthorization(
        _ settings: UNNotificationSettings
    ) -> ChildAgentIOSCapabilityObservation {
        switch settings.authorizationStatus {
        case .authorized:
            return ChildAgentIOSCapabilityObservation(
                key: "notifications",
                availability: .available,
                detail: "OS-notification-authorization-observed; delivery-unclaimed"
            )
        case .denied:
            return ChildAgentIOSCapabilityObservation(
                key: "notifications",
                availability: .unavailable,
                detail: "OS-notification-authorization-denied"
            )
        case .notDetermined, .provisional, .ephemeral:
            return ChildAgentIOSCapabilityObservation(
                key: "notifications",
                availability: .manualRequired,
                detail: "OS-notification-authorization-incomplete"
            )
        @unknown default:
            return ChildAgentIOSCapabilityObservation(
                key: "notifications",
                availability: .unavailable,
                detail: "unknown-OS-notification-state"
            )
        }
    }
}

final class AgentStatusViewController: UIViewController {
    private var refreshGeneration = 0
    private var lifecycleObservers: [NSObjectProtocol] = []

    override func viewDidLoad() {
        super.viewDidLoad()

        let statusView = UITextView(frame: .zero)
        statusView.backgroundColor = .systemBackground
        statusView.textColor = .label
        statusView.font = .monospacedSystemFont(ofSize: 14, weight: .regular)
        statusView.isEditable = false
        statusView.isScrollEnabled = true
        statusView.textContainerInset = UIEdgeInsets(top: 24, left: 20, bottom: 24, right: 20)
        statusView.accessibilityIdentifier = "child-ios-capability-status"
        view = statusView

        lifecycleObservers = [
            UIApplication.didBecomeActiveNotification,
            UIApplication.willResignActiveNotification,
            UIApplication.didEnterBackgroundNotification,
        ].map { notification in
            NotificationCenter.default.addObserver(
                forName: notification,
                object: nil,
                queue: .main
            ) { [weak self] _ in
                self?.refresh()
            }
        }

        refresh()
    }

    deinit {
        lifecycleObservers.forEach(NotificationCenter.default.removeObserver)
    }

    private func refresh() {
        refreshGeneration += 1
        let generation = refreshGeneration
        let snapshot = ChildAgentIOSCapabilitySnapshot.capture()
        render(snapshot)

        UNUserNotificationCenter.current().getNotificationSettings { [weak self] settings in
            let notificationObservation = ChildAgentIOSCapabilitySnapshot.notificationAuthorization(settings)
            let resolvedSnapshot = snapshot.withNotifications(notificationObservation)
            DispatchQueue.main.async {
                guard let self, self.refreshGeneration == generation else {
                    return
                }
                self.render(resolvedSnapshot)
            }
        }
    }

    private func render(_ snapshot: ChildAgentIOSCapabilitySnapshot) {
        guard let statusView = view as? UITextView else {
            return
        }
        statusView.text = snapshot.rendered
    }
}
