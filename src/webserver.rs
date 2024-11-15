use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4}, sync::{Arc, RwLock}
};
use axum::{http::StatusCode, response::{Html, IntoResponse}, routing::{get, post}, Extension, extract::Json, Router};
use chrono::{DateTime, Local};
use hostname;
use lazy_static::lazy_static;
use serde::Deserialize;
use strum::IntoEnumIterator;

use crate::stage::{Mode, Stage};

lazy_static! {
    static ref HOSTNAME: String = hostname::get().expect("Failed to get hostname").into_string().unwrap();
}

pub async fn start_http_webserver(port: u16, stage: Arc<RwLock<Stage>>) {
    println!("Starting HTTP web server on 0.0.0.0:{}...", port);

    let app = Router::new()
        .route("/", get(get_status))
        .route("/setmode", post(set_mode))
        .route("/setbrightness", post(set_brightness))
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

    let header = format!("<head><title>LED Controller</title><style>a,abbr,acronym,address,applet,article,aside,audio,b,big,blockquote,body,canvas,caption,center,cite,code,dd,del,details,dfn,div,dl,dt,em,embed,fieldset,figcaption,figure,footer,form,h1,h2,h3,h4,h5,h6,header,hgroup,html,i,iframe,img,ins,kbd,label,legend,li,mark,menu,nav,object,ol,output,p,pre,q,ruby,s,samp,section,small,span,strike,strong,sub,summary,sup,table,tbody,td,tfoot,th,thead,time,tr,tt,u,ul,var,video{{margin:0;padding:0;border:0;font-size:100%;font:inherit;vertical-align:baseline}}</style></head>");
    let sysinfo = format!("<p style=\"font-family:verdana;color:grey;font-size:36px;\">{} &nbsp; {}</p>", local_now.format("%Y-%m-%dT%H:%M:%S%.f%:z"), HOSTNAME.as_str());
    let stageinfo = format!("<p style=\"font-family:verdana;color:aqua;font-size:36px;\">Current Stage Mode: {:?}</p>", stage.read().unwrap().get_mode());
    let change_stage_brightness_header = "<p>Change Stage Brightness:</p>";
    let brightness_slider = "<input id=\"brightness_slider\" type=\"range\" min=\"0\" max=\"255\" value=\"255\">";
    let change_stage_brightness_script = "<script> document.getElementById(\"brightness_slider\").onchange = function () { fetch(\"/setbrightness\", { method: \"POST\", headers: { \"Content-Type\": \"application/json\", }, body: JSON.stringify({ brightness: parseInt(this.value, 10) }) })};</script>";
    let change_stage_mode_header = "<p>Change Stage Mode:</p>";
    let mut mode_options = String::new();
    for mode in Mode::iter() {
        let mode_string: String = mode.to_string();
        mode_options.push_str(&format!("<div style=\"width:100%;height:25%;background-color:{};color:{};font-weight:bold;text-align:center;cursor:pointer\"><p id={} style=\"height: 67%;padding-top: 33px;\">{}</p></div>", mode.css_button_color(), mode.css_text_color(), mode_string.to_lowercase(), mode_string.to_uppercase()));
    }
    let change_stage_mode_script = "<script>document.addEventListener(\"click\",(event)=>{const knownIds=[\"off\",\"tealwave\",\"white\",\"red\",\"orange\",\"yellow\",\"green\",\"blue\",\"pink\",\"purple\",\"rainbow\"];if(knownIds.includes(event.target.id)){fetch(\"/setmode\",{method:\"POST\",headers:{\"Content-Type\":\"application/json\"},body:JSON.stringify({mode:event.target.id})})}});</script>";
    return (StatusCode::OK, Html(format!("<html>{}<body style=\"width:100%;background-color:#282828;color:lightgrey;font-family:verdana;font-size:48px;color:aqua\">{}{}{}{}{}{}{}{}</body></html>\r\n", header, sysinfo, stageinfo, change_stage_brightness_header, brightness_slider, change_stage_mode_header, mode_options, change_stage_brightness_script, change_stage_mode_script)));
}

#[derive(Deserialize)]
struct ModeRequest {
    mode: String
}

#[derive(Deserialize)]
struct BrightnessRequest {
    brightness: u8
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

async fn set_brightness(Extension(stage): Extension<Arc<RwLock<Stage>>>, Json(payload): Json<BrightnessRequest>) -> impl IntoResponse {
    println!("Received request to change Stage Brightness to '{}'", payload.brightness);
    let new_brightness = payload.brightness;
    stage.write().unwrap().set_brightness(new_brightness);
    println!("Successfully changed Stage Brightness to '{}'", new_brightness);
    return (StatusCode::OK, Json("{success:true}"));
}
