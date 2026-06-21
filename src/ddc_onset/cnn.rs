pub const NUM_MEL_BANDS: usize = 80;
pub const NUM_FFT_FRAME_LENGTHS: usize = 3;

/// Normalizes log-Mel spectrograms to zero mean and unit variance per bin.
pub struct SpectrogramNormalizer {
    pub load_moments: bool,
}

impl SpectrogramNormalizer {
    pub fn new(load_moments: bool) -> Self {
        Self { load_moments }
    }

    pub fn forward(&self, x: &[f32]) -> Vec<f32> {
        // TODO: In the future, parse the .bin weights and normalize: (x - mean) / std
        x.to_vec()
    }
}

pub const FEATURE_CONTEXT_RADIUS_1: usize = 7;
pub const FEATURE_CONTEXT_RADIUS_2: usize = 3;

/// Predicts placement scores from log-Mel spectrograms.
pub struct PlacementCNN {
    pub load_pretrained_weights: bool,
    // Note: Rust needs a crate like `tch` (PyTorch bindings for Rust) or `tract` to run
    // real NN computation without porting manual Convolution kernels.
    // The structural skeleton here establishes the boundaries.

    // conv0: nn.Conv2d(3, 10, (7, 3))
    // maxpool0: nn.MaxPool2d((1, 3), (1, 3))
    // conv1: nn.Conv2d(10, 20, (3, 3))
    // maxpool1: nn.MaxPool2d((1, 3), (1, 3))
    // dense0: nn.Linear(1125, 256)
    // dense1: nn.Linear(256, 128)
    // output: nn.Linear(128, 1)
}

impl PlacementCNN {
    pub fn new(load_pretrained_weights: bool) -> Self {
        Self { load_pretrained_weights }
    }

    pub fn forward(
        &self,
        _x: &[f32], // shape: [num_frames, num_mel_bands (80), num_fft_frame_lengths (3)]
        _difficulties: &[i64], // DDR difficulty labels
        _output_logits: bool,
    ) -> Vec<f32> { // Returns shape: [batch_size, num_frames]

        // TODO: Implement PyTorch tensor bindings for forward pass
        Vec::new()
    }
}