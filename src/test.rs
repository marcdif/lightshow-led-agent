use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};
use std::{borrow::BorrowMut, time::Instant};

// LED strip configuration
const LED_COUNT: i32 = 500;
const LED_PIN: i32 = 18; // GPIO pin connected to data line
const BRIGHTNESS: u8 = 255; // Set brightness (0 to 255)
const WAIT_TIME: u64 = 5; // Delay between color changes (in milliseconds)

// Generate rainbow colors across 0-255 positions
fn wheel(pos: u8) -> (u8, u8, u8) {
    match pos {
        0..=85 => (pos * 3, 255 - pos * 3, 0),
        86..=170 => (255 - (pos - 85) * 3, 0, (pos - 85) * 3),
        _ => (0, (pos - 170) * 3, 255 - (pos - 170) * 3),
    }
}

// Display rainbow cycle on the LED strip
fn rainbow_cycle(controller: &mut Controller, on: bool) {
    let leds = controller.leds_mut(0);
    for led in leds {
        if on {
            *led = [255, 255, 255, 0];
        } else {
            *led = [0, 0, 0, 0];
        }
    }
    let start_time = Instant::now();
    controller.render().expect("Failed to render LEDs");
    let elapsed_ns = start_time.elapsed().as_millis();
    println!("{}ms passed", &elapsed_ns);
}

fn main() {
    // Create a new controller
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(LED_PIN)
                .count(LED_COUNT)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(BRIGHTNESS)
                .build(),
        )
        .build()
        .expect("Failed to create controller");

    // // Handle Ctrl+C to exit gracefully
    // ctrlc::set_handler(move || {
    //     controller.reset().expect("Failed to reset controller");
    //     std::process::exit(0);
    // })
    // .expect("Error setting Ctrl-C handler");

    // println!("Press Ctrl+C to exit gracefully.");

    println!("Started!");

    // Main loop to display rainbow cycle
    let mut on: bool = true;
    loop {
        on = !on;
        rainbow_cycle(&mut controller.borrow_mut(), on);
    }
}
