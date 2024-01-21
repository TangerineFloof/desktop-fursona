mod settings;
mod stage;
use gtk::prelude::*;
use gtk::{glib, Application};
use stage::Stage;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.tangerinefloof.DesktopFursona")
        .build();
    app.connect_startup(handle_startup);
    app.connect_activate(handle_activate);
    app.run()
}

fn handle_startup(_: &Application) {
    Stage::init_application();
}

fn handle_activate(app: &Application) {
    let settings = settings::Settings::load_or_create("./settings.json");
    Stage::new(app, &settings).show()
}
