mod fursona;
mod rendering;
mod settings;
mod stage;

use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use rendering::{Renderer, Renderer2D};
use settings::Settings;
use stage::Stage;
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

    let mut stage = Stage::new(&event_loop).unwrap();

    let mut renderers: Vec<Box<dyn Renderer>> = Vec::new();

    // Cheaply creates an empty DeviceState
    let device_state = DeviceState::checked_new().unwrap();
    let mut prev_mouse_state = MouseState::default();

    event_loop.run(move |event, elwt| match event {
        Event::NewEvents(StartCause::Poll) => {
            if device_state.get_keys().contains(&Keycode::Escape) {
                elwt.exit();
            }

            let mouse_state = device_state.get_mouse();
            if mouse_state != prev_mouse_state {
                stage.on_mouse_over(mouse_state.coords.0 as u32, mouse_state.coords.1 as u32);

                let lmb = mouse_state.button_pressed.get(1).unwrap_or(&false);
                if *lmb {
                    println!("LMB click: {:?}", mouse_state.coords);
                }

                prev_mouse_state = mouse_state;
            }
        }
        Event::Resumed => {
            let jack = Box::new(Renderer2D::new(&stage, "./jack_by_nal_cinnamonspots.png"));
            renderers.push(jack);
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => {
                if size.width != 0 && size.height != 0 {
                    stage.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                stage.draw(&renderers);
            }
            WindowEvent::CloseRequested => elwt.exit(),
            _ => (),
        },
        _ => (),
    })
}
