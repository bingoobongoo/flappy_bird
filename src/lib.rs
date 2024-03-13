use crate::entity::Entity;
use rand::Rng;
use sdl2::event::*;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::*;
use sdl2::sys::SDL_Quit;
use sdl2::video::*;
use sdl2::EventPump;
use std::collections::VecDeque;

mod entity;

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;

pub struct App {
    display: Canvas<Window>,
    event_pump: EventPump,
    player: Option<Entity>,
    columns: VecDeque<Entity>,
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

        let texture_creator = display.texture_creator();
        let background = texture_creator
            .load_texture("resources/background.png")
            .unwrap();

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
        // update player positon
        if let Some(ent) = &mut self.player {
            ent.update_position(delta_time);
        }

        // update columns position
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

        // check for player colison with columns
        if let Some(ent) = &mut self.player {
            for i in 0..5 {
                let column_ref = updated_columns.get(i as usize).unwrap();
                if ent.rect.has_intersection(column_ref.rect) {
                    self.player = None;
                    break;
                }
            }
        }

        // check if player has fallen down
        if let Some(ent) = &mut self.player {
            if ent.rect.top() > SCREEN_HEIGHT as i32 {
                self.player = None;
            }
        }

        // generate columns if needed
        let boundary = SCREEN_WIDTH + updated_columns.back().unwrap().rect.width() / 2;
        if updated_columns.back().unwrap().x < boundary as f64 {
            updated_columns.append(&mut Self::create_columns())
        }
        self.columns = updated_columns;
    }

    pub fn render_display(&mut self) {
        self.display.set_draw_color(Color::RGB(255, 255, 255));
        self.display.clear();

        self.draw_entities();

        self.display.present();
    }

    fn draw_entities(&mut self) {
        if let Some(player) = &mut self.player {
            self.display.set_draw_color(Color::RGB(255, 0, 0));
            self.display.fill_rect(player.rect).unwrap();
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
