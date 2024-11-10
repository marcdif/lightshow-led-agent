use rs_ws281x::{ChannelBuilder, ControllerBuilder, StripType};
use std::borrow::BorrowMut;

use clap::Parser;

use crate::stage::Stage;

mod scheduler;
mod stage;
mod utils;

#[derive(Parser)]
#[command(name = "agent")]
#[command(about = "LED Controller Agent")]
struct Cli {
    #[arg(long, default_value = "791", help = "Count of LEDs in the system")]
    led_count: i32,

    #[arg(long, default_value = "18", help = "GPIO pin connected to data line")]
    led_pin: i32,

    #[arg(long, default_value = "255", help = "Brightness of the LEDs, from 0-255")]
    brightness: u8
}

fn main() {
    let args: Cli = Cli::parse();

    let led_count = args.led_count;
    let led_pin = args.led_pin;
    let brightness = args.brightness;

    println!("Initializing application...");
    println!("==================================================");
    println!("LED Count: {}", &led_count);
    println!("LED GPIO Pin: {}", &led_pin);
    println!("Brightness: {}", &brightness);
    println!("==================================================");

    // Create an LED Stage
    let mut stage: Stage = Stage::init([196, 393, 591, 789], [[0, 0, 0, 0]; 789]);

    println!("Stage created with {} LEDs", stage.get_full_stage().len());
    println!("Stage started in '{:?}' mode", stage.get_mode());

    // Create a new controller
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(led_pin)
                .count(led_count)
                .strip_type(StripType::Ws2811Gbr)
                .brightness(brightness)
                .build(),
        )
        .build()
        .expect("Failed to create controller");

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

    // Main scheduler loop to render display
    scheduler::start_scheduler(&mut controller.borrow_mut(), &mut stage);
}
