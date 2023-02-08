mod metadata;
mod ui;

use std::{ffi::OsString, path::PathBuf};

use eframe::egui;
use libbgesav::{Sav, SavExt};

struct LoadPayload {
    path: OsString,
    sav: Sav,
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let payload = std::env::args_os().nth(1).map(|path| LoadPayload {
        sav: Sav::load_from_file(path.as_ref()).unwrap(),
        path,
    });
    eframe::run_native(
        "BG&E Save editor",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, payload))),
    );
}

struct App {
    save_path: PathBuf,
    sav: Option<Sav>,
    tab: Tab,
}

#[derive(PartialEq, Eq)]
enum Tab {
    Map,
    Party,
    MDisk,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>, payload: Option<LoadPayload>) -> Self {
        match payload {
            Some(payload) => Self {
                save_path: payload.path.into(),
                sav: Some(payload.sav),
                tab: Tab::Map,
            },
            None => Self {
                save_path: Default::default(),
                sav: Default::default(),
                tab: Tab::Map,
            },
        }
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
