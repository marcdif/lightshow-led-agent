use rs_ws281x::{ChannelBuilder, ControllerBuilder, StripType};
use std::borrow::BorrowMut;

use crate::stage::Stage;

mod scheduler;
mod stage;
mod utils;

// LED strip configuration
const LED_COUNT: i32 = 791;
const LED_PIN: i32 = 18; // GPIO pin connected to data line
const BRIGHTNESS: u8 = 255; // Set brightness (0 to 255)

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
                .strip_type(StripType::Ws2811Gbr)
                .brightness(BRIGHTNESS)
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

    let mut stage: Stage = Stage::init([196, 393, 591, 789], [[0, 0, 0, 0]; 789]);

    // stage.get_wall(0);
    // stage.get_wall(1);
    // stage.get_wall(2);
    // stage.get_wall(3);

    // Main scheduler loop to render display
    scheduler::start_scheduler(&mut controller.borrow_mut(), &mut stage);
}
