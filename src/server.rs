use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::net::SocketAddr;
use crate::arrowvortex::simfile::{Simfile, SimFormat};
use crate::beatoraja::score_data::ScoreData;
use crate::ddc::extract_feats::MelExtractor;
use crate::ddc_onset::util::{find_peaks, threshold_peaks};
use crate::ffr_diff_model::features::{HorizontalDensity, OrderedFloat, PatternDetector, StreamDetector};

/// Supported incoming WebSocket requests from the frontend or test clients
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "action", content = "payload")]
pub enum WsRequest {
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "status")]
    GetStatus,
    #[serde(rename = "parse_sm")]
    ParseSm { content: String },
    #[serde(rename = "analyze_density")]
    AnalyzeDensity { chart_notes: Vec<(f64, String)> },
    #[serde(rename = "extract_audio_features")]
    ExtractAudioFeatures { samples: Vec<f32> },
    #[serde(rename = "detect_onsets")]
    DetectOnsets {
        onset_salience: Vec<f32>,
        threshold: f32,
    },
    #[serde(rename = "calculate_ex_score")]
    CalculateExScore {
        epg: i32,
        lpg: i32,
        egr: i32,
        lgr: i32,
        notes: i32,
    },
}

/// Outgoing responses sent over WebSocket to clients
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum WsResponse {
    #[serde(rename = "pong")]
    Pong { timestamp: u64 },
    #[serde(rename = "status")]
    Status {
        version: String,
        healthy: bool,
        submodules: Vec<String>,
    },
    #[serde(rename = "sm_metadata")]
    SmMetadata {
        title: String,
        artist: String,
        charts_count: usize,
    },
    #[serde(rename = "density_analysis")]
    DensityAnalysis {
        nps: f64,
        length: f64,
        stream_percentage: f64,
        jack_percentage: f64,
    },
    #[serde(rename = "audio_features")]
    AudioFeatures {
        frames_count: usize,
        bands: usize,
        channels: usize,
    },
    #[serde(rename = "onset_detections")]
    OnsetDetections {
        peaks: Vec<usize>,
        filtered_peaks: Vec<usize>,
        total_frames: usize,
    },
    #[serde(rename = "score_analysis")]
    ScoreAnalysis {
        ex_score: i32,
        max_ex_score: i32,
        accuracy_rate: f64,
        is_valid: bool,
    },
    #[serde(rename = "error")]
    Error { message: String },
}

/// Dispatches an incoming text message and returns the serialized JSON response
pub fn process_message(text: &str) -> String {
    match serde_json::from_str::<WsRequest>(text) {
        Ok(WsRequest::Ping) => {
            let res = WsResponse::Pong {
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            };
            serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string())
        }
        Ok(WsRequest::GetStatus) => {
            let res = WsResponse::Status {
                version: env!("CARGO_PKG_VERSION").to_string(),
                healthy: true,
                submodules: vec![
                    "arrowvortex".to_string(),
                    "beatoraja".to_string(),
                    "bobmania".to_string(),
                    "ddc".to_string(),
                    "ddc_onset".to_string(),
                    "ffr-difficulty-model".to_string(),
                    "pianogame".to_string(),
                ],
            };
            serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string())
        }
        Ok(WsRequest::ParseSm { content }) => {
            let mut sim = Simfile::new();
            sim.format = SimFormat::Sm;
            for block in content.split('#').skip(1) {
                if let Some(colon_idx) = block.find(':') {
                    let tag = block[..colon_idx].trim().to_uppercase();
                    let val = block[colon_idx + 1..].trim_end_matches(';').trim();
                    match tag.as_str() {
                        "TITLE" => sim.title = val.to_string(),
                        "ARTIST" => sim.artist = val.to_string(),
                        _ => {}
                    }
                }
            }
            let charts_count = content.split("#NOTES:").count().saturating_sub(1);
            let res = WsResponse::SmMetadata {
                title: sim.title,
                artist: sim.artist,
                charts_count,
            };
            serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string())
        }
        Ok(WsRequest::AnalyzeDensity { chart_notes }) => {
            let mut chart = BTreeMap::new();
            for (time, row) in chart_notes {
                chart.insert(OrderedFloat(time), row);
            }

            let hd = HorizontalDensity::new(1.0).compute(&chart);
            let sd = StreamDetector::new(0.3).compute(&chart);
            let pd = PatternDetector::new(0.2).compute(&chart);

            let res = WsResponse::DensityAnalysis {
                nps: hd.get("nps").copied().unwrap_or(0.0),
                length: hd.get("length").copied().unwrap_or(0.0),
                stream_percentage: sd.get("stream_percentage").copied().unwrap_or(0.0),
                jack_percentage: pd.get("jack_percentage").copied().unwrap_or(0.0),
            };
            serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string())
        }
        Ok(WsRequest::ExtractAudioFeatures { samples }) => {
            let extractor = MelExtractor::new(None);
            let features = extractor.extract(&samples);
            let frames_count = features.len();
            let bands = if !features.is_empty() { features[0].len() } else { 0 };
            let channels = if !features.is_empty() && !features[0].is_empty() {
                features[0][0].len()
            } else {
                0
            };

            let res = WsResponse::AudioFeatures {
                frames_count,
                bands,
                channels,
            };
            serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string())
        }
        Ok(WsRequest::DetectOnsets { onset_salience, threshold }) => {
            let total_frames = onset_salience.len();
            let peaks = find_peaks(&onset_salience);
            let filtered_peaks = threshold_peaks(&onset_salience, &peaks, threshold);

            let res = WsResponse::OnsetDetections {
                peaks,
                filtered_peaks,
                total_frames,
            };
            serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string())
        }
        Ok(WsRequest::CalculateExScore { epg, lpg, egr, lgr, notes }) => {
            let mut score = ScoreData::new();
            score.epg = epg;
            score.lpg = lpg;
            score.egr = egr;
            score.lgr = lgr;
            score.notes = notes;
            score.passnotes = (epg + lpg + egr + lgr).min(notes);

            let ex_score = score.get_exscore();
            let max_ex_score = notes.max(0) * 2;
            let accuracy_rate = if max_ex_score > 0 {
                (ex_score as f64) / (max_ex_score as f64)
            } else {
                0.0
            };
            let is_valid = score.validate();

            let res = WsResponse::ScoreAnalysis {
                ex_score,
                max_ex_score,
                accuracy_rate,
                is_valid,
            };
            serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string())
        }
        Err(e) => {
            let res = WsResponse::Error {
                message: format!("Invalid request payload: {}", e),
            };
            serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string())
        }
    }
}

