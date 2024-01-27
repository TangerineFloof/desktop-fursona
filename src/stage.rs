use crate::settings::Settings;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct Stage {
    _window: Window,
}

impl Stage {
    pub fn new(event_loop: &EventLoop<()>, settings: &Settings) -> Result<Self, &'static str> {
        let name = &settings.name;
        let species = &settings.species;

        match WindowBuilder::new()
            .with_decorations(false)
            .with_title(format!("{name} the {species}"))
            .build(&event_loop)
        {
            Ok(window) => Ok(Self { _window: window }),
            Err(_) => Err("Failed to create window"),
        }
    }

    pub fn redraw(&self) -> () {}
}
