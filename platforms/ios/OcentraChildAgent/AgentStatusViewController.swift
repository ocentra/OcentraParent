import UIKit

private enum ChildIosEntitlementCapabilityProof {
    static let schemaVersion = "child-ios-entitlement-capability-proof"
    static let bundleId = "ca.ocentra.child.agent"
    static let capabilityOnlyState = "service-mode=capability-only"
    static let launchAvailabilityState = "launch-availability=manual-required"
    static let recoveryState = "recovery=not-implemented"
    static let familyControlsState = "family-controls=manual-required"
    static let deviceActivityState = "device-activity=manual-required"
    static let screenTimeState = "screen-time=manual-required"
    static let networkExtensionState = "network-extension=manual-required"
    static let notificationsState = "notifications=manual-required"
    static let backgroundExecutionState = "background-execution=manual-required"
    static let provisioningState = "provisioning=manual-required"
    static let supervisionState = "supervision=manual-required"
    static let signingState = "signing=manual-required"
    static let testFlightState = "testflight=manual-required"
    static let deviceProofState = "device-proof=manual-required"
    static let daemonState = "daemon=not-claimed"
    static let childAgentParityState = "child-agent-parity=not-claimed"
    static let statusText = [
        "Ocentra Child Agent iOS capability scaffold",
        schemaVersion,
        bundleId,
        capabilityOnlyState,
        launchAvailabilityState,
        recoveryState,
        familyControlsState,
        deviceActivityState,
        screenTimeState,
        networkExtensionState,
        notificationsState,
        backgroundExecutionState,
        provisioningState,
        supervisionState,
        signingState,
        testFlightState,
        deviceProofState,
        daemonState,
        childAgentParityState,
    ].joined(separator: "\n")
}

final class AgentStatusViewController: UIViewController {
    override func loadView() {
        let label = UILabel()
        label.backgroundColor = .systemBackground
        label.text = ChildIosEntitlementCapabilityProof.statusText
        label.textAlignment = .center
        label.textColor = .label
        label.numberOfLines = 0
        view = label
    }
}
