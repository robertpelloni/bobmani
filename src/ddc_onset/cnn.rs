use tract_onnx::prelude::*;
use std::path::PathBuf;

pub const NUM_MEL_BANDS: usize = 80;
pub const NUM_FFT_FRAME_LENGTHS: usize = 3;

pub struct Conv2dDef {
    pub in_channels: usize,
    pub out_channels: usize,
    pub kernel_size: (usize, usize),
}

pub struct MaxPool2dDef {
    pub kernel_size: (usize, usize),
    pub stride: (usize, usize),
}

pub struct LinearDef {
    pub in_features: usize,
    pub out_features: usize,
}

pub struct SpectrogramNormalizer {
    pub load_moments: bool,
    pub mean: Vec<f32>,
    pub std: Vec<f32>,
}

impl SpectrogramNormalizer {
    pub fn new(load_moments: bool) -> Self {
        let size = NUM_MEL_BANDS * NUM_FFT_FRAME_LENGTHS;
        Self {
            load_moments,
            mean: vec![0.0; size],
            std: vec![1.0; size],
        }
    }

    pub fn forward(&self, x: &[f32]) -> Vec<f32> {
        if x.len() != self.mean.len() {
            return x.to_vec();
        }

        let mut out = Vec::with_capacity(x.len());
        for i in 0..x.len() {
            out.push((x[i] - self.mean[i]) / self.std[i]);
        }
        out
    }
}

pub const FEATURE_CONTEXT_RADIUS_1: usize = 7;
pub const FEATURE_CONTEXT_RADIUS_2: usize = 3;

// The struct is used to outline the PyTorch architecture translated into safe Rust definitions.
pub struct PlacementCNN {
    pub load_pretrained_weights: bool,
    pub conv0: Conv2dDef,
    pub maxpool0: MaxPool2dDef,
    pub conv1: Conv2dDef,
    pub maxpool1: MaxPool2dDef,
    pub dense0: LinearDef,
    pub dense1: LinearDef,
    pub output: LinearDef,

    // Store the tract model explicitly as `Option<TractModel>` wrapper type equivalent
    pub onnx_model: Option<RunnableModel<TypedFact, Box<dyn TypedOp>>>,
}

impl PlacementCNN {
    pub fn new(load_pretrained_weights: bool) -> Self {
        Self {
            load_pretrained_weights,
            conv0: Conv2dDef { in_channels: 3, out_channels: 10, kernel_size: (7, 3) },
            maxpool0: MaxPool2dDef { kernel_size: (1, 3), stride: (1, 3) },
            conv1: Conv2dDef { in_channels: 10, out_channels: 20, kernel_size: (3, 3) },
            maxpool1: MaxPool2dDef { kernel_size: (1, 3), stride: (1, 3) },
            dense0: LinearDef { in_features: 1125, out_features: 256 },
            dense1: LinearDef { in_features: 256, out_features: 128 },
            output: LinearDef { in_features: 128, out_features: 1 },
            onnx_model: None, // Will be bound during AutoChart execution initialization
        }
    }

    pub fn forward(
        &self,
        x: &[f32],
        _difficulties: &[i64],
        _output_logits: bool,
    ) -> Vec<f32> {
        Vec::new() // Will be implemented in the model executor wrapper directly
    }
}
