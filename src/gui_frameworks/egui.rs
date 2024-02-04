#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::time;

use eframe::egui;

pub fn run() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Tomatentimer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<Tomatentimer>::default()
        }),
    )
}

struct Tomatentimer {
    name: String,
    age: u32,
    duration: time::Duration,
    last_tick: time::Instant,
    toggle_timer: bool,
}

impl Default for Tomatentimer {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            duration: time::Duration::default(),
            last_tick: time::Instant::now(),
            toggle_timer: false,
        }
    }
}

impl eframe::App for Tomatentimer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            if ui.button(format!("{}", if !self.toggle_timer { "Start" } else { "Stop" })).clicked() {
                self.toggle_timer = !self.toggle_timer;
            }

            if self.toggle_timer {
                let now = time::Instant::now();
                self.duration += now - self.last_tick;
                ui.label(format!("seconds: {}", self.duration.as_secs()));
                self.last_tick = now;
            }

            ui.image(egui::include_image!("../../resources/ferris.png"));
        });
    }
}
