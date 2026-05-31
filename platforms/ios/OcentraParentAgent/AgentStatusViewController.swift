import UIKit

private enum ChildIosEntitlementCapabilityProof {
    static let schemaVersion = "child-ios-entitlement-capability-proof"
    static let bundleId = "ca.ocentra.parent.agent"
    static let familyControlsState = "family-controls=manual-required"
    static let deviceActivityState = "device-activity=manual-required"
    static let screenTimeState = "screen-time=manual-required"
    static let networkExtensionState = "network-extension=manual-required"
    static let notificationsState = "notifications=manual-required"
    static let backgroundExecutionState = "background-execution=manual-required"
    static let signingState = "signing=manual-required"
    static let testFlightState = "testflight=manual-required"
    static let deviceProofState = "device-proof=manual-required"
    static let childAgentParityState = "child-agent-parity=not-claimed"
    static let statusText = [
        "Ocentra Parent Agent iOS scaffold",
        schemaVersion,
        bundleId,
        familyControlsState,
        deviceActivityState,
        screenTimeState,
        networkExtensionState,
        notificationsState,
        backgroundExecutionState,
        signingState,
        testFlightState,
        deviceProofState,
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
