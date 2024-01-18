use eframe::egui;
use tray_item::{IconSource, TrayItem};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let mut tray = TrayItem::new("Tray Example", IconSource::Resource("")).unwrap();

    tray.add_label("Tray Label").unwrap();

    tray.add_menu_item("Hello", || {
        println!("Hello!");
    })
    .unwrap();

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([320.0, 240.0])
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        custom_window_frame(ctx, |ui| {
            ui.label("This is just the contents of the window.");
            ui.horizontal(|ui| {
                ui.label("egui theme:");
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }
}

fn custom_window_frame(ctx: &egui::Context, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::*;

    let panel_frame = egui::Frame {
        fill: egui::Color32::TRANSPARENT, // ctx.style().visuals.window_fill(),
        // rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        // Add the contents:
        let content_rect = ui.max_rect().shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}
