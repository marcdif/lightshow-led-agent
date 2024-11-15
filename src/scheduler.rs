use std::sync::{Arc, RwLock};

use rs_ws281x::{ChannelBuilder, ControllerBuilder, StripType};

use crate::{stage::{Mode, Stage}, utils};

pub async fn start_scheduler(stage: Arc<RwLock<Stage>>, led_count: i32, led_pin: i32, brightness: u8) {
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

    let mut counter: u16 = 0;
    let mut last_mode: Mode = Mode::Off;
    loop {
        // thread::sleep(Duration::from_millis(500));
        counter = counter + 1;
        if stage.read().unwrap().get_mode() != last_mode {
            counter = 0;
            println!("Resetting!");
            last_mode = stage.read().unwrap().get_mode();
        }
        if stage.read().unwrap().get_brightness_update() {
            controller.set_brightness(0, stage.read().unwrap().get_brightness());
            stage.write().unwrap().brightness_update();
        }
        let leds = controller.leds_mut(0);

        match stage.read().unwrap().get_mode() {
            Mode::Off => {
                for led in leds.iter_mut() {
                    *led = [0, 0, 0, 0];
                }
            },
            Mode::TealWave => {
                let wave_length: u16 = 158;
                let color_r: f32 = 0.0;
                let color_g: f32 = 80.0;
                let color_b: f32 = 80.0;

                if counter >= wave_length {
                    counter = 0;
                }
                let offset: usize = (counter % wave_length) as usize;
                // println!("{} {}", counter, offset);
                for i in 0..leds.len() {
                    let i_u16: u16 = i as u16 % wave_length;
                    if (i_u16 % wave_length) < (wave_length / 2) {
                        // Increase from 0 to 128
                        let fraction: f32 = i_u16 as f32 / (wave_length as f32 / 2.0);
                        // i / (length / 2)
                        leds[(i + offset) % 791] = [(color_r * fraction) as u8, (color_g * fraction) as u8, (color_b * fraction) as u8, 0];
                    } else {
                        // Decrease from 128 to 0
                        let fraction: f32 = 1.0 - ((i_u16 - (wave_length / 2)) as f32 / (wave_length as f32 / 2.0));
                        // 1 - ((i - (length / 2)) / (length / 2))
                        leds[(i + offset) % 791] = [(color_r * fraction) as u8, (color_g * fraction) as u8, (color_b * fraction) as u8, 0];
                    }
                }
            },
            Mode::White => {
                for led in leds.iter_mut() {
                    *led = [255, 255, 255, 0];
                }
            },
            Mode::Red => {
                for led in leds.iter_mut() {
                    *led = [255, 0, 0, 0];
                }
            },
            Mode::Orange => {
                for led in leds.iter_mut() {
                    *led = [255, 30, 0, 0];
                }
            },
            Mode::Yellow => {
                for led in leds.iter_mut() {
                    *led = [255, 100, 0, 0];
                }
            },
            Mode::Green => {
                for led in leds.iter_mut() {
                    *led = [0, 255, 0, 0];
                }
            },
            Mode::Blue => {
                for led in leds.iter_mut() {
                    *led = [0, 0, 255, 0];
                }
            },
            Mode::Pink => {
                for led in leds.iter_mut() {
                    *led = [255, 10, 40, 0];
                }
            },
            Mode::Purple => {
                for led in leds.iter_mut() {
                    *led = [128, 0, 128, 0];
                }
            },
            Mode::Rainbow => {
                let temp_color = utils::color_wheel_768(counter);
                let color = (temp_color.0, (temp_color.1 / 2) as u8, (temp_color.2) as u8);
                for led in leds.iter_mut() {
                    *led = [color.0, color.1, color.2, 0];
                }
                counter += 2;
                if counter > 768 {
                    counter = 0;
                }
                // println!("{}: R({}) G({}) B({})", counter, color.0, color.1, color.2);
            },
        }

        // stage.set_full_stage([0, 0, 0]);
        // stage.get_full_stage();

        controller.render().expect("Failed to render LEDs");
    }
}
