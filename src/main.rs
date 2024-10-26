extern crate sdl2_test;
use ::sdl2_test::resource_manager::ResManager;
use sdl2_test::*;
use std::time::*;

fn main() {
    let mut app = App::init();
    let texture_creator = app.display.texture_creator();
    let mut res_manager = ResManager::new();
    res_manager.load_textures(&texture_creator);
    let mut delta_time = 0.0;
    loop {
        let time_start = Instant::now();
        app.process_input();
        app.process_logic(delta_time);
        app.render_display(&res_manager);
        delta_time = time_start.elapsed().as_secs_f64();
    }
}
