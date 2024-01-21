use crate::settings::Settings;
use gtk::prelude::*;
use gtk::{gdk::Display, Application, ApplicationWindow, CssProvider};

pub struct Stage {
    window: ApplicationWindow,
}

impl Stage {
    pub fn init_application() -> () {
        let provider = CssProvider::new();
        provider.load_from_string("* { background: transparent; border: 2px solid green; }");

        gtk::style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    pub fn new(app: &Application, settings: &Settings) -> Self {
        let name = &settings.name;
        let species = &settings.species;

        Self {
            window: ApplicationWindow::builder()
                .application(app)
                .title(format!("{name} the {species}"))
                .decorated(false)
                .build(),
        }
    }

    pub fn show(&self) -> () {
        self.window.present();
    }
}
