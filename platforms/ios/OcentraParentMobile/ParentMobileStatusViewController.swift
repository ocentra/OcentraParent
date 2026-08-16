import UIKit

private enum ParentMobileRuntimeProof {
    static let statusText = [
        "Ocentra Parent Mobile iOS scaffold",
        "bundle-id=ca.ocentra.parent.mobile",
        "observer=read-only",
        "controller-authority=manual-required",
        "child-agent-parity=not-claimed",
        "testflight=manual-required",
    ].joined(separator: "\n")
}

final class ParentMobileStatusViewController: UIViewController {
    override func loadView() {
        let label = UILabel()
        label.backgroundColor = .systemBackground
        label.text = ParentMobileRuntimeProof.statusText
        label.textAlignment = .center
        label.textColor = .label
        label.numberOfLines = 0
        view = label
    }
}
