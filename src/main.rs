mod metadata;

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

#[derive(Default)]
struct App {
    save_path: PathBuf,
    sav: Option<Sav>,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("File");
                let displayed = self.save_path.display().to_string();
                let s = if displayed.is_empty() {
                    "<No file>".to_string()
                } else {
                    displayed
                };
                ui.label(s);
                if ui.button("🗁 Open...").clicked() {
                    if let Some(path) = rfd::FileDialog::default().pick_file() {
                        let sav = Sav::load_from_file(&path).unwrap();
                        self.sav = Some(sav);
                        self.save_path = path;
                    }
                }
                if !self.save_path.as_os_str().is_empty() && ui.button("⟲ Reload").clicked() {
                    let sav = Sav::load_from_file(&self.save_path).unwrap();
                    self.sav = Some(sav);
                }
            });
            if let Some(sav) = &mut self.sav {
                ui.separator();
                ui.heading("Map");
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label("Current map")
                        .selected_text(map_text(sav.current_map.0))
                        .width(200.0)
                        .show_ui(ui, |ui| {
                            for i in 0..=255 {
                                ui.selectable_value(&mut sav.current_map.0, i, map_text(i));
                            }
                        });
                });
                ui.separator();
                ui.heading("Mdisks");
                for en in metadata::mdisk::TABLE {
                    ui.checkbox(&mut sav.mdisks.disks[en.bit_idx as usize], en.name);
                }
                ui.separator();
                if ui.button("Save").clicked() {
                    sav.save_to_file(&self.save_path).unwrap();
                }
            }
        });
    }
}

fn map_text(idx: u8) -> String {
    format!("{idx}: {name}", name = metadata::map::NAMES[idx as usize])
}
