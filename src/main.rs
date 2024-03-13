extern crate sdl2_test;
use sdl2_test::*;
use std::time::*;

fn main() {
    let mut app = App::init();
    let mut delta_time = 0.0;
    loop {
        let time_start = Instant::now();
        app.process_input();
        app.process_logic(delta_time);
        app.render_display();
        delta_time = time_start.elapsed().as_secs_f64();
    }
}
