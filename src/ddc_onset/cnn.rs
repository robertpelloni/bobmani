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
}
