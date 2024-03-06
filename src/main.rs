mod fursona;
mod rendering;
mod settings;
mod stage;
mod time;

use colored::Colorize;
use std::cell::RefCell;
use winit::error::EventLoopError;
#[cfg(target_os = "macos")]
use winit::platform::macos::{ActivationPolicy, EventLoopBuilderExtMacOS};

use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use fursona::FursonaInstance;
use settings::Settings;
use stage::{Stage, ViewportPoint};
use time::Time;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};

fn create_event_loop() -> Result<EventLoop<()>, EventLoopError> {
    let mut builder = EventLoopBuilder::new();

    #[cfg(target_os = "macos")]
    builder.with_activation_policy(ActivationPolicy::Accessory);

    let event_loop = builder.build()?;

    event_loop.set_control_flow(ControlFlow::Poll);

    Ok(event_loop)
}

fn main() -> Result<(), impl std::error::Error> {
    let event_loop = create_event_loop().unwrap();

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
