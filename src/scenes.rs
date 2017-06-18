use std::path::Path;

use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf;


use engine::scene::{Scene, SceneEvent};

// pub enum Event {
//     Pause,
//     Quit,
//     Restart,
// }
struct Text {
    contents: String,
    height: usize,
    width: usize,
}
impl Text {
    pub fn new(text: &str, width:usize, height: usize) -> Text {
        Text {
            contents: text.to_string(),
            height: height,
            width: width,
        }
    }
    pub fn rect_centered(&self, point: &Point)-> Rect {
        let mut rect = Rect::new(0, 0, self.width as u32, self.height as u32);
        rect.center_on((point.x(), point.y()));
        rect
    }
}

pub struct PauseScene {}

impl Scene for PauseScene {
    fn render(&self, renderer: &mut Canvas<Window>) {
        let main_text = Text::new("Paused", 400, 180);
        let sub_text = Text::new("Press \"P\" to resume", 400, 90);

        let fontctx = ttf::init().unwrap();
        let paused = fontctx.load_font(Path::new("assets/lobster.ttf"), 300)
            .expect("woops")
            .render(&main_text.contents)
            .blended(Color::RGB(0, 0, 0)).unwrap();
        let subtext = fontctx.load_font(Path::new("assets/lobster.ttf"), 200)
            .expect("woops")
            .render(&sub_text.contents)
            .blended(Color::RGB(0, 0, 0)).unwrap();

        let creator = renderer.texture_creator();
        let pausedt = creator.create_texture_from_surface(&paused).unwrap();
        let subt = creator.create_texture_from_surface(&subtext).unwrap();
        let (width, height) = renderer.window().size();
        let top = Point::new(width as i32 / 2, height as i32 / 2).offset(0, -50);
        let bottom = Point::new(width  as i32 / 2, height as i32 / 2).offset(0, 50);

        renderer.set_draw_color(Color::RGB(160, 160, 160));
        renderer.fill_rect(Rect::new(0, 0, width, height)).unwrap();
        renderer.copy(&pausedt, None, Some(main_text.rect_centered(&top)));
        renderer.copy(&subt, None, Some(sub_text.rect_centered(&bottom)));
        renderer.present();
    }
    fn update(&mut self) {}
    fn handle_events(&mut self, _: Vec<Event>) {}
    fn think(&self, event: Event) -> Option<SceneEvent> {
        match event {
            Pause => Some(SceneEvent::Pop),
            _ => None
        }
    }
}

pub struct StartScene {}

impl Scene for StartScene {
    fn render(&self, renderer: &mut Canvas<Window>) {
        let main_text = Text::new("Game of Life", 500, 110);
        let sub1 = Text::new("Q to Quit", 250, 70);
        let sub2 = Text::new("P to Toggle Pause", 500, 70);
        let sub3 = Text::new("D to Toggle Drawing", 600, 70);

        let fontctx = ttf::init().unwrap();
        let paused = fontctx.load_font(Path::new("assets/lobster.ttf"), 300)
            .expect("woops")
            .render(&main_text.contents)
            .blended(Color::RGB(0, 0, 0)).unwrap();
        let st1 = fontctx.load_font(Path::new("assets/lobster.ttf"), 160)
            .expect("woops")
            .render(&sub1.contents)
            .blended(Color::RGB(0, 0, 0)).unwrap();
        let st2 = fontctx.load_font(Path::new("assets/lobster.ttf"), 160)
            .expect("woops")
            .render(&sub2.contents)
            .blended(Color::RGB(0, 0, 0)).unwrap();
        let st3 = fontctx.load_font(Path::new("assets/lobster.ttf"), 160)
            .expect("woops")
            .render(&sub3.contents)
            .blended(Color::RGB(0, 0, 0)).unwrap();

        let creator = renderer.texture_creator();
        let pausedt = creator.create_texture_from_surface(&paused).unwrap();
        let ss1 = creator.create_texture_from_surface(&st1).unwrap();
        let ss2 = creator.create_texture_from_surface(&st2).unwrap();
        let ss3 = creator.create_texture_from_surface(&st3).unwrap();
        let (width, height) = renderer.window().size();
        let main = Point::new(width as i32 / 2, 200);
        let s1 = Point::new(width as i32 / 2, 320);
        let s2 = Point::new(width as i32 / 2, 390);
        let s3 = Point::new(width as i32 / 2, 470);

        renderer.set_draw_color(Color::RGB(160, 160, 160));
        renderer.fill_rect(Rect::new(0, 0, width, height)).unwrap();
        renderer.copy(&pausedt, None, Some(main_text.rect_centered(&main)));
        renderer.copy(&ss1, None, Some(sub1.rect_centered(&s1)));
        renderer.copy(&ss2, None, Some(sub2.rect_centered(&s2)));
        renderer.copy(&ss3, None, Some(sub3.rect_centered(&s3)));
        renderer.present();
    }
    fn update(&mut self) {}
    fn handle_events(&mut self, _: Vec<Event>) {}
    fn think(&self, event: Event) -> Option<SceneEvent> {
        match event {
            Event::KeyDown{keycode: Some(Keycode::Return), ..} => {
                Some(SceneEvent::Pop)
            },
            _ => None
        }
    }
}
