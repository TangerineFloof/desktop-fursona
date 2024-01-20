mod stage_frame;
use crate::settings::Settings;
use stage_frame::StageFrame;

pub struct Stage {
    name: String,
}

impl Stage {
    pub fn new(settings: &Settings) -> Self {
        let name = &settings.name;
        let species = &settings.species;

        Self {
            name: format!("{name} the {species}"),
        }
    }

    pub fn show(&self) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_decorations(false)
                .with_inner_size([320.0, 240.0])
                .with_transparent(true),
            ..Default::default()
        };

        eframe::run_native(
            &self.name,
            options,
            Box::new(|_cc| Box::<StageFrame>::default()),
        )
    }
}
