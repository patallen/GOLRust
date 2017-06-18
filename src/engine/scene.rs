use sdl2::render::WindowCanvas;
use sdl2::event::Event;


pub enum SceneEvent{
    Pop,
    Push(BoxedScene)
}


pub type BoxedScene = Box<Scene + 'static>;

pub trait Scene {
    fn render(&self, &mut WindowCanvas) -> ();
    fn update(&mut self) -> ();
    fn handle_events(&mut self, Vec<Event>) -> ();
    fn think(&self, Event) -> Option<SceneEvent>;
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
    pub fn drop(&mut self) {
        self.states.pop();
    }
    pub fn top(&self) -> Option<&Box<Scene>> {
        self.states.last()
    }
    pub fn top_mut(&mut self) -> Option<&mut Box<Scene>>  {
        self.states.last_mut()
    }
    pub fn add_scene(&mut self, scene: BoxedScene) {
        self.states.insert(0, scene);
    }
}
