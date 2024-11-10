use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4}, sync::{Arc, RwLock}
};
use axum::{http::StatusCode, response::{Html, IntoResponse}, routing::{get, post}, Extension, extract::Json, Router};
use chrono::{DateTime, Local};
use hostname;
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::stage::{Mode, Stage};

lazy_static! {
    static ref HOSTNAME: String = hostname::get().expect("Failed to get hostname").into_string().unwrap();
}

pub async fn start_http_webserver(port: u16, stage: Arc<RwLock<Stage>>) {
    println!("Starting HTTP web server on 0.0.0.0:{}...", port);

    let app = Router::new()
        .route("/", get(get_status))
        .route("/setmode", post(set_mode))
        .layer(Extension(stage))
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    axum_server::bind(std::net::SocketAddr::V4(addr))
        .serve(app)
        .await
        .unwrap();
}

async fn get_status(Extension(stage): Extension<Arc<RwLock<Stage>>>) -> impl IntoResponse {
    let local_now: DateTime<Local> = Local::now();

    let header = format!("<head><title>LED Controller</title></head>");
    let sysinfo = format!("<p style=\"font-family:verdana;color:grey;font-size:14px;\">{} &nbsp; {}</p>", local_now.format("%Y-%m-%dT%H:%M:%S%.f%:z"), HOSTNAME.as_str());
    let stageinfo = format!("<p style=\"font-family:verdana;color:aqua;font-size:14px;\">Current Stage Mode: {:?}</p>", stage.read().unwrap().get_mode());
    return (StatusCode::OK, Html(format!("{}<html><body style=\"max-width:1200px;background-color:#282828;color:lightgrey\">{}{}</body></html>\r\n", header, sysinfo, stageinfo)));
}

#[derive(Deserialize)]
struct ModeRequest {
    mode: String
}

async fn set_mode(Extension(stage): Extension<Arc<RwLock<Stage>>>, Json(payload): Json<ModeRequest>) -> impl IntoResponse {
    println!("Received request to change Stage Mode to '{}'", payload.mode);
    let new_mode_string = payload.mode;
    let new_mode: Result<Mode, _> = new_mode_string.parse();
    match new_mode {
        Ok(result) => {
            stage.write().unwrap().set_mode(result);
            println!("Successfully changed Stage Mode to '{:?}'", result);
            return (StatusCode::OK, Json("{success:true}"));
        }
        Err(_) => {
            println!("Invalid request to set Stage Mode to '{}'", new_mode_string);
            return (StatusCode::BAD_REQUEST, Json("{success:false,message:\"Invalid mode\""));
            // format!("{{success:false,message:\"Invalid mode '{}'\"}}", new_mode_string)
        }
    }
}
