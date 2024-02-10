use super::Stage;
use crate::settings::Settings;
use glutin::config::{ConfigTemplateBuilder, GlConfig};
use glutin_winit::{self, DisplayBuilder};
use winit::{
    platform::macos::WindowExtMacOS,
    window::{WindowBuilder, WindowLevel},
};

pub struct StageBuilder<'a> {
    event_loop: &'a winit::event_loop::EventLoop<()>,
    gl_config: Option<glutin::config::Config>,
}

impl<'a> StageBuilder<'a> {
    pub fn new(event_loop: &'a winit::event_loop::EventLoop<()>) -> Self {
        Self {
            event_loop,
            gl_config: None,
        }
    }

    pub fn build(&mut self, settings: &Settings) -> Result<Stage, &str> {
        let name = &settings.name;
        let species = &settings.species;

        // Only Windows requires the window to be present before creating the display.
        // Other platforms don't really need one.
        //
        // XXX if you don't care about running on Android or so you can safely remove
        // this condition and always pass the window builder.
        let window_builder = WindowBuilder::new()
            .with_transparent(true)
            .with_decorations(false)
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_title(format!("{name} the {species}"));

        let window = match self.gl_config {
            Some(_) => window_builder.build(self.event_loop).unwrap(),
            None => self.init_opengl(window_builder).unwrap(),
        };

        window.set_simple_fullscreen(true);

        Stage::new(window, self.gl_config.as_ref().unwrap())
    }

    fn init_opengl(&mut self, window_builder: WindowBuilder) -> Option<winit::window::Window> {
        // The template will match only the configurations supporting rendering
        // to windows.
        //
        // XXX We force transparency only on macOS, given that EGL on X11 doesn't
        // have it, but we still want to show window. The macOS situation is like
        // that, because we can query only one config at a time on it, but all
        // normal platforms will return multiple configs, so we can find the config
        // with transparency ourselves inside the `reduce`.
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(true);

        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (window, gl_config) = display_builder
            .build(self.event_loop, template, |configs| {
                // Find the config with the maximum number of samples, so our triangle will
                // be smooth.
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        println!("Picked a config with {} samples", gl_config.num_samples());

        if let Some(trans) = gl_config.supports_transparency() {
            println!("picked transparent? {}", trans);
        } else {
            println!("oh no...");
        }

        self.gl_config = Some(gl_config);

        window
    }
}
