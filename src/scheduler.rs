use rs_ws281x::Controller;

use crate::{stage::{Mode, Stage}, utils};

pub fn start_scheduler(controller: &mut Controller, stage: &mut Stage) {
    let mut rainbow_index: u16 = 0;
    loop {
        let leds = controller.leds_mut(0);

        match stage.get_mode() {
            Mode::OFF => {
                for led in leds.iter_mut() {
                    *led = [0, 0, 0, 0];
                }
            },
            Mode::WHITE => {
                for led in leds.iter_mut() {
                    *led = [255, 255, 255, 0];
                }
            },
            Mode::RED => {
                for led in leds.iter_mut() {
                    *led = [255, 0, 0, 0];
                }
            },
            Mode::ORANGE => {
                for led in leds.iter_mut() {
                    *led = [255, 165, 0, 0];
                }
            },
            Mode::YELLOW => {
                for led in leds.iter_mut() {
                    *led = [255, 255, 0, 0];
                }
            },
            Mode::GREEN => {
                for led in leds.iter_mut() {
                    *led = [0, 255, 0, 0];
                }
            },
            Mode::BLUE => {
                for led in leds.iter_mut() {
                    *led = [0, 0, 255, 0];
                }
            },
            Mode::PURPLE => {
                for led in leds.iter_mut() {
                    *led = [128, 0, 128, 0];
                }
            },
            Mode::RAINBOW => {
                let temp_color = utils::color_wheel_768(rainbow_index);
                let color = (temp_color.0, (temp_color.1 / 2) as u8, (temp_color.2) as u8);
                for led in leds.iter_mut() {
                    *led = [color.0, color.1, color.2, 0];
                }
                rainbow_index += 2;
                if rainbow_index > 768 {
                    rainbow_index = 0;
                }
                println!("{}: R({}) G({}) B({})", rainbow_index, color.0, color.1, color.2);
            },
        }

        // stage.set_full_stage([0, 0, 0]);
        stage.get_full_stage();

        controller.render().expect("Failed to render LEDs");
    }
}
