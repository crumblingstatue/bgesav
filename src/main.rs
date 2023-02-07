mod metadata;
mod ui;

use std::path::PathBuf;

use eframe::egui;
use libbgesav::Sav;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "BG&E Save editor",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}

struct App {
    save_path: PathBuf,
    sav: Option<Sav>,
    tab: Tab,
}

impl Default for App {
    fn default() -> Self {
        Self {
            save_path: Default::default(),
            sav: Default::default(),
            tab: Tab::Map,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Tab {
    Map,
    Party,
    MDisk,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui::top_panel(self, ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(sav) = &mut self.sav {
                match self.tab {
                    Tab::Map => ui::map(sav, ui),
                    Tab::Party => ui::party(sav, ui),
                    Tab::MDisk => ui::mdisk(sav, ui),
                }
            }
        });
    }
}
