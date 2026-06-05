use ocentra_eventing::EventCompatibilityMatrix;

fn main() {
    let matrix = EventCompatibilityMatrix::ocentra_games_lineage();
    print!("{}", matrix.render_markdown());
}
