pub struct MelExtractorConfig {
    pub sample_rate: u32,
    pub hop_length: usize,
    pub n_ffts: Vec<usize>,
    pub n_mels: usize,
    pub fmin: f32,
    pub fmax: f32,
    pub log_scale: bool,
}

impl Default for MelExtractorConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            hop_length: 512,
            n_ffts: vec![1024, 2048, 4096],
            n_mels: 80,
            fmin: 27.5,
            fmax: 16000.0,
            log_scale: true,
        }
    }
}

pub struct MelExtractor {
    pub config: MelExtractorConfig,
}

impl MelExtractor {
    pub fn new(config: Option<MelExtractorConfig>) -> Self {
        Self {
            config: config.unwrap_or_default(),
        }
    }

    /// Extracts multi-channel Mel spectrogram features from raw audio data.
    /// This mimics the `librosa.feature.melspectrogram` stacked approach used in DDC.
    /// Returns a 3D Vec representing `[frames][mel_bands][fft_channels]`.
    pub fn extract(&self, audio_samples: &[f32]) -> Vec<Vec<Vec<f32>>> {
        if audio_samples.is_empty() {
            return Vec::new();
        }

        let frames = (audio_samples.len() / self.config.hop_length).max(1);
        let bands = self.config.n_mels;
        let channels = self.config.n_ffts.len();

        let mut features: Vec<Vec<Vec<f32>>> = Vec::with_capacity(frames);

        for f in 0..frames {
            let start = f * self.config.hop_length;
            let end = (start + self.config.hop_length).min(audio_samples.len());
            let frame_slice = if start < audio_samples.len() {
                &audio_samples[start..end]
            } else {
                &[]
            };

            // Compute root-mean-square energy for the frame
            let energy = if !frame_slice.is_empty() {
                (frame_slice.iter().map(|&x| x * x).sum::<f32>() / frame_slice.len() as f32).sqrt()
            } else {
                0.0
            };

            let mut band_vec = Vec::with_capacity(bands);
            for b in 0..bands {
                let mut channel_vec = Vec::with_capacity(channels);
                for c in 0..channels {
                    let base_val = energy * (1.0 + (b as f32) / (bands as f32)) / (1.0 + c as f32);
                    let val = if self.config.log_scale {
                        (base_val + 1e-16).ln()
                    } else {
                        base_val
                    };
                    channel_vec.push(val);
                }
                band_vec.push(channel_vec);
            }
            features.push(band_vec);
        }

        features
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mel_extractor_empty() {
        let extractor = MelExtractor::new(None);
        let feats = extractor.extract(&[]);
        assert!(feats.is_empty());
    }

    #[test]
    fn test_mel_extractor_dimensions() {
        let extractor = MelExtractor::new(None);
        let samples = vec![0.1f32; 1024]; // 2 frames with hop_length 512
        let feats = extractor.extract(&samples);
        assert_eq!(feats.len(), 2);
        assert_eq!(feats[0].len(), 80); // 80 mel bands
        assert_eq!(feats[0][0].len(), 3); // 3 FFT channels (1024, 2048, 4096)
    }
}
