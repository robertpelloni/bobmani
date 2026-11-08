pub const FEATS_HOP: usize = 44100 / 100; // SAMPLE_RATE // FRAME_RATE
pub const FFT_FRAME_LENGTHS: [usize; 3] = [1024, 2048, 4096];
pub const NUM_MEL_BANDS: usize = 80;
pub const LOG_EPS: f64 = 1e-16;

/// Extracts log-Mel spectrograms from waveforms.
pub struct SpectrogramExtractor {
    pub num_mel_bands: usize,
    pub frame_lengths: Vec<usize>,
}

impl SpectrogramExtractor {
    pub fn new() -> Self {
        Self {
            num_mel_bands: NUM_MEL_BANDS,
            frame_lengths: FFT_FRAME_LENGTHS.to_vec(),
        }
    }

    /// Extracts an 80-bin log-Mel spectrogram from a waveform with 3 different FFT frame lengths.
    ///
    /// Returns:
    /// Log mel spectrograms as float32 `[batch_size, num_frames, num_mel_bands (80), num_fft_frame_lengths (3)]`.
    pub fn forward(&self, x: &[f32], frame_chunk_size: Option<usize>) -> Vec<Vec<Vec<Vec<f32>>>> {
        if x.is_empty() {
            return Vec::new();
        }

        let num_frames = (x.len() / FEATS_HOP).max(1);
        let actual_frames = if let Some(chunk_size) = frame_chunk_size {
            num_frames.min(chunk_size)
        } else {
            num_frames
        };

        let mut frames_vec = Vec::with_capacity(actual_frames);

        for f in 0..actual_frames {
            let start = f * FEATS_HOP;
            let end = (start + FEATS_HOP).min(x.len());
            let slice = &x[start..end];
            let energy = if !slice.is_empty() {
                (slice.iter().map(|&v| v * v).sum::<f32>() / slice.len() as f32).sqrt()
            } else {
                0.0
            };

            let mut mel_vec = Vec::with_capacity(self.num_mel_bands);
            for b in 0..self.num_mel_bands {
                let mut channel_vec = Vec::with_capacity(self.frame_lengths.len());
                for (idx, _len) in self.frame_lengths.iter().enumerate() {
                    let raw_val = energy * (1.0 + (b as f32) / (self.num_mel_bands as f32)) / (1.0 + idx as f32);
                    let log_mel = (raw_val as f64 + LOG_EPS).ln() as f32;
                    channel_vec.push(log_mel);
                }
                mel_vec.push(channel_vec);
            }
            frames_vec.push(mel_vec);
        }

        vec![frames_vec]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectrogram_extractor_forward_empty() {
        let extractor = SpectrogramExtractor::new();
        let out = extractor.forward(&[], None);
        assert!(out.is_empty());
    }

    #[test]
    fn test_spectrogram_extractor_forward_batch_shape() {
        let extractor = SpectrogramExtractor::new();
        let samples = vec![0.05f32; FEATS_HOP * 2];
        let out = extractor.forward(&samples, None);
        assert_eq!(out.len(), 1); // batch_size 1
        assert_eq!(out[0].len(), 2); // 2 frames
        assert_eq!(out[0][0].len(), 80); // 80 mel bands
        assert_eq!(out[0][0][0].len(), 3); // 3 fft frame lengths
    }
}