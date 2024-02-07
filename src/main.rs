mod rendering;
mod settings;
mod stage;

use settings::Settings;
use stage::StageBuilder;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{Key, NamedKey};

fn main() -> Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new().unwrap();
    let mut stage_builder = StageBuilder::new(&event_loop);

    let settings = Settings::load_or_create("./settings.json");
    let mut stage = stage_builder.build(&settings).unwrap();

    event_loop.run(move |event, elwt| {
        println!("{event:?}");
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    if size.width != 0 && size.height != 0 {
                        stage.resize(size.width, size.height);
                    }
                }
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: Key::Named(NamedKey::Escape),
                            ..
                        },
                    ..
                } => elwt.exit(),
                _ => (),
            },
            Event::AboutToWait => {
                stage.draw();
            }
            _ => (),
        }
    })
}
