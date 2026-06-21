mod ffr_diff_model;

fn main() {
    println!("Universal Rhythm Engine (Rust Monolith)");
    let predictor = ffr_diff_model::predictor::DifficultyPredictor::new();
    println!("Difficulty Predictor initialized.");
}
