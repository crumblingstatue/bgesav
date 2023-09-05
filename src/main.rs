#![warn(clippy::unwrap_used)]

use std::error::Error;

mod metadata;
mod sally_idx;
mod ui;

use {
    eframe::egui,
    libbgesav::Sav,
    std::{ffi::OsString, path::PathBuf},
    steamlocate::SteamDir,
};

struct LoadPayload {
    path: OsString,
    sav: Sav,
}

fn try_main() -> Result<(), Box<dyn Error>> {
    let native_options = eframe::NativeOptions::default();
    let payload = match std::env::args_os().nth(1) {
        Some(path) => Some(LoadPayload {
            sav: Sav::load_from_file(path.as_ref())?,
            path,
        }),
        None => None,
    };
    let mut bge_path = None;
    if let Some(mut dir) = SteamDir::locate() {
        if let Some(app) = dir.app(&15130) {
            bge_path = Some(app.path.clone());
        }
    }
    eframe::run_native(
        "BG&E Save editor",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, payload, bge_path))),
    )?;
    Ok(())
}

fn err_msg(e: &(impl Error + ?Sized)) {
    rfd::MessageDialog::new()
        .set_title("Error")
        .set_description(&e.to_string())
        .show();
}

fn main() {
    match try_main() {
        Ok(()) => (),
        Err(e) => err_msg(e.as_ref()),
    }
}

struct App {
    save_path: PathBuf,
    sav: Option<Sav>,
    ui_state: UiState,
    bge_path: Option<PathBuf>,
    slot_exist_array: SlotExistArray,
}

pub type SlotExistArray = [bool; 5];

struct UiState {
    tab: Tab,
    inv_tab: InvTab,
    map_filter: String,
    sync_pearls: bool,
    password_bufs: [String; 30],
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            tab: Tab::Inventory,
            inv_tab: InvTab::Jade,
            map_filter: Default::default(),
            sync_pearls: true,
            password_bufs: std::array::from_fn(|_| String::new()),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Tab {
    Inventory,
    Map,
    Party,
    MDisk,
    Passwords,
}

#[derive(PartialEq, Eq)]
enum InvTab {
    Jade,
    Peyj,
    DoubleH,
    Hovercraft,
}

impl App {
    fn new(
        _cc: &eframe::CreationContext<'_>,
        payload: Option<LoadPayload>,
        bge_path: Option<PathBuf>,
    ) -> Self {
        match payload {
            Some(payload) => Self {
                save_path: payload.path.into(),
                sav: Some(payload.sav),
                ui_state: UiState::default(),
                slot_exist_array: [false; 5],
                bge_path,
            },
            None => Self {
                save_path: Default::default(),
                sav: Default::default(),
                ui_state: UiState::default(),
                slot_exist_array: [false; 5],
                bge_path,
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
                match self.ui_state.tab {
                    Tab::Map => ui::map(sav, &mut self.ui_state, ui),
                    Tab::Party => ui::party(sav, ui),
                    Tab::MDisk => ui::mdisk(sav, ui),
                    Tab::Inventory => ui::inventory(sav, ui, &mut self.ui_state),
                    Tab::Passwords => ui::passwords(sav, &mut self.ui_state, ui),
                }
            }
        });
    }
}
