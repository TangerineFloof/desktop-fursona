mod settings;
mod stage;
use stage::Stage;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let settings = settings::Settings::load_or_create("./settings.json");

    let stage = Stage::new(&settings);
    stage.show()
}
