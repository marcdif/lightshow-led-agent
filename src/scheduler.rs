use rs_ws281x::Controller;

pub fn start_scheduler(controller: &mut Controller) {
    let leds = controller.leds_mut(0);
    for led in leds.iter_mut() {
        *led = [0, 0, 0, 0];
    }
    controller.render().expect("Failed to render LEDs");
}
