use tract_onnx::prelude::*;
use std::sync::Arc;

pub struct PlacementCNN {
    model: Arc<TypedRunnableModel>,
}

impl PlacementCNN {
    pub fn new(model_path: &str) -> TractResult<Self> {
        let model = tract_onnx::onnx()
            .model_for_path(model_path)?
            .into_optimized()?
            .into_runnable()?;
        Ok(Self { model })
    }

    pub fn predict(&self, input: Tensor) -> TractResult<Tensor> {
        let result = self.model.run(tvec!(input.into()))?;
        Ok(result[0].clone().into_tensor())
    }

    /// Runs the ONNX model on a 1-D onset-salience curve and flattens its
    /// output to scores. Input is fed as shape [1, len]; if the model's
    /// expected shape differs, tract returns an error and callers fall back.
    pub fn predict_salience(&self, salience: &[f32]) -> TractResult<Vec<f32>> {
        let input = Tensor::from_shape(&[1, salience.len()], salience)?;
        let out = self.predict(input)?;
        let view = out.to_plain_array_view::<f32>()?;
        Ok(view.iter().copied().collect())
    }
}

/// Default ONNX placement-model location: $BOBMANI_ONNX_MODEL, else
/// `models/placement.onnx` next to the working directory. No model ships
/// with the repo today, so this normally returns None.
pub fn default_model_path() -> Option<std::path::PathBuf> {
    if let Ok(p) = std::env::var("BOBMANI_ONNX_MODEL") {
        let pb = std::path::PathBuf::from(p);
        if pb.exists() {
            return Some(pb);
        }
    }
    let pb = std::path::PathBuf::from("models/placement.onnx");
    if pb.exists() {
        Some(pb)
    } else {
        None
    }
}

/// Honest fallback when no ONNX model is loaded: placements are the
/// thresholded onset peaks. Never presented as neural inference.
pub fn heuristic_placements(onset_salience: &[f32], threshold: f32) -> Vec<usize> {
    let peaks = crate::ddc_onset::util::find_peaks(onset_salience);
    crate::ddc_onset::util::threshold_peaks(onset_salience, &peaks, threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heuristic_placements_thresholds_peaks() {
        let s = vec![0.0, 0.1, 0.9, 0.2, 0.0, 0.8, 0.1, 0.0];
        assert_eq!(heuristic_placements(&s, 0.5), vec![2, 5]);
        assert!(heuristic_placements(&s, 0.95).is_empty());
        assert!(heuristic_placements(&[], 0.5).is_empty());
    }
}
