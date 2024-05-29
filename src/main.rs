mod event_loop;
mod fursona;
mod rendering;
mod settings;
mod stage;
mod system_tray;

use colored::Colorize;
use std::cell::RefCell;

use event_loop::{Event, EventLoop};
use fursona::FursonaInstance;
use settings::Settings;
use stage::Stage;

use crate::system_tray::SystemTray;

fn main() -> Result<(), impl std::error::Error> {
    // crate::system_tray::tray_main();

    let event_loop = EventLoop::new(SystemTray::new()).unwrap();

    let settings = Settings::load_or_create("./settings.json");
    if settings.fursona.is_empty() {
        println!(
            "{}",
            "You have no fursona configured in your settings.".red()
        );
        std::process::exit(-1);
    }

    println!("Defined fursona:");
    for fursona in settings.fursona {
        println!("  - {}", fursona.name)
    }

    let settings = RefCell::new(Settings::load_or_create("./settings.json"));

    let mut stage = Stage::new(&event_loop).unwrap();
    stage.set_debug_mode(true);

    let mut instances: Vec<FursonaInstance> = Vec::new();

    event_loop.run(move |event| match event {
        Event::Initialization => {
            instances.extend(
                settings
                    .borrow()
                    .fursona
                    .iter()
                    .map(|fursona| fursona.make_instance(&stage)),
            );
        }
        Event::MouseDown(coords) => {
            println!("LMB click: ({}, {})", coords.x, coords.y);
        }
        Event::MouseMove(coords) => {
            stage.on_mouse_over(coords);
        }
        Event::Exit => {
            println!("EXITING");
        }
        Event::Resized { width, height } => stage.resize(width, height),
        Event::Update { delta_t_ms } => {
            for instance in instances.iter_mut() {
                instance.update(delta_t_ms, &stage);
            }

            stage.draw(instances.iter());
        }
    })
}
