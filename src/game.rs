use std::time::{Instant, Duration};

use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;

use board::Board;
use engine::scene::Scene;
use engine::scene::SceneEvent;


const DEAD_COLOR: (u8, u8, u8) = (0, 28, 67);
const ALIVE_COLOR: (u8, u8, u8) = (255, 153, 0);
const BG_COLOR: (u8, u8, u8) = (0, 0, 150);


#[derive(Debug)]
enum GameMode {
    Playing,
    Editing,
}

pub struct GameScene {
    cell_size: usize,
    board: Board,
    round: usize,
    mode: GameMode,
    last_update: Option<Instant>,
    update_interval: Duration,
}
impl GameScene {
    pub fn new(width: usize, height: usize, scale: usize, speed: usize) -> GameScene {
        GameScene {
            cell_size: scale,
            board: Board::new(width, height),
            round: 0,
            mode: GameMode::Editing,
            last_update: None,
            update_interval: Duration::from_millis((1.0 / speed as f64 * 1000.0) as u64)
        }
    }
    fn can_update(&mut self) -> bool {
        let now = Instant::now();
        match self.last_update {
            Some(instant) => { now.duration_since(instant) > self.update_interval},
            None => { self.last_update = Some(Instant::now()); true }
        }
    }
    fn restart(&mut self) {
        self.board.clear();
        self.round = 0;
        self.mode = GameMode::Editing;
    }
}


impl Scene for GameScene {
    fn render(&self, renderer: &mut WindowCanvas) {
        renderer.set_draw_color(Color::RGB(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2));
        renderer.clear();
        let cells = self.board.clone_cells();
        for (y, row) in cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell.is_alive() {
                    renderer.set_draw_color(Color::RGB(ALIVE_COLOR.0, ALIVE_COLOR.1, ALIVE_COLOR.2));
                    renderer.fill_rect(
                        Rect::new(x as i32 * self.cell_size as i32,
                                y as i32 * self.cell_size as i32,
                                self.cell_size as u32 - 1, self.cell_size as u32 - 1)).unwrap();
                } else {
                    renderer.set_draw_color(Color::RGB(DEAD_COLOR.0, DEAD_COLOR.1, DEAD_COLOR.2));
                    renderer.fill_rect(
                        Rect::new(x as i32 * self.cell_size as i32,
                                y as i32 * self.cell_size as i32,
                                self.cell_size as u32 - 1, self.cell_size as u32 - 1)).unwrap();
                }
            }
        }
        renderer.present();
    }
    fn think(&self, event: Event) -> Option<SceneEvent> {
        None
    }
    fn update(&mut self){
        match self.mode {
            GameMode::Playing => {
                if self.can_update() {
                    self.last_update = Some(Instant::now());
                    self.board.update();
                }
            },
            _ => {}
        }
    }
    fn handle_events(&mut self, events: Vec<Event>) {
        for event in events.iter() {
            match event {
                &Event::KeyDown{keycode: kc, ..} => match kc {
                    Some(Keycode::R) => self.restart(),
                    Some(Keycode::D) => {
                        match self.mode {
                            GameMode::Editing => self.mode = GameMode::Playing,
                            GameMode::Playing => self.mode = GameMode::Editing,
                        }
                    },
                    _ => {}
                },
                &Event::MouseButtonDown{mouse_btn: button, x, y, ..} => match &self.mode {
                    &GameMode::Editing => {
                        let xpos = x as usize / self.cell_size;
                        let ypos = y as usize / self.cell_size;
                        match button {
                            MouseButton::Left => { self.board.set_cell_state(xpos, ypos); },
                            MouseButton::Right => { self.board.unset_cell_state(xpos, ypos); },
                            _ => {}
                        }
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
