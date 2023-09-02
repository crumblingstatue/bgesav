mod metadata;
mod ui;

use {
    eframe::egui,
    libbgesav::Sav,
    std::{ffi::OsString, path::PathBuf},
};

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
    )
    .unwrap();
}

struct App {
    save_path: PathBuf,
    sav: Option<Sav>,
    ui_state: UiState,
}

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
    fn new(_cc: &eframe::CreationContext<'_>, payload: Option<LoadPayload>) -> Self {
        match payload {
            Some(payload) => Self {
                save_path: payload.path.into(),
                sav: Some(payload.sav),
                ui_state: UiState::default(),
            },
            None => Self {
                save_path: Default::default(),
                sav: Default::default(),
                ui_state: UiState::default(),
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
