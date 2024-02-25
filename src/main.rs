mod behaviors;
mod fursona;
mod rendering;
mod settings;
mod stage;
mod time;

use std::cell::RefCell;

use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use fursona::FursonaInstance;
use settings::Settings;
use stage::{Stage, ViewportPoint};
use time::Time;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let settings = Settings::load_or_create("./settings.json");
    println!("Defined fursona:");
    for fursona in settings.fursona {
        println!("  - {}", fursona.name)
    }

    let settings = RefCell::new(Settings::load_or_create("./settings.json"));

    let mut stage = Stage::new(&event_loop).unwrap();

    let mut instances: Vec<FursonaInstance> = Vec::new();

    // Cheaply creates an empty DeviceState
    let device_state = DeviceState::checked_new().unwrap();
    let mut prev_mouse_state = MouseState::default();

    let mut time = Time::new();

    event_loop.run(move |event, elwt| match event {
        Event::NewEvents(StartCause::Poll) => {
            if device_state.get_keys().contains(&Keycode::Escape) {
                elwt.exit();
            }

            let mouse_state = device_state.get_mouse();
            if mouse_state != prev_mouse_state {
                stage.on_mouse_over(ViewportPoint {
                    x: mouse_state.coords.0 as f32,
                    y: mouse_state.coords.1 as f32,
                });

                let lmb = mouse_state.button_pressed.get(1).unwrap_or(&false);
                if *lmb {
                    println!("LMB click: {:?}", mouse_state.coords);
                }

                prev_mouse_state = mouse_state;
            }
        }
        Event::Resumed => {
            instances.extend(
                settings
                    .borrow()
                    .fursona
                    .iter()
                    .map(|fursona| fursona.make_instance(&stage)),
            );
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => {
                if size.width != 0 && size.height != 0 {
                    stage.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                let delta_t_ms = time.delta_ms();

                for instance in instances.iter_mut() {
                    instance.update(delta_t_ms, &stage);
                }

                stage.draw(instances.iter());
            }
            WindowEvent::CloseRequested => elwt.exit(),
            _ => (),
        },
        _ => (),
    })
}
