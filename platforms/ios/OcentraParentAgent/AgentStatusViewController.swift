import UIKit

final class AgentStatusViewController: UIViewController {
    override func loadView() {
        let label = UILabel()
        label.backgroundColor = .systemBackground
        label.text = "Ocentra Parent Agent iOS scaffold"
        label.textAlignment = .center
        label.textColor = .label
        label.numberOfLines = 0
        view = label
    }
}
