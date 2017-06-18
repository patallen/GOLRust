use std::process;

use sdl2;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


use engine::scene::{SceneStack, BoxedScene, SceneEvent};
use scenes::{StartScene, PauseScene};

enum EngineState {
    Paused,
    Running
}

pub struct EngineMeta {
    title: &'static str,
    width: u32,
    height: u32,
    scale: u32,
}
impl EngineMeta {
    pub fn new(title: &'static str, width: u32, height: u32, scale: u32) -> Self {
        EngineMeta {
            title: title,
            width: width,
            height: height,
            scale: scale,
        }
    }
}
pub struct Engine {
    scene_stack: SceneStack,
    renderer: WindowCanvas,
    event_pump: EventPump,
    state: EngineState,
}

impl Engine {
    pub fn new(context: sdl2::Sdl, engine_meta: EngineMeta) -> Self {
        let EngineMeta{title, width, height, scale, ..} = engine_meta;
        let video_subsytem = context.video().expect("Couldn't initialize Video Subsystem");
        let event_pump = context.event_pump().expect("Could't initialize Event Pump");
        let window = video_subsytem.window(title, width * scale, height * scale)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().accelerated().build().unwrap();
        Engine {
            scene_stack: SceneStack::new(),
            renderer: canvas,
            event_pump: event_pump,
            state: EngineState::Running
        }
    }
    pub fn update(&mut self) {
        if let Some(state) = self.scene_stack.top_mut() { state.update(); };
    }
    pub fn render(&mut self) {
        match self.scene_stack.top() {
            Some(state) => state.render(&mut self.renderer),
            None => panic!("No more states!")
        }
    }
    pub fn add_scene(&mut self, scene: BoxedScene) {
        self.scene_stack.add_scene(scene);
    }
    pub fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            if let Some(scene) = self.scene_stack.top().unwrap().think(event) {
                match scene {
                    SceneEvent::Pop => self.scene_stack.drop(),
                    SceneEvent::Push(new) => self.scene_stack.push(Box::new(*new)),
                    _ => {}
                }
                ()
            }
            match event {
                Event::Quit{..} => process::exit(1),
                Event::KeyDown{keycode, ..} => {
                    match keycode {
                        Some(Keycode::Q) => process::exit(1),
                        Some(Keycode::P) => {
                            match self.state {
                                EngineState::Paused => {
                                    self.scene_stack.drop();
                                    self.state = EngineState::Running;
                                },
                                _ => {
                                    self.scene_stack.push(Box::new(PauseScene{}));
                                    self.state = EngineState::Paused;
                                }
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }
}
