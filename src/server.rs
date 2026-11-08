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
    fn test_process_invalid_json() {
        let req = "invalid json";
        let res_str = process_message(req);
        assert!(res_str.contains(r#""type":"error""#));
    }
}
