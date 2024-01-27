mod settings;
mod stage;

use settings::Settings;
use stage::Stage;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

fn main() -> Result<(), impl std::error::Error> {
    let settings = Settings::load_or_create("./settings.json");

    let event_loop = EventLoop::new().unwrap();

    let stage = Stage::new(&event_loop, &settings).unwrap();

    event_loop.run(move |event, elwt| {
        println!("{event:?}");

        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::RedrawRequested => stage.redraw(),
                _ => (),
            }
        }
    })
}
