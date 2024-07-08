use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};
use std::{borrow::BorrowMut, time::Instant};

// LED strip configuration
const LED_COUNT: i32 = 790;
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
fn rainbow_cycle(controller: &mut Controller) {
    for j in 0..255 {
        let mut colors = Vec::with_capacity(LED_COUNT.try_into().unwrap());
        for i in 0..LED_COUNT {
            let pixel_index = (i as u32 * 256 / LED_COUNT as u32) + j as u32;
            let color = wheel((pixel_index & 255) as u8);
            colors.push(color);
        }
        let leds = controller.leds_mut(0);
        for (led, &color) in leds.iter_mut().zip(colors.iter()) {
            *led = [color.0, color.1, color.2, 0];
        }
        let start_time = Instant::now();
        controller.render().expect("Failed to render LEDs");
        let elapsed_ns = start_time.elapsed().as_millis();
        println!("{}ms passed", &elapsed_ns);
        // std::thread::sleep(std::time::Duration::from_millis(WAIT_TIME));
    }
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
    loop {
        rainbow_cycle(&mut controller.borrow_mut());
    }
}


// use rs_ws281x::{ChannelBuilder, ControllerBuilder, StripType};
// use std::thread;
// use std::time::Duration;

// // Define a struct to represent a color
// #[derive(Copy, Clone)]
// struct Color {
//     r: u8,
//     g: u8,
//     b: u8,
// }

// impl Color {
//     fn new(r: u8, g: u8, b: u8) -> Self {
//         Self { r, g, b }
//     }
// }

// fn main() {
//     // Define the number of LEDs in the strip
//     let led_count: i32 = 10;

//     // Create a new controller
//     let mut controller = ControllerBuilder::new()
//         .freq(800_000)
//         .dma(10)
//         .channel(
//             0,
//             ChannelBuilder::new()
//                 .pin(18)
//                 .count(led_count)
//                 .strip_type(StripType::Ws2811Rgb)
//                 .brightness(255)
//                 .build(),
//         )
//         .build()
//         .expect("Failed to create controller");

//     let mut colors = vec![
//         Color::new(255, 0, 0),     // Red
//         Color::new(0, 255, 0),     // Green
//         Color::new(0, 0, 255),     // Blue
//         Color::new(255, 255, 0),   // Yellow
//         Color::new(0, 255, 255),   // Cyan
//         Color::new(255, 0, 255),   // Magenta
//         Color::new(255, 255, 255), // White
//         Color::new(128, 128, 128), // Gray
//         Color::new(255, 128, 0),   // Orange
//         Color::new(128, 0, 255),   // Purple
//     ];

//     // Ensure the color vector has enough colors for all LEDs
//     // while colors.len() < led_count.try_into().unwrap() {
//     //     colors.extend_from_slice(&colors[0..(led_count - colors.len())]);
//     // }

//     let leds = controller.leds_mut(0);

//     for (i, led) in leds.iter_mut().enumerate() {
//         let color = colors[i];
//         *led = [color.r, color.g, color.b, 0]; // Assign color with an alpha channel
//     }

//     controller.render().expect("Failed to render LEDs");

//     // Keep the program running to keep the LEDs on
//     thread::sleep(Duration::from_secs(10));
// }

// // use rs_ws281x::ControllerBuilder;
// // use rs_ws281x::ChannelBuilder;
// // use rs_ws281x::StripType;

// // fn main() {
// //     // Construct a single channel controller. Note that the
// //     // Controller is initialized by default and is cleaned up on drop

// //     let mut controller = ControllerBuilder::new()
// //         .freq(800_000)
// //         .dma(10)
// //         .channel(
// //             0, // Channel Index
// //             ChannelBuilder::new()
// //                 .pin(18) // GPIO 10 = SPI0 MOSI
// //                 .count(64) // Number of LEDs
// //                 .strip_type(StripType::Ws2812)
// //                 .brightness(255) // default: 255
// //                 .build(),
// //         )
// //         .build()
// //         .unwrap();

// //     let leds = controller.leds_mut(0);

// //     for led in leds {
// //         *led = [0, 0, 255, 0];
// //     }

// //     controller.render().unwrap();
// // }
