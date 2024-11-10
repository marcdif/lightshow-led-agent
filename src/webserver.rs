use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4}, sync::Arc
};
use axum::{extract::ConnectInfo, http::StatusCode, response::{Html, IntoResponse}, routing::{get, post}, Extension, Router};
use chrono::{DateTime, Local};
use hostname;
use lazy_static::lazy_static;

use crate::stage::Stage;

lazy_static! {
    static ref HOSTNAME: String = hostname::get().expect("Failed to get hostname").into_string().unwrap();
}

pub async fn start_http_webserver(port: u16, stage: Arc<Stage>) {
    println!("Starting HTTP web server on 0.0.0.0:{}...", port);

    let shared_data = stage;
    let app = Router::new()
        .route("/", get(get_status))
        .route("/set_mode", post(set_mode))
        .layer(Extension(shared_data.clone()))
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    axum_server::bind(std::net::SocketAddr::V4(addr))
        .serve(app)
        .await
        .unwrap();
}

async fn get_status(ConnectInfo(_addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    let status_code_short = 200;
    let local_now: DateTime<Local> = Local::now();

    let header = format!("<head><title>{} {}</title></head>", status_code_short, HOSTNAME.as_str());
    let sysinfo = format!("<p style=\"font-family:verdana;color:grey;font-size:10px;\">{} &nbsp; {}</p>", local_now.format("%Y-%m-%dT%H:%M:%S%.f%:z"), HOSTNAME.as_str());
    return (StatusCode::OK, Html(format!("{}{}<html><body style=\"max-width:1200px;\">{}</body></html>\r\n", status_code_short, header, sysinfo)));
}

async fn set_mode(ConnectInfo(_addr): ConnectInfo<SocketAddr>, Extension(stage): Extension<Arc<Stage>>) -> impl IntoResponse {
    return StatusCode::NOT_IMPLEMENTED;
}
