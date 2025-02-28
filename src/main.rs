#![warn(clippy::unwrap_used, clippy::pedantic)]

use {
    egui_file_dialog::FileDialog,
    metadata::map::{MapInfo, fill_map_info},
    std::{collections::HashMap, error::Error},
};

mod bitmanip_yet_again;
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
    eframe::run_native(
        "BG&E Save editor",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, payload)))),
    )?;
    Ok(())
}

fn main() {
    match try_main() {
        Ok(()) => (),
        Err(e) => eprintln!("Fatal error: {e}"),
    }
}

struct App {
    save_path: PathBuf,
    sav: Option<Sav>,
    ui_state: UiState,
    bge_path: Option<PathBuf>,
    slot_exist_array: SlotExistArray,
    map_info: MapInfo,
    modal: ModalPopup,
    file_dialog: FileDialog,
}

#[derive(Default)]
pub struct ModalPopup {
    payload: Option<ModalPayload>,
}
impl ModalPopup {
    pub fn err(&mut self, context: &'static str, e: impl std::fmt::Display) {
        self.payload = Some(ModalPayload::Error {
            context,
            msg: e.to_string(),
        });
    }

    fn update(&mut self, ctx: &egui::Context) {
        if let Some(payload) = &self.payload {
            let mut close = false;
            egui::Modal::new("modal_popup".into()).show(ctx, |ui| {
                match payload {
                    ModalPayload::Error { context, msg } => {
                        ui.vertical_centered(|ui| {
                            ui.heading("Error");
                        });
                        ui.add_space(16.0);
                        ui.label(format!("{context}: {msg}"));
                        ui.add_space(16.0);
                    }
                }
                ui.vertical_centered(|ui| {
                    if ui.button("Close").clicked() {
                        close = true;
                    }
                });
            });
            if close {
                self.payload = None;
            }
        }
    }
}

enum ModalPayload {
    Error { context: &'static str, msg: String },
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
            map_filter: String::default(),
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
    fn new(_cc: &eframe::CreationContext<'_>, payload: Option<LoadPayload>) -> Self {
        let mut map_info = HashMap::default();
        fill_map_info(&mut map_info);
        let mut bge_path = None;
        let mut modal = ModalPopup::default();
        match SteamDir::locate() {
            Ok(dir) => match dir.find_app(15130) {
                Ok(Some((app, library))) => {
                    let path = library.resolve_app_dir(&app);
                    if path.exists() {
                        bge_path = Some(path);
                    } else {
                        modal.err(
                            "Error trying to find BG&E",
                            format!("Directory doesn't exist ({path:?})"),
                        );
                    }
                }
                Ok(None) => {
                    modal.err("Error trying to find BG&E", "Not found");
                }
                Err(e) => {
                    modal.err("Error trying to find BG&E", e);
                }
            },
            Err(e) => {
                modal.err("Error trying to locate Steam dir", e);
            }
        }
        match payload {
            Some(payload) => Self {
                save_path: payload.path.into(),
                sav: Some(payload.sav),
                ui_state: UiState::default(),
                slot_exist_array: [false; 5],
                bge_path,
                map_info,
                modal,
                file_dialog: FileDialog::default(),
            },
            None => Self {
                save_path: PathBuf::default(),
                sav: None,
                ui_state: UiState::default(),
                slot_exist_array: [false; 5],
                bge_path,
                map_info,
                modal,
                file_dialog: FileDialog::default(),
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
                    Tab::Map => ui::map(sav, &mut self.ui_state, ui, &self.map_info),
                    Tab::Party => ui::party(sav, ui, &self.map_info, &mut self.ui_state),
                    Tab::MDisk => ui::mdisk(sav, ui),
                    Tab::Inventory => ui::inventory(sav, ui, &mut self.ui_state),
                    Tab::Passwords => ui::passwords(sav, &mut self.ui_state, ui),
                }
            }
        });
        self.file_dialog.update(ctx);
        if let Some(path) = self.file_dialog.take_picked() {
            match Sav::load_from_file(&path) {
                Ok(sav) => {
                    self.sav = Some(sav);
                    self.save_path = path;
                }
                Err(e) => self.modal.err("Error loading file", e),
            }
        }
        self.modal.update(ctx);
    }
}
