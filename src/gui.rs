#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use crate::beep;

pub fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "singhans",
        options,
        Box::new(|_cc| {
            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    beep: beep::Beep,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            beep: beep::Beep {
                amplitude: 1.0,
                freq: 200.0,
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("beep");
            ui.add(egui::Slider::new(&mut self.beep.amplitude, 0.1..=5.0).text("amplitude"));
            ui.add(egui::Slider::new(&mut self.beep.freq, 50.0..=2500.0).text("frequency"));
            if ui.button("beep").clicked() {
                beep::beep(self.beep).expect("beep failed");
            }
        });
    }
}