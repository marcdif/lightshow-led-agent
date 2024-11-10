use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4}, sync::Arc
};
use axum::{extract::ConnectInfo, http::StatusCode, response::{Html, IntoResponse}, routing::get, Extension, Router};
use chrono::{DateTime, Local};
use hostname;
use lazy_static::lazy_static;
use axum_server::tls_rustls::RustlsConfig;

lazy_static! {
    static ref HOSTNAME: String = hostname::get().expect("Failed to get hostname").into_string().unwrap();
}

pub async fn start_http_webserver(port: u16, shared_monitor_responses: Arc<tokio::sync::RwLock<Vec<MonitorResponse>>>) {
    println!("Starting HTTP web server on 0.0.0.0:{}...", port);

    let shared_data = shared_monitor_responses;
    let app = Router::new()
        .route("/", get(handle_connection))
        .layer(Extension(shared_data.clone()))
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    axum_server::bind(std::net::SocketAddr::V4(addr))
        .serve(app)
        .await
        .unwrap();
}

pub async fn start_https_webserver(port: u16, shared_monitor_responses: Arc<tokio::sync::RwLock<Vec<MonitorResponse>>>) {
    println!("Starting HTTPS web server on 0.0.0.0:{}...", port);

    let shared_data = shared_monitor_responses;
    let app = Router::new()
        .route("/", get(handle_connection))
        .layer(Extension(shared_data.clone()))
        .into_make_service_with_connect_info::<SocketAddr>();

    let config = RustlsConfig::from_pem_file(
        "cert.pem",
        "key.pem",
    ).await.unwrap();

    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    axum_server::tls_rustls::bind_rustls(std::net::SocketAddr::V4(addr), config)
        .serve(app)
        .await
        .unwrap();
}

async fn handle_connection(ConnectInfo(_addr): ConnectInfo<SocketAddr>, Extension(shared_monitor_responses): Extension<Arc<tokio::sync::RwLock<Vec<MonitorResponse>>>>) -> impl IntoResponse {
    let responses: Vec<MonitorResponse> = shared_monitor_responses.read().await.to_vec();

    let full_response = format_html_response(responses);
    let status_code = &full_response[..3];
    let html_response = &full_response[3..];

    if status_code == "200" {
        (StatusCode::OK, Html(html_response.to_string()))    
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Html(html_response.to_string()))    
    }
}

fn format_html_response(responses: Vec<MonitorResponse>) -> String {
    let mut port_string = String::new();
    let mut status_table_rows = String::new();
    let mut counter: i32 = 1;
    let mut all_healthy = true;
    for response in &responses {
        port_string.push_str(&format!("{} ", response.monitor.port));
        if response.healthy {
            if response.response_time < 1000 {
                status_table_rows.push_str(&format!("<tr class=\"good-status\"><td>{}</td><td>{}</td><td>GOOD</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td>", counter, response.monitor.port, response.monitor.protocol.to_ascii_uppercase(), response.response_time, response.monitor.uri, response.failure_reason));
            } else {
                status_table_rows.push_str(&format!("<tr class=\"slow-status\"><td>{}</td><td>{}</td><td>SLOW</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td>", counter, response.monitor.port, response.monitor.protocol.to_ascii_uppercase(), response.response_time, response.monitor.uri, response.failure_reason));
            }
        } else {
            all_healthy = false;
            status_table_rows.push_str(&format!("<tr class=\"bad-status\"><td>{}</td><td>{}</td><td>BAD</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td>", counter, response.monitor.port, response.monitor.protocol.to_ascii_uppercase(), response.response_time, response.monitor.uri, response.failure_reason));
        }
        counter += 1;
    }

    let status_code_short = if all_healthy { 200 } else { 503 };
    let local_now: DateTime<Local> = Local::now();

    let header = format!("<head><title>{} {}</title></head>", status_code_short, HOSTNAME.as_str());
    let sysinfo = format!("<p style=\"font-family:verdana;color:grey;font-size:10px;\">{} &nbsp; {}</p>", local_now.format("%Y-%m-%dT%H:%M:%S%.f%:z"), HOSTNAME.as_str());
    let summary = format!("<p style=\"font-family:verdana;font-size:15px;\">Checked for ports [{}]</p>", port_string.trim());
    let status_table = format!("<style>table{{width:100%;border-collapse:collapse;}}th,td{{border:1px solid black;padding:8px;text-align:left;}}.good-status{{background-color:#eaffea;color:#006000;}}.slow-status{{background-color:#e5fa98;color:#db8c02;font-weight:bold;}}.bad-status{{background-color:#ffeaea;color:#800000;font-weight:bold;}}</style><table><tr><th>Index</th><th>Port</th><th>Status</th><th>Protocol</th><th>Response Time (ms)</th><th>URI</th><th>Failure Reason</th></tr>{}</table>", status_table_rows);

    return format!("{}{}<html><body style=\"max-width:1200px;\">{}{}{}</body></html>\r\n", status_code_short, header, sysinfo, summary, status_table);
}
