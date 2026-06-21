use std::collections::BTreeMap;
use crate::ffr_diff_model::features::OrderedFloat;

pub struct PreprocessedChart {
    pub name: String,
    pub mode: String,
    pub difficulty: String,
    pub meter: f64,
    pub chart: BTreeMap<OrderedFloat, String>,
}

pub struct SMChartPreprocessor {
    pub decimals: u32,
}

impl SMChartPreprocessor {
    pub fn new(decimals: u32) -> Self {
        Self { decimals }
    }

    pub fn preprocess(&self, _sm_file_path: &str) -> Vec<PreprocessedChart> {
        // TODO: Full simfile parsing goes here.
        // For now, return an empty array indicating the stubbed integration is ready.
        Vec::new()
    }
}
