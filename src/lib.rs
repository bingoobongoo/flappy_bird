use crate::entity::Entity;
use crate::resource_manager::ResManager;
use rand::Rng;
use sdl2::event::*;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::*;
use sdl2::sys::SDL_Quit;
use sdl2::video::*;
use sdl2::EventPump;
use std::collections::VecDeque;

pub mod entity;
pub mod resource_manager;

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;

pub struct App {
    pub display: Canvas<Window>,
    event_pump: EventPump,
    player: Option<Entity>,
    columns: VecDeque<Entity>,
    background_rect: Rect,
}

impl App {
    pub fn init() -> Self {
        let context = sdl2::init().expect("Failed to initialize SDL2");
        let window = context
            .video()
            .unwrap()
            .window("Flappy Bird", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let display = window
            .into_canvas()
            .build()
            .expect("Failed to initialize display Canvas");
        let background_rect = Rect::from_center(
            Point::new((SCREEN_WIDTH / 2) as i32, (SCREEN_HEIGHT / 2) as i32),
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        );

        let event_pump = context
            .event_pump()
            .expect("Failed to initialize EventPump");

        let player = Some(Entity::new(200.0, 10.0, 50, 50, 0.0, 0.0, true));

        let columns = Self::create_columns();

        Self {
            display,
            event_pump,
            player,
            columns,
            background_rect,
        }
    }

    pub fn process_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Escape => {
                        unsafe { SDL_Quit() };
                        std::process::exit(0);
                    }
                    Keycode::Space if self.player.is_some() => self.player.as_mut().unwrap().jump(),
                    Keycode::Space if self.player.is_none() => {
                        let player = Some(Entity::new(200.0, 10.0, 50, 50, 0.0, 0.0, true));
                        self.player = player;

                        let columns = Self::create_columns();
                        self.columns = columns;
                    }
                    _ => (),
                },

                _ => (),
            }
        }
    }

    pub fn process_logic(&mut self, delta_time: f64) {
        if let Some(ent) = &mut self.player {
            ent.update_position(delta_time);
        }

        let mut updated_columns: VecDeque<Entity> = VecDeque::new();
        for column in self.columns.iter_mut() {
            column.update_position(delta_time);
            if self.player.is_none() {
                column.stop();
            }
            if column.x > -(column.rect.width() as f64) {
                updated_columns.push_back(*column);
            }
        }

        if let Some(ent) = &mut self.player {
            for i in 0..5 {
                let column_ref = updated_columns.get(i as usize).unwrap();
                if ent.rect.has_intersection(column_ref.rect) {
                    self.player = None;
                    break;
                }
            }
        }

        if let Some(ent) = &mut self.player {
            if ent.rect.top() > SCREEN_HEIGHT as i32 {
                self.player = None;
            }
        }

        let boundary = SCREEN_WIDTH + updated_columns.back().unwrap().rect.width() / 2;
        if updated_columns.back().unwrap().x < boundary as f64 {
            updated_columns.append(&mut Self::create_columns())
        }
        self.columns = updated_columns;
    }

    pub fn render_display(&mut self, res_manager: &ResManager) {
        self.display.set_draw_color(Color::RGB(173, 216, 230));
        self.display.clear();

        let texture = res_manager.get_texture("background");
        self.display
            .copy_ex(texture, None, self.background_rect, 0.0, None, false, false)
            .unwrap();

        self.draw_entities(res_manager);

        self.display.present();
    }

    fn draw_entities(&mut self, res_manager: &ResManager) {
        if let Some(player) = &mut self.player {
            let player_texture = res_manager.get_texture("flappy_bird");
            let sprite_rect = Rect::from_center(
                player.rect.center(),
                player.rect.width() + 40,
                player.rect.height() + 30,
            );
            self.display
                .copy_ex(player_texture, None, sprite_rect, 0.0, None, false, false)
                .unwrap();
        }

        for column in self.columns.iter_mut() {
            self.display.set_draw_color(Color::RGB(0, 255, 100));
            self.display.fill_rect(column.rect).unwrap();
        }
    }

    fn create_columns() -> VecDeque<Entity> {
        let mut deque: VecDeque<Entity> = VecDeque::new();
        let mut rng = rand::thread_rng();
        let gap_size: u32 = 200;
        let col_spacing: u32 = 300;
        let col_width: u32 = 100;
        let min_col_height: u32 = 100;
        let max_col_height: u32 = SCREEN_HEIGHT - min_col_height - gap_size;
        let spawn_pos_x = (SCREEN_WIDTH + col_spacing + col_width / 2) as f64;

        for i in 0..6 {
            let col_height_upper = rng.gen_range(min_col_height..max_col_height);
            let col_height_lower = SCREEN_HEIGHT - col_height_upper - gap_size;

            let y_upper = col_height_upper / 2;
            let y_lower = SCREEN_HEIGHT - col_height_lower / 2;

            let upper = Entity::new(
                spawn_pos_x + i as f64 * col_spacing as f64,
                y_upper as f64,
                col_width,
                col_height_upper,
                -1.7,
                0.0,
                false,
            );
            let lower = Entity::new(
                spawn_pos_x + i as f64 * col_spacing as f64,
                y_lower as f64,
                col_width,
                col_height_lower,
                -1.7,
                0.0,
                false,
            );

            deque.push_back(lower);
            deque.push_back(upper);
        }

        deque
    }
}
