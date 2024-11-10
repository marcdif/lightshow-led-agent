use std::sync::{Arc, RwLock};

use clap::Parser;

use crate::stage::Stage;

mod scheduler;
mod stage;
mod utils;
mod webserver;

#[derive(Parser)]
#[command(name = "agent")]
#[command(about = "LED Controller Agent")]
struct Cli {
    #[arg(long, default_value = "791", help = "Count of LEDs in the system")]
    led_count: i32,

    #[arg(long, default_value = "18", help = "GPIO pin connected to data line")]
    led_pin: i32,

    #[arg(long, default_value = "255", help = "Brightness of the LEDs, from 0-255")]
    brightness: u8,

    #[arg(long, default_value = "8080", help = "HTTP web server port")]
    http_port: u16,
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();

    let led_count = args.led_count;
    let led_pin = args.led_pin;
    let brightness = args.brightness;
    let http_port = args.http_port;

    println!("Initializing application...");
    println!("==================================================");
    println!("LED Count: {}", &led_count);
    println!("LED GPIO Pin: {}", &led_pin);
    println!("Brightness: {}", &brightness);
    println!("HTTP Port: {}", &http_port);
    println!("==================================================");

    // Create an LED Stage
    let stage: Stage = Stage::init([196, 393, 591, 789], [[0, 0, 0, 0]; 789]);

    println!("Stage created with {} LEDs", stage.get_full_stage().len());
    println!("Stage started in '{:?}' mode", stage.get_mode());

    println!("Started!");

    // let leds = controller.leds_mut(0);

    // leds[1] = [0, 255, 0, 0];
    // leds[196] = [0, 0, 255, 0]; // 196

    // leds[197] = [0, 255, 0, 0];
    // leds[393] = [0, 0, 255, 0]; // 197

    // leds[394] = [0, 255, 0, 0];
    // leds[591] = [0, 0, 255, 0]; // 198

    // leds[592] = [0, 255, 0, 0];
    // leds[789] = [0, 0, 255, 0]; // 198

    // controller.render().expect("Failed to render LEDs");

    // stage.get_wall(0);
    // stage.get_wall(1);
    // stage.get_wall(2);
    // stage.get_wall(3);

    let shared_stage = Arc::new(RwLock::new(stage));
    let stage_controller = Arc::clone(&shared_stage);
    let stage_http = Arc::clone(&shared_stage);

    // Main scheduler loop to render display
    println!("Starting LED Controller Thread...");
    let controller_thread = tokio::spawn(scheduler::start_scheduler(stage_controller, led_count, led_pin, brightness));

    println!("Starting web server...");
    let http = tokio::spawn(webserver::start_http_webserver(http_port, stage_http));

    let _ = tokio::join!(controller_thread, http);
}
