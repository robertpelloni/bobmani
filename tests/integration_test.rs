use app::ffr_diff_model::predictor::DifficultyPredictor;
use app::ddc::autochart::AutoChart;
use app::ddc_onset::cnn::PlacementCNN;
use app::arrowvortex::simfile::Simfile;
use std::path::PathBuf;

#[test]
fn test_unification_architecture() {
    // Test that FFR Difficulty Predictor initializes properly
    let predictor = DifficultyPredictor::new();
    assert_eq!(predictor.alpha, 2.0);

    // Test that DDC AutoChart initializes without panics
    let models_dir = PathBuf::from("fake_models");
    let autochart = AutoChart::new(models_dir, None, None, None);
    assert!(autochart.google_key.is_none());

    // Test that DDC Onset Placement CNN boundaries initialize
    let placement_cnn = PlacementCNN::new(false);
    assert_eq!(placement_cnn.conv0.in_channels, 3);
    assert_eq!(placement_cnn.conv0.out_channels, 10);
    assert_eq!(placement_cnn.dense0.in_features, 1125);

    // Test ArrowVortex base Simfile creation bounds
    let simfile = Simfile::new();
    assert!(simfile.title.is_empty());
}
