use std::path::Path;

use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf;


use engine::scene::Scene;

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
        let fontctx = ttf::init().unwrap();
        let h1 = fontctx.load_font(Path::new("assets/lobster.ttf"), 300).expect("woops");
        let h2 = fontctx.load_font(Path::new("assets/lobster.ttf"), 200).expect("woops");
        let main_text = Text::new("Paused", 400, 180);
        let sub_text = Text::new("Press \"P\" to resume", 400, 90);

        let paused = h1.render(&main_text.contents).blended(Color::RGB(0, 0, 0)).unwrap();
        let subtext = h2.render(&sub_text.contents).blended(Color::RGB(0, 0, 0)).unwrap();

        let mut creator = renderer.texture_creator();
        let pausedt = creator.create_texture_from_surface(&paused).unwrap();
        let subt = creator.create_texture_from_surface(&subtext).unwrap();
        let (width, height) = renderer.window().size();
        renderer.set_draw_color(Color::RGB(160, 160, 160));
        renderer.fill_rect(Rect::new(0, 0, width, height)).unwrap();
        let top = Point::new(width as i32 / 2, height as i32 / 2).offset(0, -50);
        let bottom = Point::new(width  as i32 / 2, height as i32 / 2).offset(0, 50);
        renderer.copy(&pausedt, None, Some(main_text.rect_centered(&top)));
        renderer.copy(&subt, None, Some(sub_text.rect_centered(&bottom)));
        renderer.present();
    }
    fn update(&mut self) {}
    fn handle_events(&mut self, _: Vec<Event>) {}
}

//impl Scene {
//    pub fn think(&self, event: Event) {
//        match event {
//            Pause => SceneEvent::Pop,
//            Restart =>
//        }
//    }
//}
