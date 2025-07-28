use winit::error::EventLoopError;
use crate::engine::window::WindowManager;

mod window;
mod state;

pub struct Engine {
    window_title: String,
}

impl Engine {
    pub fn new(window_title: String) -> Self {
        Engine { window_title } // todo: add start level
    }

    pub fn run(&self) -> Result<(), EventLoopError> {
        let event_loop = winit::event_loop::EventLoop::new()?;
        let mut window = WindowManager::new(self.window_title.clone());
        event_loop.run_app(&mut window)
    }

}