pub async fn run_server() {
    let app = Router::new().route("/ws", get(ws_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                let response = process_message(&text);
                if socket.send(Message::Text(response.into())).await.is_err() {
                    println!("client disconnected");
                    return;
                }
            }
            Ok(Message::Close(_)) => {
                println!("client closed connection");
                return;
            }
            Ok(_) => {}
            Err(_) => {
                println!("socket error / client disconnected");
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_ping() {
        let req = r#"{"action":"ping"}"#;
        let res_str = process_message(req);
        assert!(res_str.contains(r#""type":"pong""#));
    }

    #[test]
    fn test_process_status() {
        let req = r#"{"action":"status"}"#;
        let res_str = process_message(req);
        let expected_ver = format!(r#""version":"{}""#, env!("CARGO_PKG_VERSION"));
        assert!(res_str.contains(&expected_ver));
        assert!(res_str.contains(r#""healthy":true"#));
        assert!(res_str.contains("arrowvortex"));
    }

    #[test]
    fn test_process_parse_sm() {
        let req = "{\"action\":\"parse_sm\",\"payload\":{\"content\":\"#TITLE:Test Track;#ARTIST:Jules;#NOTES:1000;\"}}";
        let res_str = process_message(req);
        assert!(res_str.contains(r#""title":"Test Track""#));
        assert!(res_str.contains(r#""artist":"Jules""#));
        assert!(res_str.contains(r#""charts_count":1"#));
    }

    #[test]
    fn test_process_analyze_density() {
        let req = serde_json::json!({
            "action": "analyze_density",
            "payload": {
                "chart_notes": [
                    [0.0, "1000"],
                    [0.2, "0100"],
                    [0.4, "0010"],
                    [0.6, "0001"],
                    [0.8, "1000"]
                ]
            }
        }).to_string();

        let res_str = process_message(&req);
        assert!(res_str.contains(r#""type":"density_analysis""#));
        assert!(res_str.contains(r#""stream_percentage""#));
        assert!(res_str.contains(r#""jack_percentage""#));
    }

    #[test]
    fn test_process_extract_audio_features() {
        let samples = vec![0.1f32; 1024];
        let req = serde_json::json!({
            "action": "extract_audio_features",
            "payload": {
                "samples": samples
            }
        }).to_string();

        let res_str = process_message(&req);
        assert!(res_str.contains(r#""type":"audio_features""#));
        assert!(res_str.contains(r#""frames_count":2"#));
        assert!(res_str.contains(r#""bands":80"#));
        assert!(res_str.contains(r#""channels":3"#));
    }

    #[test]
    fn test_process_detect_onsets() {
        let salience = vec![0.0, 0.1, 0.9, 0.2, 0.0, 0.8, 0.1, 0.0];
        let req = serde_json::json!({
            "action": "detect_onsets",
            "payload": {
                "onset_salience": salience,
                "threshold": 0.5
            }
        }).to_string();

        let res_str = process_message(&req);
        assert!(res_str.contains(r#""type":"onset_detections""#));
        assert!(res_str.contains(r#""total_frames":8"#));
        assert!(res_str.contains(r#""peaks""#));
        assert!(res_str.contains(r#""filtered_peaks""#));
    }

    #[test]
    fn test_process_calculate_exscore() {
        let req = serde_json::json!({
            "action": "calculate_ex_score",
            "payload": {
                "epg": 50,
                "lpg": 50,
                "egr": 10,
                "lgr": 10,
                "notes": 120
            }
        }).to_string();

        let res_str = process_message(&req);
        assert!(res_str.contains(r#""type":"score_analysis""#));
        assert!(res_str.contains(r#""ex_score":220"#)); // (50+50)*2 + 10 + 10 = 220
        assert!(res_str.contains(r#""max_ex_score":240"#)); // 120 * 2 = 240
        assert!(res_str.contains(r#""is_valid":true"#));
    }

    #[test]
    fn test_process_invalid_json() {
        let req = "invalid json";
        let res_str = process_message(req);
        assert!(res_str.contains(r#""type":"error""#));
    }
}
