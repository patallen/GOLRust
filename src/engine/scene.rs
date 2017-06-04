use sdl2::render::WindowCanvas;
use sdl2::event::Event;

pub type BoxedScene = Box<Scene + 'static>;

pub trait Scene {
    fn render(&self, &mut WindowCanvas) -> ();
    fn update(&mut self) -> ();
    fn handle_events(&mut self, Vec<Event>) -> ();
}


pub struct SceneStack {
    states: Vec<BoxedScene>
}
impl SceneStack {
    pub fn new() -> Self {
        SceneStack{
            states: Vec::new()
        }
    }
    pub fn push(&mut self, state: BoxedScene) {
        self.states.push(state);
    }
    pub fn top(&mut self) -> Option<&mut Box<Scene>> {
        self.states.last_mut()
    }
    pub fn add_scene(&mut self, scene: BoxedScene) {
        self.states.insert(0, scene);
    }
}
