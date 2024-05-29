mod time;

use crate::stage::ViewportPoint;
use crate::system_tray::{SystemTray, SystemTrayEvent};
use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use time::Time;
use winit::error::EventLoopError;
use winit::event::{Event as WinitEvent, StartCause, WindowEvent as WinitWindowEvent};
use winit::event_loop::{
    ControlFlow, EventLoop as WinitEventLoop, EventLoopBuilder as WinitEventLoopBuilder,
    EventLoopWindowTarget as WinitEventLoopWindowTarget,
};
#[cfg(target_os = "macos")]
use winit::platform::macos::{ActivationPolicy, EventLoopBuilderExtMacOS};

pub enum Event {
    Initialization,
    MouseMove(ViewportPoint),
    MouseDown(ViewportPoint),
    Update { delta_t_ms: u32 },
    Resized { width: u32, height: u32 },
    Exit,
}

pub struct EventLoop {
    system_tray: SystemTray,
    winit_event_loop: WinitEventLoop<()>,
}

impl EventLoop {
    pub fn new(system_tray: SystemTray) -> Result<Self, EventLoopError> {
        let mut builder = WinitEventLoopBuilder::new();

        #[cfg(target_os = "macos")]
        builder.with_activation_policy(ActivationPolicy::Accessory);

        let winit_event_loop = builder.build()?;

        winit_event_loop.set_control_flow(ControlFlow::Poll);

        Ok(Self {
            system_tray,
            winit_event_loop,
        })
    }

    pub fn get_winit(&self) -> &WinitEventLoopWindowTarget<()> {
        &self.winit_event_loop
    }

    pub fn run<F>(self, mut event_handler: F) -> Result<(), EventLoopError>
    where
        F: FnMut(Event),
    {
        // Cheaply creates an empty DeviceState
        let device_state = DeviceState::checked_new().unwrap();
        let mut prev_mouse_state = MouseState::default();

        let mut time = Time::new();

        let mut has_exited = false;
        let mut do_exit = |event_handler: &mut F, elwt: &WinitEventLoopWindowTarget<()>| {
            // Ensure we only fire the exit event once for cleanup, regardless of how many
            // different events we encounter in the event loop.
            if !has_exited {
                event_handler(Event::Exit);
                has_exited = true;
            }

            elwt.exit();
        };

        self.system_tray.on(|event| match event {
            SystemTrayEvent::Quit => do_exit(&mut event_handler, &self.winit_event_loop),
        });

        self.winit_event_loop.run(move |event, elwt| match event {
            WinitEvent::NewEvents(StartCause::Poll) => {
                if device_state.get_keys().contains(&Keycode::Escape) {
                    do_exit(&mut event_handler, elwt);
                }

                let mouse_state = device_state.get_mouse();
                if mouse_state != prev_mouse_state {
                    event_handler(Event::MouseMove(ViewportPoint {
                        x: mouse_state.coords.0 as f32,
                        y: mouse_state.coords.1 as f32,
                    }));

                    let lmb = mouse_state.button_pressed.get(1).unwrap_or(&false);
                    if *lmb {
                        event_handler(Event::MouseDown(ViewportPoint {
                            x: mouse_state.coords.0 as f32,
                            y: mouse_state.coords.1 as f32,
                        }));
                    }

                    prev_mouse_state = mouse_state;
                }
            }
            WinitEvent::Resumed => {
                event_handler(Event::Initialization);
            }
            WinitEvent::WindowEvent { event, .. } => match event {
                WinitWindowEvent::Resized(size) => {
                    if size.width != 0 && size.height != 0 {
                        event_handler(Event::Resized {
                            width: size.width,
                            height: size.height,
                        });
                    }
                }
                WinitWindowEvent::RedrawRequested => {
                    event_handler(Event::Update {
                        delta_t_ms: time.delta_ms(),
                    });
                }
                WinitWindowEvent::CloseRequested => do_exit(&mut event_handler, elwt),
                _ => (),
            },
            _ => (),
        })
    }
}
